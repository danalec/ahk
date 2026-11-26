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
    Emit("A1=" (1 + 2 * 3) "|" ((1 + 2) * 3) "|" (2 ** 3 ** 2) "|" (-2 ** 2) "|" ((-2) ** 2))
} catch as e {
    Emit("A1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("A2=" (10 - 2 - 3) "|" (2 + 3 * 4 - 1) "|" (1 << 3) "|" (16 >> 2))
} catch as e {
    Emit("A2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("A3=" (2 * 3 + 4 * 5) "|" (100 / 10 / 2) "|" (7 + 3 * 2 ** 2))
} catch as e {
    Emit("A3=ERR " Type(e) ": " e.Message)
}
