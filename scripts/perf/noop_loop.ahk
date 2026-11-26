; The loop that does nothing.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
;
; The clock of the machine counts in 15,6 milliseconds, so the loop has
; to be long enough for the number to say something: three million
; turns are about a hundred milliseconds.
#Requires AutoHotkey v2.0

t0 := A_TickCount
Loop 3000000 {
}
t1 := A_TickCount
FileAppend("noop_loop " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
