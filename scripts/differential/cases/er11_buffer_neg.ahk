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
    b := Buffer(-1)
    Emit("R11=" b.Size)
} catch as e {
    Emit("R11=" Type(e) "|" e.Message "|" e.Extra)
}
