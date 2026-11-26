; The property of an object, in a loop.
;
; The time of the loop is written to `perf-out.txt`, in the folder of
; the script: the harness reads it from there.
#Requires AutoHotkey v2.0

class Pair {
    __New() {
        this.a := 1
        this.b := 2
    }
}

o := Pair()
t0 := A_TickCount
Loop 500000 {
    o.a := o.a + o.b
}
t1 := A_TickCount
FileAppend("obj_prop " (t1 - t0) "`n", A_ScriptDir "\perf-out.txt")
