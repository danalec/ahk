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
    x := 1 / 0
    Emit("R1=" x)
} catch as e {
    Emit("R1=" Type(e) "|" e.Message "|" e.Extra)
}
