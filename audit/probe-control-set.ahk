out := ""
try {
  r := ControlSetText("written by ahk", "entry value", "ahk-gtk-probe")
  out .= "set_ret=[" r "]`n"
} catch as e {
  out .= "set=ERR|" e.Message "`n"
}
try {
  out .= "read_back=[" ControlGetText("written by ahk", "ahk-gtk-probe") "]`n"
} catch as e {
  out .= "read_back=ERR|" e.Message "`n"
}
try {
  r := ControlSetText("x", "nope", "ahk-gtk-probe")
  out .= "missing_ctl=[" r "]`n"
} catch as e {
  out .= "missing_ctl=ERR|" e.Message "`n"
}
try {
  r := ControlSetText("x", "entry value", "zzz-no-window")
  out .= "missing_win=[" r "]`n"
} catch as e {
  out .= "missing_win=ERR|" e.Message "`n"
}
FileAppend out, "*"
