; The two shapes that the arity of three does not cover: a parameter with a
; default, and a rest.

m1 := Menu()
try {
    m1.Add("A", (ItemName, ItemPos, MyMenu := 0) => FileAppend("default: ok", "*"))
    FileAppend("default: accepted", "*")
} catch as e {
    FileAppend("default: [" e.Message "]", "*")
}

m2 := Menu()
try {
    m2.Add("A", (ItemName, ItemPos, MyMenu, Rest*) => FileAppend("rest: ok", "*"))
    FileAppend("rest: accepted", "*")
} catch as e {
    FileAppend("rest: [" e.Message "]", "*")
}

m3 := Menu()
try {
    m3.Add("A", (Rest*) => FileAppend("only rest: ok", "*"))
    FileAppend("only rest: accepted", "*")
} catch as e {
    FileAppend("only rest: [" e.Message "]", "*")
}

m4 := Menu()
try {
    m4.Add("A", WithDefault)
    FileAppend("named with a default: accepted", "*")
} catch as e {
    FileAppend("named with a default: [" e.Message "]", "*")
}

ExitApp

WithDefault(ItemName, ItemPos, MyMenu := 0) {
    return 1
}
