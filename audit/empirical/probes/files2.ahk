; Probe: files family, part 2 (attribs, dirs, version, recycle, shortcut).
; Every path is under the user's temp folder.
acc := ""
base := A_Temp "\ahk-probe-files2"
f := base "\a.txt"

try {
    r := DirCreate(base)
    acc := acc "DirCreate=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "DirCreate=ERR|" e.Message "`n"
}
try {
    FileAppend("x", f)
} catch as e {
    acc := acc "FileAppend=ERR|" e.Message "`n"
}

try {
    r := FileGetAttrib(f)
    acc := acc "FileGetAttrib=OK|[" r "]`n"
} catch as e {
    acc := acc "FileGetAttrib=ERR|" e.Message "`n"
}
try {
    r := FileSetAttrib("+R", f)
    acc := acc "FileSetAttrib=OK|ret:[" r "]|attrib-after:[" FileGetAttrib(f) "]`n"
} catch as e {
    acc := acc "FileSetAttrib=ERR|" e.Message "`n"
}
try {
    FileSetAttrib("-R", f)
} catch as e {
    acc := acc "FileSetAttrib(-R)=ERR|" e.Message "`n"
}

try {
    r := FileGetVersion("C:\Windows\System32\notepad.exe")
    acc := acc "FileGetVersion=OK|[" r "]`n"
} catch as e {
    acc := acc "FileGetVersion=ERR|" e.Message "`n"
}

try {
    r := FileCreateShortcut("C:\Windows\System32\notepad.exe", A_Temp "\ahk-probe.lnk")
    acc := acc "FileCreateShortcut=OK|ret:[" r "]|exist:[" FileExist(A_Temp "\ahk-probe.lnk") "]`n"
} catch as e {
    acc := acc "FileCreateShortcut=ERR|" e.Message "`n"
}

try {
    r := FileRecycle(A_Temp "\ahk-probe.lnk")
    acc := acc "FileRecycle=OK|ret:[" r "]|exist:[" FileExist(A_Temp "\ahk-probe.lnk") "]`n"
} catch as e {
    acc := acc "FileRecycle=ERR|" e.Message "`n"
}

try {
    r := DirExist(base)
    acc := acc "DirExist=OK|[" r "]`n"
} catch as e {
    acc := acc "DirExist=ERR|" e.Message "`n"
}
try {
    r := DirCopy(base, base "-copy", 1)
    acc := acc "DirCopy=OK|ret:[" r "]|copied-exist:[" DirExist(base "-copy") "]`n"
} catch as e {
    acc := acc "DirCopy=ERR|" e.Message "`n"
}
try {
    r := DirMove(base "-copy", base "-moved", 1)
    acc := acc "DirMove=OK|ret:[" r "]|moved-exist:[" DirExist(base "-moved") "]`n"
} catch as e {
    acc := acc "DirMove=ERR|" e.Message "`n"
}
try {
    r := DirDelete(base "-moved")
    acc := acc "DirDelete=OK|ret:[" r "]|exist-after:[" DirExist(base "-moved") "]`n"
} catch as e {
    acc := acc "DirDelete=ERR|" e.Message "`n"
}

FileAppend acc, "*"
