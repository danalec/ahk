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
g := "init"
try {
    Emit("V1=" SetG() "|" g)
} catch as e {
    Emit("V1=ERR " Type(e) ": " e.Message)
}
SetG() {
    global g
    g := "changed"
    return g
}
