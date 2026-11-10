out := ""
try {
  ControlGetText("a", "zzz-no-such-window-zzz")
  out .= "missing_window=NOERROR`n"
} catch as e {
  out .= "missing_window=[" e.Message "]`n"
}
try {
  ControlGetText("hello control", "ahk-ctrl-probe")
  out .= "no_such_control=NOERROR`n"
} catch as e {
  out .= "no_such_control=[" e.Message "]`n"
}
FileAppend out, "*"
