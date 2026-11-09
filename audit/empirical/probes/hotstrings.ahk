; Probe: hotstrings, and the HotIf family. Kept apart from hotkeys.ahk for
; the same reason: a registration is checked before the first statement.
acc := ""
try {
    r := Hotstring("::btw::by the way")
    acc := acc "Hotstring=OK|[" r "]`n"
} catch as e {
    acc := acc "Hotstring=ERR|" e.Message "`n"
}
try {
    r := HotIfWinActive("ahk_class Notepad")
    acc := acc "HotIfWinActive=OK|[" r "]`n"
} catch as e {
    acc := acc "HotIfWinActive=ERR|" e.Message "`n"
}
try {
    r := HotIf()
    acc := acc "HotIf=OK|[" r "]`n"
} catch as e {
    acc := acc "HotIf=ERR|" e.Message "`n"
}

FileAppend acc, "*"
