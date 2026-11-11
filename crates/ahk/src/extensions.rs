//! The names that the tree carries beside the dialect of v2.0.19.
//!
//! KeySharp speaks a dialect of its own: the AutoHotkey v2.1-alpha, and
//! beside it a set of names that are its own -- the members of the classes
//! of AutoHotkey that it adds, the functions of its `Ks` module and the
//! accessors of it. The tree speaks that dialect behind the `--dialect=ks`,
//! and this module is the table of the names of it.
//!
//! The dialect of v2.0.19 stays pure. A script that does not ask for the
//! `ks` sees the surface of the v2.0.19 and nothing else: a name that is
//! here is not in the surface of the v2.0.19, and the tree does not put it
//! there. The plan of the audit says the same
//! (`egm-review/keysharp-audit/plan-to-surpass-features.md`, guardrail 2):
//! the contract of the v2.0.19 stays pure, and a new name goes in the
//! extension namespace.
//!
//! The members are measured on the documentation of KeySharp
//! (`Keysharp/docs/reference.md`), which says what each one answers and
//! what it does with the index it starts at.

use alloc::vec::Vec;

/// The members of the classes of AutoHotkey that the `ks` dialect adds.
///
/// The pair is the class and the member, both in the spelling of the lower
/// case, the way the table of the classes of the tree carries them.
pub const MEMBERS: &[(&str, &str)] = &[
    ("array", "contains"),
    ("array", "filter"),
    ("array", "findindex"),
    ("array", "indexof"),
    ("array", "join"),
    ("array", "map"),
    ("array", "remove"),
    ("array", "sort"),
    ("array", "tostring"),
    ("map", "tostring"),
    ("func", "isclosure"),
    ("func", "ismethod"),
    ("func", "params"),
    ("file", "flush"),
    ("string", "startswith"),
    ("string", "endswith"),
    ("inputhook", "beforehotkeys"),
    ("inputhook", "bufferlengthmax"),
    ("inputhook", "endcharmode"),
    ("inputhook", "onmousedown"),
    ("inputhook", "onmousemove"),
    ("inputhook", "onmouseup"),
    ("inputhook", "transcribemodifiedkeys"),
    ("inputhook", "visiblemousemove"),
    ("thread", "id"),
    ("thread", "index"),
    ("thread", "isactive"),
    ("thread", "kind"),
    ("thread", "elapsed"),
    ("thread", "priority"),
    ("thread", "allowtimers"),
    ("thread", "critical"),
    ("thread", "paused"),
    ("thread", "isinterruptible"),
    ("thread", "underlying"),
    ("thread", "exitcode"),
    ("thread", "exit"),
];

/// The members the `ks` dialect adds, with the class, the name in the
/// spelling of the dialect and the number of the parameters: the smallest
/// and the largest.
///
/// The numbers are the ones of the documentation of KeySharp: every member
/// of this block takes none, and the members of the `Array` are the ones
/// the tables above already carry.
const THE_METHODS: &[(&str, &str, &str, i64, i64)] = &[
    ("array", "contains", "Contains", 1, 1),
    ("array", "filter", "Filter", 1, 2),
    ("array", "findindex", "FindIndex", 1, 2),
    ("array", "indexof", "IndexOf", 1, 2),
    ("array", "join", "Join", 0, 1),
    ("array", "map", "Map", 1, 2),
    ("array", "remove", "Remove", 0, 1),
    ("array", "sort", "Sort", 1, 1),
    ("array", "tostring", "ToString", 0, 0),
    ("map", "tostring", "ToString", 0, 0),
    ("func", "isclosure", "IsClosure", 0, 0),
    ("func", "ismethod", "IsMethod", 0, 0),
    ("func", "params", "Params", 0, 0),
    ("file", "flush", "Flush", 0, 0),
    ("thread", "id", "Id", 0, 0),
    ("thread", "index", "Index", 0, 0),
    ("thread", "isactive", "IsActive", 0, 0),
    ("thread", "kind", "Kind", 0, 0),
    ("thread", "elapsed", "Elapsed", 0, 0),
    ("thread", "priority", "Priority", 0, 0),
    ("thread", "allowtimers", "AllowTimers", 0, 0),
    ("thread", "critical", "Critical", 0, 0),
    ("thread", "paused", "Paused", 0, 0),
    ("thread", "isinterruptible", "IsInterruptible", 0, 0),
    ("thread", "underlying", "Underlying", 0, 0),
    ("thread", "exitcode", "ExitCode", 0, 0),
    ("thread", "exit", "Exit", 0, 1),
];

/// Whether the `ks` dialect adds the member to the class.
///
/// The class and the member are the ones the script wrote, and the answer
/// does not care about the case.
pub fn carries_the_member(class: &str, member: &str) -> bool {
    let class = class.to_lowercase();
    let member = member.to_lowercase();
    MEMBERS
        .iter()
        .any(|(owner, name)| *owner == class && *name == member)
}

/// The member of the `ks` extensions that a class answers, with the name in
/// the spelling of the dialect and the numbers of the parameters.
///
/// The class and the member are the ones the script wrote, and the answer
/// does not care about the case. `None` is the answer for a member that the
/// extensions do not add, and for a class they do not touch.
pub fn the_method_of(class: &str, member: &str) -> Option<(&'static str, i64, i64)> {
    let class = class.to_lowercase();
    let member = member.to_lowercase();
    THE_METHODS
        .iter()
        .find(|(owner, name, _, _, _)| *owner == class && *name == member)
        .map(|(_, _, spelled, min, max)| (*spelled, *min, *max))
}

/// The accessors of the `Ks` module that the tree answers, in the lower
/// case without the `A_` prefix, the way the table of the built-in
/// variables of the tree carries a name.
///
/// The four of them are the ones the tree can answer honestly on every
/// machine: the two clocks read the clock of the machine, and the two
/// spellings are the ones the platform writes. The rest of the accessors
/// of KeySharp ask for a state the tree does not keep yet, and they are
/// not in the table until they answer.
pub const VARIABLES: &[&str] = &[
    "nowms",
    "nowutcms",
    "dirseparator",
    "newline",
    // The defaults of the hotstrings, the ones the KeySharp manager starts
    // with and the ones the documentation of AHK names.
    "defaulthotstringcasesensitive",
    "defaulthotstringconformtocase",
    "defaulthotstringdetectwheninsideword",
    "defaulthotstringdobackspace",
    "defaulthotstringdoreset",
    "defaulthotstringendcharrequired",
    "defaulthotstringendchars",
    "defaulthotstringkeydelay",
    "defaulthotstringnomouse",
    "defaulthotstringomitendchar",
    "defaulthotstringpriority",
    "defaulthotstringsendmode",
    "defaulthotstringsendraw",
    // The settings of the hotkeys and the threads, read out of the
    // defaults KeySharp starts with.
    "clipboardtimeout",
    "hotiftimeout",
    "maxthreads",
    "maxthreadsbuffer",
    "maxthreadsperhotkey",
    "peekfrequency",
    "persistent",
    "winactivateforce",
    "notrayicon",
    // The machine and the build.
    "osarch",
    "processarch",
    "ostype",
    "ksversion",
    "kscorepath",
    "guitheme",
    // The two booleans, the thread of the script, the two hooks and the
    // value of the last `Loop Reg`.
    "true",
    "false",
    "thread",
    "keybdhookinstalled",
    "mousehookinstalled",
    "loopregvalue",
];

/// Whether the `ks` dialect carries the variable of this name.
///
/// The name is the one the script wrote, with the `A_` prefix or without
/// it: the two spellings are the same variable, the way the built-in
/// variables of the v2 are.
pub fn carries_the_variable(name: &str) -> bool {
    let lowered = name.to_lowercase();
    let bare = lowered.strip_prefix('a').and_then(|rest| {
        // The prefix is `A_`, and only that one: a name that begins with
        // an `a` is a name of its own.
        rest.strip_prefix('_')
    });
    let bare = bare.unwrap_or(lowered.as_str());
    VARIABLES.contains(&bare)
}

/// The rows of the surface that the `ks` dialect adds to the surface of the
/// v2.0.19.
///
/// The shape of a row is the one of the table of the v2: the kind, the
/// class, the name, and the columns the kinds after the method do not use.
/// The variables carry the `A_` prefix, because that is the spelling the
/// scripts write and the one the audit reads; the state of every row is
/// answered at the run, the way the state of a name of the v2 is.
pub const SURFACE_ROWS: &str = concat!(
    "method\tArray\tContains\t\t\t\n",
    "method\tArray\tFilter\t\t\t\n",
    "method\tArray\tFindIndex\t\t\t\n",
    "method\tArray\tIndexOf\t\t\t\n",
    "method\tArray\tJoin\t\t\t\n",
    "method\tArray\tMap\t\t\t\n",
    "method\tArray\tRemove\t\t\t\n",
    "method\tArray\tSort\t\t\t\n",
    "method\tArray\tToString\t\t\t\n",
    "method\tMap\tToString\t\t\t\n",
    "method\tFunc\tIsClosure\t\t\t\n",
    "method\tFunc\tIsMethod\t\t\t\n",
    "method\tFunc\tParams\t\t\t\n",
    "method\tFile\tFlush\t\t\t\n",
    "variable\t\tA_DirSeparator\t\t\t\n",
    "variable\t\tA_NewLine\t\t\t\n",
    "variable\t\tA_NowMs\t\t\t\n",
    "variable\t\tA_NowUTCMs\t\t\t\n",
    "class\t\tThread\t\t\t\n",
    "method\tThread\tId\t\t\t\n",
    "method\tThread\tIndex\t\t\t\n",
    "method\tThread\tIsActive\t\t\t\n",
    "method\tThread\tKind\t\t\t\n",
    "method\tThread\tElapsed\t\t\t\n",
    "method\tThread\tPriority\t\t\t\n",
    "method\tThread\tAllowTimers\t\t\t\n",
    "method\tThread\tCritical\t\t\t\n",
    "method\tThread\tPaused\t\t\t\n",
    "method\tThread\tIsInterruptible\t\t\t\n",
    "method\tThread\tUnderlying\t\t\t\n",
    "method\tThread\tExitCode\t\t\t\n",
    "method\tThread\tExit\t\t\t\n",
    "method\tInputHook\tBeforeHotkeys\t\t\t\n",
    "method\tString\tStartsWith\t\t\t\n",
    "method\tString\tEndsWith\t\t\t\n",
    "method\tInputHook\tBufferLengthMax\t\t\t\n",
    "method\tInputHook\tEndCharMode\t\t\t\n",
    "method\tInputHook\tOnMouseDown\t\t\t\n",
    "method\tInputHook\tOnMouseMove\t\t\t\n",
    "method\tInputHook\tOnMouseUp\t\t\t\n",
    "method\tInputHook\tTranscribeModifiedKeys\t\t\t\n",
    "method\tInputHook\tVisibleMouseMove\t\t\t\n",
    "variable\t\tA_ClipboardTimeout\t\t\t\n",
    "variable\t\tA_DefaultHotstringCaseSensitive\t\t\t\n",
    "variable\t\tA_DefaultHotstringConformToCase\t\t\t\n",
    "variable\t\tA_DefaultHotstringDetectWhenInsideWord\t\t\t\n",
    "variable\t\tA_DefaultHotstringDoBackspace\t\t\t\n",
    "variable\t\tA_DefaultHotstringDoReset\t\t\t\n",
    "variable\t\tA_DefaultHotstringEndCharRequired\t\t\t\n",
    "variable\t\tA_DefaultHotstringEndChars\t\t\t\n",
    "variable\t\tA_DefaultHotstringKeyDelay\t\t\t\n",
    "variable\t\tA_DefaultHotstringNoMouse\t\t\t\n",
    "variable\t\tA_DefaultHotstringOmitEndChar\t\t\t\n",
    "variable\t\tA_DefaultHotstringPriority\t\t\t\n",
    "variable\t\tA_DefaultHotstringSendMode\t\t\t\n",
    "variable\t\tA_DefaultHotstringSendRaw\t\t\t\n",
    "variable\t\tA_GuiTheme\t\t\t\n",
    "variable\t\tA_HotIfTimeout\t\t\t\n",
    "variable\t\tA_KsCorePath\t\t\t\n",
    "variable\t\tA_KsVersion\t\t\t\n",
    "variable\t\tA_MaxThreads\t\t\t\n",
    "variable\t\tA_MaxThreadsBuffer\t\t\t\n",
    "variable\t\tA_MaxThreadsPerHotkey\t\t\t\n",
    "variable\t\tA_NoTrayIcon\t\t\t\n",
    "variable\t\tA_OSArch\t\t\t\n",
    "variable\t\tA_OSType\t\t\t\n",
    "variable\t\tA_PeekFrequency\t\t\t\n",
    "variable\t\tA_Persistent\t\t\t\n",
    "variable\t\tA_ProcessArch\t\t\t\n",
    "variable\t\tA_WinActivateForce\t\t\t\n",
    "variable\t\tA_KeybdHookInstalled\t\t\t\n",
    "variable\t\tA_LoopRegValue\t\t\t\n",
    "variable\t\tA_MouseHookInstalled\t\t\t\n",
    "variable\t\tA_Thread\t\t\t\n",
    "variable\t\tFalse\t\t\t\n",
    "variable\t\tTrue\t\t\t\n",
    "function\t\tProps\t\t\t\n",
    "function\t\tCompileScript\t\t\t\n",
    "function\t\tThrow\t\t\t\n",
    "function\t\tIsComponentAvailable\t\t\t\n",
    "function\t\tValidateScript\t\t\t\n",
    "function\t\tRequestCapabilities\t\t\t\n",
    "class\t\tVarRef\t\t\t\n",
    "class\t\tPropRef\t\t\t\n",
    "class\t\tArgumentError\t\t\t\n",
    "class\t\tUnsupportedError\t\t\t\n",
    "method\tPropRef\tTarget\t\t\t\n",
    "method\tPropRef\tArgs\t\t\t\n",
    "class\t\tInt8\t\t\t\n",
    "class\t\tUInt8\t\t\t\n",
    "class\t\tInt16\t\t\t\n",
    "class\t\tUInt16\t\t\t\n",
    "class\t\tInt32\t\t\t\n",
    "class\t\tUInt32\t\t\t\n",
    "class\t\tInt64\t\t\t\n",
    "class\t\tIntPtr\t\t\t\n",
    "class\t\tFloat32\t\t\t\n",
    "class\t\tFloat64\t\t\t\n",
    "class\t\tStruct\t\t\t\n",
    "function\t\tATan2\t\t\t\n",
    "function\t\tSinh\t\t\t\n",
    "function\t\tCosh\t\t\t\n",
    "function\t\tTanh\t\t\t\n",
    "function\t\tRandomSeed\t\t\t\n",
    "function\t\tReplaceLineEndings\t\t\t\n",
    "function\t\tFileFullPath\t\t\t\n",
    "function\t\tFileCreateTemp\t\t\t\n",
    "function\t\tOutputDebugLine\t\t\t\n",
    "function\t\tCollect\t\t\t\n",
);

/// The functions the `ks` dialect adds beside the table of the v2.0.19.
///
/// They are the functions of the `Ks` module of KeySharp that the tree can
/// answer honestly on every machine: the four of the mathematics and the
/// three of the text and the paths are pure, and the three that touch the
/// machine ask the desktop. The rest of the module -- the ones that need a
/// network, a mailer, a compiler of scripts or a component of Windows --
/// is not in the table until it answers, the rule of the tree being that a
/// name in the table answers or refuses with the reason.
pub static KS_BUILTINS: &[crate::builtins::Builtin] = &[
    crate::builtins::Builtin {
        name: "ATan2",
        kind: crate::builtins::BuiltinKind::Function,
        min: 2,
        max: 2,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_atan2,
    },
    crate::builtins::Builtin {
        name: "Sinh",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_hyperbolic,
    },
    crate::builtins::Builtin {
        name: "Cosh",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 1,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_hyperbolic,
    },
    crate::builtins::Builtin {
        name: "Tanh",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 2,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_hyperbolic,
    },
    crate::builtins::Builtin {
        name: "RandomSeed",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_random_seed,
    },
    crate::builtins::Builtin {
        name: "ReplaceLineEndings",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 2,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_replace_line_endings,
    },
    crate::builtins::Builtin {
        name: "FileFullPath",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_file_full_path,
    },
    crate::builtins::Builtin {
        name: "FileCreateTemp",
        kind: crate::builtins::BuiltinKind::Function,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[],
        handler: crate::builtins::ks_file_create_temp,
    },
    crate::builtins::Builtin {
        name: "OutputDebugLine",
        kind: crate::builtins::BuiltinKind::Function,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_output_debug_line,
    },
    crate::builtins::Builtin {
        name: "Collect",
        kind: crate::builtins::BuiltinKind::Function,
        min: 0,
        max: 0,
        variant: 0,
        parameters: &[],
        handler: crate::builtins::ks_collect,
    },
    crate::builtins::Builtin {
        name: "Throw",
        kind: crate::builtins::BuiltinKind::Function,
        min: 0,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_throw,
    },
    crate::builtins::Builtin {
        name: "Props",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_props,
    },
    crate::builtins::Builtin {
        name: "CompileScript",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_validate_script,
    },
    crate::builtins::Builtin {
        name: "IsComponentAvailable",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_is_component_available,
    },
    crate::builtins::Builtin {
        name: "ValidateScript",
        kind: crate::builtins::BuiltinKind::Function,
        min: 1,
        max: 1,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_validate_script,
    },
    crate::builtins::Builtin {
        name: "RequestCapabilities",
        kind: crate::builtins::BuiltinKind::Function,
        min: 0,
        max: 255,
        variant: 0,
        parameters: &[crate::builtins::Parameter::Value],
        handler: crate::builtins::ks_request_capabilities,
    },
];
