out := ""
g := Gui()
F() {
  return 1
}
try {
  out .= "close=[" g.OnEvent("Close", F) "]`n"
} catch as e {
  out .= "close=ERR|" e.Message "`n"
}
try {
  out .= "escape=[" g.OnEvent("Escape", F) "]`n"
} catch as e {
  out .= "escape=ERR|" e.Message "`n"
}
try {
  out .= "bogus=[" g.OnEvent("Bogus", F) "]`n"
} catch as e {
  out .= "bogus=ERR|" e.Message "`n"
}
try {
  out .= "notfunc=[" g.OnEvent("Close", 5) "]`n"
} catch as e {
  out .= "notfunc=ERR|" e.Message "`n"
}
FileAppend out, "*"
