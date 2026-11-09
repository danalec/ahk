; Probe: the native and the object family of v2, except DllCall, which is in
; dllcall.ahk so that a crash cannot take this report with it.
acc := ""

try {
    r := ComObject("Shell.Application")
    acc := acc "ComObject=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "ComObject=ERR|" e.Message "`n"
}
try {
    r := ComCall(1, "kv", "x")
    acc := acc "ComCall=OK|[" r "]`n"
} catch as e {
    acc := acc "ComCall=ERR|" e.Message "`n"
}
try {
    o := {a: 1}
    r := ObjGetBase(o)
    acc := acc "ObjGetBase=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "ObjGetBase=ERR|" e.Message "`n"
}
try {
    o := {a: 1}
    r := ObjSetBase(o, {})
    acc := acc "ObjSetBase=OK|[" r "]`n"
} catch as e {
    acc := acc "ObjSetBase=ERR|" e.Message "`n"
}
try {
    o := {a: 1}
    r := ObjOwnPropCount(o)
    acc := acc "ObjOwnPropCount=OK|[" r "]`n"
} catch as e {
    acc := acc "ObjOwnPropCount=ERR|" e.Message "`n"
}

FileAppend acc, "*"
