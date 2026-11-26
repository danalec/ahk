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
    m := Map()
    m.Default := 99
    Emit("O7=" m.Get("nope") "|" m.Get("nope", 7) "|" m["nope"])
} catch as e {
    Emit("O7=ERR " Type(e) ": " e.Message)
}
