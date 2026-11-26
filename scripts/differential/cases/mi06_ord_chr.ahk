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
    Emit("M13=" Ord("A") "|" Chr(65) "|" Chr(66) "|" Ord("a"))
} catch as e {
    Emit("M13=ERR " Type(e) ": " e.Message)
}
