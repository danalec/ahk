; The scope of the call: the same body, with one, two and three
; parameters.
;
; The time of each phase is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there. Every phase has the same
; number of turns, and the body of the three is the same -- the method
; returns its first parameter and does not read the others -- so the
; difference between two phases is the cost of a parameter in the scope
; of the call.
#Requires AutoHotkey v2.0

class Counter {
    One(n) {
        return n
    }
    Two(n, m) {
        return n
    }
    Three(n, m, o) {
        return n
    }
}

c := Counter()
n := 1000000
i := 0

t0 := A_TickCount
Loop n {
    i := c.One(1)
}
t1 := A_TickCount
FileAppend("phase_one_parameter " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := c.Two(1, 2)
}
t1 := A_TickCount
FileAppend("phase_two_parameters " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := c.Three(1, 2, 3)
}
t1 := A_TickCount
FileAppend("phase_three_parameters " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
