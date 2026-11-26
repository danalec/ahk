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
    Emit("S1=" SubStr("abcdef", 2) "|" SubStr("abcdef", 2, 3) "|" SubStr("abcdef", -2) "|" SubStr("abcdef", 0))
} catch as e {
    Emit("S1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S2=" SubStr("abcdef", 2, -1) "|" SubStr("abcdef", 6) "|" SubStr("abcdef", 7) "|" SubStr("abcdef", 1, 0))
} catch as e {
    Emit("S2=ERR " Type(e) ": " e.Message)
}
