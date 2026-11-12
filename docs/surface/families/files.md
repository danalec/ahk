# File and directory operations

The names of the family **File and directory operations** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `A_DD` | v1 | variable | -- | -- | value |
| `A_DDD` | v1 | variable | -- | -- | value |
| `A_DDDD` | v1 | variable | -- | -- | value |
| `A_DirSeparator` | ks | variable | -- | -- | value |
| `A_LoopField` | v1 | variable | -- | -- | value |
| `A_LoopFileName` | v1 | variable | -- | -- | value |
| `A_LoopReadLine` | v1 | variable | -- | -- | value |
| `File.AtEOF` | v2 | method | File | -- | value |
| `File.Close` | v2 | method | File | -- | value |
| `DirCopy` | v2 | command | -- | 2 to 3 | value |
| `DirCreate` | v2 | command | -- | 1 | value |
| `DirDelete` | v2 | command | -- | 1 to 2 | value |
| `DirExist` | v2 | command | -- | 1 | value |
| `DirMove` | v2 | command | -- | 2 to 3 | value |
| `DirSelect` | v2 | command | -- | 0 to 3 | value |
| `Download` | v2 | command | -- | 2 | value |
| `DriveEject` | v2 | command | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveGetCapacity` | v2 | command | -- | 1 | value |
| `DriveGetFilesystem` | v2 | command | -- | 1 | value |
| `DriveGetLabel` | v2 | command | -- | 1 | value |
| `DriveGetList` | v2 | command | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveGetSerial` | v2 | command | -- | 1 | value |
| `DriveGetSpaceFree` | v2 | command | -- | 1 | value |
| `DriveGetStatus` | v2 | command | -- | 1 | value |
| `DriveGetStatusCD` | v2 | command | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveGetType` | v2 | command | -- | 1 | value |
| `DriveLock` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveRetract` | v2 | command | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveSetLabel` | v2 | command | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `DriveUnlock` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `File.Encoding` | v2 | method | File | -- | value |
| `File` | v2 | class | -- | -- | value |
| `Error.File` | v2 | method | Error | -- | value |
| `FileAppend` | v2 | command | -- | 1 to 3 | value |
| `FileAppend` | v1 | command | -- | up to 0 | value |
| `FileCopy` | v2 | command | -- | 2 to 3 | value |
| `FileCopy` | v1 | command | -- | up to 2 | value |
| `FileCopyDir` | v1 | command | -- | up to 2 | value |
| `FileCreateDir` | v1 | command | -- | up to 1 | value |
| `FileCreateShortcut` | v2 | command | -- | 2 to 9 | value |
| `FileCreateShortcut` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileCreateTemp` | ks | function | -- | -- | value |
| `FileDelete` | v2 | command | -- | 1 | value |
| `FileDelete` | v1 | command | -- | up to 1 | value |
| `FileEncoding` | v2 | command | -- | 1 | value |
| `FileEncoding` | v1 | command | -- | up to 0 | value |
| `FileExist` | v2 | command | -- | 1 | value |
| `FileExist` | v1 | function | -- | 1 | value |
| `FileFullPath` | ks | function | -- | -- | value |
| `FileGetAttrib` | v2 | command | -- | 0 to 1 | value |
| `FileGetAttrib` | v1 | command | -- | up to 1 | value |
| `FileGetShortcut` | v2 | command | -- | 1 to 8 | value |
| `FileGetShortcut` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileGetSize` | v2 | command | -- | 0 to 2 | value |
| `FileGetSize` | v1 | command | -- | up to 1 | value |
| `FileGetTime` | v2 | command | -- | 0 to 2 | value |
| `FileGetTime` | v1 | command | -- | up to 1 | value |
| `FileGetVersion` | v2 | command | -- | 0 to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileGetVersion` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileInstall` | v2 | command | -- | 2 to 3 | value |
| `FileInstall` | v1 | command | -- | up to 2 | value |
| `FileMove` | v2 | command | -- | 2 to 3 | value |
| `FileMove` | v1 | command | -- | up to 2 | value |
| `FileMoveDir` | v1 | command | -- | up to 2 | value |
| `FileOpen` | v2 | function | -- | 2 to 3 | value |
| `FileOpen` | v1 | function | -- | 2 to 3 | value |
| `FileRead` | v2 | command | -- | 1 to 2 | value |
| `FileRead` | v1 | command | -- | up to 2 | value |
| `FileReadLine` | v1 | command | -- | up to 3 | value |
| `FileRecycle` | v2 | command | -- | 1 | value |
| `FileRecycle` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileRecycleEmpty` | v2 | command | -- | 0 to 1 | value |
| `FileRecycleEmpty` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileRemoveDir` | v1 | command | -- | up to 1 | value |
| `FileSelect` | v2 | command | -- | 0 to 4 | value |
| `FileSelectFile` | v1 | command | -- | up to 1 | value |
| `FileSelectFolder` | v1 | command | -- | up to 1 | value |
| `FileSetAttrib` | v2 | command | -- | 1 to 3 | value |
| `FileSetAttrib` | v1 | command | -- | up to 1 | value |
| `FileSetTime` | v2 | command | -- | 0 to 4 | value |
| `FileSetTime` | v1 | command | -- | up to 0 | value |
| `File.Flush` | ks | method | File | -- | value |
| `File.Handle` | v2 | method | File | -- | value |
| `IniDelete` | v2 | command | -- | 2 to 3 | value |
| `IniDelete` | v1 | command | -- | up to 2 | value |
| `IniRead` | v2 | command | -- | 1 to 4 | value |
| `IniRead` | v1 | command | -- | up to 2 | value |
| `IniWrite` | v2 | command | -- | 3 to 4 | value |
| `IniWrite` | v1 | command | -- | up to 3 | value |
| `File.Length` | v2 | method | File | -- | value |
| `LoopField` | v2 | variable | -- | -- | value |
| `LoopFileAttrib` | v2 | variable | -- | -- | value |
| `LoopFileDir` | v2 | variable | -- | -- | value |
| `LoopFileExt` | v2 | variable | -- | -- | value |
| `LoopFileFullPath` | v2 | variable | -- | -- | value |
| `LoopFileName` | v2 | variable | -- | -- | value |
| `LoopFilePath` | v2 | variable | -- | -- | value |
| `LoopFileShortName` | v2 | variable | -- | -- | value |
| `LoopFileShortPath` | v2 | variable | -- | -- | value |
| `LoopFileSize` | v2 | variable | -- | -- | value |
| `LoopFileSizeKB` | v2 | variable | -- | -- | value |
| `LoopFileSizeMB` | v2 | variable | -- | -- | value |
| `LoopFileTimeAccessed` | v2 | variable | -- | -- | value |
| `LoopFileTimeCreated` | v2 | variable | -- | -- | value |
| `LoopFileTimeModified` | v2 | variable | -- | -- | value |
| `LoopReadLine` | v2 | variable | -- | -- | value |
| `File.Pos` | v2 | method | File | -- | value |
| `File.RawRead` | v2 | method | File | -- | value |
| `File.RawWrite` | v2 | method | File | -- | value |
| `File.Read` | v2 | method | File | -- | value |
| `File.ReadChar` | v2 | method | File | -- | value |
| `File.ReadDouble` | v2 | method | File | -- | value |
| `File.ReadFloat` | v2 | method | File | -- | value |
| `File.ReadInt` | v2 | method | File | -- | value |
| `File.ReadInt64` | v2 | method | File | -- | value |
| `File.ReadLine` | v2 | method | File | -- | value |
| `File.ReadShort` | v2 | method | File | -- | value |
| `File.ReadUChar` | v2 | method | File | -- | value |
| `File.ReadUInt` | v2 | method | File | -- | value |
| `File.ReadUShort` | v2 | method | File | -- | value |
| `File.Seek` | v2 | method | File | -- | value |
| `SplitPath` | v2 | function | -- | 1 to 6 | value |
| `SplitPath` | v1 | command | -- | up to 1 | value |
| `StringGetPos` | v1 | command | -- | up to 3 | value |
| `File.Write` | v2 | method | File | -- | value |
| `File.WriteChar` | v2 | method | File | -- | value |
| `File.WriteDouble` | v2 | method | File | -- | value |
| `File.WriteFloat` | v2 | method | File | -- | value |
| `File.WriteInt` | v2 | method | File | -- | value |
| `File.WriteInt64` | v2 | method | File | -- | value |
| `File.WriteLine` | v2 | method | File | -- | value |
| `File.WriteShort` | v2 | method | File | -- | value |
| `File.WriteUChar` | v2 | method | File | -- | value |
| `File.WriteUInt` | v2 | method | File | -- | value |
| `File.WriteUShort` | v2 | method | File | -- | value |
