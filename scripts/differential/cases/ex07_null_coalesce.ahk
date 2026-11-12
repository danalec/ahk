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
    a := unset
    Emit("G1=" (a ?? "default"))
} catch as e {
    Emit("G1=ERR " Type(e) ": " e.Message)
}
try {
    b := ""
    Emit("G2=" (b ?? "default"))
} catch as e {
    Emit("G2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("G3=" (0 ?? 9) "|" ("x" ?? "y"))
} catch as e {
    Emit("G3=ERR " Type(e) ": " e.Message)
}
