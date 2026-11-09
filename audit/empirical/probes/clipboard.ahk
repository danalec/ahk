; Probe: clipboard family (v2 names)
; Platform of the run is reported by the notes; part of this family is
; platform-dependent, so every call is caught and its exact message kept.
acc := ""

; --- A_Clipboard, the v2 name of the clipboard text -------------------
try {
    acc := acc "A_Clipboard.read=OK|[" A_Clipboard "]`n"
} catch as e {
    acc := acc "A_Clipboard.read=ERR|" e.Message "`n"
}
try {
    A_Clipboard := "ahk-probe-clip-42"
    acc := acc "A_Clipboard.write=OK|readback:[" A_Clipboard "]`n"
} catch as e {
    acc := acc "A_Clipboard.write=ERR|" e.Message "`n"
}

; --- ClipboardAll, the v2 class of the whole clipboard ----------------
try {
    ca := ClipboardAll()
    acc := acc "ClipboardAll.call=OK|type:" Type(ca) "`n"
} catch as e {
    acc := acc "ClipboardAll.call=ERR|" e.Message "`n"
}
try {
    n := ClipboardAll().Size
    acc := acc "ClipboardAll.Size=OK|" n "`n"
} catch as e {
    acc := acc "ClipboardAll.Size=ERR|" e.Message "`n"
}

; --- ClipWait ---------------------------------------------------------
try {
    r := ClipWait(0.1, 0)
    acc := acc "ClipWait=OK|" r "`n"
} catch as e {
    acc := acc "ClipWait=ERR|" e.Message "`n"
}

; --- OnClipboardChange ------------------------------------------------
try {
    r := OnClipboardChange(clipcb, 1)
    acc := acc "OnClipboardChange=OK|" r "`n"
} catch as e {
    acc := acc "OnClipboardChange=ERR|" e.Message "`n"
}

; --- the v1 name Clipboard, which v2 does not have --------------------
try {
    x := Clipboard
    acc := acc "Clipboard.v1name=OK|[" x "]`n"
} catch as e {
    acc := acc "Clipboard.v1name=ERR|" e.Message "`n"
}

clipcb(kind) {
    return kind
}

FileAppend acc, "*"
