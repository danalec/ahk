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
; Measured on the binary of v2.0.19 on 2026-11-09: a hotkey whose body is one
; line is `F1::MsgBox "hi"`; the auto-execute section runs on past it, and the
; action itself never runs in this harness (no key is pressed).
F1::MsgBox "hi"
Emit("HK1")
ExitApp