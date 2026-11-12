# Hotkeys, hotstrings, and input

The names of the family **Hotkeys, hotstrings, and input** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `Hotkey.__New` | v2 | method | Hotkey | -- | value |
| `InputHook.__New` | v2 | method | InputHook | -- | value |
| `A_CaretX` | v1 | variable | -- | -- | value |
| `A_CaretY` | v1 | variable | -- | -- | value |
| `A_ClipboardTimeout` | ks | variable | -- | -- | value |
| `A_CoordModeCaret` | v1 | variable | -- | -- | value |
| `A_CoordModeMenu` | v1 | variable | -- | -- | value |
| `A_CoordModeMouse` | v1 | variable | -- | -- | value |
| `A_CoordModePixel` | v1 | variable | -- | -- | value |
| `A_CoordModeToolTip` | v1 | variable | -- | -- | value |
| `A_Cursor` | v1 | variable | -- | -- | refused: the shape of the cursor of the machine is not read by this runtime. |
| `A_DefaultHotstringCaseSensitive` | ks | variable | -- | -- | value |
| `A_DefaultHotstringConformToCase` | ks | variable | -- | -- | value |
| `A_DefaultHotstringDetectWhenInsideWord` | ks | variable | -- | -- | value |
| `A_DefaultHotstringDoBackspace` | ks | variable | -- | -- | value |
| `A_DefaultHotstringDoReset` | ks | variable | -- | -- | value |
| `A_DefaultHotstringEndCharRequired` | ks | variable | -- | -- | value |
| `A_DefaultHotstringEndChars` | ks | variable | -- | -- | value |
| `A_DefaultHotstringKeyDelay` | ks | variable | -- | -- | value |
| `A_DefaultHotstringNoMouse` | ks | variable | -- | -- | value |
| `A_DefaultHotstringOmitEndChar` | ks | variable | -- | -- | value |
| `A_DefaultHotstringPriority` | ks | variable | -- | -- | value |
| `A_DefaultHotstringSendMode` | ks | variable | -- | -- | value |
| `A_DefaultHotstringSendRaw` | ks | variable | -- | -- | value |
| `A_HotIf` | ks | variable | -- | -- | value |
| `A_HotIfTimeout` | ks | variable | -- | -- | value |
| `A_KeyDelay` | v1 | variable | -- | -- | value |
| `A_KeyDelayPlay` | v1 | variable | -- | -- | value |
| `A_KeyDuration` | v1 | variable | -- | -- | value |
| `A_KeyDurationPlay` | v1 | variable | -- | -- | value |
| `A_MouseDelay` | v1 | variable | -- | -- | value |
| `A_MouseDelayPlay` | v1 | variable | -- | -- | value |
| `A_PeekFrequency` | ks | variable | -- | -- | value |
| `A_PriorHotkey` | v1 | variable | -- | -- | value |
| `A_PriorKey` | v1 | variable | -- | -- | value |
| `A_SendLevel` | v1 | variable | -- | -- | value |
| `A_SendMode` | v1 | variable | -- | -- | value |
| `A_StoreCapsLockMode` | v1 | variable | -- | -- | value |
| `A_ThisHotkey` | v1 | variable | -- | -- | value |
| `A_TimeIdle` | v1 | variable | -- | -- | value |
| `A_TimeIdleKeyboard` | v1 | variable | -- | -- | value |
| `A_TimeIdleMouse` | v1 | variable | -- | -- | value |
| `A_TimeIdlePhysical` | v1 | variable | -- | -- | value |
| `A_TimeSincePriorHotkey` | v1 | variable | -- | -- | value |
| `A_TimeSinceThisHotkey` | v1 | variable | -- | -- | value |
| `Hotkey.BackspaceIsUndo` | v2 | method | Hotkey | -- | value |
| `InputHook.BackspaceIsUndo` | v2 | method | InputHook | -- | value |
| `BlockInput` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `BlockInput` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `InputHook.BufferLengthMax` | ks | method | InputHook | -- | value |
| `CallbackCreate` | v2 | command | -- | 1 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `CallbackFree` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Hotkey.CaseSensitive` | v2 | method | Hotkey | -- | value |
| `InputHook.CaseSensitive` | v2 | method | InputHook | -- | value |
| `Click` | v2 | function | -- | 0 to 6 | value |
| `Click` | v1 | command | -- | up to 0 | value |
| `ClipCursor` | ks | function | -- | -- | value |
| `ClipWait` | v2 | command | -- | 0 to 2 | value |
| `ClipWait` | v1 | command | -- | up to 0 | value |
| `ControlGet` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `CoordModeMouse` | v2 | variable | -- | -- | value |
| `CoordModePixel` | v2 | variable | -- | -- | value |
| `Cursor` | v2 | variable | -- | -- | refused: the shape of the cursor of the machine is not read by this runtime. |
| `DefaultMouseSpeed` | v2 | variable | -- | -- | value |
| `InputHook.EndCharMode` | ks | method | InputHook | -- | value |
| `Hotkey.EndKey` | v2 | method | Hotkey | -- | value |
| `InputHook.EndKey` | v2 | method | InputHook | -- | value |
| `Hotkey.EndMods` | v2 | method | Hotkey | -- | value |
| `InputHook.EndMods` | v2 | method | InputHook | -- | value |
| `Hotkey.EndReason` | v2 | method | Hotkey | -- | value |
| `InputHook.EndReason` | v2 | method | InputHook | -- | value |
| `EventInfo` | v2 | variable | -- | -- | value |
| `Hotkey.FindAnywhere` | v2 | method | Hotkey | -- | value |
| `InputHook.FindAnywhere` | v2 | method | InputHook | -- | value |
| `GetKeyboardLayout` | ks | function | -- | -- | value |
| `GetKeyInfo` | ks | function | -- | -- | value |
| `GetKeyName` | v2 | command | -- | 1 | value |
| `GetKeySC` | v2 | command | -- | 1 | value |
| `GetKeyState` | v2 | command | -- | 1 to 2 | value |
| `GetKeyState` | v1 | command | -- | up to 2 | value |
| `GetKeyVK` | v2 | command | -- | 1 | value |
| `HotIf` | v2 | command | -- | 0 to 1 | value |
| `HotIfWinActive` | v2 | command | -- | 0 to 2 | value |
| `HotIfWinExist` | v2 | command | -- | 0 to 2 | value |
| `HotIfWinNotActive` | v2 | command | -- | 0 to 2 | value |
| `HotIfWinNotExist` | v2 | command | -- | 0 to 2 | value |
| `Hotkey` | v2 | command | -- | 1 to 3 | value |
| `Hotkey` | v1 | command | -- | up to 1 | value |
| `HotkeyInterval` | v2 | variable | -- | -- | value |
| `HotkeyModifierTimeout` | v2 | variable | -- | -- | value |
| `Hotstring` | v2 | command | -- | 1 to 3 | value |
| `Hotstring` | v1 | function | -- | 1 to 3 | value |
| `Hotkey.InProgress` | v2 | method | Hotkey | -- | value |
| `InputHook.InProgress` | v2 | method | InputHook | -- | value |
| `Input` | v1 | command | -- | up to 0 | value |
| `Hotkey.Input` | v2 | method | Hotkey | -- | value |
| `InputHook.Input` | v2 | method | InputHook | -- | value |
| `InputHook` | v2 | class | -- | -- | value |
| `InputHook` | v1 | function | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `InstallKeybdHook` | v2 | command | -- | 0 to 2 | value |
| `InstallMouseHook` | v2 | command | -- | 0 to 2 | value |
| `IsLabel` | v2 | command | -- | 1 | value |
| `IsLabel` | v1 | function | -- | 1 | value |
| `KeyDelay` | v2 | variable | -- | -- | value |
| `KeyDelayPlay` | v2 | variable | -- | -- | value |
| `KeyDuration` | v2 | variable | -- | -- | value |
| `KeyDurationPlay` | v2 | variable | -- | -- | value |
| `KeyHistory` | v2 | command | -- | 0 to 1 | value |
| `KeyHistory` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Hotkey.KeyOpt` | v2 | method | Hotkey | -- | value |
| `InputHook.KeyOpt` | v2 | method | InputHook | -- | value |
| `KeyWait` | v2 | command | -- | 1 to 2 | value |
| `KeyWait` | v1 | command | -- | up to 1 | value |
| `ListHotkeys` | v2 | command | -- | 0 | value |
| `ListHotkeys` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Hotkey.Match` | v2 | method | Hotkey | -- | value |
| `InputHook.Match` | v2 | method | InputHook | -- | value |
| `MaxHotkeysPerInterval` | v2 | variable | -- | -- | value |
| `MenuMaskKey` | v2 | variable | -- | -- | value |
| `Hotkey.MinSendLevel` | v2 | method | Hotkey | -- | value |
| `InputHook.MinSendLevel` | v2 | method | InputHook | -- | value |
| `MouseClick` | v2 | command | -- | 0 to 7 | value |
| `MouseClick` | v1 | command | -- | up to 0 | value |
| `MouseClickDrag` | v2 | command | -- | 0 to 7 | value |
| `MouseClickDrag` | v1 | command | -- | up to 1 | value |
| `MouseDelay` | v2 | variable | -- | -- | value |
| `MouseDelayPlay` | v2 | variable | -- | -- | value |
| `MouseGetPos` | v2 | command | -- | 0 to 5 | value |
| `MouseGetPos` | v1 | command | -- | up to 0 | value |
| `MouseMove` | v2 | command | -- | 2 to 4 | value |
| `MouseMove` | v1 | command | -- | up to 2 | value |
| `Hotkey.NotifyNonText` | v2 | method | Hotkey | -- | value |
| `InputHook.NotifyNonText` | v2 | method | InputHook | -- | value |
| `Hotkey.OnChar` | v2 | method | Hotkey | -- | value |
| `InputHook.OnChar` | v2 | method | InputHook | -- | value |
| `Hotkey.OnEnd` | v2 | method | Hotkey | -- | value |
| `InputHook.OnEnd` | v2 | method | InputHook | -- | value |
| `Hotkey.OnKeyDown` | v2 | method | Hotkey | -- | value |
| `InputHook.OnKeyDown` | v2 | method | InputHook | -- | value |
| `Hotkey.OnKeyUp` | v2 | method | Hotkey | -- | value |
| `InputHook.OnKeyUp` | v2 | method | InputHook | -- | value |
| `InputHook.OnMouseDown` | ks | method | InputHook | -- | value |
| `InputHook.OnMouseMove` | ks | method | InputHook | -- | value |
| `InputHook.OnMouseUp` | ks | method | InputHook | -- | value |
| `PriorHotkey` | v2 | variable | -- | -- | value |
| `PriorKey` | v2 | variable | -- | -- | value |
| `Send` | v2 | command | -- | 1 | value |
| `Send` | v1 | command | -- | up to 1 | value |
| `SendEvent` | v2 | command | -- | 1 | value |
| `SendInput` | v2 | command | -- | 1 | value |
| `SendLevel` | v2 | command | -- | 1 | value |
| `SendLevel` | v1 | command | -- | up to 1 | value |
| `SendMode` | v2 | command | -- | 1 | value |
| `SendMode` | v1 | command | -- | up to 1 | value |
| `SendPlay` | v2 | command | -- | 1 | value |
| `SendText` | v2 | command | -- | 1 | value |
| `SetCapsLockState` | v2 | command | -- | 0 to 1 | value |
| `SetDefaultMouseSpeed` | v2 | command | -- | 1 | value |
| `SetDefaultMouseSpeed` | v1 | command | -- | up to 1 | value |
| `SetKeyDelay` | v2 | command | -- | 0 to 3 | value |
| `SetKeyDelay` | v1 | command | -- | up to 0 | value |
| `SetMouseDelay` | v2 | command | -- | 1 to 2 | value |
| `SetMouseDelay` | v1 | command | -- | up to 1 | value |
| `SetNumLockState` | v2 | command | -- | 0 to 1 | value |
| `SetScrollLockState` | v2 | command | -- | 0 to 1 | value |
| `SetStoreCapsLockMode` | v2 | command | -- | 1 | value |
| `SetStoreCapsLockMode` | v1 | command | -- | up to 1 | value |
| `Hotkey.Start` | v2 | method | Hotkey | -- | value |
| `InputHook.Start` | v2 | method | InputHook | -- | value |
| `Hotkey.Stop` | v2 | method | Hotkey | -- | value |
| `InputHook.Stop` | v2 | method | InputHook | -- | value |
| `StoreCapsLockMode` | v2 | variable | -- | -- | value |
| `ThisHotkey` | v2 | variable | -- | -- | value |
| `TimeIdle` | v2 | variable | -- | -- | value |
| `TimeIdleKeyboard` | v2 | variable | -- | -- | value |
| `TimeIdleMouse` | v2 | variable | -- | -- | value |
| `TimeIdlePhysical` | v2 | variable | -- | -- | value |
| `Hotkey.Timeout` | v2 | method | Hotkey | -- | value |
| `InputHook.Timeout` | v2 | method | InputHook | -- | value |
| `TimeSincePriorHotkey` | v2 | variable | -- | -- | value |
| `TimeSinceThisHotkey` | v2 | variable | -- | -- | value |
| `InputHook.TranscribeModifiedKeys` | ks | method | InputHook | -- | value |
| `InputHook.VisibleMouseMove` | ks | method | InputHook | -- | value |
| `Hotkey.VisibleNonText` | v2 | method | Hotkey | -- | value |
| `InputHook.VisibleNonText` | v2 | method | InputHook | -- | value |
| `Hotkey.VisibleText` | v2 | method | Hotkey | -- | value |
| `InputHook.VisibleText` | v2 | method | InputHook | -- | value |
| `Hotkey.Wait` | v2 | method | Hotkey | -- | value |
| `InputHook.Wait` | v2 | method | InputHook | -- | value |
