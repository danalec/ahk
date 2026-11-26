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
    a := [10, 20, 30]
    a.Push(40)
    Emit("O8=" a.Length "|" a[1] "|" a[4] "|" a.Has(2))
} catch as e {
    Emit("O8=ERR " Type(e) ": " e.Message)
}
try {
    Emit("O9=" a[0])
} catch as e {
    Emit("O9=ERR " Type(e) ": " e.Message)
}
