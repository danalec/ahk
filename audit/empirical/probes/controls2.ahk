; Probe: controls family, round two, with the VarRef shape that v2 gives
; ControlGetPos, against the active window, read-only.
acc := ""

try {
    r := ControlGetPos(&cx, &cy, &cw, &ch, "Edit1", "A")
    acc := acc "ControlGetPos.A=OK|ret:[" r "]|x:" cx "|y:" cy "`n"
} catch as e {
    acc := acc "ControlGetPos.A=ERR|" e.Message "`n"
}
try {
    r := ControlGetText("Edit1", "A")
    acc := acc "ControlGetText.A=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlGetText.A=ERR|" e.Message "`n"
}
try {
    r := ControlGetFocus("A")
    acc := acc "ControlGetFocus.A=OK|[" r "]`n"
} catch as e {
    acc := acc "ControlGetFocus.A=ERR|" e.Message "`n"
}

FileAppend acc, "*"
