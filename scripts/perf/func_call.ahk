; The call of a function of the script, in a loop.
;
; The function takes an argument and gives it back changed: a function
; that returns a constant is folded away by the official, and the
; measurement would be of nothing.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

f(n) {
    return n + 1
}

i := 0
t0 := A_TickCount
Loop 500000 {
    i := f(i)
}
t1 := A_TickCount
FileAppend("func_call " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
