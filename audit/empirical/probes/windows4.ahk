; Probe: the windows family with the parameters left out. v2 lets the
; title be left out, and then the active window is the one asked about;
; the message of windows.ahk said "only the active one is", so that is what
; is asked here.
acc := ""

try {
    r := WinExist()
    acc := acc "WinExist.noargs=OK|[" r "]`n"
} catch as e {
    acc := acc "WinExist.noargs=ERR|" e.Message "`n"
}
try {
    r := WinExist("A")
    acc := acc "WinExist.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinExist.A=ERR|" e.Message "`n"
}
try {
    r := WinActive()
    acc := acc "WinActive.noargs=OK|[" r "]`n"
} catch as e {
    acc := acc "WinActive.noargs=ERR|" e.Message "`n"
}
try {
    r := WinGetTitle()
    acc := acc "WinGetTitle.noargs=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetTitle.noargs=ERR|" e.Message "`n"
}
try {
    r := WinGetText()
    acc := acc "WinGetText.noargs=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetText.noargs=ERR|" e.Message "`n"
}
try {
    r := WinGetPos(&a, &b, &c, &d)
    acc := acc "WinGetPos.noargs=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetPos.noargs=ERR|" e.Message "`n"
}

FileAppend acc, "*"
