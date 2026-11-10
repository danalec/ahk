out := ""
g := Gui()
b := g.Add("Button", "", "button text")
out .= "value=[" b.Value "]`n"
try {
  b.Value := "x"
  out .= "write=NOERROR`n"
} catch as e {
  out .= "write=[" e.Message "]`n"
}
out .= "text=[" b.Text "]`n"
g.Show()
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
