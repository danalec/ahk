out := ""
g := Gui()
e := g.Add("Edit", "", "x")
Sleep 3000
try {
  v := e.Value
  out := "NO_FAILURE value=[" v "]"
} catch as err {
  out := "ERR|" err.Message
}
FileAppend out, "/tmp/kill-result.txt"
