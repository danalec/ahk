# File and directory operations

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [日本語](ja/index.md) | [中文](zh/index.md) |

Language: English

The names of the family **File and directory operations** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `A_DD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DDDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_DirSeparator` | KeySharp | variable | -- | -- | value |
| `A_LoopField` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_LoopFileName` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `A_LoopReadLine` | AutoHotkey v1.1.37.02 | variable | -- | -- | value |
| `File.AtEOF` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.Close` | AutoHotkey v2.0.19 | method | File | -- | value |
| `DirCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `DirCreate` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DirDelete` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `DirExist` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DirMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `DirSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | value |
| `Download` | AutoHotkey v2.0.19 | command | -- | 2 | value |
| `DriveEject` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `DriveGetCapacity` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetFilesystem` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetLabel` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetList` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `DriveGetSerial` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetSpaceFree` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetStatus` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveGetStatusCD` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `DriveGetType` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveLock` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `DriveRetract` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `DriveSetLabel` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `DriveUnlock` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `File.Encoding` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File` | AutoHotkey v2.0.19 | class | -- | -- | value |
| `Error.File` | AutoHotkey v2.0.19 | method | Error | -- | value |
| `FileAppend` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | value |
| `FileAppend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `FileCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `FileCopy` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileCopyDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileCreateDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileCreateShortcut` | AutoHotkey v2.0.19 | command | -- | 2 to 9 | value |
| `FileCreateShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileCreateTemp` | KeySharp | function | -- | -- | value |
| `FileDelete` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `FileDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileEncoding` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `FileEncoding` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `FileExist` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `FileExist` | AutoHotkey v1.1.37.02 | function | -- | 1 | value |
| `FileFullPath` | KeySharp | function | -- | -- | value |
| `FileGetAttrib` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `FileGetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileGetShortcut` | AutoHotkey v2.0.19 | command | -- | 1 to 8 | value |
| `FileGetShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileGetSize` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | value |
| `FileGetSize` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileGetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | value |
| `FileGetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileGetVersion` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `FileGetVersion` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileInstall` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `FileInstall` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `FileMove` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileMoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileOpen` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | value |
| `FileOpen` | AutoHotkey v1.1.37.02 | function | -- | 2 to 3 | value |
| `FileRead` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value |
| `FileRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `FileReadLine` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value |
| `FileRecycle` | AutoHotkey v2.0.19 | command | -- | 1 | value |
| `FileRecycle` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileRecycleEmpty` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value |
| `FileRecycleEmpty` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `FileRemoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | value |
| `FileSelectFile` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileSelectFolder` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileSetAttrib` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | value |
| `FileSetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `FileSetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | value |
| `FileSetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value |
| `File.Flush` | KeySharp | method | File | -- | value |
| `File.Handle` | AutoHotkey v2.0.19 | method | File | -- | value |
| `IniDelete` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value |
| `IniDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `IniRead` | AutoHotkey v2.0.19 | command | -- | 1 to 4 | value |
| `IniRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value |
| `IniWrite` | AutoHotkey v2.0.19 | command | -- | 3 to 4 | value |
| `IniWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value |
| `File.Length` | AutoHotkey v2.0.19 | method | File | -- | value |
| `LoopField` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileAttrib` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileDir` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileExt` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileName` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFilePath` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileShortName` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileShortPath` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileSize` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileSizeKB` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileSizeMB` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileTimeAccessed` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileTimeCreated` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopFileTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `LoopReadLine` | AutoHotkey v2.0.19 | variable | -- | -- | value |
| `File.Pos` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.RawRead` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.RawWrite` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.Read` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadChar` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadDouble` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadFloat` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadInt` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadInt64` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadLine` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadShort` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadUChar` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadUInt` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.ReadUShort` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.Seek` | AutoHotkey v2.0.19 | method | File | -- | value |
| `SplitPath` | AutoHotkey v2.0.19 | function | -- | 1 to 6 | value |
| `SplitPath` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value |
| `StringGetPos` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value |
| `File.Write` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteChar` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteDouble` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteFloat` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteInt` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteInt64` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteLine` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteShort` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteUChar` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteUInt` | AutoHotkey v2.0.19 | method | File | -- | value |
| `File.WriteUShort` | AutoHotkey v2.0.19 | method | File | -- | value |
