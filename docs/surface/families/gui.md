# GUI windows and window automation

The names of the family **GUI windows and window automation** in the three dialects, with what this tree answers for each of them. The family is the area a script thinks in; the names are the whole of it.

| Name | Dialect | Kind | Class | Parameters | State |
|---|---|---|---|---|---|
| `Gui.__Enum` | v2 | method | Gui | -- | value |
| `Gui.__Item` | v2 | method | Gui | -- | value |
| `Gui.__New` | v2 | method | Gui | -- | value |
| `A_ControlDelay` | v1 | variable | -- | -- | value |
| `A_DetectHiddenText` | v1 | variable | -- | -- | value |
| `A_DetectHiddenWindows` | v1 | variable | -- | -- | value |
| `A_Gui` | v1 | variable | -- | -- | value |
| `A_GuiControl` | v1 | variable | -- | -- | value |
| `A_GuiControlEvent` | v1 | variable | -- | -- | value |
| `A_GuiEvent` | v1 | variable | -- | -- | value |
| `A_GuiHeight` | v1 | variable | -- | -- | value |
| `A_GuiTheme` | ks | variable | -- | -- | value |
| `A_GuiWidth` | v1 | variable | -- | -- | value |
| `A_GuiX` | v1 | variable | -- | -- | value |
| `A_GuiY` | v1 | variable | -- | -- | value |
| `A_IconFile` | v1 | variable | -- | -- | refused: this runtime has no tray icon. |
| `A_IconHidden` | v1 | variable | -- | -- | value |
| `A_IconNumber` | v1 | variable | -- | -- | refused: this runtime has no tray icon. |
| `A_IconTip` | v1 | variable | -- | -- | value |
| `A_TitleMatchMode` | v1 | variable | -- | -- | value |
| `A_TitleMatchModeSpeed` | v1 | variable | -- | -- | value |
| `A_WinDelay` | v1 | variable | -- | -- | value |
| `A_WinDir` | v1 | variable | -- | -- | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Add` | v2 | method | Gui | -- | value |
| `Gui.Control.Add` | v2 | method | Gui.Control | -- | value |
| `Gui.ListView.Add` | v2 | method | Gui.ListView | -- | value |
| `Gui.Menu.Add` | v2 | method | Gui.Menu | -- | value |
| `Gui.StatusBar.Add` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.Add` | v2 | method | Gui.TreeView | -- | value |
| `Gui.AddActiveX` | v2 | method | Gui | -- | value |
| `Gui.AddButton` | v2 | method | Gui | -- | value |
| `Gui.AddCheckBox` | v2 | method | Gui | -- | value |
| `Gui.AddComboBox` | v2 | method | Gui | -- | value |
| `Gui.AddCustom` | v2 | method | Gui | -- | value |
| `Gui.AddDateTime` | v2 | method | Gui | -- | value |
| `Gui.AddDDL` | v2 | method | Gui | -- | value |
| `Gui.AddDropDownList` | v2 | method | Gui | -- | value |
| `Gui.AddEdit` | v2 | method | Gui | -- | value |
| `Gui.AddGroupBox` | v2 | method | Gui | -- | value |
| `Gui.AddHotkey` | v2 | method | Gui | -- | value |
| `Gui.AddLink` | v2 | method | Gui | -- | value |
| `Gui.AddListBox` | v2 | method | Gui | -- | value |
| `Gui.AddListView` | v2 | method | Gui | -- | value |
| `Gui.AddMonthCal` | v2 | method | Gui | -- | value |
| `Gui.AddPic` | v2 | method | Gui | -- | value |
| `Gui.AddPicture` | v2 | method | Gui | -- | value |
| `Gui.AddProgress` | v2 | method | Gui | -- | value |
| `Gui.AddRadio` | v2 | method | Gui | -- | value |
| `Gui.AddSlider` | v2 | method | Gui | -- | value |
| `Gui.Menu.AddStandard` | v2 | method | Gui.Menu | -- | value |
| `Gui.AddStatusBar` | v2 | method | Gui | -- | value |
| `Gui.AddTab` | v2 | method | Gui | -- | value |
| `Gui.AddTab2` | v2 | method | Gui | -- | value |
| `Gui.AddTab3` | v2 | method | Gui | -- | value |
| `Gui.AddText` | v2 | method | Gui | -- | value |
| `Gui.AddTreeView` | v2 | method | Gui | -- | value |
| `Gui.AddUpDown` | v2 | method | Gui | -- | value |
| `Gui.BackColor` | v2 | method | Gui | -- | value |
| `CaretGetPos` | v2 | function | -- | 0 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.Check` | v2 | method | Gui.Menu | -- | value |
| `Gui.Control.Choose` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.ClassNN` | v2 | method | Gui.Control | -- | value |
| `Gui.Menu.ClickCount` | v2 | method | Gui.Menu | -- | value |
| `Control` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlAddItem` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlChooseIndex` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlChooseString` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlClick` | v2 | command | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlClick` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlDelay` | v2 | variable | -- | -- | value |
| `ControlDeleteItem` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlFindItem` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlFocus` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlFocus` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetChecked` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetChoice` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetClassNN` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetEnabled` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetExStyle` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetFocus` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetFocus` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetHwnd` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetIndex` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetItems` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetPos` | v2 | command | -- | 0 to 9 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetPos` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetStyle` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetText` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetText` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlGetVisible` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlHide` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlHideDropDown` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlMove` | v2 | command | -- | 0 to 9 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlMove` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSend` | v2 | command | -- | 1 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSend` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSendText` | v2 | command | -- | 1 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetChecked` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetEnabled` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetExStyle` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetStyle` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetText` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlSetText` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlShow` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ControlShowDropDown` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `CoordModeCaret` | v2 | variable | -- | -- | value |
| `CoordModeMenu` | v2 | variable | -- | -- | value |
| `CoordModeToolTip` | v2 | variable | -- | -- | value |
| `Gui.Menu.Default` | v2 | method | Gui.Menu | -- | value |
| `Gui.Control.Delete` | v2 | method | Gui.Control | -- | value |
| `Gui.ListView.Delete` | v2 | method | Gui.ListView | -- | value |
| `Gui.Menu.Delete` | v2 | method | Gui.Menu | -- | value |
| `Gui.StatusBar.Delete` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.Delete` | v2 | method | Gui.TreeView | -- | value |
| `Gui.StatusBar.DeleteCol` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.Destroy` | v2 | method | Gui | -- | value |
| `DetectHiddenText` | v2 | command | -- | 1 | value |
| `DetectHiddenText` | v1 | command | -- | up to 1 | value |
| `DetectHiddenWindows` | v2 | command | -- | 1 | value |
| `DetectHiddenWindows` | v1 | command | -- | up to 1 | value |
| `Gui.Menu.Disable` | v2 | method | Gui.Menu | -- | value |
| `Edit` | v2 | command | -- | 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Edit` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditGetCurrentCol` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditGetCurrentLine` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditGetLine` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditGetLineCount` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditGetSelectedText` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `EditPaste` | v2 | command | -- | 2 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.Enable` | v2 | method | Gui.Menu | -- | value |
| `Gui.Control.Enabled` | v2 | method | Gui.Control | -- | value |
| `Gui.Flash` | v2 | method | Gui | -- | value |
| `Gui.Control.Focus` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.Focused` | v2 | method | Gui.Control | -- | value |
| `Gui.FocusedCtrl` | v2 | method | Gui | -- | value |
| `Gui.ListView.Get` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.Get` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.GetChild` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.GetChild` | v2 | method | Gui.TreeView | -- | value |
| `Gui.GetClientPos` | v2 | method | Gui | -- | value |
| `Gui.ListView.GetCount` | v2 | method | Gui.ListView | -- | value |
| `Gui.StatusBar.GetCount` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.GetCount` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.GetNext` | v2 | method | Gui.ListView | -- | value |
| `Gui.StatusBar.GetNext` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.GetNext` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.GetParent` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.GetParent` | v2 | method | Gui.TreeView | -- | value |
| `Gui.GetPos` | v2 | method | Gui | -- | value |
| `Gui.Control.GetPos` | v2 | method | Gui.Control | -- | value |
| `Gui.ListView.GetPrev` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.GetPrev` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.GetSelection` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.GetSelection` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.GetText` | v2 | method | Gui.ListView | -- | value |
| `Gui.StatusBar.GetText` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.GetText` | v2 | method | Gui.TreeView | -- | value |
| `GroupActivate` | v2 | command | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupActivate` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupAdd` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupAdd` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupClose` | v2 | command | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupClose` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupDeactivate` | v2 | command | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GroupDeactivate` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui` | v2 | class | -- | -- | value |
| `Gui` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Control.Gui` | v2 | method | Gui.Control | -- | value |
| `GuiControl` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GuiControlGet` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GuiCtrlFromHwnd` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `GuiFromHwnd` | v2 | command | -- | 1 to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.Handle` | v2 | method | Gui.Menu | -- | value |
| `Gui.Hide` | v2 | method | Gui | -- | value |
| `Gui.Hwnd` | v2 | method | Gui | -- | value |
| `Gui.Control.Hwnd` | v2 | method | Gui.Control | -- | value |
| `IconFile` | v2 | variable | -- | -- | refused: this runtime has no tray icon. |
| `IconHidden` | v2 | variable | -- | -- | value |
| `IconNumber` | v2 | variable | -- | -- | refused: this runtime has no tray icon. |
| `IconTip` | v2 | variable | -- | -- | value |
| `IL_Add` | v2 | command | -- | 2 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `IL_Create` | v2 | command | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `IL_Destroy` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `InputBox` | v2 | command | -- | 0 to 4 | value |
| `InputBox` | v1 | command | -- | up to 1 | value |
| `Gui.Menu.Insert` | v2 | method | Gui.Menu | -- | value |
| `Gui.StatusBar.Insert` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.StatusBar.InsertCol` | v2 | method | Gui.StatusBar | -- | value |
| `ListLines` | v2 | command | -- | 0 to 1 | value |
| `ListLines` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ListVars` | v2 | command | -- | 0 | refused: the variables are listed in a window of v2, and this runtime has no window to show them in. |
| `ListVars` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ListViewGetContent` | v2 | command | -- | 0 to 6 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.MarginX` | v2 | method | Gui | -- | value |
| `Gui.MarginY` | v2 | method | Gui | -- | value |
| `Gui.Maximize` | v2 | method | Gui | -- | value |
| `Menu` | v2 | class | -- | -- | value |
| `Menu` | v1 | command | -- | up to 2 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.MenuBar` | v2 | method | Gui | -- | value |
| `MenuFromHandle` | v2 | command | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `MenuGetHandle` | v1 | function | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `MenuGetName` | v1 | function | -- | 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `MenuSelect` | v2 | command | -- | 0 to 11 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Minimize` | v2 | method | Gui | -- | value |
| `Gui.ListView.Modify` | v2 | method | Gui.ListView | -- | value |
| `Gui.StatusBar.Modify` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.Modify` | v2 | method | Gui.TreeView | -- | value |
| `Gui.StatusBar.ModifyCol` | v2 | method | Gui.StatusBar | -- | value |
| `MonitorGet` | v2 | command | -- | 0 to 5 | value |
| `MonitorGetCount` | v2 | command | -- | 0 | value |
| `MonitorGetName` | v2 | command | -- | 0 to 1 | value |
| `MonitorGetPrimary` | v2 | command | -- | 0 | value |
| `MonitorGetWorkArea` | v2 | command | -- | 0 to 5 | value |
| `Gui.Move` | v2 | method | Gui | -- | value |
| `Gui.Control.Move` | v2 | method | Gui.Control | -- | value |
| `MsgBox` | v2 | command | -- | 0 to 3 | value |
| `MsgBox` | v1 | command | -- | up to 0 | value |
| `Gui.Name` | v2 | method | Gui | -- | value |
| `Gui.Control.Name` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.OnCommand` | v2 | method | Gui.Control | -- | value |
| `Gui.OnEvent` | v2 | method | Gui | -- | value |
| `Gui.Control.OnEvent` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.OnNotify` | v2 | method | Gui.Control | -- | value |
| `Gui.Opt` | v2 | method | Gui | -- | value |
| `Gui.Control.Opt` | v2 | method | Gui.Control | -- | value |
| `PostMessage` | v2 | command | -- | 1 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `PostMessage` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Progress` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Control.Redraw` | v2 | method | Gui.Control | -- | value |
| `RegisterCallback` | v1 | function | -- | 1 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.Rename` | v2 | method | Gui.Menu | -- | value |
| `Gui.Restore` | v2 | method | Gui | -- | value |
| `ScriptHwnd` | v2 | variable | -- | -- | refused: this runtime has no window of its own: it is a script that runs and ends. |
| `SendMessage` | v2 | command | -- | 1 to 9 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.SetColor` | v2 | method | Gui.Menu | -- | value |
| `SetControlDelay` | v2 | command | -- | 1 | value |
| `SetControlDelay` | v1 | command | -- | up to 1 | value |
| `Gui.SetFont` | v2 | method | Gui | -- | value |
| `Gui.Control.SetFont` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.SetFormat` | v2 | method | Gui.Control | -- | value |
| `Gui.ListView.SetIcon` | v2 | method | Gui.ListView | -- | value |
| `Gui.Menu.SetIcon` | v2 | method | Gui.Menu | -- | value |
| `Gui.TreeView.SetIcon` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.SetImageList` | v2 | method | Gui.ListView | -- | value |
| `Gui.StatusBar.SetImageList` | v2 | method | Gui.StatusBar | -- | value |
| `Gui.TreeView.SetImageList` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.SetParts` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.SetParts` | v2 | method | Gui.TreeView | -- | value |
| `Gui.ListView.SetText` | v2 | method | Gui.ListView | -- | value |
| `Gui.TreeView.SetText` | v2 | method | Gui.TreeView | -- | value |
| `SetTitleMatchMode` | v2 | command | -- | 1 | value |
| `SetTitleMatchMode` | v1 | command | -- | up to 1 | value |
| `SetWinDelay` | v2 | command | -- | 1 | value |
| `SetWinDelay` | v1 | command | -- | up to 1 | value |
| `Gui.Show` | v2 | method | Gui | -- | value |
| `Gui.Menu.Show` | v2 | method | Gui.Menu | -- | value |
| `SplashTextOn` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `StatusBarGetText` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `StatusBarGetText` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `StatusBarWait` | v2 | command | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `StatusBarWait` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Submit` | v2 | method | Gui | -- | value |
| `Gui.Control.Text` | v2 | method | Gui.Control | -- | value |
| `Gui.Title` | v2 | method | Gui | -- | value |
| `TitleMatchMode` | v2 | variable | -- | -- | value |
| `TitleMatchModeSpeed` | v2 | variable | -- | -- | value |
| `Gui.Menu.ToggleCheck` | v2 | method | Gui.Menu | -- | value |
| `Gui.Menu.ToggleEnable` | v2 | method | Gui.Menu | -- | value |
| `ToolTip` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `ToolTip` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `TrayMenu` | v2 | variable | -- | -- | refused: this runtime has no tray icon. |
| `TraySetIcon` | v2 | command | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `TrayTip` | v2 | command | -- | 0 to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `TrayTip` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `Gui.Menu.Uncheck` | v2 | method | Gui.Menu | -- | value |
| `Gui.Control.UseTab` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.Value` | v2 | method | Gui.Control | -- | value |
| `Gui.Control.Visible` | v2 | method | Gui.Control | -- | value |
| `WinActivate` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinActivate` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinActivateBottom` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinActivateBottom` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinActive` | v2 | function | -- | 0 to 4 | value |
| `WinActive` | v1 | function | -- | 0 to 4 | value |
| `WinClose` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinClose` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinDelay` | v2 | variable | -- | -- | value |
| `WinDir` | v2 | variable | -- | -- | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinExist` | v2 | function | -- | 0 to 4 | value |
| `WinExist` | v1 | function | -- | 0 to 4 | value |
| `WinFromPoint` | ks | function | -- | -- | value |
| `WinGet` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetActiveStats` | v1 | command | -- | up to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetActiveTitle` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetAlwaysOnTop` | ks | function | -- | -- | value |
| `WinGetClass` | v2 | command | -- | 0 to 4 | value |
| `WinGetClass` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetClientPos` | v2 | command | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetControls` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetControlsHwnd` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetCount` | v2 | command | -- | 0 to 4 | value |
| `WinGetEnabled` | ks | function | -- | -- | value |
| `WinGetExStyle` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetID` | v2 | command | -- | 0 to 4 | value |
| `WinGetIDLast` | v2 | command | -- | 0 to 4 | value |
| `WinGetList` | v2 | command | -- | 0 to 4 | value |
| `WinGetMinMax` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetPID` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetPos` | v2 | command | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetPos` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetProcessName` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetProcessPath` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetStyle` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetText` | v2 | command | -- | 0 to 4 | refused: this runtime reads no text of a window. |
| `WinGetText` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetTitle` | v2 | command | -- | 0 to 4 | value |
| `WinGetTitle` | v1 | command | -- | up to 1 | value |
| `WinGetTransColor` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinGetTransparent` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinHide` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinHide` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinKill` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinKill` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMaximize` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMaximize` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMenuSelectItem` | v1 | command | -- | up to 3 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMinimize` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMinimize` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMinimizeAll` | v2 | command | -- | 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMinimizeAll` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMinimizeAllUndo` | v2 | command | -- | 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMove` | v2 | command | -- | 0 to 8 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMove` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMoveBottom` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinMoveTop` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinRedraw` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinRestore` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinRestore` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSet` | v1 | command | -- | up to 1 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetAlwaysOnTop` | v2 | command | -- | 0 to 5 | value |
| `WinSetEnabled` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetExStyle` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetRegion` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetStyle` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetTitle` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetTitle` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetTransColor` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinSetTransparent` | v2 | command | -- | 1 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinShow` | v2 | command | -- | 0 to 4 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinShow` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWait` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWait` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWaitActive` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWaitActive` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWaitClose` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWaitClose` | v1 | command | -- | up to 0 | refused: this is a name of Windows, and this desktop is not Windows. |
| `WinWaitNotActive` | v2 | command | -- | 0 to 5 | refused: this is a name of Windows, and this desktop is not Windows. |
