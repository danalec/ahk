; The loop of the `while`.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

i := 0
t0 := A_TickCount
while i < 3000000
    i += 1
t1 := A_TickCount
FileAppend("while_loop " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
