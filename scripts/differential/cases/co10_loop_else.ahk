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
; Measured on the binary of v2.0.19 on 2026-11-09: every loop kind may carry
; an `else`, and it runs exactly when the loop took no turn.
seen := ""
Loop 0 {
    seen .= "L0-body,"
} else {
    seen .= "L0-else,"
}
Loop 1 {
    seen .= "L1-body,"
} else {
    seen .= "L1-else,"
}
i := 0
While (i < 0) {
    seen .= "W-body,"
} else {
    seen .= "W-else,"
}
For x in [] {
    seen .= "F-body,"
} else {
    seen .= "F-else,"
}
Loop Parse "" {
    seen .= "P-body,"
} else {
    seen .= "P-else,"
}
Emit("LE=" seen)