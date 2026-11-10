out := ""
g := Gui()
b := g.Add("Button", "", "b")
try {
  out .= "size=[" b.Move(, , 200, 80) "]`n"
} catch as e1 {
  out .= "size=ERR|" e1.Message "`n"
}
try {
  out .= "place=[" b.Move(10, 20) "]`n"
} catch as e2 {
  out .= "place=ERR|" e2.Message "`n"
}
try {
  out .= "none=[" b.Move() "]`n"
} catch as e3 {
  out .= "none=ERR|" e3.Message "`n"
}
g.Show()
g.Destroy()
FileAppend out, "*"
