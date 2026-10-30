"""Write the capability report of the surface of v2.0.19.

    python ahk-v2.0.19.areas.py <the runner> [<the report to write>]

The runner is the `ahk` of this tree, and the state of every name is the
one of the tree itself: `ahk --surface` asks the table of the built-ins, the
table of the built-in variables and the classes, and answers with a value,
with the reason of a refusal, or with the reason of a name to do. The
report is a reading of that answer, grouped by the area a name belongs to.

The areas are the ones a script thinks in: the parser and the run, the
files, the input, the windows, the screen, the native calls, the registry
and the COM objects -- and the rest of the surface, so that the numbers add
up to the whole of it.

The report is written where the second argument says (on stdout when it is
not given).
"""
import pathlib
import subprocess
import sys

# The areas of the report, and the names of each one. The names are the
# names of the surface of v2, without the `A_` of the built-in variables.
#
# A name that is in two kinds is told apart by the kind: the function `Min`
# is the math and the variable `Min` is the minute of the clock.
AREAS = [
    ("script", "Parser and script execution", """
        Critical Exit ExitApp OnError OnExit Pause Persistent Reload SetTimer
        Sleep Suspend Thread SetWorkingDir EnvGet EnvSet CoordMode Index
        ScriptDir ScriptFullPath ScriptName
        AllowMainWindow EndChar InitialWorkingDir IsAdmin Is64bitOS IsCompiled
        IsCritical IsPaused IsSuspended LineFile LineNumber Space Tab ThisFunc
        WorkingDir OnMessage
    """),
    ("files", "File and directory operations", """
        DirCopy DirCreate DirDelete DirExist DirMove DirSelect Download
        FileAppend FileCopy FileCreateShortcut FileDelete FileEncoding
        FileExist FileGetAttrib FileGetShortcut FileGetSize FileGetTime
        FileGetVersion FileInstall FileMove FileRead FileRecycle
        FileRecycleEmpty FileSelect FileSetAttrib FileSetTime
        IniDelete IniRead IniWrite
        DriveEject DriveGetCapacity DriveGetFilesystem DriveGetLabel
        DriveGetList DriveGetSerial DriveGetSpaceFree DriveGetStatus
        DriveGetStatusCD DriveGetType DriveLock DriveRetract DriveSetLabel
        DriveUnlock
        FileOpen SplitPath
        LoopField LoopFileAttrib LoopFileDir LoopFileExt LoopFileFullPath
        LoopFileName LoopFilePath LoopFileShortName LoopFileShortPath
        LoopFileSize LoopFileSizeKB LoopFileSizeMB LoopFileTimeAccessed
        LoopFileTimeCreated LoopFileTimeModified LoopReadLine
    """),
    ("input", "Hotkeys, hotstrings, and input", """
        BlockInput CallbackCreate CallbackFree ClipWait Click GetKeyName
        GetKeySC GetKeyState GetKeyVK HotIf HotIfWinActive HotIfWinExist
        HotIfWinNotActive HotIfWinNotExist Hotkey Hotstring InstallKeybdHook
        InstallMouseHook IsLabel KeyHistory KeyWait ListHotkeys
        MouseClick MouseClickDrag MouseGetPos MouseMove Send SendEvent
        SendInput SendLevel SendMode SendPlay SendText
        SetDefaultMouseSpeed SetKeyDelay SetMouseDelay
        SetStoreCapsLockMode SetCapsLockState SetNumLockState
        SetScrollLockState
        CoordModeMouse CoordModePixel Cursor DefaultMouseSpeed EventInfo
        HotkeyInterval HotkeyModifierTimeout KeyDelay KeyDelayPlay
        KeyDuration KeyDurationPlay MaxHotkeysPerInterval MenuMaskKey
        MouseDelay MouseDelayPlay PriorHotkey PriorKey SendLevel SendMode
        StoreCapsLockMode ThisHotkey TimeIdle TimeIdleKeyboard TimeIdleMouse
        TimeIdlePhysical TimeSincePriorHotkey TimeSinceThisHotkey
    """),
    ("gui", "GUI windows and window automation", """
        ControlAddItem ControlChooseIndex ControlChooseString ControlClick
        ControlDeleteItem ControlFindItem ControlFocus ControlGetChecked
        ControlGetChoice ControlGetClassNN ControlGetEnabled
        ControlGetExStyle ControlGetFocus ControlGetHwnd ControlGetIndex
        ControlGetItems ControlGetPos ControlGetStyle ControlGetText
        ControlGetVisible ControlHide ControlHideDropDown ControlMove
        ControlSend ControlSendText ControlSetChecked ControlSetEnabled
        ControlSetExStyle ControlSetStyle ControlSetText ControlShow
        ControlShowDropDown Edit EditGetCurrentCol EditGetCurrentLine
        EditGetLine EditGetLineCount EditGetSelectedText EditPaste
        GuiCtrlFromHwnd GuiFromHwnd GroupActivate GroupAdd GroupClose
        GroupDeactivate IL_Add IL_Create IL_Destroy ListLines ListVars
        ListViewGetContent MenuFromHandle MenuSelect MonitorGet
        MonitorGetCount MonitorGetName MonitorGetPrimary MonitorGetWorkArea
        MsgBox PostMessage SendMessage SetControlDelay SetTitleMatchMode
        SetWinDelay StatusBarGetText StatusBarWait ToolTip TraySetIcon
        TrayTip WinActivate WinActivateBottom WinClose WinGetClass
        WinGetClientPos WinGetControls WinGetControlsHwnd WinGetCount
        WinGetExStyle WinGetID WinGetIDLast WinGetList WinGetMinMax WinGetPID
        WinGetPos WinGetProcessName WinGetProcessPath WinGetStyle
        WinGetText WinGetTransColor WinGetTransparent WinHide WinKill
        WinMaximize WinMinimize WinMinimizeAll WinMinimizeAllUndo WinMove
        WinMoveBottom WinMoveTop WinRedraw WinRestore WinSetAlwaysOnTop
        WinSetEnabled WinSetExStyle WinSetRegion WinSetStyle WinSetTitle
        WinSetTransColor WinSetTransparent WinShow WinWait WinWaitActive
        WinWaitClose WinWaitNotActive
        CaretGetPos WinActive WinExist WinGetTitle InputBox WinDelay WinDir
        CoordModeCaret CoordModeMenu CoordModeToolTip ControlDelay
        DetectHiddenText DetectHiddenWindows IconFile IconHidden IconNumber
        IconTip ScriptHwnd TitleMatchMode TitleMatchModeSpeed TrayMenu
    """),
    ("screen", "Screen capture and pixel functions", """
        ImageSearch LoadPicture PixelGetColor PixelSearch Screenshot
        ScreenDPI ScreenHeight ScreenWidth
    """),
    ("native", "Native function calls (DllCall)", """
        DllCall NumGet NumPut VarSetStrCapacity
        LastError PtrSize
    """),
    ("registry", "Registry APIs", """
        RegCreateKey RegDelete RegDeleteKey RegRead RegWrite SetRegView
        LoopRegKey LoopRegName LoopRegTimeModified LoopRegType RegView
    """),
    ("com", "COM automation (ComObject)", """
        ComCall ComObjActive ComObjConnect ComObjFlags ComObjFromPtr
        ComObjGet ComObjQuery ComObjType ComObjValue ObjBindMethod
        ObjFromPtr ObjFromPtrAddRef ObjPtr ObjPtrAddRef ObjRelease
    """),
    ("core", "The core of the language: text, math and patterns", """
        Abs ACos ASin ATan Ceil Chr Cos Exp Floor Format GetMethod HasBase
        HasMethod HasProp InStr IsAlnum IsAlpha IsDigit IsFloat IsInteger
        IsLower IsNumber IsObject IsSetRef IsSpace IsTime IsUpper IsXDigit
        Ln Log LTrim Max Mod Ord Random RegExMatch RegExReplace Round
        RTrim Sort StrCompare StrGet StrLen StrLower StrPtr StrPut StrReplace
        StrTitle StrUpper SubStr Tan Trim Type VerCompare StrSplit Sin Sqrt
    """),
    ("objects", "The objects of the language: Array, Map and Func", """
        ObjAddRef ObjGetBase ObjGetCapacity ObjHasOwnProp ObjOwnPropCount
        ObjOwnProps ObjSetBase ObjSetCapacity
    """),
    ("time", "Dates and times", """
        DateAdd DateDiff FormatTime DD DDD DDDD
        Hour MDay MM MMM MMMM Mon MSec Now NowUTC Sec TickCount WDay
        YDay Year YWeek YYYY
    """),
    ("machine", "The machine, the processes and the environment", """
        OutputDebug OSVersion ProcessClose ProcessExist ProcessGetName
        ProcessGetParent ProcessGetPath ProcessSetPriority ProcessWait
        ProcessWaitClose Run RunAs RunWait Shutdown SysGet SysGetIPAddresses
        AhkPath AhkVersion AppData AppDataCommon ComSpec ComputerName
        Desktop DesktopCommon Language MyDocuments ProgramFiles Programs
        ProgramsCommon StartMenu StartMenuCommon Startup StartupCommon
        Temp UserName
    """),
    ("clipboard", "The clipboard", """
        Clipboard OnClipboardChange
    """),
    ("sound", "Sound", """
        SoundBeep SoundPlay SoundGetMute SoundGetVolume SoundGetInterface
        SoundGetName SoundSetMute SoundSetVolume
    """),
]

# The class of a method says the area, and the class of the windows is the
# area of the windows. The class types are names of a kind of their own,
# and they are in the area of the thing they are.
CLASS_AREA = {
    "Any": "core",
    "Array": "objects",
    "BoundFunc": "objects",
    "Buffer": "native",
    "Class": "core",
    "ClipboardAll": "clipboard",
    "Closure": "objects",
    "ComObject": "com",
    "ComValue": "com",
    "Enumerator": "objects",
    "File": "files",
    "Func": "objects",
    "Gui": "gui",
    "Hotkey": "input",
    "InputHook": "input",
    "Map": "objects",
    "Menu": "gui",
    "Object": "objects",
    "RegExMatchInfo": "objects",
}

# The classes of the failures are the run of the script: they are what it
# refuses with.
ERROR_CLASSES = [
    "Error", "MemoryError", "OSError", "TargetError", "TimeoutError",
    "TypeError", "UnsetError", "MemberError", "PropertyError", "MethodError",
    "UnsetItemError", "ValueError", "IndexError", "ZeroDivisionError",
]

# A name that is in two kinds: the function `Min` is the math and the
# variable `Min` is the minute of the clock.
BY_KIND = {("function", "Min"): "core", ("variable", "Min"): "time"}

# The things that the numbers do not say, measured on the binary of the
# v2.0.19 on 2026-10-29.
NOTES = """
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
""".strip()


def read_surface(runner):
    """The state of every name of the surface, as the tree answers it."""
    result = subprocess.run([runner, "--surface"], capture_output=True, text=True,
                            timeout=120, encoding="utf-8", errors="replace")
    if result.returncode != 0:
        raise SystemExit("the runner failed: %s" % result.stderr.strip()[:400])
    rows = []
    for line in result.stdout.split("\n"):
        if not line.strip() or line.startswith("the surface of v2 has"):
            continue
        parts = line.split(" ")
        rows.append((parts[1], parts[0], " ".join(parts[2:])))
    return rows


def where_of(rows):
    """The area of every name of the surface."""
    names = {}
    for key, _, blob in AREAS:
        for name in blob.split():
            names[name] = key
    where = {}
    for kind, name, _ in rows:
        if kind == "method":
            area = CLASS_AREA.get(name.split(".")[0])
        elif kind == "class":
            area = "script" if name in ERROR_CLASSES else CLASS_AREA.get(name)
        else:
            area = BY_KIND.get((kind, name), names.get(name))
        if area is None:
            raise SystemExit("the name %s (%s) is in no area" % (name, kind))
        where[(kind, name)] = area
    return where


def the_list(names):
    """The names as a list that wraps, in the order of the alphabet."""
    return ", ".join("`%s`" % name for name in sorted(names))


def write_report(rows, where, out):
    buckets = {}
    for kind, name, state in rows:
        buckets.setdefault(where[(kind, name)], []).append((kind, name, state))

    values = sum(1 for _, _, state in rows if state == "value")
    refusals = len(rows) - values

    out.append("# The capability report of the surface of AutoHotkey v2.0.19\n")
    out.append("> **Generated.** The generator is "
               "`crates/ahk/reference/ahk-v2.0.19.areas.py` , and the state of "
               "every name is the one of the tree itself: `ahk --surface` asks "
               "the table of the built-ins, the table of the built-in "
               "variables and the classes, and answers with a value, with the "
               "reason of a refusal, or with the reason of a name to do.\n")
    distinct = len(set(name for _, name, _ in rows))
    out.append("The surface of the v2.0.19 has **%d entries** : **%d answer "
               "with a value** , **%d are refused** and **none is pending**.\n"
               % (len(rows), values, refusals))
    out.append("The table has **%d entries and %d distinct names** : a property "
               "with a setter is in the table twice --- the `Hotkey` class has "
               "eleven of them and the `InputHook` has twelve --- and a name "
               "that is both a command and a "
               "variable is two entries (`SendMode` , `SendLevel` , `Min` , "
               "`FileEncoding` , `ListLines` , `DetectHiddenText` , "
               "`DetectHiddenWindows` ). The numbers below are entries.\n"
               % (len(rows), distinct))

    out.append("## The areas\n")
    out.append("| Area | Names | Answer | Refused |")
    out.append("|---|---|---|---|")
    for key, title, _ in AREAS:
        items = buckets.get(key, [])
        answered = sum(1 for _, _, state in items if state == "value")
        out.append("| %s | %d | %d | %d |"
                   % (title, len(items), answered, len(items) - answered))
    out.append("| **The whole of it** | **%d** | **%d** | **%d** |"
               % (len(rows), values, refusals))
    out.append("")

    out.append(NOTES)
    out.append("")

    for number, (key, title, _) in enumerate(AREAS, start=1):
        items = buckets.get(key, [])
        answered = [name for _, name, state in items if state == "value"]
        refused = [(name, state) for _, name, state in items if state != "value"]
        out.append("## %d. %s\n" % (number, title))
        out.append("**%d names: %d answer with a value, %d %s refused.**\n"
                   % (len(items), len(answered), len(refused),
                      "is" if len(refused) == 1 else "are"))
        if answered:
            out.append("**The ones that answer** (%d): %s\n"
                       % (len(answered), the_list(answered)))
        if refused:
            reasons = {}
            for name, state in refused:
                reason = state.split(": ", 1)[1] if ": " in state else state
                reasons.setdefault(reason, []).append(name)
            out.append("**The ones that are refused** (%d), by the reason:\n"
                       % len(refused))
            for reason in sorted(reasons, key=lambda key: -len(reasons[key])):
                names = reasons[reason]
                out.append("- *%s* (%d): %s" % (reason, len(names), the_list(names)))
            out.append("")


def main():
    if len(sys.argv) < 2:
        raise SystemExit(__doc__)
    runner = sys.argv[1]
    rows = read_surface(runner)
    where = where_of(rows)
    out = []
    write_report(rows, where, out)
    text = "\n".join(out) + "\n"
    if len(sys.argv) > 2:
        pathlib.Path(sys.argv[2]).write_text(text, encoding="utf-8", newline="\n")
    else:
        print(text)


if __name__ == "__main__":
    main()
