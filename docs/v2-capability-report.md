# The capability report of the surface of AutoHotkey v2.0.19

> **Generated.** The generator is `crates/ahk/reference/ahk-v2.0.19.areas.py` , and the state of every name is the one of the tree itself: `ahk --surface` asks the table of the built-ins, the table of the built-in variables and the classes, and answers with a value, with the reason of a refusal, or with the reason of a name to do.

The surface of the v2.0.19 has **721 entries** : **510 answer with a value** , **211 are refused** and **none is pending**.

The table has **721 entries and 685 distinct names** : a property with a setter is in the table twice --- the `Hotkey` class has eleven of them --- and a name that is both a command and a variable is two entries (`SendMode` , `SendLevel` , `Min` , `FileEncoding` , `ListLines` , `DetectHiddenText` , `DetectHiddenWindows` ). The numbers below are entries.

## The areas

| Area | Names | Answer | Refused |
|---|---|---|---|
| Parser and script execution | 36 | 29 | 7 |
| File and directory operations | 96 | 77 | 19 |
| Hotkeys, hotstrings, and input | 102 | 94 | 8 |
| GUI windows and window automation | 271 | 139 | 132 |
| Screen capture and pixel functions | 7 | 6 | 1 |
| Native function calls (DllCall) | 9 | 8 | 1 |
| Registry APIs | 11 | 4 | 7 |
| COM automation (ComObject) | 15 | 0 | 15 |
| The core of the language: text, math and patterns | 58 | 55 | 3 |
| The objects of the language: Array, Map and Func | 48 | 48 | 0 |
| Dates and times | 23 | 23 | 0 |
| The machine, the processes and the environment | 35 | 20 | 15 |
| The clipboard | 2 | 1 | 1 |
| Sound | 8 | 6 | 2 |
| **The whole of it** | **721** | **510** | **211** |

## What the numbers do not say

Three things, all measured on the binary of the v2.0.19 on 2026-10-29.

**The classes of the windows are carried, and every call of them fails.**
The names of `Gui` , of `Gui.Control` , of `Gui.ListView` , of
`Gui.StatusBar` and of `Menu` are in the tree, and the object is made ---
`myGui := Gui()` works --- but the thing itself is refused with the reason:
*"a window is a Windows window, and this desktop is a Wayland session without
one"*. The surface counts these names as answering, because the class
carries them; the call is what fails.

**The v2 has nine class types that the surface table does not list.**
`Menu` , `ComObject` and the seven error classes --- `TypeError` ,
`ValueError` , `OSError` , `TargetError` , `TimeoutError` , `PropertyError`
and `MethodError` --- are in the binary and are not in
`crates/ahk/reference/ahk-v2.0.19.surface.tsv` . The extraction read the
tables of the classes at the wrong place, the same way it read the table of
the `Buffer` and the table of the `Array` (the notes of the repository say
so). The tree carries the `Menu` --- it is the menu of a window, and it
refuses with the reason of the windows --- and it carries **none of the
other eight** : `ComObject("Shell.Application")` and `TypeError("boom")`
both fail with *"Call to nonexistent function."* The seven error classes
each carry the same six properties in the binary --- `Message` , `What` ,
`Extra` , `File` , `Line` and `Stack` --- and the tree carries none of them.

**The area of the parser is about the surface, not about the grammar.**
What a script may write is the work of the wave of 2026-10-29
(`docs/ahk-v1-dialect.md` says what the plan is) . This report is about the
names that the tree answers.

## 1. Parser and script execution

**36 names: 29 answer with a value, 7 are refused.**

**The ones that answer** (29): `AllowMainWindow`, `CoordMode`, `EndChar`, `EnvGet`, `EnvSet`, `Exit`, `ExitApp`, `Index`, `InitialWorkingDir`, `Is64bitOS`, `IsAdmin`, `IsCompiled`, `IsCritical`, `IsPaused`, `IsSuspended`, `LineFile`, `LineNumber`, `OnError`, `OnExit`, `ScriptDir`, `ScriptFullPath`, `ScriptName`, `SetTimer`, `SetWorkingDir`, `Sleep`, `Space`, `Tab`, `ThisFunc`, `WorkingDir`

**The ones that are refused** (7), by the reason:

- *this runtime runs one script in one thread, and it cannot make it critical.* (1): `Critical`
- *this is a name of Windows, and this desktop is not Windows.* (1): `OnMessage`
- *this runtime cannot wait for a key to continue: the script ends when it ends.* (1): `Pause`
- *this runtime is a script that ends when it ends, and it cannot be kept alive.* (1): `Persistent`
- *this runtime runs one script once, and it cannot run itself again.* (1): `Reload`
- *this runtime cannot stop itself: it would never end.* (1): `Suspend`
- *this runtime runs one script in one thread, and it cannot start another.* (1): `Thread`

## 2. File and directory operations

**96 names: 77 answer with a value, 19 are refused.**

**The ones that answer** (77): `DirCopy`, `DirCreate`, `DirDelete`, `DirExist`, `DirMove`, `DirSelect`, `Download`, `File.AtEOF`, `File.Close`, `File.Encoding`, `File.Encoding`, `File.Handle`, `File.Length`, `File.Length`, `File.Pos`, `File.Pos`, `File.RawRead`, `File.RawWrite`, `File.Read`, `File.ReadChar`, `File.ReadDouble`, `File.ReadFloat`, `File.ReadInt`, `File.ReadInt64`, `File.ReadLine`, `File.ReadShort`, `File.ReadUChar`, `File.ReadUInt`, `File.ReadUShort`, `File.Seek`, `File.Write`, `File.WriteChar`, `File.WriteDouble`, `File.WriteFloat`, `File.WriteInt`, `File.WriteInt64`, `File.WriteLine`, `File.WriteShort`, `File.WriteUChar`, `File.WriteUInt`, `File.WriteUShort`, `FileAppend`, `FileCopy`, `FileDelete`, `FileEncoding`, `FileEncoding`, `FileExist`, `FileGetAttrib`, `FileGetSize`, `FileGetTime`, `FileInstall`, `FileMove`, `FileOpen`, `FileRead`, `FileSelect`, `FileSetAttrib`, `FileSetTime`, `IniDelete`, `IniRead`, `IniWrite`, `LoopField`, `LoopFileAttrib`, `LoopFileDir`, `LoopFileExt`, `LoopFileFullPath`, `LoopFileName`, `LoopFilePath`, `LoopFileShortName`, `LoopFileShortPath`, `LoopFileSize`, `LoopFileSizeKB`, `LoopFileSizeMB`, `LoopFileTimeAccessed`, `LoopFileTimeCreated`, `LoopFileTimeModified`, `LoopReadLine`, `SplitPath`

**The ones that are refused** (19), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (19): `DriveEject`, `DriveGetCapacity`, `DriveGetFilesystem`, `DriveGetLabel`, `DriveGetList`, `DriveGetSerial`, `DriveGetSpaceFree`, `DriveGetStatus`, `DriveGetStatusCD`, `DriveGetType`, `DriveLock`, `DriveRetract`, `DriveSetLabel`, `DriveUnlock`, `FileCreateShortcut`, `FileGetShortcut`, `FileGetVersion`, `FileRecycle`, `FileRecycleEmpty`

## 3. Hotkeys, hotstrings, and input

**102 names: 94 answer with a value, 8 are refused.**

**The ones that answer** (94): `Click`, `ClipWait`, `CoordModeMouse`, `CoordModePixel`, `DefaultMouseSpeed`, `GetKeyName`, `GetKeySC`, `GetKeyState`, `GetKeyVK`, `HotIf`, `HotIfWinActive`, `HotIfWinExist`, `HotIfWinNotActive`, `HotIfWinNotExist`, `Hotkey`, `Hotkey.BackspaceIsUndo`, `Hotkey.BackspaceIsUndo`, `Hotkey.CaseSensitive`, `Hotkey.CaseSensitive`, `Hotkey.EndKey`, `Hotkey.EndMods`, `Hotkey.EndReason`, `Hotkey.FindAnywhere`, `Hotkey.FindAnywhere`, `Hotkey.InProgress`, `Hotkey.Input`, `Hotkey.KeyOpt`, `Hotkey.Match`, `Hotkey.MinSendLevel`, `Hotkey.MinSendLevel`, `Hotkey.NotifyNonText`, `Hotkey.NotifyNonText`, `Hotkey.OnChar`, `Hotkey.OnChar`, `Hotkey.OnEnd`, `Hotkey.OnEnd`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyUp`, `Hotkey.OnKeyUp`, `Hotkey.Start`, `Hotkey.Stop`, `Hotkey.Timeout`, `Hotkey.Timeout`, `Hotkey.VisibleNonText`, `Hotkey.VisibleNonText`, `Hotkey.VisibleText`, `Hotkey.VisibleText`, `Hotkey.Wait`, `Hotkey.__New`, `HotkeyInterval`, `HotkeyModifierTimeout`, `Hotstring`, `InstallKeybdHook`, `InstallMouseHook`, `IsLabel`, `KeyDelay`, `KeyDelayPlay`, `KeyDuration`, `KeyDurationPlay`, `KeyHistory`, `KeyWait`, `ListHotkeys`, `MaxHotkeysPerInterval`, `MenuMaskKey`, `MouseClick`, `MouseClickDrag`, `MouseDelay`, `MouseDelayPlay`, `MouseGetPos`, `MouseMove`, `PriorHotkey`, `PriorKey`, `Send`, `SendEvent`, `SendInput`, `SendLevel`, `SendLevel`, `SendMode`, `SendMode`, `SendPlay`, `SendText`, `SetDefaultMouseSpeed`, `SetKeyDelay`, `SetMouseDelay`, `SetStoreCapsLockMode`, `StoreCapsLockMode`, `ThisHotkey`, `TimeIdle`, `TimeIdleKeyboard`, `TimeIdleMouse`, `TimeIdlePhysical`, `TimeSincePriorHotkey`, `TimeSinceThisHotkey`

**The ones that are refused** (8), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (6): `BlockInput`, `CallbackCreate`, `CallbackFree`, `SetCapsLockState`, `SetNumLockState`, `SetScrollLockState`
- *this runtime does not change the cursor of the pointer.* (1): `Cursor`
- *this runtime has no events that carry information: it has no wheel hotkeys and no OnMessage.* (1): `EventInfo`

## 4. GUI windows and window automation

**271 names: 139 answer with a value, 132 are refused.**

**The ones that answer** (139): `Gui.Add`, `Gui.AddActiveX`, `Gui.AddButton`, `Gui.AddCheckBox`, `Gui.AddComboBox`, `Gui.AddCustom`, `Gui.AddDDL`, `Gui.AddDateTime`, `Gui.AddDropDownList`, `Gui.AddEdit`, `Gui.AddGroupBox`, `Gui.AddHotkey`, `Gui.AddLink`, `Gui.AddListBox`, `Gui.AddListView`, `Gui.AddMonthCal`, `Gui.AddPic`, `Gui.AddPicture`, `Gui.AddProgress`, `Gui.AddRadio`, `Gui.AddSlider`, `Gui.AddStatusBar`, `Gui.AddTab`, `Gui.AddTab2`, `Gui.AddTab3`, `Gui.AddText`, `Gui.AddTreeView`, `Gui.AddUpDown`, `Gui.BackColor`, `Gui.BackColor`, `Gui.Control.Add`, `Gui.Control.Choose`, `Gui.Control.ClassNN`, `Gui.Control.Delete`, `Gui.Control.Enabled`, `Gui.Control.Enabled`, `Gui.Control.Focus`, `Gui.Control.Focused`, `Gui.Control.GetPos`, `Gui.Control.Gui`, `Gui.Control.Hwnd`, `Gui.Control.Move`, `Gui.Control.Name`, `Gui.Control.Name`, `Gui.Control.OnCommand`, `Gui.Control.OnEvent`, `Gui.Control.OnNotify`, `Gui.Control.Opt`, `Gui.Control.Redraw`, `Gui.Control.SetFont`, `Gui.Control.SetFormat`, `Gui.Control.Text`, `Gui.Control.Text`, `Gui.Control.Type`, `Gui.Control.UseTab`, `Gui.Control.Value`, `Gui.Control.Value`, `Gui.Control.Visible`, `Gui.Control.Visible`, `Gui.Destroy`, `Gui.Flash`, `Gui.FocusedCtrl`, `Gui.GetClientPos`, `Gui.GetPos`, `Gui.Hide`, `Gui.Hwnd`, `Gui.ListView.Add`, `Gui.ListView.Delete`, `Gui.ListView.Get`, `Gui.ListView.GetChild`, `Gui.ListView.GetCount`, `Gui.ListView.GetNext`, `Gui.ListView.GetParent`, `Gui.ListView.GetPrev`, `Gui.ListView.GetSelection`, `Gui.ListView.GetText`, `Gui.ListView.Modify`, `Gui.ListView.SetIcon`, `Gui.ListView.SetImageList`, `Gui.ListView.SetParts`, `Gui.ListView.SetText`, `Gui.MarginX`, `Gui.MarginX`, `Gui.MarginY`, `Gui.MarginY`, `Gui.Maximize`, `Gui.Menu.Add`, `Gui.Menu.AddStandard`, `Gui.Menu.Check`, `Gui.Menu.ClickCount`, `Gui.Menu.ClickCount`, `Gui.Menu.Default`, `Gui.Menu.Default`, `Gui.Menu.Delete`, `Gui.Menu.Disable`, `Gui.Menu.Enable`, `Gui.Menu.Handle`, `Gui.Menu.Insert`, `Gui.Menu.Rename`, `Gui.Menu.SetColor`, `Gui.Menu.SetIcon`, `Gui.Menu.Show`, `Gui.Menu.ToggleCheck`, `Gui.Menu.ToggleEnable`, `Gui.Menu.Uncheck`, `Gui.MenuBar`, `Gui.MenuBar`, `Gui.Minimize`, `Gui.Move`, `Gui.Name`, `Gui.Name`, `Gui.OnEvent`, `Gui.Opt`, `Gui.Restore`, `Gui.SetFont`, `Gui.Show`, `Gui.StatusBar.Add`, `Gui.StatusBar.Delete`, `Gui.StatusBar.DeleteCol`, `Gui.StatusBar.GetCount`, `Gui.StatusBar.GetNext`, `Gui.StatusBar.GetText`, `Gui.StatusBar.Insert`, `Gui.StatusBar.InsertCol`, `Gui.StatusBar.Modify`, `Gui.StatusBar.ModifyCol`, `Gui.StatusBar.SetImageList`, `Gui.Submit`, `Gui.Title`, `Gui.Title`, `Gui.__Enum`, `Gui.__Item`, `Gui.__New`, `InputBox`, `ListLines`, `MsgBox`, `WinActive`, `WinExist`, `WinGetTitle`

**The ones that are refused** (132), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (115): `CaretGetPos`, `ControlAddItem`, `ControlChooseIndex`, `ControlChooseString`, `ControlClick`, `ControlDeleteItem`, `ControlFindItem`, `ControlFocus`, `ControlGetChecked`, `ControlGetChoice`, `ControlGetClassNN`, `ControlGetEnabled`, `ControlGetExStyle`, `ControlGetFocus`, `ControlGetHwnd`, `ControlGetIndex`, `ControlGetItems`, `ControlGetPos`, `ControlGetStyle`, `ControlGetText`, `ControlGetVisible`, `ControlHide`, `ControlHideDropDown`, `ControlMove`, `ControlSend`, `ControlSendText`, `ControlSetChecked`, `ControlSetEnabled`, `ControlSetExStyle`, `ControlSetStyle`, `ControlSetText`, `ControlShow`, `ControlShowDropDown`, `DetectHiddenText`, `DetectHiddenWindows`, `Edit`, `EditGetCurrentCol`, `EditGetCurrentLine`, `EditGetLine`, `EditGetLineCount`, `EditGetSelectedText`, `EditPaste`, `GroupActivate`, `GroupAdd`, `GroupClose`, `GroupDeactivate`, `GuiCtrlFromHwnd`, `GuiFromHwnd`, `IL_Add`, `IL_Create`, `IL_Destroy`, `ListViewGetContent`, `MenuFromHandle`, `MenuSelect`, `MonitorGet`, `MonitorGetCount`, `MonitorGetName`, `MonitorGetPrimary`, `MonitorGetWorkArea`, `PostMessage`, `SendMessage`, `SetControlDelay`, `SetTitleMatchMode`, `SetWinDelay`, `StatusBarGetText`, `StatusBarWait`, `ToolTip`, `TraySetIcon`, `TrayTip`, `WinActivate`, `WinActivateBottom`, `WinClose`, `WinDir`, `WinGetClass`, `WinGetClientPos`, `WinGetControls`, `WinGetControlsHwnd`, `WinGetCount`, `WinGetExStyle`, `WinGetID`, `WinGetIDLast`, `WinGetList`, `WinGetMinMax`, `WinGetPID`, `WinGetPos`, `WinGetProcessName`, `WinGetProcessPath`, `WinGetStyle`, `WinGetText`, `WinGetTransColor`, `WinGetTransparent`, `WinHide`, `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`, `WinMinimizeAllUndo`, `WinMove`, `WinMoveBottom`, `WinMoveTop`, `WinRedraw`, `WinRestore`, `WinSetAlwaysOnTop`, `WinSetEnabled`, `WinSetExStyle`, `WinSetRegion`, `WinSetStyle`, `WinSetTitle`, `WinSetTransColor`, `WinSetTransparent`, `WinShow`, `WinWait`, `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`
- *this runtime has no tray icon.* (5): `IconFile`, `IconHidden`, `IconNumber`, `IconTip`, `TrayMenu`
- *the runtime keeps no such state yet.* (3): `ControlDelay`, `TitleMatchModeSpeed`, `WinDelay`
- *the lines are listed in a window of v2, and this runtime has no window to show them in.* (1): `ListLines`
- *the variables are listed in a window of v2, and this runtime has no window to show them in.* (1): `ListVars`
- *the caret belongs to a window, and this runtime has no windows: the mode of the caret is not answered.* (1): `CoordModeCaret`
- *the menu is a window of its own, and this runtime has no windows: the mode of the menu is not answered.* (1): `CoordModeMenu`
- *the tooltip is a window of its own, and this runtime has no windows: the mode of the tooltip is not answered.* (1): `CoordModeToolTip`
- *this runtime reads no text of a window.* (1): `DetectHiddenText`
- *the windows of this desktop are the ones of the compositor, and it has no hidden ones.* (1): `DetectHiddenWindows`
- *this runtime has no window of its own: it is a script that runs and ends.* (1): `ScriptHwnd`
- *the titles are matched the way v2 matches them with the mode 2, and the mode is not changed.* (1): `TitleMatchMode`

## 5. Screen capture and pixel functions

**7 names: 6 answer with a value, 1 is refused.**

**The ones that answer** (6): `ImageSearch`, `LoadPicture`, `PixelGetColor`, `PixelSearch`, `ScreenHeight`, `ScreenWidth`

**The ones that are refused** (1), by the reason:

- *this desktop does not know the scale of the output.* (1): `ScreenDPI`

## 6. Native function calls (DllCall)

**9 names: 8 answer with a value, 1 is refused.**

**The ones that answer** (8): `Buffer.Ptr`, `Buffer.Size`, `Buffer.__New`, `DllCall`, `NumGet`, `NumPut`, `PtrSize`, `VarSetStrCapacity`

**The ones that are refused** (1), by the reason:

- *this runtime has no last error of the operating system: the tree does not call Windows.* (1): `LastError`

## 7. Registry APIs

**11 names: 4 answer with a value, 7 are refused.**

**The ones that answer** (4): `LoopRegKey`, `LoopRegName`, `LoopRegTimeModified`, `LoopRegType`

**The ones that are refused** (7), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (6): `RegCreateKey`, `RegDelete`, `RegDeleteKey`, `RegRead`, `RegWrite`, `SetRegView`
- *the registry is not carried by this runtime.* (1): `RegView`

## 8. COM automation (ComObject)

**15 names: 0 answer with a value, 15 are refused.**

**The ones that are refused** (15), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (14): `ComCall`, `ComObjActive`, `ComObjConnect`, `ComObjFlags`, `ComObjFromPtr`, `ComObjGet`, `ComObjQuery`, `ComObjType`, `ComObjValue`, `ObjFromPtr`, `ObjFromPtrAddRef`, `ObjPtr`, `ObjPtrAddRef`, `ObjRelease`
- *this runtime does not carry that name yet.* (1): `ObjBindMethod`

## 9. The core of the language: text, math and patterns

**58 names: 55 answer with a value, 3 are refused.**

**The ones that answer** (55): `ACos`, `ASin`, `ATan`, `Abs`, `Ceil`, `Chr`, `Cos`, `Exp`, `Floor`, `Format`, `GetMethod`, `HasBase`, `HasMethod`, `HasProp`, `InStr`, `IsAlnum`, `IsAlpha`, `IsDigit`, `IsFloat`, `IsInteger`, `IsLower`, `IsNumber`, `IsObject`, `IsSetRef`, `IsSpace`, `IsTime`, `IsUpper`, `IsXDigit`, `LTrim`, `Ln`, `Log`, `Max`, `Min`, `Mod`, `Ord`, `RTrim`, `Random`, `RegExMatch`, `RegExReplace`, `Round`, `Sin`, `Sort`, `Sqrt`, `StrCompare`, `StrLen`, `StrLower`, `StrReplace`, `StrSplit`, `StrTitle`, `StrUpper`, `SubStr`, `Tan`, `Trim`, `Type`, `VerCompare`

**The ones that are refused** (3), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (3): `StrGet`, `StrPtr`, `StrPut`

## 10. The objects of the language: Array, Map and Func

**48 names: 48 answer with a value, 0 are refused.**

**The ones that answer** (48): `Array.Capacity`, `Array.Clone`, `Array.Delete`, `Array.Get`, `Array.Has`, `Array.InsertAt`, `Array.Length`, `Array.Pop`, `Array.Push`, `Array.RemoveAt`, `Array.__Enum`, `Array.__Item`, `Array.__New`, `Func.Bind`, `Func.Call`, `Func.IsBuiltIn`, `Func.IsByRef`, `Func.IsOptional`, `Func.IsVariadic`, `Func.MaxParams`, `Func.MinParams`, `Func.Name`, `Map.Capacity`, `Map.CaseSense`, `Map.Clear`, `Map.Clone`, `Map.Clone`, `Map.Count`, `Map.DefineProp`, `Map.Delete`, `Map.DeleteProp`, `Map.Get`, `Map.GetOwnPropDesc`, `Map.Has`, `Map.HasOwnProp`, `Map.OwnProps`, `Map.Set`, `Map.__Enum`, `Map.__Item`, `Map.__New`, `ObjAddRef`, `ObjGetBase`, `ObjGetCapacity`, `ObjHasOwnProp`, `ObjOwnPropCount`, `ObjOwnProps`, `ObjSetBase`, `ObjSetCapacity`

## 11. Dates and times

**23 names: 23 answer with a value, 0 are refused.**

**The ones that answer** (23): `DD`, `DDD`, `DDDD`, `DateAdd`, `DateDiff`, `FormatTime`, `Hour`, `MDay`, `MM`, `MMM`, `MMMM`, `MSec`, `Min`, `Mon`, `Now`, `NowUTC`, `Sec`, `TickCount`, `WDay`, `YDay`, `YWeek`, `YYYY`, `Year`

## 12. The machine, the processes and the environment

**35 names: 20 answer with a value, 15 are refused.**

**The ones that answer** (20): `AhkVersion`, `AppData`, `AppDataCommon`, `ComSpec`, `ComputerName`, `Desktop`, `DesktopCommon`, `Language`, `MyDocuments`, `OSVersion`, `Programs`, `ProgramsCommon`, `Run`, `RunWait`, `StartMenu`, `StartMenuCommon`, `Startup`, `StartupCommon`, `Temp`, `UserName`

**The ones that are refused** (15), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (14): `OutputDebug`, `ProcessClose`, `ProcessExist`, `ProcessGetName`, `ProcessGetParent`, `ProcessGetPath`, `ProcessSetPriority`, `ProcessWait`, `ProcessWaitClose`, `ProgramFiles`, `RunAs`, `Shutdown`, `SysGet`, `SysGetIPAddresses`
- *this runtime is not AutoHotkey, and it has no path of an executable.* (1): `AhkPath`

## 13. The clipboard

**2 names: 1 answer with a value, 1 is refused.**

**The ones that answer** (1): `Clipboard`

**The ones that are refused** (1), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (1): `OnClipboardChange`

## 14. Sound

**8 names: 6 answer with a value, 2 are refused.**

**The ones that answer** (6): `SoundBeep`, `SoundGetMute`, `SoundGetVolume`, `SoundPlay`, `SoundSetMute`, `SoundSetVolume`

**The ones that are refused** (2), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (2): `SoundGetInterface`, `SoundGetName`

