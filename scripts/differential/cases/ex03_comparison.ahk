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
    Emit("C1=" (1 = "1") "|" ("a" = "A") "|" ("abc" = "ABC") "|" (1 == 1.0) "|" ("10" < "9") "|" (2 < "10"))
} catch as e {
    Emit("C1=ERR " Type(e) ": " e.Message)
}
try {
    Emit("C2=" ("" = 0) "|" ("" = "") "|" ("0" = 0) "|" ("abc" != "ABC") "|" ("abc" == "ABC"))
} catch as e {
    Emit("C2=ERR " Type(e) ": " e.Message)
}
try {
    Emit("C3=" (1 < 2) "|" (2 >= 2) "|" ("b" > "a") "|" ("B" < "a"))
} catch as e {
    Emit("C3=ERR " Type(e) ": " e.Message)
}
