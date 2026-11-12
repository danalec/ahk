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
    i := 5
    a := i++
    b := ++i
    c := i--
    d := --i
    Emit("V8=" i "|" a "|" b "|" c "|" d)
} catch as e {
    Emit("V8=ERR " Type(e) ": " e.Message)
}
try {
    n := 0
    n++
    n++
    Emit("V9=" n)
} catch as e {
    Emit("V9=ERR " Type(e) ": " e.Message)
}
