out := ""
g := Gui()
c := g.Add("Edit", "", "hello")
out .= "type=" Type(c) "`n"
out .= "class=" c.__Class "`n"
out .= "classnn=" c.ClassNN "`n"
out .= "hwnd=" c.Hwnd "`n"
g.Show()
out .= "show=ok`n"
try {
  out .= "label=" g.Add("Text", "", "a label").ClassNN "`n"
} catch as e {
  out .= "label=ERR|" e.Message "`n"
}
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
