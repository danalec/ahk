out := ""
try {
  out .= "name=[" WinGetProcessName("ahk-gtk-probe") "]`n"
} catch as e {
  out .= "name=ERR|" e.Message "`n"
}
try {
  out .= "path=[" WinGetProcessPath("ahk-gtk-probe") "]`n"
} catch as e {
  out .= "path=ERR|" e.Message "`n"
}
try {
  out .= "missing=" WinGetProcessName("zzz-no-window") "`n"
} catch as e {
  out .= "missing=ERR|" e.Message "`n"
}
try {
  out .= "persistent=" Persistent() "`n"
} catch as e {
  out .= "persistent=ERR|" e.Message "`n"
}
FileAppend out, "*"
