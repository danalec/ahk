out := ""
g := Gui("", "start-title")
out .= "read_before=[" g.Title "]`n"
g.Title := "renamed-title"
out .= "after_set=[" g.Title "]`n"
r := g.Show()
out .= "show=[" r "]`n"
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
