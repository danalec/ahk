# Operaciones de archivos y directorios

| [English](../index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [日本語](../ja/index.md) | [中文](../zh/index.md) |

Idioma: Español

Los nombres de la familia **Operaciones de archivos y directorios** en los tres dialectos, con lo que este árbol responde por cada uno. La familia es el área en la que un script piensa; los nombres son la totalidad de ella.

| Nombre | Dialecto | Tipo | Clase | Parámetros | Estado |
|---|---|---|---|---|---|
| `A_DD` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `A_DDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `A_DDDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `A_DirSeparator` | KeySharp | variable | -- | -- | valor |
| `A_LoopField` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `A_LoopFileName` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `A_LoopReadLine` | AutoHotkey v1.1.37.02 | variable | -- | -- | valor |
| `File.AtEOF` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.Close` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `DirCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `DirCreate` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DirDelete` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | valor |
| `DirExist` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DirMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `DirSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | valor |
| `Download` | AutoHotkey v2.0.19 | command | -- | 2 | valor |
| `DriveEject` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `DriveGetCapacity` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetFilesystem` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetLabel` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetList` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `DriveGetSerial` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetSpaceFree` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetStatus` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveGetStatusCD` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `DriveGetType` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveLock` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `DriveRetract` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `DriveSetLabel` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | valor |
| `DriveUnlock` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `File.Encoding` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File` | AutoHotkey v2.0.19 | class | -- | -- | valor |
| `Error.File` | AutoHotkey v2.0.19 | method | Error | -- | valor |
| `FileAppend` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | valor |
| `FileAppend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor |
| `FileCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `FileCopy` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileCopyDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileCreateDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileCreateShortcut` | AutoHotkey v2.0.19 | command | -- | 2 to 9 | valor |
| `FileCreateShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | rechazado: es un nombre de Windows, y este escritorio no es Windows. |
| `FileCreateTemp` | KeySharp | function | -- | -- | valor |
| `FileDelete` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `FileDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileEncoding` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `FileEncoding` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor |
| `FileExist` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `FileExist` | AutoHotkey v1.1.37.02 | function | -- | 1 | valor |
| `FileFullPath` | KeySharp | function | -- | -- | valor |
| `FileGetAttrib` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `FileGetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileGetShortcut` | AutoHotkey v2.0.19 | command | -- | 1 to 8 | valor |
| `FileGetShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | rechazado: es un nombre de Windows, y este escritorio no es Windows. |
| `FileGetSize` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | valor |
| `FileGetSize` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileGetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | valor |
| `FileGetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileGetVersion` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `FileGetVersion` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | rechazado: es un nombre de Windows, y este escritorio no es Windows. |
| `FileInstall` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `FileInstall` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `FileMove` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileMoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileOpen` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | valor |
| `FileOpen` | AutoHotkey v1.1.37.02 | function | -- | 2 to 3 | valor |
| `FileRead` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | valor |
| `FileRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `FileReadLine` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | valor |
| `FileRecycle` | AutoHotkey v2.0.19 | command | -- | 1 | valor |
| `FileRecycle` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | rechazado: es un nombre de Windows, y este escritorio no es Windows. |
| `FileRecycleEmpty` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | valor |
| `FileRecycleEmpty` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | rechazado: es un nombre de Windows, y este escritorio no es Windows. |
| `FileRemoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | valor |
| `FileSelectFile` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileSelectFolder` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileSetAttrib` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | valor |
| `FileSetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `FileSetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | valor |
| `FileSetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | valor |
| `File.Flush` | KeySharp | method | File | -- | valor |
| `File.Handle` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `IniDelete` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | valor |
| `IniDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `IniRead` | AutoHotkey v2.0.19 | command | -- | 1 to 4 | valor |
| `IniRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | valor |
| `IniWrite` | AutoHotkey v2.0.19 | command | -- | 3 to 4 | valor |
| `IniWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | valor |
| `File.Length` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `LoopField` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileAttrib` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileDir` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileExt` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileName` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFilePath` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileShortName` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileShortPath` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileSize` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileSizeKB` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileSizeMB` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileTimeAccessed` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileTimeCreated` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopFileTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `LoopReadLine` | AutoHotkey v2.0.19 | variable | -- | -- | valor |
| `File.Pos` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.RawRead` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.RawWrite` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.Read` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadChar` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadDouble` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadFloat` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadInt` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadInt64` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadLine` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadShort` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadUChar` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadUInt` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.ReadUShort` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.Seek` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `SplitPath` | AutoHotkey v2.0.19 | function | -- | 1 to 6 | valor |
| `SplitPath` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | valor |
| `StringGetPos` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | valor |
| `File.Write` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteChar` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteDouble` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteFloat` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteInt` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteInt64` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteLine` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteShort` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteUChar` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteUInt` | AutoHotkey v2.0.19 | method | File | -- | valor |
| `File.WriteUShort` | AutoHotkey v2.0.19 | method | File | -- | valor |
