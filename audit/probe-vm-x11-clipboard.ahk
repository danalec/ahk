out := ""
try {
  Clipboard := "abc"
  Clipboard .= "!"
  out .= "append=[ok] valor=[" Clipboard "]" Chr(10)
} catch as e {
  out .= "append=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
