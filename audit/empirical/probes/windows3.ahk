; Probe: windows family, round two. The calls are written with the shape
; that v2 gives them -- the position of a window is written into VarRefs --
; and a real window ("A", the active one) is asked about, read-only.
acc := ""

try {
    r := WinGetPos(&wx, &wy, &ww, &wh, "A")
    acc := acc "WinGetPos.A=OK|ret:[" r "]|x:" wx "|y:" wy "|w:" ww "|h:" wh "`n"
} catch as e {
    acc := acc "WinGetPos.A=ERR|" e.Message "`n"
}
try {
    r := WinGetPos(&wx, &wy, &ww, &wh, "zzz-no-such-window-zzz")
    acc := acc "WinGetPos.absent=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "WinGetPos.absent=ERR|" e.Message "`n"
}
try {
    r := WinGetTitle("A")
    acc := acc "WinGetTitle.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetTitle.A=ERR|" e.Message "`n"
}
try {
    r := WinGetText("A")
    acc := acc "WinGetText.A=OK|[" SubStr(r, 1, 30) "]`n"
} catch as e {
    acc := acc "WinGetText.A=ERR|" e.Message "`n"
}
try {
    r := WinGetClass("A")
    acc := acc "WinGetClass.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetClass.A=ERR|" e.Message "`n"
}
try {
    r := WinGetPID("A")
    acc := acc "WinGetPID.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetPID.A=ERR|" e.Message "`n"
}
try {
    r := WinGetProcess("A")
    acc := acc "WinGetProcess.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinGetProcess.A=ERR|" e.Message "`n"
}
try {
    r := WinActive("A")
    acc := acc "WinActive.A=OK|[" r "]`n"
} catch as e {
    acc := acc "WinActive.A=ERR|" e.Message "`n"
}

FileAppend acc, "*"
