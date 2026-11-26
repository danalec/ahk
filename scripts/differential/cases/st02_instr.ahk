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
    Emit("S3=" InStr("hello world", "o") "|" InStr("hello world", "o", "Off", 6) "|" InStr("hello", "z"))
} catch as e {
    Emit("S3=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S4=" InStr("Hello", "h", "Off") "|" InStr("Hello", "h", "On") "|" InStr("abcabc", "b", "On", 3))
} catch as e {
    Emit("S4=ERR " Type(e) ": " e.Message)
}
