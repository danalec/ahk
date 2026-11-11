; The callback of a menu item: how many parameters does it take?
; The reference: a closure with no parameter, and one with the three.

try {
    m := Menu()
    m.Add("&Open", () => FileAppend("no parameter: ok", "*"))
    FileAppend("added the closure with no parameter", "*")
} catch as e {
    FileAppend("no parameter: [" e.Message "]", "*")
}

try {
    m2 := Menu()
    m2.Add("&Open", (ItemName, ItemPos, MyMenu) => FileAppend("three parameters: ok", "*"))
    FileAppend("added the closure with three parameters", "*")
} catch as e {
    FileAppend("three parameters: [" e.Message "]", "*")
}
ExitApp
