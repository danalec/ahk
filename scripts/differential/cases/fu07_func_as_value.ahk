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
Double(x) {
    return x * 2
}
Apply(fn, v) {
    return fn(v)
}
try {
    Emit("F10=" Apply(Double, 21) "|" Apply(Abs, -5))
} catch as e {
    Emit("F10=ERR " Type(e) ": " e.Message)
}
