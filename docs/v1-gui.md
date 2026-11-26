# The GUI of the v1 — the insights and the work

A script of the v1 is an AHK v1.1 script. The tree runs it on its own
runtime, and the runtime has to carry the GUI of the v1 the way the real
`AutoHotkeyU64.exe` of v1.1.37.02 carries it.

This is the list of what was measured, what the CHM says, and what the tree
has to do. Every item was confirmed against the binary of the v1.1.37.02 and
against the CHM of the same version; the measurements are in
`crates/ahk/tests/v1.rs` and the notes of the measurements are below.

The authorities are:

- the CHM of the v1.1.37.02, extracted in `/tmp/chmfull/docs`;
- the binary `C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe`;

## 1. The windows of the v1 are named, and there are many of them

The v1 gives a name to each of its windows: `Gui, PVwin: Add, ...` puts a
control in the window named `PVwin`, and `Gui, Add, ...` puts a control in
the window of the thread, the one with no name.

The CHM (`Gui.htm`, "Creating Multiple GUI windows") says that a window is
found by its name or by its number, and that the name of it may also be the
handle of it (`Gui, %MyGuiHwnd%:Destroy`).

The tree had one window at a time. The state of the desktop of a Windows
machine now keeps a map of the windows of the v1, by name, and the window
with no name is the one of the number one.

**Measured**: `Gui, PVwin: -DPIScale +Resize 640 +hwndPVhwnd +LastFound
+OwnDialogs` writes the handle of the window into `PVhwnd`; `Gui, %PVj%:
+Resize` and `Gui, %PVj%: Destroy` work with the handle of a window.

## 2. The sub-command of the `Gui` may be a list of options

The CHM (`Gui.htm`, "Options and Styles for a Window") says: "One or more
options and styles may be specified immediately after the GUI command, each
separated from the next with one or more spaces or tabs."

So the sub-command of the v1 is not one option: it is a list. A script writes
them all in one line:

```ahk
Gui, PVwin: -DPIScale +Resize +MinSize480x270 +hwndPVhwnd +LastFound +OwnDialogs
```

Each word of the list carries its own sign. A word with no sign is not an
option and the v1 walks over it. An option that is not known is an error.

**Measured**: `Gui, PVa: +Resize +MinSize480x270 +MaxSize640x480` is accepted;
`Gui, PVbad: +NoSuchOption` is refused.

## 3. The options that take a value

The name of the option and the value of it are written together, with no
space between them:

| The option | The value | What it does |
|---|---|---|
| `+Hwnd` | the name of a variable | the handle of the window goes into the variable |
| `+Owner` | the name or the handle of a window | the window becomes owned by it |
| `+Parent` | the name or the handle of a window | the window becomes a child of it |
| `+Label` | the name of a prefix | the special labels of the window begin with it |
| `+Delimiter` | a character, `Space` or `Tab` | the separator of the contents of the controls |
| `+MinSize` | `WxH`, or nothing | the smallest size of the window |
| `+MaxSize` | `WxH`, or nothing | the largest size of the window |

**Measured**: all of them are accepted by the v1.

## 4. The styles without a name

The CHM says that a plus or a minus sign may be followed by a style number,
and by an extended style number with the letter `E`:

```ahk
Gui, PVgdiPic: -DPIScale +E0x20 +Disabled -Caption +E0x80000 +hwndhGDIwin +OwnerPVwin
```

A script of the v1 uses them. The tree carries the ordinary style and the extended
style of the window.

**Measured**: `Gui, PVstyle: +0x10000000 +E0x40000` is accepted.

## 5. The options of the window that the machine carries

These are the options of the window that the CHM lists, and what the tree
does with each of them:

| The option | The effect |
|---|---|
| `AlwaysOnTop` | the window stays above the others |
| `Border` | a thin border around the window |
| `Caption` | the title bar and the thick border (present by default) |
| `Delimiter` | the separator of the contents of the controls |
| `Disabled` | the window does not answer the user |
| `DPIScale` | the coordinates are scaled by the screen (on by default) |
| `LastFound` | the window becomes the last found window |
| `LastFoundExist` | the window becomes the last found window, if it is there |
| `Label` | the prefix of the special labels of the window |
| `MaximizeBox` | the button of the maximum in the title bar |
| `MinimizeBox` | the button of the minimum in the title bar (present by default) |
| `MinSize` / `MaxSize` | the limits of the size of the window |
| `OwnDialogs` | the dialogs of the thread belong to the window |
| `Owner` / `Parent` | the window belongs to another one |
| `Resize` | the window is resizable |
| `SysMenu` | the menu of the system (present by default) |
| `Theme` | the controls have the look of the system |
| `ToolWindow` | a narrow title bar and no button in the task bar |

## 6. The `Show` of a window

`Gui, Name:Show, Options, Title` — the options are `w`, `h`, `x`, `y`,
`Center`, `xCenter`, `yCenter`, `AutoSize`, `Minimize`, `Maximize`,
`Restore`, `NoActivate`, `NA` and `Hide`; the title is the third parameter.

The title is written even when the window is not shown.

## 7. The `Submit` of a window

`Gui, Name:Submit` writes the contents of the controls that have a variable
of their own into those variables. The option `NoHide` keeps the window
visible.

## 8. The `Default` of a thread

`Gui, Name:Default` makes the window the one that the commands of the thread
use when they carry no name. The v1 keeps the name of it in `A_DefaultGui`.

## 9. The `Tab` of a window

`Gui, Tab, N` and `Gui, Name:Tab, N` switch to the tab `N` of the tab
control of the window. A script of the v1 uses it many times.

The CHM of the v1 does not list the `Tab` among the sub-commands of the
`Gui`, and the tree refused it. The binary of the v1.1.37.02 accepts it, and
a script of the v1 needs it.

**Measured**: `Gui, Tab, 2` and `Gui, PVt: Tab, 3` are accepted.

## 10. The `GuiControl` of a named window

The name of the window is written before the sub-command, with a colon:

```ahk
GuiControl, MyGui:Show, MyButton
GuiControl, MyGui:, MyListBox, Item1|Item2
```

The CHM (`GuiControl.htm`, "Remarks") says: "To operate upon a window other
than the default, include its name or number (or in [v1.1.03+] its HWND)
followed by a colon in front of the sub-command."

The sub-commands of the v1 are the blank one (the new contents of the
control), `Text`, `Move`, `MoveDraw`, `Focus`, `Disable`, `Enable`, `Hide`,
`Show`, `Choose`, `ChooseString` and `Font`. The `+Redraw` and the `-Redraw`
are accepted as well.

**Measured**: `GuiControl, PVwin:, UIAbtn0, Hello`, `GuiControl, PVwin:
+Redraw, UIAbtn0`, `GuiControl, PVwin: Focus, UIAbtn0` and `GuiControl,
PVwin: Move, UIAbtn0, x10 y20 w30 h40` are all accepted.

## 11. The `GuiControlGet` of a named window

The name of the window goes in the same place:

```ahk
GuiControlGet, MyEdit, MyGui:
GuiControlGet, MyEdit, MyGui:Pos
GuiControlGet, OutputVar, MyGui:Focus
```

The sub-commands are the blank one (the contents of the control), `Pos`,
`Focus`, `FocusV`, `Enabled`, `Visible`, `Hwnd` and `Name`.

Two of them are not what the tree had:

- `Pos` writes **four** variables: the name of the caller with the letter
  `X`, `Y`, `W` and `H` at the end. The tree wrote one text with the four
  numbers.
- `Name` writes the name of the variable of the control. The tree wrote the
  ClassNN of it.

The `Hwnd` writes the handle of the control as a text with the `0x` in
front of it: `0x8b1750`.

**Measured**: all of them, with the values above.

## 12. The control of a window is found by its variable, its ClassNN or its text

The CHM says that the `ControlID` is the name of the variable of the
control, or the ClassNN of it, or its text. When the `ControlID` is blank,
the name of the variable of the caller is used.

The `GuiControlGet` of a control that is not there writes the empty text and
leaves the `ErrorLevel` at one; it does not stop the script.

## 13. The `Gui, Add` of a window

`Gui, Name:Add, Type, Options, Text`. The options carry the place of the
control (`x`, `y`, `w`, `h`), the variable of it (`v`), the label of it
(`g`) and the handle of it (`+hwndVar`).

The `Gui, Name:Add` makes the window when it is not there.

### The class of a control

A control of the v1 is a window of the machine, and the class of it is
the class of that window. The class of every kind of control was read
from the machine, with a probe that makes one control of each kind
(`gui-kinds.ahk`) and reads the classes of the windows that it makes.

**Measured on the binary of the v1.1.37.02**: the `Link` of the v1 is a
control of the class `SysLink` — the class of the window of the
`comctl32`, and not the name of the library of it. The other kinds are
the ones of the table:

| The kind | The class |
|---|---|
| `Button`, `CheckBox`, `Radio`, `GroupBox` | `Button` |
| `Text`, `Picture` | `Static` |
| `Edit` | `Edit` |
| `ListBox` | `ListBox` |
| `ComboBox`, `DDL` | `ComboBox` |
| `Progress` | `msctls_progress32` |
| `Slider` | `msctls_trackbar32` |
| `UpDown` | `msctls_updown32` |
| `DateTime` | `SysDateTimePick32` |
| `MonthCal` | `SysMonthCal32` |
| `Hotkey` | `msctls_hotkey32` |
| `Link` | `SysLink` |
| `Tab` | `SysTabControl32` |
| `StatusBar` | `msctls_statusbar32` |
| `TreeView` | `SysTreeView32` |
| `ListView` | `SysListView32` |

The tree asked the machine for a control of the class `SysLink32`, and
the machine has no class of that name: the control was not made, and the
`Gui, Add` failed.

## 14. What the tree does now

- [x] The state of the desktop keeps a map of the windows of the v1, by name.
- [x] The window with no name is the one of the thread, and the old API of
      the tree works on it.
- [x] The sub-command of the `Gui` is a list of options, and each option is
      applied to the window of its name.
- [x] The options that take a value are read: `Hwnd`, `Owner`, `Parent`,
      `Label`, `Delimiter`, `MinSize` and `MaxSize`.
- [x] The styles without a name are carried: `+0x...` and `+E0x...`.
- [x] The options of the window are carried by the machine: the ones of the
      table of the section 5.
- [x] The `Show` of a window takes the options of the section 6 and the
      title.
- [x] The `Submit` of a window writes the contents of the controls into
      their variables.
- [x] The `Default` of a thread names the window of the thread.
- [x] The `Tab` of a window switches to the tab of it.
- [x] The `GuiControl` and the `GuiControlGet` carry the name of the window
      before the sub-command.
- [x] The `GuiControlGet` of the `Pos` writes the four variables, and the
      `Name` writes the name of the variable of the control.
- [x] The control is found by its variable, its ClassNN or its text.
- [x] The `+hwndVar` of a control and of a window writes the handle into a
      variable of the script.
- [x] The control of the kind `Link` is a control of the class `SysLink`,
      the class that the v1 uses and the one that the machine has.
