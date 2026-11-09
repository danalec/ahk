out := ""
out .= "self=" ProcessExist() "`n"
out .= "name=" ProcessGetName() "`n"
out .= "path=" ProcessGetPath() "`n"
out .= "parent=" ProcessGetParent() "`n"
out .= "missing_pid=" ProcessExist(999999) "`n"
out .= "missing_name=" ProcessExist("no-such-proc-zzz") "`n"
try {
  out .= "getname_missing=" ProcessGetName(999999) "`n"
} catch as e {
  out .= "getname_missing=ERR|" e.Message "`n"
}
try {
  out .= "getpath_missing=" ProcessGetPath(999999) "`n"
} catch as e {
  out .= "getpath_missing=ERR|" e.Message "`n"
}
out .= "byName_ahk=" ProcessExist("ahk") "`n"
out .= "prio_self=" ProcessSetPriority("Normal") "`n"
out .= "prio_low=" ProcessSetPriority("Low") "`n"
try {
  out .= "prio_high=" ProcessSetPriority("High") "`n"
} catch as e {
  out .= "prio_high=ERR|" e.Message "`n"
}
FileAppend out, "*"
