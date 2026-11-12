out := ""
try {
  out .= "scale=[" A_ScreenDPI "]" Chr(10)
} catch as e {
  out .= "scale=[" e.Message "]" Chr(10)
}
try {
  out .= "user=[" A_UserName "]" Chr(10)
} catch as e {
  out .= "user=[" e.Message "]" Chr(10)
}
try {
  out .= "desktop=[" A_Desktop "]" Chr(10)
} catch as e {
  out .= "desktop=[" e.Message "]" Chr(10)
}
try {
  out .= "win=[" WinExist("i3") "]" Chr(10)
} catch as e {
  out .= "win=[" e.Message "]" Chr(10)
}
try {
  MouseGetPos &mx, &my
  out .= "mouse=[" mx "," my "]" Chr(10)
} catch as e {
  out .= "mouse=[" e.Message "]" Chr(10)
}
try {
  out .= "title=[" WinGetTitle() "]" Chr(10)
} catch as e {
  out .= "title=[" e.Message "]" Chr(10)
}
try {
  out .= "active=[" WinActive("i3") "]" Chr(10)
} catch as e {
  out .= "active=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
