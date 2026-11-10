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
; Measured on the binary of v2.0.19 on 2026-11-09: the hotkey line may carry
; no body at all, and the block on the next line is the body.
F1::
{
    MsgBox "block"
}
Emit("HK3")
ExitApp