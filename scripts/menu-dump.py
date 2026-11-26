"""Dump the menu of the window of a script, from the outside.

The instrument of the menus of a script of the v1: it starts the script
with the runner that is given, waits for the window of the script, and
walks the menu that the machine holds for it -- the bar of the window and
the dropdowns that it opens -- printing the text of every item, the
number of it and the menu that it opens.

    python scripts/menu-dump.py --exe target/release/ahk.exe --dialect=v1
        --dir C:/path/of/the/app --script the-script.ahk

The answer is the behaviour of the machine, and it is the same for the
binary of the v1 and for the tree.
"""

from __future__ import annotations

import argparse
import ctypes
import ctypes.wintypes as wintypes
import subprocess
import sys
import time

user32 = ctypes.WinDLL("user32", use_last_error=True)

MIIM_STATE = 0x00000001
MIIM_ID = 0x00000002
MIIM_SUBMENU = 0x00000004
MIIM_STRING = 0x00000040
MIIM_BITMAP = 0x00000080
MIIM_FTYPE = 0x00000100


class MENUITEMINFOW(ctypes.Structure):
    _fields_ = [
        ("cbSize", wintypes.DWORD),
        ("fMask", wintypes.DWORD),
        ("fType", wintypes.UINT),
        ("fState", wintypes.UINT),
        ("wID", wintypes.UINT),
        ("hSubMenu", wintypes.HMENU),
        ("hbmpChecked", wintypes.HBITMAP),
        ("hbmpUnchecked", wintypes.HBITMAP),
        ("dwItemData", ctypes.POINTER(wintypes.ULONG)),
        ("dwTypeData", wintypes.LPWSTR),
        ("cch", wintypes.UINT),
        ("hbmpItem", wintypes.HBITMAP),
    ]


user32.GetMenuItemInfoW.argtypes = [
    wintypes.HMENU,
    wintypes.UINT,
    ctypes.c_bool,
    ctypes.POINTER(MENUITEMINFOW),
]
user32.GetMenuItemInfoW.restype = wintypes.BOOL
user32.GetMenuItemCount.argtypes = [wintypes.HMENU]
user32.GetMenuItemCount.restype = ctypes.c_int
user32.GetMenu.argtypes = [wintypes.HWND]
user32.GetMenu.restype = wintypes.HMENU
user32.GetClassNameW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetClassNameW.restype = ctypes.c_int
user32.GetWindowTextW.argtypes = [wintypes.HWND, wintypes.LPWSTR, ctypes.c_int]
user32.GetWindowTextW.restype = ctypes.c_int
user32.IsWindowVisible.argtypes = [wintypes.HWND]
user32.IsWindowVisible.restype = wintypes.BOOL
user32.GetMenuItemRect.argtypes = [
    wintypes.HWND,
    wintypes.HMENU,
    wintypes.UINT,
    ctypes.POINTER(wintypes.RECT),
]
user32.GetMenuItemRect.restype = wintypes.BOOL
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
user32.ClientToScreen.argtypes = [wintypes.HWND, ctypes.POINTER(wintypes.RECT)]
user32.ClientToScreen.restype = wintypes.BOOL

MOUSEEVENTF_LEFTDOWN = 0x0002
MOUSEEVENTF_LEFTUP = 0x0004


def the_item_is_clicked(window: int, menu: int, place: int) -> bool:
    """Put the mouse on an item of the bar of a window and click it.

    The rectangle of an item of a bar of a window is in the coordinates
    of the window, and the mouse wants the ones of the screen.
    """
    rectangle = wintypes.RECT()
    if not user32.GetMenuItemRect(window, menu, place, ctypes.byref(rectangle)):
        return False
    user32.ClientToScreen(window, ctypes.byref(rectangle))
    x = (rectangle.left + rectangle.right) // 2
    y = (rectangle.top + rectangle.bottom) // 2
    user32.SetCursorPos(x, y)
    time.sleep(0.2)
    user32.mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, None)
    time.sleep(0.05)
    user32.mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, None)
    return True

ENUMPROC = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)


def the_windows_of(pid: int) -> list[tuple[int, str, str, bool]]:
    """The windows of a process: the handle, the class, the title and the
    visibility of them."""
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


def the_item_of(menu: int, place: int) -> tuple[str, int, int] | None:
    """The text, the number and the menu that an item of a menu opens."""
    info = MENUITEMINFOW()
    info.cbSize = ctypes.sizeof(MENUITEMINFOW)
    info.fMask = MIIM_STRING | MIIM_ID | MIIM_SUBMENU | MIIM_FTYPE | MIIM_STATE
    buffer = ctypes.create_unicode_buffer(512)
    info.dwTypeData = ctypes.cast(buffer, wintypes.LPWSTR)
    info.cch = 511
    if not user32.GetMenuItemInfoW(menu, place, True, ctypes.byref(info)):
        return None
    return buffer.value, int(info.wID), int(info.hSubMenu or 0)


def the_menu_is_dumped(menu: int, depth: int, out: list[str]) -> None:
    """The items of a menu, one per line, with the menus that they open."""
    count = user32.GetMenuItemCount(menu)
    for place in range(max(count, 0)):
        held = the_item_of(menu, place)
        if held is None:
            out.append(f"{'  ' * depth}[{place}] <the machine did not answer>")
            continue
        text, number, child = held
        out.append(f"{'  ' * depth}[{place}] {text!r} id={number} submenu={child}")
        if child and depth < 4:
            the_menu_is_dumped(child, depth + 1, out)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--exe", required=True)
    parser.add_argument("--script", required=True)
    parser.add_argument("--dir", default=".")
    parser.add_argument("--dialect", default="")
    parser.add_argument("--wait", type=float, default=25.0)
    parser.add_argument("--rounds", type=int, default=1)
    parser.add_argument("--every", type=float, default=3.0)
    parser.add_argument("--click", type=int, default=-1)
    parser.add_argument("--class-name", default="AutoHotkeyGUI")
    args = parser.parse_args()

    command = [args.exe]
    if args.dialect:
        command.append(f"--dialect={args.dialect}")
    command.append(args.script)

    process = subprocess.Popen(
        command,
        cwd=args.dir,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    try:
        window = None
        deadline = time.time() + args.wait
        while time.time() < deadline:
            if process.poll() is not None:
                break
            for held, name, text, visible in the_windows_of(process.pid):
                if name == args.class_name and visible:
                    window = held
                    break
            if window is not None:
                break
            time.sleep(0.5)

        if window is None:
            print("no window of the script")
            for held, name, text, visible in the_windows_of(process.pid):
                print(f"  0x{held:X} class={name!r} visible={visible} title={text!r}")
            process.kill()
            try:
                out, _ = process.communicate(timeout=5)
            except subprocess.TimeoutExpired:
                out = b""
            print((out or b"").decode("utf-8", "replace")[-4000:])
            return 1

        print(f"the window of the script: 0x{window:X}")
        for round_number in range(args.rounds):
            menu = user32.GetMenu(window)
            print(f"--- the menu of the window: {menu}")
            if menu:
                out: list[str] = []
                the_menu_is_dumped(menu, 0, out)
                print("\n".join(out))
            else:
                print("the window has no bar of a menu")
            if round_number + 1 < args.rounds:
                time.sleep(args.every)
        if args.click >= 0:
            menu = user32.GetMenu(window)
            if menu:
                clicked = the_item_is_clicked(window, menu, args.click)
                print(f"--- the item {args.click} of the bar was clicked: {clicked}")
                time.sleep(args.every)
                menu = user32.GetMenu(window)
                out = []
                if menu:
                    the_menu_is_dumped(menu, 0, out)
                print("\n".join(out))
                for held, name, text, visible in the_windows_of(process.pid):
                    print(f"  0x{held:X} class={name!r} visible={visible} title={text!r}")
        return 0
    finally:
        process.kill()
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            pass


if __name__ == "__main__":
    sys.exit(main())
