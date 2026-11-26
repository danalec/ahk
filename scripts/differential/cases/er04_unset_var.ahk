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
    u := UnsetThing
    Emit("R4=" u + 1)
} catch as e {
    Emit("R4=" Type(e) "|" e.Message "|" e.Extra)
}
