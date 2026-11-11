# The capability report of the surface of AutoHotkey v2.0.19

> **Generated.** The generator is `crates/ahk/reference/ahk-v2.0.19.areas.py` , and the state of every name is the one of the tree itself: `ahk --surface` asks the table of the built-ins, the table of the built-in variables and the classes, and answers with a value, with the reason of a refusal, or with the reason of a name to do.

The surface of the v2.0.19 has **833 entries** : **815 answer with a value** , **18 are refused** and **none is pending**.

The table has **833 entries and 785 distinct names** : a property with a setter is in the table twice --- the `Hotkey` class has eleven of them and the `InputHook` has twelve --- and a name that is both a command and a variable is two entries (`SendMode` , `SendLevel` , `Min` , `FileEncoding` , `ListLines` , `DetectHiddenText` , `DetectHiddenWindows` ). The numbers below are entries.

## The areas

| Area | Names | Answer | Refused |
|---|---|---|---|
| Parser and script execution | 56 | 51 | 5 |
| File and directory operations | 97 | 97 | 0 |
| Hotkeys, hotstrings, and input | 138 | 137 | 1 |
| GUI windows and window automation | 288 | 276 | 12 |
| Screen capture and pixel functions | 7 | 7 | 0 |
| Native function calls (DllCall) | 10 | 10 | 0 |
| Registry APIs | 11 | 11 | 0 |
| COM automation (ComObject) | 18 | 18 | 0 |
| The core of the language: text, math and patterns | 69 | 69 | 0 |
| The objects of the language: Array, Map and Func | 68 | 68 | 0 |
| Dates and times | 23 | 23 | 0 |
| The machine, the processes and the environment | 35 | 35 | 0 |
| The clipboard | 5 | 5 | 0 |
| Sound | 8 | 8 | 0 |
| **The whole of it** | **833** | **815** | **18** |

## What the numbers do not say

Three things, all measured on the binary of the v2.0.19 on 2026-10-29.

**The classes of the windows are carried, and every call of them fails.**
The names of `Gui` , of `Gui.Control` , of `Gui.ListView` , of
`Gui.StatusBar` and of `Menu` are in the tree, and the object is made ---
`myGui := Gui()` works --- but the thing itself is refused with the reason:
*"a window is a Windows window, and this desktop is a Wayland session without
one"*. The surface counts these names as answering, because the class
carries them; the call is what fails.

**The class types are in the table, and one of them is not a class.**
The table carries the thirty-two class types of v2.0.19 --- `Any` ,
`Array` , `BoundFunc` , `Buffer` , `Class` , `ClipboardAll` , `Closure` ,
`ComObject` , `ComValue` , `Enumerator` , `Error` , `File` , `Func` ,
`Gui` , `InputHook` , `Map` , `Menu` , `Object` , `RegExMatchInfo` and
the thirteen classes of the failures --- measured on the binary of
v2.0.19 on 2026-10-31, where a script reads every one of them as the
class. The `Hotkey` is the exception: the name is a function of v2, and
the class of a hotkey is the class of the object that the function makes.

**Three of the class types are not in the documentation.**
The `BoundFunc` , the `Closure` and the `RegExMatchInfo` are classes of
the binary that the documentation of v2 does not name. A script reads them
as a class, and the call of them is refused with *"Invalid base."* --- the
runtime makes them, and not a script --- the way the call of the `File`
and of the `Func` is refused.

**The calls of the class types are the calls of v2.**
The number of the arguments of every constructor is the one of the binary,
and the refusal of a call that has the wrong number of it carries the name
of the class the way v2 carries it. The `Any` is not a class that can be
made --- the call of it answers *"This value of type "Class" has no method
named "Call"."* --- and the `ComObject` and the `ComValue` are classes of
Windows that this runtime does not carry: *"the COM objects are a thing of
Windows, and this runtime does not carry them."* The checks of the dialect
come first: the number of the arguments and the kind of them are the
dialect's, and the reason of the machine comes after them.

**The area of the parser is about the surface, not about the grammar.**
What a script may write is the work of the wave of 2026-10-29
(`docs/ahk-v1-dialect.md` says what the plan is) . This report is about the
names that the tree answers.

## 1. Parser and script execution

**56 names: 51 answer with a value, 5 are refused.**

**The ones that answer** (51): `AllowMainWindow`, `CoordMode`, `EndChar`, `EnvGet`, `EnvSet`, `Error`, `Error.Extra`, `Error.File`, `Error.Line`, `Error.Message`, `Error.Stack`, `Error.What`, `Exit`, `ExitApp`, `Index`, `IndexError`, `InitialWorkingDir`, `Is64bitOS`, `IsAdmin`, `IsCompiled`, `IsCritical`, `IsPaused`, `IsSuspended`, `LineFile`, `LineNumber`, `MemberError`, `MemoryError`, `MethodError`, `OSError`, `OnError`, `OnExit`, `OnMessage`, `Persistent`, `PropertyError`, `ScriptDir`, `ScriptFullPath`, `ScriptName`, `SetTimer`, `SetWorkingDir`, `Sleep`, `Space`, `Tab`, `TargetError`, `ThisFunc`, `TimeoutError`, `TypeError`, `UnsetError`, `UnsetItemError`, `ValueError`, `WorkingDir`, `ZeroDivisionError`

**The ones that are refused** (5), by the reason:

- *this runtime runs one script in one thread, and it cannot make it critical.* (1): `Critical`
- *this runtime cannot wait for a key to continue: the script ends when it ends.* (1): `Pause`
- *this runtime runs one script once, and it cannot run itself again.* (1): `Reload`
- *this runtime cannot stop itself: it would never end.* (1): `Suspend`
- *this runtime runs one script in one thread, and it cannot start another.* (1): `Thread`

## 2. File and directory operations

**97 names: 97 answer with a value, 0 are refused.**

**The ones that answer** (97): `DirCopy`, `DirCreate`, `DirDelete`, `DirExist`, `DirMove`, `DirSelect`, `Download`, `DriveEject`, `DriveGetCapacity`, `DriveGetFilesystem`, `DriveGetLabel`, `DriveGetList`, `DriveGetSerial`, `DriveGetSpaceFree`, `DriveGetStatus`, `DriveGetStatusCD`, `DriveGetType`, `DriveLock`, `DriveRetract`, `DriveSetLabel`, `DriveUnlock`, `File`, `File.AtEOF`, `File.Close`, `File.Encoding`, `File.Encoding`, `File.Handle`, `File.Length`, `File.Length`, `File.Pos`, `File.Pos`, `File.RawRead`, `File.RawWrite`, `File.Read`, `File.ReadChar`, `File.ReadDouble`, `File.ReadFloat`, `File.ReadInt`, `File.ReadInt64`, `File.ReadLine`, `File.ReadShort`, `File.ReadUChar`, `File.ReadUInt`, `File.ReadUShort`, `File.Seek`, `File.Write`, `File.WriteChar`, `File.WriteDouble`, `File.WriteFloat`, `File.WriteInt`, `File.WriteInt64`, `File.WriteLine`, `File.WriteShort`, `File.WriteUChar`, `File.WriteUInt`, `File.WriteUShort`, `FileAppend`, `FileCopy`, `FileCreateShortcut`, `FileDelete`, `FileEncoding`, `FileEncoding`, `FileExist`, `FileGetAttrib`, `FileGetShortcut`, `FileGetSize`, `FileGetTime`, `FileGetVersion`, `FileInstall`, `FileMove`, `FileOpen`, `FileRead`, `FileRecycle`, `FileRecycleEmpty`, `FileSelect`, `FileSetAttrib`, `FileSetTime`, `IniDelete`, `IniRead`, `IniWrite`, `LoopField`, `LoopFileAttrib`, `LoopFileDir`, `LoopFileExt`, `LoopFileFullPath`, `LoopFileName`, `LoopFilePath`, `LoopFileShortName`, `LoopFileShortPath`, `LoopFileSize`, `LoopFileSizeKB`, `LoopFileSizeMB`, `LoopFileTimeAccessed`, `LoopFileTimeCreated`, `LoopFileTimeModified`, `LoopReadLine`, `SplitPath`

## 3. Hotkeys, hotstrings, and input

**138 names: 137 answer with a value, 1 is refused.**

**The ones that answer** (137): `BlockInput`, `CallbackCreate`, `CallbackFree`, `Click`, `ClipWait`, `CoordModeMouse`, `CoordModePixel`, `DefaultMouseSpeed`, `EventInfo`, `GetKeyName`, `GetKeySC`, `GetKeyState`, `GetKeyVK`, `HotIf`, `HotIfWinActive`, `HotIfWinExist`, `HotIfWinNotActive`, `HotIfWinNotExist`, `Hotkey`, `Hotkey.BackspaceIsUndo`, `Hotkey.BackspaceIsUndo`, `Hotkey.CaseSensitive`, `Hotkey.CaseSensitive`, `Hotkey.EndKey`, `Hotkey.EndMods`, `Hotkey.EndReason`, `Hotkey.FindAnywhere`, `Hotkey.FindAnywhere`, `Hotkey.InProgress`, `Hotkey.Input`, `Hotkey.KeyOpt`, `Hotkey.Match`, `Hotkey.MinSendLevel`, `Hotkey.MinSendLevel`, `Hotkey.NotifyNonText`, `Hotkey.NotifyNonText`, `Hotkey.OnChar`, `Hotkey.OnChar`, `Hotkey.OnEnd`, `Hotkey.OnEnd`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyUp`, `Hotkey.OnKeyUp`, `Hotkey.Start`, `Hotkey.Stop`, `Hotkey.Timeout`, `Hotkey.Timeout`, `Hotkey.VisibleNonText`, `Hotkey.VisibleNonText`, `Hotkey.VisibleText`, `Hotkey.VisibleText`, `Hotkey.Wait`, `Hotkey.__New`, `HotkeyInterval`, `HotkeyModifierTimeout`, `Hotstring`, `InputHook`, `InputHook.BackspaceIsUndo`, `InputHook.BackspaceIsUndo`, `InputHook.CaseSensitive`, `InputHook.CaseSensitive`, `InputHook.EndKey`, `InputHook.EndMods`, `InputHook.EndReason`, `InputHook.FindAnywhere`, `InputHook.FindAnywhere`, `InputHook.InProgress`, `InputHook.Input`, `InputHook.KeyOpt`, `InputHook.Match`, `InputHook.MinSendLevel`, `InputHook.MinSendLevel`, `InputHook.NotifyNonText`, `InputHook.NotifyNonText`, `InputHook.OnChar`, `InputHook.OnChar`, `InputHook.OnEnd`, `InputHook.OnEnd`, `InputHook.OnKeyDown`, `InputHook.OnKeyDown`, `InputHook.OnKeyUp`, `InputHook.OnKeyUp`, `InputHook.Start`, `InputHook.Stop`, `InputHook.Timeout`, `InputHook.Timeout`, `InputHook.VisibleNonText`, `InputHook.VisibleNonText`, `InputHook.VisibleText`, `InputHook.VisibleText`, `InputHook.Wait`, `InputHook.__New`, `InstallKeybdHook`, `InstallMouseHook`, `IsLabel`, `KeyDelay`, `KeyDelayPlay`, `KeyDuration`, `KeyDurationPlay`, `KeyHistory`, `KeyWait`, `ListHotkeys`, `MaxHotkeysPerInterval`, `MenuMaskKey`, `MouseClick`, `MouseClickDrag`, `MouseDelay`, `MouseDelayPlay`, `MouseGetPos`, `MouseMove`, `PriorHotkey`, `PriorKey`, `Send`, `SendEvent`, `SendInput`, `SendLevel`, `SendLevel`, `SendMode`, `SendMode`, `SendPlay`, `SendText`, `SetCapsLockState`, `SetDefaultMouseSpeed`, `SetKeyDelay`, `SetMouseDelay`, `SetNumLockState`, `SetScrollLockState`, `SetStoreCapsLockMode`, `StoreCapsLockMode`, `ThisHotkey`, `TimeIdle`, `TimeIdleKeyboard`, `TimeIdleMouse`, `TimeIdlePhysical`, `TimeSincePriorHotkey`, `TimeSinceThisHotkey`

**The ones that are refused** (1), by the reason:

- *the shape of the cursor of the machine is not read by this runtime.* (1): `Cursor`

## 4. GUI windows and window automation

**288 names: 276 answer with a value, 12 are refused.**

**The ones that answer** (276): `CaretGetPos`, `ControlAddItem`, `ControlChooseIndex`, `ControlChooseString`, `ControlClick`, `ControlDelay`, `ControlDeleteItem`, `ControlFindItem`, `ControlFocus`, `ControlGetChecked`, `ControlGetChoice`, `ControlGetClassNN`, `ControlGetEnabled`, `ControlGetFocus`, `ControlGetHwnd`, `ControlGetIndex`, `ControlGetItems`, `ControlGetPos`, `ControlGetText`, `ControlGetVisible`, `ControlHide`, `ControlHideDropDown`, `ControlMove`, `ControlSend`, `ControlSendText`, `ControlSetChecked`, `ControlSetEnabled`, `ControlSetText`, `ControlShow`, `ControlShowDropDown`, `CoordModeCaret`, `CoordModeMenu`, `CoordModeToolTip`, `DetectHiddenText`, `DetectHiddenText`, `DetectHiddenWindows`, `DetectHiddenWindows`, `EditGetCurrentCol`, `EditGetCurrentLine`, `EditGetLine`, `EditGetLineCount`, `EditGetSelectedText`, `EditPaste`, `GroupActivate`, `GroupAdd`, `GroupClose`, `GroupDeactivate`, `Gui`, `Gui.Add`, `Gui.AddActiveX`, `Gui.AddButton`, `Gui.AddCheckBox`, `Gui.AddComboBox`, `Gui.AddCustom`, `Gui.AddDDL`, `Gui.AddDateTime`, `Gui.AddDropDownList`, `Gui.AddEdit`, `Gui.AddGroupBox`, `Gui.AddHotkey`, `Gui.AddLink`, `Gui.AddListBox`, `Gui.AddListView`, `Gui.AddMonthCal`, `Gui.AddPic`, `Gui.AddPicture`, `Gui.AddProgress`, `Gui.AddRadio`, `Gui.AddSlider`, `Gui.AddStatusBar`, `Gui.AddTab`, `Gui.AddTab2`, `Gui.AddTab3`, `Gui.AddText`, `Gui.AddTreeView`, `Gui.AddUpDown`, `Gui.BackColor`, `Gui.BackColor`, `Gui.Control.Add`, `Gui.Control.Choose`, `Gui.Control.ClassNN`, `Gui.Control.Delete`, `Gui.Control.Enabled`, `Gui.Control.Enabled`, `Gui.Control.Focus`, `Gui.Control.Focused`, `Gui.Control.GetPos`, `Gui.Control.Gui`, `Gui.Control.Hwnd`, `Gui.Control.Move`, `Gui.Control.Name`, `Gui.Control.Name`, `Gui.Control.OnCommand`, `Gui.Control.OnEvent`, `Gui.Control.OnNotify`, `Gui.Control.Opt`, `Gui.Control.Redraw`, `Gui.Control.SetFont`, `Gui.Control.SetFormat`, `Gui.Control.Text`, `Gui.Control.Text`, `Gui.Control.Type`, `Gui.Control.UseTab`, `Gui.Control.Value`, `Gui.Control.Value`, `Gui.Control.Visible`, `Gui.Control.Visible`, `Gui.Destroy`, `Gui.Flash`, `Gui.FocusedCtrl`, `Gui.GetClientPos`, `Gui.GetPos`, `Gui.Hide`, `Gui.Hwnd`, `Gui.ListView.Add`, `Gui.ListView.Delete`, `Gui.ListView.Get`, `Gui.ListView.GetChild`, `Gui.ListView.GetCount`, `Gui.ListView.GetNext`, `Gui.ListView.GetParent`, `Gui.ListView.GetPrev`, `Gui.ListView.GetSelection`, `Gui.ListView.GetText`, `Gui.ListView.Modify`, `Gui.ListView.SetIcon`, `Gui.ListView.SetImageList`, `Gui.ListView.SetParts`, `Gui.ListView.SetText`, `Gui.MarginX`, `Gui.MarginX`, `Gui.MarginY`, `Gui.MarginY`, `Gui.Maximize`, `Gui.Menu.Add`, `Gui.Menu.AddStandard`, `Gui.Menu.Check`, `Gui.Menu.ClickCount`, `Gui.Menu.ClickCount`, `Gui.Menu.Default`, `Gui.Menu.Default`, `Gui.Menu.Delete`, `Gui.Menu.Disable`, `Gui.Menu.Enable`, `Gui.Menu.Handle`, `Gui.Menu.Insert`, `Gui.Menu.Rename`, `Gui.Menu.SetColor`, `Gui.Menu.SetIcon`, `Gui.Menu.Show`, `Gui.Menu.ToggleCheck`, `Gui.Menu.ToggleEnable`, `Gui.Menu.Uncheck`, `Gui.MenuBar`, `Gui.MenuBar`, `Gui.Minimize`, `Gui.Move`, `Gui.Name`, `Gui.Name`, `Gui.OnEvent`, `Gui.Opt`, `Gui.Restore`, `Gui.SetFont`, `Gui.Show`, `Gui.StatusBar.Add`, `Gui.StatusBar.Delete`, `Gui.StatusBar.DeleteCol`, `Gui.StatusBar.GetCount`, `Gui.StatusBar.GetNext`, `Gui.StatusBar.GetText`, `Gui.StatusBar.Insert`, `Gui.StatusBar.InsertCol`, `Gui.StatusBar.Modify`, `Gui.StatusBar.ModifyCol`, `Gui.StatusBar.SetImageList`, `Gui.Submit`, `Gui.Title`, `Gui.Title`, `Gui.TreeView.Add`, `Gui.TreeView.Delete`, `Gui.TreeView.Get`, `Gui.TreeView.GetChild`, `Gui.TreeView.GetCount`, `Gui.TreeView.GetNext`, `Gui.TreeView.GetParent`, `Gui.TreeView.GetPrev`, `Gui.TreeView.GetSelection`, `Gui.TreeView.GetText`, `Gui.TreeView.Modify`, `Gui.TreeView.SetIcon`, `Gui.TreeView.SetImageList`, `Gui.TreeView.SetParts`, `Gui.TreeView.SetText`, `Gui.__Enum`, `Gui.__Item`, `Gui.__New`, `GuiCtrlFromHwnd`, `GuiFromHwnd`, `IL_Add`, `IL_Create`, `IL_Destroy`, `IconFile`, `IconHidden`, `IconNumber`, `IconTip`, `InputBox`, `ListLines`, `ListLines`, `ListVars`, `ListViewGetContent`, `Menu`, `MenuFromHandle`, `MenuSelect`, `MonitorGet`, `MonitorGetCount`, `MonitorGetName`, `MonitorGetPrimary`, `MonitorGetWorkArea`, `MsgBox`, `PostMessage`, `SendMessage`, `SetControlDelay`, `SetTitleMatchMode`, `SetWinDelay`, `StatusBarGetText`, `StatusBarWait`, `TitleMatchMode`, `TitleMatchModeSpeed`, `ToolTip`, `TrayMenu`, `TraySetIcon`, `TrayTip`, `WinActivate`, `WinActivateBottom`, `WinActive`, `WinClose`, `WinDelay`, `WinDir`, `WinExist`, `WinGetClass`, `WinGetClientPos`, `WinGetControls`, `WinGetControlsHwnd`, `WinGetCount`, `WinGetID`, `WinGetIDLast`, `WinGetList`, `WinGetMinMax`, `WinGetPID`, `WinGetPos`, `WinGetProcessName`, `WinGetProcessPath`, `WinGetText`, `WinGetTitle`, `WinGetTransparent`, `WinHide`, `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`, `WinMinimizeAllUndo`, `WinMove`, `WinMoveBottom`, `WinMoveTop`, `WinRedraw`, `WinRestore`, `WinSetAlwaysOnTop`, `WinSetEnabled`, `WinSetRegion`, `WinSetTitle`, `WinSetTransparent`, `WinShow`, `WinWait`, `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`

**The ones that are refused** (12), by the reason:

- *this is a name of Windows, and this desktop is not Windows.* (7): `Edit`, `WinGetExStyle`, `WinGetStyle`, `WinGetTransColor`, `WinSetExStyle`, `WinSetStyle`, `WinSetTransColor`
- *these names act on a list-shaped control or a status bar, and this runtime makes no such control; the reference can be asked about the list ones, so a ComboBox is the next piece of work.* (4): `ControlGetExStyle`, `ControlGetStyle`, `ControlSetExStyle`, `ControlSetStyle`
- *this runtime has no window of its own: it is a script that runs and ends.* (1): `ScriptHwnd`

## 5. Screen capture and pixel functions

**7 names: 7 answer with a value, 0 are refused.**

**The ones that answer** (7): `ImageSearch`, `LoadPicture`, `PixelGetColor`, `PixelSearch`, `ScreenDPI`, `ScreenHeight`, `ScreenWidth`

## 6. Native function calls (DllCall)

**10 names: 10 answer with a value, 0 are refused.**

**The ones that answer** (10): `Buffer`, `Buffer.Ptr`, `Buffer.Size`, `Buffer.__New`, `DllCall`, `LastError`, `NumGet`, `NumPut`, `PtrSize`, `VarSetStrCapacity`

## 7. Registry APIs

**11 names: 11 answer with a value, 0 are refused.**

**The ones that answer** (11): `LoopRegKey`, `LoopRegName`, `LoopRegTimeModified`, `LoopRegType`, `RegCreateKey`, `RegDelete`, `RegDeleteKey`, `RegRead`, `RegView`, `RegWrite`, `SetRegView`

## 8. COM automation (ComObject)

**18 names: 18 answer with a value, 0 are refused.**

**The ones that answer** (18): `ComCall`, `ComObjActive`, `ComObjConnect`, `ComObjFlags`, `ComObjFromPtr`, `ComObjGet`, `ComObjQuery`, `ComObjType`, `ComObjValue`, `ComObject`, `ComValue`, `ComValue.Ptr`, `ObjBindMethod`, `ObjFromPtr`, `ObjFromPtrAddRef`, `ObjPtr`, `ObjPtrAddRef`, `ObjRelease`

## 9. The core of the language: text, math and patterns

**69 names: 69 answer with a value, 0 are refused.**

**The ones that answer** (69): `ACos`, `ASin`, `ATan`, `Abs`, `Any`, `Ceil`, `Chr`, `Class`, `Class.Base`, `Class.Call`, `Class.Prototype`, `Class.__Init`, `Cos`, `Exp`, `Float`, `Floor`, `Format`, `GetMethod`, `HasBase`, `HasMethod`, `HasProp`, `InStr`, `Integer`, `IsAlnum`, `IsAlpha`, `IsDigit`, `IsFloat`, `IsInteger`, `IsLower`, `IsNumber`, `IsObject`, `IsSetRef`, `IsSpace`, `IsTime`, `IsUpper`, `IsXDigit`, `LTrim`, `Ln`, `Log`, `Max`, `Min`, `Mod`, `Number`, `Ord`, `Primitive`, `RTrim`, `Random`, `RegExMatch`, `RegExReplace`, `Round`, `Sin`, `Sort`, `Sqrt`, `StrCompare`, `StrGet`, `StrLen`, `StrLower`, `StrPtr`, `StrPut`, `StrReplace`, `StrSplit`, `StrTitle`, `StrUpper`, `String`, `SubStr`, `Tan`, `Trim`, `Type`, `VerCompare`

## 10. The objects of the language: Array, Map and Func

**68 names: 68 answer with a value, 0 are refused.**

**The ones that answer** (68): `Array`, `Array.Capacity`, `Array.Clone`, `Array.Delete`, `Array.Get`, `Array.Has`, `Array.InsertAt`, `Array.Length`, `Array.Pop`, `Array.Push`, `Array.RemoveAt`, `Array.__Enum`, `Array.__Item`, `Array.__New`, `BoundFunc`, `Closure`, `Enumerator`, `Enumerator.Call`, `Func`, `Func.Bind`, `Func.Call`, `Func.IsBuiltIn`, `Func.IsByRef`, `Func.IsOptional`, `Func.IsVariadic`, `Func.MaxParams`, `Func.MinParams`, `Func.Name`, `Map`, `Map.Capacity`, `Map.CaseSense`, `Map.Clear`, `Map.Clone`, `Map.Clone`, `Map.Count`, `Map.DefineProp`, `Map.Delete`, `Map.DeleteProp`, `Map.Get`, `Map.GetOwnPropDesc`, `Map.Has`, `Map.HasOwnProp`, `Map.OwnProps`, `Map.Set`, `Map.__Enum`, `Map.__Item`, `Map.__New`, `ObjAddRef`, `ObjGetBase`, `ObjGetCapacity`, `ObjHasOwnProp`, `ObjOwnPropCount`, `ObjOwnProps`, `ObjSetBase`, `ObjSetCapacity`, `Object`, `Object.Base`, `Object.Clone`, `Object.DefineProp`, `Object.DeleteProp`, `Object.GetMethod`, `Object.GetOwnPropDesc`, `Object.HasBase`, `Object.HasMethod`, `Object.HasOwnProp`, `Object.HasProp`, `Object.OwnProps`, `RegExMatchInfo`

## 11. Dates and times

**23 names: 23 answer with a value, 0 are refused.**

**The ones that answer** (23): `DD`, `DDD`, `DDDD`, `DateAdd`, `DateDiff`, `FormatTime`, `Hour`, `MDay`, `MM`, `MMM`, `MMMM`, `MSec`, `Min`, `Mon`, `Now`, `NowUTC`, `Sec`, `TickCount`, `WDay`, `YDay`, `YWeek`, `YYYY`, `Year`

## 12. The machine, the processes and the environment

**35 names: 35 answer with a value, 0 are refused.**

**The ones that answer** (35): `AhkPath`, `AhkVersion`, `AppData`, `AppDataCommon`, `ComSpec`, `ComputerName`, `Desktop`, `DesktopCommon`, `Language`, `MyDocuments`, `OSVersion`, `OutputDebug`, `ProcessClose`, `ProcessExist`, `ProcessGetName`, `ProcessGetParent`, `ProcessGetPath`, `ProcessSetPriority`, `ProcessWait`, `ProcessWaitClose`, `ProgramFiles`, `Programs`, `ProgramsCommon`, `Run`, `RunAs`, `RunWait`, `Shutdown`, `StartMenu`, `StartMenuCommon`, `Startup`, `StartupCommon`, `SysGet`, `SysGetIPAddresses`, `Temp`, `UserName`

## 13. The clipboard

**5 names: 5 answer with a value, 0 are refused.**

**The ones that answer** (5): `Clipboard`, `ClipboardAll`, `ClipboardAll.Ptr`, `ClipboardAll.Size`, `OnClipboardChange`

## 14. Sound

**8 names: 8 answer with a value, 0 are refused.**

**The ones that answer** (8): `SoundBeep`, `SoundGetInterface`, `SoundGetMute`, `SoundGetName`, `SoundGetVolume`, `SoundPlay`, `SoundSetMute`, `SoundSetVolume`

