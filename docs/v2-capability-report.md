# The capability report of the surface of AutoHotkey v2.0.32

> **Generated.** The generator is `crates/ahk/reference/ahk-v2.0.32.areas.py` , and the state of every name is the one of the tree itself: `ahk --surface` asks the table of the built-ins, the table of the built-in variables and the classes, and answers with a value, with the reason of a refusal, or with the reason of a name to do.

The surface of the v2.0.32 has **843 entries** : **843 answer with a value** , **0 are refused** and **none is pending**.

The table has **843 entries and 795 distinct names** : a property with a setter is in the table twice --- the `Hotkey` class has eleven of them and the `InputHook` has twelve --- and a name that is both a command and a variable is two entries (`SendMode` , `SendLevel` , `Min` , `FileEncoding` , `ListLines` , `DetectHiddenText` , `DetectHiddenWindows` ). The numbers below are entries.

## The areas

| Area | Names | Answer | Refused |
|---|---|---|---|
| Parser and script execution | 56 | 56 | 0 |
| File and directory operations | 97 | 97 | 0 |
| Hotkeys, hotstrings, and input | 138 | 138 | 0 |
| GUI windows and window automation | 298 | 298 | 0 |
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
| **The whole of it** | **843** | **843** | **0** |

## What the numbers do not say

Three things, all measured on the binary of the v2.0.19.

**The classes of the windows are carried, and the call is made.**
The names of `Gui` , of `Gui.Control` , of `Gui.ListView` , of
`Gui.StatusBar` and of `Menu` are in the tree, and the object is made ---
`myGui := Gui()` works. On Windows the call talks to the real APIs; on the
Linux box it is drawn through the GTK helper of the window. The surface
counts these names as answering, because the class carries them and the
call is made where the machine has the primitive.

**The class types are in the table, and one of them is not a class.**
The table carries the thirty-two class types of v2.0.32 --- `Any` ,
`Array` , `BoundFunc` , `Buffer` , `Class` , `ClipboardAll` , `Closure` ,
`ComObject` , `ComValue` , `Enumerator` , `Error` , `File` , `Func` ,
`Gui` , `InputHook` , `Map` , `Menu` , `Object` , `RegExMatchInfo` and
the thirteen classes of the failures --- measured on the binary of
v2.0.19, where a script reads every one of them as the
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
named "Call"."* --- and the `ComObject` and the `ComValue` are the COM of
Windows, carried there; on the Linux box the object model of the session is
D-Bus (`DBusObject`), and the parts with no equivalent refuse with the
reason. The checks of the dialect
come first: the number of the arguments and the kind of them are the
dialect's, and the reason of the machine comes after them.

**The area of the parser is about the surface, not about the grammar.**
What a script may write is the work of the wave
(`docs/ahk-v1-dialect.md` says what the plan is) . This report is about the
names that the tree answers.

## 1. Parser and script execution

**56 names: 56 answer with a value, 0 are refused.**

**The ones that answer** (56): `AllowMainWindow`, `CoordMode`, `Critical`, `EndChar`, `EnvGet`, `EnvSet`, `Error`, `Error.Extra`, `Error.File`, `Error.Line`, `Error.Message`, `Error.Stack`, `Error.What`, `Exit`, `ExitApp`, `Index`, `IndexError`, `InitialWorkingDir`, `Is64bitOS`, `IsAdmin`, `IsCompiled`, `IsCritical`, `IsPaused`, `IsSuspended`, `LineFile`, `LineNumber`, `MemberError`, `MemoryError`, `MethodError`, `OSError`, `OnError`, `OnExit`, `OnMessage`, `Pause`, `Persistent`, `PropertyError`, `Reload`, `ScriptDir`, `ScriptFullPath`, `ScriptName`, `SetTimer`, `SetWorkingDir`, `Sleep`, `Space`, `Suspend`, `Tab`, `TargetError`, `ThisFunc`, `Thread`, `TimeoutError`, `TypeError`, `UnsetError`, `UnsetItemError`, `ValueError`, `WorkingDir`, `ZeroDivisionError`

## 2. File and directory operations

**97 names: 97 answer with a value, 0 are refused.**

**The ones that answer** (97): `DirCopy`, `DirCreate`, `DirDelete`, `DirExist`, `DirMove`, `DirSelect`, `Download`, `DriveEject`, `DriveGetCapacity`, `DriveGetFilesystem`, `DriveGetLabel`, `DriveGetList`, `DriveGetSerial`, `DriveGetSpaceFree`, `DriveGetStatus`, `DriveGetStatusCD`, `DriveGetType`, `DriveLock`, `DriveRetract`, `DriveSetLabel`, `DriveUnlock`, `File`, `File.AtEOF`, `File.Close`, `File.Encoding`, `File.Encoding`, `File.Handle`, `File.Length`, `File.Length`, `File.Pos`, `File.Pos`, `File.RawRead`, `File.RawWrite`, `File.Read`, `File.ReadChar`, `File.ReadDouble`, `File.ReadFloat`, `File.ReadInt`, `File.ReadInt64`, `File.ReadLine`, `File.ReadShort`, `File.ReadUChar`, `File.ReadUInt`, `File.ReadUShort`, `File.Seek`, `File.Write`, `File.WriteChar`, `File.WriteDouble`, `File.WriteFloat`, `File.WriteInt`, `File.WriteInt64`, `File.WriteLine`, `File.WriteShort`, `File.WriteUChar`, `File.WriteUInt`, `File.WriteUShort`, `FileAppend`, `FileCopy`, `FileCreateShortcut`, `FileDelete`, `FileEncoding`, `FileEncoding`, `FileExist`, `FileGetAttrib`, `FileGetShortcut`, `FileGetSize`, `FileGetTime`, `FileGetVersion`, `FileInstall`, `FileMove`, `FileOpen`, `FileRead`, `FileRecycle`, `FileRecycleEmpty`, `FileSelect`, `FileSetAttrib`, `FileSetTime`, `IniDelete`, `IniRead`, `IniWrite`, `LoopField`, `LoopFileAttrib`, `LoopFileDir`, `LoopFileExt`, `LoopFileFullPath`, `LoopFileName`, `LoopFilePath`, `LoopFileShortName`, `LoopFileShortPath`, `LoopFileSize`, `LoopFileSizeKB`, `LoopFileSizeMB`, `LoopFileTimeAccessed`, `LoopFileTimeCreated`, `LoopFileTimeModified`, `LoopReadLine`, `SplitPath`

## 3. Hotkeys, hotstrings, and input

**138 names: 138 answer with a value, 0 are refused.**

**The ones that answer** (138): `BlockInput`, `CallbackCreate`, `CallbackFree`, `Click`, `ClipWait`, `CoordModeMouse`, `CoordModePixel`, `Cursor`, `DefaultMouseSpeed`, `EventInfo`, `GetKeyName`, `GetKeySC`, `GetKeyState`, `GetKeyVK`, `HotIf`, `HotIfWinActive`, `HotIfWinExist`, `HotIfWinNotActive`, `HotIfWinNotExist`, `Hotkey`, `Hotkey.BackspaceIsUndo`, `Hotkey.BackspaceIsUndo`, `Hotkey.CaseSensitive`, `Hotkey.CaseSensitive`, `Hotkey.EndKey`, `Hotkey.EndMods`, `Hotkey.EndReason`, `Hotkey.FindAnywhere`, `Hotkey.FindAnywhere`, `Hotkey.InProgress`, `Hotkey.Input`, `Hotkey.KeyOpt`, `Hotkey.Match`, `Hotkey.MinSendLevel`, `Hotkey.MinSendLevel`, `Hotkey.NotifyNonText`, `Hotkey.NotifyNonText`, `Hotkey.OnChar`, `Hotkey.OnChar`, `Hotkey.OnEnd`, `Hotkey.OnEnd`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyDown`, `Hotkey.OnKeyUp`, `Hotkey.OnKeyUp`, `Hotkey.Start`, `Hotkey.Stop`, `Hotkey.Timeout`, `Hotkey.Timeout`, `Hotkey.VisibleNonText`, `Hotkey.VisibleNonText`, `Hotkey.VisibleText`, `Hotkey.VisibleText`, `Hotkey.Wait`, `Hotkey.__New`, `HotkeyInterval`, `HotkeyModifierTimeout`, `Hotstring`, `InputHook`, `InputHook.BackspaceIsUndo`, `InputHook.BackspaceIsUndo`, `InputHook.CaseSensitive`, `InputHook.CaseSensitive`, `InputHook.EndKey`, `InputHook.EndMods`, `InputHook.EndReason`, `InputHook.FindAnywhere`, `InputHook.FindAnywhere`, `InputHook.InProgress`, `InputHook.Input`, `InputHook.KeyOpt`, `InputHook.Match`, `InputHook.MinSendLevel`, `InputHook.MinSendLevel`, `InputHook.NotifyNonText`, `InputHook.NotifyNonText`, `InputHook.OnChar`, `InputHook.OnChar`, `InputHook.OnEnd`, `InputHook.OnEnd`, `InputHook.OnKeyDown`, `InputHook.OnKeyDown`, `InputHook.OnKeyUp`, `InputHook.OnKeyUp`, `InputHook.Start`, `InputHook.Stop`, `InputHook.Timeout`, `InputHook.Timeout`, `InputHook.VisibleNonText`, `InputHook.VisibleNonText`, `InputHook.VisibleText`, `InputHook.VisibleText`, `InputHook.Wait`, `InputHook.__New`, `InstallKeybdHook`, `InstallMouseHook`, `IsLabel`, `KeyDelay`, `KeyDelayPlay`, `KeyDuration`, `KeyDurationPlay`, `KeyHistory`, `KeyWait`, `ListHotkeys`, `MaxHotkeysPerInterval`, `MenuMaskKey`, `MouseClick`, `MouseClickDrag`, `MouseDelay`, `MouseDelayPlay`, `MouseGetPos`, `MouseMove`, `PriorHotkey`, `PriorKey`, `Send`, `SendEvent`, `SendInput`, `SendLevel`, `SendLevel`, `SendMode`, `SendMode`, `SendPlay`, `SendText`, `SetCapsLockState`, `SetDefaultMouseSpeed`, `SetKeyDelay`, `SetMouseDelay`, `SetNumLockState`, `SetScrollLockState`, `SetStoreCapsLockMode`, `StoreCapsLockMode`, `ThisHotkey`, `TimeIdle`, `TimeIdleKeyboard`, `TimeIdleMouse`, `TimeIdlePhysical`, `TimeSincePriorHotkey`, `TimeSinceThisHotkey`

## 4. GUI windows and window automation

**298 names: 298 answer with a value, 0 are refused.**

**The ones that answer** (298): `CaretGetPos`, `ControlAddItem`, `ControlChooseIndex`, `ControlChooseString`, `ControlClick`, `ControlDelay`, `ControlDeleteItem`, `ControlFindItem`, `ControlFocus`, `ControlGetChecked`, `ControlGetChoice`, `ControlGetClassNN`, `ControlGetEnabled`, `ControlGetExStyle`, `ControlGetFocus`, `ControlGetHwnd`, `ControlGetIndex`, `ControlGetItems`, `ControlGetPos`, `ControlGetStyle`, `ControlGetText`, `ControlGetVisible`, `ControlHide`, `ControlHideDropDown`, `ControlMove`, `ControlSend`, `ControlSendText`, `ControlSetChecked`, `ControlSetEnabled`, `ControlSetExStyle`, `ControlSetStyle`, `ControlSetText`, `ControlShow`, `ControlShowDropDown`, `CoordModeCaret`, `CoordModeMenu`, `CoordModeToolTip`, `DetectHiddenText`, `DetectHiddenText`, `DetectHiddenWindows`, `DetectHiddenWindows`, `Edit`, `EditGetCurrentCol`, `EditGetCurrentLine`, `EditGetLine`, `EditGetLineCount`, `EditGetSelectedText`, `EditPaste`, `GroupActivate`, `GroupAdd`, `GroupClose`, `GroupDeactivate`, `Gui`, `Gui.ActiveX`, `Gui.Add`, `Gui.AddActiveX`, `Gui.AddButton`, `Gui.AddCheckBox`, `Gui.AddComboBox`, `Gui.AddCustom`, `Gui.AddDDL`, `Gui.AddDateTime`, `Gui.AddDropDownList`, `Gui.AddEdit`, `Gui.AddGroupBox`, `Gui.AddHotkey`, `Gui.AddLink`, `Gui.AddListBox`, `Gui.AddListView`, `Gui.AddMonthCal`, `Gui.AddPic`, `Gui.AddPicture`, `Gui.AddProgress`, `Gui.AddRadio`, `Gui.AddSlider`, `Gui.AddStatusBar`, `Gui.AddTab`, `Gui.AddTab2`, `Gui.AddTab3`, `Gui.AddText`, `Gui.AddTreeView`, `Gui.AddUpDown`, `Gui.BackColor`, `Gui.BackColor`, `Gui.Button`, `Gui.CheckBox`, `Gui.ComboBox`, `Gui.Control`, `Gui.Control.ClassNN`, `Gui.Control.Enabled`, `Gui.Control.Enabled`, `Gui.Control.Focus`, `Gui.Control.Focused`, `Gui.Control.GetPos`, `Gui.Control.Gui`, `Gui.Control.Hwnd`, `Gui.Control.Move`, `Gui.Control.Name`, `Gui.Control.Name`, `Gui.Control.OnCommand`, `Gui.Control.OnEvent`, `Gui.Control.OnNotify`, `Gui.Control.Opt`, `Gui.Control.Redraw`, `Gui.Control.SetFont`, `Gui.Control.Text`, `Gui.Control.Text`, `Gui.Control.Type`, `Gui.Control.Value`, `Gui.Control.Value`, `Gui.Control.Visible`, `Gui.Control.Visible`, `Gui.Custom`, `Gui.DDL`, `Gui.DateTime`, `Gui.DateTime.SetFormat`, `Gui.Destroy`, `Gui.Edit`, `Gui.Flash`, `Gui.FocusedCtrl`, `Gui.GetClientPos`, `Gui.GetPos`, `Gui.GroupBox`, `Gui.Hide`, `Gui.Hotkey`, `Gui.Hwnd`, `Gui.Link`, `Gui.List`, `Gui.List.Add`, `Gui.List.Choose`, `Gui.List.Delete`, `Gui.ListBox`, `Gui.ListView`, `Gui.ListView.Add`, `Gui.ListView.Delete`, `Gui.ListView.DeleteCol`, `Gui.ListView.GetCount`, `Gui.ListView.GetNext`, `Gui.ListView.GetText`, `Gui.ListView.Insert`, `Gui.ListView.InsertCol`, `Gui.ListView.Modify`, `Gui.ListView.ModifyCol`, `Gui.ListView.SetImageList`, `Gui.MarginX`, `Gui.MarginX`, `Gui.MarginY`, `Gui.MarginY`, `Gui.Maximize`, `Gui.Menu.Add`, `Gui.Menu.AddStandard`, `Gui.Menu.Check`, `Gui.Menu.ClickCount`, `Gui.Menu.ClickCount`, `Gui.Menu.Default`, `Gui.Menu.Default`, `Gui.Menu.Delete`, `Gui.Menu.Disable`, `Gui.Menu.Enable`, `Gui.Menu.Handle`, `Gui.Menu.Insert`, `Gui.Menu.Rename`, `Gui.Menu.SetColor`, `Gui.Menu.SetIcon`, `Gui.Menu.Show`, `Gui.Menu.ToggleCheck`, `Gui.Menu.ToggleEnable`, `Gui.Menu.Uncheck`, `Gui.MenuBar`, `Gui.MenuBar`, `Gui.Minimize`, `Gui.MonthCal`, `Gui.Move`, `Gui.Name`, `Gui.Name`, `Gui.OnEvent`, `Gui.Opt`, `Gui.Pic`, `Gui.Progress`, `Gui.Radio`, `Gui.Restore`, `Gui.SetFont`, `Gui.Show`, `Gui.Slider`, `Gui.StatusBar`, `Gui.StatusBar.SetIcon`, `Gui.StatusBar.SetParts`, `Gui.StatusBar.SetText`, `Gui.Submit`, `Gui.Tab`, `Gui.Tab.UseTab`, `Gui.Text`, `Gui.Title`, `Gui.Title`, `Gui.TreeView`, `Gui.TreeView.Add`, `Gui.TreeView.Delete`, `Gui.TreeView.Get`, `Gui.TreeView.GetChild`, `Gui.TreeView.GetCount`, `Gui.TreeView.GetNext`, `Gui.TreeView.GetParent`, `Gui.TreeView.GetPrev`, `Gui.TreeView.GetSelection`, `Gui.TreeView.GetText`, `Gui.TreeView.Modify`, `Gui.TreeView.SetImageList`, `Gui.UpDown`, `Gui.__Enum`, `Gui.__Item`, `Gui.__New`, `GuiCtrlFromHwnd`, `GuiFromHwnd`, `IL_Add`, `IL_Create`, `IL_Destroy`, `IconFile`, `IconHidden`, `IconNumber`, `IconTip`, `InputBox`, `ListLines`, `ListLines`, `ListVars`, `ListViewGetContent`, `Menu`, `MenuFromHandle`, `MenuSelect`, `MonitorGet`, `MonitorGetCount`, `MonitorGetName`, `MonitorGetPrimary`, `MonitorGetWorkArea`, `MsgBox`, `PostMessage`, `ScriptHwnd`, `SendMessage`, `SetControlDelay`, `SetTitleMatchMode`, `SetWinDelay`, `StatusBarGetText`, `StatusBarWait`, `TitleMatchMode`, `TitleMatchModeSpeed`, `ToolTip`, `TrayMenu`, `TraySetIcon`, `TrayTip`, `WinActivate`, `WinActivateBottom`, `WinActive`, `WinClose`, `WinDelay`, `WinDir`, `WinExist`, `WinGetClass`, `WinGetClientPos`, `WinGetControls`, `WinGetControlsHwnd`, `WinGetCount`, `WinGetExStyle`, `WinGetID`, `WinGetIDLast`, `WinGetList`, `WinGetMinMax`, `WinGetPID`, `WinGetPos`, `WinGetProcessName`, `WinGetProcessPath`, `WinGetStyle`, `WinGetText`, `WinGetTitle`, `WinGetTransColor`, `WinGetTransparent`, `WinHide`, `WinKill`, `WinMaximize`, `WinMinimize`, `WinMinimizeAll`, `WinMinimizeAllUndo`, `WinMove`, `WinMoveBottom`, `WinMoveTop`, `WinRedraw`, `WinRestore`, `WinSetAlwaysOnTop`, `WinSetEnabled`, `WinSetExStyle`, `WinSetRegion`, `WinSetStyle`, `WinSetTitle`, `WinSetTransColor`, `WinSetTransparent`, `WinShow`, `WinWait`, `WinWaitActive`, `WinWaitClose`, `WinWaitNotActive`

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

