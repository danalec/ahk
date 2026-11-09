; Probe: the time family, round two. The unit of DateAdd and DateDiff is
; asked in the several shapes v2 accepts, so a refusal says which shape the
; tree reads and which it does not.
acc := ""

try {
    r := DateAdd("20260102030405", 10, "Days")
    acc := acc "DateAdd.string-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateAdd.string-unit=ERR|" e.Message "`n"
}
try {
    r := DateAdd("20260102030405", 10, 1)
    acc := acc "DateAdd.number-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateAdd.number-unit=ERR|" e.Message "`n"
}
try {
    r := DateAdd("20260102030405", 10, "d")
    acc := acc "DateAdd.short-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateAdd.short-unit=ERR|" e.Message "`n"
}
try {
    r := DateAdd("20260102030405", 10, "seconds")
    acc := acc "DateAdd.long-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateAdd.long-unit=ERR|" e.Message "`n"
}
try {
    r := DateDiff("20260102030405", "20260202030405", "Days")
    acc := acc "DateDiff.string-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateDiff.string-unit=ERR|" e.Message "`n"
}
try {
    r := DateDiff("20260102030405", "20260202030405", 1)
    acc := acc "DateDiff.number-unit=OK|[" r "]`n"
} catch as e {
    acc := acc "DateDiff.number-unit=ERR|" e.Message "`n"
}

FileAppend acc, "*"
