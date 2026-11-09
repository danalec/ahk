out := ""
t := WinGetTitle()
out .= "title=[" t "]`n"
try {
  out .= "minmax=[" WinGetMinMax(t) "]`n"
} catch as e {
  out .= "minmax=ERR|" e.Message "`n"
}
try {
  out .= "missing=[" WinGetMinMax("zzz-no-such-zzz") "]`n"
} catch as e {
  out .= "missing=ERR|" e.Message "`n"
}
FileAppend out, "*"
