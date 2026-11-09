out := ""
t := A_TickCount
r := ProcessWait("ahk-no-such-zzz", 0.2)
out .= "wait_absent=" r " elapsed=" (A_TickCount - t) "`n"
t := A_TickCount
r := ProcessWait(ProcessExist(), 0.2)
out .= "wait_self=" r " elapsed=" (A_TickCount - t) "`n"

Run "cp /bin/sleep /tmp/ahkprobe-sleep2"
Run "setsid /tmp/ahkprobe-sleep2 2 > /dev/null 2>&1 < /dev/null &"
Sleep 200
t := A_TickCount
r := ProcessWaitClose("ahkprobe-sleep2", 10)
out .= "waitclose_closes=" r " elapsed=" (A_TickCount - t) "`n"

Run "setsid /tmp/ahkprobe-sleep2 30 > /dev/null 2>&1 < /dev/null &"
Sleep 200
t := A_TickCount
r := ProcessWaitClose("ahkprobe-sleep2", 0.2)
out .= "waitclose_timeout=" r " elapsed=" (A_TickCount - t) "`n"
ProcessClose("ahkprobe-sleep2")

try {
  r := TrayTip("the text of the probe", "the title of the probe")
  out .= "traytip=" Type(r) "`n"
} catch as e {
  out .= "traytip=ERR|" e.Message "`n"
}
FileAppend out, "*"
