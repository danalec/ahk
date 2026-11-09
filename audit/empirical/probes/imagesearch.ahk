; Probe: ImageSearch, and the shape of the failure it gives. In screen.ahk
; the call failed with an empty message, which is the thing this probe
; pins down: the class, the What, the Extra and the length of the message.
acc := ""

try {
    r := ImageSearch(&ix, &iy, 0, 0, 100, 100, "C:\Windows\win.ini")
    acc := acc "ImageSearch=OK|[" r "]`n"
} catch as e {
    acc := acc "ImageSearch=ERR|class:" Type(e) "|what:[" e.What "]|extra:[" e.Extra "]|len:" StrLen(e.Message) "|msg:[" e.Message "]`n"
}

FileAppend acc, "*"
