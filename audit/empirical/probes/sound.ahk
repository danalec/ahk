; Probe: sound family (v2 names). No real playback is attempted with a long
; sound: the calls are made against a file that is not there, and the
; refusal or the value is recorded.
acc := ""

try {
    r := SoundBeep(500, 50)
    acc := acc "SoundBeep=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "SoundBeep=ERR|" e.Message "`n"
}
try {
    r := SoundPlay("C:\ahk-no-such-file-zzz.wav")
    acc := acc "SoundPlay=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "SoundPlay=ERR|" e.Message "`n"
}
try {
    r := SoundGetVolume()
    acc := acc "SoundGetVolume=OK|" r "`n"
} catch as e {
    acc := acc "SoundGetVolume=ERR|" e.Message "`n"
}
try {
    r := SoundSetVolume(50)
    acc := acc "SoundSetVolume=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "SoundSetVolume=ERR|" e.Message "`n"
}
try {
    r := SoundGet()
    acc := acc "SoundGet=OK|" r "`n"
} catch as e {
    acc := acc "SoundGet=ERR|" e.Message "`n"
}
try {
    r := SoundSet(50)
    acc := acc "SoundSet=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "SoundSet=ERR|" e.Message "`n"
}
try {
    r := SoundGetMute()
    acc := acc "SoundGetMute=OK|" r "`n"
} catch as e {
    acc := acc "SoundGetMute=ERR|" e.Message "`n"
}

FileAppend acc, "*"
