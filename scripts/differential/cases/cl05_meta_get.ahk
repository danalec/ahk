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
class Logger {
    __Get(name, params) {
        return "GET:" name
    }
}
try {
    l := Logger()
    Emit("C5=" l.foo "|" l.bar)
} catch as e {
    Emit("C5=ERR " Type(e) ": " e.Message)
}
