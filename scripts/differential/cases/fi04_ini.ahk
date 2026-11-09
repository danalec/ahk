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
    name := "t4.ini"
    if FileExist(name)
        FileDelete(name)
    IniWrite("bar", name, "sec", "key")
    Emit("G5=" IniRead(name, "sec", "key", "def") "|" IniRead(name, "sec", "nokey", "def"))
} catch as e {
    Emit("G5=ERR " Type(e) ": " e.Message)
}
