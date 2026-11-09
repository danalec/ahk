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
try
    SetWorkingDir(EnvGet("AUDIT_TMP"))
catch as e
    Emit("SETUP=ERR " Type(e) ": " e.Message)
try {
    name := "t8.txt"
    if FileExist(name)
        FileDelete(name)
    h := FileOpen(name, "w")
    h.Write("abcdef")
    h.Close()
    h2 := FileOpen(name, "r")
    len := h2.Length
    h2.Close()
    Emit("G9=" len)
} catch as e {
    Emit("G9=ERR " Type(e) ": " e.Message)
}
