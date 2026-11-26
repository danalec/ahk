# the probes of a script of the v1, under the v1 and under the tree
#
# The twin of `scripts/v1-probes.sh`, for the machines of Windows that
# have no bash.  Every probe of `v1-probes/` is run under the binary of
# AutoHotkey v1.1.37.02 and under the tree, and the answers of the two
# are put side by side.  The probe that answers the same under the two is
# the one that measures the v1 right.
#
#   powershell -File scripts/v1-probes.ps1              # every probe
#   powershell -File scripts/v1-probes.ps1 probe-menu   # one probe
#
# The tree is built with `cargo build --release` first, and the binary of
# the v1 is at the place below (the path may be given as the first
# argument of the script).

param(
    [Parameter(Position = 0, ValueFromRemainingArguments = $true)][string[]]$Probes = @(),
    [string]$V1 = 'C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe',
    [string]$Tree = ''
)

$ErrorActionPreference = 'Stop'
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
$probesDir = Join-Path $here 'v1-probes'
if (-not $Tree) {
    $Tree = Join-Path (Split-Path -Parent $here) 'target\release\ahk.exe'
}
if (-not (Test-Path $V1)) { throw "the binary of the v1 is not there: $V1" }
if (-not (Test-Path $Tree)) { throw "the tree is not there: $Tree" }

if ($Probes.Count -eq 0) {
    $Probes = @(
        'probe-menu', 'probe-timer', 'probe-catch', 'probe-hwnd2', 'probe-rcb',
        'probe-rcb2', 'probe-icon', 'probe-dllstr', 'probe-base',
        'probe-pointer', 'probe-dyn', 'probe-chain', 'probe-hook', 'probe-guiev'
    )
}

function Invoke-Probe {
    param([string]$Probe, [string]$Exe)
    $script = Join-Path $probesDir "$Probe.ahk"
    $answer = Join-Path $probesDir "$Probe.txt"
    if (Test-Path $answer) { [System.IO.File]::WriteAllText($answer, '') }
    $arguments = if ($Exe -like '*AutoHotkey*') { @($script) } else { @('--dialect=v1', $script) }
    $process = Start-Process -FilePath $Exe -ArgumentList $arguments -WorkingDirectory $probesDir -PassThru
    Start-Sleep -Seconds 6
    $text = ''
    if (Test-Path $answer) { $text = [System.IO.File]::ReadAllText($answer) }
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 700
    return $text
}

foreach ($probe in $Probes) {
    $a = Invoke-Probe -Probe $probe -Exe $V1
    $b = Invoke-Probe -Probe $probe -Exe $Tree
    if ($a -eq '' -or $b -eq '') {
        # an empty answer is not an agreement: the probe failed under one of
        # the two, and the two must not be called the same
        Write-Output "ERROR $probe  (the answer of the v1 is empty: $($a -eq ''), the answer of the tree is empty: $($b -eq ''))"
        if ($a -ne '') { Write-Output "--- the v1 ---"; Write-Output $a }
        if ($b -ne '') { Write-Output "--- the tree ---"; Write-Output $b }
    } elseif ($a -match 'the pointer stayed where it was put = NO' -or $b -match 'the pointer stayed where it was put = NO') {
        Write-Output "UNSTABLE $probe  (the pointer of the machine did not stay where it was put)"
        Write-Output "--- the v1 ---"
        Write-Output $a
        Write-Output "--- the tree ---"
        Write-Output $b
    } elseif ($a -eq $b) {
        Write-Output "OK    $probe"
    } else {
        Write-Output "DIFF  $probe"
        Write-Output "--- the v1 ---"
        Write-Output $a
        Write-Output "--- the tree ---"
        Write-Output $b
    }
}
