#Requires AutoHotkey v2.0
; Minimal differential probe of AutoHotkey v2.0.19: only the questions that
; cannot crash or block the binary are asked, so the answers may be compared
; with the ones the tree gave. Every statement is inside a try.
f := A_Temp "\ahk-diff3-out.txt"
out := "A_AhkVersion=[" A_AhkVersion "]`n"
out .= "#Requires-v2.0=loaded`n"

try
    out .= "A_ScreenWidth=[" A_ScreenWidth "]`n"
catch as e
    out .= "A_ScreenWidth=ERR [" e.Message "]`n"

try
    out .= "A_OSVersion=[" A_OSVersion "]`n"
catch as e
    out .= "A_OSVersion=ERR [" e.Message "]`n"

try
    out .= "A_ComputerName=[" A_ComputerName "]`n"
catch as e
    out .= "A_ComputerName=ERR [" e.Message "]`n"

try
    out .= "A_UserName=[" A_UserName "]`n"
catch as e
    out .= "A_UserName=ERR [" e.Message "]`n"

try
    out .= "Run=[" Run("cmd /c exit 0") "]`n"
catch as e
    out .= "Run=ERR [" e.Message "]`n"

try
    out .= "DateAdd.Days=[" DateAdd("20260102030405", 10, "Days") "]`n"
catch as e
    out .= "DateAdd.Days=ERR [" e.Message "]`n"

try
    out .= "WinGetTitle=[" WinGetTitle() "]`n"
catch as e
    out .= "WinGetTitle=ERR [" e.Message "]`n"

try
    out .= "A_Args=[" Type(A_Args) "]`n"
catch as e
    out .= "A_Args=ERR [" e.Message "]`n"

FileAppend out, f
