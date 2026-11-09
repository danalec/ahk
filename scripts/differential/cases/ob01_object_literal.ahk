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
    o := {a: 1, b: "two"}
    Emit("O1=" o.a "|" o.b "|" o.HasProp("a") "|" o.HasProp("z"))
} catch as e {
    Emit("O1=ERR " Type(e) ": " e.Message)
}
try {
    o2 := {nested: {v: 9}}
    Emit("O2=" o2.nested.v)
} catch as e {
    Emit("O2=ERR " Type(e) ": " e.Message)
}
