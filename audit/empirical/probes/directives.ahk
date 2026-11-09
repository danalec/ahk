#Requires AutoHotkey v2.0
#SingleInstance Force
#NoTrayIcon
#Warn
#MaxThreadsPerHotkey 1
#MaxThreads 2
#MaxThreadsBuffer Off
#Hotstring EndChars -()[]{}:;'"/\,.?!`n`,
#HotIf WinActive("zzz-no-such-window")
#HotIf

; Probe: the directives of v2 that a script may write at the top. If a
; directive is not carried, the load fails before the first statement and
; the report says which one and why.
acc := "directives loaded`n"
FileAppend acc, "*"
