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
; Measured on the binary of v2.0.19 on 2026-11-09: `return` is a body of its
; own; the auto-execute section runs on past it.
F1::return
Emit("HK4")
ExitApp