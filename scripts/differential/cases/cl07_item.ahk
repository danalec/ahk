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
class Store {
    __New() {
        this._data := []
    }
    __Item[key] {
        get {
            return this._data[key]
        }
        set {
            this._data[key] := value
        }
    }
}
try {
    s := Store()
    s[1] := "one"
    s[2] := "two"
    Emit("C7=" s[1] "|" s[2])
} catch as e {
    Emit("C7=ERR " Type(e) ": " e.Message)
}
try {
    Emit("C8=" s[9])
} catch as e {
    Emit("C8=ERR " Type(e) ": " e.Message)
}
