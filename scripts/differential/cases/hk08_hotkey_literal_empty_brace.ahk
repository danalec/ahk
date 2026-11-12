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
; Measured on the binary of v2.0.19 on 2026-11-09: the literal one-line
; `F1::{ }` is REFUSED -- "This line does not contain a recognized action.
; Specifically: { }" -- and the block has to open on the line and close on a
; line of its own. The case keeps that refusal in the differential so the
; harness sees the shape the binary does not have.
F1::{ }
Emit("HK8")
ExitApp