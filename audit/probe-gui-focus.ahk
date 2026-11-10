out := ""
g := Gui()
e := g.Add("Edit", "", "e")
g.Show()
Sleep 300
try {
  out .= "focus=[" e.Focus() "]`n"
} catch as err {
  out .= "focus=ERR|" err.Message "`n"
}
try {
  e.Focus(1)
  out .= "two=NO_ERROR`n"
} catch as err2 {
  out .= "two=ERR|" err2.Message "`n"
}
g.Destroy()
FileAppend out, "*"
