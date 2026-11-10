out := ""
g := Gui()
e := g.Add("Edit", "", "edit text")
out .= "edit.text=[" e.Text "]`n"
e.Text := "changed"
out .= "after=[" e.Text "] value=[" e.Value "]`n"
l := g.Add("Text", "", "label text")
out .= "label.text=[" l.Text "]`n"
g.Show()
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
