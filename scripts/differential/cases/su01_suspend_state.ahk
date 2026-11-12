#ErrorStdOut
#Warn All, StdOut
OnError(__backstop)
__backstop(e, mode) {
    FileAppend("UNCAUGHT|" Type(e) "|" e.Message "`n", "*")
    return true
}
Emit(s) {
    FileAppend(s "`n", "*")
}
; Measured on the binary of v2.0.19 on 2026-11-09: `A_IsSuspended` is 0 before
; `Suspend 1`, 1 after it, and 0 again after `Suspend 0`.
Emit("before=" A_IsSuspended)
Suspend 1
Emit("after1=" A_IsSuspended)
Suspend 0
Emit("after0=" A_IsSuspended)
ExitApp