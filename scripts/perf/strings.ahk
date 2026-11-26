; The strings of a loop: the search and the length.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

hay := "the haystack of the workload of the strings of a loop"
i := 0
t0 := A_TickCount
Loop 500000 {
    i := InStr(hay, "b")
    i := StrLen(hay)
}
t1 := A_TickCount
FileAppend("strings " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
