$path = 'crates\ahk\src\builtins.rs'
$text = Get-Content $path -Raw
$rows = @(
    @{ name = 'GroupAdd';        handler = 'group_add' },
    @{ name = 'GroupActivate';   handler = 'group_activate' },
    @{ name = 'GroupDeactivate'; handler = 'group_close' },
    @{ name = 'GroupClose';      handler = 'group_close' }
)
foreach ($row in $rows) {
    $marker = '        name: "' + $row.name + '",'
    $at = $text.IndexOf($marker)
    if ($at -lt 0) { Write-Output "$($row.name): NO MARKER"; continue }
    $before = $text.Substring(0, $at)
    $after = $text.Substring($at)
    $pattern = '(?s)^(.*?variant: )\d+(,\s*parameters: [^,]+,\s*handler: )\w+(,)'
    $replacement = '${1}0${2}' + $row.handler + '${3}'
    $after2 = [regex]::Replace($after, $pattern, $replacement, 1)
    if ($after2 -eq $after) { Write-Output "$($row.name): NO MATCH"; continue }
    $text = $before + $after2
    Write-Output "$($row.name): set $($row.handler) / 0"
}
$text | Set-Content $path -NoNewline
