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
    f := 5
    Emit("R13=" f())
} catch as e {
    Emit("R13=" Type(e) "|" e.Message "|" e.Extra)
}
