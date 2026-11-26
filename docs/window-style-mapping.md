# The window style, the bits, and the machines that carry them

The style of a window is a **number of Windows**. `WinGetStyle` reads it,
`WinGetExStyle` reads the extended one, `WinSetStyle` and `WinSetExStyle`
write them, and the `Control*` names do the same for a control -- which on
Windows is a window of its own. The numbers are the `WS_*` and `WS_EX_*` of
the machine, and a window that has none of them answers zero.

The same four questions -- and the two of the transparent colour,
`WinGetTransColor` and `WinSetTransColor` -- are asked of this tree on the
machines that are not Windows. **The X11 has no style number**: it has
attributes, kept by the window manager and by the server, and the decoration
of a window belongs to the compositor and not to the client. The tree
therefore carries a **mapping**, and the mapping is a choice of this tree.
It is written down here so that a script knows what it reads and what it
writes, and so that the choice is not a secret buried in the code.

The code is `crates/ahk/src/window_style.rs` (the bits, the text of a style,
the colours), and the reading of the X11 attributes is the layer of
`crates/ahk-box/src/x11.rs` (`WindowFacts`, `style_bits`, `plan_style_change`).
The measurements are on the binary of AutoHotkey **v2.0.31**
(`%TEMP%\ahk-v2.0.32-fresh\release\AutoHotkey64.exe`, from
<https://www.autohotkey.com/download/ahk-v2.zip>) and on its source at
`refs/AutoHotkey-2.0.31`,.

The binary's own numbers, measured on that day: the style of the script's
hidden window is `80674816` = `0x04CF0000` -- `WS_OVERLAPPEDWINDOW` with the
`WS_CLIPSIBLINGS` that the machine adds by itself -- and its extended style
is `256` = `WS_EX_WINDOWEDGE`, not `WS_EX_TOOLWINDOW` (128): the script's
window carries no tool-window bit unless the front window is the task bar.

## The bits the tree names

The numbers are the ones of the Windows headers, and both dialects read and
write the same numbers. The left column is the style (`WS_*`), the right the
extended style (`WS_EX_*`).

| Bit | Number | What it says | The X11 attribute that answers it |
|---|---|---|---|
| `WS_VISIBLE` | 0x10000000 | the window is on the screen | the map state is `IsViewable` (`XGetWindowAttributes`) |
| `WS_MINIMIZE` | 0x20000000 | the window is minimized | `_NET_WM_STATE` holds `_NET_WM_STATE_HIDDEN` |
| `WS_MAXIMIZE` | 0x01000000 | the window is maximized | `_NET_WM_STATE` holds `MAXIMIZED_VERT` **and** `MAXIMIZED_HORZ` |
| `WS_CHILD` | 0x40000000 | the window is inside another | the window has a parent that is not the root |
| `WS_POPUP` | 0x80000000 | the window stands on its own | override-redirect, or the parent is the root |
| `WS_EX_TOPMOST` | 0x00000008 | the window is above the others | `_NET_WM_STATE` holds `_NET_WM_STATE_ABOVE` |
| `WS_EX_LAYERED` | 0x00080000 | the window lets something through | `_NET_WM_WINDOW_OPACITY` is set |
| `WS_EX_TRANSPARENT` | 0x00000020 | clicks go through the window | **no equivalent**: the X11 does not express it |

Two rules the mapping follows and that a reader should know:

- **A top-level window is never style zero.** On Windows a window with no
  bits is possible; under X11 every window either stands on its own
  (`WS_POPUP`) or lives inside another (`WS_CHILD`), so the style of a
  top-level window always carries `WS_POPUP`, and the extended style is the
  one that may be zero.
- **The root is not a parent.** Every window under X11 has a parent -- the
  root for the top-level ones -- and counting it as a mother would make
  every window `WS_CHILD`. The root of the screen is therefore no mother,
  and a window whose parent is the root answers `WS_POPUP`.

## Writing the style

`WinSetStyle` and its cousins are **not** a raw write: the text says what to
do with the style that is there. The table is the one of v2 (`lib/win.cpp`
`WinSetStyle`, measured on the binary):

| The text says | It means | Written as |
|---|---|---|
| `"0x10"` | replace the style with this number | `Replace(0x10)` |
| `"+0x10"` | add these bits to the style that is there | `current \| 0x10` |
| `"-0x10"` | take these bits away from the style that is there | `current & !0x10` |
| `"^0x10"` | turn these bits around in the style that is there | `current ^ 0x10` |
| `""` | an empty text is refused | "Parameter #1 of … is invalid." with "empty string" in the `Extra` |

The number inside the text is read the way the `ATOU` of v2 reads it
(`util.h:499`, `nstrtoi64`): **hexadecimal only behind `0x`**, decimal
otherwise, and zero when the text holds no number at all --
`WinSetStyle("zzz")` is not a refusal, it writes zero.

On Windows the write is `SetWindowLongPtrW` followed by `SetWindowPos` with
`SWP_DRAWFRAME|SWP_FRAMECHANGED|SWP_NOMOVE|SWP_NOSIZE|SWP_NOZORDER|SWP_NOACTIVATE`
and `InvalidateRect` -- the frame is drawn again by the machine, the way v2
does it. The write is **read back and checked**, as v2 checks it
(`lib/win.cpp:1821-1837`): a style that did not change is a success without
a call, and a style that did change must be seen by a second read.

On X11 the write applies **only what the attributes can express**, and
refuses with the reason what they cannot:

| Bit asked for | What the tree does on X11 |
|---|---|
| `WS_VISIBLE` on or off | `XMapWindow` / `XUnmapWindow` |
| `WS_EX_TOPMOST` on or off | the client message `_NET_WM_STATE` with `_NET_WM_STATE_ABOVE` |
| `WS_MAXIMIZE` on or off | the client message with `MAXIMIZED_VERT` and `MAXIMIZED_HORZ` |
| `WS_MINIMIZE` on or off | the ICCCM `WM_CHANGE_STATE` with `IconicState` / `NormalState` |
| `WS_EX_LAYERED` on | `_NET_WM_WINDOW_OPACITY` set to the full |
| `WS_EX_LAYERED` off | the property deleted (`XDeleteProperty`) |
| `WS_EX_TRANSPARENT` | **refused with the reason** -- the X11 cannot say it |
| `WS_CHILD` / `WS_POPUP` | **refused with the reason** -- the mother of a window is decided when it is created |

The refusal comes **before** any change: a write that asks for what the
machine cannot do does nothing half, and says so. That is the rule of the
tree -- fail closed, never silent -- and the KeySharp audit named the silent
`+Theme` no-ops of its own dialect as the bug class this rule tests against.

## The colour a window lets through

`WinGetTransColor` reads the **colour key** of a layered window on Windows
(`GetLayeredWindowAttributes`, `LWA_COLORKEY`), and answers it as
`"0xRRGGBB"` in the red-green order; a window that is not layered has no
colour key and answers the **empty text** -- measured on the binary where the window of the script answered `""`.

`WinSetTransColor` takes the colour as a number, as one of the seventeen HTML
names (`"Red"`, `"Aqua"`, …), or as the word `"Off"`, which takes the key
away. On Windows the write is the `CLR_NONE` path of v2: the colour goes and
`WS_EX_LAYERED` goes with it, because the machine forgets the transparency
the moment the bit leaves.

**X11 answers the empty text and refuses the write.** The transparency of a
window of the X11 is the `_NET_WM_WINDOW_OPACITY` of the whole window --
the alpha, not one colour made invisible -- and answering a colour would be
answering a different question than the one the script asked. The name of
the colour is refused with that reason.

## Why the mapping and not a refusal

The audit (`egm-review/keysharp-audit/audit-keysharp-vs-ahk.md` §5.4) counts
these names among the refusals of the tree that a Linux machine *could*
answer in some form. The pure refusal -- "this is a name of Windows" -- was
true of the decoration and false of the state: the map of a window, the
above of a window, the opacity of a window **are** facts of the X11, and
hiding them behind one sentence was the tree saying less than it knows.
The mapping answers those facts with the numbers a Windows script expects;
the places where the two machines truly differ -- the mother of a window,
the click-through, the colour key -- keep the refusal and say the reason.

A portable script must know the price: **a style number read on X11 is not
the style number read on Windows**. The mapping is what makes it readable,
and this page is what makes it honest.
