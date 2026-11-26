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
    Emit("M7=" FormatTime("20200102030405", "yyyy-MM-dd HH:mm:ss") "|" FormatTime("20200102030405", "yyyyMMdd"))
} catch as e {
    Emit("M7=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M8=" FormatTime("20200102030405", "MM/dd/yyyy") "|" FormatTime("19991231235959", "yyyy-MM-dd"))
} catch as e {
    Emit("M8=ERR " Type(e) ": " e.Message)
}
