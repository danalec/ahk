out := ""
F() {
  return 1
}
Handler(the) {
  return 2
}
g := Gui()
b := g.Add("Button", "", "b")
try {
  b.OnEvent("Click", F)
} catch as e1 {
  out .= "F: " e1.Message "`n"
}
try {
  b.OnEvent("Click", Handler)
} catch as e2 {
  out .= "Handler: " e2.Message "`n"
}
FileAppend out, "*"
