out := ""
Run "setsid zenity --info --title=ahk-close-probe --text=probe > /dev/null 2>&1 < /dev/null &"
Sleep 1800
out .= "before=" WinExist("ahk-close-probe") "`n"
out .= "count_before=" WinGetCount("ahk-close-probe") "`n"
try {
  r := WinClose("ahk-close-probe")
  out .= "closed=[" r "]`n"
} catch as e {
  out .= "closed=ERR|" e.Message "`n"
}
Sleep 1500
out .= "after=" WinExist("ahk-close-probe") "`n"
try {
  r := WinClose("zzz-no-such-zzz")
  out .= "missing=[" r "]`n"
} catch as e {
  out .= "missing=ERR|" e.Message "`n"
}
FileAppend out, "*"
