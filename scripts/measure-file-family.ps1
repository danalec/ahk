# Measure the file family of AutoHotkey v2.0.19 on the reference binary.
#
#   powershell -NoProfile -ExecutionPolicy Bypass scripts/measure-file-family.ps1
#
# WHY: the tree follows the binaries, and a rule without its measurement is
# an opinion. This script is the instrument of the measurements of the file
# family: it runs the real AutoHotkey v2.0.19 on a case, catches the
# failures, and prints what the reference answers.
#
# The binary lives at %TEMP%\ahk-v2-measure\AutoHotkey64.exe. It is a
# Windows program, so this script has no .sh twin: there is no AutoHotkey
# on the Linux box to measure against.
#
# Every case runs in its own process and is killed after a few seconds: a
# case that opens a dialog would block the run, and a load error of the
# binary is a dialog.
#
# The cases that take something away come last: a case that deletes a
# folder makes the case after it fail for the wrong reason.
param(
    [string]$Binary = "$env:TEMP\ahk-v2-measure\AutoHotkey64.exe",
    [int]$Seconds = 8
)

if (-not (Test-Path $Binary)) {
    Write-Output "the reference binary is not there: $Binary"
    exit 1
}

$dir = Join-Path $env:TEMP "ahk-measure-file-family"
if (Test-Path $dir) { Remove-Item -Recurse -Force $dir }
New-Item -ItemType Directory -Force -Path $dir | Out-Null
Set-Content -Path (Join-Path $dir "hello.txt") -Value "hello" -Encoding UTF8
New-Item -ItemType Directory -Force -Path (Join-Path $dir "empty") | Out-Null
foreach ($name in @("src", "src2", "src3")) {
    $folder = Join-Path $dir $name
    New-Item -ItemType Directory -Force -Path $folder | Out-Null
    Set-Content -Path (Join-Path $folder "a.txt") -Value "a" -Encoding UTF8
    Set-Content -Path (Join-Path $folder "b.log") -Value "b" -Encoding UTF8
    New-Item -ItemType Directory -Force -Path (Join-Path $folder "deep") | Out-Null
    Set-Content -Path (Join-Path $folder "deep\c.txt") -Value "c" -Encoding UTF8
}

function Measure-Ahk([string]$name, [string]$body) {
    $script = Join-Path $dir "$name.ahk"
    Set-Content -Path $script -Value $body -Encoding UTF8
    $outFile = Join-Path $dir "$name.out"
    $errFile = Join-Path $dir "$name.err"
    $process = Start-Process -FilePath $Binary -ArgumentList "/ErrorStdOut", $script `
        -PassThru -NoNewWindow -RedirectStandardOutput $outFile -RedirectStandardError $errFile
    $timedOut = -not $process.WaitForExit($Seconds * 1000)
    if ($timedOut) { try { $process.Kill() } catch { } }
    $out = (Get-Content $outFile -Raw -ErrorAction SilentlyContinue)
    $err = (Get-Content $errFile -Raw -ErrorAction SilentlyContinue)
    $text = ("$out" + $err).Trim()
    Write-Output "=== $name" + $(if ($timedOut) { "  [dialog: killed after $Seconds s]" } else { "" })
    if ($text -eq "") {
        Write-Output "   (no text on stdout or stderr)"
    } else {
        Write-Output ("   " + ($text -replace "`r?`n", " | "))
    }
}

Write-Output "# the command style in an expression"
Measure-Ahk "read_command_style" @'
try {
    text := FileRead "hello.txt"
    FileAppend "value=" text, "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "exist_command_style" @'
try {
    r := FileExist "hello.txt"
    FileAppend "value=" r, "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Write-Output "# the overwrite flag of FileInstall"
Measure-Ahk "fileinstall_overwrite" @'
try {
    FileInstall "hello.txt", "copy.txt", 1
    FileInstall "hello.txt", "copy.txt", 1
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.Wait, "*"
}
'@

Measure-Ahk "fileinstall_no_overwrite" @'
try {
    FileInstall "hello.txt", "copy2.txt", 0
    FileInstall "hello.txt", "copy2.txt", 0
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Write-Output "# the wildcards of FileSetTime and FileSetAttrib"
Measure-Ahk "filesettime_wildcard" @'
try {
    FileSetTime "20000101000000", "*.txt"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "filesettime_pattern_matches_nothing" @'
try {
    FileSetTime "20000101000000", "*.nothing"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "filesettime_mode_r" @'
try {
    FileSetTime "20000101000000", "*.txt", "R"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "filesetattrib_wildcard" @'
try {
    FileSetAttrib "+H", "*.txt"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Write-Output "# the copy and the move of a folder"
Measure-Ahk "dircopy_plain" @'
try {
    DirCopy "src", "dest"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "dircopy_overwrite" @'
try {
    DirCopy "src2", "dest", 1
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "dircopy_wildcard" @'
try {
    DirCopy "src3\*", "dest2"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "dirmove_wildcard" @'
try {
    DirMove "src\*", "dest3"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Write-Output "# the order of the parameters of FileSetTime and IniWrite"
Measure-Ahk "filesettime_name_first" @'
try {
    FileSetTime "hello.txt", "20000101000000"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "iniwrite_value_first" @'
try {
    IniWrite "1", "out.ini", "sec", "key"
    FileAppend "done", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Write-Output "# the taking away of a folder (last: it destroys what it names)"
Measure-Ahk "dirdelete_returns" @'
try {
    r := DirDelete("empty")
    FileAppend "value=[" r "]", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "dirdelete_not_empty" @'
try {
    r := DirDelete("src")
    FileAppend "value=[" r "]", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@

Measure-Ahk "dirdelete_recurse" @'
try {
    r := DirDelete("src2", 1)
    FileAppend "value=[" r "]", "*"
} catch as e {
    FileAppend "caught: " e.Message " | what=" e.What, "*"
}
'@
