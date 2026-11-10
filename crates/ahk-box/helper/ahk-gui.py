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
from datetime import datetime

import gi

gi.require_version("Gtk", "3.0")
from gi.repository import GLib, Gtk  # noqa: E402
from gi.repository import Gdk  # noqa: E402

state = {"window": None, "box": None, "controls": [], "next_id": 1, "tooltip": None, "tip_no": 0}


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
    # And the one it has when a key is pressed in it, which for v2 is `Escape`
    # and nothing else: the reference accepts `Escape` as a window event and not
    # the other keys.
    window.connect("key-press-event", the_key_of_the_window)
    answer({"ok": True})


def the_show():
    """Put the window on the screen. `Show()` of v2 answers the empty text."""
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window to show"})
        return
    state["window"].show_all()
    answer({"ok": True})


def the_date_value(text, month_only):
    """The timestamp of a DateTime or MonthCal, normalised.

    Measured on the binary of v2.0.19 on 2026-11-10: a DateTime's `Value` is
    fourteen digits (yyyyMMddHHmmss) and a MonthCal's eight (yyyyMMdd). A
    short write is padded with zeros, and an empty one means "now".
    """
    wanted = 8 if month_only else 14
    digits = "".join(ch for ch in text if ch.isdigit())
    if not digits:
        digits = datetime.now().strftime("%Y%m%d" if month_only else "%Y%m%d%H%M%S")
    return digits[:wanted].ljust(wanted, "0")


def the_date_display(value):
    """The `M/D/YYYY` text a DateTime shows in `WinGetText`.

    Measured on the binary of v2.0.19 on 2026-11-10: after setting the value
    to 20260101, `WinGetText` answers "1/1/2026".
    """
    return "%d/%d/%s" % (int(value[4:6]), int(value[6:8]), value[0:4])


def the_range_and_value(text):
    """The lower bound, upper bound and starting value of a numeric control.

    The options carry `RangeLOW-HIGH` and the text (for a Slider or a Progress)
    carries the starting number. Measured on the binary of v2.0.19 on
    2026-11-10: `Range0-100` with a text of 50 starts at 50, and with no text
    starts at the lower bound.
    """
    low, high, start = 0, 100, None
    for word in text.split():
        if word.lower().startswith("range"):
            try:
                bounds = word[5:].split("-")
                low, high = int(bounds[0]), int(bounds[-1])
            except (ValueError, IndexError):
                pass
        else:
            try:
                start = int(word)
            except ValueError:
                pass
    return low, high, low if start is None else max(low, min(high, start))


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
    elif kind == "CheckBox":
        control = Gtk.CheckButton(label=text)
    elif kind == "StatusBar":
        # A status bar of v2 is a row of text; this tree carries one part, the
        # text the control was made with. Measured on the binary of v2.0.19 on
        # 2026-11-10: `StatusBarGetText(1, WinTitle)` answers it.
        control = Gtk.Label(label=text)
    elif kind == "ListView":
        # `Add("ListView", options, ["Col1", "Col2"])` -- the columns come as
        # lines of the text the language joined. Measured on the binary of
        # v2.0.19 on 2026-11-10: a ListView is `SysListView321`, rows are added
        # with `lv.Add("", "a", "b")` and `ListViewGetContent` answers the
        # cells separated by tabs and the rows by line breaks.
        columns = [name for name in text.split("\n") if name]
        if not columns:
            columns = ["Column"]
        store = Gtk.ListStore(*([str] * len(columns)))
        control = Gtk.TreeView(model=store)
        for index, name in enumerate(columns):
            control.append_column(
                Gtk.TreeViewColumn(name, Gtk.CellRendererText(), text=index)
            )
        control.set_headers_visible(True)
        control.get_selection().set_mode(Gtk.SelectionMode.SINGLE)
        state.setdefault("list_columns", {})[state["next_id"]] = len(columns)
    elif kind == "TreeView":
        # `Add("TreeView", options)` -- one column of item texts, with parents.
        store = Gtk.TreeStore(str, int)
        control = Gtk.TreeView(model=store)
        control.append_column(
            Gtk.TreeViewColumn("Item", Gtk.CellRendererText(), text=0)
        )
        control.set_headers_visible(False)
        control.get_selection().set_mode(Gtk.SelectionMode.SINGLE)
        state.setdefault("tree_items", {})[state["next_id"]] = {
            "next": 1,
            "by_id": {},
        }
    elif kind == "UpDown":
        # An UpDown is a spinner attached to the Edit that was added before it.
        # Measured on the binary of v2.0.19 on 2026-11-10: it is
        # `msctls_updown321`, its `Value` is an Integer, and it shares that
        # value with the buddy in BOTH directions -- setting the Edit's text
        # changes what the UpDown reads and the other way round.
        low, high, _start = the_range_and_value(text)
        control = Gtk.SpinButton()
        control.set_range(low, high)
        control.set_increments(1, 1)
        control.set_value(low)
        number = state["next_id"]
        # The buddy is the last Edit added before this one.
        buddy = None
        for held, held_kind, _held_control in reversed(state["controls"]):
            if held_kind == "Edit":
                buddy = held
                break
        state.setdefault("updown_buddy", {})[number] = buddy
        if buddy is not None:
            for held, _held_kind, held_control in state["controls"]:
                if held == buddy:
                    held_control.set_text(str(low))
                    break
    elif kind == "Slider":
        # `msctls_trackbar321`. Measured on the binary of v2.0.19 on 2026-11-10:
        # `Value` is an Integer, it starts at the number in the text (or the
        # lower bound), and setting it moves the thumb.
        low, high, start = the_range_and_value(text)
        control = Gtk.Scale.new_with_range(
            Gtk.Orientation.HORIZONTAL, low, high, 1
        )
        control.set_value(start)
        state.setdefault("number_values", {})[state["next_id"]] = start
        state.setdefault("number_ranges", {})[state["next_id"]] = (low, high)
    elif kind == "Progress":
        # `msctls_progress321`. Measured on the same day: `Value` is an Integer
        # that starts at the number in the text (or the lower bound).
        low, high, start = the_range_and_value(text)
        control = Gtk.ProgressBar()
        span = max(1, high - low)
        control.set_fraction((start - low) / span)
        state.setdefault("number_values", {})[state["next_id"]] = start
        state.setdefault("number_ranges", {})[state["next_id"]] = (low, high)
    elif kind in ("DateTime", "MonthCal"):
        # Both hold a timestamp STRING. Measured on the binary of v2.0.19 on
        # 2026-11-10: DateTime is SysDateTimePick321 (14 digits) and MonthCal
        # SysMonthCal321 (8 digits), short writes are padded, and the DateTime
        # shows M/D/YYYY in WinGetText while the MonthCal shows nothing.
        month_only = kind == "MonthCal"
        digits = "".join(ch for ch in text if ch.isdigit())
        if not month_only and digits and len(digits) <= 8:
            # Measured on the binary of v2.0.19 on 2026-11-10: giving Add a
            # date-only text fills the time with the CURRENT clock, while
            # writing `Value` later pads it with zeros instead.
            value = digits.ljust(8, "0") + datetime.now().strftime("%H%M%S")
        else:
            value = the_date_value(text, month_only)
        control = Gtk.Entry()
        control.set_text(the_date_display(value) if not month_only else "")
        control.set_editable(False)
        state.setdefault("date_values", {})[state["next_id"]] = value
        state.setdefault("date_kind", {})[state["next_id"]] = kind
    elif kind == "Link":
        # A Link's caption is its raw markup, which is what `WinGetText` shows.
        # `Value` is always empty and writing it is refused by v2; the class
        # dispatcher handles that, while `Text` reads and writes this label.
        control = Gtk.Label(label=text)
        control.set_text(text)
    elif kind == "Hotkey":
        # A Hotkey holds the hotkey string in `Value`; `Text` is always empty.
        control = Gtk.Entry()
        control.set_text(text)
    elif kind == "Pic":
        # A Pic is a Static that shows an image. Measured on the binary of
        # v2.0.19 on 2026-11-10: its `ClassNN` shares the Static counter with
        # Text, `Value` and `Text` answer the file path, and WRITING `Value`
        # is ignored -- the image v2 shows does not change.
        control = Gtk.Image()
        if text:
            control.set_from_file(text)
        state.setdefault("pic_paths", {})[state["next_id"]] = text
    elif kind == "Tab":
        # A Tab is a notebook with one page per item. Measured on the binary
        # of v2.0.19 on 2026-11-10: it is `SysTabControl321`, `Value` is the
        # 1-based number of the chosen page, and a page that is not there is
        # refused with "Invalid value.". The chosen page is kept HERE rather
        # than read out of the widget: on an unrealized notebook GTK answers
        # -1 for the current page even after it is set (measured 2026-11-10),
        # and v2's first tab is chosen from the start.
        control = Gtk.Notebook()
        pages = 0
        for name in text.split("\n"):
            if name:
                control.append_page(Gtk.Label(label=name), Gtk.Label(label=name))
                pages += 1
        control.set_current_page(0)
        state.setdefault("tab_pages", {})[state["next_id"]] = 1 if pages else 0
    elif kind in ("DropDownList", "ComboBox", "ListBox"):
        # A ComboBox of v2: the third argument of Add is an array of items,
        # joined into lines by the language before it reaches the helper.
        # Measured on the binary of v2.0.19 on 2026-11-10: `Value` is a
        # 1-based index (0 = nothing chosen) and `Text` is the chosen item.
        control = Gtk.ComboBoxText()
        for item in text.split("\n"):
            if item:
                control.append_text(item)
        if kind == "ListBox":
            control.set_wrap_width(1)
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
    answer({"ok": True, "id": number, "classnn": "%s%d" % (the_class_of(kind), same_kind)})


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


def the_text_of_a_control(number, kind, control):
    """The text a control shows, whatever its kind.

    A ComboBox has no `get_text`: its text is the item that is chosen, or the
    empty text when nothing is. `WinGetText` and `Value` of a control ask this
    of every control of the window.
    """
    if kind in ("Button", "CheckBox"):
        return control.get_label()
    if kind in ("DropDownList", "ComboBox", "ListBox"):
        index = control.get_active()
        if index < 0:
            return ""
        return control.get_model()[index][0]
    if kind == "ListView":
        # The text of a list view is its rows, which `ListViewGetContent` reads.
        model = control.get_model()
        return "\n".join("\t".join(list(row)) for row in model)
    if kind == "TreeView":
        model = control.get_model()
        return "\n".join(row[0] for row in model)
    if kind == "Pic":
        # The text of a Pic is the file it shows. Measured on the binary of
        # v2.0.19 on 2026-11-10: `WinGetText` answers the path.
        return state.get("pic_paths", {}).get(number, "")
    return control.get_text()


def the_value(number, text):
    """Read the text of a control, or write it. `Value` of v2 is both."""
    for held, kind, control in state["controls"]:
        if held != number:
            continue
        if kind in ("DateTime", "MonthCal"):
            # Both hold a timestamp STRING read and written through `Value`.
            # Measured on the binary of v2.0.19 on 2026-11-10.
            if text is None:
                answer({
                    "ok": True,
                    "value": state.get("date_values", {}).get(number, ""),
                })
            else:
                value = the_date_value(text, kind == "MonthCal")
                state.setdefault("date_values", {})[number] = value
                if kind == "DateTime":
                    control.set_text(the_date_display(value))
                answer({"ok": True, "value": value})
            return
        if kind == "Tab":
            # A Tab's `Value` is the 1-based page number, kept in the state
            # because an unrealized notebook answers -1 for its current page.
            # A page that is not there is refused with "Invalid value.".
            pages = control.get_n_pages()
            held = state.setdefault("tab_pages", {})
            if text is None:
                answer({"ok": True, "value": str(held.get(number, 1))})
            else:
                try:
                    page = int(text)
                except ValueError:
                    page = 0
                if page < 1 or page > pages:
                    answer({"ok": False, "error": "Invalid value."})
                    return
                held[number] = page
                control.set_current_page(page - 1)
                answer({"ok": True, "value": text})
            return
        if kind in ("Slider", "Progress"):
            # Both hold an Integer `Value`. Measured on the binary of v2.0.19
            # on 2026-11-10. The helper keeps the number so the toolkit widget
            # and the script read the same thing.
            if text is None:
                answer({
                    "ok": True,
                    "value": str(state.get("number_values", {}).get(number, 0)),
                })
            else:
                try:
                    value = int(text)
                except ValueError:
                    value = 0
                state.setdefault("number_values", {})[number] = value
                if kind == "Slider":
                    control.set_value(value)
                else:
                    low, high = state.get("number_ranges", {}).get(number, (0, 100))
                    span = max(1, high - low)
                    control.set_fraction(max(0.0, min(1.0, (value - low) / span)))
                answer({"ok": True, "value": str(value)})
            return
        if kind == "UpDown":
            # The spinner shares its number with the buddy Edit in both
            # directions. Measured on the binary of v2.0.19 on 2026-11-10.
            buddy = state.get("updown_buddy", {}).get(number)
            buddy_text = ""
            if buddy is not None:
                for held2, _kind2, buddy_control in state["controls"]:
                    if held2 == buddy:
                        buddy_text = buddy_control.get_text()
                        break
            if text is None:
                try:
                    answer({"ok": True, "value": str(int(buddy_text))})
                except ValueError:
                    answer({"ok": True, "value": "0"})
            else:
                try:
                    value = int(text)
                except ValueError:
                    value = 0
                control.set_value(value)
                if buddy is not None:
                    for held2, _kind2, buddy_control in state["controls"]:
                        if held2 == buddy:
                            buddy_control.set_text(str(value))
                            break
                answer({"ok": True, "value": str(value)})
            return
        if text is None:
            answer({"ok": True, "value": the_text_of_a_control(number, kind, control)})
        else:
            if kind == "Pic":
                # Measured on the binary of v2.0.19 on 2026-11-10: writing
                # `Value` of a Pic answers ok and changes NOTHING -- the image
                # v2 shows does not change either.
                answer({"ok": True, "value": the_text_of_a_control(number, kind, control)})
                return
            if kind in ("DropDownList", "ComboBox", "ListBox"):
                # Setting `Value` of a ComboBox chooses an item by its index.
                model = control.get_model()
                try:
                    index = int(text)
                except ValueError:
                    index = 0
                control.set_active(index - 1 if 0 < index <= len(model) else -1)
            elif kind == "Button":
                control.set_label(text)
            elif kind == "CheckBox":
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


def the_key_of_the_window(_window, event):
    """A key pressed in the window. `Escape` is an event of v2; keys are not."""
    if event.keyval == Gdk.KEY_Escape:
        an_event("escape")
    return False


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


def the_size(number, width, height):
    """The size of a control. The place is not this: a child of a box has none.

    `Move` of v2 is a place and a size, and the window this helper makes is a
    vertical box, so only the size can be honoured here.
    """
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        control.set_size_request(width, height)
        answer({"ok": True, "value": [width, height]})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_focus(number):
    """Put the focus on a control of the window. `Focus` of v2, which answers nothing."""
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        window = state["window"]
        if window is not None:
            window.present()
        control.grab_focus()
        answer({"ok": True, "value": 1 if control.has_focus() else 0})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_hide():
    """Take the window off the screen. `Hide` of v2, which answers nothing."""
    if state["window"] is not None:
        state["window"].hide()
    answer({"ok": True})


def the_option(name, on):
    """Apply one option of the window, for the ones the toolkit honours.

    The four are the ones GTK gives a program control of. The rest of v2's options
    are refused by the caller with a reason, because a window whose title bar
    belongs to the compositor is not one whose buttons a program sets.
    """
    window = state["window"]
    if window is None:
        answer({"ok": False, "error": "there is no window"})
        return
    low = name.lower()
    if low == "alwaysontop":
        window.set_keep_above(on)
    elif low == "resize":
        window.set_resizable(on)
    elif low == "caption":
        window.set_decorated(on)
    elif low == "disabled":
        window.set_sensitive(not on)
    elif low == "activate":
        window.present()
    elif low == "minimize":
        window.iconify()
    elif low == "maximize":
        window.maximize()
    elif low == "restore":
        window.unmaximize()
        window.deiconify()
    elif low == "close":
        window.close()
    elif low == "redraw":
        window.queue_draw()
    else:
        answer({"ok": False, "error": "%s is not an option this window can set" % name})
        return
    answer({"ok": True})


def the_line_count(number):
    """How many lines the text of a control holds. `EditGetLineCount` of v2.

    The text of the entry carries its newlines (measured: three lines came back
    as twenty-eight characters), so the count is the number of them plus one.
    """
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        answer({"ok": True, "value": control.get_text().count("\n") + 1})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_cursor(number):
    """Where the cursor of a control is, as a place in its text."""
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        answer({"ok": True, "value": control.get_position()})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_selection(number):
    """The text a control has selected. `EditGetSelectedText` of v2."""
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        bounds = control.get_selection_bounds()
        if not bounds or len(bounds) < 2:
            answer({"ok": True, "value": ""})
            return
        text = control.get_text()
        start = max(0, int(bounds[0]))
        end = min(len(text), int(bounds[1]))
        answer({"ok": True, "value": text[start:end]})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_focused():
    """Which control of the window has the focus, as its class and number.

    `ControlGetFocus` of v2, for a window of the script's own -- the one the helper
    made and therefore knows the focus of.
    """
    window = state["window"]
    if window is None:
        answer({"ok": False, "error": "there is no window"})
        return
    held = window.get_focus()
    for number, kind, control in state["controls"]:
        if control is held:
            answer({"ok": True, "value": "%s%d" % (the_class_of(kind), number)})
            return
    answer({"ok": True, "value": ""})


def the_number_named(classnn):
    """The number that names a control of the window, from its class and number.

    `ControlGetHwnd` of v2: the handle of a control, which is the number this helper
    gives it and the one an event about it carries.
    """
    wanted = classnn.strip().lower()
    for number, _kind, _control in state["controls"]:
        if (the_classnn_of(number) or "").lower() == wanted:
            answer({"ok": True, "value": number})
            return
    answer({"ok": False, "error": "there is no control named %s" % classnn})


def the_state_of(classnn, what, on):
    """Set a state of a control named by its class and number.

    `ControlShow`/`Hide` for the visibility and `ControlSetEnabled` for the
    sensitivity. Round 35 refused the enabled one because the accessibility layer
    has no call to change another program's control state -- which is true, and is
    not this case: the window is the helper's own.
    """
    wanted = classnn.strip().lower()
    for number, kind, control in state["controls"]:
        if (the_classnn_of(number) or "").lower() != wanted:
            continue
        if what == "enabled":
            control.set_sensitive(on)
        else:
            control.set_visible(on)
        answer({"ok": True})
        return
    answer({"ok": False, "error": "there is no control named %s" % classnn})


def the_scale():
    """The scale factor of the output. `A_ScreenDPI` is 96 times this."""
    try:
        screen = Gdk.Display.get_default().get_monitors().get_item(0).get_scale_factor()
        answer({"ok": True, "value": int(screen) if screen else 1})
    except Exception:
        answer({"ok": True, "value": 1})


def the_checked(number, setting):
    """Whether a check box is ticked, or tick it.

    `Value` of a check box in v2 is a NUMBER -- nought or one -- and its `Text` is
    the label. They are two different things on one control, which is why they come
    through two different operations.
    """
    for held, kind, control in state["controls"]:
        if held != number:
            continue
        if kind != "CheckBox":
            answer({"ok": False, "error": "%s%d is not a check box" % (kind, number)})
            return
        if setting is not None:
            control.set_active(setting)
        answer({"ok": True, "value": 1 if control.get_active() else 0})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_paste(number, text):
    """Put a text into a control at its cursor. `EditPaste` of v2.

    **Not `insert_at_cursor`**: measured in round 147, that call stops the helper
    answering, and the runner's bound turns it into a failure. The buffer is written
    instead and the cursor set after it, which is the same result without asking the
    toolkit to do the placing.
    """
    for held, kind, control in state["controls"]:
        if held != number:
            continue
        if kind != "Edit":
            answer({"ok": False, "error": "%s%d is not an edit" % (kind, number)})
            return
        whole = control.get_text()
        at = max(0, min(len(whole), control.get_position()))
        control.set_text(whole[:at] + text + whole[at:])
        control.set_position(at + len(text))
        answer({"ok": True, "value": control.get_text()})
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_opacity(setting):
    """The opacity of the window, or set it. `WinSetTransparent` of v2 counts 0-255
    and the toolkit counts 0.0-1.0, so the answer is the scale v2 uses."""
    window = state["window"]
    if window is None:
        answer({"ok": False, "error": "there is no window"})
        return
    if setting is not None:
        window.set_opacity(max(0.0, min(1.0, setting / 255.0)))
    answer({"ok": True, "value": int(round(window.get_opacity() * 255))})


def the_text_of_all():
    """The text of every control of the window, each followed by a line break.

    `WinGetText` of v2, for the script's own window. Measured on the binary: a label
    and an edit answered their two texts, one after the other, each with a break after
    it.
    """
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window"})
        return
    parts = []
    for number, kind, control in state["controls"]:
        # An UpDown, Slider, Progress or MonthCal has no text of its own in
        # `WinGetText`; v2 leaves it out entirely, not even an empty line. A
        # DateTime is different: its M/D/YYYY display IS its text, which the
        # entry it is made of already holds. Measured on the binary of v2.0.19
        # on 2026-11-10.
        if kind in ("UpDown", "Slider", "Progress", "MonthCal", "Hotkey"):
            continue
        parts.append(the_text_of_a_control(number, kind, control))
        parts.append("\n")
    answer({"ok": True, "value": "".join(parts)})


def the_class_of(kind):
    """The class that names a control in v2.

    Measured on the binary of v2.0.19 on 2026-11-09: a `Text` control is a **`Static`**
    to v2 -- `WinGetControls` of a window with a label, an edit and a button answers
    `Static1`, `Edit1` and `Button1` -- and the other kinds are named as they are.
    """
    if kind == "Text":
        return "Static"
    # Measured on the binary of v2.0.19 on 2026-11-10: the `ClassNN` of a
    # DropDownList is `ComboBox1`, not `DropDownList1`. ComboBox and ListBox
    # are the same family to v2.
    if kind in ("DropDownList", "ComboBox", "ListBox"):
        return "ComboBox"
    # Measured on the binary of v2.0.19 on 2026-11-10: a StatusBar is named
    # `msctls_statusbar321` (the Win32 common-control class), not `StatusBar1`.
    if kind == "StatusBar":
        return "msctls_statusbar32"
    # A ListView is `SysListView321`, and a TreeView is `SysTreeView321`.
    if kind == "ListView":
        return "SysListView32"
    if kind == "TreeView":
        return "SysTreeView32"
    # Measured on the binary of v2.0.19 on 2026-11-10: an UpDown is named
    # `msctls_updown321`, a Slider `msctls_trackbar321` and a Progress
    # `msctls_progress321` -- the Win32 common-control classes.
    if kind == "UpDown":
        return "msctls_updown32"
    if kind == "Slider":
        return "msctls_trackbar32"
    if kind == "Progress":
        return "msctls_progress32"
    # Measured on the binary of v2.0.19 on 2026-11-10: a DateTime is
    # `SysDateTimePick321` and a MonthCal `SysMonthCal321`.
    if kind == "DateTime":
        return "SysDateTimePick32"
    if kind == "MonthCal":
        return "SysMonthCal32"
    # Measured on the binary of v2.0.19 on 2026-11-10: a Link is `SysLink1`
    # and a Hotkey `msctls_hotkey321`.
    if kind == "Hotkey":
        return "msctls_hotkey32"
    if kind == "Link":
        return "SysLink"
    # Measured on the binary of v2.0.19 on 2026-11-10: a Pic is a `Static1`
    # -- it shares the counter with Text -- and shows the file it was given.
    if kind == "Pic":
        return "Static"
    # A Tab is `SysTabControl321`.
    if kind == "Tab":
        return "SysTabControl32"
    return kind


def the_classnn_of(number):
    """The class and number that names a control.

    The class is v2's (`Static` for a label) and the number is counted **within that
    class**, which is what v2 does: a label, an edit and a button are `Static1`, `Edit1`
    and `Button1` and not `Static1`, `Edit2` and `Button3`.
    """
    seen = {}
    for held, kind, _control in state["controls"]:
        held_class = the_class_of(kind)
        seen[held_class] = seen.get(held_class, 0) + 1
        if held == number:
            return "%s%d" % (held_class, seen[held_class])
    return None


def the_controls():
    """The class and number of every control of the window, one per line.

    `WinGetControls` of v2, which answers an array of them.
    """
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window"})
        return
    # The number of a control is counted PER CLASS, which is what v2 does: a label, an
    # edit and a button are `Static1`, `Edit1` and `Button1` and not `Static1`, `Edit2`
    # and `Button3`.
    seen = {}
    names = []
    for _number, kind, _control in state["controls"]:
        held = the_class_of(kind)
        seen[held] = seen.get(held, 0) + 1
        names.append("%s%d" % (held, seen[held]))
    answer({"ok": True, "value": "\n".join(names)})


def the_tooltip(text, x, y):
    """Show a small window of text, or take it away with an empty text.

    `ToolTip` of v2. Measured on the binary of v2.0.19 on 2026-11-10: showing one
    answers a number and `ToolTip()` clears it. The number here is an identity,
    which is what a script can hold and compare; the address v2 answers is a
    pointer of its own process and would mean nothing here.
    """
    if state["tooltip"] is not None:
        state["tooltip"].destroy()
        state["tooltip"] = None
    if text == "":
        answer({"ok": True, "value": 0})
        return
    state["tip_no"] += 1
    window = Gtk.Window(type=Gtk.WindowType.POPUP)
    window.set_title("tooltip")
    window.add(Gtk.Label(label=text))
    window.move(x, y)
    window.show_all()
    state["tooltip"] = window
    answer({"ok": True, "value": state["tip_no"]})


def the_window_move(x, y, width, height):
    """Move and size the window of the script.

    `WinMove` of v2, measured on the binary on 2026-11-10. A Wayland compositor
    places the windows itself, so `move` is the call the toolkit offers and the
    compositor need not honour it; the size is the one thing a client can ask for.
    """
    if state["window"] is None:
        answer({"ok": False, "error": "there is no window"})
        return
    state["window"].move(x, y)
    if width > 0 and height > 0:
        state["window"].resize(width, height)
    answer({"ok": True})


def the_combo(number):
    """The control of this number, if it is a ComboBox of the script."""
    for held, kind, control in state["controls"]:
        if held == number and kind in ("DropDownList", "ComboBox", "ListBox"):
            return control
    return None


def the_combo_items(number):
    """The items of a ComboBox, in order. `ControlGetItems` of v2."""
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    items = []
    model = control.get_model()
    for row in model:
        items.append(row[0])
    answer({"ok": True, "value": "\n".join(items)})


def the_combo_add(number, text):
    """Append an item to a ComboBox. `ControlAddItem` of v2."""
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    control.append_text(text)
    answer({"ok": True})


def the_combo_select(number, wanted, by_text):
    """Choose an item, by its 1-based index or by its text.

    `ControlChooseIndex`, `ControlChooseString` and the `Value`/`Text` setters
    of a ComboBox. Measured on the binary of v2.0.19 on 2026-11-10: index 2
    chooses the second item, and setting `Text` to an item's text chooses it.
    """
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    model = control.get_model()
    if by_text:
        for index, row in enumerate(model):
            if row[0] == wanted:
                control.set_active(index)
                answer({"ok": True, "value": index + 1})
                return
        answer({"ok": False, "error": "there is no item named %s" % wanted})
        return
    try:
        index = int(wanted)
    except ValueError:
        index = 0
    # v2's index is 1-based; GTK's is 0-based, and -1 is nothing chosen.
    if index <= 0:
        control.set_active(-1)
        answer({"ok": True, "value": 0})
        return
    if index > len(model):
        answer({"ok": False, "error": "there is no item at %d" % index})
        return
    control.set_active(index - 1)
    answer({"ok": True, "value": index})


def the_combo_choice(number):
    """The chosen item of a ComboBox, as its text and its 1-based index.

    `ControlGetChoice` reads the text, and `Value`/`Text` of the control read
    these two. Measured on the binary of v2.0.19 on 2026-11-10: with nothing
    chosen the index is 0 and the text is empty.
    """
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    index = control.get_active()
    if index < 0:
        answer({"ok": True, "index": 0, "text": ""})
        return
    model = control.get_model()
    answer({"ok": True, "index": index + 1, "text": model[index][0]})


def the_combo_delete(number, wanted):
    """Remove an item by its 1-based index. `ControlDeleteItem` of v2."""
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    try:
        index = int(wanted)
    except ValueError:
        index = 0
    if index < 1 or index > len(control.get_model()):
        answer({"ok": False, "error": "there is no item at %s" % wanted})
        return
    control.remove(index - 1)
    answer({"ok": True})


def the_combo_find(number, text):
    """The 1-based index of the first item that is this text.

    `ControlFindItem` of v2. Measured on the binary of v2.0.19 on 2026-11-10:
    the answer is an index, and zero when the item is not there.
    """
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    for index, row in enumerate(control.get_model()):
        if row[0] == text:
            answer({"ok": True, "value": index + 1})
            return
    answer({"ok": True, "value": 0})


def the_combo_dropdown(number, show):
    """Open or close the pop-up of a ComboBox.

    `ControlShowDropDown` and `ControlHideDropDown` of v2. Measured on the
    binary of v2.0.19 on 2026-11-10: both answer the empty text. GTK has the
    two calls of its own for a ComboBoxText, and a Wayland compositor shows the
    pop-up where it will.
    """
    control = the_combo(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list-shaped control" % number})
        return
    if show:
        control.popup()
    else:
        control.popdown()
    answer({"ok": True})


def the_status_text():
    """The text of the status bar of the window.

    `StatusBarGetText` of v2. Measured on the binary of v2.0.19 on 2026-11-10:
    `StatusBarGetText(1, WinTitle)` answers the text the StatusBar was made
    with. This tree carries one part, so the first StatusBar is the answer.
    """
    for _number, kind, control in state["controls"]:
        if kind == "StatusBar":
            answer({"ok": True, "value": control.get_text()})
            return
    answer({"ok": False, "error": "there is no status bar on the window"})


def the_list_view(number):
    """The TreeView of this number, if it is a ListView of the script."""
    for held, kind, control in state["controls"]:
        if held == number and kind == "ListView":
            return control
    return None


def the_list_rows(number):
    """Every row of a ListView as a list of cell texts."""
    control = the_list_view(number)
    if control is None:
        return None
    model = control.get_model()
    return [list(row) for row in model]


def the_list_add(number, cells):
    """Append a row to a ListView. `lv.Add` of v2.

    Measured on the binary of v2.0.19 on 2026-11-10: `lv.Add("", "a", "b")`
    answers the 1-based number of the row. The options come first and are not
    carried here; the rest are the cells.
    """
    control = the_list_view(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    columns = state.get("list_columns", {}).get(number, 1)
    row = list(cells[:columns])
    while len(row) < columns:
        row.append("")
    control.get_model().append(row)
    answer({"ok": True, "value": len(control.get_model())})


def the_list_count(number):
    """The number of rows. `lv.GetCount` of v2."""
    control = the_list_view(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    answer({"ok": True, "value": len(control.get_model())})


def the_list_text(number, row, column):
    """The text of one cell. `lv.GetText` of v2, row and column 1-based."""
    rows = the_list_rows(number)
    if rows is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    if row < 1 or row > len(rows) or column < 1 or column > len(rows[row - 1]):
        answer({"ok": True, "value": ""})
        return
    answer({"ok": True, "value": rows[row - 1][column - 1]})


def the_list_content(number, selected_only):
    """The rows of a ListView, cells by tabs and rows by line breaks.

    `ListViewGetContent` of v2. Measured on the binary of v2.0.19 on
    2026-11-10: all rows answer "alpha\t10\nbeta\t20", and the "Selected"
    option answers only the rows that are chosen.
    """
    control = the_list_view(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    model = control.get_model()
    chosen = None
    if selected_only:
        chosen = control.get_selection().get_selected()[1]
    parts = []
    for index, row in enumerate(model):
        if selected_only and (chosen is None or model.get_path(chosen).get_indices()[0] != index):
            continue
        parts.append("\t".join(list(row)))
    answer({"ok": True, "value": "\n".join(parts)})


def the_list_modify(number, row, cells):
    """Change the cells of a row, and select it. `lv.Modify` of v2."""
    control = the_list_view(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    model = control.get_model()
    if row < 1 or row > len(model):
        answer({"ok": False, "error": "there is no row %d" % row})
        return
    current = list(model[row - 1])
    for index, text in enumerate(cells):
        if index < len(current):
            current[index] = text
    model[row - 1] = current
    control.get_selection().select_iter(model.get_iter(row - 1))
    answer({"ok": True, "value": 1})


def the_list_next(number, start):
    """The next row after a row, or zero. `lv.GetNext` of v2.

    Measured on the binary of v2.0.19 on 2026-11-10: with no starting row the
    answer is the first SELECTED row, or zero when nothing is chosen. With a
    starting row it is that row plus one, or zero past the end.
    """
    control = the_list_view(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a list view" % number})
        return
    count = len(control.get_model())
    if start == 0:
        selected = control.get_selection().get_selected()[1]
        if selected is None:
            answer({"ok": True, "value": 0})
            return
        answer({"ok": True, "value": control.get_model().get_path(selected).get_indices()[0] + 1})
        return
    answer({"ok": True, "value": start + 1 if start + 1 <= count else 0})


def the_tree(number):
    """The TreeView and its table of items, if this number is a tree."""
    for held, kind, control in state["controls"]:
        if held == number and kind == "TreeView":
            return control, state.get("tree_items", {}).get(number)
    return None, None


def the_tree_iter(table, item_id):
    """The GtkTreeIter of an item id, or None when the id is not there."""
    entry = table.get("by_id", {}).get(item_id) if table else None
    return entry.get("iter") if entry else None


def the_tree_add(number, text, parent, options):
    """Add an item to a tree, and answer its id. `tv.Add` of v2.

    Measured on the binary of v2.0.19 on 2026-11-10: `tv.Add("Child", parent)`
    answers an id that GetParent and GetChild use. The ids here are sequential
    integers from one, which is an identity a script can hold and compare.
    """
    control, table = the_tree(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    item_id = table["next"]
    table["next"] += 1
    parent_iter = the_tree_iter(table, parent) if parent else None
    row = control.get_model().append(parent_iter, [text, item_id])
    table["by_id"][item_id] = {
        "text": text,
        "parent": parent if parent else 0,
        "iter": row,
        "order": item_id,
    }
    if "select" in options.lower():
        control.get_selection().select_iter(row)
    answer({"ok": True, "value": item_id})


def the_tree_count(number):
    """The number of items. `tv.GetCount` of v2."""
    _control, table = the_tree(number)
    if table is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    answer({"ok": True, "value": len(table["by_id"])})


def the_tree_text(number, item_id):
    """The text of an item, or the empty text. `tv.GetText` of v2."""
    _control, table = the_tree(number)
    if table is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    entry = table["by_id"].get(item_id)
    answer({"ok": True, "value": entry["text"] if entry else ""})


def the_tree_relation(number, item_id, relation):
    """The id of a related item: parent, child, next or previous.

    Measured on the binary of v2.0.19 on 2026-11-10: GetParent of a child is
    the parent's id, GetChild of a parent is its first child, GetNext is the
    next sibling (or the next root) and GetPrev the one before. A missing
    relation answers zero.
    """
    _control, table = the_tree(number)
    if table is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    by_id = table["by_id"]
    entry = by_id.get(item_id)
    if entry is None:
        answer({"ok": True, "value": 0})
        return
    if relation == "parent":
        answer({"ok": True, "value": entry["parent"]})
        return
    if relation == "child":
        for other in by_id.values():
            if other["parent"] == item_id:
                answer({"ok": True, "value": other["order"]})
                return
        answer({"ok": True, "value": 0})
        return
    # Next and previous among the items with the same parent.
    siblings = sorted(
        (other for other in by_id.values() if other["parent"] == entry["parent"]),
        key=lambda held: held["order"],
    )
    places = [other["order"] for other in siblings]
    at = places.index(item_id)
    if relation == "next" and at + 1 < len(places):
        answer({"ok": True, "value": places[at + 1]})
        return
    if relation == "prev" and at > 0:
        answer({"ok": True, "value": places[at - 1]})
        return
    # Past the last sibling, GetNext continues with the next root item.
    if relation == "next":
        roots = sorted(
            (other for other in by_id.values() if other["parent"] == 0),
            key=lambda held: held["order"],
        )
        for other in roots:
            if other["order"] > item_id:
                answer({"ok": True, "value": other["order"]})
                return
    answer({"ok": True, "value": 0})


def the_tree_selection(number):
    """The id of the selected item, or zero. `tv.GetSelection` of v2."""
    control, table = the_tree(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    selected = control.get_selection().get_selected()[1]
    if selected is None:
        answer({"ok": True, "value": 0})
        return
    item_id = control.get_model().get_value(selected, 1)
    answer({"ok": True, "value": item_id})


def the_tree_modify(number, item_id, options):
    """Select an item (and optionally expand it). `tv.Modify` of v2."""
    control, table = the_tree(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    row = the_tree_iter(table, item_id)
    if row is None:
        answer({"ok": False, "error": "there is no item %d" % item_id})
        return
    if "select" in options.lower():
        control.get_selection().select_iter(row)
    if "expand" in options.lower():
        control.expand_to_path(control.get_model().get_path(row))
    answer({"ok": True, "value": item_id})


def the_tree_delete(number, item_id):
    """Remove an item. `tv.Delete` of v2."""
    control, table = the_tree(number)
    if control is None:
        answer({"ok": False, "error": "%d is not a tree view" % number})
        return
    entry = table["by_id"].pop(item_id, None)
    if entry is None:
        answer({"ok": True, "value": 1})
        return
    control.get_model().remove(entry["iter"])
    answer({"ok": True, "value": 1})


def the_second_value(number, setting):
    """The second, independent text of a control.

    A Hotkey keeps `Value` and `Text` apart; a Link's `Value` is always empty.
    Measured on the binary of v2.0.19 on 2026-11-10. The other controls have
    no such slot and their Value and Text are the same text.
    """
    held = state.setdefault("second_values", {})
    if setting is None:
        answer({"ok": True, "value": held.get(number, "")})
        return
    held[number] = setting
    answer({"ok": True, "value": setting})


def the_control_pos(number):
    """The place and the size of a control, relative to the window's client area.

    `ControlGetPos` of v2. Measured on the binary of v2.0.19 on 2026-11-10
    with four VarRef outputs: `ControlGetPos &X, &Y, &W, &H, "Edit1", title`
    answers the four numbers. The earlier note that the reference HANGS on
    this name was one call shape -- passing plain strings instead of VarRefs.

    GTK's allocation is the widget's own place in the box of the window,
    which is exactly the client-area-relative rectangle v2 describes.
    """
    for held, _kind, control in state["controls"]:
        if held != number:
            continue
        place = control.get_allocation()
        answer({
            "ok": True,
            "x": place.x,
            "y": place.y,
            "w": place.width,
            "h": place.height,
        })
        return
    answer({"ok": False, "error": "there is no control with the number %d" % number})


def the_menu_bar(items):
    """Show a menu bar at the top of the window. `Gui.Menu :=` of v2.

    Measured on the binary of v2.0.19 on 2026-11-10: a MenuBar attached with
    `Gui.Menu :=` shows at the top of the window. Each entry is
    `[name, checked, enabled, callback?]`: the check mark and the sensitivity
    come from `Check`/`Enable` of the runtime, and choosing an item is an
    event the runtime reads from the same channel as a click on a control.
    """
    if state["box"] is None:
        answer({"ok": False, "error": "there is no window to put a menu bar on"})
        return
    if state.get("menubar") is None:
        bar = Gtk.MenuBar()
        state["box"].pack_start(bar, False, False, 0)
        state["box"].reorder_child(bar, 0)
        bar.show()
        state["menubar"] = bar
    else:
        bar = state["menubar"]
        for child in bar.get_children():
            bar.remove(child)
    for entry in items:
        name = entry[0]
        checked = bool(entry[1]) if len(entry) > 1 else False
        enabled = bool(entry[2]) if len(entry) > 2 else True
        item = Gtk.MenuItem(label=name)
        item.set_sensitive(enabled)
        if checked:
            # A GTK menu item has no check mark of its own in a MenuBar; the
            # state is kept in the entry and the label carries the mark the
            # way Windows draws it, so the check is visible.
            item.set_label("✓ " + name)
        if len(entry) > 3:
            held = name
            item.connect(
                "activate",
                lambda *_args, held=held: an_event("menu:%s" % held),
            )
        item.show()
        bar.append(item)
    answer({"ok": True})


def run_on_loop(what):
    GLib.idle_add(what)
    return False


def handle(command):
    op = command.get("op", "")
    if op == "window":
        run_on_loop(lambda: the_window(command.get("title", "ahk-gui")))
    elif op == "visibility":
        run_on_loop(
            lambda: the_state_of(
                command.get("classnn", ""), "visible", command.get("on", True)
            )
        )
    elif op == "controls":
        run_on_loop(the_controls)
    elif op == "all-text":
        run_on_loop(the_text_of_all)
    elif op == "opacity":
        run_on_loop(lambda: the_opacity(command.get("on")))
    elif op == "paste":
        run_on_loop(lambda: the_paste(command.get("id", 0), command.get("text", "")))
    elif op == "checked":
        run_on_loop(lambda: the_checked(command.get("id", 0), command.get("on")))
    elif op == "scale":
        run_on_loop(the_scale)
    elif op == "state":
        run_on_loop(
            lambda: the_state_of(
                command.get("classnn", ""),
                command.get("what", "visible"),
                command.get("on", True),
            )
        )
    elif op == "resolve":
        run_on_loop(lambda: the_number_named(command.get("classnn", "")))
    elif op == "focused":
        run_on_loop(the_focused)
    elif op == "selection":
        run_on_loop(lambda: the_selection(command.get("id", 0)))
    elif op == "cursor":
        run_on_loop(lambda: the_cursor(command.get("id", 0)))
    elif op == "lines":
        run_on_loop(lambda: the_line_count(command.get("id", 0)))
    elif op == "option":
        run_on_loop(
            lambda: the_option(command.get("name", ""), command.get("on", True))
        )
    elif op == "hide":
        run_on_loop(the_hide)
    elif op == "focus":
        run_on_loop(lambda: the_focus(command.get("id", 0)))
    elif op == "size":
        run_on_loop(
            lambda: the_size(
                command.get("id", 0), command.get("w", -1), command.get("h", -1)
            )
        )
    elif op == "watch":
        run_on_loop(lambda: the_watch(command.get("path", "")))
    elif op == "geometry":
        run_on_loop(the_geometry)
    elif op == "tooltip":
        run_on_loop(
            lambda: the_tooltip(
                command.get("text", ""), command.get("x", 0), command.get("y", 0)
            )
        )
    elif op == "window-move":
        run_on_loop(
            lambda: the_window_move(
                command.get("x", 0),
                command.get("y", 0),
                command.get("w", 0),
                command.get("h", 0),
            )
        )
    elif op == "combo-items":
        run_on_loop(lambda: the_combo_items(command.get("id", 0)))
    elif op == "combo-add":
        run_on_loop(
            lambda: the_combo_add(command.get("id", 0), command.get("text", ""))
        )
    elif op == "combo-select":
        run_on_loop(
            lambda: the_combo_select(
                command.get("id", 0),
                command.get("wanted", ""),
                command.get("by_text", False),
            )
        )
    elif op == "combo-choice":
        run_on_loop(lambda: the_combo_choice(command.get("id", 0)))
    elif op == "combo-delete":
        run_on_loop(
            lambda: the_combo_delete(command.get("id", 0), command.get("wanted", ""))
        )
    elif op == "combo-find":
        run_on_loop(
            lambda: the_combo_find(command.get("id", 0), command.get("text", ""))
        )
    elif op == "combo-dropdown":
        run_on_loop(
            lambda: the_combo_dropdown(
                command.get("id", 0), command.get("show", True)
            )
        )
    elif op == "status-text":
        run_on_loop(the_status_text)
    elif op == "list-add":
        run_on_loop(
            lambda: the_list_add(command.get("id", 0), command.get("cells", []))
        )
    elif op == "list-count":
        run_on_loop(lambda: the_list_count(command.get("id", 0)))
    elif op == "list-text":
        run_on_loop(
            lambda: the_list_text(
                command.get("id", 0), command.get("row", 0), command.get("col", 0)
            )
        )
    elif op == "list-content":
        run_on_loop(
            lambda: the_list_content(
                command.get("id", 0), command.get("selected", False)
            )
        )
    elif op == "list-modify":
        run_on_loop(
            lambda: the_list_modify(
                command.get("id", 0), command.get("row", 0), command.get("cells", [])
            )
        )
    elif op == "list-next":
        run_on_loop(
            lambda: the_list_next(command.get("id", 0), command.get("start", 0))
        )
    elif op == "tree-add":
        run_on_loop(
            lambda: the_tree_add(
                command.get("id", 0),
                command.get("text", ""),
                command.get("parent", 0),
                command.get("options", ""),
            )
        )
    elif op == "tree-count":
        run_on_loop(lambda: the_tree_count(command.get("id", 0)))
    elif op == "tree-text":
        run_on_loop(
            lambda: the_tree_text(command.get("id", 0), command.get("item", 0))
        )
    elif op == "tree-relation":
        run_on_loop(
            lambda: the_tree_relation(
                command.get("id", 0),
                command.get("item", 0),
                command.get("relation", ""),
            )
        )
    elif op == "tree-selection":
        run_on_loop(lambda: the_tree_selection(command.get("id", 0)))
    elif op == "tree-modify":
        run_on_loop(
            lambda: the_tree_modify(
                command.get("id", 0),
                command.get("item", 0),
                command.get("options", ""),
            )
        )
    elif op == "tree-delete":
        run_on_loop(
            lambda: the_tree_delete(command.get("id", 0), command.get("item", 0))
        )
    elif op == "second-value":
        run_on_loop(
            lambda: the_second_value(command.get("id", 0), command.get("text"))
        )
    elif op == "control-pos":
        run_on_loop(lambda: the_control_pos(command.get("id", 0)))
    elif op == "menu-bar":
        run_on_loop(lambda: the_menu_bar(command.get("items", [])))
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
