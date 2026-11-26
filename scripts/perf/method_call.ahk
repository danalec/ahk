; The call of a method of a class, in a loop.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

class Counter {
    Inc(n) {
        return n + 1
    }
}

c := Counter()
i := 0
t0 := A_TickCount
Loop 300000 {
    i := c.Inc(1)
}
t1 := A_TickCount
FileAppend("method_call " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
