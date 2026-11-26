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
    Emit("N1=" (7 // 2) "|" (-7 // 2) "|" (7 // (-2)) "|" (Mod(7, 2)) "|" (Mod(-7, 2)) "|" (Mod(7, -2)))
} catch as e {
    Emit("N1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N2=" (0 // 5) "|" (5 // 5) "|" (6 // 5) "|" (Mod(0, 5)) "|" (Mod(10, 3)))
} catch as e {
    Emit("N2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N3=" (7 / 2) "|" (8 / 2) "|" (1 / 4))
} catch as e {
    Emit("N3=ERR " Type(e) ": " e.Message)
}
