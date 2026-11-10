out := ""
try {
  out .= "pid=" WinGetPID("ahk-gtk-probe") "`n"
} catch as e {
  out .= "pid=ERR|" e.Message "`n"
}
try {
  out .= "missing=" WinGetPID("zzz-no-window") "`n"
} catch as e {
  out .= "missing=ERR|" e.Message "`n"
}
FileAppend out, "*"
