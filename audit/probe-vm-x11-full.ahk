out := ""
try {
  MouseGetPos &mx, &my
  out .= "mouse=[" mx "," my "]" Chr(10)
} catch as e {
  out .= "mouse=[" e.Message "]" Chr(10)
}
try {
  out .= "user=[" A_UserName "]" Chr(10)
} catch as e {
  out .= "user=[" e.Message "]" Chr(10)
}
try {
  out .= "host=[" A_ComputerName "]" Chr(10)
} catch as e {
  out .= "host=[" e.Message "]" Chr(10)
}
try {
  out .= "scale=[" A_ScreenDPI "]" Chr(10)
} catch as e {
  out .= "scale=[" e.Message "]" Chr(10)
}
try {
  f := FileOpen("/tmp/ahk-x11-probe.txt", "w")
  f.Write("hello from the x11 path")
  f.Close()
  out .= "file=[" FileRead("/tmp/ahk-x11-probe.txt") "]" Chr(10)
} catch as e {
  out .= "file=[" e.Message "]" Chr(10)
}
try {
  out .= "shell=[" A_AhkPath "]" Chr(10)
} catch as e {
  out .= "ahkpath=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
