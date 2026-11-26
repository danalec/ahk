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
    u := unset
    tmpPath := TempPath()
    Emit("V6=" IsSet(u))
} catch as e {
    Emit("V6=ERR " Type(e) ": " e.Message)
}
try {
    w := 5
    Emit("V7=" IsSet(w) "|" IsSet(u))
} catch as e {
    Emit("V7=ERR " Type(e) ": " e.Message)
}
TempPath() {
    return "x"
}
