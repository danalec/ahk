; the icon of the window of the GUI, measured against the tree
;
; The answer of this probe is the file `probe-icon.txt` next to it: the
; icon of the window of the GUI.
;
; The icon of the window of the script is not in the answer: the tree
; has no tray and no icon in the window of the script, and that is the
; machine of the tree and not a difference of the behaviour.
;
; The tray of the script is not set either: the v1 stops the script when
; the icon of the tray cannot be loaded, and the probe would answer
; nothing at all.

out := ""
Gui, +hwndPVhwnd
small := DllCall("user32\SendMessageW", "Ptr", PVhwnd, "UInt", 0x7F, "UInt", 0, "UInt", 0)
big := DllCall("user32\SendMessageW", "Ptr", PVhwnd, "UInt", 0x7F, "UInt", 1, "UInt", 0)
out .= "the gui window has an icon = " (small || big ? "YES" : "NO") "`n"
FileAppend, %out%, %A_ScriptDir%\probe-icon.txt
ExitApp
