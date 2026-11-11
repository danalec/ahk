; The tray of the script: the icon goes into the tray of the session, and a
; click on the icon runs the item by default of the menu of the script.
;
; The click is not made here: the compositor of the box is asked for it from
; the outside, with the bus of the item. The script waits in small slices,
; because a long sleep swallows the events that happen inside it.

; The icon is a file of the machine: the icon is loaded before it is set, and
; a name of a themed icon is not a file that can be loaded.
TraySetIcon("/usr/share/icons/hicolor/256x256/apps/shelly-tray.png")

m := A_TrayMenu
; The callback of a menu item is called with (ItemName, ItemPos, MyMenu),
; and the binary refuses a callback that cannot take the three.
; The parenthesis of the call is not spaced: a spaced one is the
; concatenation of the name and the value, and the name of a built-in is
; a function in an expression.
m.Add("&Open", (ItemName, ItemPos, MyMenu) => MsgBox("clicked: " ItemName))

Loop 30 {
    Sleep 200
}
MsgBox "done"
