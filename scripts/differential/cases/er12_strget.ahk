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
try {
    Emit("R12=" StrGet(0))
} catch as e {
    Emit("R12=" Type(e) "|" e.Message "|" e.Extra)
}
