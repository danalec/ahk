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
; Measured on the binary of v2.0.19 on 2026-11-09: a chord of the modifiers is
; a hotkey too, and its block body is the same shape as the plain one.
^!a::
{
    MsgBox "chord"
}
Emit("HK5")
ExitApp