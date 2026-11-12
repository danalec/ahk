out := ""
out .= "screen=[" A_ScreenWidth "x" A_ScreenHeight "]" Chr(10)
try {
  out .= "title=[" WinGetTitle() "]" Chr(10)
} catch as e {
  out .= "title=[" e.Message "]" Chr(10)
}
try {
  out .= "exist-i3=[" WinExist("i3") "]" Chr(10)
} catch as e {
  out .= "exist-i3=[" e.Message "]" Chr(10)
}
try {
  out .= "exist-nada=[" WinExist("zzz-nada-zzz") "]" Chr(10)
} catch as e {
  out .= "exist-nada=[" e.Message "]" Chr(10)
}
try {
  p := MouseGetPos()
  out .= "mouse=[" p.X "x" p.Y "]" Chr(10)
} catch as e {
  out .= "mouse=[" e.Message "]" Chr(10)
}
try {
  out .= "pixel=[" PixelGet(100, 100) "]" Chr(10)
} catch as e {
  out .= "pixel=[" e.Message "]" Chr(10)
}
try {
  out .= "menu-algum=[" MenuSelect("i3", "", "&File") "]" Chr(10)
} catch as e {
  out .= "menu-algum=[" e.Message "]" Chr(10)
}
try {
  out .= "tecla=[" Send("{F13}") "]" Chr(10)
} catch as e {
  out .= "tecla=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
