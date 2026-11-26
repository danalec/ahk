struct Point {
    x : Int32, y : Int32
}

; Use the class itself to pass by value (C type: POINT).
WindowFromPoint := DllCall.Bind("WindowFromPoint", Point, unset, "uptr")

; Use "ptr" to pass by reference without type-checking (C type: POINT*).
GetCursorPos := DllCall.Bind("GetCursorPos", "ptr", unset)
pt1 := Point()
GetCursorPos(pt1)
MsgBox WinGetClass(WindowFromPoint(pt1))

; Use .Ptr to pass by reference or receive output via a VarRef.
GetCursorPos := DllCall.Bind("GetCursorPos", Point.Ptr, unset)
GetCursorPos(&pt2)
MsgBox WinGetClass(WindowFromPoint(pt2))
