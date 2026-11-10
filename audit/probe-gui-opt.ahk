out := ""
g := Gui()
g.Show()
try {
  out .= "ontop=[" g.Opt("+AlwaysOnTop") "]`n"
} catch as e1 {
  out .= "ontop=ERR|" e1.Message "`n"
}
try {
  out .= "off=[" g.Opt("-AlwaysOnTop") "]`n"
} catch as e2 {
  out .= "off=ERR|" e2.Message "`n"
}
try {
  out .= "bad=[" g.Opt("+NotAThing") "]`n"
} catch as e3 {
  out .= "bad=ERR|" e3.Message "`n"
}
try {
  out .= "refused=[" g.Opt("+SysMenu") "]`n"
} catch as e4 {
  out .= "refused=ERR|" e4.Message "`n"
}
g.Destroy()
FileAppend out, "*"
