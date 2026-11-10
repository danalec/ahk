$path = 'crates\ahk\src\builtins.rs'
$text = Get-Content $path -Raw

# name -> (handler, variant). variant 0 = an answer; 100 = a refusal.
$rows = @(
    @{ name = 'SysGet';              handler = 'sys_get';              variant = 0 },
    @{ name = 'ToolTip';             handler = 'tooltip';              variant = 0 },
    @{ name = 'WinMove';             handler = 'win_move';             variant = 0 },
    @{ name = 'ControlSend';         handler = 'control_send';         variant = 0 },
    @{ name = 'RunAs';               handler = 'run_as';               variant = 0 },
    @{ name = 'BlockInput';          handler = 'block_input';          variant = 0 },
    @{ name = 'OnClipboardChange';   handler = 'on_clipboard_change';  variant = 100 },
    @{ name = 'CallbackCreate';      handler = 'callback_create';      variant = 0 },
    @{ name = 'Shutdown';            handler = 'shutdown';             variant = 100 },
    @{ name = 'OnMessage';           handler = 'on_message';           variant = 100 },
    @{ name = 'ControlGetPos';       handler = 'control_get_pos';      variant = 100 },
    @{ name = 'WinActivateBottom';   handler = 'all_windows';          variant = 100 },
    @{ name = 'WinMinimizeAll';      handler = 'all_windows';          variant = 100 },
    @{ name = 'WinMinimizeAllUndo';  handler = 'all_windows';          variant = 100 }
)

foreach ($row in $rows) {
    $marker = '        name: "' + $row.name + '",'
    $at = $text.IndexOf($marker)
    if ($at -lt 0) { Write-Output "$($row.name): NO MARKER"; continue }
    $before = $text.Substring(0, $at)
    $after = $text.Substring($at)
    # The fields of a row are: variant, parameters, handler. Replace all three
    # in ONE anchored match, so no "next occurrence" can land on a neighbour.
    $pattern = '(?s)^(.*?variant: )\d+(,\s*parameters: [^,]+,\s*handler: )\w+(,)'
    $replacement = '${1}' + $row.variant + '${2}' + $row.handler + '${3}'
    $after2 = [regex]::Replace($after, $pattern, $replacement, 1)
    if ($after2 -eq $after) { Write-Output "$($row.name): NO MATCH"; continue }
    $text = $before + $after2
    Write-Output "$($row.name): set $($row.handler) / $($row.variant)"
}
$text | Set-Content $path -NoNewline
