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
    name := "t5.txt"
    if FileExist(name)
        FileDelete(name)
    FileAppend("a`nb`nc`n", name)
    s := ""
    Loop Read "t5.txt"
        s .= "[" A_LoopReadLine "]"
    Emit("G6=" s)
} catch as e {
    Emit("G6=ERR " Type(e) ": " e.Message)
}
