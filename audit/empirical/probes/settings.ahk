; Probe: the settings of the script that the hotkey engine owns
; (Suspend, Pause, Persistent) and the warm-up of the main thread. Nothing
; here types or clicks.
acc := ""

try {
    r := Suspend(1)
    acc := acc "Suspend.on=OK|[" r "]`n"
} catch as e {
    acc := acc "Suspend.on=ERR|" e.Message "`n"
}
try {
    r := Suspend(0)
    acc := acc "Suspend.off=OK|[" r "]`n"
} catch as e {
    acc := acc "Suspend.off=ERR|" e.Message "`n"
}
try {
    r := Pause(1)
    acc := acc "Pause.on=OK|[" r "]`n"
} catch as e {
    acc := acc "Pause.on=ERR|" e.Message "`n"
}
try {
    r := Pause(0)
    acc := acc "Pause.off=OK|[" r "]`n"
} catch as e {
    acc := acc "Pause.off=ERR|" e.Message "`n"
}
try {
    r := Persistent(1)
    acc := acc "Persistent=OK|[" r "]`n"
} catch as e {
    acc := acc "Persistent=ERR|" e.Message "`n"
}

FileAppend acc, "*"
