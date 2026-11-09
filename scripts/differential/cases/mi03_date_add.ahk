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
    Emit("M9=" DateAdd("20200131", 1, "Days") "|" DateAdd("20200101", 1, "Hours") "|" DateAdd("20200101", 90, "Seconds") "|" DateAdd("20200101", -1, "Days"))
} catch as e {
    Emit("M9=ERR " Type(e) ": " e.Message)
}
try {
    Emit("M9B=" DateAdd("20200101", 1, "D") "|" DateAdd("20200101", 1, "H") "|" DateAdd("20200101", 1, "M") "|" DateAdd("20200101", 1, "S"))
} catch as e {
    Emit("M9B=ERR " Type(e) ": " e.Message)
}
