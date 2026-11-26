; the built-in variables of the event of a control, measured against
; the tree
;
; The answer of this probe is the file `probe-guiev.txt` next to it: the
; `A_Gui`, the `A_GuiControl` and the `A_GuiEvent` before the event of
; a control, and the name of the control that the event carries.
;
; The numbers of the machine are not printed: what is printed is the
; behaviour.

out := ""

Gui, EvGui: +ToolWindow +hwndMyHwnd
Gui, EvGui: Add, Button, , Btn
Gui, EvGui: Show, x100 y100 w300 h200
Sleep, 200

empty := "YES"
if (A_Gui != "")
   empty := "NO"
out .= "P1 the A_Gui before the event of a control is empty = " empty "`n"

empty := "YES"
if (A_GuiControl != "")
   empty := "NO"
out .= "P2 the A_GuiControl before the event of a control is empty = " empty "`n"

empty := "YES"
if (A_GuiEvent != "")
   empty := "NO"
out .= "P3 the A_GuiEvent before the event of a control is empty = " empty "`n"

txt := ""
ControlGetText, txt, Button1, ahk_id %MyHwnd%
out .= "P4 the text of the control of the window = " (txt != "" ? "YES" : "NO") "`n"

FileAppend, %out%, %A_ScriptDir%\probe-guiev.txt
ExitApp
