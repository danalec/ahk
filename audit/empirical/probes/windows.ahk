; Probe: windows family, the read-only half. A window that is not there is
; asked about, so nothing of the desktop is touched; the questions are
; caught one by one and their exact answer is kept.
acc := ""
nonexistent := "zzz-no-such-window-zzz-42"

try {
    r := WinExist(nonexistent)
    acc := acc "WinExist.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinExist.absent=ERR|" e.Message "`n"
}
try {
    r := WinActive(nonexistent)
    acc := acc "WinActive.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinActive.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetTitle(nonexistent)
    acc := acc "WinGetTitle.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetTitle.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetText(nonexistent)
    acc := acc "WinGetText.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetText.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetPos(nonexistent)
    acc := acc "WinGetPos.absent=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "WinGetPos.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetList()
    acc := acc "WinGetList=OK|type:" Type(r) "|count:" r.Length "`n"
} catch as e {
    acc := acc "WinGetList=ERR|" e.Message "`n"
}
try {
    r := WinGetClass(nonexistent)
    acc := acc "WinGetClass.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetClass.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetProcess(nonexistent)
    acc := acc "WinGetProcess.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetProcess.absent=ERR|" e.Message "`n"
}
try {
    r := WinExist("A")
    acc := acc "WinExist.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinExist.A=ERR|" e.Message "`n"
}

FileAppend acc, "*"
