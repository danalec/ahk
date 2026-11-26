# The hook of the messages of a script of the v1

A script of the v1 installs a hook of the messages of the windows of
its thread: a `WH_CALLWNDPROC` (`SetWindowsHookEx` with the type 4) whose
procedure is a function of the script. The hook is called for **every**
message of **every** window of the thread, and the function of the script
answers `CallNextHookEx`.

This is what the v1 does, what the tree did, and what the tree does now.
Every number here is a measurement, and the probe of it is named.

## The shape of the parameters

The machine calls the hook of the `WH_CALLWNDPROC` with three numbers:
`(nCode, wParam, lParam)`. The first is zero (`HC_ACTION`), the second is
the `wParam` of the message, and the third is the pointer of the
`CWPSTRUCT` -- the `NumGet(lP+0, 2*A_PtrSize, "UInt")` of a script of
the v1 reads the message out of it, and the `NumGet(lP+0, 3*A_PtrSize,
"UPtr")` reads the `hwnd` out of it.

- Measured on the binary of AutoHotkey v1.1.37.02, with
  the probes the probe of the measurement (three
  parameters), `hook3.ahk` (four) and `hook5.ahk` (one and two): the
  function of the script receives the registers of the call, in the order
  of the call.
- The low level mouse hook (the type 14) has the same shape: the probe
  `hook6.ahk` measured `nCode=0`, `wP=512` (`WM_MOUSEMOVE`) and the
  pointer of the `MSLLHOOKSTRUCT` in the third parameter.
- The binary of the v1 says the same. The `RegisterCallback` of the v1
  writes a stub of sixteen bytes into a block that `GlobalAlloc` gave it
  (`sub_140081940` of the decompilation in `C:\tmp\ahk-decomp`):
  `48 8d 05 f9 ff ff ff` (`lea rax, [rip-7]`) and `ff 25 03 00 00 00`
  (`jmp qword ptr [rip+3]`). The jump goes to `sub_1400d6740`, which
  copies the four registers of the call (`RCX`, `RDX`, `R8`, `R9`) into a
  frame and calls the handler (`sub_140080f00`). The handler copies the
  i-th register into the i-th parameter of the function of the script
  (`MOV RDX, [R12 + RDI*8]` at `0x1400810b7`).
- The tree does the same: the trampoline of the pool of the callbacks
  takes the four registers of the call and gives them to the function of
  the script in the order of the call
  (`crates/ahk/src/callback.rs`).

## The callback of the script runs while the machine pumps the messages

The messages of the thread are pumped by the runtime of the tree, and not
by a native call of the script. The runtime of the tree was kept for the
thread only for the time of a native call (`crates/ahk/src/callback.rs`,
`set_current`), and the callback of the machine found no runtime when the
hook of the script was called: **the callback of the script did not run
at all**.

- Measured on the binary of AutoHotkey v1.1.37.02, with
  the probe the probe of the measurement: the
  callback of the script ran **19** times during the half second of the
  sleep of the script (the machine pumped the messages of the thread), and
  **20** times after the message that the script sent to its own window.
- The tree before the change: **0** times during the sleep, and **1** time
  after the message (the message went through the native call of the
  script, and the runtime was kept for it).
- The tree after the change: the callback runs for the messages of the
  machine, the way the v1 does it. The number of the callbacks depends on
  the messages the machine sends (19 to 46 in the same probe).

## The answer of the callback comes back to the machine

The `CallNextHookEx` of a script of the v1 is the answer of the
function of the script, and the machine must get it. The trampoline of the
tree answers the number the function of the script answered.

- Measured on the binary of AutoHotkey v1.1.37.02 with
  the probe the probe of the measurement: the
  `DllCall` of the address of the callback gives the answer of the
  function of the script to the caller.
- The case is in `crates/ahk/tests/v1_hook.rs`: the function answers 42,
  and the machine gets 42.

## The callback does not re-enter the runtime that is running it
The callback of the machine runs the runtime again, and the run of the
callback is inside the run that called it: the runtime is already pumping
the messages of the machine, and the pump of the callback is the same
pump. The counter of the callbacks that run (`callbacks_running`) is what
the pump of the events looks at.

The counter did not grow for the callback of a **name** (the
`RegisterCallback` of the v1) before: only the callback of a value (the
`CallbackCreate`) grew it. The hook of a script of the v1 is a
`RegisterCallback`, so the pump of the tree did not see the callback of
the hook.

- Measured on a script of the v1: with the trace of
  the callbacks (`AHK_TRACE_CALLBACK=1`), the 365 callbacks of the run
  were all at the depth 0 -- **the callback never ran inside another
  callback**.
- The trace of the run (`AHK_TRACE=1`) does not repeat the body of the
  hook (`37846 If`, `37848 Assign`, `37849 If`, `37852 Return`): the body
  ran 390 times, and never twice in a row.

## The modal box of the machine

The message box of the machine is a modal loop, and the loop pumps the
messages of the thread: the hook of the script is called for every one
of them, and the run of the callback does not pump the messages again.

- Measured on the binary of AutoHotkey v1.1.37.02, with
  the probe the probe of the measurement: the
  hook of the script ran 57 times before the box, and it kept running
  while the box waited.
- The tree after the change: 46 times before the box, and the same while
  the box waited. The tree before the change ran the hook only for the
  messages that a native call of the script sent.

## The run of a script of the v1

- Before the change: the body of the hook ran **56** times -- the
  messages that a native call of the script sent -- and the menu of the
  a script of the v1 never saw the messages of the machine.
- After the change: the body of the hook ran **390** times, and the run
  of a script of the v1 ended with the code 0.

The run of a script of the v1 is under `timeout 30`, with
`AHK_TRACE=1`, from `C:the script of the v1
The numbers are the ones of the tree with the change of this branch and
with the scope of the variables of the v1 (the `master` of the swarm):
without the scope of the variables, a script of the v1 stops at the
message box of error and never reaches the hook.
