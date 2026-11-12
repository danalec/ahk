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
    o := {a: 1}
    Emit("R7=" o.b.c)
} catch as e {
    Emit("R7=" Type(e) "|" e.Message "|" e.Extra)
}
