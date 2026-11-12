# Parser and script execution

The names of the family **Parser and script execution** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `#CommentFlag` | v1 | directive | -- | -- | value |
| `#ErrorStdOut` | v1 | directive | -- | -- | value |
| `#EscapeChar` | v1 | directive | -- | -- | value |
| `#HotkeyInterval` | v1 | directive | -- | -- | value |
| `#HotkeyModifierTimeout` | v1 | directive | -- | -- | value |
| `#Hotstring` | v1 | directive | -- | -- | value |
| `#If` | v1 | directive | -- | -- | value |
| `#IfTimeout` | v1 | directive | -- | -- | value |
| `#IfWinActive` | v1 | directive | -- | -- | value |
| `#Include` | v1 | directive | -- | -- | value |
| `#InputLevel` | v1 | directive | -- | -- | value |
| `#InstallKeybdHook` | v1 | directive | -- | -- | value |
| `#InstallMouseHook` | v1 | directive | -- | -- | value |
| `#KeyHistory` | v1 | directive | -- | -- | value |
| `#MaxHotkeysPerInterval` | v1 | directive | -- | -- | value |
| `#MaxMem` | v1 | directive | -- | -- | value |
| `#MaxThreads` | v1 | directive | -- | -- | value |
| `#MaxThreadsBuffer` | v1 | directive | -- | -- | value |
| `#MaxThreadsPerHotkey` | v1 | directive | -- | -- | value |
| `#MenuMaskKey` | v1 | directive | -- | -- | value |
| `#NoEnv` | v1 | directive | -- | -- | value |
| `#NoTrayIcon` | v1 | directive | -- | -- | value |
| `#Persistent` | v1 | directive | -- | -- | value |
| `#Requires` | v1 | directive | -- | -- | value |
| `#SingleInstance` | v1 | directive | -- | -- | value |
| `#UseHook` | v1 | directive | -- | -- | value |
| `#Warn` | v1 | directive | -- | -- | value |
| `#WinActivateForce` | v1 | directive | -- | -- | value |
| `A_AhkPath` | v1 | variable | -- | -- | value |
| `A_AhkVersion` | v1 | variable | -- | -- | value |
| `A_AutoTrim` | v1 | variable | -- | -- | value |
| `A_BatchLines` | v1 | variable | -- | -- | value |
| `A_DefaultGui` | v1 | variable | -- | -- | value |
| `A_DefaultListView` | v1 | variable | -- | -- | value |
| `A_DefaultTreeView` | v1 | variable | -- | -- | value |
| `A_EndChar` | v1 | variable | -- | -- | value |
| `A_ExitReason` | v1 | variable | -- | -- | value |
| `A_Index` | v1 | variable | -- | -- | value |
| `A_InitialWorkingDir` | v1 | variable | -- | -- | value |
| `A_Is64bitOS` | v1 | variable | -- | -- | value |
| `A_IsAdmin` | v1 | variable | -- | -- | value |
| `A_IsCompiled` | v1 | variable | -- | -- | value |
| `A_IsCritical` | v1 | variable | -- | -- | value |
| `A_IsPaused` | v1 | variable | -- | -- | value |
| `A_IsSuspended` | v1 | variable | -- | -- | value |
| `A_IsUnicode` | v1 | variable | -- | -- | value |
| `A_KeybdHookInstalled` | ks | variable | -- | -- | value |
| `A_KsCorePath` | ks | variable | -- | -- | value |
| `A_KsVersion` | ks | variable | -- | -- | value |
| `A_LastError` | v1 | variable | -- | -- | value |
| `A_LineFile` | v1 | variable | -- | -- | value |
| `A_LineNumber` | v1 | variable | -- | -- | value |
| `A_ListLines` | v1 | variable | -- | -- | value |
| `A_MaxThreads` | ks | variable | -- | -- | value |
| `A_MaxThreadsBuffer` | ks | variable | -- | -- | value |
| `A_MaxThreadsPerHotkey` | ks | variable | -- | -- | value |
| `A_MouseHookInstalled` | ks | variable | -- | -- | value |
| `A_NumBatchLines` | v1 | variable | -- | -- | value |
| `A_OSType` | v1 | variable | -- | -- | refused: this name is about Windows, and this machine is not. |
| `A_OSVersion` | v1 | variable | -- | -- | value |
| `A_Persistent` | ks | variable | -- | -- | value |
| `A_PtrSize` | v1 | variable | -- | -- | value |
| `A_RealThread` | ks | variable | -- | -- | value |
| `A_ScriptDir` | v1 | variable | -- | -- | value |
| `A_ScriptFullPath` | v1 | variable | -- | -- | value |
| `A_ScriptHwnd` | v1 | variable | -- | -- | refused: this runtime has no window of its own: it is a script that runs and ends. |
| `A_ScriptName` | v1 | variable | -- | -- | value |
| `A_Space` | v1 | variable | -- | -- | value |
| `A_Tab` | v1 | variable | -- | -- | value |
| `A_ThisFunc` | v1 | variable | -- | -- | value |
| `A_ThisLabel` | v1 | variable | -- | -- | value |
| `A_ThisMenu` | v1 | variable | -- | -- | value |
| `A_ThisMenuItem` | v1 | variable | -- | -- | value |
| `A_ThisMenuItemPos` | v1 | variable | -- | -- | value |
| `A_Timers` | ks | variable | -- | -- | value |
| `A_WorkingDir` | v1 | variable | -- | -- | value |
| `AllowMainWindow` | v2 | variable | -- | -- | value |
| `Collect` | ks | function | -- | -- | value |
| `CompileScript` | ks | function | -- | -- | value |
| `CoordMode` | v2 | command | -- | 1 to 2 | value |
| `CoordMode` | v1 | command | -- | up to 1 | value |
| `Critical` | v2 | command | -- | 0 to 1 | refused: this runtime runs one script in one thread, and it cannot make it critical. |
| `Critical` | v1 | command | -- | up to 0 | refused: this runtime runs one script in one thread, and it cannot make it critical. |
| `Thread.Critical` | ks | method | Thread | -- | value |
| `Else` | v1 | command | -- | up to 0 | value |
| `EndChar` | v2 | variable | -- | -- | value |
| `EnvGet` | v2 | command | -- | 1 | value |
| `EnvGet` | v1 | command | -- | up to 2 | value |
| `EnvSet` | v2 | command | -- | 1 to 2 | value |
| `EnvSet` | v1 | command | -- | up to 1 | value |
| `ErrorLevel` | v1 | variable | -- | -- | value |
| `Exit` | v2 | command | -- | 0 to 1 | value |
| `Exit` | v1 | command | -- | up to 0 | value |
| `Thread.Exit` | ks | method | Thread | -- | value |
| `ExitApp` | v2 | command | -- | 0 to 1 | value |
| `ExitApp` | v1 | command | -- | up to 0 | value |
| `IfInString` | v1 | command | -- | up to 2 | value |
| `IfMsgBox` | v1 | command | -- | up to 1 | value |
| `IfNotInString` | v1 | command | -- | up to 2 | value |
| `Index` | v2 | variable | -- | -- | value |
| `Thread.Index` | ks | method | Thread | -- | value |
| `InitialWorkingDir` | v2 | variable | -- | -- | value |
| `Is64bitOS` | v2 | variable | -- | -- | value |
| `IsAdmin` | v2 | variable | -- | -- | value |
| `IsCompiled` | v2 | variable | -- | -- | value |
| `IsComponentAvailable` | ks | function | -- | -- | value |
| `IsCritical` | v2 | variable | -- | -- | value |
| `IsFunc` | v1 | function | -- | 1 | value |
| `IsPaused` | v2 | variable | -- | -- | value |
| `IsSet` | v1 | function | -- | 1 | value |
| `IsSuspended` | v2 | variable | -- | -- | value |
| `LineFile` | v2 | variable | -- | -- | value |
| `LineNumber` | v2 | variable | -- | -- | value |
| `OnError` | v2 | command | -- | 1 to 2 | value |
| `OnError` | v1 | function | -- | 1 to 2 | value |
| `OnExit` | v2 | command | -- | 1 to 2 | value |
| `OnMessage` | v2 | command | -- | 2 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `OnMessage` | v1 | function | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `OutputDebugLine` | ks | function | -- | -- | value |
| `Pause` | v2 | command | -- | 0 to 1 | value |
| `Pause` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Persistent` | v2 | command | -- | 0 to 1 | refused: this runtime is a script that ends when it ends, and it cannot be kept alive. |
| `Reload` | v2 | command | -- | 0 | refused: this runtime runs one script once, and it cannot run itself again. |
| `Reload` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RequestCapabilities` | ks | function | -- | -- | value |
| `RunScript` | ks | function | -- | -- | value |
| `ScriptDir` | v2 | variable | -- | -- | value |
| `ScriptFullPath` | v2 | variable | -- | -- | value |
| `ScriptName` | v2 | variable | -- | -- | value |
| `SetBatchLines` | v1 | command | -- | up to 1 | value |
| `SetTimer` | v2 | command | -- | 0 to 3 | value |
| `SetTimer` | v1 | command | -- | up to 0 | value |
| `SetWorkingDir` | v2 | command | -- | 1 | value |
| `SetWorkingDir` | v1 | command | -- | up to 1 | value |
| `ShowDebug` | ks | function | -- | -- | value |
| `Sleep` | v2 | command | -- | 1 | value |
| `Sleep` | v1 | command | -- | up to 1 | value |
| `Space` | v2 | variable | -- | -- | value |
| `Suspend` | v2 | command | -- | 0 to 1 | value |
| `Suspend` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Tab` | v2 | variable | -- | -- | value |
| `ThisFunc` | v2 | variable | -- | -- | value |
| `Thread` | v2 | command | -- | 1 to 3 | refused: this runtime runs one script in one thread, and it cannot start another. |
| `Thread` | v1 | command | -- | up to 1 | refused: this runtime runs one script in one thread, and it cannot start another. |
| `ValidateScript` | ks | function | -- | -- | value |
| `WorkingDir` | v2 | variable | -- | -- | value |
