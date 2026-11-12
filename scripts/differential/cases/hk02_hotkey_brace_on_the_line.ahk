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
; Measured on the binary of v2.0.19 on 2026-11-09: the body may be the block
; whose opening brace is on the line of the hotkey; the auto-execute section
; runs on past the block.
F1::{
}
Emit("HK2")
ExitApp