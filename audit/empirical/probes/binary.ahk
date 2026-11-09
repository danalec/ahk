; Probe: the binary family of v2 (NumGet, NumPut, VarSetStrCapacity,
; Buffer, StrPtr). No address of the machine is read: the buffer is one
; the script makes.
acc := ""

try {
    r := VarSetStrCapacity(&s, 64)
    acc := acc "VarSetStrCapacity=OK|[" r "]`n"
} catch as e {
    acc := acc "VarSetStrCapacity=ERR|" e.Message "`n"
}
try {
    r := StrPtr("hello")
    acc := acc "StrPtr=OK|" r "`n"
} catch as e {
    acc := acc "StrPtr=ERR|" e.Message "`n"
}
try {
    b := Buffer(16, 0)
    acc := acc "Buffer.call=OK|type:" Type(b) "|size:" b.Size "`n"
} catch as e {
    acc := acc "Buffer.call=ERR|" e.Message "`n"
}
try {
    r := NumGet(1, 0, "Int")
    acc := acc "NumGet=OK|[" r "]`n"
} catch as e {
    acc := acc "NumGet=ERR|" e.Message "`n"
}
try {
    r := NumPut("Int", 42, 1, 0)
    acc := acc "NumPut=OK|[" r "]`n"
} catch as e {
    acc := acc "NumPut=ERR|" e.Message "`n"
}
try {
    b := Buffer(8, 0)
    r := NumPut("Int", 7, b, 0)
    acc := acc "NumPut.buffer=OK|[" r "]`n"
} catch as e {
    acc := acc "NumPut.buffer=ERR|" e.Message "`n"
}
try {
    b := Buffer(8, 0)
    NumPut("Int", 7, b, 0)
    r := NumGet(b, 0, "Int")
    acc := acc "NumGet.buffer=OK|[" r "]`n"
} catch as e {
    acc := acc "NumGet.buffer=ERR|" e.Message "`n"
}

FileAppend acc, "*"
