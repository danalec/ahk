; The windows of the session, read of the compositor.

count := WinGetCount()
titles := ""
for id in WinGetList() {
    titles .= id " "
}
FileAppend "count=[" count "] ids=[" titles "]", "*"
for id in WinGetList() {
    FileAppend "title[" id "]=[" WinGetTitle(id) "]", "*"
}
waited := WinWait("zzz-nothing-zzz", "", 1)
FileAppend "wait=[" waited "]", "*"
exist := WinExist("zzz-nothing-zzz")
FileAppend "exist=[" exist "]", "*"
