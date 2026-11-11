list := DriveGetList()
FileAppend "list=[" StrReplace(list, "`n", ",") "]", "*"
free := DriveGetSpaceFree("/")
FileAppend "free=[" free "]", "*"
cap := DriveGetCapacity("/")
FileAppend "cap=[" cap "]", "*"
kind := DriveGetType("/")
FileAppend "type=[" kind "]", "*"
fs := DriveGetFilesystem("/")
FileAppend "fs=[" fs "]", "*"
label := DriveGetLabel("/")
FileAppend "label=[" label "]", "*"
status := DriveGetStatus("/")
FileAppend "status=[" status "]", "*"
try {
  bad := DriveGetLabel("/no-such-drive-zzz")
  FileAppend "bad=[" bad "]", "*"
} catch as e {
  FileAppend "bad-reason=[" e.Message "]", "*"
}
