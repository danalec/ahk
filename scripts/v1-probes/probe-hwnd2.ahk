; the native call takes the value of a built-in variable
out := ""
h := A_ScriptHwnd
out .= "with the variable = " DllCall("user32\IsWindow", "Ptr", h) "`n"
out .= "with the built-in = " DllCall("user32\IsWindow", "Ptr", A_ScriptHwnd) "`n"
thread := DllCall("user32\GetWindowThreadProcessId", "Ptr", A_ScriptHwnd, "Ptr", 0)
answer := "NO"
if (thread)
   answer := "YES"
out .= "the thread of the built-in is there = " answer "`n"
thread2 := DllCall("user32\GetWindowThreadProcessId", "Ptr", h, "Ptr", 0)
same := "NO"
if (thread2 = thread)
   same := "YES"
out .= "the thread of the variable is the same = " same "`n"
FileAppend, %out%, %A_ScriptDir%\probe-hwnd2.txt
ExitApp
