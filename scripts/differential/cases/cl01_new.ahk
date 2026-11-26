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
class Point {
    __New(x, y) {
        this.X := x
        this.Y := y
    }
    Sum() {
        return this.X + this.Y
    }
}
try {
    p := Point(3, 4)
    Emit("C1=" p.Sum() "|" p.X "|" p.Y "|" Type(p))
} catch as e {
    Emit("C1=ERR " Type(e) ": " e.Message)
}
