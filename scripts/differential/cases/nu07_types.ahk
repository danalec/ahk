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
    Emit("N17=" Type(2) "|" Type(2.0) "|" Type(1/2) "|" Type(0x10) "|" Type("s"))
} catch as e {
    Emit("N17=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N18=" IsNumber("42") "|" IsNumber("abc") "|" IsInteger(2.0) "|" IsFloat(2) "|" IsObject([1]))
} catch as e {
    Emit("N18=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N19=" ("" 2) "|" ("" 2.0) "|" ("" 0x10) "|" ("" (2 = 2.0)))
} catch as e {
    Emit("N19=ERR " Type(e) ": " e.Message)
}
