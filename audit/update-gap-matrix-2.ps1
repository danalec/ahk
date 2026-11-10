$path = 'audit\gap-matrix.tsv'
$lines = Get-Content $path
$covered = @('Gui', 'GuiControl', 'Menu', 'ControlSendRaw', 'ControlClick')
$out = New-Object System.Collections.ArrayList
$changed = 0
foreach ($line in $lines) {
    $f = $line -split "`t"
    if ($f.Count -ge 7 -and ($f[5] -eq 'GAP-gui-toolkit' -or $f[5] -eq 'GAP-controls')) {
        $v1 = $f[1]
        if ($covered -contains $v1) {
            $f[4] = 'value'
            $f[5] = 'covered'
            $f[6] = 'the gui capability of rounds 70 to 132, or the control family of rounds 30 to 34'
            $line = $f -join "`t"
            $changed = $changed + 1
        }
    }
    $null = $out.Add($line)
}
($out -join "`r`n") + "`r`n" | Set-Content $path
Write-Output "rows changed: $changed"
$after = Get-Content $path
$after | Select-Object -Skip 1 | ForEach-Object { ($_ -split "`t")[5] } | Group-Object | Sort-Object Count -Descending | Select-Object -First 6 | ForEach-Object { "$($_.Count) x [$($_.Name)]" }
