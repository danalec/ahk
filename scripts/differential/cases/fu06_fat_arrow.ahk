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
    f := (x) => x * 2
    Emit("F8=" f(21))
} catch as e {
    Emit("F8=ERR " Type(e) ": " e.Message)
}
try {
    g := (a, b) => a + b
    Emit("F9=" g(2, 3))
} catch as e {
    Emit("F9=ERR " Type(e) ": " e.Message)
}
