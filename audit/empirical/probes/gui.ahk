; Probe: GUI family, the part that is safe to call without a user in front
; of the screen. The dialogs are probed in dialogs.ahk, which may block.
acc := ""

try {
    g := Gui("+AlwaysOnTop", "ahk probe window")
    acc := acc "Gui.call=OK|type:" Type(g) "`n"
    try {
        r := g.Show()
        acc := acc "Gui.Show=OK|[" r "]`n"
    } catch as e {
        acc := acc "Gui.Show=ERR|" e.Message "`n"
    }
    try {
        r := g.Add("Text", , "hello")
        acc := acc "Gui.Add=OK|type:" Type(r) "`n"
    } catch as e {
        acc := acc "Gui.Add=ERR|" e.Message "`n"
    }
    try {
        r := g.Destroy()
        acc := acc "Gui.Destroy=OK|[" r "]`n"
    } catch as e {
        acc := acc "Gui.Destroy=ERR|" e.Message "`n"
    }
} catch as e {
    acc := acc "Gui.call=ERR|" e.Message "`n"
}

try {
    m := Menu()
    acc := acc "Menu.call=OK|type:" Type(m) "`n"
} catch as e {
    acc := acc "Menu.call=ERR|" e.Message "`n"
}

try {
    r := MsgBox("a message from the probe")
    acc := acc "MsgBox=OK|[" r "]`n"
} catch as e {
    acc := acc "MsgBox=ERR|" e.Message "`n"
}

try {
    r := ToolTip("a tip")
    acc := acc "ToolTip=OK|[" r "]`n"
} catch as e {
    acc := acc "ToolTip=ERR|" e.Message "`n"
}
try {
    r := TrayTip("a tray tip", "title")
    acc := acc "TrayTip=OK|[" r "]`n"
} catch as e {
    acc := acc "TrayTip=ERR|" e.Message "`n"
}

FileAppend acc, "*"
