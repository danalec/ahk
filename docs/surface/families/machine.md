# The machine, the processes and the environment

The names of the family **The machine, the processes and the environment** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `A_AppData` | v1 | variable | -- | -- | value |
| `A_AppDataCommon` | v1 | variable | -- | -- | value |
| `A_Args` | v1 | variable | -- | -- | value |
| `A_ComputerName` | v1 | variable | -- | -- | value |
| `A_ComSpec` | v1 | variable | -- | -- | value |
| `A_DefaultMouseSpeed` | v1 | variable | -- | -- | value |
| `A_Desktop` | v1 | variable | -- | -- | value |
| `A_DesktopCommon` | v1 | variable | -- | -- | value |
| `A_EventInfo` | v1 | variable | -- | -- | value |
| `A_FileEncoding` | v1 | variable | -- | -- | value |
| `A_IPAddress1` | v1 | variable | -- | -- | refused: this desktop does not know the address of the network of the machine. |
| `A_Language` | v1 | variable | -- | -- | value |
| `A_MDay` | v1 | variable | -- | -- | value |
| `A_MM` | v1 | variable | -- | -- | value |
| `A_MMM` | v1 | variable | -- | -- | value |
| `A_MMMM` | v1 | variable | -- | -- | value |
| `A_MyDocuments` | v1 | variable | -- | -- | value |
| `A_NoTrayIcon` | ks | variable | -- | -- | value |
| `A_NowMs` | ks | variable | -- | -- | value |
| `A_NowUTCMs` | ks | variable | -- | -- | value |
| `A_ProgramFiles` | v1 | variable | -- | -- | refused: this is a name of Windows, and this desktop is not Windows. |
| `A_Programs` | v1 | variable | -- | -- | value |
| `A_ProgramsCommon` | v1 | variable | -- | -- | value |
| `A_StartMenu` | v1 | variable | -- | -- | value |
| `A_StartMenuCommon` | v1 | variable | -- | -- | value |
| `A_Startup` | v1 | variable | -- | -- | value |
| `A_StartupCommon` | v1 | variable | -- | -- | value |
| `A_Temp` | v1 | variable | -- | -- | value |
| `A_Thread` | ks | variable | -- | -- | value |
| `A_TickCount` | v1 | variable | -- | -- | value |
| `A_UserName` | v1 | variable | -- | -- | value |
| `A_WinActivateForce` | ks | variable | -- | -- | value |
| `A_YWeek` | v1 | variable | -- | -- | value |
| `AhkPath` | v2 | variable | -- | -- | value |
| `AhkVersion` | v2 | variable | -- | -- | value |
| `Thread.AllowTimers` | ks | method | Thread | -- | value |
| `AppData` | v2 | variable | -- | -- | value |
| `AppDataCommon` | v2 | variable | -- | -- | value |
| `PropRef.Args` | ks | method | PropRef | -- | value |
| `Struct.At` | ks | method | Struct | -- | value |
| `ComputerName` | v2 | variable | -- | -- | value |
| `ComSpec` | v2 | variable | -- | -- | value |
| `ComSpec` | v1 | variable | -- | -- | value |
| `Desktop` | v2 | variable | -- | -- | value |
| `DesktopCommon` | v2 | variable | -- | -- | value |
| `Drive` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveGet` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveSpaceFree` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Thread.Elapsed` | ks | method | Thread | -- | value |
| `VarRef.Empty` | ks | method | VarRef | -- | value |
| `EnvAdd` | v1 | command | -- | up to 2 | value |
| `EnvDiv` | v1 | command | -- | up to 2 | value |
| `EnvMult` | v1 | command | -- | up to 2 | value |
| `EnvSub` | v1 | command | -- | up to 2 | value |
| `EnvUpdate` | v1 | command | -- | up to 0 | value |
| `EnvUpdate` | ks | function | -- | -- | not measured |
| `ScriptProcess.ExitCode` | ks | method | ScriptProcess | -- | value |
| `Thread.ExitCode` | ks | method | Thread | -- | value |
| `ScriptProcess.ExitTime` | ks | method | ScriptProcess | -- | value |
| `ScriptProcess.HasExited` | ks | method | ScriptProcess | -- | value |
| `Thread.Id` | ks | method | Thread | -- | value |
| `Thread.IsActive` | ks | method | Thread | -- | value |
| `Thread.IsInterruptible` | ks | method | Thread | -- | value |
| `Thread.Kind` | ks | method | Thread | -- | value |
| `Language` | v2 | variable | -- | -- | value |
| `MyDocuments` | v2 | variable | -- | -- | value |
| `OSVersion` | v2 | variable | -- | -- | value |
| `OutputDebug` | v2 | command | -- | 1 | value |
| `OutputDebug` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Thread.Paused` | ks | method | Thread | -- | value |
| `Thread.Priority` | ks | method | Thread | -- | value |
| `Process` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ProcessClose` | v2 | command | -- | 1 | value |
| `ProcessExist` | v2 | command | -- | 0 to 1 | value |
| `ProcessGetName` | v2 | command | -- | 0 to 1 | value |
| `ProcessGetParent` | v2 | command | -- | 0 to 1 | value |
| `ProcessGetPath` | v2 | command | -- | 0 to 1 | value |
| `ProcessSetPriority` | v2 | command | -- | 1 to 2 | value |
| `ProcessWait` | v2 | command | -- | 1 to 2 | value |
| `ProcessWaitClose` | v2 | command | -- | 1 to 2 | value |
| `ProgramFiles` | v2 | variable | -- | -- | refused: this is a name of Windows, and this desktop is not Windows. |
| `ProgramFiles` | v1 | variable | -- | -- | refused: this is a name of Windows, and this desktop is not Windows. |
| `Programs` | v2 | variable | -- | -- | value |
| `ProgramsCommon` | v2 | variable | -- | -- | value |
| `Run` | v2 | command | -- | 1 to 4 | value |
| `Run` | v1 | command | -- | up to 1 | value |
| `RunAs` | v2 | command | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RunAs` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `RunWait` | v2 | function | -- | 1 to 4 | value |
| `RunWait` | v1 | command | -- | up to 1 | value |
| `ScriptProcess` | ks | class | -- | -- | value |
| `SetEnv` | v1 | command | -- | up to 1 | value |
| `Shutdown` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Shutdown` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `StartMenu` | v2 | variable | -- | -- | value |
| `StartMenuCommon` | v2 | variable | -- | -- | value |
| `Startup` | v2 | variable | -- | -- | value |
| `StartupCommon` | v2 | variable | -- | -- | value |
| `ScriptProcess.StdErr` | ks | method | ScriptProcess | -- | value |
| `ScriptProcess.StdIn` | ks | method | ScriptProcess | -- | value |
| `ScriptProcess.StdOut` | ks | method | ScriptProcess | -- | value |
| `SysGet` | v2 | command | -- | 1 | value |
| `SysGet` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `SysGetIPAddresses` | v2 | command | -- | 0 | value |
| `PropRef.Target` | ks | method | PropRef | -- | value |
| `Temp` | v2 | variable | -- | -- | value |
| `Thread.Underlying` | ks | method | Thread | -- | value |
| `URLDownloadToFile` | v1 | command | -- | up to 2 | value |
| `UserName` | v2 | variable | -- | -- | value |
