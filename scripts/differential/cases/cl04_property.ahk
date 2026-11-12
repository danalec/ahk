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
class Temp {
    __New() {
        this._c := 0
    }
    Celsius {
        get {
            return this._c
        }
        set {
            this._c := value
        }
    }
    Fahrenheit {
        get {
            return this._c * 9 / 5 + 32
        }
    }
}
try {
    t := Temp()
    t.Celsius := 100
    Emit("C4=" t.Celsius "|" t.Fahrenheit)
} catch as e {
    Emit("C4=ERR " Type(e) ": " e.Message)
}
