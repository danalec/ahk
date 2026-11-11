#!/usr/bin/python3
"""The tray of a script, as the session of the box shows it.

The compositor of the box is a COSMIC, and it carries a watcher of the
StatusNotifierItem -- the protocol that a program of a desktop uses to put
an icon in the tray. This helper is that program: it registers an item in
the watcher, it carries the icon and the title of the script, and a click
on the icon is written to the file of the events of the run, where the
runtime reads it.

    in   {"op": "icon", "name": "application-x-executable"}
    out  {"ok": true}

The watcher is `org.kde.StatusNotifierWatcher`, which COSMIC answers under
that name, and the item is registered under the name of the bus of this
process, so that the watcher asks this process for the properties of it.
"""

import json
import os
import sys
import threading

import gi

gi.require_version("Gtk", "3.0")
from gi.repository import Gio, GLib  # noqa: E402

IFACE = """
<node>
  <interface name='org.kde.StatusNotifierItem'>
    <property name='Category' type='s' access='read'/>
    <property name='Id' type='s' access='read'/>
    <property name='Title' type='s' access='read'/>
    <property name='Status' type='s' access='read'/>
    <property name='IconName' type='s' access='read'/>
    <property name='ToolTip' type='(sa(iiay)ss)' access='read'/>
    <property name='ItemIsMenu' type='b' access='read'/>
    <method name='Activate'>
      <arg name='x' type='i' direction='in'/>
      <arg name='y' type='i' direction='in'/>
    </method>
    <method name='ContextMenu'>
      <arg name='x' type='i' direction='in'/>
      <arg name='y' type='i' direction='in'/>
    </method>
  </interface>
</node>
"""

# The name of the bus of this process: the watcher asks it for the
# properties of the item.
BUS_NAME = "org.khronos.ahk.tray"

# The properties of the item, as the watcher reads them.
STATE = {
    "Category": "ApplicationStatus",
    "Id": "ahk",
    "Title": "the AHK",
    "Status": "Active",
    "IconName": "application-x-executable",
    "ItemIsMenu": False,
}

# The file of the events of the run: the same one the window of the script
# writes, and the name of it comes from the environment.
EVENTS = os.environ.get("AHK_EVENTS_FILE", "")


def answer(what):
    sys.stdout.write(json.dumps(what) + "\n")
    sys.stdout.flush()


def the_property(connection, sender, path, interface, name):
    """The property of the item, for the watcher that asks for it."""
    if name == "ToolTip":
        return GLib.Variant("(sa(iiay)ss)", (STATE["Title"], [], "", STATE["Title"]))
    value = STATE.get(name, "")
    return value


def the_click(connection, sender, path, interface, method, parameters):
    """A click on the icon of the tray: the runtime is told."""
    if EVENTS:
        with open(EVENTS, "a", encoding="utf-8") as events:
            events.write("tray\n")
    answer({"ok": True, "clicked": method})


def main():
    bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
    Gio.bus_own_name_on_connection(
        bus, BUS_NAME, Gio.BusNameOwnerFlags.NONE, None, None
    )
    node = Gio.DBusNodeInfo.new_for_xml(IFACE)
    bus.register_object(
        "/StatusNotifierItem",
        node.interfaces[0],
        the_click,
        the_property,
        None,
    )
    # The watcher is asked to take the item in.
    try:
        bus.call_sync(
            "org.kde.StatusNotifierWatcher",
            "/StatusNotifierWatcher",
            "org.kde.StatusNotifierWatcher",
            "RegisterStatusNotifierItem",
            GLib.Variant("(s)", (BUS_NAME,)),
            None,
            Gio.DBusCallFlags.NONE,
            -1,
            None,
        )
    except GLib.Error as error:
        answer({"ok": False, "error": "the watcher said no: %s" % error.message})
        return 1
    answer({"ok": True})
    # The commands of the runner come on the standard input, and the icon of
    # the tray is set with them.
    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        try:
            command = json.loads(line)
        except ValueError:
            answer({"ok": False, "error": "the command is not JSON"})
            continue
        op = command.get("op", "")
        if op == "icon":
            STATE["IconName"] = command.get("name", STATE["IconName"])
            if command.get("title"):
                STATE["Title"] = command["title"]
            answer({"ok": True})
        elif op == "quit":
            answer({"ok": True})
            break
        else:
            answer({"ok": False, "error": "%s is not a command of the tray" % op})
    return 0


if __name__ == "__main__":
    sys.exit(main())
