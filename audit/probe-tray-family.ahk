seen := 0
Handler(n) {
  global seen
  seen := n
}
r := OnClipboardChange(Handler)
FileAppend "reg=[" r "]", "*"
A_Clipboard := "tray probe text"
FileAppend "seen=[" seen "]", "*"
i := TraySetIcon("/usr/share/pixmaps/steam_tray_mono.png")
FileAppend "icon=[" A_IconFile "|" A_IconNumber "]", "*"
t := TrayTip("the text of the tip", "the title")
FileAppend "tip=[" t "]", "*"
m := A_TrayMenu
FileAppend "menu=[" Type(m) "|" m.Default "]", "*"
try {
  TraySetIcon("/tmp/no-such-icon.png")
} catch as e {
  FileAppend "bad-icon=[" e.Message "]", "*"
}