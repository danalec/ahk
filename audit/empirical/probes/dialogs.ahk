; Probe: the user dialogs. These may open a window and wait for a person,
; so this probe is run alone, with a hard timeout, and its report is read
; only if it comes back.
acc := ""

try {
    r := InputBox("type something")
    acc := acc "InputBox=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "InputBox=ERR|" e.Message "`n"
}

try {
    r := FileSelect()
    acc := acc "FileSelect=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "FileSelect=ERR|" e.Message "`n"
}

try {
    r := DirSelect()
    acc := acc "DirSelect=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "DirSelect=ERR|" e.Message "`n"
}

FileAppend acc, "*"
