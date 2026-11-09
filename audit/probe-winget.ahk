out := ""
out .= "count_all=" WinGetCount() "`n"
try {
  l := WinGetList()
  out .= "list_len=" l.Length "`n"
  if l.Length {
    out .= "first_id=[" l[1] "]`n"
  }
} catch as e {
  out .= "list=ERR|" e.Message "`n"
}
try {
  out .= "id_all=[" WinGetID() "]`n"
} catch as e {
  out .= "id_all=ERR|" e.Message "`n"
}
try {
  out .= "class_all=[" WinGetClass() "]`n"
} catch as e {
  out .= "class_all=ERR|" e.Message "`n"
}
try {
  out .= "id_none=[" WinGetID("zzz-no-such-zzz") "]`n"
} catch as e {
  out .= "id_none=ERR|" e.Message "`n"
}
out .= "count_none=" WinGetCount("zzz-no-such-zzz") "`n"
FileAppend out, "*"
