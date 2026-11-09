; Probe: INI family (IniRead, IniWrite, IniDelete).
acc := ""
ini := A_Temp "\ahk-probe.ini"

try {
    r := IniWrite("hello", ini, "sec", "key")
    acc := acc "IniWrite=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "IniWrite=ERR|" e.Message "`n"
}
try {
    r := IniRead(ini, "sec", "key", "default")
    acc := acc "IniRead=OK|[" r "]`n"
} catch as e {
    acc := acc "IniRead=ERR|" e.Message "`n"
}
try {
    r := IniRead(ini, "sec", "absent", "the-default")
    acc := acc "IniRead.default=OK|[" r "]`n"
} catch as e {
    acc := acc "IniRead.default=ERR|" e.Message "`n"
}
try {
    r := IniDelete(ini, "sec", "key")
    acc := acc "IniDelete=OK|ret:[" r "]|after:[" IniRead(ini, "sec", "key", "gone") "]`n"
} catch as e {
    acc := acc "IniDelete=ERR|" e.Message "`n"
}

FileAppend acc, "*"
