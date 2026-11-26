# SPDX-License-Identifier: Apache-2.0
"""Authored reference prose, batch 6: the GUI / windows family."""

E = {}
def add(n, d, s, r, x, rel=None, p=None):
    E[n] = {"desc": d, "syntax": s, "params": p or [], "returns": r, "example": x, "related": rel or []}

TYPES = ["ActiveX", "Button", "CheckBox", "ComboBox", "Custom", "DDL", "DateTime", "DropDownList", "Edit", "GroupBox", "Hotkey", "Link", "ListBox", "ListView", "MonthCal", "Pic", "Picture", "Progress", "Radio", "Slider", "Standard", "StatusBar", "Tab", "Tab2", "Tab3", "Text", "TreeView", "UpDown"]
for t in TYPES:
    add("Add" + t, f"Adds a {t} control to the GUI and answers it.", "C := Gui.Add" + t + "(Options, Text)", "The control object.", "g.Add" + t + "(, \"x\")", ["Gui", "Gui." + t])
for t in ["ActiveX", "Button", "CheckBox", "ComboBox", "Control", "Custom", "DDL", "DateTime", "Edit", "GroupBox", "Hotkey", "Link", "List", "ListBox", "ListView", "MonthCal", "Pic", "Progress", "Radio", "Slider", "StatusBar", "Tab", "Text", "TreeView", "UpDown"]:
    add("Gui." + t, f"The class of the {t} control of a GUI.", "Gui." + t, "The class.", "c := g.Add(\"" + t + "\")", ["Gui", "Add" + t])

# -- Gui methods / properties ------------------------------------------------
for n, d, s, r, x in [
    ("Show", "Shows the GUI window.", "Gui.Show(Options)", "Nothing.", "g.Show()"),
    ("Hide", "Hides the GUI window.", "Gui.Hide()", "Nothing.", "g.Hide()"),
    ("Destroy", "Destroys the GUI window.", "Gui.Destroy()", "Nothing.", "g.Destroy()"),
    ("Submit", "Hides the GUI and collects the control values.", "Gui.Submit(Hide)", "An object of the values.", "v := g.Submit()"),
    ("SetFont", "Sets the font of the GUI.", "Gui.SetFont(Options, FontName)", "Nothing.", "g.SetFont(\"s10\", \"Segoe UI\")"),
    ("SetColor", "Sets the background/text colour.", "Gui.SetColor(Bg, Text)", "Nothing.", "g.SetColor(\"White\")"),
    ("SetIcon", "Sets the GUI/tray icon.", "Gui.SetIcon(Filename, Number)", "Nothing.", "g.SetIcon(\"a.ico\")"),
    ("Add", "Adds a control by type name.", "C := Gui.Add(Type, Options, Text)", "The control.", "g.Add(\"Text\", , \"x\")"),
    ("Opt", "Sets GUI options/styles.", "Gui.Opt(Options)", "Nothing.", "g.Opt(\"+Resize\")"),
    ("GetPos", "Reads the GUI position/size.", "Gui.GetPos(&X, &Y, &W, &H)", "Nothing.", "g.GetPos(&x, &y)"),
    ("GetClientPos", "Reads the client-area position/size.", "Gui.GetClientPos(&X, &Y, &W, &H)", "Nothing.", "g.GetClientPos(&x, &y)"),
    ("Move", "Moves/resizes the GUI.", "Gui.Move(X, Y, W, H)", "Nothing.", "g.Move(0, 0, 300, 200)"),
    ("Maximize", "Maximizes the GUI.", "Gui.Maximize()", "Nothing.", "g.Maximize()"),
    ("Minimize", "Minimizes the GUI.", "Gui.Minimize()", "Nothing.", "g.Minimize()"),
    ("Restore", "Restores the GUI.", "Gui.Restore()", "Nothing.", "g.Restore()"),
    ("Redraw", "Redraws the GUI.", "Gui.Redraw()", "Nothing.", "g.Redraw()"),
    ("Flash", "Flashes the GUI taskbar button.", "Gui.Flash(Off)", "Nothing.", "g.Flash()"),
    ("SetTitleMatchMode", "Sets the title match mode for the GUI.", "Gui.SetTitleMatchMode(Mode)", "Nothing.", "g.SetTitleMatchMode(2)"),
    ("OnEvent", "Registers a GUI event callback.", "Gui.OnEvent(Event, Callback, Add)", "Nothing.", "g.OnEvent(\"Close\", Fn)"),
    ("OnCommand", "Registers a GUI command callback.", "Gui.OnCommand(Cmd, Callback)", "Nothing.", "g.OnCommand(1, Fn)"),
    ("OnNotify", "Registers a GUI notify callback.", "Gui.OnNotify(Code, Callback)", "Nothing.", "g.OnNotify(0, Fn)"),
]:
    add(n, "Gui: " + d, s, r, x, ["Gui"])
for n, d in [
    ("BackColor", "The background colour of the GUI/control."),
    ("Title", "The title of the GUI."),
    ("Name", "The name of the GUI."),
    ("Hwnd", "The window handle of the GUI."),
    ("MenuBar", "The menu bar of the GUI."),
    ("MarginX", "The horizontal margin between controls."),
    ("MarginY", "The vertical margin between controls."),
    ("Visible", "Whether the GUI/control is visible."),
    ("Enabled", "Whether the GUI/control is enabled."),
    ("Focused", "Whether the control has focus."),
    ("Value", "The value of the control."),
    ("Text", "The text of the control."),
    ("ClassNN", "The ClassNN of the control."),
    ("FocusedCtrl", "The focused control of the GUI."),
]:
    add(n, "GUI/control property: " + d, "V := x." + n, "The value; assigning sets it.", "x." + n, ["Gui", "Control"])
add("Default", "The default GUI/control.", "x := Gui.Default", "The identifier.", "g.Default", ["Gui"])
add("Control", "The control object of a GUI.", "C := Gui.Control", "The control.", "g[\"Edit1\"]", ["Gui", "ClassNN"])
add("GuiFromHwnd", "The GUI object of a window handle.", "G := GuiFromHwnd(Hwnd)", "The GUI.", "GuiFromHwnd(h)", ["Gui", "Hwnd"])
add("GuiCtrlFromHwnd", "The control object of a control handle.", "C := GuiCtrlFromHwnd(Hwnd)", "The control.", "GuiCtrlFromHwnd(h)", ["Gui", "Control"])
add("GuiControl", "Operates on a control (v1).", "GuiControl Cmd, ControlID, Value", "Nothing.", "GuiControl \"\", \"Edit1\", \"x\"", ["ControlSetText", "Gui"])
add("GuiControlGet", "Reads a control (v1).", "GuiControlGet Out, Cmd, ControlID, Value", "Nothing.", "GuiControlGet t, , \"Edit1\"", ["ControlGetText", "Gui"])

# -- Control* ----------------------------------------------------------------
for n, d, s, r, x in [
    ("ControlAddItem", "Adds an item to a list control.", "ControlAddItem Text, Control, WinTitle", "The index.", "ControlAddItem \"x\", \"ListBox1\""),
    ("ControlChooseIndex", "Chooses an item by index.", "ControlChooseIndex N, Control, WinTitle", "Nothing.", "ControlChooseIndex 1, \"ListBox1\""),
    ("ControlChooseString", "Chooses an item by text.", "ControlChooseString Text, Control, WinTitle", "Nothing.", "ControlChooseString \"x\", \"ListBox1\""),
    ("ControlDeleteItem", "Deletes an item by index.", "ControlDeleteItem N, Control, WinTitle", "Nothing.", "ControlDeleteItem 1, \"ListBox1\""),
    ("ControlFindItem", "Finds an item by text.", "N := ControlFindItem(Text, Control, WinTitle)", "The index or 0.", "ControlFindItem(\"x\", \"ListBox1\")"),
    ("ControlFocus", "Focuses a control.", "ControlFocus Control, WinTitle", "Nothing.", "ControlFocus \"Edit1\""),
    ("ControlGetChecked", "Reads whether a checkbox is checked.", "B := ControlGetChecked(Control, WinTitle)", "1/0.", "ControlGetChecked(\"CheckBox1\")"),
    ("ControlGetChoice", "Reads the chosen item of a list.", "S := ControlGetChoice(Control, WinTitle)", "The text.", "ControlGetChoice(\"ComboBox1\")"),
    ("ControlGetClassNN", "Reads the ClassNN of a control.", "S := ControlGetClassNN(Control, WinTitle)", "The ClassNN.", "ControlGetClassNN(h)"),
    ("ControlGetEnabled", "Reads whether a control is enabled.", "B := ControlGetEnabled(Control, WinTitle)", "1/0.", "ControlGetEnabled(\"Edit1\")"),
    ("ControlGetFocus", "Reads the focused control.", "S := ControlGetFocus(WinTitle)", "The ClassNN.", "ControlGetFocus(\"A\")"),
    ("ControlGetHwnd", "Reads the handle of a control.", "H := ControlGetHwnd(Control, WinTitle)", "The handle.", "ControlGetHwnd(\"Edit1\")"),
    ("ControlGetIndex", "Reads the chosen index of a list.", "N := ControlGetIndex(Control, WinTitle)", "The index.", "ControlGetIndex(\"ListBox1\")"),
    ("ControlGetItems", "Reads the items of a list.", "A := ControlGetItems(Control, WinTitle)", "The Array.", "ControlGetItems(\"ListBox1\")"),
    ("ControlGetPos", "Reads the position of a control.", "ControlGetPos &X, &Y, &W, &H, Control, WinTitle", "Nothing.", "ControlGetPos &x, &y, &w, &h, \"Edit1\""),
    ("ControlGetVisible", "Reads whether a control is visible.", "B := ControlGetVisible(Control, WinTitle)", "1/0.", "ControlGetVisible(\"Edit1\")"),
    ("ControlHide", "Hides a control.", "ControlHide Control, WinTitle", "Nothing.", "ControlHide \"Edit1\""),
    ("ControlHideDropDown", "Hides the drop-down of a combo.", "ControlHideDropDown Control, WinTitle", "Nothing.", "ControlHideDropDown \"ComboBox1\""),
    ("ControlMove", "Moves/resizes a control.", "ControlMove X, Y, W, H, Control, WinTitle", "Nothing.", "ControlMove 0, 0, 100, 20, \"Edit1\""),
    ("ControlSendText", "Sends text to a control.", "ControlSendText Text, Control, WinTitle", "Nothing.", "ControlSendText \"x\", \"Edit1\""),
    ("ControlSetChecked", "Sets a checkbox's state.", "ControlSetChecked B, Control, WinTitle", "Nothing.", "ControlSetChecked 1, \"CheckBox1\""),
    ("ControlSetEnabled", "Enables/disables a control.", "ControlSetEnabled B, Control, WinTitle", "Nothing.", "ControlSetEnabled 0, \"Edit1\""),
    ("ControlSetText", "Sets the text of a control.", "ControlSetText Text, Control, WinTitle", "Nothing.", "ControlSetText \"x\", \"Edit1\""),
    ("ControlShow", "Shows a control.", "ControlShow Control, WinTitle", "Nothing.", "ControlShow \"Edit1\""),
    ("ControlShowDropDown", "Shows the drop-down of a combo.", "ControlShowDropDown Control, WinTitle", "Nothing.", "ControlShowDropDown \"ComboBox1\""),
    ("ControlClick", "Clicks a control.", "ControlClick Control, WinTitle, Button, Count", "Nothing.", "ControlClick \"Button1\""),
    ("ControlGet", "Reads a control property (v1).", "ControlGet Out, Cmd, Value, Control, WinTitle", "Nothing.", "ControlGet t, \"Text\", , \"Edit1\""),
    ("ControlDelay", "The control delay.", "x := ControlDelay", "The ms.", "x := ControlDelay"),
    ("SetControlDelay", "Sets the control delay.", "SetControlDelay Delay", "Nothing.", "SetControlDelay 10"),
]:
    add(n, d, s, r, x, ["ControlSend", "Gui"])

# -- Edit* / ListView / IL / StatusBar ---------------------------------------
for n, d, s, r, x in [
    ("EditGetCurrentCol", "The current column of an edit.", "N := EditGetCurrentCol(Control, WinTitle)", "The column.", "EditGetCurrentCol(\"Edit1\")"),
    ("EditGetCurrentLine", "The current line of an edit.", "N := EditGetCurrentLine(Control, WinTitle)", "The line.", "EditGetCurrentLine(\"Edit1\")"),
    ("EditGetLine", "Reads a line of an edit.", "S := EditGetLine(N, Control, WinTitle)", "The line.", "EditGetLine(1, \"Edit1\")"),
    ("EditGetLineCount", "The number of lines of an edit.", "N := EditGetLineCount(Control, WinTitle)", "The count.", "EditGetLineCount(\"Edit1\")"),
    ("EditGetSelectedText", "The selected text of an edit.", "S := EditGetSelectedText(Control, WinTitle)", "The text.", "EditGetSelectedText(\"Edit1\")"),
    ("EditPaste", "Pastes text at the caret of an edit.", "EditPaste Text, Control, WinTitle", "Nothing.", "EditPaste \"x\", \"Edit1\""),
    ("ListViewGetContent", "Reads the content of a ListView.", "S := ListViewGetContent(Options, Control, WinTitle)", "The text.", "ListViewGetContent(\"\", \"SysListView321\")"),
    ("IL_Create", "Creates an image list.", "H := IL_Create(Initial, Grow, Large)", "The handle.", "h := IL_Create()"),
    ("IL_Add", "Adds an image to an image list.", "N := IL_Add(IL, Filename, Icon, Resize)", "The index.", "IL_Add(h, \"a.ico\")"),
    ("IL_Destroy", "Destroys an image list.", "IL_Destroy IL", "Nothing.", "IL_Destroy(h)"),
    ("StatusBarGetText", "Reads the text of a status bar.", "S := StatusBarGetText(Part, WinTitle)", "The text.", "StatusBarGetText(1)"),
    ("StatusBarWait", "Waits for a status bar text.", "StatusBarWait Text, Timeout, Part, WinTitle", "Nothing.", "StatusBarWait \"ready\""),
]:
    add(n, d, s, r, x, ["Gui", "ControlGetText"])

# -- Monitor* ----------------------------------------------------------------
for n, d, s, r, x in [
    ("MonitorGet", "Reads the bounds of a monitor.", "MonitorGet N, &L, &T, &R, &B", "Nothing.", "MonitorGet(1, &l, &t, &r, &b)"),
    ("MonitorGetCount", "The number of monitors.", "N := MonitorGetCount()", "The count.", "MonitorGetCount()"),
    ("MonitorGetName", "The name of a monitor.", "S := MonitorGetName(N)", "The name.", "MonitorGetName(1)"),
    ("MonitorGetPrimary", "The index of the primary monitor.", "N := MonitorGetPrimary()", "The index.", "MonitorGetPrimary()"),
    ("MonitorGetWorkArea", "Reads the work area of a monitor.", "MonitorGetWorkArea N, &L, &T, &R, &B", "Nothing.", "MonitorGetWorkArea(1, &l, &t, &r, &b)"),
]:
    add(n, d, s, r, x, ["ScreenWidth", "ScreenHeight"])

# -- Win* --------------------------------------------------------------------
for n, d, s, r, x in [
    ("WinActivateBottom", "Activates the bottom-most matching window.", "WinActivateBottom WinTitle", "Nothing.", "WinActivateBottom \"ahk_class X\""),
    ("WinGet", "Reads a window property (v1).", "WinGet Out, Cmd, WinTitle", "Nothing.", "WinGet t, \"Text\""),
    ("WinGetActiveStats", "Reads the active window's title/pos.", "WinGetActiveStats &T, &X, &Y, &W, &H", "Nothing.", "WinGetActiveStats &t, &x, &y, &w, &h"),
    ("WinGetActiveTitle", "The title of the active window (v1).", "WinGetActiveTitle Out", "Nothing.", "WinGetActiveTitle t"),
    ("WinGetAlwaysOnTop", "Whether a window is always-on-top.", "B := WinGetAlwaysOnTop(WinTitle)", "1/0.", "WinGetAlwaysOnTop(\"A\")"),
    ("WinGetControls", "The ClassNNs of a window's controls.", "A := WinGetControls(WinTitle)", "The Array.", "WinGetControls(\"A\")"),
    ("WinGetControlsHwnd", "The handles of a window's controls.", "A := WinGetControlsHwnd(WinTitle)", "The Array.", "WinGetControlsHwnd(\"A\")"),
    ("WinGetEnabled", "Whether a window is enabled.", "B := WinGetEnabled(WinTitle)", "1/0.", "WinGetEnabled(\"A\")"),
    ("WinGetID", "The id of a window.", "H := WinGetID(WinTitle)", "The HWND.", "WinGetID(\"A\")"),
    ("WinGetIDLast", "The last-found window id.", "H := WinGetIDLast()", "The HWND.", "WinGetIDLast()"),
    ("WinGetMinMax", "The min/max state of a window.", "N := WinGetMinMax(WinTitle)", "-1/0/1.", "WinGetMinMax(\"A\")"),
    ("WinGetProcessPath", "The executable path of a window's process.", "S := WinGetProcessPath(WinTitle)", "The path.", "WinGetProcessPath(\"A\")"),
    ("WinGetText", "The text of a window.", "S := WinGetText(WinTitle)", "The text.", "WinGetText(\"A\")"),
    ("WinGetTransparent", "The transparency of a window.", "N := WinGetTransparent(WinTitle)", "0..255 or empty.", "WinGetTransparent(\"A\")"),
    ("WinKill", "Forces a window to close.", "WinKill WinTitle, SecondsToWait", "Nothing.", "WinKill \"A\""),
    ("WinMenuSelectItem", "Selects a menu item of a window.", "WinMenuSelectItem WinTitle, Menu, Items...", "Nothing.", "WinMenuSelectItem \"A\", , \"File\", \"Open\""),
    ("WinMinimizeAll", "Minimizes all windows.", "WinMinimizeAll", "Nothing.", "WinMinimizeAll"),
    ("WinMinimizeAllUndo", "Undoes WinMinimizeAll.", "WinMinimizeAllUndo", "Nothing.", "WinMinimizeAllUndo"),
    ("WinMoveBottom", "Moves a window to the bottom.", "WinMoveBottom WinTitle", "Nothing.", "WinMoveBottom \"A\""),
    ("WinMoveTop", "Moves a window to the top.", "WinMoveTop WinTitle", "Nothing.", "WinMoveTop \"A\""),
    ("WinRedraw", "Redraws a window.", "WinRedraw WinTitle", "Nothing.", "WinRedraw \"A\""),
    ("WinSet", "Sets a window property (v1).", "WinSet Cmd, Value, WinTitle", "Nothing.", "WinSet \"Transparent\", 128"),
    ("WinSetAlwaysOnTop", "Sets a window always-on-top.", "WinSetAlwaysOnTop On|Off|Toggle, WinTitle", "Nothing.", "WinSetAlwaysOnTop 1, \"A\""),
    ("WinSetEnabled", "Enables/disables a window.", "WinSetEnabled B, WinTitle", "Nothing.", "WinSetEnabled 0, \"A\""),
    ("WinSetRegion", "Sets a window's region.", "WinSetRegion Options, WinTitle", "Nothing.", "WinSetRegion \"0-0 W100 H100\""),
    ("WinSetTransparent", "Sets a window's transparency.", "WinSetTransparent N, WinTitle", "Nothing.", "WinSetTransparent 128, \"A\""),
    ("WinWaitNotActive", "Waits until a window is not active.", "WinWaitNotActive WinTitle, Timeout", "Nothing.", "WinWaitNotActive \"A\""),
    ("WinFromPoint", "The window at a screen point (ks).", "H := WinFromPoint(X, Y)", "The HWND.", "WinFromPoint(x, y)"),
    ("WinDelay", "The Win* delay.", "x := WinDelay", "The ms.", "x := WinDelay"),
    ("SetWinDelay", "Sets the Win* delay.", "SetWinDelay Delay", "Nothing.", "SetWinDelay 10"),
    ("WinDir", "The Windows directory.", "x := WinDir", "The path.", "x := WinDir"),
    ("DetectHiddenText", "Sets hidden-text detection.", "DetectHiddenText On|Off", "Nothing.", "DetectHiddenText On"),
    ("DetectHiddenWindows", "Sets hidden-window detection.", "DetectHiddenWindows On|Off", "Nothing.", "DetectHiddenWindows On"),
    ("SetTitleMatchMode", "Sets the title match mode.", "SetTitleMatchMode Mode", "Nothing.", "SetTitleMatchMode 2"),
    ("TitleMatchMode", "The title match mode.", "x := TitleMatchMode", "The mode.", "x := TitleMatchMode"),
    ("TitleMatchModeSpeed", "The title match speed.", "x := TitleMatchModeSpeed", "The speed.", "x := TitleMatchModeSpeed"),
    ("CoordModeCaret", "Sets the caret coordinate mode.", "CoordModeCaret Mode", "Nothing.", "CoordModeCaret Screen"),
    ("CoordModeMenu", "Sets the menu coordinate mode.", "CoordModeMenu Mode", "Nothing.", "CoordModeMenu Screen"),
    ("CoordModeToolTip", "Sets the tooltip coordinate mode.", "CoordModeToolTip Mode", "Nothing.", "CoordModeToolTip Screen"),
    ("CaretGetPos", "Reads the caret position.", "CaretGetPos &X, &Y", "Nothing.", "CaretGetPos &x, &y"),
]:
    add(n, d, s, r, x, ["WinActivate", "WinExist"])

# -- Menu / Group / Tray / misc ----------------------------------------------
for n, d, s, r, x in [
    ("Menu", "Creates/operates a menu.", "M := Menu()", "The menu object.", "m := Menu()"),
    ("MenuFromHandle", "The menu object of a handle.", "M := MenuFromHandle(H)", "The menu.", "MenuFromHandle(h)"),
    ("MenuSelect", "Selects a menu item (v1).", "MenuSelect WinTitle, Menu, Items...", "Nothing.", "MenuSelect \"A\", , \"File\""),
    ("TrayMenu", "The tray menu object.", "M := TrayMenu", "The menu.", "TrayMenu.Add(\"x\", Fn)"),
    ("TraySetIcon", "Sets the tray icon.", "TraySetIcon Filename, Number, Freeze", "Nothing.", "TraySetIcon \"a.ico\""),
    ("GroupAdd", "Adds a window to a group.", "GroupAdd Group, WinTitle", "Nothing.", "GroupAdd \"g\", \"ahk_class X\""),
    ("GroupActivate", "Activates the next window of a group.", "GroupActivate Group", "Nothing.", "GroupActivate \"g\""),
    ("GroupClose", "Closes the active window of a group.", "GroupClose Group", "Nothing.", "GroupClose \"g\""),
    ("GroupDeactivate", "Activates the next non-group window.", "GroupDeactivate Group", "Nothing.", "GroupDeactivate \"g\""),
    ("ListLines", "Shows the lines executed (debug).", "ListLines On|Off", "Nothing.", "ListLines"),
    ("ListVars", "Shows the variables (debug).", "ListVars", "Nothing.", "ListVars"),
    ("PostMessage", "Posts a message to a window.", "PostMessage Msg, wParam, lParam, Control, WinTitle", "Nothing.", "PostMessage 0x100, 0, 0"),
    ("SendMessage", "Sends a message to a window and waits.", "R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)", "The result.", "SendMessage(0x100, 0, 0)"),
]:
    add(n, d, s, r, x, ["Menu", "Gui", "OnMessage"])
for n, d in [
    ("IconFile", "The tray icon file."), ("IconHidden", "Whether the tray icon is hidden."),
    ("IconNumber", "The tray icon number."), ("IconTip", "The tray icon tooltip."),
]:
    add(n, d, "x := " + n, "The value.", "x := " + n, ["TraySetIcon", "A_IconTip"])
# -- control object methods --------------------------------------------------
for n, d, s, r, x in [
    ("Check", "Checks a checkbox/radio.", "C.Check()", "Nothing.", "c.Check()"),
    ("Uncheck", "Unchecks a checkbox/radio.", "C.Uncheck()", "Nothing.", "c.Uncheck()"),
    ("ToggleCheck", "Toggles a checkbox.", "C.ToggleCheck()", "Nothing.", "c.ToggleCheck()"),
    ("ToggleEnable", "Toggles a control's enabled state.", "C.ToggleEnable()", "Nothing.", "c.ToggleEnable()"),
    ("Enable", "Enables a control.", "C.Enable()", "Nothing.", "c.Enable()"),
    ("Disable", "Disables a control.", "C.Disable()", "Nothing.", "c.Disable()"),
    ("Focus", "Focuses a control.", "C.Focus()", "Nothing.", "c.Focus()"),
    ("Choose", "Chooses an item of a list.", "C.Choose(N)", "Nothing.", "c.Choose(1)"),
    ("Delete", "Deletes an item of a list.", "C.Delete(N)", "Nothing.", "c.Delete(1)"),
    ("DeleteCol", "Deletes a column of a ListView.", "C.DeleteCol(N)", "Nothing.", "lv.DeleteCol(1)"),
    ("Insert", "Inserts an item of a list.", "C.Insert(N, Text)", "Nothing.", "c.Insert(1, \"x\")"),
    ("InsertCol", "Inserts a column of a ListView.", "C.InsertCol(N, Options, Text)", "Nothing.", "lv.InsertCol(1, , \"x\")"),
    ("Modify", "Modifies an item of a list.", "C.Modify(N, Options, Text)", "Nothing.", "c.Modify(1, , \"x\")"),
    ("ModifyCol", "Modifies a column of a ListView.", "C.ModifyCol(N, Options, Text)", "Nothing.", "lv.ModifyCol(1, , \"x\")"),
    ("GetCount", "The number of items of a list.", "N := C.GetCount()", "The count.", "c.GetCount()"),
    ("GetNext", "The next selected item of a list.", "N := C.GetNext(Start, Type)", "The index.", "c.GetNext()"),
    ("GetSelection", "The selected item(s) of a list.", "A := C.GetSelection()", "The selection.", "c.GetSelection()"),
    ("GetText", "The text of an item/control.", "S := C.GetText(N)", "The text.", "c.GetText(1)"),
    ("SetText", "Sets the text of a control.", "C.SetText(Text)", "Nothing.", "c.SetText(\"x\")"),
    ("SetImageList", "Sets the image list of a ListView/TreeView.", "C.SetImageList(IL, Type)", "Nothing.", "lv.SetImageList(h)"),
    ("SetParts", "Sets the parts of a status bar.", "C.SetParts(Widths...)", "Nothing.", "sb.SetParts(100, 100)"),
    ("UseTab", "Uses a tab of a Tab control.", "C.UseTab(N)", "Nothing.", "tab.UseTab(2)"),
    ("GetChild", "The child tree item.", "N := C.GetChild(Parent)", "The item.", "tv.GetChild(0)"),
    ("GetParent", "The parent tree item.", "N := C.GetParent(Item)", "The item.", "tv.GetParent(i)"),
    ("GetPrev", "The previous tree item.", "N := C.GetPrev(Item)", "The item.", "tv.GetPrev(i)"),
    ("Rename", "Renames a tree item.", "C.Rename(Item, Text)", "Nothing.", "tv.Rename(i, \"x\")"),
    ("ClickCount", "The click count of a GUI event.", "x := ClickCount", "The number.", "x := ClickCount"),
    ("AddActiveX", "Adds an ActiveX control.", "C := Gui.AddActiveX(Options, CLSID)", "The control.", "g.AddActiveX(, \"...\")"),
]:
    add(n, "Control: " + d, s, r, x, ["Gui", "Control"])
add("__Enum", "The enumerator of a collection/object.", "for k, v in Obj", "The enumerator.", "for k, v in a", ["Array", "Map", "OwnProps"])
add("__Item", "The item accessor of a collection.", "V := Obj[Index]", "The item.", "a[1]", ["Array", "Map"])
add("__New", "The constructor of a class.", "C := Class(Args...)", "The instance.", "o := C()", ["Class", "__Init"])
add("Progress", "Shows/sets a progress window (v1).", "Progress Param, SubText, MainText, Title", "Nothing.", "Progress 50", ["Gui", "AddProgress"])
add("SplashTextOn", "Shows a splash text window (v1).", "SplashTextOn W, H, Title, Text", "Nothing.", "SplashTextOn 200, 100, \"t\", \"x\"", ["Progress", "ToolTip"])
add("KeyOpt", "Sets the options of a key/hotkey (v1).", "KeyOpt Key, Options", "Nothing.", "KeyOpt \"a\", \"T\"", ["Hotkey", "Input"])
add("SetFormat", "Sets the number format (v1).", "SetFormat Type, Format", "Nothing.", "SetFormat \"Integer\", \"H\"", ["Format", "A_FormatInteger"])
add("Target", "The target of an error (the window/control that was not found).", "e.Target", "The target string.", "e.Target", ["TargetError", "Error"])

EXTRA6 = E
