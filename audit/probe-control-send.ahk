out := ""
try {
  r := ControlSendText("typed by ahk", "entry value", "ahk-gtk-probe")
  out .= "send_ret=[" r "]`n"
} catch as e {
  out .= "send=ERR|" e.Message "`n"
}
try {
  out .= "read_back=[" ControlGetText("typed by ahk", "ahk-gtk-probe") "]`n"
} catch as e {
  out .= "read_back=ERR|" e.Message "`n"
}
try {
  r := ControlSendText("x", "nope", "ahk-gtk-probe")
  out .= "missing_ctl=[" r "]`n"
} catch as e {
  out .= "missing_ctl=ERR|" e.Message "`n"
}
FileAppend out, "*"
