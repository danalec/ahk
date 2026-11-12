# 解析器与脚本执行

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

语言: 中文（简体）

三个方言中属于 **解析器与脚本执行** 类的名称，以及这棵树对每个名称给出的回答。分类是脚本思考的领域，这些名称即是它的全部。

| 名称 | 方言 | 种类 | 类 | 参数 | 状态 | 调用探测 |
|---|---|---|---|---|---|---|
| `#CommentFlag` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#ErrorStdOut` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#EscapeChar` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#HotkeyInterval` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#HotkeyModifierTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#Hotstring` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#If` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#IfTimeout` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#IfWinActive` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#Include` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#InputLevel` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#InstallKeybdHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#InstallMouseHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#KeyHistory` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MaxHotkeysPerInterval` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MaxMem` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MaxThreads` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MaxThreadsBuffer` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MaxThreadsPerHotkey` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#MenuMaskKey` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#NoEnv` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#NoTrayIcon` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#Persistent` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#Requires` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#SingleInstance` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#UseHook` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#Warn` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `#WinActivateForce` | AutoHotkey v1.1.37.02 | directive | -- | -- | 值 | -- |
| `A_AhkPath` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_AhkVersion` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_AutoTrim` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_BatchLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_DefaultGui` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_DefaultListView` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_DefaultTreeView` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_EndChar` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ExitReason` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Index` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_InitialWorkingDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Is64bitOS` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsAdmin` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsCompiled` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsCritical` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsPaused` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsSuspended` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_IsUnicode` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_KeybdHookInstalled` | KeySharp | variable | -- | -- | 值 | -- |
| `A_KsCorePath` | KeySharp | variable | -- | -- | 值 | -- |
| `A_KsVersion` | KeySharp | variable | -- | -- | 值 | -- |
| `A_LastError` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_LineFile` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_LineNumber` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ListLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_MaxThreads` | KeySharp | variable | -- | -- | 值 | -- |
| `A_MaxThreadsBuffer` | KeySharp | variable | -- | -- | 值 | -- |
| `A_MaxThreadsPerHotkey` | KeySharp | variable | -- | -- | 值 | -- |
| `A_MouseHookInstalled` | KeySharp | variable | -- | -- | 值 | -- |
| `A_NumBatchLines` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_OSType` | AutoHotkey v1.1.37.02 | variable | -- | -- | 拒绝：this name is about Windows, and this machine is not. | refusal: this name is about Windows, and this machine is not. |
| `A_OSVersion` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Persistent` | KeySharp | variable | -- | -- | 值 | -- |
| `A_PtrSize` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_RealThread` | KeySharp | variable | -- | -- | 值 | -- |
| `A_ScriptDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ScriptFullPath` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ScriptHwnd` | AutoHotkey v1.1.37.02 | variable | -- | -- | 拒绝：this runtime has no window of its own: it is a script that runs and ends. | refusal: this runtime has no window of its own: it is a script that runs and ends. |
| `A_ScriptName` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Space` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Tab` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ThisFunc` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ThisLabel` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ThisMenu` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ThisMenuItem` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_ThisMenuItemPos` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `A_Timers` | KeySharp | variable | -- | -- | 值 | -- |
| `A_WorkingDir` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `AllowMainWindow` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Collect` | KeySharp | function | -- | -- | 值 | -- |
| `CompileScript` | KeySharp | function | -- | -- | 值 | -- |
| `CoordMode` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `CoordMode` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `Critical` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 拒绝：this runtime runs one script in one thread, and it cannot make it critical. | refusal: this runtime runs one script in one thread, and it cannot make it critical. |
| `Critical` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this runtime runs one script in one thread, and it cannot make it critical. | refusal: this runtime runs one script in one thread, and it cannot make it critical. |
| `Thread.Critical` | KeySharp | method | Thread | -- | 值 | -- |
| `Else` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `EndChar` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `EnvGet` | AutoHotkey v2.0.19 | command | -- | 1 | 值 | -- |
| `EnvGet` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | 值 | -- |
| `EnvSet` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `EnvSet` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `ErrorLevel` | AutoHotkey v1.1.37.02 | variable | -- | -- | 值 | -- |
| `Exit` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 值 | -- |
| `Exit` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `Thread.Exit` | KeySharp | method | Thread | -- | 值 | -- |
| `ExitApp` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 值 | -- |
| `ExitApp` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `IfInString` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | 值 | -- |
| `IfMsgBox` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `IfNotInString` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | 值 | -- |
| `Index` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Thread.Index` | KeySharp | method | Thread | -- | 值 | -- |
| `InitialWorkingDir` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Is64bitOS` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `IsAdmin` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `IsCompiled` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `IsComponentAvailable` | KeySharp | function | -- | -- | 值 | -- |
| `IsCritical` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `IsFunc` | AutoHotkey v1.1.37.02 | function | -- | 1 | 值 | -- |
| `IsPaused` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `IsSet` | AutoHotkey v1.1.37.02 | function | -- | 1 | 值 | -- |
| `IsSuspended` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `LineFile` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `LineNumber` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `OnError` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `OnError` | AutoHotkey v1.1.37.02 | function | -- | 1 to 2 | 值 | -- |
| `OnExit` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | 值 | -- |
| `OnMessage` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | 值 | -- |
| `OnMessage` | AutoHotkey v1.1.37.02 | function | -- | 1 to 3 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `OutputDebugLine` | KeySharp | function | -- | -- | 值 | -- |
| `Pause` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 值 | -- |
| `Pause` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `Persistent` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 值 | -- |
| `Reload` | AutoHotkey v2.0.19 | command | -- | 0 | 拒绝：this runtime runs one script once, and it cannot run itself again. | refusal: this runtime runs one script once, and it cannot run itself again. |
| `Reload` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `RequestCapabilities` | KeySharp | function | -- | -- | 值 | -- |
| `RunScript` | KeySharp | function | -- | -- | 值 | -- |
| `ScriptDir` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `ScriptFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `ScriptName` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `SetBatchLines` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `SetTimer` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | 值 | -- |
| `SetTimer` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 值 | -- |
| `SetWorkingDir` | AutoHotkey v2.0.19 | command | -- | 1 | 值 | -- |
| `SetWorkingDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `ShowDebug` | KeySharp | function | -- | -- | 值 | -- |
| `Sleep` | AutoHotkey v2.0.19 | command | -- | 1 | 值 | -- |
| `Sleep` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 值 | -- |
| `Space` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Suspend` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | 值 | -- |
| `Suspend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | 拒绝：this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `Tab` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `ThisFunc` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |
| `Thread` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | 拒绝：this runtime runs one script in one thread, and it cannot start another. | refusal: this runtime runs one script in one thread, and it cannot start another. |
| `Thread` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | 拒绝：this runtime runs one script in one thread, and it cannot start another. | refusal: this runtime runs one script in one thread, and it cannot start another. |
| `ValidateScript` | KeySharp | function | -- | -- | 值 | -- |
| `WorkingDir` | AutoHotkey v2.0.19 | variable | -- | -- | 值 | -- |

## 这棵树对它做什么

### A_Index

正在运行的循环的轮次编号。第一个循环之前它是零；每个循环都保存它原有的值，并在结束时还回去。

- **参数:** nothing; it is a variable, not a call
- **返回:** 轮次编号，作为数字
- **错误:** nothing to read; a script may write the name, and the loop goes on from the value that was written.

```ahk
for item in [1, 2, 3]
    FileAppend A_Index, "*"
```

### A_Is64bitOS

机器是否是 64 位。这个树用它自身指针的宽度回答：运行时的 64 位构建是 1，32 位构建是 0。

- **参数:** nothing; it is a variable, not a call
- **返回:** 1 或 0
- **错误:** nothing to read; the write is refused.

```ahk
FileAppend A_Is64bitOS, "*"
```

### A_IsAdmin

脚本是否以管理员权限运行：有效用户 ID，root 就是值为零的那个。

- **参数:** nothing; it is a variable, not a call
- **返回:** 提权时为 1，未提权时为 0
- **错误:** the read fails when the machine cannot say; the write is refused.

```ahk
FileAppend A_IsAdmin, "*"
```

### A_OSVersion

机器操作系统的版本：内核的发行版，从 `/proc/sys/kernel/osrelease` 读出。

- **参数:** nothing; it is a variable, not a call
- **返回:** 版本，作为文本
- **错误:** the read fails when the release of the kernel cannot be read; the write is refused.

```ahk
FileAppend A_OSVersion, "*"
```

### A_ScriptDir

运行开始的目录。运行器的脚本来自标准输入，本身没有文件，所以脚本的目录就是运行的目录。

- **参数:** nothing; it is a variable, not a call
- **返回:** 目录的路径，作为文本
- **错误:** the read fails when the runtime does not keep the directory where the run started; the write is refused.

```ahk
FileAppend A_ScriptDir, "*"
```

### A_ScriptFullPath

脚本的路径。运行器的脚本来自标准输入，没有文件，所以路径是一个星号。

- **参数:** nothing; it is a variable, not a call
- **返回:** 文本 "*"
- **错误:** the write is refused.

```ahk
FileAppend A_ScriptFullPath, "*"
```

### A_ScriptName

脚本的名称。运行器的脚本来自标准输入，没有文件，所以名称是一个星号。脚本可以写这个名称，写入的值就是它保留的值。

- **参数:** nothing; it is a variable, not a call
- **返回:** 文本 "*"，或写入的文本
- **错误:** nothing to read; the write takes any value.

```ahk
FileAppend A_ScriptName, "*"
```

### A_ThisFunc

正在运行的函数的名称。

- **参数:** nothing; it is a variable, not a call
- **返回:** 名称，作为文本；没有函数运行时是空文本
- **错误:** nothing to read; the write is refused.

```ahk
FileAppend A_ThisFunc, "*"
```

### CoordMode

说明坐标如何解读。v2 有五个目标：鼠标、像素、插入符、菜单和提示；本树没有窗口，插入符、菜单和提示的模式照样保存，但没有人使用它们。

- **参数:** Target [, Relative]
- **返回:** 之前的模式
- **错误:** a target or a relative that v2 does not know is refused as an invalid parameter.

```ahk
CoordMode "Mouse", "Screen"
```

### CoordMode

说明坐标如何解读。v2 有五个目标：鼠标、像素、插入符、菜单和提示；本树没有窗口，插入符、菜单和提示的模式照样保存，但没有人使用它们。

- **参数:** Target [, Relative]
- **返回:** 之前的模式
- **错误:** a target or a relative that v2 does not know is refused as an invalid parameter.

```ahk
CoordMode "Mouse", "Screen"
```

### Critical

这棵树拒绝这个名字：这个运行时只用一个线程运行一个脚本，无法把它设为关键状态。

- **参数:** [OnOff]
- **返回:** 无返回值；调用失败
- **错误:** the refusal is "this runtime runs one script in one thread, and it cannot make it critical."

```ahk
Critical
```

### Critical

这棵树拒绝这个名字：这个运行时只用一个线程运行一个脚本，无法把它设为关键状态。

- **参数:** [OnOff]
- **返回:** 无返回值；调用失败
- **错误:** the refusal is "this runtime runs one script in one thread, and it cannot make it critical."

```ahk
Critical
```

### Else

if 的另一分支。else if 可以串联多个条件，第一个匹配的条件会执行。

- **参数:** a statement or a block
- **返回:** 无返回值
- **错误:** none.

```ahk
if (0)
    MsgBox "no"
else
    MsgBox "yes"
```

### EnvGet

给出运行器环境变量的值；环境就是脚本所在进程的环境。

- **参数:** VariableName
- **返回:** 变量的值，环境里没有时是空文本
- **错误:** nothing; a name that is not there is the empty text and not a failure.

```ahk
home := EnvGet("HOME")
```

### EnvGet

给出运行器环境变量的值；环境就是脚本所在进程的环境。

- **参数:** VariableName
- **返回:** 变量的值，环境里没有时是空文本
- **错误:** nothing; a name that is not there is the empty text and not a failure.

```ahk
home := EnvGet("HOME")
```

### EnvSet

设置运行器的一个环境变量并返回该值；脚本只有一个线程，改动时没有别的代码在读环境。

- **参数:** VariableName, Value
- **返回:** 被设置的值（文本）
- **错误:** an empty name and a name that carries an equal sign are refused with the reason of the machine.

```ahk
EnvSet "MY_VAR", "value"
```

### EnvSet

设置运行器的一个环境变量并返回该值；脚本只有一个线程，改动时没有别的代码在读环境。

- **参数:** VariableName, Value
- **返回:** 被设置的值（文本）
- **错误:** an empty name and a name that carries an equal sign are refused with the reason of the machine.

```ahk
EnvSet "MY_VAR", "value"
```

### Exit

停止脚本。退出码被保留，脚本在调用它的那条语句结束时停止。

- **参数:** [Code]
- **返回:** 退出码
- **错误:** none.

```ahk
Exit 0
```

### Exit

停止脚本。退出码被保留，脚本在调用它的那条语句结束时停止。

- **参数:** [Code]
- **返回:** 退出码
- **错误:** none.

```ahk
Exit 0
```

### ExitApp

以一个退出码结束脚本。该退出码进入报告的 exit_code。

- **参数:** [Code]
- **返回:** 退出码
- **错误:** none.

```ahk
ExitApp 3
```

### ExitApp

以一个退出码结束脚本。该退出码进入报告的 exit_code。

- **参数:** [Code]
- **返回:** 退出码
- **错误:** none.

```ahk
ExitApp 3
```

### IsSet

调用者的变量是否有值。这个树除 `IsSetRef` 之外也带着它，它要的是变量，而不是变量的值。

- **参数:** the variable, not its value
- **返回:** 变量有值时为 1，没有时为 0
- **错误:** an argument that is not a variable is refused with "IsSet requires a variable.".

```ahk
x := 1
FileAppend IsSet(x), "*"
```

### OnError

注册处理未被捕获的失败的函数。

- **参数:** Callback, [Order]
- **返回:** 整数 1
- **错误:** a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

```ahk
OnError((err, mode) => 0)
```

### OnError

注册处理未被捕获的失败的函数。

- **参数:** Callback, [Order]
- **返回:** 整数 1
- **错误:** a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

```ahk
OnError((err, mode) => 0)
```

### OnExit

注册脚本结束时运行的函数，无论结束的原因是什么：最后一条语句、脚本请求停止的退出码，或让脚本停下的失败。

- **参数:** Callback, [Order]
- **返回:** 整数 1
- **错误:** a callback that is not a function object, or a prototype that does not take two parameters, is refused with "Invalid callback function.".

```ahk
OnExit((reason, code) => 0)
```

### Pause

设置脚本的暂停状态。这棵树只记录状态，而不是等待按键——没有键盘的脚本会因此挂住。状态可在 A_IsPaused 中看到。

- **参数:** [OnOff]
- **返回:** 空文本
- **错误:** a parameter that is not 0, 1 or -1 is an invalid parameter.

```ahk
Pause true
```

### Pause

设置脚本的暂停状态。这棵树只记录状态，而不是等待按键——没有键盘的脚本会因此挂住。状态可在 A_IsPaused 中看到。

- **参数:** [OnOff]
- **返回:** 空文本
- **错误:** a parameter that is not 0, 1 or -1 is an invalid parameter.

```ahk
Pause true
```

### Persistent

返回 0，并且不改变任何东西。这个运行时的脚本从第一行跑到最后一行，没有消息循环可以让它存活，因此这个设置在这里没有意义。

- **参数:** [OnOff]
- **返回:** 整数 0
- **错误:** none.

```ahk
Persistent
```

### Reload

这棵树拒绝这个名字：这个运行时只运行一次脚本，无法再次运行自己。

- **参数:** none
- **返回:** 无返回值；调用失败
- **错误:** the refusal is "this runtime runs one script once, and it cannot run itself again."

```ahk
Reload
```

### Reload

这棵树拒绝这个名字：这个运行时只运行一次脚本，无法再次运行自己。

- **参数:** none
- **返回:** 无返回值；调用失败
- **错误:** the refusal is "this runtime runs one script once, and it cannot run itself again."

```ahk
Reload
```

### SetTimer

注册一个在语句之间运行的脚本函数。这个运行时只有一个线程，因此定时器不是独立的线程：它在语句之间运行，与脚本共用预算。

- **参数:** Name, [Period]
- **返回:** 整数 1
- **错误:** more than two parameters are refused with "the priority of the timer is not carried yet."

```ahk
SetTimer("Tick", 1000)
```

### SetTimer

注册一个在语句之间运行的脚本函数。这个运行时只有一个线程，因此定时器不是独立的线程：它在语句之间运行，与脚本共用预算。

- **参数:** Name, [Period]
- **返回:** 整数 1
- **错误:** more than two parameters are refused with "the priority of the timer is not carried yet."

```ahk
SetTimer("Tick", 1000)
```

### SetWorkingDir

设置脚本的工作目录。不是绝对路径的名字会被视为工作目录下的子目录。

- **参数:** Directory
- **返回:** 整数 1
- **错误:** an empty name is refused with "the name of the directory is missing.", and a folder that cannot be entered fails with the reason of the desktop.

```ahk
SetWorkingDir "."
```

### SetWorkingDir

设置脚本的工作目录。不是绝对路径的名字会被视为工作目录下的子目录。

- **参数:** Directory
- **返回:** 整数 1
- **错误:** an empty name is refused with "the name of the directory is missing.", and a folder that cannot be entered fails with the reason of the desktop.

```ahk
SetWorkingDir "."
```

### Sleep

让脚本暂停指定的毫秒数。

- **参数:** Milliseconds
- **返回:** 等待的毫秒数
- **错误:** none; a wait longer than 60000 milliseconds is cut at that number.

```ahk
Sleep 500
```

### Sleep

让脚本暂停指定的毫秒数。

- **参数:** Milliseconds
- **返回:** 等待的毫秒数
- **错误:** none; a wait longer than 60000 milliseconds is cut at that number.

```ahk
Sleep 500
```

### Suspend

关闭或打开脚本的热键。不带参数时切换状态。状态可在 A_IsSuspended 和热键表的启用标志中看到。

- **参数:** [OnOff]
- **返回:** 空文本
- **错误:** a parameter that is not 0, 1 or -1 is an invalid parameter.

```ahk
Suspend true
```

### Suspend

关闭或打开脚本的热键。不带参数时切换状态。状态可在 A_IsSuspended 和热键表的启用标志中看到。

- **参数:** [OnOff]
- **返回:** 空文本
- **错误:** a parameter that is not 0, 1 or -1 is an invalid parameter.

```ahk
Suspend true
```
