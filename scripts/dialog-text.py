"""Read the text of the dialogs of a process.

The instrument of the errors of a script: it starts the script with the
runner that is given, waits for a dialog of it, and prints the text of
the dialog and of the controls of it.

    python scripts/dialog-text.py --exe target/release/ahk.exe --dialect=v1
        --dir C:/path/of/the/app --script probe.ahk
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

user32.EnumWindows.argtypes = [ENUMPROC, wintypes.LPARAM]
user32.EnumChildWindows.argtypes = [wintypes.HWND, ENUMPROC, wintypes.LPARAM]
user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetWindowTextW.restype = ctypes.c_int
user32.GetClassNameW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetClassNameW.restype = ctypes.c_int
user32.IsWindowVisible.argtypes = [wintypes.HWND]
user32.IsWindowVisible.restype = wintypes.BOOL
user32.GetWindowThreadProcessId.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.DWORD)]


def the_text_of(window: int) -> str:
    buffer = ctypes.create_unicode_buffer(1024)
    user32.GetWindowTextW(window, buffer, 1024)
    return buffer.value


def the_class_of(window: int) -> str:
    buffer = ctypes.create_unicode_buffer(256)
    user32.GetClassNameW(window, buffer, 256)
    return buffer.value


def the_dialogs_of(pid: int) -> list[int]:
    found: list[int] = []

    def visit(window, _):
        owner = wintypes.DWORD()
        user32.GetWindowThreadProcessId(window, ctypes.byref(owner))
        if owner.value == pid and the_class_of(window) == "#32770":
            found.append(window)
        return True

    user32.EnumWindows(ENUMPROC(visit), 0)
    return found


def the_children_of(window: int) -> list[int]:
    found: list[int] = []

    def visit(child, _):
        found.append(child)
        return True

    user32.EnumChildWindows(window, ENUMPROC(visit), 0)
    return found


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--exe", required=True)
    parser.add_argument("--script", required=True)
    parser.add_argument("--dir", default=".")
    parser.add_argument("--dialect", default="")
    parser.add_argument("--wait", type=float, default=20.0)
    args = parser.parse_args()

    command = [args.exe]
    if args.dialect:
        command.append(f"--dialect={args.dialect}")
    command.append(args.script)

    process = subprocess.Popen(
        command, cwd=args.dir, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    try:
        deadline = time.time() + args.wait
        while time.time() < deadline:
            dialogs = the_dialogs_of(process.pid)
            if dialogs:
                for dialog in dialogs:
                    print(f"the dialog 0x{dialog:X}: {the_text_of(dialog)!r}")
                    for child in the_children_of(dialog):
                        print(f"  {the_class_of(child)}: {the_text_of(child)!r}")
                return 0
            if process.poll() is not None:
                break
            time.sleep(0.5)
        print("no dialog")
        return 1
    finally:
        process.kill()
        try:
            out, _ = process.communicate(timeout=5)
        except subprocess.TimeoutExpired:
            out = b""
        print((out or b"").decode("utf-8", "replace")[-3000:])


if __name__ == "__main__":
    sys.exit(main())
