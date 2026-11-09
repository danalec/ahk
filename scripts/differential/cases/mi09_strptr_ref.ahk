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
    v := "hello"
    Emit("M16=" Type(StrPtr(&v)))
} catch as e {
    Emit("M16=ERR " Type(e) ": " e.Message)
}
