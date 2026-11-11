; The controls of a window, read through the bus of the accessibility.

g := Gui()
g.Title := "the-probe-of-the-controls"
e := g.Add("Edit", "", "hello from the port")
g.Show()
Sleep 1500
t := ControlGetText("Edit1", "the-probe-of-the-controls")
FileAppend "[" t "]", "*"
g.Destroy()
