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
    name := "t3.txt"
    if FileExist(name)
        FileDelete(name)
    h := FileOpen(name, "w")
    h.Write("0123456789")
    h.Close()
    h2 := FileOpen(name, "r")
    first := h2.Read(3)
    h2.Seek(6)
    rest := h2.Read()
    eof := h2.AtEOF
    pos := h2.Pos
    h2.Close()
    Emit("G4=" first "|" rest "|" eof "|" pos)
} catch as e {
    Emit("G4=ERR " Type(e) ": " e.Message)
}
