; the window and the control that are under the pointer, measured
; against the tree
;
; The answer of this probe is the file `probe-pointer.txt` next to it:
; the window of the top level that is under the pointer, and the control
; that is under it.
;
; The numbers of the machine -- the handles -- are not printed: they are
; the machine's and they are not the same under the v1 and under the
; tree. What is printed is the behaviour.
;
; The button is made as big as the window, and the pointer is put over
; it, at a fixed distance from the corner of the window: the place of
; the window is read with `WinGetPos`, because the v1 puts the window
; where the monitor of the pointer is, and not where the script says.
;
; The pointer of the machine is moved by a person, and so the probe
; puts it again and again until it stays where it was put, and it says
; in the answer when it could not stay. The pointer of the machine is
; put back where it was at the end.

MouseGetPos, ox, oy

Gui, ProbeGui: +ToolWindow +hwndMyHwnd
Gui, ProbeGui: Add, Button, w280 h180, Btn
Gui, ProbeGui: Show, x100 y100 w300 h200
Sleep, 300

out := ""
WinGetPos, wx, wy, ww, wh, ahk_id %MyHwnd%
if (wx = "") {
   out .= "P0 the window of the script was not found`n"
} else {
   tx := wx + 150
   ty := wy + 100
   stable := "NO"
   Loop 40 {
      MouseMove, %tx%, %ty%
      MouseGetPos, mx, my, w, c
      if (mx = tx && my = ty) {
         stable := "YES"
         break
      }
      Sleep, 30
   }
   out .= "P1 the pointer stayed where it was put = " stable "`n"
   out .= "P2 the window under the pointer is the window of the script = " (w = MyHwnd ? "YES" : "NO") "`n"
   out .= "P3 the window under the pointer is not empty = " (w != "" ? "YES" : "NO") "`n"
   out .= "P4 the control under the pointer is the button = " (c = "Button1" ? "YES" : "NO") "`n"
}

FileAppend, %out%, %A_ScriptDir%\probe-pointer.txt
MouseMove, %ox%, %oy%
ExitApp
