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
try {
    f := Func("Double")
    Emit("F5=" f(21) "|" f.Name "|" f.MinParams "|" f.MaxParams)
} catch as e {
    Emit("F5=ERR " Type(e) ": " e.Message)
}
