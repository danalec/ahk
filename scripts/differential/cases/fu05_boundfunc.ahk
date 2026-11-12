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
Add(a, b) {
    return a + b
}
try {
    bf := Add.Bind(10)
    Emit("F6=" bf(5))
} catch as e {
    Emit("F6=ERR " Type(e) ": " e.Message)
}
try {
    Emit("F7=" IsObject(bf) "|" Type(bf))
} catch as e {
    Emit("F7=ERR " Type(e) ": " e.Message)
}
