; Probe: #Include with a file that is there. The runner resolves the
; include before the script is parsed, so the function of the included file
; is defined and the line of the call is a line of the whole.
#Include "lib-probe.ahk"
MsgBox greet("world")
