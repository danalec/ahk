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
    Emit("M10=" DateDiff("20200110", "20200101", "Days") "|" DateDiff("20200301", "20200101", "Days") "|" DateDiff("20210101", "20200101", "Days"))
} catch as e {
    Emit("M10=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M10B=" DateDiff("20200102", "20200101", "Hours") "|" DateDiff("20200101", "20200101", "Seconds") "|" DateDiff("20200102", "20200101", "D") "|" DateDiff("20200101", "20200101", "M"))
} catch as e {
    Emit("M10B=ERR " Type(e) ": " e.Message)
}
