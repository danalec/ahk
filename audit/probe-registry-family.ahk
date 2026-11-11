c := RegCreateKey("HKCU\Software\AhkPortProbe")
FileAppend "create=[" c "]", "*"
w := RegWrite("42", "REG_SZ", "HKCU\Software\AhkPortProbe", "Answer")
FileAppend "write=[" w "]", "*"
r := RegRead("HKCU\Software\AhkPortProbe", "Answer")
FileAppend "read=[" r "]", "*"
d := RegDelete("HKCU\Software\AhkPortProbe", "Answer")
FileAppend "delete=[" d "]", "*"
try {
  RegRead("HKCU\Software\AhkPortProbe", "Answer")
} catch as e1 {
  FileAppend "gone=[" e1.Message "]", "*"
}
k := RegDeleteKey("HKCU\Software\AhkPortProbe")
FileAppend "delkey=[" k "]", "*"
ll := ListLines(0)
FileAppend "listlines=[" ll "] state=[" A_ListLines "]", "*"
p := Persistent()
FileAppend "persistent=[" p "]", "*"
shown := ListVars()
FileAppend "listvars=[" shown "]", "*"