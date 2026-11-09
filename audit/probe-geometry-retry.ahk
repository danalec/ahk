out := ""
t := WinGetTitle()
out .= "title=[" t "]`n"
Loop 3 {
  x := y := w := h := 0
  try {
    WinGetPos(&x, &y, &w, &h, t)
    out .= "try" A_Index ": x=" x " y=" y " w=" w " h=" h "`n"
  } catch as e {
    out .= "try" A_Index "=ERR|" e.Message "`n"
  }
  Sleep 300
}
FileAppend out, "*"
