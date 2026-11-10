$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path
$covered = @('WinGetText', 'ControlGetText', 'ControlSetText', 'ControlGetFocus', 'ControlFocus', 'Control')
$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7 -and $f[5] -eq 'GAP-accessibility') {
        $v1 = $f[1]
        if ($covered -contains $v1) {
            $f[4] = 'value'
            $f[5] = 'covered'
            $f[6] = 'the control family of rounds 30 to 34 and the window text of round 216'
            $line = $f -join "`t"
            $changed = $changed + 1
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
$after | Select-Object -Skip 1 | ForEach-Object { ($_ -split "`t")[5] } | Group-Object | Sort-Object Count -Descending | Select-Object -First 6 | ForEach-Object { "$($_.Count) x [$($_.Name)]" }
