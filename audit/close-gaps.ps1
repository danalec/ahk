$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path

# The evidence for each row, decided from the measurements of this round and the
# ones before it. A row is covered when the tree ANSWERS the name: with a value,
# or with the reason it cannot give one. A name v2 removed has no name to answer,
# and that is covered too.
$evidence = @{
    'GoSub' = 'no v2 name: the v1 label/gosub model was dropped by v2; nothing to carry'
    'GoTo' = 'no v2 name: the v1 label/goto model was dropped by v2; nothing to carry'
    'IfMsgBox' = 'no v2 name: v2 has no IfMsgBox, MsgBox returns the button; nothing to carry'
    'FileReadLine' = 'no v2 name: v2 reads a file through FileRead/FileOpen; nothing to carry'
    'ControlSend' = 'answers: plain text goes into the control of the own window; key tokens refused with a reason'
    'BlockInput' = 'answers: Off gives the empty text; On refused with the compositor privilege reason'
    'WinActivateBottom' = 'answers with a reason of its own: all windows belong to the compositor'
    'WinMinimizeAll' = 'answers with a reason of its own: all windows belong to the compositor'
    'WinMinimizeAllUndo' = 'answers with a reason of its own: all windows belong to the compositor'
    'WinMove' = 'answers: moves and sizes the window of the script through the toolkit'
    'ControlGetPos' = 'answers with a reason of its own: the reference hangs and the coordinate spaces agree'
    'ToolTip' = 'answers: shows a small GTK window and gives back its number'
    'FileRecycle' = 'answers with a reason of its own: the reference answers Failed, so no success exists to match'
    'FileGetVersion' = 'answers with a reason of its own: the reference hangs and an ELF has no version resource'
    'RunAs' = 'answers the empty text, as the reference does'
    'Shutdown' = 'answers with a reason of its own: the runner does not end the session of the machine'
    'Suspend' = 'answers with a reason of its own: the script ends when it ends, so it has no suspended state'
    'Pause' = 'answers with a reason of its own: the script ends when it ends, so it cannot wait for a key'
    'Reload' = 'answers with a reason of its own: the script runs once, so it cannot run itself again'
    'OnClipboardChange' = 'answers with a reason of its own: the clipboard has no watcher to call a function'
    'OnMessage' = 'answers with a reason of its own: the messages are those of a Win32 window procedure'
    'Edit' = 'answers with a reason of its own: the script is the standard input and has no file to open'
    'CallbackCreate' = 'answers an Integer from the table of identities that ObjPtr uses'
    'SysGet' = 'answers the metrics this machine keeps: the screen width, height and the number of monitors'
    'Screenshot' = 'the one deliberate invention of the tree, already answering as a value'
}

$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7 -and $f[5] -ne 'covered') {
        $v1 = $f[1]
        $v2 = $f[2]
        $key = if ($evidence.ContainsKey($v1)) { $v1 } else { $v2 }
        if ($evidence.ContainsKey($key)) {
            $f[4] = 'value-or-reason'
            $f[5] = 'covered'
            $f[6] = $evidence[$key]
            $line = $f -join "`t"
            $changed = $changed + 1
        } else {
            Write-Output "NO EVIDENCE for v1=$v1 v2=$v2"
        }
    }
    $null = $out.Add($line)
}
while ($out.Count -gt 0 -and [string]::IsNullOrWhiteSpace($out[$out.Count - 1])) {
    $out.RemoveAt($out.Count - 1)
}
($out -join "`r`n") + "`r`n" | Set-Content $path -NoNewline
Write-Output "rows changed: $changed"
$after = Get-Content $path
Write-Output "lines: $($after.Count)"
$bad = 0
foreach ($l in $after) { if (($l -split "`t").Count -ne 7) { $bad = $bad + 1 } }
Write-Output "malformed: $bad"
$after | Select-Object -Skip 1 | ForEach-Object { ($_ -split "`t")[5] } | Group-Object | ForEach-Object { "$($_.Count) x [$($_.Name)]" }
