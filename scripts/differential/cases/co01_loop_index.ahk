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
    s := ""
    Loop 5
        s .= A_Index
    Emit("K1=" s)
} catch as e {
    Emit("K1=ERR " Type(e) ": " e.Message)
}
try {
    s2 := ""
    Loop 0
        s2 .= "x"
    Emit("K2=" (s2 = ""))
} catch as e {
    Emit("K2=ERR " Type(e) ": " e.Message)
}
