; the timers of the v1: a positive period repeats, a negative one runs once
out := ""
count := 0
SetTimer, TheRepeating, 100
SetTimer, TheOnce, -100
TheRepeating() {
   global count := count + 1
}
TheOnce() {
   global once := once + 1
}
once := 0
Sleep, 600
out .= "the repeating timer ran " count " times`n"
out .= "the one-shot timer ran " once " times`n"
FileAppend, %out%, %A_ScriptDir%\probe-timer.txt
ExitApp
