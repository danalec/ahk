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
    m := Map()
    m[1] := "int"
    m[1.0] := "float"
    m["1.0"] := "str"
    Emit("O15=" m.Count "|" m[1] "|" m[1.0] "|" m["1.0"])
} catch as e {
    Emit("O15=ERR " Type(e) ": " e.Message)
}
