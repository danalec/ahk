out := ""
F() {
  global out
  out .= "handler ran`n"
}
g := Gui()
g.OnEvent("Close", F)
g.Show()
g.Destroy()
Sleep 500
FileAppend "[" out "]", "*"
