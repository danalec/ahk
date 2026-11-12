"""The behavior of the names: what the tree does with them.

    from surface_behavior import behavior_of

The pages of `docs/surface/` carry the **ficha** of a name: its kind, its
class, its parameters and the state of the tree. The ficha says that
`FileAppend` exists; the behavior says what it does, what it answers,
what it refuses and how a script writes it.

The behavior lives in `scripts/behaviour/`, one file per chunk of names,
and every entry is written in the languages of the subtree:

    BEHAVIOR = {
        "FileAppend": {
            "en": {
                "summary": "Writes text at the end of a file.",
                "params": "Text, FileName, [Mode], [Encoding]",
                "returns": "nothing",
                "errors": "OSError when the file cannot be opened.",
                "example": 'FileAppend "hello`n", "out.txt"',
            },
            "zh": {...},
            "pt-BR": {...},
        },
    }

The key of the entry is the label of the name -- the name itself, or
`Class.Method` for a method. A language that is missing falls back to
English, so a page is never half empty because a translation was not
written yet.
"""

import glob
import os

HERE = os.path.dirname(os.path.abspath(__file__))
BEHAVIOUR_DIR = os.path.join(HERE, "behaviour")

# The names that carry a behavior, in the order of the pages. The list is
# the names a script uses most; the tree carries the rest of the surface
# without a description.
#
# The chunks are the files of `scripts/behaviour/`: one chunk, one file,
# one writer. The name of a chunk file is `chunk1.py`, `chunk2.py` and so
# on.
CHUNKS = {
    1: """
        MsgBox Sleep Exit ExitApp Run RunWait Reload Suspend Pause
        Persistent OnExit OnError Throw Try Catch Finally
        If Else While Loop For Break Continue Return Switch Goto
        SetTimer SetWorkingDir Critical
        ToolTip TrayTip InputBox
    """,
    2: """
        StrLen SubStr StrReplace StrSplit StrLower StrUpper Trim LTrim
        RTrim InStr Format Chr Ord StrGet StrPut RegExMatch RegExReplace
        Sort FormatTime VerCompare Type IsNumber IsObject
        Abs Ceil Floor Round Max Min Mod Random Sin Cos Tan Sqrt Exp Ln
    """,
    3: """
        FileAppend FileRead FileOpen FileDelete FileExist FileGetSize
        FileGetTime FileSetTime FileSetAttrib FileCopy FileMove FileCreateShortcut
        DirCreate DirExist DirDelete DirCopy DirMove
        IniRead IniWrite IniDelete Download FileRecycle FileInstall
        SplitPath
    """,
    4: """
        Send SendText SendEvent SendInput SendPlay SendMode SendLevel
        Click MouseClick MouseMove MouseGetPos KeyWait GetKeyState
        GetKeyName GetKeyVK GetKeySC BlockInput CoordMode InstallKeybdHook
        InstallMouseHook Hotkey Hotstring
        PixelGetColor PixelSearch ImageSearch Screenshot
        WinExist WinActive WinGetTitle WinGetClass WinGetList WinGetCount
        WinGetPos WinGetClientPos WinActivate WinClose WinHide WinShow
    """,
    5: """
        WinMinimize WinMaximize WinRestore WinMove WinWait WinWaitActive
        WinWaitClose WinGetPID WinGetProcessName WinSetTitle
        ProcessExist ProcessGetName ProcessWait ProcessWaitClose ProcessClose
        ProcessGetPath SysGet EnvGet EnvSet
        RegRead RegWrite RegDelete
        DllCall NumGet NumPut
        Clipboard SoundBeep SoundPlay
        DateAdd DateDiff
    """,
    6: """
        A_Now A_NowUTC A_TickCount A_ScriptDir A_ScriptFullPath A_ScriptName
        A_Index A_LoopField A_LoopFileName A_LoopFileExt A_LoopFileDir
        A_LoopReadLine A_LoopRegName A_ErrorLevel A_ThisFunc A_ThisHotkey
        A_Hour A_Min A_Sec A_MSec A_YDay A_YYYY A_MM A_Mon A_WDay A_DD
        A_AppData A_Desktop A_Temp A_UserName A_ComputerName A_OSVersion
        A_IsAdmin A_Is64bitOS A_Clipboard A_KeyDelay A_SendMode A_Cursor
        A_ScreenWidth A_ScreenHeight A_LoopFileSize
    """,
    7: """
        Array Map File Object Func Enumerator
        Array.Push Array.Pop Array.Length Map.Set Map.Get Map.Has Map.Delete
        File.Read File.ReadLine File.Write File.WriteLine File.Close
        File.AtEOF File.Pos File.Length
        ObjGetBase HasBase HasMethod HasProp GetMethod IsSet
    """,
}

# The languages the behavior is written in. A language that is missing
# falls back to English.
LANGUAGES = ("en", "zh", "pt-BR")

_CACHE = None


def _load():
    """The behavior of every chunk file, merged."""
    global _CACHE
    if _CACHE is not None:
        return _CACHE
    merged = {}
    if os.path.isdir(BEHAVIOUR_DIR):
        for path in sorted(glob.glob(os.path.join(BEHAVIOUR_DIR, "*.py"))):
            namespace = {}
            with open(path, "r", encoding="utf-8") as handle:
                source = handle.read()
            exec(compile(source, path, "exec"), namespace)
            for name, entry in (namespace.get("BEHAVIOR") or {}).items():
                merged.setdefault(name, {}).update(entry)
    _CACHE = merged
    return merged


def the_names():
    """The names that carry a behavior, in the order of the chunks."""
    names = []
    for key in sorted(CHUNKS):
        names.extend(CHUNKS[key].split())
    return names


def behavior_of(label, lang):
    """The behavior of a name, in a language; None when it has none.

    The fallback is English, and it is per field: a name described in one
    language and not in another keeps its description, and a language
    that carries only the summary keeps the summary.
    """
    entry = _load().get(label)
    if not entry:
        return None
    english = entry.get("en") or {}
    other = entry.get(lang) or {}
    if not english and not other:
        return None
    text = {}
    for field in ("summary", "params", "returns", "errors", "example"):
        value = other.get(field) or english.get(field)
        if value:
            text[field] = value
    return text or None
