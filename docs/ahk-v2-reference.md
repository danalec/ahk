# The surface of v2.0.19

Every name of AutoHotkey v2.0.19, with what this runtime answers for it.

The list is read out of the binary of v2.0.19 (`crates/ahk/reference/ahk-v2.0.19.surface.tsv`), and the state of each name is the answer of `ahk --surface`. The summary is:

> **833 entries: 815 answer with a value, 18 are refused, 0 are pending.**

A name that answers with a value does the thing. A name that is refused is refused **with the reason** -- a question the desktop cannot answer fails, and it never answers `false`. A pending name is one the tree does not carry yet; the table of them is empty, because the port carries the whole of v2.

Regenerate this chapter with `python egm-review/gen_docs_reference.py`.

## The commands

A command is a name a script writes at the start of a line, without parentheses. In v2 every one of them may also be written as a call with parentheses.

| Name | Class | Parameters | State |
|---|---|---|---|
| `BlockInput` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `CallbackCreate` | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `CallbackFree` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ClipWait` | -- | 0 to 2 | value |
| `ControlAddItem` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlChooseIndex` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlChooseString` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlClick` | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlDeleteItem` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlFindItem` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlFocus` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetChecked` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetChoice` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetClassNN` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetEnabled` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetExStyle` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetFocus` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetHwnd` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetIndex` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetItems` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetPos` | -- | 0 to 9 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetStyle` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetText` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetVisible` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlHide` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlHideDropDown` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlMove` | -- | 0 to 9 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSend` | -- | 1 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSendText` | -- | 1 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetChecked` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetEnabled` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetExStyle` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetStyle` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetText` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlShow` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlShowDropDown` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `CoordMode` | -- | 1 to 2 | value |
| `Critical` | -- | 0 to 1 | refused: this runtime runs one script in one thread, and it cannot make it critical |
| `DateAdd` | -- | 3 to 3 | value |
| `DateDiff` | -- | 3 to 3 | value |
| `DetectHiddenText` | -- | 1 to 1 | value |
| `DetectHiddenWindows` | -- | 1 to 1 | value |
| `DirCopy` | -- | 2 to 3 | value |
| `DirCreate` | -- | 1 to 1 | value |
| `DirDelete` | -- | 1 to 2 | value |
| `DirExist` | -- | 1 to 1 | value |
| `DirMove` | -- | 2 to 3 | value |
| `DirSelect` | -- | 0 to 3 | value |
| `Download` | -- | 2 to 2 | value |
| `DriveEject` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetCapacity` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetFilesystem` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetLabel` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetList` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetSerial` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetSpaceFree` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetStatus` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetStatusCD` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGetType` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveLock` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveRetract` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveSetLabel` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveUnlock` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `Edit` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditGetCurrentCol` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditGetCurrentLine` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditGetLine` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditGetLineCount` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditGetSelectedText` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `EditPaste` | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `EnvGet` | -- | 1 to 1 | value |
| `EnvSet` | -- | 1 to 2 | value |
| `Exit` | -- | 0 to 1 | value |
| `ExitApp` | -- | 0 to 1 | value |
| `FileAppend` | -- | 1 to 3 | value |
| `FileCopy` | -- | 2 to 3 | value |
| `FileCreateShortcut` | -- | 2 to 9 | refused: this is a name of Windows, and this desktop is not Windows |
| `FileDelete` | -- | 1 to 1 | value |
| `FileEncoding` | -- | 1 to 1 | value |
| `FileExist` | -- | 1 to 1 | value |
| `FileGetAttrib` | -- | 0 to 1 | value |
| `FileGetShortcut` | -- | 1 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `FileGetSize` | -- | 0 to 2 | value |
| `FileGetTime` | -- | 0 to 2 | value |
| `FileGetVersion` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `FileInstall` | -- | 2 to 3 | value |
| `FileMove` | -- | 2 to 3 | value |
| `FileRead` | -- | 1 to 2 | value |
| `FileRecycle` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `FileRecycleEmpty` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `FileSelect` | -- | 0 to 4 | value |
| `FileSetAttrib` | -- | 1 to 3 | value |
| `FileSetTime` | -- | 0 to 4 | value |
| `GetKeyName` | -- | 1 to 1 | value |
| `GetKeySC` | -- | 1 to 1 | value |
| `GetKeyState` | -- | 1 to 2 | value |
| `GetKeyVK` | -- | 1 to 1 | value |
| `GroupActivate` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupAdd` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupClose` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupDeactivate` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `GuiCtrlFromHwnd` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `GuiFromHwnd` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `HotIf` | -- | 0 to 1 | value |
| `HotIfWinActive` | -- | 0 to 2 | value |
| `HotIfWinExist` | -- | 0 to 2 | value |
| `HotIfWinNotActive` | -- | 0 to 2 | value |
| `HotIfWinNotExist` | -- | 0 to 2 | value |
| `Hotkey` | -- | 1 to 3 | value |
| `Hotstring` | -- | 1 to 3 | value |
| `IL_Add` | -- | 2 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `IL_Create` | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `IL_Destroy` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ImageSearch` | -- | 7 to 7 | value |
| `IniDelete` | -- | 2 to 3 | value |
| `IniRead` | -- | 1 to 4 | value |
| `IniWrite` | -- | 3 to 4 | value |
| `InputBox` | -- | 0 to 4 | value |
| `InstallKeybdHook` | -- | 0 to 2 | value |
| `InstallMouseHook` | -- | 0 to 2 | value |
| `IsLabel` | -- | 1 to 1 | value |
| `KeyHistory` | -- | 0 to 1 | value |
| `KeyWait` | -- | 1 to 2 | value |
| `ListHotkeys` | -- | 0 to 0 | value |
| `ListLines` | -- | 0 to 1 | refused: the lines are listed in a window of v2, and this runtime has no window to show them in |
| `ListVars` | -- | 0 to 0 | refused: the variables are listed in a window of v2, and this runtime has no window to show them in |
| `ListViewGetContent` | -- | 0 to 6 | refused: this is a name of Windows, and this desktop is not Windows |
| `LoadPicture` | -- | 1 to 3 | value |
| `MenuFromHandle` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `MenuSelect` | -- | 0 to 11 | refused: this is a name of Windows, and this desktop is not Windows |
| `MonitorGet` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `MonitorGetCount` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `MonitorGetName` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `MonitorGetPrimary` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `MonitorGetWorkArea` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `MouseClick` | -- | 0 to 7 | value |
| `MouseClickDrag` | -- | 0 to 7 | value |
| `MouseGetPos` | -- | 0 to 5 | value |
| `MouseMove` | -- | 2 to 4 | value |
| `MsgBox` | -- | 0 to 3 | value |
| `OnClipboardChange` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `OnError` | -- | 1 to 2 | value |
| `OnExit` | -- | 1 to 2 | value |
| `OnMessage` | -- | 2 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `OutputDebug` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `Pause` | -- | 0 to 1 | refused: this runtime cannot wait for a key to continue: the script ends when it ends |
| `Persistent` | -- | 0 to 1 | refused: this runtime is a script that ends when it ends, and it cannot be kept alive |
| `PixelGetColor` | -- | 2 to 3 | value |
| `PixelSearch` | -- | 7 to 8 | value |
| `PostMessage` | -- | 1 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessClose` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessExist` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessGetName` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessGetParent` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessGetPath` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessSetPriority` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessWait` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `ProcessWaitClose` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `Reload` | -- | 0 to 0 | refused: this runtime runs one script once, and it cannot run itself again |
| `Run` | -- | 1 to 4 | value |
| `RunAs` | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `Send` | -- | 1 to 1 | value |
| `SendEvent` | -- | 1 to 1 | value |
| `SendInput` | -- | 1 to 1 | value |
| `SendLevel` | -- | 1 to 1 | value |
| `SendMessage` | -- | 1 to 9 | refused: this is a name of Windows, and this desktop is not Windows |
| `SendMode` | -- | 1 to 1 | value |
| `SendPlay` | -- | 1 to 1 | value |
| `SendText` | -- | 1 to 1 | value |
| `SetCapsLockState` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `SetControlDelay` | -- | 1 to 1 | value |
| `SetDefaultMouseSpeed` | -- | 1 to 1 | value |
| `SetKeyDelay` | -- | 0 to 3 | value |
| `SetMouseDelay` | -- | 1 to 2 | value |
| `SetNumLockState` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `SetRegView` | -- | 1 to 1 | value |
| `SetScrollLockState` | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `SetStoreCapsLockMode` | -- | 1 to 1 | value |
| `SetTimer` | -- | 0 to 3 | value |
| `SetTitleMatchMode` | -- | 1 to 1 | value |
| `SetWinDelay` | -- | 1 to 1 | value |
| `SetWorkingDir` | -- | 1 to 1 | value |
| `Shutdown` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `Sleep` | -- | 1 to 1 | value |
| `SoundBeep` | -- | 0 to 2 | value |
| `SoundPlay` | -- | 1 to 2 | value |
| `StatusBarGetText` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `StatusBarWait` | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `StrSplit` | -- | 1 to 4 | value |
| `Suspend` | -- | 0 to 1 | refused: this runtime cannot stop itself: it would never end |
| `SysGet` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `SysGetIPAddresses` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `Thread` | -- | 1 to 3 | refused: this runtime runs one script in one thread, and it cannot start another |
| `ToolTip` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `TraySetIcon` | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `TrayTip` | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinActivate` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinActivateBottom` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinClose` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetClass` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetClientPos` | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetControls` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetControlsHwnd` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetCount` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetExStyle` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetID` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetIDLast` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetList` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetMinMax` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetPID` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetPos` | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetProcessName` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetProcessPath` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetStyle` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetText` | -- | 0 to 4 | refused: this runtime reads no text of a window |
| `WinGetTitle` | -- | 0 to 4 | value |
| `WinGetTransColor` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetTransparent` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinHide` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinKill` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMaximize` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMinimize` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMinimizeAll` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMinimizeAllUndo` | -- | 0 to 0 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMove` | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMoveBottom` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMoveTop` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinRedraw` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinRestore` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetAlwaysOnTop` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetEnabled` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetExStyle` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetRegion` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetStyle` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetTitle` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetTransColor` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetTransparent` | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinShow` | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWait` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWaitActive` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWaitClose` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWaitNotActive` | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows |

## The functions

A function is a name a script calls with parentheses, or with the parameters of the line when the answer is not used.

| Name | Class | Parameters | State |
|---|---|---|---|
| `Abs` | -- | 1 to 1 | value |
| `ACos` | -- | 1 to 1 | value |
| `ASin` | -- | 1 to 1 | value |
| `ATan` | -- | 1 to 1 | value |
| `CaretGetPos` | -- | 0 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `Ceil` | -- | 1 to 1 | value |
| `Chr` | -- | 1 to 1 | value |
| `Click` | -- | 0 to 6 | value |
| `ComCall` | -- | 2 to any | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjActive` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjConnect` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjFlags` | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjFromPtr` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjGet` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjQuery` | -- | 2 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjType` | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjValue` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `Cos` | -- | 1 to 1 | value |
| `DllCall` | -- | 1 to any | value |
| `Exp` | -- | 1 to 1 | value |
| `FileOpen` | -- | 2 to 3 | value |
| `Floor` | -- | 1 to 1 | value |
| `Format` | -- | 1 to any | value |
| `FormatTime` | -- | 0 to 2 | value |
| `GetMethod` | -- | 1 to 3 | value |
| `HasBase` | -- | 2 to 2 | value |
| `HasMethod` | -- | 1 to 3 | value |
| `HasProp` | -- | 2 to 2 | value |
| `InStr` | -- | 2 to 5 | value |
| `IsAlnum` | -- | 1 to 2 | value |
| `IsAlpha` | -- | 1 to 2 | value |
| `IsDigit` | -- | 1 to 1 | value |
| `IsFloat` | -- | 1 to 1 | value |
| `IsInteger` | -- | 1 to 1 | value |
| `IsLower` | -- | 1 to 2 | value |
| `IsNumber` | -- | 1 to 1 | value |
| `IsObject` | -- | 1 to 1 | value |
| `IsSetRef` | -- | 1 to 1 | value |
| `IsSpace` | -- | 1 to 1 | value |
| `IsTime` | -- | 1 to 1 | value |
| `IsUpper` | -- | 1 to 2 | value |
| `IsXDigit` | -- | 1 to 1 | value |
| `Ln` | -- | 1 to 1 | value |
| `Log` | -- | 1 to 1 | value |
| `LTrim` | -- | 1 to 2 | value |
| `Max` | -- | 1 to any | value |
| `Min` | -- | 1 to any | value |
| `Mod` | -- | 2 to 2 | value |
| `NumGet` | -- | 2 to 3 | value |
| `NumPut` | -- | 3 to any | value |
| `ObjAddRef` | -- | 1 to 1 | value |
| `ObjBindMethod` | -- | 1 to any | value |
| `ObjFromPtr` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ObjFromPtrAddRef` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ObjGetBase` | -- | 1 to 1 | value |
| `ObjGetCapacity` | -- | 1 to 1 | value |
| `ObjHasOwnProp` | -- | 2 to 2 | value |
| `ObjOwnPropCount` | -- | 1 to 1 | value |
| `ObjOwnProps` | -- | 1 to 1 | value |
| `ObjPtr` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ObjPtrAddRef` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ObjRelease` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `ObjSetBase` | -- | 2 to 2 | value |
| `ObjSetCapacity` | -- | 2 to 2 | value |
| `Ord` | -- | 1 to 1 | value |
| `Random` | -- | 0 to 2 | value |
| `RegCreateKey` | -- | 0 to 1 | refused: the registry is a thing of Windows, and this runtime does not carry it |
| `RegDelete` | -- | 0 to 2 | refused: the registry is a thing of Windows, and this runtime does not carry it |
| `RegDeleteKey` | -- | 0 to 1 | refused: the registry is a thing of Windows, and this runtime does not carry it |
| `RegExMatch` | -- | 2 to 4 | value |
| `RegExReplace` | -- | 2 to 6 | value |
| `RegRead` | -- | 0 to 3 | refused: the registry is a thing of Windows, and this runtime does not carry it |
| `RegWrite` | -- | 0 to 4 | refused: the registry is a thing of Windows, and this runtime does not carry it |
| `Round` | -- | 1 to 2 | value |
| `RTrim` | -- | 1 to 2 | value |
| `RunWait` | -- | 1 to 4 | value |
| `Sin` | -- | 1 to 1 | value |
| `Sort` | -- | 1 to 3 | value |
| `SoundGetInterface` | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `SoundGetMute` | -- | 0 to 2 | value |
| `SoundGetName` | -- | 0 to 2 | refused: this is a name of Windows, and this desktop is not Windows |
| `SoundGetVolume` | -- | 0 to 2 | value |
| `SoundSetMute` | -- | 1 to 3 | value |
| `SoundSetVolume` | -- | 1 to 3 | value |
| `SplitPath` | -- | 1 to 6 | value |
| `Sqrt` | -- | 1 to 1 | value |
| `StrCompare` | -- | 2 to 3 | value |
| `StrGet` | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows |
| `StrLen` | -- | 1 to 1 | value |
| `StrLower` | -- | 1 to 1 | value |
| `StrPtr` | -- | 1 to 1 | refused: this is a name of Windows, and this desktop is not Windows |
| `StrPut` | -- | 1 to 4 | refused: this is a name of Windows, and this desktop is not Windows |
| `StrReplace` | -- | 2 to 6 | value |
| `StrTitle` | -- | 1 to 1 | value |
| `StrUpper` | -- | 1 to 1 | value |
| `SubStr` | -- | 2 to 3 | value |
| `Tan` | -- | 1 to 1 | value |
| `Trim` | -- | 1 to 2 | value |
| `Type` | -- | 1 to 1 | value |
| `VarSetStrCapacity` | -- | 1 to 2 | value |
| `VerCompare` | -- | 2 to 2 | value |
| `WinActive` | -- | 0 to 4 | value |
| `WinExist` | -- | 0 to 4 | value |

## The built-in variables

The name of a built-in variable is spelled with the `A_` prefix, as the dialect spells it.

| Name | Class | Parameters | State |
|---|---|---|---|
| `AhkPath` | -- | -- | refused: this runtime is not AutoHotkey, and it has no path of an executable |
| `AhkVersion` | -- | -- | value |
| `AllowMainWindow` | -- | -- | value |
| `AppData` | -- | -- | value |
| `AppDataCommon` | -- | -- | value |
| `Clipboard` | -- | -- | value |
| `ComputerName` | -- | -- | value |
| `ComSpec` | -- | -- | value |
| `ControlDelay` | -- | -- | value |
| `CoordModeCaret` | -- | -- | value |
| `CoordModeMenu` | -- | -- | value |
| `CoordModeMouse` | -- | -- | value |
| `CoordModePixel` | -- | -- | value |
| `CoordModeToolTip` | -- | -- | value |
| `Cursor` | -- | -- | refused: the shape of the cursor of the machine is not read by this runtime |
| `DD` | -- | -- | value |
| `DDD` | -- | -- | value |
| `DDDD` | -- | -- | value |
| `DefaultMouseSpeed` | -- | -- | value |
| `Desktop` | -- | -- | value |
| `DesktopCommon` | -- | -- | value |
| `DetectHiddenText` | -- | -- | value |
| `DetectHiddenWindows` | -- | -- | value |
| `EndChar` | -- | -- | value |
| `EventInfo` | -- | -- | value |
| `FileEncoding` | -- | -- | value |
| `HotkeyInterval` | -- | -- | value |
| `HotkeyModifierTimeout` | -- | -- | value |
| `Hour` | -- | -- | value |
| `IconFile` | -- | -- | refused: this runtime has no tray icon |
| `IconHidden` | -- | -- | value |
| `IconNumber` | -- | -- | refused: this runtime has no tray icon |
| `IconTip` | -- | -- | value |
| `Index` | -- | -- | value |
| `InitialWorkingDir` | -- | -- | value |
| `Is64bitOS` | -- | -- | value |
| `IsAdmin` | -- | -- | value |
| `IsCompiled` | -- | -- | value |
| `IsCritical` | -- | -- | value |
| `IsPaused` | -- | -- | value |
| `IsSuspended` | -- | -- | value |
| `KeyDelay` | -- | -- | value |
| `KeyDelayPlay` | -- | -- | value |
| `KeyDuration` | -- | -- | value |
| `KeyDurationPlay` | -- | -- | value |
| `Language` | -- | -- | value |
| `LastError` | -- | -- | value |
| `LineFile` | -- | -- | value |
| `LineNumber` | -- | -- | value |
| `ListLines` | -- | -- | value |
| `LoopField` | -- | -- | value |
| `LoopFileAttrib` | -- | -- | value |
| `LoopFileDir` | -- | -- | value |
| `LoopFileExt` | -- | -- | value |
| `LoopFileFullPath` | -- | -- | value |
| `LoopFileName` | -- | -- | value |
| `LoopFilePath` | -- | -- | value |
| `LoopFileShortName` | -- | -- | value |
| `LoopFileShortPath` | -- | -- | value |
| `LoopFileSize` | -- | -- | value |
| `LoopFileSizeKB` | -- | -- | value |
| `LoopFileSizeMB` | -- | -- | value |
| `LoopFileTimeAccessed` | -- | -- | value |
| `LoopFileTimeCreated` | -- | -- | value |
| `LoopFileTimeModified` | -- | -- | value |
| `LoopReadLine` | -- | -- | value |
| `LoopRegKey` | -- | -- | value |
| `LoopRegName` | -- | -- | value |
| `LoopRegTimeModified` | -- | -- | value |
| `LoopRegType` | -- | -- | value |
| `MaxHotkeysPerInterval` | -- | -- | value |
| `MDay` | -- | -- | value |
| `MenuMaskKey` | -- | -- | value |
| `Min` | -- | -- | value |
| `MM` | -- | -- | value |
| `MMM` | -- | -- | value |
| `MMMM` | -- | -- | value |
| `Mon` | -- | -- | value |
| `MouseDelay` | -- | -- | value |
| `MouseDelayPlay` | -- | -- | value |
| `MSec` | -- | -- | value |
| `MyDocuments` | -- | -- | value |
| `Now` | -- | -- | value |
| `NowUTC` | -- | -- | value |
| `OSVersion` | -- | -- | value |
| `PriorHotkey` | -- | -- | value |
| `PriorKey` | -- | -- | value |
| `ProgramFiles` | -- | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `Programs` | -- | -- | value |
| `ProgramsCommon` | -- | -- | value |
| `PtrSize` | -- | -- | value |
| `RegView` | -- | -- | value |
| `ScreenDPI` | -- | -- | refused: this desktop does not know the scale of the output |
| `ScreenHeight` | -- | -- | value |
| `ScreenWidth` | -- | -- | value |
| `ScriptDir` | -- | -- | value |
| `ScriptFullPath` | -- | -- | value |
| `ScriptHwnd` | -- | -- | refused: this runtime has no window of its own: it is a script that runs and ends |
| `ScriptName` | -- | -- | value |
| `Sec` | -- | -- | value |
| `SendLevel` | -- | -- | value |
| `SendMode` | -- | -- | value |
| `Space` | -- | -- | value |
| `StartMenu` | -- | -- | value |
| `StartMenuCommon` | -- | -- | value |
| `Startup` | -- | -- | value |
| `StartupCommon` | -- | -- | value |
| `StoreCapsLockMode` | -- | -- | value |
| `Tab` | -- | -- | value |
| `Temp` | -- | -- | value |
| `ThisFunc` | -- | -- | value |
| `ThisHotkey` | -- | -- | value |
| `TickCount` | -- | -- | value |
| `TimeIdle` | -- | -- | value |
| `TimeIdleKeyboard` | -- | -- | value |
| `TimeIdleMouse` | -- | -- | value |
| `TimeIdlePhysical` | -- | -- | value |
| `TimeSincePriorHotkey` | -- | -- | value |
| `TimeSinceThisHotkey` | -- | -- | value |
| `TitleMatchMode` | -- | -- | value |
| `TitleMatchModeSpeed` | -- | -- | value |
| `TrayMenu` | -- | -- | refused: this runtime has no tray icon |
| `UserName` | -- | -- | value |
| `WDay` | -- | -- | value |
| `WinDelay` | -- | -- | value |
| `WinDir` | -- | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `WorkingDir` | -- | -- | value |
| `YDay` | -- | -- | value |
| `Year` | -- | -- | value |
| `YWeek` | -- | -- | value |
| `YYYY` | -- | -- | value |

## The class types

The class types are classes of the runtime. A script reads them as a class value, and it cannot extend them.

| Name | Class | Parameters | State |
|---|---|---|---|
| `Any` | -- | -- | value |
| `Array` | -- | -- | value |
| `BoundFunc` | -- | -- | value |
| `Buffer` | -- | -- | value |
| `Class` | -- | -- | value |
| `ClipboardAll` | -- | -- | value |
| `Closure` | -- | -- | value |
| `ComObject` | -- | -- | value |
| `ComValue` | -- | -- | value |
| `Enumerator` | -- | -- | value |
| `Error` | -- | -- | value |
| `File` | -- | -- | value |
| `Float` | -- | -- | value |
| `Func` | -- | -- | value |
| `Gui` | -- | -- | value |
| `IndexError` | -- | -- | value |
| `InputHook` | -- | -- | value |
| `Integer` | -- | -- | value |
| `Map` | -- | -- | value |
| `MemberError` | -- | -- | value |
| `MemoryError` | -- | -- | value |
| `Menu` | -- | -- | value |
| `MethodError` | -- | -- | value |
| `Number` | -- | -- | value |
| `Object` | -- | -- | value |
| `OSError` | -- | -- | value |
| `Primitive` | -- | -- | value |
| `PropertyError` | -- | -- | value |
| `RegExMatchInfo` | -- | -- | value |
| `String` | -- | -- | value |
| `TargetError` | -- | -- | value |
| `TimeoutError` | -- | -- | value |
| `TypeError` | -- | -- | value |
| `UnsetError` | -- | -- | value |
| `UnsetItemError` | -- | -- | value |
| `ValueError` | -- | -- | value |
| `ZeroDivisionError` | -- | -- | value |

## The methods of the class types

A method belongs to a class. The number of the parameters is the one of the binary, and it counts the object when the method is handed over as a value: `m.Get` takes two and answers three, because the object is an argument as well. The first parameter is the object, then the ones the script writes.

### Array

| Method | Parameters | State |
|---|---|---|
| `__Enum` | 1 to 2 (the object counts) | value |
| `__Item` | -- | value |
| `__New` | 1 to 1 (the object counts) | value |
| `Capacity` | -- | value |
| `Clone` | 1 to 1 (the object counts) | value |
| `Delete` | 2 to 2 (the object counts) | value |
| `Get` | 2 to 3 (the object counts) | value |
| `Has` | 2 to 2 (the object counts) | value |
| `InsertAt` | 2 to 2 (the object counts) | value |
| `Length` | -- | value |
| `Pop` | 1 to 1 (the object counts) | value |
| `Push` | 1 to 1 (the object counts) | value |
| `RemoveAt` | 2 to 3 (the object counts) | value |

### Buffer

| Method | Parameters | State |
|---|---|---|
| `__New` | 1 to 3 (the object counts) | value |
| `Ptr` | -- | value |
| `Size` | -- | value |

### Class

| Method | Parameters | State |
|---|---|---|
| `__Init` | -- | value |
| `Base` | -- | value |
| `Call` | -- | value |
| `Prototype` | -- | value |

### ClipboardAll

| Method | Parameters | State |
|---|---|---|
| `Ptr` | -- | value |
| `Size` | -- | value |

### ComValue

| Method | Parameters | State |
|---|---|---|
| `Ptr` | -- | value |

### Enumerator

| Method | Parameters | State |
|---|---|---|
| `Call` | 1 to 3 (the object counts) | value |

### Error

| Method | Parameters | State |
|---|---|---|
| `Extra` | -- | value |
| `File` | -- | value |
| `Line` | -- | value |
| `Message` | -- | value |
| `Stack` | -- | value |
| `What` | -- | value |

### File

| Method | Parameters | State |
|---|---|---|
| `AtEOF` | -- | value |
| `Close` | 1 to 1 (the object counts) | value |
| `Encoding` | -- | value |
| `Encoding` | -- | value |
| `Handle` | -- | value |
| `Length` | -- | value |
| `Length` | -- | value |
| `Pos` | -- | value |
| `Pos` | -- | value |
| `RawRead` | 2 to 3 (the object counts) | value |
| `RawWrite` | 2 to 3 (the object counts) | value |
| `Read` | 1 to 2 (the object counts) | value |
| `ReadChar` | 1 to 1 (the object counts) | value |
| `ReadDouble` | 1 to 1 (the object counts) | value |
| `ReadFloat` | 1 to 1 (the object counts) | value |
| `ReadInt` | 1 to 1 (the object counts) | value |
| `ReadInt64` | 1 to 1 (the object counts) | value |
| `ReadLine` | 1 to 1 (the object counts) | value |
| `ReadShort` | 1 to 1 (the object counts) | value |
| `ReadUChar` | 1 to 1 (the object counts) | value |
| `ReadUInt` | 1 to 1 (the object counts) | value |
| `ReadUShort` | 1 to 1 (the object counts) | value |
| `Seek` | 2 to 3 (the object counts) | value |
| `Write` | 2 to 2 (the object counts) | value |
| `WriteChar` | 2 to 2 (the object counts) | value |
| `WriteDouble` | 2 to 2 (the object counts) | value |
| `WriteFloat` | 2 to 2 (the object counts) | value |
| `WriteInt` | 2 to 2 (the object counts) | value |
| `WriteInt64` | 2 to 2 (the object counts) | value |
| `WriteLine` | 1 to 2 (the object counts) | value |
| `WriteShort` | 2 to 2 (the object counts) | value |
| `WriteUChar` | 2 to 2 (the object counts) | value |
| `WriteUInt` | 2 to 2 (the object counts) | value |
| `WriteUShort` | 2 to 2 (the object counts) | value |

### Func

| Method | Parameters | State |
|---|---|---|
| `Bind` | 1 to 1 (the object counts) | value |
| `Call` | 1 to 1 (the object counts) | value |
| `IsBuiltIn` | -- | value |
| `IsByRef` | 1 to 1 (the object counts) | value |
| `IsOptional` | 1 to 1 (the object counts) | value |
| `IsVariadic` | -- | value |
| `MaxParams` | -- | value |
| `MinParams` | -- | value |
| `Name` | -- | value |

### Gui

| Method | Parameters | State |
|---|---|---|
| `__Enum` | -- | value |
| `__Item` | -- | value |
| `__New` | -- | value |
| `Add` | -- | value |
| `AddActiveX` | -- | value |
| `AddButton` | -- | value |
| `AddCheckBox` | -- | value |
| `AddComboBox` | -- | value |
| `AddCustom` | -- | value |
| `AddDateTime` | -- | value |
| `AddDDL` | -- | value |
| `AddDropDownList` | -- | value |
| `AddEdit` | -- | value |
| `AddGroupBox` | -- | value |
| `AddHotkey` | -- | value |
| `AddLink` | -- | value |
| `AddListBox` | -- | value |
| `AddListView` | -- | value |
| `AddMonthCal` | -- | value |
| `AddPic` | -- | value |
| `AddPicture` | -- | value |
| `AddProgress` | -- | value |
| `AddRadio` | -- | value |
| `AddSlider` | -- | value |
| `AddStatusBar` | -- | value |
| `AddTab` | -- | value |
| `AddTab2` | -- | value |
| `AddTab3` | -- | value |
| `AddText` | -- | value |
| `AddTreeView` | -- | value |
| `AddUpDown` | -- | value |
| `BackColor` | -- | value |
| `BackColor` | -- | value |
| `Destroy` | -- | value |
| `Flash` | -- | value |
| `FocusedCtrl` | -- | value |
| `GetClientPos` | -- | value |
| `GetPos` | -- | value |
| `Hide` | -- | value |
| `Hwnd` | -- | value |
| `MarginX` | -- | value |
| `MarginX` | -- | value |
| `MarginY` | -- | value |
| `MarginY` | -- | value |
| `Maximize` | -- | value |
| `MenuBar` | -- | value |
| `MenuBar` | -- | value |
| `Minimize` | -- | value |
| `Move` | -- | value |
| `Name` | -- | value |
| `Name` | -- | value |
| `OnEvent` | -- | value |
| `Opt` | -- | value |
| `Restore` | -- | value |
| `SetFont` | -- | value |
| `Show` | -- | value |
| `Submit` | -- | value |
| `Title` | -- | value |
| `Title` | -- | value |

### Gui.Control

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `Choose` | -- | value |
| `ClassNN` | -- | value |
| `Delete` | -- | value |
| `Enabled` | -- | value |
| `Enabled` | -- | value |
| `Focus` | -- | value |
| `Focused` | -- | value |
| `GetPos` | -- | value |
| `Gui` | -- | value |
| `Hwnd` | -- | value |
| `Move` | -- | value |
| `Name` | -- | value |
| `Name` | -- | value |
| `OnCommand` | -- | value |
| `OnEvent` | -- | value |
| `OnNotify` | -- | value |
| `Opt` | -- | value |
| `Redraw` | -- | value |
| `SetFont` | -- | value |
| `SetFormat` | -- | value |
| `Text` | -- | value |
| `Text` | -- | value |
| `Type` | -- | value |
| `UseTab` | -- | value |
| `Value` | -- | value |
| `Value` | -- | value |
| `Visible` | -- | value |
| `Visible` | -- | value |

### Gui.ListView

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `Delete` | -- | value |
| `Get` | -- | value |
| `GetChild` | -- | value |
| `GetCount` | -- | value |
| `GetNext` | -- | value |
| `GetParent` | -- | value |
| `GetPrev` | -- | value |
| `GetSelection` | -- | value |
| `GetText` | -- | value |
| `Modify` | -- | value |
| `SetIcon` | -- | value |
| `SetImageList` | -- | value |
| `SetParts` | -- | value |
| `SetText` | -- | value |

### Gui.Menu

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `AddStandard` | -- | value |
| `Check` | -- | value |
| `ClickCount` | -- | value |
| `ClickCount` | -- | value |
| `Default` | -- | value |
| `Default` | -- | value |
| `Delete` | -- | value |
| `Disable` | -- | value |
| `Enable` | -- | value |
| `Handle` | -- | value |
| `Insert` | -- | value |
| `Rename` | -- | value |
| `SetColor` | -- | value |
| `SetIcon` | -- | value |
| `Show` | -- | value |
| `ToggleCheck` | -- | value |
| `ToggleEnable` | -- | value |
| `Uncheck` | -- | value |

### Gui.StatusBar

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `Delete` | -- | value |
| `DeleteCol` | -- | value |
| `GetCount` | -- | value |
| `GetNext` | -- | value |
| `GetText` | -- | value |
| `Insert` | -- | value |
| `InsertCol` | -- | value |
| `Modify` | -- | value |
| `ModifyCol` | -- | value |
| `SetImageList` | -- | value |

### Gui.TreeView

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `Delete` | -- | value |
| `Get` | -- | value |
| `GetChild` | -- | value |
| `GetCount` | -- | value |
| `GetNext` | -- | value |
| `GetParent` | -- | value |
| `GetPrev` | -- | value |
| `GetSelection` | -- | value |
| `GetText` | -- | value |
| `Modify` | -- | value |
| `SetIcon` | -- | value |
| `SetImageList` | -- | value |
| `SetParts` | -- | value |
| `SetText` | -- | value |

### Hotkey

| Method | Parameters | State |
|---|---|---|
| `__New` | -- | value |
| `BackspaceIsUndo` | -- | value |
| `BackspaceIsUndo` | -- | value |
| `CaseSensitive` | -- | value |
| `CaseSensitive` | -- | value |
| `EndKey` | -- | value |
| `EndMods` | -- | value |
| `EndReason` | -- | value |
| `FindAnywhere` | -- | value |
| `FindAnywhere` | -- | value |
| `InProgress` | -- | value |
| `Input` | -- | value |
| `KeyOpt` | -- | value |
| `Match` | -- | value |
| `MinSendLevel` | -- | value |
| `MinSendLevel` | -- | value |
| `NotifyNonText` | -- | value |
| `NotifyNonText` | -- | value |
| `OnChar` | -- | value |
| `OnChar` | -- | value |
| `OnEnd` | -- | value |
| `OnEnd` | -- | value |
| `OnKeyDown` | -- | value |
| `OnKeyDown` | -- | value |
| `OnKeyUp` | -- | value |
| `OnKeyUp` | -- | value |
| `Start` | -- | value |
| `Stop` | -- | value |
| `Timeout` | -- | value |
| `Timeout` | -- | value |
| `VisibleNonText` | -- | value |
| `VisibleNonText` | -- | value |
| `VisibleText` | -- | value |
| `VisibleText` | -- | value |
| `Wait` | -- | value |

### InputHook

| Method | Parameters | State |
|---|---|---|
| `__New` | -- | value |
| `BackspaceIsUndo` | -- | value |
| `BackspaceIsUndo` | -- | value |
| `CaseSensitive` | -- | value |
| `CaseSensitive` | -- | value |
| `EndKey` | -- | value |
| `EndMods` | -- | value |
| `EndReason` | -- | value |
| `FindAnywhere` | -- | value |
| `FindAnywhere` | -- | value |
| `InProgress` | -- | value |
| `Input` | -- | value |
| `KeyOpt` | -- | value |
| `Match` | -- | value |
| `MinSendLevel` | -- | value |
| `MinSendLevel` | -- | value |
| `NotifyNonText` | -- | value |
| `NotifyNonText` | -- | value |
| `OnChar` | -- | value |
| `OnChar` | -- | value |
| `OnEnd` | -- | value |
| `OnEnd` | -- | value |
| `OnKeyDown` | -- | value |
| `OnKeyDown` | -- | value |
| `OnKeyUp` | -- | value |
| `OnKeyUp` | -- | value |
| `Start` | -- | value |
| `Stop` | -- | value |
| `Timeout` | -- | value |
| `Timeout` | -- | value |
| `VisibleNonText` | -- | value |
| `VisibleNonText` | -- | value |
| `VisibleText` | -- | value |
| `VisibleText` | -- | value |
| `Wait` | -- | value |

### Map

| Method | Parameters | State |
|---|---|---|
| `__Enum` | 1 to 2 (the object counts) | value |
| `__Item` | -- | value |
| `__New` | 1 to 1 (the object counts) | value |
| `Capacity` | -- | value |
| `CaseSense` | -- | value |
| `Clear` | 1 to 1 (the object counts) | value |
| `Clone` | 1 to 1 (the object counts) | value |
| `Clone` | 1 to 1 (the object counts) | value |
| `Count` | -- | value |
| `DefineProp` | -- | value |
| `Delete` | 2 to 2 (the object counts) | value |
| `DeleteProp` | -- | value |
| `Get` | 2 to 3 (the object counts) | value |
| `GetOwnPropDesc` | -- | value |
| `Has` | 2 to 2 (the object counts) | value |
| `HasOwnProp` | -- | value |
| `OwnProps` | -- | value |
| `Set` | 1 to 1 (the object counts) | value |

### Object

| Method | Parameters | State |
|---|---|---|
| `Base` | -- | value |
| `Clone` | -- | value |
| `DefineProp` | -- | value |
| `DeleteProp` | -- | value |
| `GetMethod` | -- | value |
| `GetOwnPropDesc` | -- | value |
| `HasBase` | -- | value |
| `HasMethod` | -- | value |
| `HasOwnProp` | -- | value |
| `HasProp` | -- | value |
| `OwnProps` | -- | value |
