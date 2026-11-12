out := ""
try {
  ImageSearch &fx, &fy, 0, 0, 1279, 799, "/usr/share/pixmaps/archlinux-logo.png"
  out .= "search=[" fx "," fy "]" Chr(10)
} catch as e {
  out .= "search=[" e.Message "]" Chr(10)
}
try {
  h := LoadPicture("/usr/share/pixmaps/archlinux-logo.png")
  out .= "load=[" h "]" Chr(10)
} catch as e {
  out .= "load=[" e.Message "]" Chr(10)
}
try {
  out .= "notify=[" TrayTip("a tip", "a title") "]" Chr(10)
} catch as e {
  out .= "notify=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
