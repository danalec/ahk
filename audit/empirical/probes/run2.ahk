; Probe: Run, round two. v2 gives Run the process id of the new process,
; and it may be asked to write it into a variable; here a process that
; lives a moment is started so the id can be compared with zero.
acc := ""

try {
    r := Run("cmd /c exit 0")
    acc := acc "Run.short=OK|[" r "]`n"
} catch as e {
    acc := acc "Run.short=ERR|" e.Message "`n"
}
try {
    r := Run("cmd /c ping -n 3 127.0.0.1 > nul")
    acc := acc "Run.longer=OK|[" r "]`n"
} catch as e {
    acc := acc "Run.longer=ERR|" e.Message "`n"
}
try {
    pid := 0
    r := Run("cmd /c exit 0", , , &pid)
    acc := acc "Run.pidvar=OK|ret:[" r "]|pid:[" pid "]`n"
} catch as e {
    acc := acc "Run.pidvar=ERR|" e.Message "`n"
}
try {
    r := RunWait("cmd /c exit 5", , 2)
    acc := acc "RunWait.timeout=OK|[" r "]`n"
} catch as e {
    acc := acc "RunWait.timeout=ERR|" e.Message "`n"
}

FileAppend acc, "*"
