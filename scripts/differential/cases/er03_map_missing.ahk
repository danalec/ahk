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
    m := Map("a", 1)
    Emit("R3=" m["zz"])
} catch as e {
    Emit("R3=" Type(e) "|" e.Message "|" e.Extra)
}
