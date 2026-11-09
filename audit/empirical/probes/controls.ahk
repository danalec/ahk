; Probe: controls family. A window that is not there is named, so no
; control of the desktop is touched.
acc := ""
w := "zzz-no-such-window-zzz-42"

try {
    r := ControlGetText("Edit1", w)
    acc := acc "ControlGetText.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlGetText.absent=ERR|" e.Message "`n"
}
try {
    r := ControlSetText("new", "Edit1", w)
    acc := acc "ControlSetText.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlSetText.absent=ERR|" e.Message "`n"
}
try {
    r := ControlGetPos("Edit1", w)
    acc := acc "ControlGetPos.absent=OK|type:" Type(r) "`n"
} catch as e {
    acc := acc "ControlGetPos.absent=ERR|" e.Message "`n"
}
try {
    r := ControlClick("Edit1", w)
    acc := acc "ControlClick.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlClick.absent=ERR|" e.Message "`n"
}
try {
    r := ControlSend("hello", "Edit1", w)
    acc := acc "ControlSend.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlSend.absent=ERR|" e.Message "`n"
}
try {
    r := ControlFocus("Edit1", w)
    acc := acc "ControlFocus.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlFocus.absent=ERR|" e.Message "`n"
}
try {
    r := ControlGetFocus(w)
    acc := acc "ControlGetFocus.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlGetFocus.absent=ERR|" e.Message "`n"
}
try {
    r := ControlMove(10, 10, 50, 20, "Edit1", w)
    acc := acc "ControlMove.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlMove.absent=ERR|" e.Message "`n"
}

FileAppend acc, "*"
