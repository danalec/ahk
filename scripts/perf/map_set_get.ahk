; The write in a map and the read of it, in a loop.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

m := Map()
i := 0
t0 := A_TickCount
Loop 1000000 {
    m[i] := i
    i := m[i]
    i += 1
}
t1 := A_TickCount
FileAppend("map_set_get " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
