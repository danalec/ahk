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
; Measured on the binary of v2.0.19 on 2026-11-09: the double quote is a
; character of the trigger and of the replacement, not the start of a string,
; so the definition is accepted and the auto-execute section runs on past it.
:*:i"q::I "quote"
Emit("HS3")
ExitApp