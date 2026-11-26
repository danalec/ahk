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
    Emit("N9=" (0x10) "|" (0xFF) "|" (0x10 + 1) "|" (0xFF & 0xF0))
} catch as e {
    Emit("N9=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N10=" Integer("0x1F") "|" Integer("42") "|" Float("2.5") "|" Number("7"))
} catch as e {
    Emit("N10=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N11=" Format("{:x}", 255) "|" Format("{:X}", 255) "|" Format("{:d}", 42))
} catch as e {
    Emit("N11=ERR " Type(e) ": " e.Message)
}
