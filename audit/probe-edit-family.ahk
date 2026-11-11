; The edit that has the focus, read of the window of the script.

g := Gui()
g.Title := "the-probe-of-the-edit"
e := g.Add("Edit", "", "hello from the port")
g.Show()
Sleep 1500
line := EditGetCurrentLine()
col := EditGetCurrentCol()
count := EditGetLineCount()
selected := EditGetSelectedText()
caret := CaretGetPos()
FileAppend "line=[" line "] col=[" col "] count=[" count "] selected=[" selected "]", "*"
FileAppend "caret=[" caret "]", "*"
g.Destroy()
