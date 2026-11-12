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
    o := Object()
    o.DefineProp("x", {Value: 7})
    Emit("O3=" o.x)
} catch as e {
    Emit("O3=ERR " Type(e) ": " e.Message)
}
try {
    o2 := Object()
    o2.DefineProp("y", {Value: "v"})
    o2.y := "changed"
    Emit("O4=" o2.y)
} catch as e {
    Emit("O4=ERR " Type(e) ": " e.Message)
}
