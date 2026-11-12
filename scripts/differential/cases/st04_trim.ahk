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
    Emit("S7=" Trim("  hi  ") "|" LTrim("  hi") "|" RTrim("hi  ") "|" Trim("xxhixx", "x"))
} catch as e {
    Emit("S7=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S8=" Trim("`t hi `t") "|" Trim("xxhixx", "x") "|" LTrim("xxabc", "x"))
} catch as e {
    Emit("S8=ERR " Type(e) ": " e.Message)
}
