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
    Emit("S19=" Format("{:.2f}", 3.14159) "|" Format("{} and {}", "a", "b") "|" Format("{:.1f}", 2.55))
} catch as e {
    Emit("S19=ERR " Type(e) ": " e.Message)
}
try {
    Emit("S20=" Format("{:05d}", 42) "|" Format("{:x}", 255) "|" Format("{:.0f}", 2.5))
} catch as e {
    Emit("S20=ERR " Type(e) ": " e.Message)
}
