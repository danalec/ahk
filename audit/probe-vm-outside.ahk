out := ""
try {
  out .= "fora=[" PixelGetColor(5000, 5000) "]" Chr(10)
} catch as e {
  out .= "fora=[" e.Message "]" Chr(10)
}
out .= "depois=[ok]" Chr(10)
FileAppend(out, "*")
