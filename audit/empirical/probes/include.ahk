; Probe: #Include, which the runner resolves and not the language. The
; include names a file that is not there, so the report is the reduced one
; and the run is not a run. Kept alone for that reason.
#Include "ahk-probe-no-such-include.ahk"
MsgBox "never reached"
