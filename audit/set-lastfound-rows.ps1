$path = 'crates\ahk\src\builtins.rs'
$text = Get-Content $path -Raw
$rows = @(
    @{ name = 'WinGetIDLast';    handler = 'win_get_id_last'; variant = 0 },
    @{ name = 'GuiFromHwnd';     handler = 'from_hwnd';       variant = 130 },
    @{ name = 'GuiCtrlFromHwnd'; handler = 'from_hwnd';       variant = 130 },
    @{ name = 'MenuFromHandle';  handler = 'from_hwnd';       variant = 130 },
    @{ name = 'MenuSelect';      handler = 'from_hwnd';       variant = 130 }
)
foreach ($row in $rows) {
    $marker = '        name: "' + $row.name + '",'
    $at = $text.IndexOf($marker)
    if ($at -lt 0) { Write-Output "$($row.name): NO MARKER"; continue }
    $before = $text.Substring(0, $at)
    $after = $text.Substring($at)
    $pattern = '(?s)^(.*?variant: )\d+(,\s*parameters: [^,]+,\s*handler: )\w+(,)'
    $replacement = '${1}' + $row.variant + '${2}' + $row.handler + '${3}'
    $after2 = [regex]::Replace($after, $pattern, $replacement, 1)
    if ($after2 -eq $after) { Write-Output "$($row.name): NO MATCH"; continue }
    $text = $before + $after2
    Write-Output "$($row.name): set $($row.handler) / $($row.variant)"
}
$text | Set-Content $path -NoNewline
