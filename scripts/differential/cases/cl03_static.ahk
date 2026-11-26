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
class Counter {
    static Total := 0
    static Add(n) {
        Counter.Total += n
        return Counter.Total
    }
}
try {
    Emit("C3=" Counter.Add(5) "|" Counter.Add(3) "|" Counter.Total)
} catch as e {
    Emit("C3=ERR " Type(e) ": " e.Message)
}
