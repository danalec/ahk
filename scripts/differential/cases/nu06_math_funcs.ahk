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
    Emit("N14=" Abs(-3) "|" Abs(-2.5) "|" Abs(7))
} catch as e {
    Emit("N14=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N15=" Max(2, 9, 4) "|" Min(2, 9, 4) "|" Ceil(2.1) "|" Floor(2.9))
} catch as e {
    Emit("N15=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N16=" Mod(10, 3) "|" Mod(-10, 3) "|" Mod(10, -3))
} catch as e {
    Emit("N16=ERR " Type(e) ": " e.Message)
}
