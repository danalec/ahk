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
; Measured on the binary of v2.0.19 on 2026-11-09: a semicolon inside a
; hotstring definition is text and not the start of a comment -- it is in the
; trigger and in the replacement -- and the definition is accepted.
:?:; btu::; but
Emit("HS4")
ExitApp