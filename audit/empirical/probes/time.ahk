; Probe: the time family of v2.
acc := ""

try {
    r := FormatTime(, "yyyy-MM-dd HH:mm:ss")
    acc := acc "FormatTime.now=OK|[" r "]`n"
} catch as e {
    acc := acc "FormatTime.now=ERR|" e.Message "`n"
}
try {
    r := FormatTime("20260102030405", "yyyy-MM-dd")
    acc := acc "FormatTime.given=OK|[" r "]`n"
} catch as e {
    acc := acc "FormatTime.given=ERR|" e.Message "`n"
}
try {
    r := DateAdd("20260102030405", 10, "Days")
    acc := acc "DateAdd=OK|[" r "]`n"
} catch as e {
    acc := acc "DateAdd=ERR|" e.Message "`n"
}
try {
    r := DateDiff("20260102030405", "20260202030405", "Days")
    acc := acc "DateDiff=OK|[" r "]`n"
} catch as e {
    acc := acc "DateDiff=ERR|" e.Message "`n"
}
try {
    r := A_Now
    acc := acc "A_Now=OK|[" r "]`n"
} catch as e {
    acc := acc "A_Now=ERR|" e.Message "`n"
}
try {
    r := A_Year
    acc := acc "A_Year=OK|[" r "]`n"
} catch as e {
    acc := acc "A_Year=ERR|" e.Message "`n"
}
try {
    a := A_TickCount
    Sleep 5
    b := A_TickCount
    acc := acc "A_TickCount=OK|[move:" (b - a) "]`n"
} catch as e {
    acc := acc "A_TickCount=ERR|" e.Message "`n"
}

FileAppend acc, "*"
