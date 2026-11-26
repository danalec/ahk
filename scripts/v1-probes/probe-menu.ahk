; the menu of the v1, measured against the tree
;
; The answer of this probe is the file `probe-menu.txt` next to it: the
; life of the menus, the labels of the items of a dropdown, and the
; `MenuGetHandle` of a menu that is not there.
;
; The numbers of the machine -- the handles and the numbers of the items
; -- are not printed: they are the machine's and they are not the same
; under the v1 and under the tree. What is printed is the behaviour.
;
; It runs under the binary of AutoHotkey v1.1.37.02 and under the tree,
; and the answers of the two are the same.

out := ""
Lbl1 := ""
Lbl2 := ""
Lbl2b := ""
Lbl2c := ""
Lbl4 := ""

Gui, +Resize +hwndPVhwnd
Menu, M1, Add, Plain, Lbl1
Gui, Menu, M1
Gui, Show, w300 h120

hBar := MenuGetHandle("M1")
id1 := DllCall("GetMenuItemID", "Ptr", hBar, "Int", 0)
DllCall("user32\SendMessageW", "Ptr", PVhwnd, "UInt", 0x111, "UInt", id1, "UInt", 0)
Sleep, 200
r := "NO"
if (Lbl1 != "")
   r := "YES"
fits := "NO"
if (id1 >= 0 && id1 <= 0xFFFF)
   fits := "YES"
out .= "P1 plain item of the bar: label ran = " r " the number of it fits in sixteen bits = " fits "`n"

Menu, M2, Add, Inside, Lbl2
Menu, M3, Add, &File, :M2
Gui, Menu, M3
Sleep, 100
hBar2 := MenuGetHandle("M3")
hSub := DllCall("GetSubMenu", "Ptr", hBar2, "Int", 0)
out .= "P2 the dropdown of the bar has " DllCall("GetMenuItemCount", "Ptr", hSub) " items`n"
id2 := DllCall("GetMenuItemID", "Ptr", hSub, "Int", 0)
DllCall("user32\SendMessageW", "Ptr", PVhwnd, "UInt", 0x111, "UInt", id2, "UInt", 0)
Sleep, 200
r := "NO"
if (Lbl2 != "")
   r := "YES"
out .= "P2 item inside the dropdown: label ran = " r "`n"

Menu, M2, Add, Second, Lbl2b
Sleep, 100
out .= "P3 count of the dropdown after a change = " DllCall("GetMenuItemCount", "Ptr", hSub) "`n"
hSub2 := DllCall("GetSubMenu", "Ptr", MenuGetHandle("M3"), "Int", 0)
same := "NO"
if (hSub2 = hSub)
   same := "YES"
out .= "P3 the dropdown of the bar is the same one = " same "`n"

Menu, M2, DeleteAll
Sleep, 100
out .= "P4 count of the dropdown after DeleteAll = " DllCall("GetMenuItemCount", "Ptr", hSub) "`n"
alive := "NO"
if (MenuGetHandle("M2"))
   alive := "YES"
out .= "P4 the menu of the script is still there = " alive "`n"

Menu, M2, Add, Again, Lbl2c
Sleep, 100
out .= "P5 count of M2 after a new Add = " DllCall("GetMenuItemCount", "Ptr", MenuGetHandle("M2")) "`n"

Menu, M4, Add, Deep, Lbl4
Menu, M5, Add, &Edit, :M4
Gui, Menu, M5
Sleep, 100
hM4 := MenuGetHandle("M4")
Menu, M5, DeleteAll
Sleep, 100
out .= "P6 count of M4 after the DeleteAll of the bar = " DllCall("GetMenuItemCount", "Ptr", hM4) "`n"
alive := "NO"
if (MenuGetHandle("M4"))
   alive := "YES"
out .= "P6 the menu that the bar opened is still there = " alive "`n"

Menu, M6, Add, Deep2, Lbl4
Menu, M7, Add, &Tools, :M6
Gui, Menu, M7
Sleep, 100
hM6 := MenuGetHandle("M6")
Menu, M7, Delete
Sleep, 100
out .= "P7 count of M6 after the Delete of the bar = " DllCall("GetMenuItemCount", "Ptr", hM6) "`n"
alive := "NO"
if (MenuGetHandle("M6"))
   alive := "YES"
out .= "P7 the menu that the bar opened is still there = " alive "`n"

out .= "P8 handle of a menu that is not there = " MenuGetHandle("NoSuchMenu") "`n"
out .= "P9 name of the handle of M1 = " MenuGetName(MenuGetHandle("M1")) "`n"

FileAppend, %out%, %A_ScriptDir%\probe-menu.txt
ExitApp

Lbl1() {
   global Lbl1 := "clicked"
}
Lbl2() {
   global Lbl2 := "clicked"
}
Lbl2b() {
   global Lbl2b := "clicked"
}
Lbl2c() {
   global Lbl2c := "clicked"
}
Lbl4() {
   global Lbl4 := "clicked"
}
