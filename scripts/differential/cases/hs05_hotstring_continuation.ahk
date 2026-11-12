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
; Measured on the binary of v2.0.19 on 2026-11-09: the replacement may be a
; multi-line continuation -- `::text::` and a `( … )` block -- and the
; definition is accepted with the auto-execute section running on past it.
::text1::
(
Line1
Line2
Line3
)
Emit("HS5")
ExitApp