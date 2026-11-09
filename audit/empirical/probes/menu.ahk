; Probe: the Menu class. gui.ahk showed that Menu() constructs; this probe
; asks what the menu can be asked to do. No window and no tray is touched:
; the calls are about the menu object the script makes.
acc := ""

try {
    m := Menu()
    acc := acc "Menu.call=OK|type:" Type(m) "`n"
    try {
        r := m.Add("item-one", menuHandler)
        acc := acc "Menu.Add=OK|[" r "]`n"
    } catch as e {
        acc := acc "Menu.Add=ERR|" e.Message "`n"
    }
    try {
        r := m.Show()
        acc := acc "Menu.Show=ERR|[" r "]`n"
    } catch as e {
        acc := acc "Menu.Show=ERR|" e.Message "`n"
    }
    try {
        r := m.Delete("item-one")
        acc := acc "Menu.Delete=OK|[" r "]`n"
    } catch as e {
        acc := acc "Menu.Delete=ERR|" e.Message "`n"
    }
} catch as e {
    acc := acc "Menu.call=ERR|" e.Message "`n"
}

menuHandler(*) {
    return
}

FileAppend acc, "*"
