; Probe: DllCall on its own. It is the one native name the tree does not
; refuse, and it calls a library of the machine: a crash here is a finding
; of its own, so it does not share a report with the rest.
acc := ""

try {
    r := DllCall("kernel32.dll\GetTickCount", "UInt")
    acc := acc "DllCall.GetTickCount=OK|type:" Type(r) "|value:" r "`n"
} catch as e {
    acc := acc "DllCall.GetTickCount=ERR|" e.Message "`n"
}
try {
    r := DllCall("kernel32.dll\GetCurrentProcessId", "UInt")
    acc := acc "DllCall.GetCurrentProcessId=OK|" r "`n"
} catch as e {
    acc := acc "DllCall.GetCurrentProcessId=ERR|" e.Message "`n"
}
try {
    r := DllCall("user32.dll\GetSystemMetrics", "Int", 0, "Int")
    acc := acc "DllCall.GetSystemMetrics=OK|" r "`n"
} catch as e {
    acc := acc "DllCall.GetSystemMetrics=ERR|" e.Message "`n"
}
try {
    r := DllCall("no-such-library-zzz.dll\Foo", "Int")
    acc := acc "DllCall.missinglib=OK|[" r "]`n"
} catch as e {
    acc := acc "DllCall.missinglib=ERR|" e.Message "`n"
}

FileAppend acc, "*"
