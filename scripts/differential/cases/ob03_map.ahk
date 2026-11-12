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
    m := Map("a", 1, "b", 2)
    m["c"] := 3
    Emit("O5=" m.Count "|" m["a"] "|" m.Has("c") "|" m.Has("z") "|" m.Get("b"))
} catch as e {
    Emit("O5=ERR " Type(e) ": " e.Message)
}
try {
    m2 := Map()
    m2["k"] := "v"
    removed := m2.Delete("k")
    Emit("O6=" m2.Count "|" removed)
} catch as e {
    Emit("O6=ERR " Type(e) ": " e.Message)
}
