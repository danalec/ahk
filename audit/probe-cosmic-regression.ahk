count := WinGetCount()
FileAppend "count=[" count "]", "*"
exist := WinExist("zzz-nothing-zzz")
FileAppend "exist=[" exist "]", "*"
waited := WinWait("zzz-nothing-zzz", "", 1)
FileAppend "wait=[" waited "]", "*"
try {
  t := WinGetTitle()
  FileAppend "title=[" t "]", "*"
} catch as e {
  FileAppend "title-reason=[" e.Message "]", "*"
}
