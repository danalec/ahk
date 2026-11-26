# SPDX-License-Identifier: Apache-2.0
"""Authored reference prose, batch 5: input + misc."""

E = {}
def add(n, d, s, r, x, rel=None, p=None):
    E[n] = {"desc": d, "syntax": s, "params": p or [], "returns": r, "example": x, "related": rel or []}

# -- InputHook properties ----------------------------------------------------
for n, d in [
    ("BackspaceIsUndo", "Whether backspace undoes the last char."),
    ("BufferLengthMax", "The max length of the input buffer."),
    ("CaseSensitive", "Whether the match is case-sensitive."),
    ("EndCharMode", "How the end char is handled."),
    ("EndKey", "The key that ended the input."),
    ("EndMods", "The modifiers at the end."),
    ("EndReason", "Why the input ended (EndKey/Timeout/Max/... )."),
    ("EventInfo", "Extra info about the input event."),
    ("FindAnywhere", "Whether the match may start anywhere."),
    ("InProgress", "Whether the input is running."),
    ("Match", "The current match text."),
    ("NotifyNonText", "Whether non-text keys fire events."),
    ("TranscribeModifiedKeys", "Whether modified keys are transcribed."),
    ("VisibleMouseMove", "Whether the mouse moves visibly."),
    ("VisibleNonText", "Whether non-text keys are visible."),
    ("VisibleText", "Whether typed text is visible."),
    ("Timeout", "The timeout of the input (seconds)."),
    ("MinSendLevel", "The min send level the hook sees."),
]:
    add(n, "InputHook property: " + d, "V := InputHook." + n, "The value; assigning sets it.", "ih." + n, ["InputHook", "Input"])
# -- InputHook methods -------------------------------------------------------
for n, d in [
    ("OnChar", "Sets the callback for a typed char."),
    ("OnEnd", "Sets the callback for the end of the input."),
    ("OnKeyDown", "Sets the callback for a key down."),
    ("OnKeyUp", "Sets the callback for a key up."),
    ("OnMouseDown", "Sets the callback for a mouse button down."),
    ("OnMouseMove", "Sets the callback for mouse movement."),
    ("OnMouseUp", "Sets the callback for a mouse button up."),
]:
    add(n, "InputHook: " + d, "InputHook." + n + "(Callback)", "Nothing.", "ih." + n + "(Fn)", ["InputHook", "Input"])
add("Start", "Starts the input hook.", "InputHook.Start()", "Nothing.", "ih.Start()", ["InputHook", "Stop", "Wait"])
add("Stop", "Stops the input hook.", "InputHook.Stop()", "Nothing.", "ih.Stop()", ["InputHook", "Start"])
add("Wait", "Waits for the input to end.", "S := InputHook.Wait(MaxSeconds)", "The EndReason.", "ih.Wait()", ["InputHook", "Start", "Stop"])
add("Input", "Reads input from the keyboard (v1).", "Input OutputVar, Options, EndKeys, MatchList", "Nothing; the var is filled.", "Input v, L5", ["InputHook", "KeyWait"])
add("InputHook", "Creates an input-hook object.", "IH := InputHook(Options, EndKeys, MatchList)", "The InputHook.", "ih := InputHook()", ["Input", "Hotkey"])
# -- HotIf -------------------------------------------------------------------
for n, d in [
    ("HotIf", "Makes hotkeys context-sensitive by a function/expression."),
    ("HotIfWinActive", "Context by active window."),
    ("HotIfWinExist", "Context by existing window."),
    ("HotIfWinNotActive", "Context by not-active window."),
    ("HotIfWinNotExist", "Context by not-existing window."),
]:
    add(n, d, n + " Criteria", "Nothing (a directive-like function).", n + " \"ahk_class X\"", ["Hotkey", "#If"])
# -- settings ----------------------------------------------------------------
for n, d, s in [
    ("SetKeyDelay", "Sets the delay/duration of Send keys.", "SetKeyDelay Delay, Duration, Play"),
    ("SetMouseDelay", "Sets the delay of mouse commands.", "SetMouseDelay Delay, Play"),
    ("SetCapsLockState", "Sets the CapsLock state.", "SetCapsLockState On|Off|AlwaysOn|AlwaysOff"),
    ("SetNumLockState", "Sets the NumLock state.", "SetNumLockState On|Off|AlwaysOn|AlwaysOff"),
    ("SetScrollLockState", "Sets the ScrollLock state.", "SetScrollLockState On|Off|AlwaysOn|AlwaysOff"),
    ("SetStoreCapsLockMode", "Sets whether CapsLock mode is stored.", "SetStoreCapsLockMode On|Off"),
    ("SetDefaultMouseSpeed", "Sets the default mouse speed.", "SetDefaultMouseSpeed Speed"),
    ("CoordModeMouse", "Sets the mouse coordinate mode.", "CoordModeMouse Client|Screen|Window"),
    ("CoordModePixel", "Sets the pixel coordinate mode.", "CoordModePixel Client|Screen|Window"),
    ("SetBatchLines", "Sets the v1 batch lines.", "SetBatchLines Lines|ms"),
]:
    add(n, d, s, "Nothing.", s.split(" ")[0], ["Sleep", "Send"])
for n, d in [
    ("KeyDelay", "The Send key delay."), ("KeyDelayPlay", "The SendPlay key delay."),
    ("KeyDuration", "The Send key duration."), ("KeyDurationPlay", "The SendPlay key duration."),
    ("MouseDelay", "The mouse delay."), ("MouseDelayPlay", "The SendPlay mouse delay."),
    ("DefaultMouseSpeed", "The default mouse speed."), ("StoreCapsLockMode", "Whether CapsLock mode is stored."),
    ("HotkeyInterval", "The hotkey-rate interval."), ("HotkeyModifierTimeout", "The hotkey modifier timeout."),
    ("MaxHotkeysPerInterval", "The max hotkeys per interval."), ("MenuMaskKey", "The Win-mask key."),
    ("KeyHistory", "The key-history size."), ("PriorHotkey", "The previous hotkey."),
    ("PriorKey", "The previous key."), ("ThisHotkey", "The current hotkey."),
    ("TimeIdle", "ms since any input."), ("TimeIdleKeyboard", "ms since keyboard input."),
    ("TimeIdleMouse", "ms since mouse input."), ("TimeIdlePhysical", "ms since physical input."),
    ("TimeSincePriorHotkey", "ms since the prior hotkey."), ("TimeSinceThisHotkey", "ms since this hotkey."),
]:
    add(n, d, "x := " + n, "The value.", "x := " + n, ["SetKeyDelay", "A_TimeIdle"])
add("ListHotkeys", "Lists the hotkeys (shows the key history).", "ListHotkeys", "Nothing.", "ListHotkeys", ["KeyHistory", "Hotkey"])
add("IsLabel", "Whether a name is a label (v1).", "B := IsLabel(Name)", "1/0.", "IsLabel(\"L\")", ["Goto", "Gosub"])
add("ClipWait", "Waits for the clipboard to hold data.", "ClipWait Timeout, WaitForAny", "Nothing; throws TimeoutError.", "ClipWait 2", ["ClipboardAll", "A_Clipboard"])
add("ClipCursor", "Confines the pointer to a rect (ks).", "ClipCursor(X1, Y1, X2, Y2)", "Nothing; no rect frees.", "ClipCursor(0, 0, 100, 100)", ["MouseMove", "MouseGetPos"])
add("MouseClickDrag", "Drags the mouse from one point to another.", "MouseClickDrag Button, X1, Y1, X2, Y2, Speed", "Nothing.", "MouseClickDrag \"L\", 0, 0, 100, 100", ["Click", "MouseMove"])
add("CallbackCreate", "Creates a callback address for a function (ks).", "P := CallbackCreate(Func, Options, ParamCount)", "The address.", "p := CallbackCreate(Fn)", ["RegisterCallback", "CallbackFree", "DllCall"])
add("CallbackFree", "Frees a callback address.", "CallbackFree Ptr", "Nothing.", "CallbackFree p", ["CallbackCreate"])
add("BeforeHotkeys", "The hotkey-before hook (ks).", "x := Hotkey.BeforeHotkeys", "The value.", "hk.BeforeHotkeys", ["Hotkey"])

# -- misc: typed numbers -----------------------------------------------------
for n, d in [("Int8", "a signed 8-bit"), ("UInt8", "an unsigned 8-bit"), ("Int16", "a signed 16-bit"), ("UInt16", "an unsigned 16-bit"), ("Int32", "a signed 32-bit"), ("UInt32", "an unsigned 32-bit"), ("Int64", "a signed 64-bit"), ("IntPtr", "a pointer-width int"), ("Float32", "a 32-bit float"), ("Float64", "a 64-bit float")]:
    add(n, "The typed-number type " + d + " (ks).", "x := " + n + "(Value)", "The typed value.", n + "(1)", ["Integer", "Float", "NumPut"])
# -- misc math ---------------------------------------------------------------
add("ATan2", "The arc-tangent of y/x (radians).", "N := ATan2(Y, X)", "The angle.", "ATan2(1, 1)", ["ATan", "Sin", "Cos"])
add("Sinh", "The hyperbolic sine.", "N := Sinh(X)", "The value.", "Sinh(1)", ["Cosh", "Tanh"])
add("Cosh", "The hyperbolic cosine.", "N := Cosh(X)", "The value.", "Cosh(1)", ["Sinh", "Tanh"])
add("Tanh", "The hyperbolic tangent.", "N := Tanh(X)", "The value.", "Tanh(1)", ["Sinh", "Cosh"])
add("True", "The boolean 1.", "True", "1.", "x := True", ["False"])
add("False", "The boolean 0.", "False", "0.", "x := False", ["True"])
add("Asc", "The code point of the first char (v1).", "N := Asc(String)", "The code point.", "Asc(\"A\")", ["Chr", "Ord"])
add("At", "The item of a collection at an index (ks).", "V := Arr.At(Index)", "The value.", "a.At(1)", ["Array", "Get"])
add("Args", "The command-line args (ks).", "A := Args", "The Array.", "x := Args", ["A_Args"])
add("PropRef", "A reference to a property (ks).", "r := PropRef(Obj, Name)", "The reference.", "r := PropRef(o, \"x\")", ["VarRef"])
add("Props", "The properties of an object (ks).", "A := Props(Obj)", "The Array.", "Props(o)", ["OwnProps"])
add("Id", "The id of a thread/process (ks).", "N := x.Id", "The id.", "t.Id", ["Thread", "A_Thread"])
add("Kind", "The kind of a thread (ks).", "S := x.Kind", "The kind.", "t.Kind", ["Thread"])
add("Priority", "The priority of a thread (ks).", "N := x.Priority", "The priority.", "t.Priority", ["Thread", "ProcessSetPriority"])
add("Paused", "Whether a thread is paused (ks).", "B := x.Paused", "1/0.", "t.Paused", ["Thread", "Pause"])
add("Elapsed", "The elapsed time of a thread (ks).", "N := x.Elapsed", "The ms.", "t.Elapsed", ["Thread", "A_TickCount"])
add("AllowTimers", "Whether a thread allows timers (ks).", "B := x.AllowTimers", "1/0.", "t.AllowTimers", ["Thread", "SetTimer"])
add("Underlying", "The underlying value of a typed number (ks).", "V := x.Underlying", "The value.", "n.Underlying", ["Int32", "Float64"])
add("Empty", "Whether a collection is empty (ks).", "B := x.Empty", "1/0.", "a.Empty", ["Array", "Count"])
add("ReplaceLineEndings", "Normalizes the line endings of a string (ks).", "S := ReplaceLineEndings(Text, Ending)", "The text.", "ReplaceLineEndings(s, \"`n\")", ["StrReplace"])
add("RandomSeed", "Sets the random seed (ks).", "RandomSeed Seed", "Nothing.", "RandomSeed 42", ["Random"])
add("ErrorLevel", "The v1 error level of the last command.", "ErrorLevel", "The number/text.", "x := ErrorLevel", ["OnError"])
# -- v1 ----------------------------------------------------------------------
add("IfInString", "v1: if a substring is present.", "IfInString Var, Search", "A conditional.", "IfInString s, \"x\"", ["InStr", "IfNotInString"])
add("IfNotInString", "v1: if a substring is absent.", "IfNotInString Var, Search", "A conditional.", "IfNotInString s, \"x\"", ["InStr", "IfInString"])
add("IfMsgBox", "v1: if the last MsgBox answered a button.", "IfMsgBox Button", "A conditional.", "IfMsgBox Yes", ["MsgBox"])
add("Transform", "v1: transforms a value ( Asc/Chr/... ).", "Transform Out, Cmd, Value", "Nothing.", "Transform o, Asc, \"A\"", ["Asc", "Chr"])
add("SplashTextOn", "v1: shows a splash text window.", "SplashTextOn W, H, Title, Text", "Nothing.", "SplashTextOn 200, 100, \"t\", \"x\"", ["SplashTextOff", "ToolTip"])
add("AutoTrim", "v1: sets auto-trim.", "AutoTrim On|Off", "Nothing.", "AutoTrim Off", ["Trim"])
add("Drive", "v1: operates on a drive.", "Drive Cmd, Drive, Value", "Nothing.", "Drive \"Eject\", \"D:\"", ["DriveGet"])
add("DriveSpaceFree", "v1: free space of a drive.", "DriveSpaceFree Out, Path", "Nothing.", "DriveSpaceFree f, \"C:\\\"", ["DriveGetSpaceFree"])
# -- ks ----------------------------------------------------------------------
add("Mail", "Sends a mail message through an SMTP server (ks).", "Mail(To, Subject, Body, ...)", "Nothing.", "Mail(\"a@b\", \"s\", \"body\")", ["URLDownloadToFile"])
add("GetKeyboardLayout", "The keymap of the session (ks).", "S := GetKeyboardLayout()", "The layout.", "l := GetKeyboardLayout()", ["GetKeyInfo", "A_Language"])
add("GetKeyInfo", "The state of a key (ks).", "S := GetKeyInfo(Key)", "The info.", "GetKeyInfo(\"a\")", ["GetKeyState", "GetKeyboardLayout"])
add("ShowDebug", "Shows the debug window (ks).", "ShowDebug()", "Nothing.", "ShowDebug()", ["OutputDebug", "OutputDebugLine"])
add("OutputDebugLine", "Writes a line to the debug log (ks).", "OutputDebugLine Text", "Nothing.", "OutputDebugLine \"x\"", ["OutputDebug", "ShowDebug"])
add("CompileScript", "Compiles a script to a string/bytes (ks).", "B := CompileScript(Path)", "The compiled form.", "CompileScript(\"a.ahk\")", ["ValidateScript", "RunScript"])
add("ValidateScript", "Validates a script's syntax (ks).", "B := ValidateScript(Path)", "1/0 or the errors.", "ValidateScript(\"a.ahk\")", ["CompileScript"])
add("RequestCapabilities", "Requests the capabilities of the host (ks).", "C := RequestCapabilities()", "The capabilities.", "RequestCapabilities()", ["A_OSType"])
add("Collect", "Collects garbage (ks).", "Collect()", "Nothing.", "Collect()", ["ObjRelease"])
add("ArgumentError", "An error of a wrong argument (ks).", "e := ArgumentError(...)", "The error.", "throw ArgumentError()", ["Error", "TypeError"])
add("UnsupportedError", "An error of an unsupported operation (ks).", "e := UnsupportedError(...)", "The error.", "throw UnsupportedError()", ["Error"])

EXTRA5 = E
