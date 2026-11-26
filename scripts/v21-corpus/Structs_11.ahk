struct RECT {
    Left : Int32
    Top : Int32
    Right : Int32
    Bottom : Int32
}

struct MONITORINFO {
    cbSize : Int32 := 40
    rcMonitor : RECT
    rcWork : RECT
    dwFlags : Int32
}

DllCall("GetCursorPos", "uint64*", &point:=0)
hMonitor := DllCall("MonitorFromPoint", "int64", point, "uint", 0, "ptr")
DllCall("user32\GetMonitorInfo", "ptr", hMonitor, "ptr", MI := MONITORINFO())
MsgBox(
(
    "Monitor info from point:
    Left: " MI.rcMonitor.Left "
    Top: " MI.rcMonitor.Top "
    Right: " MI.rcMonitor.Right "
    Bottom: " MI.rcMonitor.Bottom "
    WALeft: " MI.rcWork.Left "
    WATop: " MI.rcWork.Top "
    WARight: " MI.rcWork.Right "
    WABottom: " MI.rcWork.Bottom "
    Primary: " MI.dwFlags
))
