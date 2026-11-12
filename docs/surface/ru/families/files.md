# Операции с файлами и каталогами

| [English](../index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Язык: Русский

Имена семейства **Операции с файлами и каталогами** в трёх диалектах и то, что это дерево отвечает на каждое из них. Семейство -- это область, в которой думает скрипт; имена составляют её целиком.

| Имя | Диалект | Вид | Класс | Параметры | Состояние | Проба |
|---|---|---|---|---|---|---|
| `A_DD` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `A_DDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `A_DDDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `A_DirSeparator` | KeySharp | variable | -- | -- | значение | -- |
| `A_LoopField` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `A_LoopFileName` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `A_LoopReadLine` | AutoHotkey v1.1.37.02 | variable | -- | -- | значение | -- |
| `File.AtEOF` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.Close` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `DirCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `DirCreate` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DirDelete` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | значение | -- |
| `DirExist` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DirMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `DirSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | значение | -- |
| `Download` | AutoHotkey v2.0.19 | command | -- | 2 | значение | -- |
| `DriveEject` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `DriveGetCapacity` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetFilesystem` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetLabel` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetList` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `DriveGetSerial` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetSpaceFree` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetStatus` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveGetStatusCD` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `DriveGetType` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveLock` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `DriveRetract` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `DriveSetLabel` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | значение | -- |
| `DriveUnlock` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `File.Encoding` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File` | AutoHotkey v2.0.19 | class | -- | -- | значение | -- |
| `Error.File` | AutoHotkey v2.0.19 | method | Error | -- | значение | -- |
| `FileAppend` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | значение | -- |
| `FileAppend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | значение | -- |
| `FileCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `FileCopy` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileCopyDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileCreateDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileCreateShortcut` | AutoHotkey v2.0.19 | command | -- | 2 to 9 | значение | -- |
| `FileCreateShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | отказано: это имя Windows, а этот рабочий стол — не Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileCreateTemp` | KeySharp | function | -- | -- | значение | -- |
| `FileDelete` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `FileDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileEncoding` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `FileEncoding` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | значение | -- |
| `FileExist` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `FileExist` | AutoHotkey v1.1.37.02 | function | -- | 1 | значение | -- |
| `FileFullPath` | KeySharp | function | -- | -- | значение | -- |
| `FileGetAttrib` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `FileGetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileGetShortcut` | AutoHotkey v2.0.19 | command | -- | 1 to 8 | значение | -- |
| `FileGetShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | отказано: это имя Windows, а этот рабочий стол — не Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileGetSize` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | значение | -- |
| `FileGetSize` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileGetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | значение | -- |
| `FileGetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileGetVersion` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `FileGetVersion` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | отказано: это имя Windows, а этот рабочий стол — не Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileInstall` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `FileInstall` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `FileMove` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileMoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileOpen` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | значение | -- |
| `FileOpen` | AutoHotkey v1.1.37.02 | function | -- | 2 to 3 | значение | -- |
| `FileRead` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | значение | -- |
| `FileRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `FileReadLine` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | значение | -- |
| `FileRecycle` | AutoHotkey v2.0.19 | command | -- | 1 | значение | -- |
| `FileRecycle` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | отказано: это имя Windows, а этот рабочий стол — не Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileRecycleEmpty` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | значение | -- |
| `FileRecycleEmpty` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | отказано: это имя Windows, а этот рабочий стол — не Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileRemoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | значение | -- |
| `FileSelectFile` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileSelectFolder` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileSetAttrib` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | значение | -- |
| `FileSetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `FileSetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | значение | -- |
| `FileSetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | значение | -- |
| `File.Flush` | KeySharp | method | File | -- | значение | -- |
| `File.Handle` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `IniDelete` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | значение | -- |
| `IniDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `IniRead` | AutoHotkey v2.0.19 | command | -- | 1 to 4 | значение | -- |
| `IniRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | значение | -- |
| `IniWrite` | AutoHotkey v2.0.19 | command | -- | 3 to 4 | значение | -- |
| `IniWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | значение | -- |
| `File.Length` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `LoopField` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileAttrib` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileDir` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileExt` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileName` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFilePath` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileShortName` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileShortPath` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileSize` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileSizeKB` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileSizeMB` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileTimeAccessed` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileTimeCreated` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopFileTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `LoopReadLine` | AutoHotkey v2.0.19 | variable | -- | -- | значение | -- |
| `File.Pos` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.RawRead` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.RawWrite` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.Read` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadChar` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadDouble` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadFloat` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadInt` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadInt64` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadLine` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadShort` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadUChar` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadUInt` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.ReadUShort` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.Seek` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `SplitPath` | AutoHotkey v2.0.19 | function | -- | 1 to 6 | значение | -- |
| `SplitPath` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | значение | -- |
| `StringGetPos` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | значение | -- |
| `File.Write` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteChar` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteDouble` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteFloat` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteInt` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteInt64` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteLine` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteShort` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteUChar` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteUInt` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |
| `File.WriteUShort` | AutoHotkey v2.0.19 | method | File | -- | значение | -- |

## Что дерево делает с ним

Поведение этого семейства написано по-английски, по-китайски и по-португальски (Бразилия); читайте его на [Язык](../families/files.md).
