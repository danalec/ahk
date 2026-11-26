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
    Emit("E1=" (5 & 3) "|" (5 | 2) "|" (5 ^ 1) "|" (~5))
} catch as e {
    Emit("E1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("E2=" (1 << 10) "|" (1024 >> 3) "|" (0xFF & 0x0F) "|" (0x0F | 0xF0) "|" (0xFF ^ 0x0F))
} catch as e {
    Emit("E2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("E3=" (7 & 1) "|" (6 & 1) "|" (-1 >> 1) "|" ((-1) >>> 1))
} catch as e {
    Emit("E3=ERR " Type(e) ": " e.Message)
}
