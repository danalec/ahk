; the chain of the right click, measured against the tree
;
; The answer of this probe is the file `probe-chain.txt` next to it: the
; message that the script watches, the timer that it sets, and the menu
; that the timer builds.
;
; The menu is not shown: the show of a menu is a modal loop, and the
; probe would not end.

Build() {
   global out
   out .= "P3 the timer of the chain ran`n"
   Menu, PMenu, Add, &One, POne
   Menu, PMenu, Add, &Two, POne
   SetTimer, Report, -50
}

Report() {
   global out
   out .= "P4 the menu of the chain is built`n"
   count := DllCall("GetMenuItemCount", "Ptr", MenuGetHandle("PMenu"))
   out .= "P5 the menu of the chain has " count " items`n"
   FileAppend, %out%, %A_ScriptDir%\probe-chain.txt
   ExitApp
}

POne() {
   return
}

out := ""
OnMessage(0x205, "WM_RBUTTONUP")
out .= "P1 the script watches the message of the right click`n"
SetTimer, Build, -50
out .= "P2 the timer of the chain is set`n"
; the sleep holds the script while the timers of the chain run
Sleep, 600
