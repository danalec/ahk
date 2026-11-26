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
    s := "abc"
    Emit("R6=" s + 1)
} catch as e {
    Emit("R6=" Type(e) "|" e.Message "|" e.Extra)
}
