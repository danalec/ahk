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
    a := [1, 2]
    c := a.Clone()
    c.Push(3)
    Emit("O13=" a.Length "|" c.Length)
} catch as e {
    Emit("O13=ERR " Type(e) ": " e.Message)
}
try {
    m := Map("a", 1)
    m2 := m.Clone()
    m2["b"] := 2
    Emit("O14=" m.Count "|" m2.Count)
} catch as e {
    Emit("O14=ERR " Type(e) ": " e.Message)
}
