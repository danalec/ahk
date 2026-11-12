out := ""
seen := 0
Handler(n) {
  global seen
  seen := n
}
try {
  r := OnClipboardChange(Handler)
  out .= "clip-reg=[" r "]" Chr(10)
  A_Clipboard := "vm probe text"
  out .= "clip-seen=[" seen "]" Chr(10)
} catch as e {
  out .= "clip=[" e.Message "]" Chr(10)
}
try {
  TraySetIcon("/usr/share/pixmaps/archlinux-logo.png")
  out .= "tray-icon=[" A_IconFile "|" A_IconNumber "]" Chr(10)
} catch as e {
  out .= "tray-icon=[" e.Message "]" Chr(10)
}
try {
  m := A_TrayMenu
  out .= "tray-menu=[" Type(m) "|" m.Default "]" Chr(10)
} catch as e {
  out .= "tray-menu=[" e.Message "]" Chr(10)
}
try {
  t := TrayTip("the text of the tip", "the title")
  out .= "tray-tip=[" t "]" Chr(10)
} catch as e {
  out .= "tray-tip=[" e.Message "]" Chr(10)
}
try {
  c := RegCreateKey("HKCU\Software\AhkVmProbe")
  out .= "reg-create=[" c "]" Chr(10)
  w := RegWrite("42", "REG_SZ", "HKCU\Software\AhkVmProbe", "Answer")
  out .= "reg-write=[" w "]" Chr(10)
  rd := RegRead("HKCU\Software\AhkVmProbe", "Answer")
  out .= "reg-read=[" rd "]" Chr(10)
} catch as e {
  out .= "reg=[" e.Message "]" Chr(10)
}
try {
  l := IL_Create(4)
  out .= "il-create=[" l "]" Chr(10)
  ia := IL_Add(l, "/usr/share/pixmaps/archlinux-logo.png")
  out .= "il-add=[" ia "]" Chr(10)
  out .= "il-destroy=[" IL_Destroy(l) "]" Chr(10)
} catch as e {
  out .= "il=[" e.Message "]" Chr(10)
}
try {
  out .= "windir=[" A_WinDir "]" Chr(10)
} catch as e {
  out .= "windir=[" e.Message "]" Chr(10)
}
try {
  out .= "dpi=[" A_ScreenDPI "]" Chr(10)
} catch as e {
  out .= "dpi=[" e.Message "]" Chr(10)
}
try {
  MenuSelect("zzz-nothing-zzz", "", "&File")
  out .= "menu=[ok]" Chr(10)
} catch as e {
  out .= "menu=[" e.Message "]" Chr(10)
}
try {
  out .= "comp=[" ComObjGet("Shell.Application") "]" Chr(10)
} catch as e {
  out .= "comp=[" e.Message "]" Chr(10)
}
out .= "screen=[" A_ScreenWidth "x" A_ScreenHeight "]" Chr(10)
FileAppend(out, "*")
