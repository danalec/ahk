out := ""
None0() {
  return 1
}
One1(the) {
  return 2
}
g := Gui()
b := g.Add("Button", "", "b")
e := g.Add("Edit", "", "e")
try {
  out .= "click_none=[" b.OnEvent("Click", None0) "]`n"
} catch as err {
  out .= "click_none=ERR|" err.Message "`n"
}
try {
  out .= "click_one=[" b.OnEvent("Click", One1) "]`n"
} catch as err {
  out .= "click_one=ERR|" err.Message "`n"
}
try {
  out .= "bogus=[" b.OnEvent("Bogus", One1) "]`n"
} catch as err {
  out .= "bogus=ERR|" err.Message "`n"
}
try {
  out .= "close=[" b.OnEvent("Close", One1) "]`n"
} catch as err {
  out .= "close=ERR|" err.Message "`n"
}
try {
  out .= "change=[" e.OnEvent("Change", One1) "]`n"
} catch as err {
  out .= "change=ERR|" err.Message "`n"
}
try {
  out .= "edtclick=[" e.OnEvent("Click", One1) "]`n"
} catch as err {
  out .= "edtclick=ERR|" err.Message "`n"
}
try {
  out .= "notfunc=[" b.OnEvent("Click", 5) "]`n"
} catch as err {
  out .= "notfunc=ERR|" err.Message "`n"
}
FileAppend out, "*"
