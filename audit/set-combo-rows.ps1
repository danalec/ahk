$path = 'crates\ahk\src\builtins.rs'
$text = Get-Content $path -Raw
$rows = @(
    @{ name = 'ControlGetItems';     handler = 'control_get_items' },
    @{ name = 'ControlAddItem';      handler = 'control_add_item' },
    @{ name = 'ControlChooseIndex';  handler = 'control_choose_index' },
    @{ name = 'ControlChooseString'; handler = 'control_choose_string' },
    @{ name = 'ControlGetChoice';    handler = 'control_get_choice' },
    @{ name = 'ControlGetIndex';     handler = 'control_get_index' },
    @{ name = 'ControlDeleteItem';   handler = 'control_delete_item' },
    @{ name = 'ControlFindItem';     handler = 'control_find_item' }
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
