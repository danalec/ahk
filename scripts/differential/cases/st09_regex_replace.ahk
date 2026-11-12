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
    Emit("S17=" RegExReplace("a1b2", "\d", "#") "|" RegExReplace("aaa", "a+", "X"))
} catch as e {
    Emit("S17=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S18=" RegExReplace("abc", "b", "") "|" RegExReplace("a.b.c", "\.", "-"))
} catch as e {
    Emit("S18=ERR " Type(e) ": " e.Message)
}
