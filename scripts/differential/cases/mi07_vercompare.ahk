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
    Emit("M14=" VerCompare("2.0.19", "2.0.10") "|" VerCompare("2.0", "2.0.19") "|" VerCompare("2.1", "2.0.19"))
} catch as e {
    Emit("M14=ERR " Type(e) ": " e.Message)
}
