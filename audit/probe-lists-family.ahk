l := IL_Create(4)
FileAppend "create=[" l "]", "*"
zero := IL_Add(l, "/tmp/no-such-image.png")
FileAppend "add-missing=[" zero "]", "*"
img := IL_Add(l, "/tmp/probe-capture.png")
FileAppend "add-real=[" img "]", "*"
d := IL_Destroy(l)
FileAppend "destroy=[" d "]", "*"
d2 := IL_Destroy(l)
FileAppend "destroy-again=[" d2 "]", "*"
try {
  MenuSelect("zzz-nothing-zzz", "", "&File")
} catch as e1 {
  FileAppend "menu-nowin=[" e1.Message "]", "*"
}
try {
  MenuSelect("Steam", "", "&File")
} catch as e2 {
  FileAppend "menu-steam=[" e2.Message "]", "*"
}