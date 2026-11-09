out := ""
Run "cp /bin/sleep /tmp/ahkprobe-sleep"
Run "setsid /tmp/ahkprobe-sleep 20 > /dev/null 2>&1 < /dev/null &"
Sleep 400
pid := ProcessExist("ahkprobe-sleep")
out .= "found=" pid "`n"
if pid {
  out .= "name=" ProcessGetName(pid) "`n"
  out .= "closed=" ProcessClose("ahkprobe-sleep") "`n"
  Sleep 400
  out .= "after=" ProcessExist("ahkprobe-sleep") "`n"
}
out .= "close_missing=" ProcessClose(999999) "`n"
FileAppend out, "*"
