out := ""
out .= "color=[" PixelGetColor(100, 100) "]" Chr(10)
MouseGetPos &mx, &my
out .= "mouse=[" mx "," my "]" Chr(10)
out .= "scale=[" A_ScreenDPI "]" Chr(10)
out .= "user=[" A_UserName "]" Chr(10)
out .= "desktop=[" A_Desktop "]" Chr(10)
out .= "win-exists=[" WinExist("i3") "]" Chr(10)
out .= "pixel-search=[" PixelSearch(0, 0, 100, 100, "0xFFFFFF") "]" Chr(10)
FileAppend(out, "*")
