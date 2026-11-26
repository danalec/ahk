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
    Emit("M1=" StrReplace(Sort("b`na`nc"), "`n", "|"))
} catch as e {
    Emit("M1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M2=" StrReplace(Sort("233`n40", "N"), "`n", "|"))
} catch as e {
    Emit("M2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M3=" StrReplace(Sort("b`nA`na"), "`n", "|"))
} catch as e {
    Emit("M3=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M4=" StrReplace(Sort("b`nA`na", "C"), "`n", "|"))
} catch as e {
    Emit("M4=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M5=" StrReplace(Sort("a`nb`na", "U"), "`n", "|"))
} catch as e {
    Emit("M5=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M6=" StrReplace(Sort("c,b,a", "D,"), ",", "|"))
} catch as e {
    Emit("M6=ERR " Type(e) ": " e.Message)
}
