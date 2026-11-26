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
    loop {
        i += 1
        s .= i
    } until i >= 3
    Emit("K5=" s)
} catch as e {
    Emit("K5=ERR " Type(e) ": " e.Message)
}
