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
; Measured on the binary of v2.0.19 on 2026-11-09: `f(a, b?)` marks the
; parameter optional; a call that leaves it out leaves it unset, and the
; arity counts the required parameters alone.
f(a, b?) {
    return IsSet(b) ? "set:" b : "unset"
}
class Holder {
    m(a, b?) {
        return IsSet(b)
    }
}
try {
    Emit("PA1=" f(1) "|" f(1, 2) "|" Holder().m(1) "|" Holder().m(1, 2))
} catch as e {
    Emit("PA1=ERR " Type(e) ": " e.Message)
}
g := f
try {
    Emit("PA2=" g(9) "|" g(9, 8))
} catch as e {
    Emit("PA2=ERR " Type(e) ": " e.Message)
}
