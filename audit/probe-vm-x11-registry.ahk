out := ""
try {
  c := RegCreateKey("HKCU\Software\AhkX11Probe")
  w := RegWrite("42", "REG_SZ", "HKCU\Software\AhkX11Probe", "Answer")
  r := RegRead("HKCU\Software\AhkX11Probe", "Answer")
  out .= "reg=[" r "]" Chr(10)
  d := RegDelete("HKCU\Software\AhkX11Probe", "Answer")
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
  out .= "clip=[" ClipboardRead() "]" Chr(10)
} catch as e {
  out .= "clip=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
