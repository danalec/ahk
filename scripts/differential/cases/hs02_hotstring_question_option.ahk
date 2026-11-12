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
; Measured on the binary of v2.0.19 on 2026-11-09: `?` is an option of the
; hotstring -- the trigger may fire inside a word -- and the definition is
; accepted with the auto-execute section running on past it.
:?:btw::by the way
Emit("HS2")
ExitApp