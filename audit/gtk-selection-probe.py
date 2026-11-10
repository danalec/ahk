import gi

gi.require_version("Gtk", "3.0")
from gi.repository import Gtk

entry = Gtk.Entry()
entry.set_text("some text")
print("after set_text:  bounds =", entry.get_selection_bounds(), " position =", entry.get_position())

entry.select_region(0, 0)
print("after select_region(0,0): bounds =", entry.get_selection_bounds())

entry.set_position(-1)
print("after set_position(-1):   bounds =", entry.get_selection_bounds(), " position =", entry.get_position())

entry.set_text("second")
print("after a second set_text:  bounds =", entry.get_selection_bounds())
print("has select_region:", hasattr(entry, "select_region"))
