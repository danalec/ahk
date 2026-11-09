; Probe: the string functions, round two: the shapes that were refused in
; regex.ahk are asked again with the plain form, so the refusal is known to
; be about the parameter and not about the function.
acc := ""

try {
    r := Sort("c`na`nb")
    acc := acc "Sort.plain=OK|[" r "]`n"
} catch as e {
    acc := acc "Sort.plain=ERR|" e.Message "`n"
}
try {
    r := RegExMatch("hello world 42", "(\d+)")
    acc := acc "RegExMatch.plain=OK|[" r "]`n"
} catch as e {
    acc := acc "RegExMatch.plain=ERR|" e.Message "`n"
}
try {
    m := []
    r := RegExMatch("hello 42", "(\d+)", &m)
    acc := acc "RegExMatch.matchvar=OK|[" r "]`n"
} catch as e {
    acc := acc "RegExMatch.matchvar=ERR|" e.Message "`n"
}
try {
    r := RegExReplace("hello world", "(\w+) (\w+)", "$2 $1")
    acc := acc "RegExReplace.groups=OK|[" r "]`n"
} catch as e {
    acc := acc "RegExReplace.groups=ERR|" e.Message "`n"
}
try {
    r := InStr("hello", "l", 1, 1)
    acc := acc "InStr.casesense=OK|[" r "]`n"
} catch as e {
    acc := acc "InStr.casesense=ERR|" e.Message "`n"
}
try {
    r := Format("{:d}|{:s}|{:.2f}", 42, "x", 3.14159)
    acc := acc "Format.forms=OK|[" r "]`n"
} catch as e {
    acc := acc "Format.forms=ERR|" e.Message "`n"
}
try {
    r := StrLen("hello")
    acc := acc "StrLen=OK|[" r "]`n"
} catch as e {
    acc := acc "StrLen=ERR|" e.Message "`n"
}

FileAppend acc, "*"
