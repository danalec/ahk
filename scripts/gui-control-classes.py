"""Read the classes of the controls of a window, from the machine.

A control of a script of the v1 is a window of the machine, and the class
of it is the class of that window. This tool runs a script that makes a
window with controls, and prints the class of every control of it.

The instrument of the measurement of the classes of the controls of the v1
(the class of the `Link` is `SysLink`, and the tree asked for
`SysLink32`, that the machine does not have).

    python scripts/gui-control-classes.py \
        --exe "C:\\Program Files\\AutoHotkey\\v1.1.37.02\\AutoHotkeyU64.exe" \
        --script scripts/v1-probes/probe-gui-kinds.ahk --title TheControls
"""

from __future__ import annotations

import argparse
import ctypes
import ctypes.wintypes as wintypes
import os
import subprocess
import sys
import time

user32 = ctypes.WinDLL("user32", use_last_error=True)

user32.GetClassNameW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetClassNameW.restype = ctypes.c_int
user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetWindowTextW.restype = ctypes.c_int
user32.IsWindowVisible.argtypes = [wintypes.HWND]
user32.IsWindowVisible.restype = wintypes.BOOL

ENUMPROC = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)


def the_windows_of(pid: int):
    """The windows of a process, with their class, their title and their state."""
    found = []

    def visit(window, _):
        owner = wintypes.DWORD()
        user32.GetWindowThreadProcessId(window, ctypes.byref(owner))
        if owner.value == pid:
            text = ctypes.create_unicode_buffer(512)
            user32.GetWindowTextW(window, text, 512)
            name = ctypes.create_unicode_buffer(256)
            user32.GetClassNameW(window, name, 256)
            found.append(
                (window, name.value, text.value, bool(user32.IsWindowVisible(window)))
            )
        return True

    user32.EnumWindows(ENUMPROC(visit), 0)
    return found


def the_children_of(window: int):
    """The children of a window, with their class and their text."""
    found = []

    def visit(child, _):
        name = ctypes.create_unicode_buffer(256)
        user32.GetClassNameW(child, name, 256)
        text = ctypes.create_unicode_buffer(512)
        user32.GetWindowTextW(child, text, 512)
        found.append((child, name.value, text.value))
        return True

    user32.EnumChildWindows(window, ENUMPROC(visit), 0)
    return found


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--exe", required=True, help="the binary that runs the script")
    parser.add_argument("--script", required=True, help="the script that makes the window")
    parser.add_argument("--dir", default=".", help="the folder of the run")
    parser.add_argument("--dialect", default="", help="the dialect of the runner")
    parser.add_argument("--title", default="TheControls", help="the title of the window")
    parser.add_argument("--wait", type=float, default=30.0, help="the seconds to wait")
    args = parser.parse_args()

    command = [args.exe]
    if args.dialect:
        command.append(f"--dialect={args.dialect}")
    command.append(args.script)

    handle = open(os.devnull, "wb")
    process = subprocess.Popen(
        command, cwd=args.dir, stdout=handle, stderr=subprocess.STDOUT
    )
    try:
        target = 0
        deadline = time.time() + args.wait
        while time.time() < deadline and not target:
            if process.poll() is not None:
                print("the script ended before the window appeared")
                break
            for held, name, text, visible in the_windows_of(process.pid):
                if text == args.title:
                    target = held
                    break
            time.sleep(0.1)
        if not target:
            print(f"no window with the title {args.title!r}")
            for held, name, text, visible in the_windows_of(process.pid):
                print(f"  0x{held:X} class={name!r} visible={visible} title={text!r}")
            return 1
        print(f"the window 0x{target:X} has the title {args.title!r}")
        for child, name, text in the_children_of(target):
            print(f"  0x{child:X} class={name!r} text={text!r}")
        return 0
    finally:
        if process.poll() is None:
            process.kill()
        try:
            handle.close()
        except Exception:
            pass


if __name__ == "__main__":
    sys.exit(main())
