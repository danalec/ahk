out := ""
g := Gui()
c := g.Add("Edit", "", "hello")
out .= "read=[" c.Value "]`n"
c.Value := "written by ahk"
out .= "after=[" c.Value "]`n"
g.Show()
out .= "show=ok`n"
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
