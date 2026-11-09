#Requires AutoHotkey v2.0
; Differential probe: the same questions asked of the reference binary of
; AutoHotkey v2.0.19 on this machine, so that a value the tree answers can
; be compared with the value the dialect gives. Nothing of the desktop is
; driven: the answers go to a file in the temp folder.
f := A_Temp "\ahk-diff1-out.txt"
FileDelete f
out := "A_AhkVersion=" A_AhkVersion "`n"
out .= "#Requires AutoHotkey v2.0 loaded`n"
try
    out .= "Run.return=" Run("cmd /c exit 0") "`n"
catch as e
    out .= "Run.return=ERR " e.Message "`n"
try
    out .= "DateAdd.Days=" DateAdd("20260102030405", 10, "Days") "`n"
catch as e
    out .= "DateAdd.Days=ERR " e.Message "`n"
try
    out .= "DateAdd.d=" DateAdd("20260102030405", 10, "d") "`n"
catch as e
    out .= "DateAdd.d=ERR " e.Message "`n"
try
    out .= "DateDiff.Days=" DateDiff("20260102030405", "20260202030405", "Days") "`n"
catch as e
    out .= "DateDiff.Days=ERR " e.Message "`n"
try
    out .= "A_ScreenWidth=" A_ScreenWidth "`n"
catch as e
    out .= "A_ScreenWidth=ERR " e.Message "`n"
try
    out .= "FileReadLine=" FileReadLine("C:\Windows\win.ini", 1) "`n"
catch as e
    out .= "FileReadLine=ERR " e.Message "`n"
try
    out .= "Sort.plain=" StrReplace(Sort("c`na`nb"), "`n", ",") "`n"
catch as e
    out .= "Sort.plain=ERR " e.Message "`n"
FileAppend out, f
