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
    Emit("B1=" ("x" 1 + 2) "|" (1 + 2 "d") "|" ("a" . "b") "|" ("a" "b" "c"))
} catch as e {
    Emit("B1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("B2=" ("n=" 42) "|" (42 "=" 42) "|" ("v" 1 + 2 "w"))
} catch as e {
    Emit("B2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("B3=" ("" 1/3) "|" ("" 2.0) "|" ("" 10/2) "|" ("" 0x10))
} catch as e {
    Emit("B3=ERR " Type(e) ": " e.Message)
}
