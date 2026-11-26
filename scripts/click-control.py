"""Click a control of a window of a script, from the outside.

The instrument of the click: it starts the script with the runner that
is given, waits for the window of the script, finds the control of the
name that is given, and clicks it with the mouse of the machine.

    python scripts/click-control.py --exe target/release/ahk.exe --dialect=v1
        --dir C:/path/of/the/app --script probe.ahk --control TheButton
"""

from __future__ import annotations

import argparse
import ctypes
import ctypes.wintypes as wintypes
import subprocess
import sys
import time

user32 = ctypes.WinDLL("user32", use_last_error=True)

ENUMPROC = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)

user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetWindowTextW.restype = ctypes.c_int
user32.GetClassNameW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetClassNameW.restype = ctypes.c_int
user32.IsWindowVisible.argtypes = [wintypes.HWND]
user32.IsWindowVisible.restype = wintypes.BOOL
user32.GetWindowRect.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.RECT)]
user32.GetWindowRect.restype = wintypes.BOOL
user32.SetCursorPos.argtypes = [ctypes.c_int, ctypes.c_int]
user32.SetCursorPos.restype = wintypes.BOOL
user32.mouse_event.argtypes = [
    wintypes.DWORD,
    wintypes.DWORD,
    wintypes.DWORD,
    wintypes.DWORD,
    ctypes.c_void_p,
]
user32.mouse_event.restype = None
user32.SetForegroundWindow.argtypes = [wintypes.HWND]
user32.SetForegroundWindow.restype = wintypes.BOOL

MOUSEEVENTF_LEFTDOWN = 0x0002
MOUSEEVENTF_LEFTUP = 0x0004


def the_windows_of(pid: int) -> list[tuple[int, str, str, bool]]:
    found: list[tuple[int, str, str, bool]] = []

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


def the_children_of(window: int) -> list[tuple[int, str, str]]:
    found: list[tuple[int, str, str]] = []

    def visit(child, _):
        text = ctypes.create_unicode_buffer(512)
        user32.GetWindowTextW(child, text, 512)
        name = ctypes.create_unicode_buffer(256)
        user32.GetClassNameW(child, name, 256)
        found.append((child, name.value, text.value))
        return True

    user32.EnumChildWindows(window, ENUMPROC(visit), 0)
    return found


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--exe", required=True)
    parser.add_argument("--script", required=True)
    parser.add_argument("--dir", default=".")
    parser.add_argument("--dialect", default="")
    parser.add_argument("--control", default="")
    parser.add_argument("--wait", type=float, default=25.0)
    args = parser.parse_args()

    command = [args.exe]
    if args.dialect:
        command.append(f"--dialect={args.dialect}")
    command.append(args.script)

    process = subprocess.Popen(
        command, cwd=args.dir, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    try:
        window = None
        deadline = time.time() + args.wait
        while time.time() < deadline and window is None:
            for held, name, text, visible in the_windows_of(process.pid):
                if name == "AutoHotkeyGUI" and visible:
                    window = held
                    break
            time.sleep(0.4)
        if window is None:
            print("no window of the script")
            return 1
        print(f"the window of the script: 0x{window:X}")
        target = None
        for child, name, text in the_children_of(window):
            print(f"  0x{child:X} class={name!r} text={text!r}")
            if args.control and text == args.control:
                target = child
        if target is None:
            print("the control was not found")
            return 1
        rectangle = wintypes.RECT()
        user32.GetWindowRect(target, ctypes.byref(rectangle))
        x = (rectangle.left + rectangle.right) // 2
        y = (rectangle.top + rectangle.bottom) // 2
        print(f"the control is at ({x},{y})")
        user32.SetForegroundWindow(window)
        time.sleep(0.4)
        user32.SetCursorPos(x, y)
        time.sleep(0.3)
        user32.mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, None)
        time.sleep(0.08)
        user32.mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, None)
        time.sleep(1.0)
        return 0
    finally:
        process.kill()
        try:
            process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            pass


if __name__ == "__main__":
    sys.exit(main())
