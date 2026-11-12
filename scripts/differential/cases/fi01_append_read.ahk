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
    name := "t1.txt"
    if FileExist(name)
        FileDelete(name)
    FileAppend("line1`nline2`n", name)
    Emit("G1=" StrReplace(FileRead(name), "`n", "|"))
} catch as e {
    Emit("G1=ERR " Type(e) ": " e.Message)
}
try {
    if FileExist("t1b.txt")
        FileDelete("t1b.txt")
    FileAppend("tail", "t1b.txt")
    FileAppend("more", "t1b.txt")
    Emit("G2=" FileRead("t1b.txt"))
} catch as e {
    Emit("G2=ERR " Type(e) ": " e.Message)
}
