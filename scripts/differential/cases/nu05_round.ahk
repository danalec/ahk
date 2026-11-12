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
    Emit("N12=" Round(2.4) "|" Round(2.5) "|" Round(3.5) "|" Round(-2.5) "|" Round(2.567, 2))
} catch as e {
    Emit("N12=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N13=" Round(1.5) "|" Round(0.5) "|" Round(-0.5) "|" Round(2.345, 2))
} catch as e {
    Emit("N13=ERR " Type(e) ": " e.Message)
}
