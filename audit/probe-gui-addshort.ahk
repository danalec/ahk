out := ""
g := Gui()
a := g.AddText("", "long form")
out .= "long:  class=" a.__Class " classnn=" a.ClassNN "`n"
b := g.AddEdit("", "short form")
out .= "short: class=" b.__Class " classnn=" b.ClassNN " value=[" b.Value "]`n"
c := g.AddButton("", "a button")
out .= "button: class=" c.__Class " text=[" c.Text "]`n"
g.Show()
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
