; Probe: hotstrings, round two, with the parameters that v2 gives the
; command: the trigger and the replacement are two parameters, not one.
acc := ""

try {
    r := Hotstring("::btw::", "by the way")
    acc := acc "Hotstring.two-params=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "Hotstring.two-params=ERR|" e.Message "`n"
}
try {
    r := Hotstring(":oc:btw", "by the way")
    acc := acc "Hotstring.options=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "Hotstring.options=ERR|" e.Message "`n"
}
try {
    r := Hotstring("::btw::", "off", "Off")
    acc := acc "Hotstring.off=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "Hotstring.off=ERR|" e.Message "`n"
}

FileAppend acc, "*"
