; Probe: the shape of a failure of the file family. The probes before this
; one recorded that a failure happened; this one records what the failure
; says -- the class, the What, the Extra and the message -- because a
; message that does not name the cause is a finding of its own.
acc := ""

try {
    FileCopy(A_Temp "\ahk-probe-nonexistent-zzz.txt", A_Temp "\ahk-probe-x.txt")
} catch as e {
    acc := acc "FileCopy.missing=ERR|class:" Type(e) "|what:[" e.What "]|extra:[" e.Extra "]|msg:[" e.Message "]`n"
}
try {
    FileMove(A_Temp "\ahk-probe-nonexistent-zzz.txt", A_Temp "\ahk-probe-x.txt")
} catch as e {
    acc := acc "FileMove.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}
try {
    FileDelete(A_Temp "\ahk-probe-nonexistent-zzz.txt")
} catch as e {
    acc := acc "FileDelete.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}
try {
    FileRead(A_Temp "\ahk-probe-nonexistent-zzz.txt")
} catch as e {
    acc := acc "FileRead.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}
try {
    DirCreate(A_Temp "\ahk-probe-files3")
    FileAppend("x", A_Temp "\ahk-probe-files3\a.txt")
    DirDelete(A_Temp "\ahk-probe-files3")
} catch as e {
    acc := acc "DirDelete.nonempty=ERR|what:[" e.What "]|msg:[" e.Message "]|still-there:[" DirExist(A_Temp "\ahk-probe-files3") "]`n"
}
try {
    FileGetSize(A_Temp "\ahk-probe-nonexistent-zzz.txt")
} catch as e {
    acc := acc "FileGetSize.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}
try {
    FileGetTime(A_Temp "\ahk-probe-nonexistent-zzz.txt")
} catch as e {
    acc := acc "FileGetTime.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}
try {
    FileGetAttrib(A_Temp "\ahk-probe-nonexistent-zzz.txt")
} catch as e {
    acc := acc "FileGetAttrib.missing=ERR|what:[" e.What "]|msg:[" e.Message "]`n"
}

FileAppend acc, "*"
