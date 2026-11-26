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
    v := ""
    cap := VarSetStrCapacity(&v, 64)
    Emit("M11=" cap "|" IsSet(v))
} catch as e {
    Emit("M11=ERR " Type(e) ": " e.Message)
}
try {
    p := StrPtr("abc")
    Emit("M12=" Type(p) "|" (p > 0))
} catch as e {
    Emit("M12=ERR " Type(e) ": " e.Message)
}
