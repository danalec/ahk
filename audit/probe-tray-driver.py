#!/usr/bin/python3
"""The helper of the tray, driven by hand, with its error visible.

The helper is started the way the runtime starts it -- with a pipe on its
standard input and the file of the events in its environment -- and the
click is made through its bus, so the traceback of the helper is seen
instead of a "Remote peer disconnected" from the bus.
"""

import json
import os
import subprocess
import time

HELPER = os.path.expanduser("~/ahk/crates/ahk-box/helper/ahk-tray.py")
EVENTS = "/tmp/tray-events-diag.txt"

if os.path.exists(EVENTS):
    os.remove(EVENTS)

environment = dict(os.environ)
environment["AHK_EVENTS_FILE"] = EVENTS

helper = subprocess.Popen(
    ["python3", HELPER],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    env=environment,
)
print("the pid of the helper: %d" % helper.pid)
print("the answer of the start: %s" % helper.stdout.readline().decode().strip())

helper.stdin.write(json.dumps({"op": "icon", "name": "application-x-executable"}).encode() + b"\n")
helper.stdin.flush()
print("the answer of the icon: %s" % helper.stdout.readline().decode().strip())

# The name of the bus is only owned when the loop of the helper runs, and
# the loop runs while it waits for the next command.
time.sleep(1)

name = "org.khronos.ahk.tray.p%d" % helper.pid
print("--- the name of the item is owned? ---")
listed = subprocess.run(
    ["gdbus", "call", "--session", "--dest", "org.freedesktop.DBus",
     "--object-path", "/org/freedesktop/DBus",
     "--method", "org.freedesktop.DBus.ListNames"],
    capture_output=True, text=True,
)
print("listed: %s" % ("yes" if name in listed.stdout else "no"))

print("--- the click ---")
click = subprocess.run(
    ["gdbus", "call", "--session", "--dest", name, "--object-path", "/StatusNotifierItem",
     "--method", "org.kde.StatusNotifierItem.Activate", "0", "0"],
    capture_output=True, text=True,
)
print("the click said: %s %s" % (click.stdout.strip(), click.stderr.strip()))

time.sleep(1)
print("--- the events file ---")
print(open(EVENTS).read() if os.path.exists(EVENTS) else "(there is none)")
print("--- the helper is alive? ---")
print("yes" if helper.poll() is None else "no: it died with %s" % helper.poll())

helper.stdin.write(b'{"op":"quit"}\n')
helper.stdin.flush()
time.sleep(0.5)
helper.terminate()
