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
    Emit("V3=" Cnt() "|" Cnt() "|" Cnt())
} catch as e {
    Emit("V3=ERR " Type(e) ": " e.Message)
}
Cnt() {
    static n := 0
    n += 1
    return n
}
