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
    a := [1, 2, 3]
    a.InsertAt(2, "x")
    first := a.RemoveAt(1)
    last := a.Pop()
    Emit("O10=" a.Length "|" first "|" last "|" a[1])
} catch as e {
    Emit("O10=ERR " Type(e) ": " e.Message)
}
