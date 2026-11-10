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

state = {"window": None, "box": None, "controls": [], "next_id": 1}


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
    window.add(box)
    state["window"] = window
    state["box"] = box
    state["controls"] = []
    state["next_id"] = 1
    window.show_all()
    answer({"ok": True})


def the_control(kind, text):
    """Add a control to the window, and give it the number that names it.

    The number is the `Hwnd` the script will see. This desktop has no window
    handles -- a window here is a Wayland toplevel and a node of the
    accessibility tree -- so the runtime numbers its **own** controls as it makes
    them, which is honest because the window is the runtime's own. The same
    number is the `ClassNN` of the control: `Edit1` and `Button2` are the first
    of each kind, in the order of `Add`.
    """
    if state["box"] is None:
        answer({"ok": False, "error": "there is no window to add a control to"})
        return
    if kind == "Edit":
        control = Gtk.Entry()
        control.set_text(text)
    elif kind == "Button":
        control = Gtk.Button(label=text)
    elif kind == "Text":
        control = Gtk.Label(label=text)
    else:
        answer({"ok": False, "error": "%s is not a kind of control" % kind})
        return
    number = state["next_id"]
    state["next_id"] += 1
    state["controls"].append((number, kind, control))
    state["box"].pack_start(control, False, False, 0)
    control.show_all()
    # The class and number of v2: `Edit1` is the first edit, `Button1` the first
    # button, whatever the order of the Add calls.
    same_kind = sum(1 for _n, k, _c in state["controls"] if k == kind)
    answer({"ok": True, "id": number, "classnn": "%s%d" % (kind, same_kind)})


def the_destroy():
    """Take the window away. `Destroy` of v2 answers the empty text."""
    if state["window"] is not None:
        state["window"].destroy()
    state["window"] = None
    state["box"] = None
    state["controls"] = []
    state["next_id"] = 1
    answer({"ok": True})


def the_title(setting):
    """Read the title of the window, or write it. `Title` of v2 is both."""
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window"})
        return
    if setting is not None:
        state["window"].set_title(setting)
    answer({"ok": True, "title": state["window"].get_title()})


def run_on_loop(what):
    GLib.idle_add(what)
    return False


def handle(command):
    op = command.get("op", "")
    if op == "window":
        run_on_loop(lambda: the_window(command.get("title", "ahk-gui")))
    elif op == "add":
        run_on_loop(
            lambda: the_control(command.get("kind", ""), command.get("text", ""))
        )
    elif op == "destroy":
        run_on_loop(the_destroy)
    elif op == "title":
        run_on_loop(lambda: the_title(command.get("set")))
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
