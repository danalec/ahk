$path = 'crates\ahk\src\builtins.rs'
$text = Get-Content $path -Raw

# name -> variant (100 + the index into refusal_message)
$groups = @{
    124 = @('ComCall','ComObjActive','ComObjConnect','ComObjFlags','ComObjFromPtr','ComObjGet','ComObjQuery','ComObjType','ComObjValue')
    125 = @('IL_Add','IL_Create','IL_Destroy')
    126 = @('PostMessage','SendMessage','WinSetRegion')
    127 = @('ControlAddItem','ControlChooseIndex','ControlChooseString','ControlDeleteItem','ControlFindItem','ControlGetChoice','ControlGetExStyle','ControlGetIndex','ControlGetItems','ControlGetStyle','ControlHideDropDown','ControlSetExStyle','ControlSetStyle','ControlShowDropDown','ListViewGetContent','StatusBarGetText','StatusBarWait')
    128 = @('DriveEject','DriveGetLabel','DriveGetList','DriveGetSerial','DriveGetStatusCD','DriveLock','DriveRetract','DriveSetLabel','DriveUnlock')
    129 = @('GroupActivate','GroupAdd','GroupClose','GroupDeactivate')
    130 = @('GuiCtrlFromHwnd','GuiFromHwnd','MenuFromHandle','MenuSelect','WinGetIDLast')
    131 = @('SetCapsLockState','SetNumLockState','SetScrollLockState')
    132 = @('CaretGetPos','FileGetShortcut','SoundGetInterface','TraySetIcon','CallbackFree')
    133 = @('StrGet','StrPtr')
}

$set = 0
foreach ($variant in $groups.Keys) {
    foreach ($name in $groups[$variant]) {
        $marker = '        name: "' + $name + '",'
        $at = $text.IndexOf($marker)
        if ($at -lt 0) { Write-Output "$name : NO MARKER"; continue }
        $before = $text.Substring(0, $at)
        $after = $text.Substring($at)
        # Replace variant in ONE anchored match on this row only.
        $after2 = [regex]::Replace($after, '(?s)^(.*?variant: )\d+(,)', '${1}' + $variant + '${2}', 1)
        if ($after2 -eq $after) { Write-Output "$name : NO MATCH"; continue }
        $text = $before + $after2
        $set = $set + 1
    }
}
$text | Set-Content $path -NoNewline
Write-Output "rows set: $set"
