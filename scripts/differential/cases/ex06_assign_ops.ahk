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
    x := 5
    x += 2
    x -= 1
    x *= 3
    Emit("F1=" x)
} catch as e {
    Emit("F1=ERR " Type(e) ": " e.Message)
}
try {
    y := 17
    y //= 2
    Emit("F2=" y)
} catch as e {
    Emit("F2=ERR " Type(e) ": " e.Message)
}
try {
    s := "ab"
    s .= "cd"
    Emit("F3=" s)
} catch as e {
    Emit("F3=ERR " Type(e) ": " e.Message)
}
try {
    b := 6
    b |= 1
    b &= 7
    b ^= 2
    Emit("F4=" b)
} catch as e {
    Emit("F4=ERR " Type(e) ": " e.Message)
}
try {
    c := 8
    c >>= 1
    c <<= 2
    Emit("F5=" c)
} catch as e {
    Emit("F5=ERR " Type(e) ": " e.Message)
}
