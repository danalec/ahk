# The file family of v2: what the reference answers, and what the tree answers

The record of the measurement of the file family of AutoHotkey v2.0.19
against this tree. It is a record, not a guide: it says what the
reference binary answered, what the tree answered before, and what the
tree answers now.

The instrument is `scripts/measure-file-family.ps1`: it runs the real
AutoHotkey v2.0.19 on a case, catches the failure, and prints what the
reference answers. The binary is a Windows program, so the instrument has
no `.sh` twin.

The measurement was made on **2026-11-12**, on the binary of v2.0.19 in
`%TEMP%\ahk-v2-measure\AutoHotkey64.exe`.

## The table

| The case | The reference answers | The tree answered | The tree answers |
|---|---|---|---|
| `text := FileRead "f.txt"` | `Expected a String but got a Func.` | the same | the same |
| `r := FileExist "f.txt"` | `Expected a String but got a Func.` | the same | the same |
| `FileInstall s, d, 1` twice | done | `Failed` on the second | done |
| `FileInstall s, d, 0` twice | `Failed` on the second | `Failed` on the second | `Failed` on the second |
| `r := DirDelete("empty")` | the empty text | the path of the folder | the empty text |
| `DirDelete("folder")` (not empty) | `Failed` | `Failed` | `Failed` |
| `r := DirDelete("folder", 1)` | the empty text | the path of the folder | the empty text |
| `FileSetTime "20000101000000", "*.txt"` | done, and the time of every file of the pattern is changed | refused: *the wildcards of v2 are not carried yet.* | done, and the time of every file of the pattern is changed |
| `FileSetTime "20000101000000", "*.nothing"` | done | refused with the reason of the wildcards | done, and nothing is changed |
| `FileSetTime "20000101000000", "sub\*.txt"` | done, and only the files of the folder | refused with the reason of the wildcards | done, and only the files of the folder |
| `FileSetTime "20000101000000", "a.txt"` | the creation time and the modification time are changed | the modification time alone | the creation time and the modification time |
| `FileSetTime "20000101000000", "a.txt", "M"` | the modification time | the modification time | the modification time |
| `FileSetTime "20000101000000", "a.txt", "C"` | the creation time | the creation time | the creation time |
| `FileSetTime "20000101000000", "a.txt", "A"` | the access time | the access time | the access time |
| `FileSetTime "20000101000000", "*.txt", "R"` | `Parameter #3 of FileSetTime is invalid.` | *the recursion of v2 is not carried yet.* | `Parameter #3 of FileSetTime is invalid.` |
| `FileSetTime "20000101000000", "a.txt", "M", "R"` | done, and the subfolders are walked into | *the recursion of v2 is not carried yet.* | done, and the subfolders are walked into |
| `FileSetTime "20000101000000", "a.txt", "M", "X"` | `Parameter #4 of FileSetTime is invalid.` | *the recursion of v2 is not carried yet.* | `Parameter #4 of FileSetTime is invalid.` |
| `FileSetAttrib "+H", "*.txt"` | done | refused: *the wildcards of v2 are not carried yet.* | done |
| `FileSetAttrib "+H", "*.txt", "R"` | done | *the recursion of v2 is not carried yet.* | done, and the subfolders are walked into |
| `FileSetAttrib "+H", "*.txt", "X"` | `Parameter #3 of FileSetAttrib is invalid.` | *the recursion of v2 is not carried yet.* | `Parameter #3 of FileSetAttrib is invalid.` |
| `DirCopy "src", "dest"` | done, and the whole tree is copied | done | done |
| `DirCopy "src", "dest", 1` (the folder is there) | done | done | done |
| `DirCopy "src\*", "dest"` | `Failed` | refused with the reason of the wildcards | refused with the reason of the wildcards |
| `DirMove "src\*", "dest"` | `Failed` | refused with the reason of the wildcards | refused with the reason of the wildcards |
| `FileSetTime "f.txt", "20000101000000"` | `Parameter #1 of FileSetTime is invalid.` | the same | the same |
| `IniWrite "1", "out.ini", "sec", "key"` | done | done | done |

## What changed, and why

1. **`FileInstall` reads the overwrite flag from the wrong parameter.**
   The v2 takes the flag as the third parameter
   (`crates/ahk/reference/ahk-v2.0.19.surface.tsv` says the arity is 2..3)
   and the v1 takes it as the fourth (`FileInstall Source, Dest, Source,
   Overwrite`). The handler read the fourth for both, so the flag of the
   v2 was never read: the second install of a file onto itself failed with
   `Failed` even when the script asked for the overwrite.

2. **`DirDelete` answered the path of the folder.** The v2 answers
   nothing: a name that answers nothing is the empty text. The path was
   the answer of the tree alone.

3. **`FileSetTime` and `FileSetAttrib` refused the wildcards.** The v2
   takes a pattern in the name of the file and changes every file the
   pattern names; a pattern that names nothing is not a failure. The tree
   refused the pattern with the reason of the wildcards. The helper that
   expands a pattern was already in the tree -- `the_files_to_work_on`,
   the one `FileCopy` and `FileMove` use -- and the walk of the desktop
   already knew the recursion.

4. **The recursion of `FileSetTime` and of `FileSetAttrib` was refused.**
   The fourth parameter of the `FileSetTime` is the recursion, and the
   third of the `FileSetAttrib` is the recursion; the "R" walks into the
   subfolders of the folder of the pattern. The tree refused the "R" with
   the reason of the wildcards, and the message of the reference for
   anything that is not the recursion is "Parameter #3 of FileSetTime is
   invalid." (or "Parameter #4", for the fourth parameter).

5. **`FileSetTime` without a letter changed the modification time alone.**
   The v2 changes the creation time and the modification time when it is
   given no letter. The v1 changes the modification time alone, and the
   tree keeps the answer of the v1: the handler asks the dialect which of
   the two it is running.

## What was *not* a divergence

Three of the notes of the audit were the tree already being right, and
the measurement is what says so:

- **The command style in an expression.** `text := FileRead "f.txt"` is
  a failure in the reference too, and the message is the same one:
  `Expected a String but got a Func.` The v2 has no kind of command, and
  the value of a name is reachable with the parentheses only.

- **The order of the parameters of `FileSetTime` and `IniWrite`.** The
  time comes first and the name of the file second; the value comes
  first and the name of the INI second. The reference refuses the other
  order of `FileSetTime` with `Parameter #1 of FileSetTime is invalid.`
  The tree takes the same order.

- **`FileCreateShortcut` and `FileRecycle`.** The reference makes a
  shortcut of Windows (a `.lnk`) and puts a file in the recycle bin of
  Windows. The desktop of this tree is a Linux one: the shortcut is a
  freedesktop `.desktop` file and the trash is the one of the XDG. The
  tree refuses the trash of a machine that has no home directory for the
  user, with the reason of the machine.

## The wildcards of a folder

The v2 does not take a pattern in the name of a folder: `DirCopy` and
`DirMove` fail with `Failed` when the name holds a `*` or a `?`. The
tree refuses them with the reason of the wildcards, which is the way the
tree refuses what it does not carry: the reason, never a `false` where
the reference fails.
