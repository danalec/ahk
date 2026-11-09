# Run the probe scripts through the release runner and capture the report.
# The source of each probe is a file under probes/, and the report it
# prints is written beside it as <name>.json. A probe that does not exit
# inside the hard timeout is killed and reported as KILLED, because a
# dialog or a blocked call would otherwise hang the audit.
param(
    [Parameter(Mandatory = $true)][string[]]$Names,
    [int]$TimeoutSec = 5
)

$ErrorActionPreference = 'Stop'
$bin = Join-Path (Get-Location) 'target\release\ahk.exe'
$dir = Join-Path (Get-Location) 'audit\empirical\probes'

foreach ($n in $Names) {
    $src = Join-Path $dir "$n.ahk"
    $out = Join-Path $dir "$n.json"
    $err = Join-Path $dir "$n.err.txt"
    if (-not (Test-Path $src)) { "MISSING $n"; continue }
    $p = Start-Process -FilePath $bin `
        -ArgumentList "--timeout=$TimeoutSec" `
        -RedirectStandardInput $src `
        -RedirectStandardOutput $out `
        -RedirectStandardError $err `
        -NoNewWindow -PassThru
    $exitTimeout = ($TimeoutSec + 20) * 1000
    if (-not $p.WaitForExit($exitTimeout)) {
        try { $p.Kill($true) } catch { }
        "KILLED  $n  (no exit inside $($exitTimeout)ms)"
        continue
    }
    $errLen = 0
    if (Test-Path $err) { $errLen = (Get-Item $err).Length }
    $outLen = 0
    if (Test-Path $out) { $outLen = (Get-Item $out).Length }
    "exit=$($p.ExitCode)  $n  json=$outLen bytes  stderr=$errLen bytes"
}
