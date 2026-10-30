//! The `InputHook` class of v2.
//!
//! The tree does not carry the class: it is a keyboard hook, and this
//! runtime has none. The names of it are here, because they are the
//! surface of v2, and a script that asks for one of them is refused
//! with the reason of the runtime.

/// The names of the `InputHook` class, read out of the binary.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the table of the
/// class is at 0x00125170, with a stride of forty bytes, and it holds
/// twenty-three names. The property with a setter is in it twice, the
/// getter and the setter.
pub(crate) const NAMES: [&str; 23] = [
    "__new",
    "backspaceisundo",
    "casesensitive",
    "endkey",
    "endmods",
    "endreason",
    "findanywhere",
    "inprogress",
    "input",
    "keyopt",
    "match",
    "minsendlevel",
    "notifynontext",
    "onchar",
    "onend",
    "onkeydown",
    "onkeyup",
    "start",
    "stop",
    "timeout",
    "visiblenontext",
    "visibletext",
    "wait",
];
