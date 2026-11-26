; The `NumPut` and the `NumGet` of a loop.
;
; The type of a number of the v2 is a name and not the number of it:
; the `NumPut("Int64", ...)` is accepted by the official v2.0.32, and
; the `NumPut("U64", ...)` is refused by the two with the same words.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

buf := Buffer(64)
i := 0
t0 := A_TickCount
Loop 1000000 {
    NumPut("Int64", i, buf, 0)
    i := NumGet(buf, 0, "Int64")
}
t1 := A_TickCount
FileAppend("numput_numget " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
