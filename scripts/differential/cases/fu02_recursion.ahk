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
Fact(n) {
    if (n <= 1)
        return 1
    return n * Fact(n - 1)
}
Fib(n) {
    if (n < 2)
        return n
    return Fib(n - 1) + Fib(n - 2)
}
try {
    Emit("F3=" Fact(5) "|" Fib(10))
} catch as e {
    Emit("F3=ERR " Type(e) ": " e.Message)
}
