$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path
$answered = @(
    'WinActivate', 'WinClose', 'WinKill', 'WinMaximize', 'WinMinimize',
    'WinRestore', 'WinSetTitle', 'WinWait', 'WinWaitActive', 'WinWaitClose',
    'WinWaitNotActive'
)
$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7 -and $f[5] -eq 'GAP-windows-act') {
        $v1 = $f[1]
        $isAnswered = ($answered -contains $v1) -or ($v1 -eq 'WinHide/WinShow')
        if ($isAnswered) {
            $f[4] = 'value'
            $f[5] = 'covered'
            $f[6] = 'answered on the script own window, rounds 193 to 199'
            $line = $f -join "`t"
            $changed = $changed + 1
        }
    }
    $null = $out.Add($line)
}
($out -join "`r`n") + "`r`n" | Set-Content $path
Write-Output "rows changed: $changed"
$after = Get-Content $path
$after | Select-Object -Skip 1 | ForEach-Object { ($_ -split "`t")[5] } | Group-Object | Sort-Object Count -Descending | Select-Object -First 5 | ForEach-Object { "$($_.Count) x [$($_.Name)]" }

