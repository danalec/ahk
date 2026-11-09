; Probe: the unit of DateAdd and DateDiff, one letter at a time. v2
; documents the units as the words "Seconds", "Minutes", "Hours" and
; "Days"; only the one-letter form was accepted in date2.ahk, so each
; letter is asked here to see which ones the tree reads.
acc := ""
units := ["s", "m", "h", "d", "S", "D", "Seconds", "Minutes", "Hours", "Days"]

for u in units {
    try {
        r := DateAdd("20260102030405", 10, u)
        acc := acc "DateAdd[" u "]=OK|[" r "]`n"
    } catch as e {
        acc := acc "DateAdd[" u "]=ERR|" e.Message "`n"
    }
}
for u in units {
    try {
        r := DateDiff("20260102030405", "20260202030405", u)
        acc := acc "DateDiff[" u "]=OK|[" r "]`n"
    } catch as e {
        acc := acc "DateDiff[" u "]=ERR|" e.Message "`n"
    }
}

FileAppend acc, "*"
