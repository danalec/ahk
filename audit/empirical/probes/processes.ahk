; Probe: processes and the machine of v2. The only process touched is the
; runner's own, and the launcher runs a command that ends at once.
acc := ""

try {
    r := Run("cmd /c exit 0")
    acc := acc "Run=OK|[" r "]`n"
} catch as e {
    acc := acc "Run=ERR|" e.Message "`n"
}
try {
    r := RunWait("cmd /c exit 7")
    acc := acc "RunWait=OK|exitcode:[" r "]`n"
} catch as e {
    acc := acc "RunWait=ERR|" e.Message "`n"
}
try {
    r := ProcessExist("ahk.exe")
    acc := acc "ProcessExist=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessExist=ERR|" e.Message "`n"
}
try {
    pid := ProcessExist("ahk.exe")
    r := ProcessGetName(pid)
    acc := acc "ProcessGetName=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessGetName=ERR|" e.Message "`n"
}
try {
    pid := ProcessExist("ahk.exe")
    r := ProcessSetPriority("Normal", pid)
    acc := acc "ProcessSetPriority=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessSetPriority=ERR|" e.Message "`n"
}
try {
    r := ProcessWait("zzz-no-such-process-zzz", 0.2)
    acc := acc "ProcessWait.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessWait.absent=ERR|" e.Message "`n"
}
try {
    r := ProcessClose("zzz-no-such-process-zzz")
    acc := acc "ProcessClose.absent=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessClose.absent=ERR|" e.Message "`n"
}
try {
    r := ProcessWaitClose("ahk.exe", 0.2)
    acc := acc "ProcessWaitClose.self=OK|[" r "]`n"
} catch as e {
    acc := acc "ProcessWaitClose.self=ERR|" e.Message "`n"
}

try {
    r := EnvGet("PATH")
    acc := acc "EnvGet.PATH=OK|len:" StrLen(r) "`n"
} catch as e {
    acc := acc "EnvGet.PATH=ERR|" e.Message "`n"
}
try {
    EnvSet("AHK_PROBE_VAR", "probe-42")
    r := EnvGet("AHK_PROBE_VAR")
    acc := acc "EnvSet=OK|readback:[" r "]`n"
} catch as e {
    acc := acc "EnvSet=ERR|" e.Message "`n"
}

try {
    r := SysGet(80)
    acc := acc "SysGet.80=OK|[" r "]`n"
} catch as e {
    acc := acc "SysGet.80=ERR|" e.Message "`n"
}

FileAppend acc, "*"
