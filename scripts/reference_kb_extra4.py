# SPDX-License-Identifier: Apache-2.0
"""Authored reference prose, batch 4: files + script."""

E = {}
def add(n, d, s, r, x, rel=None, p=None):
    E[n] = {"desc": d, "syntax": s, "params": p or [], "returns": r, "example": x, "related": rel or []}

# -- File object: typed reads/writes -----------------------------------------
for t, desc in [("Char", "a signed 8-bit"), ("UChar", "an unsigned 8-bit"), ("Short", "a signed 16-bit"), ("UShort", "an unsigned 16-bit"), ("Int", "a signed 32-bit"), ("UInt", "an unsigned 32-bit"), ("Int64", "a signed 64-bit"), ("Float", "a 32-bit float"), ("Double", "a 64-bit float")]:
    add("Read" + t, f"Reads {desc} number from the file at the current position.", "N := File.Read" + t + "()", "The number.", "n := f.Read" + t + "()", ["File", "Read", "Write" + t])
    add("Write" + t, f"Writes {desc} number to the file at the current position.", "File.Write" + t + "(N)", "The bytes written.", "f.Write" + t + "(1)", ["File", "Write", "Read" + t])
add("Read", "Reads text (or a number of chars) from the file.", "S := File.Read(Chars)", "The text.", "s := f.Read()", ["File", "ReadLine", "Write"])
add("ReadLine", "Reads one line from the file.", "S := File.ReadLine()", "The line (without the newline).", "s := f.ReadLine()", ["File", "Read", "WriteLine"])
add("Write", "Writes text to the file.", "File.Write(Text)", "The chars written.", "f.Write(\"x\")", ["File", "WriteLine", "Read"])
add("WriteLine", "Writes a line (with the newline) to the file.", "File.WriteLine(Text)", "The chars written.", "f.WriteLine(\"x\")", ["File", "Write", "ReadLine"])
add("Seek", "Moves the file position.", "N := File.Seek(Distance, Origin)", "The new position.", "f.Seek(0)", ["File", "Pos", "Length"])
add("Pos", "The current file position.", "N := File.Pos", "The position; assigning seeks.", "p := f.Pos", ["File", "Seek", "Length"])
add("Length", "The length of the file in bytes.", "N := File.Length", "The length; assigning sets it.", "n := f.Length", ["File", "Pos"])
add("AtEOF", "Whether the file position is at the end.", "B := File.AtEOF", "1/0.", "f.AtEOF", ["File", "Read", "Pos"])
add("Close", "Closes the file handle.", "File.Close()", "Nothing.", "f.Close()", ["File", "FileOpen"])
add("Flush", "Flushes the file's buffers to disk.", "File.Flush()", "Nothing.", "f.Flush()", ["File", "Write"])
add("Handle", "The OS handle of the file.", "H := File.Handle", "The handle.", "h := f.Handle", ["File", "FileOpen"])
add("RawRead", "Reads raw bytes into a Buffer/variable.", "N := File.RawRead(Var, Bytes)", "The bytes read.", "n := f.RawRead(buf, 16)", ["File", "RawWrite", "Buffer"])
add("RawWrite", "Writes raw bytes from a Buffer/variable.", "N := File.RawWrite(Var, Bytes)", "The bytes written.", "n := f.RawWrite(buf, 16)", ["File", "RawRead"])
add("Encoding", "The encoding of the file.", "S := File.Encoding", "The encoding; assigning sets it.", "f.Encoding := \"UTF-8\"", ["File", "FileOpen"])
add("FileReadLine", "Reads a line by number (v1).", "FileReadLine Out, Filename, LineNum", "Nothing.", "FileReadLine s, \"a.txt\", 2", ["FileRead", "LoopRead"])
add("FileGetAttrib", "The attributes of a file (v1).", "FileGetAttrib Out, Filename", "Nothing.", "FileGetAttrib a, \"a.txt\"", ["FileExist", "FileSetAttrib"])
add("FileGetShortcut", "Reads a shortcut's fields.", "FileGetShortcut Path, OutTarget, OutDir, OutArgs, OutDesc, OutIcon, OutIconNum, OutRunState", "Nothing; the vars are filled.", "FileGetShortcut \"a.lnk\", &t", ["FileCreateShortcut"])
add("FileGetVersion", "The version of a file.", "S := FileGetVersion(Filename)", "The version.", "v := FileGetVersion(\"a.exe\")", ["FileExist"])
add("FileFullPath", "The full path of a file (v1).", "FileFullPath Out, Filename", "Nothing.", "FileFullPath p, \"a.txt\"", ["FileExist"])
add("FileCreateTemp", "Creates a temporary file and answers its path.", "S := FileCreateTemp(Prefix)", "The path.", "t := FileCreateTemp()", ["FileAppend", "DirCreate"])
add("FileEncoding", "Sets the default file encoding (v1).", "FileEncoding Encoding", "Nothing.", "FileEncoding UTF-8", ["FileRead", "FileAppend"])
add("FileRecycleEmpty", "Empties the recycle bin.", "FileRecycleEmpty Path", "Nothing.", "FileRecycleEmpty", ["FileRecycle"])
add("FileCopyDir", "Copies a directory (v1).", "FileCopyDir Source, Dest, Overwrite", "Nothing.", "FileCopyDir \"a\", \"b\", 1", ["DirCopy"])
add("FileCreateDir", "Creates a directory (v1).", "FileCreateDir Path", "Nothing.", "FileCreateDir \"a\"", ["DirCreate"])
add("FileMoveDir", "Moves a directory (v1).", "FileMoveDir Source, Dest, Flag", "Nothing.", "FileMoveDir \"a\", \"b\"", ["DirMove"])
add("FileRemoveDir", "Removes a directory (v1).", "FileRemoveDir Path, Recurse", "Nothing.", "FileRemoveDir \"a\", 1", ["DirDelete"])
add("FileSelect", "Shows a file-selection dialog.", "S := FileSelect(Options, RootDir, Prompt, Filter)", "The path(s) or empty.", "p := FileSelect()", ["DirSelect", "InputBox"])
add("DirSelect", "Shows a folder-selection dialog.", "S := DirSelect(StartingFolder, Options, Prompt)", "The path or empty.", "p := DirSelect()", ["FileSelect"])
add("FileSelectFile", "The v1 file-selection dialog.", "FileSelectFile Out, Options, RootDir, Prompt, Filter", "Nothing.", "FileSelectFile p", ["FileSelect"])
add("FileSelectFolder", "The v1 folder-selection dialog.", "FileSelectFolder Out, StartingFolder, Options, Prompt", "Nothing.", "FileSelectFolder p", ["DirSelect"])

# -- Drive* ------------------------------------------------------------------
for n, d, r in [
    ("DriveEject", "Ejects a CD/drive.", "Nothing."),
    ("DriveRetract", "Retracts a CD/drive.", "Nothing."),
    ("DriveLock", "Locks a drive.", "Nothing."),
    ("DriveUnlock", "Unlocks a drive.", "Nothing."),
    ("DriveSetLabel", "Sets a drive's label.", "Nothing."),
]:
    add(n, d + " (Windows).", n + " Drive, Value", r, n + " \"D:\"", ["DriveGet"])
add("DriveGet", "Reads a property of a drive (List/Capacity/SpaceFree/... ).", "S := DriveGet(Cmd, Value)", "The property.", "DriveGet(\"List\")", ["DriveGetList", "DriveGetCapacity"])
add("DriveGetList", "The list of drive letters.", "S := DriveGetList(Type)", "The letters.", "DriveGetList()", ["DriveGet"])
add("DriveGetCapacity", "The capacity of a drive in MB.", "N := DriveGetCapacity(Path)", "The capacity.", "DriveGetCapacity(\"C:\\\")", ["DriveGetSpaceFree"])
add("DriveGetSpaceFree", "The free space of a drive in MB.", "N := DriveGetSpaceFree(Path)", "The free space.", "DriveGetSpaceFree(\"C:\\\")", ["DriveGetCapacity"])
add("DriveGetFilesystem", "The filesystem of a drive (NTFS/FAT32).", "S := DriveGetFilesystem(Path)", "The name.", "DriveGetFilesystem(\"C:\\\")", ["DriveGet"])
add("DriveGetLabel", "The label of a drive.", "S := DriveGetLabel(Path)", "The label.", "DriveGetLabel(\"C:\\\")", ["DriveSetLabel"])
add("DriveGetSerial", "The serial number of a drive.", "N := DriveGetSerial(Path)", "The serial.", "DriveGetSerial(\"C:\\\")", ["DriveGet"])
add("DriveGetStatus", "The status of a drive (Ready/NotReady).", "S := DriveGetStatus(Path)", "The status.", "DriveGetStatus(\"D:\")", ["DriveGetStatusCD"])
add("DriveGetStatusCD", "The status of a CD drive (open/ready/playing).", "S := DriveGetStatusCD(Drive)", "The status.", "DriveGetStatusCD(\"D:\")", ["DriveGetStatus"])
add("DriveGetType", "The type of a drive (Fixed/Removable/CDROM/...).", "S := DriveGetType(Path)", "The type.", "DriveGetType(\"C:\\\")", ["DriveGet"])

# -- LoopFile* variables -----------------------------------------------------
for n, d in [
    ("LoopField", "The current field in a Loop Parse."),
    ("LoopFileAttrib", "The attributes of the current file in a Loop Files."),
    ("LoopFileDir", "The directory of the current file."),
    ("LoopFileExt", "The extension of the current file."),
    ("LoopFileFullPath", "The full path of the current file."),
    ("LoopFileName", "The name of the current file."),
    ("LoopFilePath", "The path of the current file."),
    ("LoopFileShortName", "The 8.3 name of the current file."),
    ("LoopFileShortPath", "The 8.3 path of the current file."),
    ("LoopFileSize", "The size of the current file in bytes."),
    ("LoopFileSizeKB", "The size in KB."),
    ("LoopFileSizeMB", "The size in MB."),
    ("LoopFileTimeAccessed", "The access time of the current file."),
    ("LoopFileTimeCreated", "The creation time of the current file."),
    ("LoopFileTimeModified", "The modification time of the current file."),
    ("LoopReadLine", "The current line in a Loop Read."),
]:
    add(n, d, "S := " + n, "The value.", "x := " + n, ["Loop", "FileExist"])

# -- script: directives ------------------------------------------------------
for n, d in [
    ("#ClipboardTimeout", "Sets the clipboard timeout."),
    ("#CommentFlag", "Sets the v1 comment flag."),
    ("#ErrorStdOut", "Sends load errors to stdout."),
    ("#EscapeChar", "Sets the v1 escape char."),
    ("#HotkeyInterval", "The interval for hotkey-rate limits."),
    ("#HotkeyModifierTimeout", "The modifier timeout of hotkeys."),
    ("#Hotstring", "Sets hotstring options."),
    ("#If", "Makes hotkeys context-sensitive (v1)."),
    ("#IfTimeout", "The timeout of a #If expression."),
    ("#IfWinActive", "Context-sensitive hotkeys by active window (v1)."),
    ("#Include", "Includes another script file."),
    ("#InputLevel", "The send level of the script."),
    ("#InstallKeybdHook", "Installs the keyboard hook."),
    ("#InstallMouseHook", "Installs the mouse hook."),
    ("#KeyHistory", "Sets the key-history size."),
    ("#MaxHotkeysPerInterval", "The max hotkeys per interval."),
    ("#MaxMem", "The v1 max memory per variable."),
    ("#MaxThreads", "The max total threads."),
    ("#MaxThreadsBuffer", "Buffers threads over the max."),
    ("#MaxThreadsPerHotkey", "The max threads per hotkey."),
    ("#MenuMaskKey", "The key used to mask the Win key."),
    ("#NoEnv", "Disables empty-var environment fallback (v1)."),
    ("#NoTrayIcon", "Hides the tray icon."),
    ("#Persistent", "Keeps the script running."),
    ("#Requires", "Requires a minimum version."),
    ("#SingleInstance", "Controls single-instance behaviour."),
    ("#UseHook", "Uses the hook for hotkeys."),
    ("#Warn", "Enables warnings."),
    ("#WinActivateForce", "Forces WinActivate."),
]:
    add(n, d, n + " …", "A directive (load-time).", n, ["#Include", "#Requires"])

# -- script variables / functions -------------------------------------------
for n, d in [
    ("InitialWorkingDir", "The working directory at startup."),
    ("Is64bitOS", "Whether the OS is 64-bit."),
    ("IsAdmin", "Whether the script is elevated."),
    ("IsCompiled", "Whether the script is compiled."),
    ("IsCritical", "Whether the thread is critical."),
    ("IsPaused", "Whether the script is paused."),
    ("IsSuspended", "Whether hotkeys are suspended."),
    ("LineFile", "The file of the current line."),
    ("LineNumber", "The current line number."),
    ("ScriptDir", "The directory of the script."),
    ("ScriptFullPath", "The full path of the script."),
    ("ScriptName", "The name of the script."),
    ("Space", "The space character."),
    ("Tab", "The tab character."),
    ("ThisFunc", "The current function (v1)."),
    ("WorkingDir", "The current working directory."),
    ("EndChar", "The last hotstring end char."),
    ("Index", "The current hotstring/index (ks)."),
]:
    add(n, d, "x := " + n, "The value.", "x := " + n, ["A_WorkingDir", "A_LineNumber"])
add("OnMessage", "Registers a function to handle a window message.", "OnMessage MsgNumber, Function, MaxThreads", "Nothing.", "OnMessage(0x100, Fn)", ["OnExit", "OnError", "SendMessage"])
add("AllowMainWindow", "Allows the main window to be shown (ks).", "AllowMainWindow()", "Nothing.", "AllowMainWindow()", ["Gui", "#NoTrayIcon"])

EXTRA4 = E
