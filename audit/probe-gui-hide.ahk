out := ""
g := Gui()
g.Title := "hide-probe"
g.Show()
Sleep 300
try {
  out .= "hide=[" g.Hide() "]`n"
} catch as err {
  out .= "hide=ERR|" err.Message "`n"
}
try {
  g.Hide(1)
  out .= "two=NO_ERROR`n"
} catch as err2 {
  out .= "two=ERR|" err2.Message "`n"
}
g.Destroy()
FileAppend out, "*"
