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
    if FileExist("q7.txt")
        FileDelete("q7.txt")
    FileAppend("z", "q7.txt")
    Emit("G8=" (FileExist("q7.txt") = "A") "|" (FileExist("nope_xyz.txt") = "") "|" FileExist("./"))
} catch as e {
    Emit("G8=ERR " Type(e) ": " e.Message)
}
