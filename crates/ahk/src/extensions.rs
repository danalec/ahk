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
//! The first block is the members of the `Array` that KeySharp adds. They
//! are measured on the documentation of KeySharp
//! (`Keysharp/docs/reference.md`, the lines of the members of the classes
//! of AutoHotkey), which says what each one answers and what it does with
//! the index it starts at.

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

/// The members that the `ks` dialect adds to the class, in the spelling of
/// the class table.
pub fn the_members_of(class: &str) -> Vec<&'static str> {
    let class = class.to_lowercase();
    MEMBERS
        .iter()
        .filter(|(owner, _)| *owner == class)
        .map(|(_, name)| *name)
        .collect()
}

/// The members the `ks` dialect adds, with the name in the spelling of the
/// dialect and the number of the parameters: the smallest and the largest,
/// where -1 means "as many as the caller gives".
///
/// The numbers are the ones of the documentation of KeySharp
/// (`Keysharp/docs/reference.md`, the members of the `Array`): every member
/// has a parameter that is required, and every one but `Sort` has a second
/// one that is optional.
const ARRAY_METHODS: &[(&str, &str, i64, i64)] = &[
    ("contains", "Contains", 1, 1),
    ("filter", "Filter", 1, 2),
    ("findindex", "FindIndex", 1, 2),
    ("indexof", "IndexOf", 1, 2),
    ("join", "Join", 0, 1),
    ("map", "Map", 1, 2),
    ("remove", "Remove", 0, 1),
    ("sort", "Sort", 1, 1),
    ("tostring", "ToString", 0, 0),
];

/// The member of the `ks` extensions that a class answers, with the name in
/// the spelling of the dialect and the numbers of the parameters.
///
/// The class and the member are the ones the script wrote, and the answer
/// does not care about the case. `None` is the answer for a member that the
/// extensions do not add, and for a class they do not touch.
pub fn the_method_of(class: &str, member: &str) -> Option<(&'static str, i64, i64)> {
    let class = class.to_lowercase();
    let member = member.to_lowercase();
    if class != "array" {
        return None;
    }
    ARRAY_METHODS
        .iter()
        .find(|(name, _, _, _)| *name == member)
        .map(|(_, spelled, min, max)| (*spelled, *min, *max))
}

/// The rows of the surface that the `ks` dialect adds to the surface of the
/// v2.0.19.
///
/// The shape of a row is the one of the table of the v2: the kind, the
/// class, the name, and the columns the kinds after the method do not use.
/// The state of every row is answered at the run, the way the state of a
/// method of the v2 is.
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
);