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
; The `Thread` of v2.0.31 writes the state of the thread of the run: the
; commands are "Priority", "Interrupt" and "NoTimers", the word is read
; without regard to its case, and a word that is not a command refuses with
; "Parameter #1 of Thread is invalid." -- measured on the binary of
; v2.0.31.
try {
    Thread "Priority", 5
    Emit("T1=ok")
} catch as e {
    Emit("T1=ERR " Type(e) ": " e.Message)
}
try {
    Thread "priority", 7
    Emit("T2=ok")
} catch as e {
    Emit("T2=ERR " Type(e) ": " e.Message)
}
try {
    Thread "Interrupt", 100, 5
    Emit("T3=ok")
} catch as e {
    Emit("T3=ERR " Type(e) ": " e.Message)
}
try {
    Thread "NoTimers", 0
    Emit("T4=ok")
} catch as e {
    Emit("T4=ERR " Type(e) ": " e.Message)
}
try {
    Thread "Bogus"
    Emit("T5=ok")
} catch as e {
    Emit("T5=ERR " Type(e) ": " e.Message)
}
