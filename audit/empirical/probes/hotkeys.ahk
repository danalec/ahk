; Probe: the hotkeys of v2. The registration of a hotkey is checked before
; the first statement runs, so this probe is kept on its own: if the
; registration is refused, there are no steps and no output, and the
; report says why. Nothing is typed, so no key of the desktop is driven.
acc := ""
try {
    r := Hotkey("#j", hk)
    acc := acc "Hotkey=OK|[" r "]`n"
} catch as e {
    acc := acc "Hotkey=ERR|" e.Message "`n"
}
try {
    r := Hotkey("#j", "Off")
    acc := acc "Hotkey.off=OK|[" r "]`n"
} catch as e {
    acc := acc "Hotkey.off=ERR|" e.Message "`n"
}

hk(*) {
    return
}

FileAppend acc, "*"
