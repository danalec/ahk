; Probe: windows family, the mutating half. Every call names a window that
; is not there, so no real window of the desktop is activated, moved or
; closed. The waits are given a fraction of a second.
acc := ""
w := "zzz-no-such-window-zzz-42"

try {
    r := WinActivate(w)
    acc := acc "WinActivate.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinActivate.absent=ERR|" e.Message "`n"
}
try {
    r := WinMove(0, 0, 100, 100, w)
    acc := acc "WinMove.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinMove.absent=ERR|" e.Message "`n"
}
try {
    r := WinClose(w)
    acc := acc "WinClose.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinClose.absent=ERR|" e.Message "`n"
}
try {
    r := WinShow(w)
    acc := acc "WinShow.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinShow.absent=ERR|" e.Message "`n"
}
try {
    r := WinHide(w)
    acc := acc "WinHide.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinHide.absent=ERR|" e.Message "`n"
}
try {
    r := WinMinimize(w)
    acc := acc "WinMinimize.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinMinimize.absent=ERR|" e.Message "`n"
}
try {
    r := WinMaximize(w)
    acc := acc "WinMaximize.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinMaximize.absent=ERR|" e.Message "`n"
}
try {
    r := WinSetTitle("new title", w)
    acc := acc "WinSetTitle.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinSetTitle.absent=ERR|" e.Message "`n"
}
try {
    r := WinWait(w, , 0.2)
    acc := acc "WinWait.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinWait.absent=ERR|" e.Message "`n"
}
try {
    r := WinWaitActive(w, , 0.2)
    acc := acc "WinWaitActive.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinWaitActive.absent=ERR|" e.Message "`n"
}
try {
    r := WinWaitClose(w, , 0.2)
    acc := acc "WinWaitClose.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinWaitClose.absent=ERR|" e.Message "`n"
}
try {
    r := WinSetAlwaysOnTop(1, w)
    acc := acc "WinSetAlwaysOnTop.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "WinSetAlwaysOnTop.absent=ERR|" e.Message "`n"
}

FileAppend acc, "*"
