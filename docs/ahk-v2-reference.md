# The surface of v2.0.32

Every name of AutoHotkey v2.0.32, with what this runtime answers for it.

The list is read out of the binary of v2.0.32 (`crates/ahk/reference/ahk-v2.0.32.surface.tsv`), and the state of each name is the answer of `ahk --surface`. The summary is:

> **843 entries: 843 answer with a value, 0 are refused, 0 are pending.**

A name that answers with a value does the thing. A name that is refused is refused **with the reason** -- a question the desktop cannot answer fails, and it never answers `false`. A pending name is one the tree does not carry yet; the table of them is empty, because the port carries the whole of v2.

Regenerate this chapter with `python egm-review/gen_docs_reference.py`.

## The commands

A command is a name a script writes at the start of a line, without parentheses. In v2 every one of them may also be written as a call with parentheses.

| Name | Class | Parameters | State |
|---|---|---|---|
| `BlockInput` | -- | 1 to 1 | value |
| `CallbackCreate` | -- | 1 to 3 | value |
| `CallbackFree` | -- | 1 to 1 | value |
| `ClipWait` | -- | 0 to 2 | value |
| `ControlAddItem` | -- | 2 to 6 | value |
| `ControlChooseIndex` | -- | 2 to 6 | value |
| `ControlChooseString` | -- | 2 to 6 | value |
| `ControlClick` | -- | 0 to 8 | value |
| `ControlDeleteItem` | -- | 2 to 6 | value |
| `ControlFindItem` | -- | 2 to 6 | value |
| `ControlFocus` | -- | 1 to 5 | value |
| `ControlGetChecked` | -- | 1 to 5 | value |
| `ControlGetChoice` | -- | 1 to 5 | value |
| `ControlGetClassNN` | -- | 1 to 5 | value |
| `ControlGetEnabled` | -- | 1 to 5 | value |
| `ControlGetExStyle` | -- | 1 to 5 | value |
| `ControlGetFocus` | -- | 0 to 4 | value |
| `ControlGetHwnd` | -- | 1 to 5 | value |
| `ControlGetIndex` | -- | 1 to 5 | value |
| `ControlGetItems` | -- | 1 to 5 | value |
| `ControlGetPos` | -- | 0 to 9 | value |
| `ControlGetStyle` | -- | 1 to 5 | value |
| `ControlGetText` | -- | 1 to 5 | value |
| `ControlGetVisible` | -- | 1 to 5 | value |
| `ControlHide` | -- | 1 to 5 | value |
| `ControlHideDropDown` | -- | 1 to 5 | value |
| `ControlMove` | -- | 0 to 9 | value |
| `ControlSend` | -- | 1 to 6 | value |
| `ControlSendText` | -- | 1 to 6 | value |
| `ControlSetChecked` | -- | 2 to 6 | value |
| `ControlSetEnabled` | -- | 2 to 6 | value |
| `ControlSetExStyle` | -- | 2 to 6 | value |
| `ControlSetStyle` | -- | 2 to 6 | value |
| `ControlSetText` | -- | 2 to 6 | value |
| `ControlShow` | -- | 1 to 5 | value |
| `ControlShowDropDown` | -- | 1 to 5 | value |
| `CoordMode` | -- | 1 to 2 | value |
| `Critical` | -- | 0 to 1 | value |
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
| `DriveEject` | -- | 0 to 1 | value |
| `DriveGetCapacity` | -- | 1 to 1 | value |
| `DriveGetFilesystem` | -- | 1 to 1 | value |
| `DriveGetLabel` | -- | 1 to 1 | value |
| `DriveGetList` | -- | 0 to 1 | value |
| `DriveGetSerial` | -- | 1 to 1 | value |
| `DriveGetSpaceFree` | -- | 1 to 1 | value |
| `DriveGetStatus` | -- | 1 to 1 | value |
| `DriveGetStatusCD` | -- | 0 to 1 | value |
| `DriveGetType` | -- | 1 to 1 | value |
| `DriveLock` | -- | 1 to 1 | value |
| `DriveRetract` | -- | 0 to 1 | value |
| `DriveSetLabel` | -- | 1 to 2 | value |
| `DriveUnlock` | -- | 1 to 1 | value |
| `Edit` | -- | 0 to 0 | value |
| `EditGetCurrentCol` | -- | 1 to 5 | value |
| `EditGetCurrentLine` | -- | 1 to 5 | value |
| `EditGetLine` | -- | 2 to 6 | value |
| `EditGetLineCount` | -- | 1 to 5 | value |
| `EditGetSelectedText` | -- | 1 to 5 | value |
| `EditPaste` | -- | 2 to 6 | value |
| `EnvGet` | -- | 1 to 1 | value |
| `EnvSet` | -- | 1 to 2 | value |
| `Exit` | -- | 0 to 1 | value |
| `ExitApp` | -- | 0 to 1 | value |
| `FileAppend` | -- | 1 to 3 | value |
| `FileCopy` | -- | 2 to 3 | value |
| `FileCreateShortcut` | -- | 2 to 9 | value |
| `FileDelete` | -- | 1 to 1 | value |
| `FileEncoding` | -- | 1 to 1 | value |
| `FileExist` | -- | 1 to 1 | value |
| `FileGetAttrib` | -- | 0 to 1 | value |
| `FileGetShortcut` | -- | 1 to 8 | value |
| `FileGetSize` | -- | 0 to 2 | value |
| `FileGetTime` | -- | 0 to 2 | value |
| `FileGetVersion` | -- | 0 to 1 | value |
| `FileInstall` | -- | 2 to 3 | value |
| `FileMove` | -- | 2 to 3 | value |
| `FileRead` | -- | 1 to 2 | value |
| `FileRecycle` | -- | 1 to 1 | value |
| `FileRecycleEmpty` | -- | 0 to 1 | value |
| `FileSelect` | -- | 0 to 4 | value |
| `FileSetAttrib` | -- | 1 to 3 | value |
| `FileSetTime` | -- | 0 to 4 | value |
| `GetKeyName` | -- | 1 to 1 | value |
| `GetKeySC` | -- | 1 to 1 | value |
| `GetKeyState` | -- | 1 to 2 | value |
| `GetKeyVK` | -- | 1 to 1 | value |
| `GroupActivate` | -- | 1 to 2 | value |
| `GroupAdd` | -- | 1 to 5 | value |
| `GroupClose` | -- | 1 to 2 | value |
| `GroupDeactivate` | -- | 1 to 2 | value |
| `GuiCtrlFromHwnd` | -- | 1 to 1 | value |
| `GuiFromHwnd` | -- | 1 to 2 | value |
| `HotIf` | -- | 0 to 1 | value |
| `HotIfWinActive` | -- | 0 to 2 | value |
| `HotIfWinExist` | -- | 0 to 2 | value |
| `HotIfWinNotActive` | -- | 0 to 2 | value |
| `HotIfWinNotExist` | -- | 0 to 2 | value |
| `Hotkey` | -- | 1 to 3 | value |
| `Hotstring` | -- | 1 to 3 | value |
| `IL_Add` | -- | 2 to 4 | value |
| `IL_Create` | -- | 0 to 3 | value |
| `IL_Destroy` | -- | 1 to 1 | value |
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
| `ListLines` | -- | 0 to 1 | value |
| `ListVars` | -- | 0 to 0 | value |
| `ListViewGetContent` | -- | 0 to 6 | value |
| `LoadPicture` | -- | 1 to 3 | value |
| `MenuFromHandle` | -- | 1 to 1 | value |
| `MenuSelect` | -- | 0 to 11 | value |
| `MonitorGet` | -- | 0 to 5 | value |
| `MonitorGetCount` | -- | 0 to 0 | value |
| `MonitorGetName` | -- | 0 to 1 | value |
| `MonitorGetPrimary` | -- | 0 to 0 | value |
| `MonitorGetWorkArea` | -- | 0 to 5 | value |
| `MouseClick` | -- | 0 to 7 | value |
| `MouseClickDrag` | -- | 0 to 7 | value |
| `MouseGetPos` | -- | 0 to 5 | value |
| `MouseMove` | -- | 2 to 4 | value |
| `MsgBox` | -- | 0 to 3 | value |
| `OnClipboardChange` | -- | 1 to 2 | value |
| `OnError` | -- | 1 to 2 | value |
| `OnExit` | -- | 1 to 2 | value |
| `OnMessage` | -- | 2 to 3 | value |
| `OutputDebug` | -- | 1 to 1 | value |
| `Pause` | -- | 0 to 1 | value |
| `Persistent` | -- | 0 to 1 | value |
| `PixelGetColor` | -- | 2 to 3 | value |
| `PixelSearch` | -- | 7 to 8 | value |
| `PostMessage` | -- | 1 to 8 | value |
| `ProcessClose` | -- | 1 to 1 | value |
| `ProcessExist` | -- | 0 to 1 | value |
| `ProcessGetName` | -- | 0 to 1 | value |
| `ProcessGetParent` | -- | 0 to 1 | value |
| `ProcessGetPath` | -- | 0 to 1 | value |
| `ProcessSetPriority` | -- | 1 to 2 | value |
| `ProcessWait` | -- | 1 to 2 | value |
| `ProcessWaitClose` | -- | 1 to 2 | value |
| `Reload` | -- | 0 to 0 | value |
| `Run` | -- | 1 to 4 | value |
| `RunAs` | -- | 0 to 3 | value |
| `Send` | -- | 1 to 1 | value |
| `SendEvent` | -- | 1 to 1 | value |
| `SendInput` | -- | 1 to 1 | value |
| `SendLevel` | -- | 1 to 1 | value |
| `SendMessage` | -- | 1 to 9 | value |
| `SendMode` | -- | 1 to 1 | value |
| `SendPlay` | -- | 1 to 1 | value |
| `SendText` | -- | 1 to 1 | value |
| `SetCapsLockState` | -- | 0 to 1 | value |
| `SetControlDelay` | -- | 1 to 1 | value |
| `SetDefaultMouseSpeed` | -- | 1 to 1 | value |
| `SetKeyDelay` | -- | 0 to 3 | value |
| `SetMouseDelay` | -- | 1 to 2 | value |
| `SetNumLockState` | -- | 0 to 1 | value |
| `SetRegView` | -- | 1 to 1 | value |
| `SetScrollLockState` | -- | 0 to 1 | value |
| `SetStoreCapsLockMode` | -- | 1 to 1 | value |
| `SetTimer` | -- | 0 to 3 | value |
| `SetTitleMatchMode` | -- | 1 to 1 | value |
| `SetWinDelay` | -- | 1 to 1 | value |
| `SetWorkingDir` | -- | 1 to 1 | value |
| `Shutdown` | -- | 1 to 1 | value |
| `Sleep` | -- | 1 to 1 | value |
| `SoundBeep` | -- | 0 to 2 | value |
| `SoundPlay` | -- | 1 to 2 | value |
| `StatusBarGetText` | -- | 0 to 5 | value |
| `StatusBarWait` | -- | 0 to 8 | value |
| `StrSplit` | -- | 1 to 4 | value |
| `Suspend` | -- | 0 to 1 | value |
| `SysGet` | -- | 1 to 1 | value |
| `SysGetIPAddresses` | -- | 0 to 0 | value |
| `Thread` | -- | 1 to 3 | value |
| `ToolTip` | -- | 0 to 4 | value |
| `TraySetIcon` | -- | 0 to 3 | value |
| `TrayTip` | -- | 0 to 3 | value |
| `WinActivate` | -- | 0 to 4 | value |
| `WinActivateBottom` | -- | 0 to 4 | value |
| `WinClose` | -- | 0 to 5 | value |
| `WinGetClass` | -- | 0 to 4 | value |
| `WinGetClientPos` | -- | 0 to 8 | value |
| `WinGetControls` | -- | 0 to 4 | value |
| `WinGetControlsHwnd` | -- | 0 to 4 | value |
| `WinGetCount` | -- | 0 to 4 | value |
| `WinGetExStyle` | -- | 0 to 4 | value |
| `WinGetID` | -- | 0 to 4 | value |
| `WinGetIDLast` | -- | 0 to 4 | value |
| `WinGetList` | -- | 0 to 4 | value |
| `WinGetMinMax` | -- | 0 to 4 | value |
| `WinGetPID` | -- | 0 to 4 | value |
| `WinGetPos` | -- | 0 to 8 | value |
| `WinGetProcessName` | -- | 0 to 4 | value |
| `WinGetProcessPath` | -- | 0 to 4 | value |
| `WinGetStyle` | -- | 0 to 4 | value |
| `WinGetText` | -- | 0 to 4 | value |
| `WinGetTitle` | -- | 0 to 4 | value |
| `WinGetTransColor` | -- | 0 to 4 | value |
| `WinGetTransparent` | -- | 0 to 4 | value |
| `WinHide` | -- | 0 to 4 | value |
| `WinKill` | -- | 0 to 5 | value |
| `WinMaximize` | -- | 0 to 4 | value |
| `WinMinimize` | -- | 0 to 4 | value |
| `WinMinimizeAll` | -- | 0 to 0 | value |
| `WinMinimizeAllUndo` | -- | 0 to 0 | value |
| `WinMove` | -- | 0 to 8 | value |
| `WinMoveBottom` | -- | 0 to 4 | value |
| `WinMoveTop` | -- | 0 to 4 | value |
| `WinRedraw` | -- | 0 to 4 | value |
| `WinRestore` | -- | 0 to 4 | value |
| `WinSetAlwaysOnTop` | -- | 0 to 5 | value |
| `WinSetEnabled` | -- | 1 to 5 | value |
| `WinSetExStyle` | -- | 1 to 5 | value |
| `WinSetRegion` | -- | 0 to 5 | value |
| `WinSetStyle` | -- | 1 to 5 | value |
| `WinSetTitle` | -- | 1 to 5 | value |
| `WinSetTransColor` | -- | 1 to 5 | value |
| `WinSetTransparent` | -- | 1 to 5 | value |
| `WinShow` | -- | 0 to 4 | value |
| `WinWait` | -- | 0 to 5 | value |
| `WinWaitActive` | -- | 0 to 5 | value |
| `WinWaitClose` | -- | 0 to 5 | value |
| `WinWaitNotActive` | -- | 0 to 5 | value |

## The functions

A function is a name a script calls with parentheses, or with the parameters of the line when the answer is not used.

| Name | Class | Parameters | State |
|---|---|---|---|
| `Abs` | -- | 1 to 1 | value |
| `ACos` | -- | 1 to 1 | value |
| `ASin` | -- | 1 to 1 | value |
| `ATan` | -- | 1 to 1 | value |
| `CaretGetPos` | -- | 0 to 2 | value |
| `Ceil` | -- | 1 to 1 | value |
| `Chr` | -- | 1 to 1 | value |
| `Click` | -- | 0 to 6 | value |
| `ComCall` | -- | 2 to any | value |
| `ComObjActive` | -- | 1 to 1 | value |
| `ComObjConnect` | -- | 1 to 2 | value |
| `ComObjFlags` | -- | 1 to 3 | value |
| `ComObjFromPtr` | -- | 1 to 1 | value |
| `ComObjGet` | -- | 1 to 1 | value |
| `ComObjQuery` | -- | 2 to 3 | value |
| `ComObjType` | -- | 1 to 2 | value |
| `ComObjValue` | -- | 1 to 1 | value |
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
| `ObjFromPtr` | -- | 1 to 1 | value |
| `ObjFromPtrAddRef` | -- | 1 to 1 | value |
| `ObjGetBase` | -- | 1 to 1 | value |
| `ObjGetCapacity` | -- | 1 to 1 | value |
| `ObjHasOwnProp` | -- | 2 to 2 | value |
| `ObjOwnPropCount` | -- | 1 to 1 | value |
| `ObjOwnProps` | -- | 1 to 1 | value |
| `ObjPtr` | -- | 1 to 1 | value |
| `ObjPtrAddRef` | -- | 1 to 1 | value |
| `ObjRelease` | -- | 1 to 1 | value |
| `ObjSetBase` | -- | 2 to 2 | value |
| `ObjSetCapacity` | -- | 2 to 2 | value |
| `Ord` | -- | 1 to 1 | value |
| `Random` | -- | 0 to 2 | value |
| `RegCreateKey` | -- | 0 to 1 | value |
| `RegDelete` | -- | 0 to 2 | value |
| `RegDeleteKey` | -- | 0 to 1 | value |
| `RegExMatch` | -- | 2 to 4 | value |
| `RegExReplace` | -- | 2 to 6 | value |
| `RegRead` | -- | 0 to 3 | value |
| `RegWrite` | -- | 1 to 4 | value |
| `Round` | -- | 1 to 2 | value |
| `RTrim` | -- | 1 to 2 | value |
| `RunWait` | -- | 1 to 4 | value |
| `Sin` | -- | 1 to 1 | value |
| `Sort` | -- | 1 to 3 | value |
| `SoundGetInterface` | -- | 1 to 3 | value |
| `SoundGetMute` | -- | 0 to 2 | value |
| `SoundGetName` | -- | 0 to 2 | value |
| `SoundGetVolume` | -- | 0 to 2 | value |
| `SoundSetMute` | -- | 1 to 3 | value |
| `SoundSetVolume` | -- | 1 to 3 | value |
| `SplitPath` | -- | 1 to 6 | value |
| `Sqrt` | -- | 1 to 1 | value |
| `StrCompare` | -- | 2 to 3 | value |
| `StrGet` | -- | 1 to 3 | value |
| `StrLen` | -- | 1 to 1 | value |
| `StrLower` | -- | 1 to 1 | value |
| `StrPtr` | -- | 1 to 1 | value |
| `StrPut` | -- | 1 to 4 | value |
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
| `AhkPath` | -- | -- | value |
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
| `Cursor` | -- | -- | value |
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
| `IconFile` | -- | -- | value |
| `IconHidden` | -- | -- | value |
| `IconNumber` | -- | -- | value |
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
| `ProgramFiles` | -- | -- | value |
| `Programs` | -- | -- | value |
| `ProgramsCommon` | -- | -- | value |
| `PtrSize` | -- | -- | value |
| `RegView` | -- | -- | value |
| `ScreenDPI` | -- | -- | value |
| `ScreenHeight` | -- | -- | value |
| `ScreenWidth` | -- | -- | value |
| `ScriptDir` | -- | -- | value |
| `ScriptFullPath` | -- | -- | value |
| `ScriptHwnd` | -- | -- | value |
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
| `TrayMenu` | -- | -- | value |
| `UserName` | -- | -- | value |
| `WDay` | -- | -- | value |
| `WinDelay` | -- | -- | value |
| `WinDir` | -- | -- | value |
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
| `Gui.ActiveX` | -- | -- | value |
| `Gui.Button` | -- | -- | value |
| `Gui.CheckBox` | -- | -- | value |
| `Gui.ComboBox` | -- | -- | value |
| `Gui.Control` | -- | -- | value |
| `Gui.Custom` | -- | -- | value |
| `Gui.DateTime` | -- | -- | value |
| `Gui.DDL` | -- | -- | value |
| `Gui.Edit` | -- | -- | value |
| `Gui.GroupBox` | -- | -- | value |
| `Gui.Hotkey` | -- | -- | value |
| `Gui.Link` | -- | -- | value |
| `Gui.List` | -- | -- | value |
| `Gui.ListBox` | -- | -- | value |
| `Gui.ListView` | -- | -- | value |
| `Gui.MonthCal` | -- | -- | value |
| `Gui.Pic` | -- | -- | value |
| `Gui.Progress` | -- | -- | value |
| `Gui.Radio` | -- | -- | value |
| `Gui.Slider` | -- | -- | value |
| `Gui.StatusBar` | -- | -- | value |
| `Gui.Tab` | -- | -- | value |
| `Gui.Text` | -- | -- | value |
| `Gui.TreeView` | -- | -- | value |
| `Gui.UpDown` | -- | -- | value |
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
| `ClassNN` | -- | value |
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
| `Text` | -- | value |
| `Text` | -- | value |
| `Type` | -- | value |
| `Value` | -- | value |
| `Value` | -- | value |
| `Visible` | -- | value |
| `Visible` | -- | value |

### Gui.DateTime

| Method | Parameters | State |
|---|---|---|
| `SetFormat` | -- | value |

### Gui.List

| Method | Parameters | State |
|---|---|---|
| `Add` | -- | value |
| `Choose` | -- | value |
| `Delete` | -- | value |

### Gui.ListView

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
| `SetIcon` | -- | value |
| `SetParts` | -- | value |
| `SetText` | -- | value |

### Gui.Tab

| Method | Parameters | State |
|---|---|---|
| `UseTab` | -- | value |

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
| `SetImageList` | -- | value |

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
