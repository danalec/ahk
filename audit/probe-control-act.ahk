out := ""
try {
  r := ControlFocus("entry value", "ahk-gtk-probe")
  out .= "focus=[" r "]`n"
} catch as e {
  out .= "focus=ERR|" e.Message "`n"
}
try {
  r := ControlFocus("hello control", "ahk-gtk-probe")
  out .= "focus_label=[" r "]`n"
} catch as e {
  out .= "focus_label=ERR|" e.Message "`n"
}
try {
  r := ControlClick("Close", "ahk-gtk-probe")
  out .= "click=[" r "]`n"
} catch as e {
  out .= "click=ERR|" e.Message "`n"
}
Sleep 900
try {
  r := ControlGetText("hello control", "ahk-gtk-probe")
  out .= "after=[" r "]`n"
} catch as e {
  out .= "after=ERR|" e.Message "`n"
}
FileAppend out, "*"
