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
; Measured on the binary of the v2.0.32: the name of a hotkey is the text
; of the line, and the backtick of it is the escape of the character that
; follows it, and the quote of it is the key of the quote: the grave key and
; the apostrophe key are hotkeys with CapsLock.
CapsLock & `;::Emit("HK2=grave")
CapsLock & '::Emit("HK2=quote")
Emit("HK2")
ExitApp
