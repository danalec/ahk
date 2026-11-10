out := ""
Pressed(control) {
  global out
  out .= "pressed [" control.__Class "]`n"
}
g := Gui()
b := g.Add("Button", "", "press me")
b.OnEvent("Click", Pressed)
g.Title := "click-probe"
g.Show()
ControlClick("press me", "click-probe")
Sleep 900
g.Destroy()
FileAppend "[" out "]", "*"
