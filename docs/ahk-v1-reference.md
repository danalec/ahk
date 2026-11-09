# The surface of v1.1.37.02

Every name of AutoHotkey v1.1.37.02, with what this runtime answers for it.

The list was measured on the binary of v1.1.37.02 (`crates/ahk/reference/ahk-v1.1.37.02.surface.tsv`), and the state of each name is the answer of `ahk --surface --dialect=v1`. The summary is:

> **408 entries: 301 answer with a value, 107 are refused, 0 are pending.**

The v1 dialect is chosen with `--dialect=v1`; see the language guide for what the dialect means. A command is written with commas and no parentheses (`StringLen, n, text`), and a function with parentheses (`StrLen(text)`).

**The shape of a parameter** says what the parameter is, in order:

| Shape | What it is |
|---|---|
| `Value` | an expression: the value reaches the handler |
| `Variable` | the **name** of a variable of the script, read by the command |
| `Output` | the **name** of a variable of the script that the command writes into |

**The number of the parameters**: `any` is the 255 of the table, which means the v1 ignores the parameters a command does not need. A command with `0` takes none.

Regenerate this chapter with `python egm-review/gen_docs_reference.py`.

## The commands

A command is a name a script writes at the start of a line, with commas between the parameters.

| Name | Parameters | Shape | State |
|---|---|---|---|
| `AutoTrim` | 1 to any | `Value` | value |
| `BlockInput` | 1 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Click` | 0 to any | `Value` | value |
| `ClipWait` | 0 to any | `Value,Value` | value |
| `Control` | 1 to any | `Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlClick` | 0 to any | `Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlFocus` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGet` | 2 to any | `Output,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetFocus` | 1 to any | `Output,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetPos` | 0 to any | `Output,Output,Output,Output,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlGetText` | 1 to any | `Output,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlMove` | 0 to any | `Value,Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSend` | 2 to any | `Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ControlSetText` | 0 to any | `Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `CoordMode` | 1 to any | `Value,Value` | value |
| `Critical` | 0 to any | `Value` | refused: this runtime runs one script in one thread, and it cannot make it critical |
| `DetectHiddenText` | 1 to any | `Value` | value |
| `DetectHiddenWindows` | 1 to any | `Value` | value |
| `Drive` | 1 to any | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveGet` | 0 to any | `Output,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `DriveSpaceFree` | 2 to any | `Output,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Edit` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `Else` | 0 to any | -- | value |
| `EnvAdd` | 2 to any | `Output,Value,Value` | value |
| `EnvDiv` | 2 to any | `Output,Value` | value |
| `EnvGet` | 2 to any | `Output,Value` | value |
| `EnvMult` | 2 to any | `Output,Value` | value |
| `EnvSet` | 1 to any | `Name,Value` | value |
| `EnvSub` | 2 to any | `Output,Value,Value` | value |
| `EnvUpdate` | 0 to any | -- | value |
| `Exit` | 0 to any | `Value` | value |
| `ExitApp` | 0 to any | `Value` | value |
| `FileAppend` | 0 to any | `Value,Value,Value` | value |
| `FileCopy` | 2 to any | `Value,Value,Value` | value |
| `FileCopyDir` | 2 to any | `Value,Value,Value` | value |
| `FileCreateDir` | 1 to any | `Value` | value |
| `FileCreateShortcut` | 2 to any | `Value,Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `FileDelete` | 1 to any | `Value` | value |
| `FileEncoding` | 0 to any | `Value` | value |
| `FileGetAttrib` | 1 to any | `Output,Value` | value |
| `FileGetShortcut` | 1 to any | `Value,Output,Output,Output,Output,Output,Output,Output` | refused: this is a name of Windows, and this desktop is not Windows |
| `FileGetSize` | 1 to any | `Output,Value,Value` | value |
| `FileGetTime` | 1 to any | `Output,Value,Value` | value |
| `FileGetVersion` | 1 to any | `Output,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `FileInstall` | 2 to any | `Value,Value,Value` | value |
| `FileMove` | 2 to any | `Value,Value,Value` | value |
| `FileMoveDir` | 2 to any | `Value,Value,Value` | value |
| `FileRead` | 2 to any | `Output,Value` | value |
| `FileReadLine` | 3 to any | `Output,Value,Value` | value |
| `FileRecycle` | 1 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `FileRecycleEmpty` | 0 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `FileRemoveDir` | 1 to any | `Value,Value` | value |
| `FileSelectFile` | 1 to any | `Output,Value,Value,Value,Value` | value |
| `FileSelectFolder` | 1 to any | `Output,Value,Value,Value` | value |
| `FileSetAttrib` | 1 to any | `Value,Value,Value,Value` | value |
| `FileSetTime` | 0 to any | `Value,Value,Value,Value,Value` | value |
| `FormatTime` | 1 to any | `Output,Value,Value` | value |
| `GetKeyState` | 2 to any | `Output,Value,Value` | value |
| `GroupActivate` | 1 to any | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupAdd` | 1 to any | `Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupClose` | 1 to any | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `GroupDeactivate` | 1 to any | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Gui` | 1 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `GuiControl` | 2 to any | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `GuiControlGet` | 1 to any | `Output,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Hotkey` | 1 to any | `Value,Value,Value` | value |
| `IfInString` | 2 to any | `Variable,Value` | value |
| `IfMsgBox` | 1 to any | `Value` | value |
| `IfNotInString` | 2 to any | `Variable,Value` | value |
| `ImageSearch` | 0 to any | `Output,Output,Value,Value,Value,Value,Value` | value |
| `IniDelete` | 2 to any | `Value,Value,Value` | value |
| `IniRead` | 2 to any | `Output,Value,Value,Value,Value` | value |
| `IniWrite` | 3 to any | `Value,Value,Value,Value` | value |
| `Input` | 0 to any | `Output,Value,Value,Value` | value |
| `InputBox` | 1 to any | `Output,Value,Value,Value,Value,Value,Value,Value,Value,Value,Value` | value |
| `KeyHistory` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `KeyWait` | 1 to any | `Value,Value` | value |
| `ListHotkeys` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `ListLines` | 0 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ListVars` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `Menu` | 2 to any | `Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `MouseClick` | 0 to any | `Value,Value,Value,Value,Value,Value,Value` | value |
| `MouseClickDrag` | 1 to any | `Value,Value,Value,Value,Value,Value,Value` | value |
| `MouseGetPos` | 0 to any | `Output,Output,Output,Output,Value` | value |
| `MouseMove` | 2 to any | `Value,Value,Value,Value` | value |
| `MsgBox` | 0 to any | `Value,Value,Value,Value` | value |
| `OutputDebug` | 1 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Pause` | 0 to any | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `PixelGetColor` | 3 to any | `Output,Value,Value,Value` | value |
| `PixelSearch` | 0 to any | `Output,Output,Value,Value,Value,Value,Value,Value,Value` | value |
| `PostMessage` | 1 to any | `Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Process` | 1 to any | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Progress` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Random` | 0 to any | `Output,Value,Value` | value |
| `RegDelete` | 0 to any | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `RegRead` | 1 to any | `Output,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `RegWrite` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Reload` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `Run` | 1 to any | `Value,Value,Value,Output` | value |
| `RunAs` | 0 to any | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `RunWait` | 1 to any | `Value,Value,Value,Output` | value |
| `Send` | 1 to any | `Value` | value |
| `SendLevel` | 1 to any | `Value` | value |
| `SendMode` | 1 to any | `Value` | value |
| `SetBatchLines` | 1 to any | `Value` | value |
| `SetControlDelay` | 1 to any | `Value` | value |
| `SetDefaultMouseSpeed` | 1 to any | `Value` | value |
| `SetEnv` | 1 to any | `Output,Value` | value |
| `SetFormat` | 2 to any | `Value,Value` | value |
| `SetKeyDelay` | 0 to any | `Value,Value,Value` | value |
| `SetMouseDelay` | 1 to any | `Value,Value` | value |
| `SetRegView` | 1 to any | `Value` | value |
| `SetStoreCapsLockMode` | 1 to any | `Value` | value |
| `SetTimer` | 0 to any | `Value,Value,Value` | value |
| `SetTitleMatchMode` | 1 to any | `Value` | value |
| `SetWinDelay` | 1 to any | `Value` | value |
| `SetWorkingDir` | 1 to any | `Value` | value |
| `Shutdown` | 1 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Sleep` | 1 to any | `Value` | value |
| `Sort` | 1 to any | `Variable,Value` | value |
| `SoundBeep` | 0 to any | `Value,Value` | value |
| `SoundGet` | 1 to any | `Output,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SoundGetWaveVolume` | 1 to any | `Output,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SoundPlay` | 1 to any | `Value,Value` | value |
| `SoundSet` | 1 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SoundSetWaveVolume` | 1 to any | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SplashTextOn` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SplitPath` | 1 to any | `Variable,Output,Output,Output,Output,Output` | value |
| `StatusBarGetText` | 1 to any | `Output,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `StatusBarWait` | 0 to any | `Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `StringCaseSense` | 1 to any | `Value` | value |
| `StringGetPos` | 3 to any | `Output,Variable,Value,Value,Value` | value |
| `StringLeft` | 3 to any | `Output,Variable,Value` | value |
| `StringLen` | 2 to any | `Output,Variable` | value |
| `StringLower` | 2 to any | `Output,Variable,Value` | value |
| `StringMid` | 3 to any | `Output,Variable,Value,Value,Value` | value |
| `StringReplace` | 3 to any | `Output,Variable,Value,Value,Value` | value |
| `StringSplit` | 2 to any | `Output,Variable,Value,Value` | value |
| `StringTrimLeft` | 3 to any | `Output,Variable,Value` | value |
| `Suspend` | 0 to any | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `SysGet` | 2 to any | `Output,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Thread` | 1 to any | `Value,Value,Value` | refused: this runtime runs one script in one thread, and it cannot start another |
| `ToolTip` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Transform` | 2 to any | `Output,Value,Value,Value` | value |
| `TrayTip` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `URLDownloadToFile` | 2 to any | `Value,Value` | value |
| `WinActivate` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinActivateBottom` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinClose` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGet` | 1 to any | `Output,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetActiveStats` | 5 to any | `Output,Output,Output,Output,Output` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetActiveTitle` | 1 to any | `Output` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetClass` | 1 to any | `Output,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetPos` | 0 to any | `Output,Output,Output,Output,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetText` | 1 to any | `Output,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinGetTitle` | 1 to any | `Output,Value,Value,Value,Value` | value |
| `WinHide` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinKill` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMaximize` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMenuSelectItem` | 3 to any | `Value,Value,Value,Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMinimize` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMinimizeAll` | 0 to any | -- | refused: this is a name of Windows, and this desktop is not Windows |
| `WinMove` | 0 to any | `Value,Value,Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinRestore` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSet` | 1 to any | `Value,Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinSetTitle` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinShow` | 0 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWait` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWaitActive` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `WinWaitClose` | 0 to any | `Value,Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |

## The functions

A function is a name a script calls with parentheses. The v1 has the function of the v2 beside the command of the v1: `StrLen(text)` is the function, `StringLen, n, text` the command.

| Name | Parameters | Shape | State |
|---|---|---|---|
| `Abs` | 1 to 1 | `Value` | value |
| `ACos` | 1 to 1 | `Value` | value |
| `Asc` | 1 to 1 | `Value` | value |
| `ASin` | 1 to 1 | `Value` | value |
| `ATan` | 1 to 1 | `Value` | value |
| `Ceil` | 1 to 1 | `Value` | value |
| `Chr` | 1 to 1 | `Value` | value |
| `ComObjActive` | 0 to 3 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjArray` | 2 to any | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjConnect` | 1 to 2 | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjCreate` | 1 to 2 | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjError` | 0 to 1 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjFlags` | 1 to 3 | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjGet` | 1 to 1 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjQuery` | 2 to 3 | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjType` | 1 to 2 | `Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `ComObjValue` | 1 to 1 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Cos` | 1 to 1 | `Value` | value |
| `DllCall` | 1 to any | `Value,Value,Value,Value,Value,Value` | value |
| `Exp` | 1 to 1 | `Value` | value |
| `FileExist` | 1 to 1 | `Value` | value |
| `FileOpen` | 2 to 3 | `Value,Value,Value` | value |
| `Floor` | 1 to 1 | `Value` | value |
| `Format` | 1 to any | `Value,Value` | value |
| `Func` | 1 to 1 | `Value` | value |
| `GetKeyState` | 1 to 2 | `Value,Value` | value |
| `Hotstring` | 1 to 3 | `Value,Value,Value` | value |
| `InputHook` | 0 to 3 | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `InStr` | 2 to 5 | `Value,Value,Value,Value,Value` | value |
| `IsByRef` | 1 to 1 | `Value` | value |
| `IsFunc` | 1 to 1 | `Value` | value |
| `IsLabel` | 1 to 1 | `Value` | value |
| `IsObject` | 1 to any | `Value` | value |
| `IsSet` | 1 to 1 | `Value` | value |
| `Ln` | 1 to 1 | `Value` | value |
| `LoadPicture` | 1 to 3 | `Value,Value,Output` | value |
| `Log` | 1 to 1 | `Value` | value |
| `Max` | 1 to any | `Value` | value |
| `MenuGetHandle` | 1 to 1 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `MenuGetName` | 1 to 1 | `Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Min` | 1 to any | `Value` | value |
| `Mod` | 2 to 2 | `Value` | value |
| `NumGet` | 1 to 3 | `Value,Value,Value` | value |
| `NumPut` | 2 to 4 | `Value,Value,Value,Value` | value |
| `ObjAddRef` | 1 to 1 | `Value` | value |
| `ObjBindMethod` | 2 to any | `Value,Value,Value` | value |
| `Object` | 0 to any | `Value,Value,Value,Value` | value |
| `OnError` | 1 to 2 | `Value,Value` | value |
| `OnMessage` | 1 to 3 | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Ord` | 1 to 1 | `Value` | value |
| `RegExMatch` | 2 to 4 | `Value,Value,Output,Value` | value |
| `RegExReplace` | 2 to 6 | `Value,Value,Value,Output,Value,Value` | value |
| `RegisterCallback` | 1 to 4 | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `Round` | 1 to 2 | `Value` | value |
| `Sin` | 1 to 1 | `Value` | value |
| `Sqrt` | 1 to 1 | `Value` | value |
| `StrGet` | 1 to 3 | `Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `StrLen` | 1 to 1 | `Value` | value |
| `StrPut` | 1 to 4 | `Value,Value,Value,Value` | refused: this is a name of Windows, and this desktop is not Windows |
| `StrReplace` | 2 to 5 | `Value,Value,Value,Output,Value` | value |
| `StrSplit` | 1 to 4 | `Value,Value,Value,Value` | value |
| `SubStr` | 2 to 3 | `Value,Value,Value` | value |
| `Tan` | 1 to 1 | `Value` | value |
| `Trim` | 1 to 2 | `Value,Value` | value |
| `VarSetCapacity` | 1 to 3 | `Value,Value,Value` | value |
| `VerCompare` | 2 to 2 | `Value,Value` | value |
| `WinActive` | 0 to 4 | `Value,Value,Value,Value` | value |
| `WinExist` | 0 to 4 | `Value,Value,Value,Value` | value |

## The built-in variables

The v1 spells its variables with the `A_` prefix, as the v2 does, and `ErrorLevel` is the one that has no prefix.

| Name | State |
|---|---|
| `A_AhkPath` | refused: this runtime is not AutoHotkey, and it has no path of an executable |
| `A_AhkVersion` | value |
| `A_AppData` | value |
| `A_AppDataCommon` | value |
| `A_Args` | value |
| `A_AutoTrim` | value |
| `A_BatchLines` | value |
| `A_CaretX` | value |
| `A_CaretY` | value |
| `A_Clipboard` | value |
| `A_ComputerName` | value |
| `A_ComSpec` | value |
| `A_ControlDelay` | value |
| `A_CoordModeCaret` | value |
| `A_CoordModeMenu` | value |
| `A_CoordModeMouse` | value |
| `A_CoordModePixel` | value |
| `A_CoordModeToolTip` | value |
| `A_Cursor` | refused: the shape of the cursor of the machine is not read by this runtime |
| `A_DD` | value |
| `A_DDD` | value |
| `A_DDDD` | value |
| `A_DefaultGui` | value |
| `A_DefaultListView` | value |
| `A_DefaultMouseSpeed` | value |
| `A_DefaultTreeView` | value |
| `A_Desktop` | value |
| `A_DesktopCommon` | value |
| `A_DetectHiddenText` | value |
| `A_DetectHiddenWindows` | value |
| `A_EndChar` | value |
| `A_EventInfo` | value |
| `A_ExitReason` | value |
| `A_FileEncoding` | value |
| `A_FormatFloat` | value |
| `A_FormatInteger` | value |
| `A_Gui` | value |
| `A_GuiControl` | value |
| `A_GuiControlEvent` | value |
| `A_GuiEvent` | value |
| `A_GuiHeight` | value |
| `A_GuiWidth` | value |
| `A_GuiX` | value |
| `A_GuiY` | value |
| `A_Hour` | value |
| `A_IconFile` | refused: this runtime has no tray icon |
| `A_IconHidden` | value |
| `A_IconNumber` | refused: this runtime has no tray icon |
| `A_IconTip` | value |
| `A_Index` | value |
| `A_InitialWorkingDir` | value |
| `A_IPAddress1` | refused: this desktop does not know the address of the network of the machine |
| `A_Is64bitOS` | value |
| `A_IsAdmin` | value |
| `A_IsCompiled` | value |
| `A_IsCritical` | value |
| `A_IsPaused` | value |
| `A_IsSuspended` | value |
| `A_IsUnicode` | value |
| `A_KeyDelay` | value |
| `A_KeyDelayPlay` | value |
| `A_KeyDuration` | value |
| `A_KeyDurationPlay` | value |
| `A_Language` | value |
| `A_LastError` | value |
| `A_LineFile` | value |
| `A_LineNumber` | value |
| `A_ListLines` | value |
| `A_LoopField` | value |
| `A_LoopFileName` | value |
| `A_LoopReadLine` | value |
| `A_LoopRegName` | value |
| `A_MDay` | value |
| `A_Min` | value |
| `A_MM` | value |
| `A_MMM` | value |
| `A_MMMM` | value |
| `A_Mon` | value |
| `A_MouseDelay` | value |
| `A_MouseDelayPlay` | value |
| `A_MSec` | value |
| `A_MyDocuments` | value |
| `A_Now` | value |
| `A_NowUTC` | value |
| `A_NumBatchLines` | value |
| `A_OSType` | refused: this name is about Windows, and this machine is not |
| `A_OSVersion` | value |
| `A_PriorHotkey` | value |
| `A_PriorKey` | value |
| `A_ProgramFiles` | refused: this is a name of Windows, and this desktop is not Windows |
| `A_Programs` | value |
| `A_ProgramsCommon` | value |
| `A_PtrSize` | value |
| `A_RegView` | value |
| `A_ScreenDPI` | refused: this desktop does not know the scale of the output |
| `A_ScreenHeight` | value |
| `A_ScreenWidth` | value |
| `A_ScriptDir` | value |
| `A_ScriptFullPath` | value |
| `A_ScriptHwnd` | refused: this runtime has no window of its own: it is a script that runs and ends |
| `A_ScriptName` | value |
| `A_Sec` | value |
| `A_SendLevel` | value |
| `A_SendMode` | value |
| `A_Space` | value |
| `A_StartMenu` | value |
| `A_StartMenuCommon` | value |
| `A_Startup` | value |
| `A_StartupCommon` | value |
| `A_StoreCapsLockMode` | value |
| `A_StringCaseSense` | value |
| `A_Tab` | value |
| `A_Temp` | value |
| `A_ThisFunc` | value |
| `A_ThisHotkey` | value |
| `A_ThisLabel` | value |
| `A_ThisMenu` | value |
| `A_ThisMenuItem` | value |
| `A_ThisMenuItemPos` | value |
| `A_TickCount` | value |
| `A_TimeIdle` | value |
| `A_TimeIdleKeyboard` | value |
| `A_TimeIdleMouse` | value |
| `A_TimeIdlePhysical` | value |
| `A_TimeSincePriorHotkey` | value |
| `A_TimeSinceThisHotkey` | value |
| `A_TitleMatchMode` | value |
| `A_TitleMatchModeSpeed` | value |
| `A_UserName` | value |
| `A_WDay` | value |
| `A_WinDelay` | value |
| `A_WinDir` | refused: this is a name of Windows, and this desktop is not Windows |
| `A_WorkingDir` | value |
| `A_YDay` | value |
| `A_Year` | value |
| `A_YWeek` | value |
| `A_YYYY` | value |
| `Clipboard` | value |
| `ComSpec` | value |
| `ErrorLevel` | value |
| `ProgramFiles` | refused: this is a name of Windows, and this desktop is not Windows |

## The directives

A directive begins with `#` and is read when the script loads.

| Name | State |
|---|---|
| `#ClipboardTimeout` | value |
| `#CommentFlag` | value |
| `#ErrorStdOut` | value |
| `#EscapeChar` | value |
| `#HotkeyInterval` | value |
| `#HotkeyModifierTimeout` | value |
| `#Hotstring` | value |
| `#If` | value |
| `#IfTimeout` | value |
| `#IfWinActive` | value |
| `#Include` | value |
| `#InputLevel` | value |
| `#InstallKeybdHook` | value |
| `#InstallMouseHook` | value |
| `#KeyHistory` | value |
| `#MaxHotkeysPerInterval` | value |
| `#MaxMem` | value |
| `#MaxThreads` | value |
| `#MaxThreadsBuffer` | value |
| `#MaxThreadsPerHotkey` | value |
| `#MenuMaskKey` | value |
| `#NoEnv` | value |
| `#NoTrayIcon` | value |
| `#Persistent` | value |
| `#Requires` | value |
| `#SingleInstance` | value |
| `#UseHook` | value |
| `#Warn` | value |
| `#WinActivateForce` | value |
