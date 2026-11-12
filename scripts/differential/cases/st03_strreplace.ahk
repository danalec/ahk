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
    Emit("S5=" StrReplace("a-b-c", "-", "+") "|" StrReplace("AAA", "A", "x") "|" StrReplace("abc", "z", "x"))
} catch as e {
    Emit("S5=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S6=" StrReplace("aaa", "a", "b") "|" StrReplace("abc", "b", "") "|" StrReplace("a1b1", "1", "2"))
} catch as e {
    Emit("S6=ERR " Type(e) ": " e.Message)
}
