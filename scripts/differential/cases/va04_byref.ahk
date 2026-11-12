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
Swap(&a, &b) {
    tmp := a
    a := b
    b := tmp
}
try {
    x := 1
    y := 2
    Swap(&x, &y)
    Emit("V4=" x "|" y)
} catch as e {
    Emit("V4=ERR " Type(e) ": " e.Message)
}
try {
    s := "left"
    t := "right"
    Swap(&s, &t)
    Emit("V5=" s "|" t)
} catch as e {
    Emit("V5=ERR " Type(e) ": " e.Message)
}
