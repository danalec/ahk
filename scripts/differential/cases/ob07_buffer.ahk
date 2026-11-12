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
    buf := Buffer(8, 0)
    NumPut("Int", 42, buf, 0)
    NumPut("Int", 7, buf, 4)
    Emit("O11=" buf.Size "|" NumGet(buf, 0, "Int") "|" NumGet(buf, 4, "Int"))
} catch as e {
    Emit("O11=ERR " Type(e) ": " e.Message)
}
try {
    b2 := Buffer(4, 0)
    NumPut("UChar", 65, b2, 0)
    Emit("O12=" NumGet(b2, 0, "UChar"))
} catch as e {
    Emit("O12=ERR " Type(e) ": " e.Message)
}
