# Parser and script execution

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文](zh/index.md) |

Language: English

The names of the family **Parser and script execution** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `#CommentFlag` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#ErrorStdOut` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#EscapeChar` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#HotkeyInterval` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#HotkeyModifierTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#Hotstring` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#If` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#IfTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#IfWinActive` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#Include` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#InputLevel` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#InstallKeybdHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#InstallMouseHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#KeyHistory` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MaxHotkeysPerInterval` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MaxMem` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MaxThreads` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MaxThreadsBuffer` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MaxThreadsPerHotkey` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#MenuMaskKey` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#NoEnv` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#NoTrayIcon` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#Persistent` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#Requires` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#SingleInstance` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#UseHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#Warn` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `#WinActivateForce` | AutoHotkey v1.1.37.02 | directive | -- | -- | value |
| `A_AhkPath` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_AhkVersion` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_AutoTrim` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_BatchLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DefaultGui` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DefaultListView` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DefaultTreeView` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_EndChar` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ExitReason` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Index` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_InitialWorkingDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Is64bitOS` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsAdmin` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsCompiled` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsCritical` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsPaused` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsSuspended` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_IsUnicode` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_KeybdHookInstalled` | KeySharp | variable | -- | -- | value |
| `A_KsCorePath` | KeySharp | variable | -- | -- | value |
| `A_KsVersion` | KeySharp | variable | -- | -- | value |
| `A_LastError` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_LineFile` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_LineNumber` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ListLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_MaxThreads` | KeySharp | variable | -- | -- | value |
| `A_MaxThreadsBuffer` | KeySharp | variable | -- | -- | value |
| `A_MaxThreadsPerHotkey` | KeySharp | variable | -- | -- | value |
| `A_MouseHookInstalled` | KeySharp | variable | -- | -- | value |
| `A_NumBatchLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_OSType` | AutoHotkey v1.1.37.02 | variable | -- | -- | refused: this name is about Windows, and this machine is not. |
| `A_OSVersion` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Persistent` | KeySharp | variable | -- | -- | value |
| `A_PtrSize` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_RealThread` | KeySharp | variable | -- | -- | value |
| `A_ScriptDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ScriptFullPath` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ScriptHwnd` | AutoHotkey v1.1.37.02 | variable | -- | -- | refused: this runtime has no window of its own: it is a script that runs and ends. |
| `A_ScriptName` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Space` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Tab` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ThisFunc` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ThisLabel` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ThisMenu` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ThisMenuItem` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_ThisMenuItemPos` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_Timers` | KeySharp | variable | -- | -- | value |
| `A_WorkingDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `AllowMainWindow` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `Collect` | KeySharp | function | -- | -- | value |
| `CompileScript` | KeySharp | function | -- | -- | value |
| `CoordMode` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `CoordMode` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `Critical` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | refused: this runtime runs one script in one thread, and it cannot make it critical. |
| `Critical` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this runtime runs one script in one thread, and it cannot make it critical. |
| `Thread.Critical` | KeySharp | method | Thread | -- | value |
| `Else` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `EndChar` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `EnvGet` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `EnvGet` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `EnvSet` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `EnvSet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `ErrorLevel` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `Exit` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `Exit` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `Thread.Exit` | KeySharp | method | Thread | -- | value |
| `ExitApp` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `ExitApp` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `IfInString` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `IfMsgBox` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `IfNotInString` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `Index` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `Thread.Index` | KeySharp | method | Thread | -- | value |
| `InitialWorkingDir` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `Is64bitOS` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `IsAdmin` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `IsCompiled` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `IsComponentAvailable` | KeySharp | function | -- | -- | value |
| `IsCritical` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `IsFunc` | AutoHotkey v1.1.37.02 | function | -- | 1 | value |
| `IsPaused` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `IsSet` | AutoHotkey v1.1.37.02 | function | -- | 1 | value |
| `IsSuspended` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LineFile` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LineNumber` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `OnError` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `OnError` | AutoHotkey v1.1.37.02 | function | -- | 1 to 2 | value |
| `OnExit` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `OnMessage` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `OnMessage` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `OutputDebugLine` | KeySharp | function | -- | -- | value |
| `Pause` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `Pause` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Persistent` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `Reload` | AutoHotkey v2.0.19 | command | -- | 0 | refused: this runtime runs one script once, and it cannot run itself again. |
| `Reload` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RequestCapabilities` | KeySharp | function | -- | -- | value |
| `RunScript` | KeySharp | function | -- | -- | value |
| `ScriptDir` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `ScriptFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `ScriptName` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `SetBatchLines` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `SetTimer` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | value |
| `SetTimer` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `SetWorkingDir` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `SetWorkingDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `ShowDebug` | KeySharp | function | -- | -- | value |
| `Sleep` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `Sleep` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `Space` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `Suspend` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `Suspend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Tab` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `ThisFunc` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `Thread` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | refused: this runtime runs one script in one thread, and it cannot start another. |
| `Thread` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this runtime runs one script in one thread, and it cannot start another. |
| `ValidateScript` | KeySharp | function | -- | -- | value |
| `WorkingDir` | AutoHotkey v2.0.19 | variable | -- | -- | value |
