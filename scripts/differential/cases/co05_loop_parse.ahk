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
    Loop Parse "a,b,,c", ","
        s .= "[" A_LoopField "]"
    Emit("K6=" s)
} catch as e {
    Emit("K6=ERR " Type(e) ": " e.Message)
}
try {
    s2 := ""
    Loop Parse "a1b1c", "1"
        s2 .= A_LoopField
    Emit("K7=" s2)
} catch as e {
    Emit("K7=ERR " Type(e) ": " e.Message)
}
