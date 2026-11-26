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
i := 5
try {
    Emit("D1=" (i > 3 ? "big" : "small") "|" (i = 5 && i < 10) "|" (i = 0 || i = 5) "|" (!0) "|" (!1) "|" (!!"x"))
} catch as e {
    Emit("D1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("D2=" (1 && 2) "|" (0 || 3) "|" ("" || "fallback") "|" (1 ? 0 : 9))
} catch as e {
    Emit("D2=ERR " Type(e) ": " e.Message)
}
