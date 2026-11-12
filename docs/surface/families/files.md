# File and directory operations

| [English](index.md) | [Español](es/index.md) | [Português (Brasil)](pt-BR/index.md) | [Français](fr/index.md) | [Italiano](it/index.md) | [Polski](pl/index.md) | [Türkçe](tr/index.md) | [Deutsch](de/index.md) | [Русский](ru/index.md) | [日本語](ja/index.md) | [中文（简体）](zh/index.md) | [繁體中文](zh-Hant/index.md) | [한국어](ko/index.md) |

Language: English

The names of the family **File and directory operations** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State | Probe |
|---|---|---|---|---|---|---|
| `A_DD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_DDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_DDDD` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_DirSeparator` | KeySharp | variable | -- | -- | value | -- |
| `A_LoopField` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_LoopFileName` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `A_LoopReadLine` | AutoHotkey v1.1.37.02 | variable | -- | -- | value | -- |
| `File.AtEOF` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.Close` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `DirCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `DirCreate` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DirDelete` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value | -- |
| `DirExist` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DirMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `DirSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 3 | value | -- |
| `Download` | AutoHotkey v2.0.19 | command | -- | 2 | value | -- |
| `DriveEject` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `DriveGetCapacity` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetFilesystem` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetLabel` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetList` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `DriveGetSerial` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetSpaceFree` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetStatus` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveGetStatusCD` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `DriveGetType` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveLock` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `DriveRetract` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `DriveSetLabel` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value | -- |
| `DriveUnlock` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `File.Encoding` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File` | AutoHotkey v2.0.19 | class | -- | -- | value | -- |
| `Error.File` | AutoHotkey v2.0.19 | method | Error | -- | value | -- |
| `FileAppend` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | value | -- |
| `FileAppend` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `FileCopy` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `FileCopy` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileCopyDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileCreateDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileCreateShortcut` | AutoHotkey v2.0.19 | command | -- | 2 to 9 | value | -- |
| `FileCreateShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileCreateTemp` | KeySharp | function | -- | -- | value | -- |
| `FileDelete` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `FileDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileEncoding` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `FileEncoding` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `FileExist` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `FileExist` | AutoHotkey v1.1.37.02 | function | -- | 1 | value | -- |
| `FileFullPath` | KeySharp | function | -- | -- | value | -- |
| `FileGetAttrib` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `FileGetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileGetShortcut` | AutoHotkey v2.0.19 | command | -- | 1 to 8 | value | -- |
| `FileGetShortcut` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileGetSize` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | value | -- |
| `FileGetSize` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileGetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 2 | value | -- |
| `FileGetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileGetVersion` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `FileGetVersion` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileInstall` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `FileInstall` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileMove` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `FileMove` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileMoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileOpen` | AutoHotkey v2.0.19 | function | -- | 2 to 3 | value | -- |
| `FileOpen` | AutoHotkey v1.1.37.02 | function | -- | 2 to 3 | value | -- |
| `FileRead` | AutoHotkey v2.0.19 | command | -- | 1 to 2 | value | -- |
| `FileRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `FileReadLine` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value | -- |
| `FileRecycle` | AutoHotkey v2.0.19 | command | -- | 1 | value | -- |
| `FileRecycle` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileRecycleEmpty` | AutoHotkey v2.0.19 | command | -- | 0 to 1 | value | -- |
| `FileRecycleEmpty` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. | refusal: this is a name of Windows, and this desktop is not Windows. |
| `FileRemoveDir` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileSelect` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | value | -- |
| `FileSelectFile` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileSelectFolder` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileSetAttrib` | AutoHotkey v2.0.19 | command | -- | 1 to 3 | value | -- |
| `FileSetAttrib` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `FileSetTime` | AutoHotkey v2.0.19 | command | -- | 0 to 4 | value | -- |
| `FileSetTime` | AutoHotkey v1.1.37.02 | command | -- | up to 0 | value | -- |
| `File.Flush` | KeySharp | method | File | -- | value | -- |
| `File.Handle` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `IniDelete` | AutoHotkey v2.0.19 | command | -- | 2 to 3 | value | -- |
| `IniDelete` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `IniRead` | AutoHotkey v2.0.19 | command | -- | 1 to 4 | value | -- |
| `IniRead` | AutoHotkey v1.1.37.02 | command | -- | up to 2 | value | -- |
| `IniWrite` | AutoHotkey v2.0.19 | command | -- | 3 to 4 | value | -- |
| `IniWrite` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value | -- |
| `File.Length` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `LoopField` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileAttrib` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileDir` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileExt` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileFullPath` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileName` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFilePath` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileShortName` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileShortPath` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileSize` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileSizeKB` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileSizeMB` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileTimeAccessed` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileTimeCreated` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopFileTimeModified` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `LoopReadLine` | AutoHotkey v2.0.19 | variable | -- | -- | value | -- |
| `File.Pos` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.RawRead` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.RawWrite` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.Read` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadChar` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadDouble` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadFloat` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadInt` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadInt64` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadLine` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadShort` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadUChar` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadUInt` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.ReadUShort` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.Seek` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `SplitPath` | AutoHotkey v2.0.19 | function | -- | 1 to 6 | value | -- |
| `SplitPath` | AutoHotkey v1.1.37.02 | command | -- | up to 1 | value | -- |
| `StringGetPos` | AutoHotkey v1.1.37.02 | command | -- | up to 3 | value | -- |
| `File.Write` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteChar` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteDouble` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteFloat` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteInt` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteInt64` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteLine` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteShort` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteUChar` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteUInt` | AutoHotkey v2.0.19 | method | File | -- | value | -- |
| `File.WriteUShort` | AutoHotkey v2.0.19 | method | File | -- | value | -- |

## What the tree does with it

### A_DD

The day of the month of the local time, in two digits.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the day, as a text of two digits
- **Errors:** the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

```ahk
FileAppend A_DD, "*"
```

### A_LoopField

The field the parse loop is on: the piece of the text between two delimiters.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the field, as a text; the empty text before the first parse loop
- **Errors:** nothing to read; the write is refused.

```ahk
Loop Parse "a,b,c", ","
    FileAppend A_LoopField, "*"
```

### A_LoopFileName

The name of the file the file loop is on, without its folder.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the name of the file, as a text; the empty text before the first file loop
- **Errors:** nothing to read; the write is refused.

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```

### A_LoopReadLine

The line of the file the read loop is on.

- **Parameters:** nothing; it is a variable, not a call
- **Answers:** the line, as a text; the empty text before the first read loop
- **Errors:** nothing to read; the write is refused.

```ahk
Loop Read "notes.txt"
    FileAppend A_LoopReadLine, "*"
```

### File.AtEOF

Whether the file is at its end. It is a property and not a method: `f.AtEOF`.

- **Parameters:** nothing; it is a property, not a call
- **Answers:** 1 when the file is at its end, 0 when it is not
- **Errors:** a closed file answers one, for there is nothing more to read; the size of a file the machine cannot measure fails with the reason of the desktop.

```ahk
f := FileOpen("notes.txt", "r")
FileAppend f.AtEOF, "*"
```

### File.Close

Closes the file and gives the handle back to the machine.

- **Parameters:** nothing
- **Answers:** the empty text
- **Errors:** the console is closed with the empty text; a close that the machine refuses fails with the reason of the desktop.

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```

### DirCopy

Copies a directory and everything in it. What is copied is the content of the directory into the destination.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not a directory, or when the destination is there and the overwrite is not asked for; a name that holds a wildcard is refused with the reason that the wildcards of v2 are not carried.

```ahk
DirCopy "src", "dst"
```

### DirCreate

Makes a directory, with the folders that are missing in the way.

- **Parameters:** DirName
- **Answers:** nothing
- **Errors:** OSError when the directory cannot be made.

```ahk
DirCreate "out"
```

### DirDelete

Takes a directory away. A directory that is not empty stays without it unless the recursion is asked for.

- **Parameters:** DirName, [Recurse]
- **Answers:** the name of the directory.
- **Errors:** Error `Failed` when the directory is not there, or when it is not empty and the recursion is not asked for.

```ahk
d := DirDelete("out", 1)
```

### DirExist

Answers the attributes of the first folder that the name or the pattern matches.

- **Parameters:** DirName
- **Answers:** `D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.
- **Errors:** nothing fails.

```ahk
x := DirExist("out")
```

### DirMove

Moves a directory, with everything in it. A destination that is there is merged with the one that comes: the files of the source win.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not a directory; a name that holds a wildcard is refused with the reason that the wildcards of v2 are not carried.

```ahk
DirMove "src", "dst"
```

### Download

Fetches a file of the network and writes it on the disk. The box does it with `curl`; the flag of the cache that v2 writes in front of the url is accepted and dropped.

- **Parameters:** URL, FileName
- **Answers:** nothing
- **Errors:** Error when the url cannot be reached at all, or when the machine has no `curl`; a file that is not there on the network is not a failure, and the page of the error of the server is what is written.

```ahk
Download "https://example.com/x", "x.html"
```

### File

The class of an open file of the machine. The class is not made: `File(...)` answers "Invalid base."; the file is made by the `FileOpen` built-in, and it is the only way to have one.

- **Parameters:** nothing that makes it; the arguments of `FileOpen` open the file
- **Answers:** the object of the class `File`
- **Errors:** the call of the class is refused with "Invalid base.".

```ahk
f := FileOpen("notes.txt", "w")
f.Close()
```

### FileAppend

Adds text at the end of a file. The name of the file may be `*`, and the text then goes to the output of the report; inside the body of a `Loop Read` the name may be left out, and the file of the loop is the one that is written.

- **Parameters:** Text, FileName, [Options]
- **Answers:** nothing
- **Errors:** ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

```ahk
FileAppend "hello`n", "out.txt"
```

### FileAppend

Adds text at the end of a file. The name of the file may be `*`, and the text then goes to the output of the report; inside the body of a `Loop Read` the name may be left out, and the file of the loop is the one that is written.

- **Parameters:** Text, FileName, [Options]
- **Answers:** nothing
- **Errors:** ValueError when the name of the file is empty or when an option is not one of v2; OSError when the file cannot be opened for writing. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and a linefeed; a code page the tree does not carry is refused with the reason.

```ahk
FileAppend "hello`n", "out.txt"
```

### FileCopy

Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

```ahk
FileCopy "a.txt", "b.txt"
```

### FileCopy

Copies a file, or every file that a pattern names. A destination that ends with a separator, or that is a folder, is the folder the files are copied into.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

```ahk
FileCopy "a.txt", "b.txt"
```

### FileCreateShortcut

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

- **Parameters:** Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]
- **Answers:** nothing
- **Errors:** Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```

### FileCreateShortcut

Makes a shortcut to a program. On this desktop a shortcut is a `.desktop` file of the freedesktop specification, and it is made even for a target that is not there.

- **Parameters:** Target, LinkFile, [WorkingDir], [Args], [Description], [Icon], [IconNumber], [Run]
- **Answers:** nothing
- **Errors:** Error when the file of the shortcut cannot be written. The icon number and the run state are not carried.

```ahk
FileCreateShortcut "/bin/ls", "ls.desktop"
```

### FileDelete

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

- **Parameters:** FileName
- **Answers:** nothing
- **Errors:** OSError when the file cannot be taken away.

```ahk
FileDelete "out.txt"
```

### FileDelete

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

- **Parameters:** FileName
- **Answers:** nothing
- **Errors:** OSError when the file cannot be taken away.

```ahk
FileDelete "out.txt"
```

### FileExist

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

- **Parameters:** FileName
- **Answers:** the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.
- **Errors:** nothing fails: a name that matches nothing answers the empty text.

```ahk
x := FileExist("out.txt")
```

### FileExist

Answers the attributes of the first entry that the name or the pattern matches. The value of the command is taken with the parenthesis, the way `FileExist("a.txt")` is written.

- **Parameters:** FileName
- **Answers:** the attributes as letters: `A` for a file, `D` for a folder, `RA` for a file that cannot be written, and the empty text when there is no such file.
- **Errors:** nothing fails: a name that matches nothing answers the empty text.

```ahk
x := FileExist("out.txt")
```

### FileGetSize

Answers how big a file is.

- **Parameters:** FileName, [Unit]
- **Answers:** the size, rounded down; a folder answers zero.
- **Errors:** ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

```ahk
n := FileGetSize("out.txt", "K")
```

### FileGetSize

Answers how big a file is.

- **Parameters:** FileName, [Unit]
- **Answers:** the size, rounded down; a folder answers zero.
- **Errors:** ValueError when the unit is not `B`, `K` or `M`; OSError when the file cannot be read.

```ahk
n := FileGetSize("out.txt", "K")
```

### FileGetTime

Answers when a file was made, when it was changed, or when it was read.

- **Parameters:** FileName, [Which]
- **Answers:** the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.
- **Errors:** ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

```ahk
t := FileGetTime("out.txt", "M")
```

### FileGetTime

Answers when a file was made, when it was changed, or when it was read.

- **Parameters:** FileName, [Which]
- **Answers:** the stamp of v2, the fourteen digits of `YYYYMMDDHH24MISS`, in the local time of the machine.
- **Errors:** ValueError when `Which` is not `M`, `C`, `A` or `CUT`; OSError when the file cannot be read.

```ahk
t := FileGetTime("out.txt", "M")
```

### FileInstall

Copies a file the way the installer of a compiled script does. Nothing is compiled here: the file is copied.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there; the overwrite flag of v2 is not read by the tree, so a copy onto a file that is there fails.

```ahk
FileInstall "data.txt", "data.out.txt"
```

### FileInstall

Copies a file the way the installer of a compiled script does. Nothing is compiled here: the file is copied.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there; the overwrite flag of v2 is not read by the tree, so a copy onto a file that is there fails.

```ahk
FileInstall "data.txt", "data.out.txt"
```

### FileMove

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

```ahk
FileMove "a.txt", "b.txt"
```

### FileMove

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

- **Parameters:** Source, Dest, [Overwrite]
- **Answers:** nothing
- **Errors:** Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

```ahk
FileMove "a.txt", "b.txt"
```

### FileOpen

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

- **Parameters:** FileName, Mode, [Encoding]
- **Answers:** a `File` object.
- **Errors:** ValueError when the mode is not one of the four; OSError when the file cannot be opened.

```ahk
f := FileOpen("out.txt", "w")
```

### FileOpen

Opens a file and answers the object of it, of the class `File`. The modes are `r`, `w`, `a` and `rw`; the name `*` is the console of the machine.

- **Parameters:** FileName, Mode, [Encoding]
- **Answers:** a `File` object.
- **Errors:** ValueError when the mode is not one of the four; OSError when the file cannot be opened.

```ahk
f := FileOpen("out.txt", "w")
```

### FileRead

Reads a whole file and answers its text. The mark of the encoding decides the encoding of the file, and the option of the script decides when the file has no mark. The value of the command is taken with the parenthesis, the way `FileRead("out.txt")` is written.

- **Parameters:** FileName, [Options]
- **Answers:** the text of the file; a Buffer when the option `RAW` is there.
- **Errors:** OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

```ahk
text := FileRead("out.txt")
```

### FileRead

Reads a whole file and answers its text. The mark of the encoding decides the encoding of the file, and the option of the script decides when the file has no mark. The value of the command is taken with the parenthesis, the way `FileRead("out.txt")` is written.

- **Parameters:** FileName, [Options]
- **Answers:** the text of the file; a Buffer when the option `RAW` is there.
- **Errors:** OSError when the file cannot be read; ValueError when an option is not one of v2. The options are the encoding of the file -- `UTF-8`, `UTF-16` and the code page of the machine --, `RAW` and `m<n>`, the number of bytes to read.

```ahk
text := FileRead("out.txt")
```

### FileRecycle

Sends a file, a folder, or the files that a pattern matches, to the trash of the machine. The box uses the trash of the XDG specification, and it is the home directory of the user that holds it.

- **Parameters:** FileName
- **Answers:** nothing
- **Errors:** Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

```ahk
FileRecycle "out.txt"
```

### FileRecycle

Sends a file, a folder, or the files that a pattern matches, to the trash of the machine. The box uses the trash of the XDG specification, and it is the home directory of the user that holds it.

- **Parameters:** FileName
- **Answers:** nothing
- **Errors:** Error `Failed` when the path is not there; a pattern that matches nothing is not a failure; ValueError when the name is empty; Error when the machine does not say where the home directory of the user is.

```ahk
FileRecycle "out.txt"
```

### FileSetTime

Changes the time of a file. The time comes first and the name of the file second, the way the binary of v2.0.19 takes them; a time that is blank is the moment of the machine.

- **Parameters:** TimeStamp, FileName, [Which], [Mode]
- **Answers:** nothing
- **Errors:** ValueError when the stamp or the time that is wanted is not one of v2; OSError when the times of the file cannot be changed. The recursion of v2 is refused with the reason.

```ahk
FileSetTime "20000101000000", "out.txt", "M"
```

### FileSetTime

Changes the time of a file. The time comes first and the name of the file second, the way the binary of v2.0.19 takes them; a time that is blank is the moment of the machine.

- **Parameters:** TimeStamp, FileName, [Which], [Mode]
- **Answers:** nothing
- **Errors:** ValueError when the stamp or the time that is wanted is not one of v2; OSError when the times of the file cannot be changed. The recursion of v2 is refused with the reason.

```ahk
FileSetTime "20000101000000", "out.txt", "M"
```

### IniDelete

Takes a key of an ini file away. A key that is not given takes the whole section with it.

- **Parameters:** FileName, Section, [Key]
- **Answers:** nothing
- **Errors:** Error when the file cannot be written.

```ahk
IniDelete "app.ini", "Main", "Name"
```

### IniDelete

Takes a key of an ini file away. A key that is not given takes the whole section with it.

- **Parameters:** FileName, Section, [Key]
- **Answers:** nothing
- **Errors:** Error when the file cannot be written.

```ahk
IniDelete "app.ini", "Main", "Name"
```

### IniRead

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

- **Parameters:** FileName, Section, Key, [Default]
- **Answers:** the value of the key; the default of the script when the key is not there and a default is given.
- **Errors:** Error `The requested key, section or file was not found.` when the key is not there and no default is given.

```ahk
v := IniRead("app.ini", "Main", "Name")
```

### IniRead

Reads the value of a key of an ini file. With no section it answers the names of the sections; with no key it answers the section itself. The ini files are the ones of Windows: `[section]`, `key=value`, and `;` for a comment.

- **Parameters:** FileName, Section, Key, [Default]
- **Answers:** the value of the key; the default of the script when the key is not there and a default is given.
- **Errors:** Error `The requested key, section or file was not found.` when the key is not there and no default is given.

```ahk
v := IniRead("app.ini", "Main", "Name")
```

### IniWrite

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.19 takes them; a file that is not there is made.

- **Parameters:** Value, FileName, Section, Key
- **Answers:** nothing
- **Errors:** Error when the file cannot be written.

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

### IniWrite

Writes a key of an ini file. The value comes first and the name of the file second, the way the binary of v2.0.19 takes them; a file that is not there is made.

- **Parameters:** Value, FileName, Section, Key
- **Answers:** nothing
- **Errors:** Error when the file cannot be written.

```ahk
IniWrite "hello", "app.ini", "Main", "Name"
```

### File.Length

The size of the file, in bytes. It is a property and not a method: `f.Length`.

- **Parameters:** nothing; it is a property, not a call
- **Answers:** the size, as a number; a closed file answers zero
- **Errors:** the write of a size below the size of the file is refused: this desktop cannot make a file shorter, it cannot cut one.

```ahk
f := FileOpen("notes.txt", "r")
FileAppend f.Length, "*"
```

### File.Pos

The position of the file, in bytes. It is a property and not a method: `f.Pos`.

- **Parameters:** nothing; it is a property, not a call
- **Answers:** the position, as a number; a closed file answers -1
- **Errors:** the write of a negative position is refused with "the position of a file cannot be negative.".

```ahk
f := FileOpen("notes.txt", "r")
FileAppend f.Pos, "*"
```

### File.Read

Reads the text of the file at its position and moves the position on.

- **Parameters:** the number of characters, which is optional
- **Answers:** the text, as a text; without a number it is the rest of the file
- **Errors:** a closed file, a file opened to be written and the console answer the empty text; a negative count is refused with "the count of characters cannot be negative."; an encoding the runtime cannot read is refused with the reason.

```ahk
f := FileOpen("notes.txt", "r")
FileAppend f.Read(5), "*"
```

### File.ReadLine

Reads the line of the file at its position and moves the position on.

- **Parameters:** nothing
- **Answers:** the line, as a text, without the newline that ends it
- **Errors:** a closed file, a file opened to be written and the console answer the empty text; an encoding the runtime cannot read is refused with the reason.

```ahk
f := FileOpen("notes.txt", "r")
FileAppend f.ReadLine(), "*"
```

### SplitPath

Cuts a path into its pieces and writes them into the variables of the caller. A path of a file is cut at the backslash and a url at the slash, and the drive of a url is its scheme and its host.

- **Parameters:** Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive
- **Answers:** nothing
- **Errors:** TypeError when a piece is not a variable to write into.

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```

### SplitPath

Cuts a path into its pieces and writes them into the variables of the caller. A path of a file is cut at the backslash and a url at the slash, and the drive of a url is its scheme and its host.

- **Parameters:** Path, &OutFileName, &OutDir, &OutExt, &OutNameNoExt, &OutDrive
- **Answers:** nothing
- **Errors:** TypeError when a piece is not a variable to write into.

```ahk
SplitPath "C:\dir\file.txt", &name, &dir, &ext
```

### File.Write

Writes a text into the file at its position and moves the position on.

- **Parameters:** the text
- **Answers:** the number of characters that were written
- **Errors:** a closed file takes the text and keeps none; the write of the machine fails with the reason of the desktop.

```ahk
f := FileOpen("notes.txt", "w")
FileAppend f.Write("hello"), "*"
```

### File.WriteLine

Writes a text and a newline into the file at its position and moves the position on.

- **Parameters:** the text
- **Answers:** the number of characters that were written, the newline included
- **Errors:** a closed file takes the text and keeps none; the write of the machine fails with the reason of the desktop.

```ahk
f := FileOpen("notes.txt", "w")
FileAppend f.WriteLine("hello"), "*"
```
