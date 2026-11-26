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
; A window that is not there refuses with "Target window not found." and the
; title in the `Extra` -- measured on the binary of v2.0.31.
try {
    Emit("W1=" WinGetStyle("ZZZ_NO_WINDOW_ZZZ"))
} catch as e {
    Emit("W1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("W2=" WinGetExStyle("ZZZ_NO_WINDOW_ZZZ"))
} catch as e {
    Emit("W2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("W3=" WinGetTransColor("ZZZ_NO_WINDOW_ZZZ"))
} catch as e {
    Emit("W3=ERR " Type(e) ": " e.Message)
}
try {
    Emit("W4=" ControlGetStyle("Edit1", "ZZZ_NO_WINDOW_ZZZ"))
} catch as e {
    Emit("W4=ERR " Type(e) ": " e.Message)
}
try {
    Emit("W5=" ControlGetExStyle("Edit1", "ZZZ_NO_WINDOW_ZZZ"))
} catch as e {
    Emit("W5=ERR " Type(e) ": " e.Message)
}
