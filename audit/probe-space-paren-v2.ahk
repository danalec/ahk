; The parenthesis that is spaced: is it a call or a concatenation?

try {
    n := StrLen ("abc")
    FileAppend("spaced: a call, and the answer is " n, "*")
} catch as e {
    FileAppend("spaced: [" e.Message "]", "*")
}

try {
    n := StrLen("abc")
    FileAppend("tight: a call, and the answer is " n, "*")
} catch as e {
    FileAppend("tight: [" e.Message "]", "*")
}

f := (a) => StrLen (a)
try {
    n := f("abcd")
    FileAppend("in a closure: a call, and the answer is " n, "*")
} catch as e {
    FileAppend("in a closure: [" e.Message "]", "*")
}

ExitApp
