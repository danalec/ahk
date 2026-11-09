; Probe: files family, part 1 (copy/move/delete/exist/read/append/size/time).
; Every path is under the user's temp folder, in a folder this probe makes,
; so nothing of the machine is touched. The result is written to stdout.
acc := ""
base := A_Temp "\ahk-probe-files"
src := base "\a.txt"
dst := base "\b.txt"

try {
    r := DirCreate(base)
    acc := acc "DirCreate=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "DirCreate=ERR|" e.Message "`n"
}

try {
    r := FileAppend("line-one`n", src)
    acc := acc "FileAppend=OK|ret:[" r "]`n"
} catch as e {
    acc := acc "FileAppend=ERR|" e.Message "`n"
}

try {
    r := FileRead(src)
    acc := acc "FileRead=OK|[" r "]`n"
} catch as e {
    acc := acc "FileRead=ERR|" e.Message "`n"
}

try {
    r := FileExist(src)
    acc := acc "FileExist=OK|[" r "]`n"
} catch as e {
    acc := acc "FileExist=ERR|" e.Message "`n"
}

try {
    r := FileCopy(src, dst, 1)
    acc := acc "FileCopy=OK|ret:[" r "]|dst-exist:[" FileExist(dst) "]`n"
} catch as e {
    acc := acc "FileCopy=ERR|" e.Message "`n"
}

try {
    r := FileGetSize(dst)
    acc := acc "FileGetSize=OK|" r "`n"
} catch as e {
    acc := acc "FileGetSize=ERR|" e.Message "`n"
}

try {
    r := FileGetTime(dst, "M")
    acc := acc "FileGetTime=OK|[" r "]`n"
} catch as e {
    acc := acc "FileGetTime=ERR|" e.Message "`n"
}

try {
    r := FileSetTime("20200101120000", dst, "M")
    acc := acc "FileSetTime=OK|ret:[" r "]|now:[" FileGetTime(dst, "M") "]`n"
} catch as e {
    acc := acc "FileSetTime=ERR|" e.Message "`n"
}

try {
    r := FileMove(dst, base "\c.txt", 1)
    acc := acc "FileMove=OK|ret:[" r "]|src-exist:[" FileExist(dst) "]|dst-exist:[" FileExist(base "\c.txt") "]`n"
} catch as e {
    acc := acc "FileMove=ERR|" e.Message "`n"
}

try {
    r := FileReadLine(base "\c.txt", 1)
    acc := acc "FileReadLine=OK|[" r "]`n"
} catch as e {
    acc := acc "FileReadLine=ERR|" e.Message "`n"
}

try {
    r := FileDelete(base "\c.txt")
    acc := acc "FileDelete=OK|ret:[" r "]|exist-after:[" FileExist(base "\c.txt") "]`n"
} catch as e {
    acc := acc "FileDelete=ERR|" e.Message "`n"
}

FileAppend acc, "*"
