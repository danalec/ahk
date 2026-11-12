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
    for v in [10, 20, 30]
        s .= v "-"
    Emit("K13=" s)
} catch as e {
    Emit("K13=ERR " Type(e) ": " e.Message)
}
try {
    s2 := ""
    for k, v in Map("a", 1, "b", 2)
        s2 .= k "=" v ";"
    Emit("K14=" s2)
} catch as e {
    Emit("K14=ERR " Type(e) ": " e.Message)
}
