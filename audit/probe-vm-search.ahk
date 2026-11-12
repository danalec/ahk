out := ""
try {
  out .= "search=[" PixelSearch(0, 0, 100, 100, "0xFFFFFF") "]" Chr(10)
} catch as e {
  out .= "search=[" e.Message "]" Chr(10)
}
try {
  out .= "color=[" PixelGetColor(100, 100) "]" Chr(10)
} catch as e {
  out .= "color=[" e.Message "]" Chr(10)
}
FileAppend(out, "*")
