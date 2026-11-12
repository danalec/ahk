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
    name := "t2.txt"
    if FileExist(name)
        FileDelete(name)
    f := FileOpen(name, "w")
    written := f.Write("hello")
    f.Close()
    f2 := FileOpen(name, "r")
    content := f2.Read()
    f2.Close()
    Emit("G3=" content "|" written)
} catch as e {
    Emit("G3=ERR " Type(e) ": " e.Message)
}
