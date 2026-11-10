out := ""
g := Gui()
c := g.Add("Edit", "", "x")
out .= "fresh: enabled=" c.Enabled " visible=" c.Visible "`n"
c.Enabled := 0
c.Visible := 0
out .= "off:   enabled=" c.Enabled " visible=" c.Visible "`n"
c.Enabled := 1
c.Visible := 1
out .= "on:    enabled=" c.Enabled " visible=" c.Visible "`n"
g.Show()
r := g.Destroy()
out .= "destroy=[" r "]`n"
FileAppend out, "*"
