out := ""
try {
  out .= "vis=[" ControlGetVisible("hello control", "ahk-gtk-probe") "]`n"
} catch as e {
  out .= "vis=ERR|" e.Message "`n"
}
try {
  out .= "en=[" ControlGetEnabled("hello control", "ahk-gtk-probe") "]`n"
} catch as e {
  out .= "en=ERR|" e.Message "`n"
}
try {
  out .= "flag_missing=[" ControlGetVisible("nope", "ahk-gtk-probe") "]`n"
} catch as e {
  out .= "flag_missing=ERR|" e.Message "`n"
}
try {
  out .= "flag_badwin=[" ControlGetVisible("hello control", "zzz-no-window") "]`n"
} catch as e {
  out .= "flag_badwin=ERR|" e.Message "`n"
}
FileAppend out, "*"
