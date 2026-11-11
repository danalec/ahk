Handler(Msg) {
  global seen
  seen := Msg
}
a := OnMessage(0x0100, Handler)
FileAppend "onmessage=[" a "]", "*"
try {
  PostMessage(1234, 0x0100, 0)
} catch as e {
  FileAppend "post=[" e.Message "]", "*"
}
try {
  SendMessage(1234, 0x0100, 0)
} catch as e1 {
  FileAppend "send=[" e1.Message "]", "*"
}
try {
  MenuFromHandle(5)
} catch as e2 {
  FileAppend "menu=[" e2.Message "]", "*"
}
cb := CallbackCreate(Handler)
FileAppend "cb=[" cb "]", "*"
try {
  CallbackFree("x")
} catch as e3 {
  FileAppend "free=[" e3.Message "]", "*"
}
g := Gui()
h := g.Hwnd
g2 := GuiFromHwnd(h)
FileAppend "roundtrip=[" Type(g2) "]", "*"
nothing := GuiFromHwnd(999999)
FileAppend "nothing=[" nothing "]", "*"