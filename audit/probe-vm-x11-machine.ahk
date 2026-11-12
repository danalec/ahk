out := ""
try {
  RegWrite "42", "HKEY_CURRENT_USER\\Software\\AhkProbe", "Value", "String"
  out .= "reg=[" RegRead("HKEY_CURRENT_USER\\Software\\AhkProbe", "Value") "]" Chr(10)
  RegDelete "HKEY_CURRENT_USER\\Software\\AhkProbe", "Value"
} catch as e {
  out .= "reg=[" e.Message "]" Chr(10)
}
try {
  d := DllCall("libc.so.6 getpid", "int")
  out .= "dll=[" d "]" Chr(10)
} catch as e {
  out .= "dll=[" e.Message "]" Chr(10)
}
try {
  out .= "dirs=[" A_Desktop "|" A_Documents "]" Chr(10)
} catch as e {
  out .= "dirs=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
