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
MakeCounter(start) {
    n := start
    return Increment
    Increment() {
        n += 1
        return n
    }
}
try {
    c := MakeCounter(10)
    c()
    c()
    Emit("F4=" c())
} catch as e {
    Emit("F4=ERR " Type(e) ": " e.Message)
}
