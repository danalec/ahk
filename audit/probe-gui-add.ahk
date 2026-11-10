out := ""
g := Gui()
try {
  c := g.Add("Edit", "", "hello")
  out .= "add_type=" Type(c) "`n"
  out .= "add_class=" c.__Class "`n"
} catch as e {
  out .= "add=ERR|" e.Message "`n"
}
try {
  g.Show()
  out .= "show=ok`n"
} catch as e {
  out .= "show=ERR|" e.Message "`n"
}
FileAppend out, "*"
