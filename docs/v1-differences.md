# The script of the v1 on the tree — the differences that were measured

The script of the v1 that exercised the tree the most is an AHK v1.1
script of 108 thousand lines.
It is the script that exercises the tree the most, and every difference
that the tree had with the v1 showed up in it first.

This is the list of the differences that were measured on the binary of the
v1.1.37.02 and on the tree, with the probe of each one. The menu has its
own record: [`v1-menu.md`](v1-menu.md).

## 1. The timers of the script run while it waits

The v1: the timers of the script run while it waits, and the wait goes on
for its whole time.

**Measured on the binary of the v1.1.37.02**: the timer of
a hundred milliseconds runs **five** times during a `Sleep, 600`, and the
one-shot timer of a negative period runs **once**.

The tree slept with the clock of the runtime and the timers of the script
did not run at all: the script has a timer of five milliseconds that
drives its mouse and its panels, and it never ran.

**=> The wait of the script pumps the machine**: the timers that are due,
the callbacks that were posted, the messages of the windows of the script
and the hotkeys that fired (`the_run_waits_for` of the runtime).

## 2. The catch of the v1 has no class

The v1: `catch e` (without a comma) catches everything that was thrown,
and the `e` holds the exception.

**Measured on the binary of the v1.1.37.02**: the
`catch e` of the v1 catches the error that was thrown, and the script
goes on.

The tree read the name after the `catch` as the name of a class, and it
refused it with "the class of a catch is not a class of this runtime: e".
The script catches the errors of its own panels with `catch weh` and reads
`weh.message` of them.

**=> In the v1, the name that follows the `catch` is the variable that
holds the exception** (the parser of the v1 ✓).

## 3. The native call takes the value of a built-in variable

The v1: `DllCall("user32\GetWindowThreadProcessId", "Ptr", A_ScriptHwnd,
"Ptr", 0)` answers the thread of the window of the script.

**Measured on the binary of the v1.1.37.02**: the answer is
the thread of the window, and the tree answered zero.

The tree read the built-in variables as the variables of the script, and a
built-in variable is not one: the call took the empty text, and the empty
text is the zero.

The script asks for the thread of its window this way, and the hook of the
taskbar interface (`SetWinEventHook`) failed with the error 1427
because of it.

**=> The built-in variables are read from the table of them, and the value
of them is what goes to the machine** (the `marshal` of the builtins ✓).

## 4. The callback of a name that is not there is the empty text

The v1: `RegisterCallback("")` and `RegisterCallback("NoSuchName")` answer
the empty text, and the script goes on.

**Measured on the binary of the v1.1.37.02**: both answer
the empty text.

**=> The tree answers the empty text as well, and it does not take a place
in the pool of the callbacks** ✓.

## 5. The callback of a method is the method

The v1: `RegisterCallback(this.MyMethod)` answers the address of the
callback of the method, and the same is true when the `this` is the value
of the class -- the `this` of a static method.

**Measured on the binary of the v1.1.37.02**: the
`RegisterCallback(this.MyMethod)` of the v1 answers the address of the
callback of the method, and the tree answered the empty text.

The script asks for the callback of the method of its taskbar interface this
way, and the hook of the taskbar interface was never installed.

**=> The tree takes the method itself, and the value of a class hands over
the methods of the class in the v1** (the flag
`the_class_value_hands_over_its_methods` ✓).

## 6. The window of the script carries the icon of the program

The v1: the window of the script carries the icon of the program that runs
it -- the small one in the title bar and the big one in the list of the
windows. The icon of the tray is another thing: it does not change the
icon of the window.

**Measured on the binary of the v1.1.37.02**: the window
of the script of the v1 has an icon, and the `Menu, Tray, Icon` does not
change it.

The tree gave no icon at all to the windows of the v1.

**=> The tree gives the icon of the program to the windows of the v1** ✓.

## 7. The loop of the registry of a key that is not there

The v1: the `Loop, Reg` of a key that cannot be opened is a loop with no
turn. The script goes on, and nothing is said about the key.

**Measured on the binary of the v1.1.37.02** (`source/script.cpp`, the
`PerformLoopReg`): the `RegOpenKeyEx` that does not answer
`ERROR_SUCCESS` makes the v1 return `OK` -- the loop does not run and the
script goes on. The same is true of the `RegQueryInfoKey` that follows,
and of the items that the enumeration cannot read: the v1 skips them and
goes on.

The tree refused, and the refusal ended the run: a script that enumerates
the registry of the class of a COM object
(`Loop, Reg, HKEY_CLASSES_ROOT\CLSID\%A_loopField%, KV`) dies with it when
the class is not registered on the machine -- and the `OnExit` of the
script closes the process with it.

**=> The tree gives the loop of a key that cannot be opened no turn, and
the script goes on** (the flag
`a_key_of_the_registry_that_cannot_be_opened_is_not_a_failure` ✓). The v2
keeps the refusal: its `Loop Reg` of a key that is not there throws the
failure of the machine.

## 8. The native call that takes a variable with content

The v1: a variable that goes to the machine as a "Str" or as a "WStr" is the
buffer of the variable, and the call is not refused when the variable has
content of its own.

**Measured on the binary of the v1.1.37.02**: the
`DllCall("kernel32\lstrlenA", "Str", s)` of a script whose `s` holds
"hello" answers **one** -- the length of the ANSI text of the buffer of the
variable -- and the call is not refused. The same call as a "WStr" answers
**five**, the length of the wide text of the same buffer.

The tree refused the call with "This DllCall requires a prior VarSetCapacity."
and the script ended with it. The script of the app asks the machine for the
GUID of the GDI+ effect it draws its welcome screen with --
`DllCall("ole32\CLSIDFromString", "WStr", String, "UPtr", &CLSID)` -- and the
tree died there, before the first window of the app was drawn.

**=> The tree refuses the call only when the variable has no content and no
buffer of its own** (`the_call_that_needs_a_capacity` of the v1 ✓).

## 9. The window that is destroyed with the thread

The tree: the windows of the thread are destroyed when the thread ends, and
the procedure of each of them asks the runtime for the loan of the call that
goes on. The thread-local of the loans may be gone by then.

**Measured on the tree**: the run of the script of the app ended with the
panic "cannot access a Thread Local Storage value during or after
destruction", and the process was killed with it (the code 0xC0000409). The
backtrace shows the destruction of the window in the end of the thread
(`RtlShutdownThread`, `NtUserDestroyWindow`) and the procedure of it asking
for the loan.

**=> The procedure of a window asks with `try_with`, and the absence of the
loan is the answer**: the message goes to the procedure of the machine, and
the thread ends (`the_slot` of the callback ✓).

## 10. The base of an instance is the class of it

The v1: the `Base` of an object is the class of it, and the statics of the
classes of the chain are statics of the class.

**Measured on the binary of the v1.1.37.02**: the `This.Base.X` of a method
of a class that extends another is the `Static X` of the class that it
extends, and the `This.X` of it is the same. The `Static Y` of the class
itself is read the same way, and the `o.Base.X` of an object outside a
method is the static of the class of o.

The tree read the base of an instance as the prototype of the class of it --
the base of the v2 -- and the statics live in the class value of the class,
not in the prototype. The `This.Base.MaxOptions` and the `This.MaxOptions` of
a method of the v1 were both empty in the tree, and the class of the database
of the app (`Class_SQLiteDB.ahk`) counts its instances with them.

**=> The base of an object of the v1 is the class of it, and the statics of
the classes of the chain are statics of the class**
(`the_class_of_an_instance` of the runtime and the flag
`the_base_of_an_object_is_the_class_of_it` ✓).

## The probes

The probes are in the probe of the measurement:

| The probe | What it measures |
|---|---|
| `probe-timer.ahk` | the timers of a script that waits |
| `probe-catch.ahk` | the catch of the v1 and the exception it holds |
| `probe-hwnd2.ahk` | the native call of a built-in variable |
| `probe-rcb.ahk` | the callback of a name that is not there |
| `probe-rcb2.ahk` | the callback of a method |
| `probe-tbi2.ahk` | the taskbar interface of the script |
| `probe-icon.ahk` | the icon of the windows of the script |
| `probe-reg.ahk` | the loop of the registry of a key that is not there |
| `probe-dllstr.ahk` | the native call that takes a variable with content |
| `probe-base.ahk` | the base of an instance and the statics of the chain |

Every one of them runs under the binary of the v1.1.37.02 and under the
tree, and the answers of the two are the same.

## The trace of the run

The tree and the v1 that is built for the trace write the same four
kinds of events, and the two traces are compared with a diff. The run of
the script under the tree and under the v1 is the same: the timers that fire
are the same, the native calls are the same, and the messages of the
windows are the same.
