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
    Emit("S11=" StrUpper("abc") "|" StrLower("ABC") "|" StrLen("hello") "|" StrLen(""))
} catch as e {
    Emit("S11=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S12=" SubStr(StrUpper("abc"), 1, 2) "|" StrUpper("a1b2") "|" StrLower("A1B2"))
} catch as e {
    Emit("S12=ERR " Type(e) ": " e.Message)
}
