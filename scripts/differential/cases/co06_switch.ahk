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
    r := ""
    switch "b" {
        case "a":
            r := "A"
        case "b":
            r := "B"
        default:
            r := "D"
    }
    Emit("K8=" r)
} catch as e {
    Emit("K8=ERR " Type(e) ": " e.Message)
}
try {
    r2 := ""
    x := 3
    switch x {
        case 1, 2:
            r2 := "low"
        case 3, 4:
            r2 := "high"
        default:
            r2 := "other"
    }
    Emit("K9=" r2)
} catch as e {
    Emit("K9=ERR " Type(e) ": " e.Message)
}
