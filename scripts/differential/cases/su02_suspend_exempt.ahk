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
; Measured on the binary of v2.0.19 on 2026-11-09: `#SuspendExempt` is a
; directive with no arguments, and the hotkeys that follow it are exempt from
; `Suspend`; the auto-execute section still runs on past the definition.
#SuspendExempt
F1::MsgBox "hi"
Emit("exempt=" A_IsSuspended)
Suspend 1
Emit("suspended=" A_IsSuspended)
ExitApp