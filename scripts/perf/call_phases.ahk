; The phases of a call: what the call of a method costs and what the
; call of a function costs.
;
; The time of each phase is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there. Every phase has the same
; number of turns, so the difference between two phases is the
; difference between the two works.
;
; The phase of the loop alone is the floor: it is what the loop costs
; without a call in it.
;
; Every call gives a value to the body of it: a method that returns a
; constant is folded away by the official, and the phase would measure
; nothing at all.
#Requires AutoHotkey v2.0

class Counter {
    One(n) {
        return n
    }
    Two(n, m) {
        return n + m
    }
}

f(n) {
    return n + 1
}

c := Counter()
n := 1000000
i := 0

t0 := A_TickCount
Loop n {
    i := 1
}
t1 := A_TickCount
FileAppend("phase_loop_alone " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := c.One(1)
}
t1 := A_TickCount
FileAppend("phase_method_one " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := c.Two(1, 2)
}
t1 := A_TickCount
FileAppend("phase_method_two " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := f(1)
}
t1 := A_TickCount
FileAppend("phase_func_one " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
