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
    parts := StrSplit("a,b,c", ",")
    Emit("S13=" parts.Length "|" parts[1] "|" parts[3])
} catch as e {
    Emit("S13=ERR " Type(e) ": " e.Message)
}
try {
    p2 := StrSplit("a b c")
    Emit("S14=" p2.Length "|" p2[2])
} catch as e {
    Emit("S14=ERR " Type(e) ": " e.Message)
}
