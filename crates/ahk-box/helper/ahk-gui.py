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
    """Make the window, or replace the one that is there. On the main thread.

    **It is not shown.** Measured on the binary of v2.0.19 on 2026-11-09: `Gui()`
    makes the window and does not show it -- a script may add to it, set its
    title and read it back before anything is on the screen -- and `Show()` is
    what puts it there. A helper that showed on creation would make `Gui()` and
    `Show()` the same command.
    """
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
    # The two events of v2 that a window of a script has: it is closed, or a
    # control in it is clicked.
    window.connect("destroy", lambda *_args: an_event("close"))
    answer({"ok": True})


def the_show():
    """Put the window on the screen. `Show()` of v2 answers the empty text."""
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window to show"})
        return
    state["window"].show_all()
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
    if kind == "Button":
        control.connect(
            "clicked", lambda *_args, held=number: an_event("click:%d" % held)
        )
    # The control is not shown on its own: `show_all` on the window shows the
    # window and everything in it, the way `Show()` does.
    control.show()
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


def the_value(number, text):
    """Read the text of a control, or write it. `Value` of v2 is both."""
    for held, kind, control in state["controls"]:
        if held != number:
            continue
        if text is None:
            if kind == "Button":
                answer({"ok": True, "value": control.get_label()})
            elif kind == "Text":
                answer({"ok": True, "value": control.get_text()})
            else:
                answer({"ok": True, "value": control.get_text()})
        else:
            if kind == "Button":
                control.set_label(text)
            elif kind == "Text":
                control.set_text(text)
            else:
                control.set_text(text)
            answer({"ok": True, "value": text})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_flag(number, name, setting):
    """Read a flag of a control, or write it. `Enabled` and `Visible` of v2.

    These are the calls of the window itself -- `set_sensitive` and
    `set_visible` -- and not of the accessibility layer: the window is the
    runtime's own, so nothing is being asked of a program that does not answer.
    The answer is the integer of nought and one that v2 gives.
    """
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        if name == "enabled":
            if setting is not None:
                control.set_sensitive(setting)
            answer({"ok": True, "value": 1 if control.get_sensitive() else 0})
        elif name == "visible":
            if setting is not None:
                control.set_visible(setting)
            answer({"ok": True, "value": 1 if control.get_visible() else 0})
        else:
            answer({"ok": False, "error": "%s is not a flag of a control" % name})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_geometry():
    """The place and the size of the window, on the screen.

    `GetPos` of v2. **A Wayland client is not told where its window is** -- the
    compositor places it and does not report the place -- so the position a
    toolkit answers may be noughts. What this answers is therefore what the
    toolkit says, and the box run is what decides whether it is the screen place
    or a pair of noughts.
    """
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window"})
        return
    x, y = state["window"].get_position()
    width, height = state["window"].get_size()
    answer({"ok": True, "value": [x, y, width, height]})


def the_watch(path):
    """Start writing the events of the window to a file.

    This is the first half of the channel that an `OnEvent` of v2 needs. Every
    channel built before it is question and answer -- one line in, one line out
    -- and an **event is the opposite**: the helper speaks first, at a moment the
    runtime is not waiting. So the events go somewhere the runtime can look
    between its statements, and a file is the plainest such place.

    One line per event, and the event is named the way the member that catches it
    is: `close` for the window and `click:<number>` for a control.
    """
    state["events"] = path
    answer({"ok": True})


def an_event(name):
    """Write one event, if a watch has been asked for."""
    path = state.get("events")
    if not path:
        return
    try:
        with open(path, "a", encoding="utf-8") as handle:
            handle.write(name + "\n")
    except OSError:
        # An event that cannot be written is not worth stopping the window for.
        pass


def run_on_loop(what):
    GLib.idle_add(what)
    return False


def handle(command):
    op = command.get("op", "")
    if op == "window":
        run_on_loop(lambda: the_window(command.get("title", "ahk-gui")))
    elif op == "watch":
        run_on_loop(lambda: the_watch(command.get("path", "")))
    elif op == "geometry":
        run_on_loop(the_geometry)
    elif op == "add":
        run_on_loop(
            lambda: the_control(command.get("kind", ""), command.get("text", ""))
        )
    elif op == "value":
        run_on_loop(lambda: the_value(command.get("id", 0), command.get("text")))
    elif op == "flag":
        run_on_loop(
            lambda: the_flag(
                command.get("id", 0),
                command.get("name", ""),
                command.get("on"),
            )
        )
    elif op == "show":
        run_on_loop(the_show)
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
