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
    p := "C:\dir\file.txt"
    n := ""
    d := ""
    SplitPath(p, &n, &d)
    Emit("S9=" n "|" d)
} catch as e {
    Emit("S9=ERR " Type(e) ": " e.Message)
}
try {
    p2 := "C:\dir\file.txt"
    a1 := ""
    a2 := ""
    a3 := ""
    a4 := ""
    a5 := ""
    SplitPath(p2, &a1, &a2, &a3, &a4, &a5)
    Emit("S10=" a1 "|" a2 "|" a3 "|" a4 "|" a5)
} catch as e {
    Emit("S10=ERR " Type(e) ": " e.Message)
}
