$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path
$covered = @('DriveSpaceFree', '#Persistent')
$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7) {
        $v1 = $f[1]
        $isCovered = ($covered -contains $v1)
        if (-not $isCovered -and $f[5] -eq 'GAP-machine-metrics' -and $f[2] -like 'MonitorGet*') {
            $isCovered = $true
        }
        if ($isCovered -and ($f[5] -eq 'GAP-machine-metrics' -or $f[5] -eq 'lifecycle')) {
            $f[4] = 'value'
            $f[5] = 'covered'
            $f[6] = 'the drives of round 136, the monitors of round 55, and Persistent from the quick wins'
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
