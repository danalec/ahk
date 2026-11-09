out := ""
t := WinGetTitle()
out .= "active_title=[" t "]`n"
out .= "count=" WinGetCount() "`n"

x := y := w := h := 0
try {
  r := WinGetPos(&x, &y, &w, &h, t)
  out .= "pos=[" r "] x=" x " y=" y " w=" w " h=" h "`n"
} catch as e {
  out .= "pos=ERR|" e.Message "`n"
}

x := y := w := h := 0
try {
  r := WinGetClientPos(&x, &y, &w, &h, t)
  out .= "client=[" r "] x=" x " y=" y " w=" w " h=" h "`n"
} catch as e {
  out .= "client=ERR|" e.Message "`n"
}

x := y := w := h := 0
try {
  r := WinGetPos(&x, &y, &w, &h, "zzz-no-such-zzz")
  out .= "missing=[" r "]`n"
} catch as e {
  out .= "missing=ERR|" e.Message "`n"
}

FileAppend out, "*"
