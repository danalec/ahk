; Probe: the output of a script -- FileAppend to the console of the machine
; (the "*" file) and the MsgBox, which the runner puts in the report's
; output instead of showing a window.
acc := ""

FileAppend "from-FileAppend-star`n", "*"
MsgBox "from-MsgBox"
FileAppend "second-star-line`n", "*"

try {
    r := FileAppend("explicit-star`n", "*")
    acc := acc "FileAppend.star=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "FileAppend.star=ERR|" e.Message "`n"
}

try {
    f := FileOpen("*", "w")
    acc := acc "FileOpen.star=OK|type:" Type(f) "`n"
    try {
        r := f.Write("via-FileOpen-star`n")
        acc := acc "FileOpen.star.Write=OK|[" r "]`n"
    } catch as e2 {
        acc := acc "FileOpen.star.Write=ERR|" e2.Message "`n"
    }
    try {
        f.Close()
    } catch as e2 {
        acc := acc "FileOpen.star.Close=ERR|" e2.Message "`n"
    }
} catch as e {
    acc := acc "FileOpen.star=ERR|" e.Message "`n"
}

MsgBox "last"

FileAppend acc, "*"
