; The native call that takes a variable with content.
;
; Measured on the binary of AutoHotkey v1.1.37.02: the
; `DllCall("kernel32\lstrlenA", "Str", s)` of a script whose `s` holds
; "hello" answers one -- the length of the ANSI text of the buffer of the
; variable -- and the call is not refused. The variable that has content of
; its own is not the one that needs a `VarSetCapacity`.
;
; The answer of the probe is the length that the call answers: it is the
; same in the v1 and in the tree.

out := ""
s := "hello"
out .= "the length of the ANSI text = " DllCall("kernel32\lstrlenA", "Str", s) "`n"
out .= "the length of the wide text = " DllCall("kernel32\lstrlenW", "WStr", s) "`n"
FileAppend, %out%, %A_ScriptDir%\probe-dllstr.txt
ExitApp
