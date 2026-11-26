; The loop of the registry of a key that is not there.
;
; Measured on the binary of AutoHotkey v1.1.37.02 (`source/script.cpp`,
; the `PerformLoopReg`): the `RegOpenKeyEx` that does not answer
; `ERROR_SUCCESS` makes the v1 return `OK` -- the loop does not run, and
; the script goes on.
;
; The answer of the probe is the number of turns of the loop: it is zero
; in the v1 and in the tree.

count := 0
Loop, Reg, HKCU\Software\NoSuchKeyOfTheProbe, KV
{
    count += 1
}
FileAppend, %count%, *
