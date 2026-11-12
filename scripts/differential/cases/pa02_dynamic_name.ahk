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
; Measured on the binary of v2.0.19 on 2026-11-09: a name that the script
; builds is read, written and incremented the way a variable is.
x := 1
y1 := 5
try {
    Emit("PD1=" y%x%)
} catch as e {
    Emit("PD1=ERR " Type(e) ": " e.Message)
}
try {
    y%x% := 9
    Emit("PD2=" y1)
} catch as e {
    Emit("PD2=ERR " Type(e) ": " e.Message)
}
y1 := 5
try {
    a := y%x%++
    b := ++y%x%
    Emit("PD3=" a "|" b "|" y1)
} catch as e {
    Emit("PD3=ERR " Type(e) ": " e.Message)
}