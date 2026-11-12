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
  out .= "active=[" WinGetTitle() "]" Chr(10)
} catch as e {
  out .= "active=[" e.Message "]" Chr(10)
}
try {
  RegWrite "42", "REG_SZ", "HKCU\Software\AhkX11Probe", "Answer"
  out .= "reg=[" RegRead("HKCU\Software\AhkX11Probe", "Answer") "]" Chr(10)
  RegDelete "HKCU\Software\AhkX11Probe", "Answer"
  RegDeleteKey("HKCU\Software\AhkX11Probe")
} catch as e {
  out .= "reg=[" e.Message "]" Chr(10)
}
try {
  out .= "tray=[" TraySetIcon("/usr/share/pixmaps/archlinux-logo.png") "]" Chr(10)
} catch as e {
  out .= "tray=[" e.Message "]" Chr(10)
}
try {
  Clipboard := "abc"
  Clipboard .= "!"
  out .= "clip=[" Clipboard "]" Chr(10)
} catch as e {
  out .= "clip=[" e.Message "]" Chr(10)
}
try {
  out .= "load=[" LoadPicture("/usr/share/pixmaps/archlinux-logo.png") "]" Chr(10)
} catch as e {
  out .= "load=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
