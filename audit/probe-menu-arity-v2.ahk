; The arity of the callback of a menu item, measured on the binary.

m0 := Menu()
try {
    m0.Add("A", () => FileAppend("zero: ok", "*"))
    FileAppend("zero: accepted", "*")
} catch as e {
    FileAppend("zero: [" e.Message "]", "*")
}

m1 := Menu()
try {
    m1.Add("A", (ItemName) => FileAppend("one: ok", "*"))
    FileAppend("one: accepted", "*")
} catch as e {
    FileAppend("one: [" e.Message "]", "*")
}

m2 := Menu()
try {
    m2.Add("A", (ItemName, ItemPos) => FileAppend("two: ok", "*"))
    FileAppend("two: accepted", "*")
} catch as e {
    FileAppend("two: [" e.Message "]", "*")
}

m3 := Menu()
try {
    m3.Add("A", (ItemName, ItemPos, MyMenu) => FileAppend("three: ok", "*"))
    FileAppend("three: accepted", "*")
} catch as e {
    FileAppend("three: [" e.Message "]", "*")
}

m4 := Menu()
try {
    m4.Add("A", (ItemName, ItemPos, MyMenu, More) => FileAppend("four: ok", "*"))
    FileAppend("four: accepted", "*")
} catch as e {
    FileAppend("four: [" e.Message "]", "*")
}

m5 := Menu()
try {
    m5.Add("A", () => FileAppend("variadic: ok", "*"))
    FileAppend("variadic: accepted", "*")
} catch as e {
    FileAppend("variadic: [" e.Message "]", "*")
}

m6 := Menu()
try {
    m6.Add("A", Named)
    FileAppend("named: accepted", "*")
} catch as e {
    FileAppend("named: [" e.Message "]", "*")
}

ExitApp

Named(ItemName, ItemPos, MyMenu) {
    return 1
}
