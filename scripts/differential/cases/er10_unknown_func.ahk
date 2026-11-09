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
    Emit("R10=" NoSuchFunc())
} catch as e {
    Emit("R10=" Type(e) "|" e.Message "|" e.Extra)
}
