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
    Emit("V2=" LocalTest() "|" LocalTest())
} catch as e {
    Emit("V2=ERR " Type(e) ": " e.Message)
}
LocalTest() {
    L := "local"
    return L
}
