#Requires AutoHotkey v2.0
; Differential probe against the reference binary of AutoHotkey v2.0.19:
; the same questions the tree was asked, asked of the dialect, so that a
; value and a refusal can be compared with what v2 actually answers. Every
; statement is inside a try, because an uncaught failure of this binary
; opens a dialog and would wait for a person. Nothing of the desktop is
; driven: the answers go to a file in the temp folder.
f := A_Temp "\ahk-diff2-out.txt"
out := "A_AhkVersion=[" A_AhkVersion "]`n"
out .= "#Requires-v2.0=loaded`n"

try
    out .= "Run=[" Run("cmd /c exit 0") "]`n"
catch as e
    out .= "Run=ERR [" e.Message "]`n"

try
    out .= "DateAdd.Days=[" DateAdd("20260102030405", 10, "Days") "]`n"
catch as e
    out .= "DateAdd.Days=ERR [" e.Message "]`n"

try
    out .= "A_ScreenWidth=[" A_ScreenWidth "]`n"
catch as e
    out .= "A_ScreenWidth=ERR [" e.Message "]`n"

try
    out .= "SysGet80=[" SysGet(80) "]`n"
catch as e
    out .= "SysGet80=ERR [" e.Message "]`n"

try
    out .= "A_ComputerName=[" A_ComputerName "]`n"
catch as e
    out .= "A_ComputerName=ERR [" e.Message "]`n"

try
    out .= "A_UserName=[" A_UserName "]`n"
catch as e
    out .= "A_UserName=ERR [" e.Message "]`n"

try
    out .= "A_OSVersion=[" A_OSVersion "]`n"
catch as e
    out .= "A_OSVersion=ERR [" e.Message "]`n"

try
    out .= "ProcessExist-ahk=[" ProcessExist("ahk.exe") "]`n"
catch as e
    out .= "ProcessExist-ahk=ERR [" e.Message "]`n"

try
    out .= "WinExist-A=[" WinExist("A") "]`n"
catch as e
    out .= "WinExist-A=ERR [" e.Message "]`n"

try
    out .= "WinGetTitle=[" WinGetTitle() "]`n"
catch as e
    out .= "WinGetTitle=ERR [" e.Message "]`n"

try
    out .= "WinGetList-count=[" WinGetList().Length "]`n"
catch as e
    out .= "WinGetList-count=ERR [" e.Message "]`n"

try
    out .= "SoundGetVolume=[" SoundGetVolume() "]`n"
catch as e
    out .= "SoundGetVolume=ERR [" e.Message "]`n"

try
    out .= "A_Clipboard=[" SubStr(A_Clipboard, 1, 20) "]`n"
catch as e
    out .= "A_Clipboard=ERR [" e.Message "]`n"

try
    out .= "FileCopy-missing=ERR [" FileCopy(A_Temp "\ahk-diff-nonexistent.txt", A_Temp "\ahk-diff-x.txt") "]`n"
catch as e
    out .= "FileCopy-missing=THROW [" e.Message "] what:[" e.What "]`n"

try
    out .= "FileReadLine=[" SubStr(FileReadLine("C:\Windows\win.ini", 1), 1, 20) "]`n"
catch as e
    out .= "FileReadLine=ERR [" e.Message "]`n"

try
    m := []
    out .= "RegExMatch-matchvar=[" RegExMatch("hello 42", "(\d+)", &m) "] m1=[" m[1] "]`n"
catch as e
    out .= "RegExMatch-matchvar=ERR [" e.Message "]`n"

try
    out .= "Sort-options=[" StrReplace(Sort("c`na`nb", "D`n"), "`n", ",") "]`n"
catch as e
    out .= "Sort-options=ERR [" e.Message "]`n"

try
    h := Hotstring("::btw::", "by the way")
    out .= "Hotstring=[" Type(h) "]`n"
catch as e
    out .= "Hotstring=ERR [" e.Message "]`n"

try
    out .= "ToolTip=[" ToolTip("x") "]`n"
catch as e
    out .= "ToolTip=ERR [" e.Message "]`n"

try
    out .= "ImageSearch=[" ImageSearch(&ix, &iy, 0, 0, 100, 100, "C:\Windows\win.ini") "]`n"
catch as e
    out .= "ImageSearch=THROW class:[" Type(e) "] msg:[" e.Message "] what:[" e.What "]`n"

try
    out .= "ComObject=[" Type(ComObject("Shell.Application")) "]`n"
catch as e
    out .= "ComObject=ERR [" e.Message "]`n"

try
    out .= "DllCall=[" DllCall("kernel32.dll\GetCurrentProcessId", "UInt") "]`n"
catch as e
    out .= "DllCall=ERR [" e.Message "]`n"

try
    out .= "A_Args=[" Type(A_Args) "]`n"
catch as e
    out .= "A_Args=ERR [" e.Message "]`n"

try
    out .= "ClipWait=[" ClipWait(0.1, 0) "]`n"
catch as e
    out .= "ClipWait=ERR [" e.Message "]`n"

FileAppend out, f
