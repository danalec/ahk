#!/usr/bin/python3
"""The window of a script, drawn by GTK3, driven over a pipe.

This is the first slice of the channel that a `Gui` of v2 rides on. The language
crate does no I/O and holds no toolkit, so the window is made here, by a program
of its own, and the runner talks to it over the standard input and output:

    in   {"op": "window", "title": "ahk-gui-probe"}
    out  {"ok": true}

Why a helper at all: the tree already reaches the desktop by running the tools of
the box (`grim`, `zenity`, `busctl`), and a toolkit is the same kind of thing. A
binding would put GTK inside the runner and a window loop inside a runtime that
has none. The helper keeps the language crate free of both, and it is reversible.

The GTK loop owns the main thread, as it must; the pipe is read on a thread of
its own and every command is handed back to the loop with `idle_add`. A window
made off the main thread does not map, and that is the whole reason for the two.
"""

import json
import sys
import threading

import gi

gi.require_version("Gtk", "3.0")
from gi.repository import GLib, Gtk  # noqa: E402

state = {"window": None}


def answer(what):
    sys.stdout.write(json.dumps(what) + "\n")
    sys.stdout.flush()


def the_window(title):
    """Make the window, or replace the one that is there. On the main thread."""
    if state["window"] is not None:
        state["window"].destroy()
    window = Gtk.Window(title=title)
    window.set_default_size(320, 180)
    box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
    box.pack_start(Gtk.Label(label="the label of the window"), True, True, 0)
    entry = Gtk.Entry()
    entry.set_text("the text of the entry")
    box.pack_start(entry, False, False, 0)
    window.add(box)
    window.show_all()
    state["window"] = window
    answer({"ok": True})


def run_on_loop(what):
    GLib.idle_add(what)
    return False


def handle(command):
    op = command.get("op", "")
    if op == "window":
        run_on_loop(lambda: the_window(command.get("title", "ahk-gui")))
    elif op == "quit":
        answer({"ok": True})
        GLib.idle_add(Gtk.main_quit)
    else:
        answer({"ok": False, "error": "%s is not a command of the helper" % op})


def read_the_pipe():
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            command = json.loads(line)
        except ValueError:
            answer({"ok": False, "error": "the command is not JSON"})
            continue
        GLib.idle_add(handle, command)
    # The input closed: the runner is gone, so the window goes with it.
    GLib.idle_add(Gtk.main_quit)


if __name__ == "__main__":
    threading.Thread(target=read_the_pipe, daemon=True).start()
    Gtk.main()
