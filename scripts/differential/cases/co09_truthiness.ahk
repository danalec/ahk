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
    r := ""
    if (0)
        r .= "0"
    if ("")
        r .= "e"
    if ("0")
        r .= "s"
    if (1)
        r .= "1"
    if ("x")
        r .= "x"
    Emit("K15=" r)
} catch as e {
    Emit("K15=ERR " Type(e) ": " e.Message)
}
