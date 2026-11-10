out := ""
try {
  r := SysGetIPAddresses()
  out .= "type=" Type(r) " len=" r.Length "`n"
  for a in r
    out .= "[" a "]`n"
} catch as e {
  out .= "ERR|" e.Message "`n"
}
FileAppend out, "*"
