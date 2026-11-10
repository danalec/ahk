out := ""
g := Gui("", "smoke")
t := g.AddText("", "a label")
e := g.AddEdit("", "some text")
b := g.AddButton("", "press me")
out .= "1 types: " t.__Class " " e.__Class " " b.__Class "`n"
out .= "2 values: [" e.Value "] [" t.Text "] [" b.Text "]`n"
e.Value := "changed"
out .= "3 changed: [" e.Value "]`n"
e.Enabled := 0
out .= "4 enabled: " e.Enabled "`n"
e.Enabled := 1
out .= "5 back: " e.Enabled "`n"
e.Visible := 0
out .= "6 visible: " e.Visible "`n"
e.Visible := 1
g.Title := "smoke renamed"
out .= "7 title: [" g.Title "]`n"
g.Show()
out .= "8 shown`n"
r := g.Destroy()
out .= "9 destroy=[" r "]`n"
FileAppend out, "*"
