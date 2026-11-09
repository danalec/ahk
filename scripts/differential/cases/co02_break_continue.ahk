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
    s := ""
    Loop 6 {
        if (Mod(A_Index, 2) = 0)
            continue
        if (A_Index > 4)
            break
        s .= A_Index
    }
    Emit("K3=" s)
} catch as e {
    Emit("K3=ERR " Type(e) ": " e.Message)
}
