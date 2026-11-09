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
    i := 0
    s := ""
    while (i < 3) {
        i += 1
        s .= i
    }
    Emit("K4=" s "|" i)
} catch as e {
    Emit("K4=ERR " Type(e) ": " e.Message)
}
