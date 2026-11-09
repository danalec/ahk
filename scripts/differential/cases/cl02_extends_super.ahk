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
class Animal {
    __New(name) {
        this.Name := name
    }
    Speak() {
        return this.Name " speaks"
    }
}
class Dog extends Animal {
    Speak() {
        return super.Speak() " woof"
    }
}
try {
    Emit("C2=" Dog("Rex").Speak() "|" Dog("Rex").Name)
} catch as e {
    Emit("C2=ERR " Type(e) ": " e.Message)
}
