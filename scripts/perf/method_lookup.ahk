; The lookup of the method: a class with one method and a class with
; many.
;
; The time of each phase is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there. Every phase has the same
; number of turns, so the difference between two phases is the
; difference between the two works.
;
; The names of the methods of the class with many are the same length
; and the same letters, so that the comparison of the names of the
; lookup costs the same in every one of them.
#Requires AutoHotkey v2.0

class One {
    Call(n) {
        return n
    }
}

class Many {
    A1(n) {
        return n
    }
    A2(n) {
        return n
    }
    A3(n) {
        return n
    }
    A4(n) {
        return n
    }
    A5(n) {
        return n
    }
    A6(n) {
        return n
    }
    A7(n) {
        return n
    }
    A8(n) {
        return n
    }
    A9(n) {
        return n
    }
    B1(n) {
        return n
    }
    B2(n) {
        return n
    }
    B3(n) {
        return n
    }
    B4(n) {
        return n
    }
    B5(n) {
        return n
    }
    B6(n) {
        return n
    }
    B7(n) {
        return n
    }
    B8(n) {
        return n
    }
    B9(n) {
        return n
    }
}

the_one := One()
the_many := Many()
n := 1000000
i := 0

t0 := A_TickCount
Loop n {
    i := the_one.Call(1)
}
t1 := A_TickCount
FileAppend("phase_one_method " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")

t0 := A_TickCount
Loop n {
    i := the_many.B9(1)
}
t1 := A_TickCount
FileAppend("phase_many_methods " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
