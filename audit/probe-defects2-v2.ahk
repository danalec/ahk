; The two defects of the audit that are still open, measured on the binary.

o := {a: 1}
try {
    p := ObjAddRef(o)
    FileAppend("objaddref of an object: " p, "*")
} catch as e {
    FileAppend("objaddref of an object: [" e.Message "]", "*")
}

try {
    r := ObjRelease(o)
    FileAppend("objrelease of an object: " r, "*")
} catch as e {
    FileAppend("objrelease of an object: [" e.Message "]", "*")
}

try {
    r := ObjRelease(&o)
    FileAppend("objrelease of a pointer: " r, "*")
} catch as e {
    FileAppend("objrelease of a pointer: [" e.Message "]", "*")
}

try {
    FileCopy("nope-not-here.txt", "nope2.txt")
    FileAppend("filecopy: no failure at all", "*")
} catch as e {
    FileAppend("filecopy: [" e.Message "]", "*")
}

try {
    FileMove("nope-not-here.txt", "nope2.txt")
    FileAppend("filemove: no failure at all", "*")
} catch as e {
    FileAppend("filemove: [" e.Message "]", "*")
}

ExitApp
