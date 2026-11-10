out := ""
F() {
  return 1
}
G(the) {
  return 2
}
g := Gui()
b := g.Add("Button", "", "b")
e := g.Add("Edit", "", "e")
try {
  out .= "click0=[" b.OnEvent("Click", F) "]`n"
} catch as err {
  out .= "click0=ERR|" err.Message "`n"
}
try {
  out .= "click1=[" b.OnEvent("Click", G) "]`n"
} catch as err {
  out .= "click1=ERR|" err.Message "`n"
}
try {
  out .= "bogus=[" b.OnEvent("Bogus", G) "]`n"
} catch as err {
  out .= "bogus=ERR|" err.Message "`n"
}
try {
  out .= "close=[" b.OnEvent("Close", G) "]`n"
} catch as err {
  out .= "close=ERR|" err.Message "`n"
}
try {
  out .= "edtchange=[" e.OnEvent("Change", G) "]`n"
} catch as err {
  out .= "edtchange=ERR|" err.Message "`n"
}
try {
  out .= "edtclick=[" e.OnEvent("Click", G) "]`n"
} catch as err {
  out .= "edtclick=ERR|" err.Message "`n"
}
try {
  out .= "notfunc=[" b.OnEvent("Click", 5) "]`n"
} catch as err {
  out .= "notfunc=ERR|" err.Message "`n"
}
FileAppend out, "*"
