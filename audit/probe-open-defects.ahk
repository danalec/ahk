; The defects of the audit that are still open, probed one by one.
; Each call is in its own try, so that one failure does not stop the rest.

; 1. #Requires rejects its own version.
try {
    #Requires AutoHotkey v2.0
    MsgBox "requires: accepted"
} catch as e {
    MsgBox "requires: [" e.Message "]"
}

; 2. ImageSearch fails with an EMPTY message.
x := 0
y := 0
try {
    z := ImageSearch(&x, &y, 0, 0, 10, 10, "nope-not-a-picture.png")
    MsgBox "imagesearch: " z
} catch as e {
    MsgBox "imagesearch: [" e.Message "]"
}

; 3. DateAdd and DateDiff with the documented unit words.
try {
    d := DateAdd("20260101000000", 11, "Days")
    MsgBox "dateadd: " d
} catch as e {
    MsgBox "dateadd: [" e.Message "]"
}

try {
    d := DateDiff("20260101000000", "20260103000000", "Days")
    MsgBox "datediff: " d
} catch as e {
    MsgBox "datediff: [" e.Message "]"
}

; 4. FileCopy answers the literal "Failed", losing the cause.
try {
    FileCopy("nope-not-here.txt", "nope2.txt")
    MsgBox "filecopy: no failure at all"
} catch as e {
    MsgBox "filecopy: [" e.Message "]"
}

; 7. ObjRelease refused while ObjAddRef is a value.
o := {a: 1}
try {
    r := ObjAddRef(o)
    MsgBox "objaddref: " r
} catch as e {
    MsgBox "objaddref: [" e.Message "]"
}
try {
    r := ObjRelease(o)
    MsgBox "objrelease: " r
} catch as e {
    MsgBox "objrelease: [" e.Message "]"
}
