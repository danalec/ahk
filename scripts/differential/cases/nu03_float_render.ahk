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
    Emit("N6=" ("" (0.1 + 0.2)) "|" ("" (1/3)) "|" ("" (2/3)))
} catch as e {
    Emit("N6=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N7=" ("" 1.0) "|" ("" 100/4) "|" ("" 1e10) "|" ("" 2.5))
} catch as e {
    Emit("N7=ERR " Type(e) ": " e.Message)
}
try {
    Emit("N8=" ("" (0 - 0.5)) "|" ("" (1 - 0.9)) "|" ("" (10 / 4)))
} catch as e {
    Emit("N8=ERR " Type(e) ": " e.Message)
}
