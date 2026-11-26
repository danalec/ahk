; the name that the script builds, measured against the tree
;
; The answer of this probe is the file `probe-dyn.txt` next to it: the
; read of a name that is blank, the read of a name that is there, and
; the write of a name that is blank.
;
; The v1 refuses the name that is blank with "This dynamic variable is
; blank." and the thread of the script ends with the refusal, so the
; answer is what came before the refusal: the file is the same under the
; two when the two agree, and it is longer under the one that goes on.

out := ""

a := ""
out .= "P1 before the read of a name that is blank`n"
FileAppend, %out%, %A_ScriptDir%\probe-dyn.txt
x := %a%
out .= "P2 the read of a name that is blank went on`n"

b := "c"
c := "the value of c"
y := %b%
out .= "P3 the read of a name that is there = " y "`n"

FileAppend, %out%, %A_ScriptDir%\probe-dyn.txt
ExitApp
