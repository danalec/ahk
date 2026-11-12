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
Def(a, b := 10, c := 20) {
    return a + b + c
}
Sum(rest*) {
    s := 0
    for v in rest
        s += v
    return s
}
try {
    Emit("F1=" Def(1) "|" Def(1, 2) "|" Def(1, 2, 3))
} catch as e {
    Emit("F1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("F2=" Sum(1, 2, 3, 4) "|" Sum())
} catch as e {
    Emit("F2=ERR " Type(e) ": " e.Message)
}
