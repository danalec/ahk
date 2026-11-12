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
class Fallback {
    __Call(name, params) {
        return "CALL:" name "(" params.Length ")"
    }
}
try {
    Emit("C6=" Fallback().anything(1, 2) "|" Fallback().other())
} catch as e {
    Emit("C6=ERR " Type(e) ": " e.Message)
}
