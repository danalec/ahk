; Probe: the keyboard and the pointer of the machine, with the input side
; kept harmless. Send is given the empty string, so even a desktop that does
; carry the keyboard has nothing to type; MouseGetPos only reads.
acc := ""

try {
    r := Send("")
    acc := acc "Send.empty=OK|[" r "]`n"
} catch as e {
    acc := acc "Send.empty=ERR|" e.Message "`n"
}
try {
    r := SendText("")
    acc := acc "SendText.empty=OK|[" r "]`n"
} catch as e {
    acc := acc "SendText.empty=ERR|" e.Message "`n"
}
try {
    r := MouseGetPos(&mx, &my)
    acc := acc "MouseGetPos=OK|[" r "]|x:" mx "|y:" my "`n"
} catch as e {
    acc := acc "MouseGetPos=ERR|" e.Message "`n"
}
try {
    r := KeyWait("a", "T0.2")
    acc := acc "KeyWait=OK|[" r "]`n"
} catch as e {
    acc := acc "KeyWait=ERR|" e.Message "`n"
}

FileAppend acc, "*"
