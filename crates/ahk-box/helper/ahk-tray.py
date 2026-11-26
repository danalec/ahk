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

**The loop of the helper is the loop of the bus.** The watcher asks this
process for the properties of the item, and it asks it by the bus -- so a
helper that waits for the commands of the runner in a blocking read never
answers the watcher, and the click on the icon is never delivered. The
commands of the runner are therefore read by the loop itself, and the
answer of the start is written when the item is in the watcher.
"""

import json
import os
import sys

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
    <property name='Menu' type='o' access='read'/>
  </interface>
</node>
"""

# The menu of the tray: the com.canonical.dbusmenu that the watcher draws
# when the right button of the mouse asks for the menu of the item. It
# carries one item, Exit, and the click on it is written to the file of
# the events of the run.
DBUSMENU_IFACE = """
<node>
  <interface name='com.canonical.dbusmenu'>
    <property name='Version' type='u' access='read'/>
    <property name='Status' type='s' access='read'/>
    <property name='TextDirection' type='s' access='read'/>
    <property name='IconThemePath' type='as' access='read'/>
    <method name='GetLayout'>
      <arg name='parentId' type='i' direction='in'/>
      <arg name='recursionDepth' type='i' direction='in'/>
      <arg name='propertyNames' type='as' direction='in'/>
      <arg name='revision' type='u' direction='out'/>
      <arg name='layout' type='(ia{sv}av)' direction='out'/>
    </method>
    <method name='GetGroupProperties'>
      <arg name='ids' type='ai' direction='in'/>
      <arg name='propertyNames' type='as' direction='in'/>
      <arg name='properties' type='a(ia{sv})' direction='out'/>
    </method>
    <method name='GetProperty'>
      <arg name='id' type='i' direction='in'/>
      <arg name='name' type='s' direction='in'/>
      <arg name='value' type='v' direction='out'/>
    </method>
    <method name='Event'>
      <arg name='id' type='i' direction='in'/>
      <arg name='eventId' type='s' direction='in'/>
      <arg name='data' type='v' direction='in'/>
      <arg name='timestamp' type='u' direction='in'/>
    </method>
    <method name='EventGroup'>
      <arg name='events' type='a(isvu)' direction='in'/>
      <arg name='idErrors' type='ai' direction='out'/>
    </method>
    <method name='AboutToShow'>
      <arg name='id' type='i' direction='in'/>
      <arg name='needUpdate' type='b' direction='out'/>
    </method>
    <method name='AboutToShowGroup'>
      <arg name='ids' type='ai' direction='in'/>
      <arg name='updatesNeeded' type='ai' direction='out'/>
      <arg name='idErrors' type='ai' direction='out'/>
    </method>
  </interface>
</node>
"""

# The name of the bus of this process: the watcher asks it for the
# properties of the item. The name carries the number of the process, so
# that two runs of a script do not fight over it -- the one that came
# first would answer for the one that came second. An element of a name
# of a bus may not begin with a digit, so the number carries a letter.
BUS_NAME = "org.khronos.ahk.tray.p%d" % os.getpid()

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

# The loop of the helper, and the commands that were read but not answered
# yet: a command that arrives in pieces is kept until it is whole.
LOOP = GLib.MainLoop()
PENDING = []


def answer(what):
    sys.stdout.write(json.dumps(what) + "\n")
    sys.stdout.flush()


def the_log(text):
    """A line of the log of the helper.

    The log goes to the standard error, which the runtime throws away: the
    helper is a program of the machine, and a program that fails says so
    where it can be read.
    """
    sys.stderr.write(text + "\n")
    sys.stderr.flush()


def the_property(connection, sender, path, interface, name):
    """The property of the item, for the watcher that asks for it.

    The answer is a **variant**: the binding cannot turn a text of Python
    into the value of a property of the bus, and the watcher is answered
    with a failure when the answer is not a variant.
    """
    try:
        if name == "ToolTip":
            return GLib.Variant("(sa(iiay)ss)", (STATE["Title"], [], "", STATE["Title"]))
        if name == "ItemIsMenu":
            return GLib.Variant("b", bool(STATE["ItemIsMenu"]))
        if name == "Menu":
            return GLib.Variant("o", "/MenuBar")
        return GLib.Variant("s", str(STATE.get(name, "")))
    except Exception as error:
        the_log("the property %s could not be answered: %r" % (name, error))
        return GLib.Variant("s", "")


def the_click(*arguments):
    """A click on the icon of the tray: the runtime is told.

    The bus calls this with the connection, the sender, the path, the
    interface, the name of the method and the parameters -- and with the
    invocation, in some versions of the binding. The arguments are taken
    as they come, and the invocation is answered when there is one.
    """
    try:
        if EVENTS:
            with open(EVENTS, "a", encoding="utf-8") as events:
                events.write("tray\n")
    except OSError:
        # The file of the events is gone: the run is over, and the click on
        # an icon of a run that is over is of no interest.
        pass
    # The method gives nothing back, and the bus wants an answer all the
    # same.
    if arguments and hasattr(arguments[-1], "return_value"):
        try:
            arguments[-1].return_value(None)
        except Exception as error:
            the_log("the click could not be answered: %r" % error)
    return None


def the_menu_property(connection, sender, path, interface, name):
    """The property of the menu, for the watcher that draws it."""
    if name == "Version":
        return GLib.Variant("u", 3)
    if name == "Status":
        return GLib.Variant("s", "normal")
    if name == "TextDirection":
        return GLib.Variant("s", "ltr")
    if name == "IconThemePath":
        return GLib.Variant("as", [])
    return GLib.Variant("s", "")


def the_exit_is_written():
    """The Exit of the menu: the run is told through the file of events."""
    try:
        if EVENTS:
            with open(EVENTS, "a", encoding="utf-8") as events:
                events.write("tray-exit\n")
    except OSError:
        pass


def the_menu_method(connection, sender, path, interface, method, params, invocation):
    """One method of the menu of the tray, answered.

    The menu carries one item -- Exit, the number 1 -- and the click on it
    is written to the file of the events of the run.
    """
    try:
        if method == "GetLayout":
            exit_child = GLib.Variant(
                "(ia{sv}av)", (1, {"label": GLib.Variant("s", "Exit")}, [])
            )
            root = GLib.Variant(
                "(ia{sv}av)",
                (0, {"children-display": GLib.Variant("s", "submenu")}, [exit_child]),
            )
            invocation.return_value(GLib.Variant("(u(ia{sv}av))", (1, root)))
        elif method == "GetGroupProperties":
            props = []
            for item_id in params[0]:
                if item_id == 1:
                    props.append(
                        GLib.Variant("(ia{sv})", (1, {"label": GLib.Variant("s", "Exit")}))
                    )
            invocation.return_value(GLib.Variant("(a(ia{sv}))", (props,)))
        elif method == "GetProperty":
            invocation.return_value(GLib.Variant("(v)", (GLib.Variant("s", "Exit"),)))
        elif method == "Event":
            if params[0] == 1 and params[1] == "clicked":
                the_exit_is_written()
            invocation.return_value(None)
        elif method == "EventGroup":
            for event in params[0]:
                if event[0] == 1 and event[1] == "clicked":
                    the_exit_is_written()
            invocation.return_value(GLib.Variant("(ai)", ([],)))
        elif method == "AboutToShow":
            invocation.return_value(GLib.Variant("(b)", (False,)))
        elif method == "AboutToShowGroup":
            invocation.return_value(GLib.Variant("(aiai)", ([], [])))
        else:
            invocation.return_value(None)
    except Exception as error:
        the_log("the menu %s could not be answered: %r" % (method, error))
        try:
            invocation.return_value(None)
        except Exception:
            pass


def the_command(line):
    """One command of the runner, answered."""
    try:
        command = json.loads(line)
    except ValueError:
        answer({"ok": False, "error": "the command is not JSON"})
        return
    op = command.get("op", "")
    if op == "icon":
        STATE["IconName"] = command.get("name", STATE["IconName"])
        if command.get("title"):
            STATE["Title"] = command["title"]
        answer({"ok": True})
    elif op == "quit":
        answer({"ok": True})
        LOOP.quit()
    else:
        answer({"ok": False, "error": "%s is not a command of the tray" % op})


def the_input_is_ready(source, condition, user_data):
    """The commands of the runner come on the standard input.

    The read is done by the loop and not by a blocking read, because the
    loop is also what answers the watcher. The binding hands the three
    arguments to the watcher -- the source, the condition and the data of
    the caller -- and the watcher wants a boolean back.
    """
    try:
        data = os.read(0, 4096)
    except OSError:
        data = b""
    if not data:
        # The runner closed the pipe: the run is over.
        LOOP.quit()
        return False
    PENDING.extend(data.split(b"\n"))
    while PENDING:
        line = PENDING.pop(0).strip()
        if not line:
            continue
        the_command(line.decode("utf-8", "replace"))
    return True


def the_name_is_ours(connection, name):
    """The item is registered when the name of the bus is ours.

    The watcher asks this process for the properties of the item, and it
    asks it by the name -- so the item is registered after the name is
    owned, and the answer of the start is written after that: an answer
    that says "ok" is the answer of an item that the watcher holds.
    """
    try:
        connection.call_sync(
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
        LOOP.quit()
        return
    answer({"ok": True})


def watch_the_input():
    """The standard input is watched by the loop.

    The binding that watches a file descriptor moved to `GLibUnix`, and the
    old name is deprecated. Both are the same watcher, and the old one is
    used when the new one is not there.
    """
    try:
        from gi.repository import GLibUnix

        return GLibUnix.fd_add_full(
            GLib.PRIORITY_DEFAULT, 0, GLib.IO_IN | GLib.IO_HUP, the_input_is_ready, None
        )
    except (ImportError, AttributeError):
        return GLib.unix_fd_add_full(
            GLib.PRIORITY_DEFAULT, 0, GLib.IO_IN | GLib.IO_HUP, the_input_is_ready, None
        )


def main():
    bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
    node = Gio.DBusNodeInfo.new_for_xml(IFACE)
    bus.register_object(
        "/StatusNotifierItem",
        node.interfaces[0],
        the_click,
        the_property,
        None,
    )
    # The menu of the tray: the watcher draws it when the right button of
    # the mouse asks for the menu of the item.
    menu_node = Gio.DBusNodeInfo.new_for_xml(DBUSMENU_IFACE)
    bus.register_object(
        "/MenuBar",
        menu_node.interfaces[0],
        the_menu_method,
        the_menu_property,
        None,
    )
    Gio.bus_own_name_on_connection(
        bus, BUS_NAME, Gio.BusNameOwnerFlags.NONE, the_name_is_ours, None
    )
    # The commands of the runner are read by the loop, and the loop is what
    # answers the watcher. The hangup is watched as well: a pipe that is
    # closed is not readable, and a helper that waits for a command that
    # will never come would stay in the session for ever.
    watch_the_input()
    LOOP.run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
