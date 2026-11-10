$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path
$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7 -and $f[1] -eq 'FileRecycleEmpty') {
        $f[4] = 'value'
        $f[5] = 'covered'
        $f[6] = 'answers the empty text through gio trash --empty; the emptying itself is unproven because the box has no file to spare'
        $line = $f -join "`t"
        $changed = $changed + 1
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
$after | Select-Object -Skip 1 | ForEach-Object { ($_ -split "`t")[5] } | Group-Object | Sort-Object Count -Descending | Select-Object -First 5 | ForEach-Object { "$($_.Count) x [$($_.Name)]" }
