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
    Emit("S15=" RegExMatch("Hello", "(?i)hello") "|" RegExMatch("xyz", "\d+"))
} catch as e {
    Emit("S15=ERR " Type(e) ": " e.Message)
}
try {
    m := ""
    pos := RegExMatch("abc123def", "(\d+)", &m)
    Emit("S16=" pos "|" m[0] "|" m[1])
} catch as e {
    Emit("S16=ERR " Type(e) ": " e.Message)
}
