; The defects of the audit that are still open, probed one by one.
; Each call is in its own try, so that one failure does not stop the rest.

; 1. #Requires rejects its own version.
try {
    #Requires AutoHotkey v2.0
    Msg "requires: accepted"
} catch as e {
    Msg "requires: [" e.Message "]"
}

; 2. ImageSearch fails with an EMPTY message.
x := 0
y := 0
try {
    z := ImageSearch(&x, &y, 0, 0, 10, 10, "nope-not-a-picture.png")
    Msg "imagesearch: " z
} catch as e {
    Msg "imagesearch: [" e.Message "]"
}

; 3. DateAdd and DateDiff with the documented unit words.
try {
    d := DateAdd("20260101000000", 11, "Days")
    Msg "dateadd: " d
} catch as e {
    Msg "dateadd: [" e.Message "]"
}

try {
    d := DateDiff("20260101000000", "20260103000000", "Days")
    Msg "datediff: " d
} catch as e {
    Msg "datediff: [" e.Message "]"
}

; 4. FileCopy answers the literal "Failed", losing the cause.
try {
    FileCopy("nope-not-here.txt", "nope2.txt")
    Msg "filecopy: no failure at all"
} catch as e {
    Msg "filecopy: [" e.Message "]"
}

; 7. ObjRelease refused while ObjAddRef is a value.
o := {a: 1}
try {
    r := ObjAddRef(o)
    Msg "objaddref: " r
} catch as e {
    Msg "objaddref: [" e.Message "]"
}
try {
    r := ObjRelease(o)
    Msg "objrelease: " r
} catch as e {
    Msg "objrelease: [" e.Message "]"
}
