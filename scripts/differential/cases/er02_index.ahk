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
    a := [1, 2]
    Emit("R2=" a[5])
} catch as e {
    Emit("R2=" Type(e) "|" e.Message "|" e.Extra)
}
