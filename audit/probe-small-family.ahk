a := BlockInput("Off")
FileAppend "off=[" a "]", "*"
try {
  BlockInput("On")
} catch as e {
  FileAppend "on=[" e.Message "]", "*"
}
try {
  Edit()
} catch as e2 {
  FileAppend "edit=[" e2.Message "]", "*"
}
s := RunAs()
FileAppend "runas=[" s "]", "*"
d := Shutdown(0)
FileAppend "shutdown=[" d "]", "*"
try {
  FileGetVersion("/bin/sh")
} catch as e3 {
  FileAppend "elf=[" e3.Message "]", "*"
}
try {
  FileGetVersion("/tmp/no-such-file.exe")
} catch as e4 {
  FileAppend "missing=[" e4.Message "]", "*"
}