#!/usr/bin/python3
"""A window with a named control, so the control family has something to read.

GTK3 publishes the names and the texts of its controls to the accessibility bus;
GTK4 on the box publishes the window and leaves everything inside it unnamed.
This is the fixture that a Control* command can be measured against.
"""
import gi

gi.require_version("Gtk", "3.0")
from gi.repository import Gtk, GLib  # noqa: E402

window = Gtk.Window(title="ahk-gtk-probe")
window.set_default_size(320, 200)

box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
label = Gtk.Label(label="hello control")
box.pack_start(label, True, True, 0)
entry = Gtk.Entry()
entry.set_text("entry value")
box.pack_start(entry, False, False, 0)
# A check button, so the family of the checked state has something to read and
# to toggle.
check = Gtk.CheckButton(label="tick me")
box.pack_start(check, False, False, 0)
window.add(box)

window.connect("destroy", Gtk.main_quit)
# The window goes away on its own, so a forgotten fixture does not outlive the
# measurement.
GLib.timeout_add_seconds(90, Gtk.main_quit)
window.show_all()
Gtk.main()
