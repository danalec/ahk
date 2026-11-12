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
; Measured on the binary of v2.0.19 on 2026-11-09: the doubled colon inside the
; trigger is the literal `::` of the text that is typed, so `::1:::2` replaces
; the typed `1::` with `2`. The definition is accepted and the auto-execute
; section runs on past it.
::1:::2
Emit("HK7")
ExitApp