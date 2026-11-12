"""The families of the surface: which area a name of a dialect belongs to.

    from surface_areas import families_of, AREA_KEYS

The tree of the AutoHotkey dialects has three surfaces -- the v1, the v2
and the extensions of KeySharp -- and the documents of `docs/surface/`
are grouped by the **area** a name belongs to, not by the kind of it. An
area is the thing a script thinks in: the files, the input, the windows,
the screen, the native calls, the registry, the COM objects, the core of
the language.

The areas themselves are not invented here. The v2 ones are read out of
`crates/ahk/reference/ahk-v2.0.19.areas.py`, which is the file that
writes `docs/v2-capability-report.md`: one source, so the report and the
surface documents cannot disagree about the name of an area.

What this module adds is:

* the names of the v1 and of the KeySharp that no v2 area names, placed by
  hand in `EXTRA_V1` and `EXTRA_KS`;
* the rules `RULES`, which place a name that no table places -- the rules
  are a fallback and are printed by the generator when they fire, so a
  name that lands in the wrong area is visible instead of silent.

The families are the areas. A family page of `docs/surface/families/` is
one area, and it carries the names of the three dialects in it.
"""

import ast
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
AREAS_PY = ROOT / "crates" / "ahk" / "reference" / "ahk-v2.0.19.areas.py"


def _literal_assignment(path, name):
    """The value of a module-level assignment, read without running the module."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    for node in tree.body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name) and target.id == name:
                    return ast.literal_eval(node.value)
    raise KeyError("%s has no assignment %s" % (path, name))


# The areas of the v2, in the order the capability report prints them.
#   (key, title, the names of the area)
_AREAS = _literal_assignment(AREAS_PY, "AREAS")

# The area of the class of a method: the method of the `Gui` is of the
# area of the windows, and the method of the `File` is of the area of the
# files.
CLASS_AREA = _literal_assignment(AREAS_PY, "CLASS_AREA")

# The classes that the capability report does not place, because it counts
# the names and not the classes of the methods. The nested classes of the
# `Gui` are of the area of the windows, and the classes of the failures
# are of the core of the language: they are the values a `catch` holds.
CLASS_AREA.update({
    "Gui.Control": "gui",
    "Gui.ListView": "gui",
    "Gui.Menu": "gui",
    "Gui.StatusBar": "gui",
    "Gui.TreeView": "gui",
    "Error": "core",
    "MemoryError": "core",
    "OSError": "core",
    "TargetError": "core",
    "TimeoutError": "core",
    "IndexError": "core",
    "MemberError": "core",
    "MethodError": "core",
    "PropertyError": "core",
    "UnsetError": "core",
    "UnsetItemError": "core",
    "ValueError": "core",
    "ZeroDivisionError": "core",
    "ScriptProcess": "machine",
    "String": "core",
})

# The names of the v2 that no area places, because the name is of a kind
# that the area of it does not follow: the `Min` is the math in a call and
# the minute of the clock in a read.
EXTRA_V2 = {
    "time": """
        Min
    """,
}

# The key of an area, and the title of the page of it.
AREA_KEYS = [key for key, _, _ in _AREAS]
AREA_TITLES = {key: title for key, title, _ in _AREAS}
AREA_ORDER = {key: index for index, key in enumerate(AREA_KEYS)}

# The names of the v2 of each area, as a set.
V2_AREA_NAMES = {}
for _key, _title, _names in _AREAS:
    V2_AREA_NAMES[_key] = set(_names.split())

# The area of a name of the v2: the name is in a list, or the class of it
# is a class of the tree.
V2_NAME_AREA = {}
for _key, _title, _names in _AREAS:
    for _name in _names.split():
        V2_NAME_AREA.setdefault(_name, _key)
for _class, _area in CLASS_AREA.items():
    V2_NAME_AREA.setdefault(_class, _area)
for _area, _names in EXTRA_V2.items():
    for _name in _names.split():
        V2_NAME_AREA.setdefault(_name, _area)


# The names of the v1 that the v2 does not have, placed by hand. A name
# of the v1 that the v2 has takes the area of the v2 name: the two are
# the same thing in the two dialects.
EXTRA_V1 = {
    "script": """
        ErrorLevel IsFunc IsSet SetBatchLines Suspend Critical
        #CommentFlag #ErrorStdOut #EscapeChar #HotkeyInterval
        #HotkeyModifierTimeout #Hotstring #If #IfTimeout #IfWinActive
        #Include #InputLevel #MaxHotkeysPerInterval #MaxMem #MaxThreads
        #MaxThreadsBuffer #MaxThreadsPerHotkey #MenuMaskKey #NoEnv
        #NoTrayIcon #Persistent #Requires #SingleInstance #UseHook #Warn
        #WinActivateForce
        A_AhkPath A_AhkVersion A_AutoTrim A_BatchLines A_DefaultGui
        A_DefaultListView A_DefaultTreeView A_EndChar A_ExitReason
        A_Index A_InitialWorkingDir A_Is64bitOS A_IsAdmin A_IsCompiled
        A_IsCritical A_IsPaused A_IsSuspended A_IsUnicode A_LastError
        A_LineFile A_LineNumber A_ListLines A_NumBatchLines A_OSType
        A_OSVersion A_PtrSize A_ScriptDir A_ScriptFullPath A_ScriptHwnd
        A_ScriptName A_Space A_Tab A_ThisFunc A_ThisLabel A_ThisMenu
        A_ThisMenuItem A_ThisMenuItemPos A_WorkingDir A_ErrorLevel
        Else IfInString IfMsgBox IfNotInString
    """,
    "files": """
        FileCopyDir FileCreateDir FileMoveDir FileReadLine FileRemoveDir
        FileSelectFile FileSelectFolder StringGetPos
        A_DD A_DDD A_DDDD A_LoopField A_LoopFileName A_LoopReadLine
    """,
    "input": """
        ControlGet MouseGetPos SendInput SendEvent SendPlay SendText
        A_CaretX A_CaretY A_CoordModeCaret A_CoordModeMenu A_CoordModeMouse
        A_CoordModePixel A_CoordModeToolTip A_Cursor A_KeyDelay A_KeyDelayPlay
        A_KeyDuration A_KeyDurationPlay A_MenuMaskKey A_MouseDelay
        A_MouseDelayPlay A_PriorHotkey A_PriorKey A_SendLevel A_SendMode
        A_ThisHotkey A_TimeIdle A_TimeIdleKeyboard A_TimeIdleMouse
        A_TimeIdlePhysical A_TimeSincePriorHotkey A_TimeSinceThisHotkey
        A_StoreCapsLockMode
    """,
    "gui": """
        Control ControlGet Gui GuiControl GuiControlGet MenuGetHandle
        MenuGetName Progress RegisterCallback SplashTextOn WinGet
        WinGetActiveStats WinGetActiveTitle WinMenuSelectItem WinSet
        A_ControlDelay A_DetectHiddenText A_DetectHiddenWindows A_Gui
        A_GuiControl A_GuiControlEvent A_GuiEvent A_GuiHeight A_GuiWidth
        A_GuiX A_GuiY A_IconFile A_IconHidden A_IconNumber A_IconTip
        A_TitleMatchMode A_TitleMatchModeSpeed A_WinDelay A_WinDir
    """,
    "core": """
        Asc AutoTrim StringCaseSense StringGetPos StringLeft StringLen
        StringLower StringMid StringReplace StringSplit StringTrimLeft
        StringUpper Transform VarSetCapacity SetFormat
        A_FormatFloat A_FormatInteger A_StringCaseSense
    """,
    "machine": """
        Drive DriveGet DriveSpaceFree EnvAdd EnvDiv EnvMult EnvSub EnvUpdate
        A_AppData A_AppDataCommon A_ComSpec A_ComputerName A_Desktop
        A_DesktopCommon A_Language A_MyDocuments A_ProgramFiles A_Programs
        A_ProgramsCommon A_StartMenu A_StartMenuCommon A_Startup
        A_StartupCommon A_Temp A_UserName
    """,
    "time": """
        A_Day A_Hour A_Min A_Mon A_MSec A_Now A_NowUTC A_Sec A_WDay A_YDay
        A_YYYY A_Year
    """,
    "registry": """
        A_LoopRegName A_RegView
    """,
    "clipboard": """
        #ClipboardTimeout A_Clipboard
    """,
    "sound": """
        SoundGet SoundGetWaveVolume SoundSet SoundSetWaveVolume
    """,
    "com": """
        ComObjArray ComObjCreate ComObjError
    """,
    "screen": """
        A_ScreenDPI A_ScreenHeight A_ScreenWidth
    """,
}


# The names of the KeySharp that the v2 does not have, placed by hand.
EXTRA_KS = {
    "script": """
        CompileScript ValidateScript RunScript ShowDebug Collect
        IsComponentAvailable RequestCapabilities OutputDebugLine
        A_KsCorePath A_KsVersion A_Persistent A_RealThread A_Timers
        A_MaxThreads A_MaxThreadsBuffer A_MaxThreadsPerHotkey A_MouseHookInstalled
        A_KeybdHookInstalled
    """,
    "core": """
        ATan2 Cosh Sinh Tanh RandomSeed ReplaceLineEndings
        Float32 Float64 Int8 Int16 Int32 Int64 UInt8 UInt16 UInt32 IntPtr
        ArgumentError UnsupportedError Throw True False
        A_NewLine A_OSType A_OSArch A_ProcessArch A_LoopRegValue
    """,
    "objects": """
        Struct PropRef Props VarRef ToString Contains Filter FindIndex
        IndexOf Join Remove BeforeHotkeys IsClosure IsMethod Params
        A_Args
    """,
    "input": """
        ClipCursor GetKeyboardLayout GetKeyInfo
        EndCharMode OnMouseDown OnMouseMove OnMouseUp TranscribeModifiedKeys
        VisibleMouseMove
        A_ClipboardTimeout A_DefaultHotstringCaseSensitive
        A_DefaultHotstringConformToCase A_DefaultHotstringDetectWhenInsideWord
        A_DefaultHotstringDoBackspace A_DefaultHotstringDoReset
        A_DefaultHotstringEndCharRequired A_DefaultHotstringEndChars
        A_DefaultHotstringKeyDelay A_DefaultHotstringNoMouse
        A_DefaultHotstringOmitEndChar A_DefaultHotstringPriority
        A_DefaultHotstringSendMode A_DefaultHotstringSendRaw
        A_HotIf A_HotIfTimeout A_PeekFrequency
    """,
    "files": """
        FileCreateTemp FileFullPath Flush
        A_DirSeparator
    """,
    "machine": """
        EnvUpdate ScriptProcess
        At AllowTimers Elapsed ExitCode ExitTime HasExited Id IsActive
        IsInterruptible Kind Paused Priority Underlying Empty
        Args Target StdErr StdIn StdOut
    """,
    "gui": """
        WinFromPoint WinGetAlwaysOnTop WinGetEnabled
        A_GuiTheme
    """,
    "native": """
        ObjCreateWithDataPtr ObjGetDataPtr ObjGetDataSize ObjSetDataPtr
    """,
    "network": """
        Mail
    """,
}


# The rules that place a name that no table places. The first rule that
# matches wins. The generator prints every name that a rule places, so a
# name that a rule puts in the wrong area is visible.
RULES = [
    (r"^#", "script"),
    (r"^(Env|SetEnv|OSVersion|SysGet)", "machine"),
    (r"^(ComObj|Obj|Var|Dll|Num|MenuGet|ClipboardAll)", "core"),
    (r"^(File|Dir|Ini|LoopFile)", "files"),
    (r"^(Win|Gui|Control|Menu|Progress|StatusBar|ToolTip|ListView)", "gui"),
    (r"^(Send|Mouse|Key|Clip|Click|BlockInput|Hotkey|Hotstring|Install|Input)", "input"),
    (r"^(Pixel|Image|Screen)", "screen"),
    (r"^(Reg)", "registry"),
    (r"^(Date|Now|Tick|Time|Year|Month)", "time"),
    (r"^(Sound)", "sound"),
    (r"^(Process|Run|Shutdown|Download|URLDownload)", "machine"),
    (r"^(Str|Sub|InStr|Ln|Ord|Chr|Trim|Format|RegEx|Type|IsNumber|Is[A-Z])", "core"),
    (r"^A_", "machine"),
]


def _extras(table):
    """The extra names of a dialect, as {area: set(names)}."""
    return {
        area: set(names.split())
        for area, names in table.items()
    }


EXTRA_V1_BY_AREA = _extras(EXTRA_V1)
EXTRA_KS_BY_AREA = _extras(EXTRA_KS)


def _rule_area(name):
    """The area of a name that no table places, or None."""
    for pattern, area in RULES:
        if re.match(pattern, name):
            return area
    return None


def area_of(name, dialect, klass=""):
    """The area of a name of a dialect.

    The name is looked for in the tables of the dialect, then in the
    rules. `None` means that no table and no rule placed it, and the
    generator says so instead of guessing.
    """
    if dialect == "v2":
        if name in V2_NAME_AREA:
            return V2_NAME_AREA[name]
        if klass and klass in CLASS_AREA:
            return CLASS_AREA[klass]
    elif dialect == "v1":
        for area, names in EXTRA_V1_BY_AREA.items():
            if name in names:
                return area
        if name in V2_NAME_AREA:
            return V2_NAME_AREA[name]
        if klass and klass in CLASS_AREA:
            return CLASS_AREA[klass]
    elif dialect == "ks":
        for area, names in EXTRA_KS_BY_AREA.items():
            if name in names:
                return area
        if name in V2_NAME_AREA:
            return V2_NAME_AREA[name]
        if klass and klass in CLASS_AREA:
            return CLASS_AREA[klass]
    return _rule_area(name)


def families_of():
    """The families, in the order of the pages: (key, title)."""
    return [(key, AREA_TITLES[key]) for key in AREA_KEYS]
