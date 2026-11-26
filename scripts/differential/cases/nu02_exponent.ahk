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
    Emit("N4=" (2 ** 10) "|" (2 ** 0.5) "|" (2 ** (-1)) "|" (3 ** 3))
} catch as e {
    Emit("N4=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N5=" (Sqrt(16)) "|" (Sqrt(2)) "|" (4 ** 0.5))
} catch as e {
    Emit("N5=ERR " Type(e) ": " e.Message)
}
