#Requires AutoHotkey v2.0
; Differential probe, round two: the questions whose answers decide whether
; a tree answer is a refusal of a platform or a difference from the
; dialect. Only names that v2.0.19 has are asked, and every statement is
; inside a try so a failure cannot open a dialog.
f := A_Temp "\ahk-diff4-out.txt"
out := ""

try
    out .= "Run-longer=[" Run("cmd /c ping -n 3 127.0.0.1 > nul") "]`n"
catch as e
    out .= "Run-longer=ERR [" e.Message "]`n"

try
    out .= "DateDiff.Days=[" DateDiff("20260102030405", "20260202030405", "Days") "]`n"
catch as e
    out .= "DateDiff.Days=ERR [" e.Message "]`n"

try
    m := []
    out .= "RegExMatch.matchvar=[" RegExMatch("hello 42", "(\d+)", &m) "] m1=[" m[1] "]`n"
catch as e
    out .= "RegExMatch.matchvar=ERR [" e.Message "]`n"

try
    out .= "Sort.options=[" StrReplace(Sort("c`na`nb", "D`n"), "`n", ",") "]`n"
catch as e
    out .= "Sort.options=ERR [" e.Message "]`n"

try
    h := Hotstring("::btw::", "by the way")
    out .= "Hotstring=[" Type(h) "]`n"
catch as e
    out .= "Hotstring=ERR [" e.Message "]`n"

try
    out .= "ProcessExist=[" ProcessExist("explorer.exe") "]`n"
catch as e
    out .= "ProcessExist=ERR [" e.Message "]`n"

try
    out .= "SysGet80=[" SysGet(80) "]`n"
catch as e
    out .= "SysGet80=ERR [" e.Message "]`n"

try
    out .= "SoundGetVolume=[" SoundGetVolume() "]`n"
catch as e
    out .= "SoundGetVolume=ERR [" e.Message "]`n"

try
    out .= "ClipWait=[" ClipWait(0.1, 0) "]`n"
catch as e
    out .= "ClipWait=ERR [" e.Message "]`n"

try
    out .= "WinGetList.count=[" WinGetList().Length "]`n"
catch as e
    out .= "WinGetList.count=ERR [" e.Message "]`n"

FileAppend out, f
