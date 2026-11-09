; Probe: Reload, on its own. Reload replaces the running script with a new
; run, so the probe is run alone with a hard timeout: if the runner
; restarts itself, the audit stops it instead of letting it loop.
acc := ""
try {
    r := Reload()
    acc := acc "Reload=OK|[" r "]`n"
} catch as e {
    acc := acc "Reload=ERR|" e.Message "`n"
}

FileAppend acc, "*"
