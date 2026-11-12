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
    try {
        throw ValueError("bad")
    } catch ValueError as e {
        Emit("K10=" Type(e) "|" e.Message)
    }
} catch as e {
    Emit("K10=ERR " Type(e) ": " e.Message)
}
try {
    try {
        throw TypeError("t")
    } catch ValueError as e1 {
        Emit("K11=wrong")
    } catch as e2 {
        Emit("K11=" Type(e2))
    }
} catch as e {
    Emit("K11=ERR " Type(e) ": " e.Message)
}
try {
    s := ""
    try {
        throw ValueError("x")
    } finally {
        s .= "fin"
    }
    Emit("K12=" s)
} catch as e {
    Emit("K12=ERR " Type(e) ": " e.Message)
}
