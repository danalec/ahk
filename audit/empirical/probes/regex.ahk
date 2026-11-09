; Probe: the regular expressions and the string functions of v2.
acc := ""

try {
    r := RegExMatch("hello world 42", "(\d+)", &m)
    acc := acc "RegExMatch=OK|[" r "]|m1:[" m[1] "]`n"
} catch as e {
    acc := acc "RegExMatch=ERR|" e.Message "`n"
}
try {
    r := RegExReplace("hello world", "o", "0")
    acc := acc "RegExReplace=OK|[" r "]`n"
} catch as e {
    acc := acc "RegExReplace=ERR|" e.Message "`n"
}
try {
    r := InStr("hello world", "world")
    acc := acc "InStr=OK|[" r "]`n"
} catch as e {
    acc := acc "InStr=ERR|" e.Message "`n"
}
try {
    a := StrSplit("a,b,c", ",")
    acc := acc "StrSplit=OK|type:" Type(a) "|len:" a.Length "|2nd:[" a[2] "]`n"
} catch as e {
    acc := acc "StrSplit=ERR|" e.Message "`n"
}
try {
    r := Sort("c`na`nb", "D`n")
    acc := acc "Sort=OK|[" r "]`n"
} catch as e {
    acc := acc "Sort=ERR|" e.Message "`n"
}
try {
    r := Format("{1}-{2:05.2f}", "a", 3.14159)
    acc := acc "Format=OK|[" r "]`n"
} catch as e {
    acc := acc "Format=ERR|" e.Message "`n"
}
try {
    r := StrReplace("a-b-c", "-", "+")
    acc := acc "StrReplace=OK|[" r "]`n"
} catch as e {
    acc := acc "StrReplace=ERR|" e.Message "`n"
}
try {
    r := SubStr("abcdef", 2, 3)
    acc := acc "SubStr=OK|[" r "]`n"
} catch as e {
    acc := acc "SubStr=ERR|" e.Message "`n"
}
try {
    r := RegExMatch("abc", "(?i)XYZ")
    acc := acc "RegExMatch.option=OK|[" r "]`n"
} catch as e {
    acc := acc "RegExMatch.option=ERR|" e.Message "`n"
}

FileAppend acc, "*"
