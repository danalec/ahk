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
    if FileExist("f1.txt")
        FileDelete("f1.txt")
    if FileExist("f2.txt")
        FileDelete("f2.txt")
    FileAppend("x", "f1.txt")
    FileAppend("y", "f2.txt")
    names := []
    Loop Files "f*.txt"
        names.Push(A_LoopFileName)
    joined := ""
    for n in names
        joined .= n "`n"
    Emit("G7=" StrReplace(Sort(joined), "`n", "|") "|" names.Length)
} catch as e {
    Emit("G7=ERR " Type(e) ": " e.Message)
}
