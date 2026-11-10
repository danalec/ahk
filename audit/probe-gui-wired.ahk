out := ""
g := Gui("", "ahk-gui-wired")
r := g.Show()
out .= "show=[" r "]`n"
Sleep 400
r := g.Show()
out .= "show_again=[" r "]`n"
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
