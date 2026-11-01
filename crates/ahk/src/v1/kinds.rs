//! The parameters of the v1 that want a number.
//!
//! Measured on the CHM of AutoHotkey v1.1.37.02 and on the binary on
//! 2026-11-02: the v1 does not check the kind of the arguments of a
//! built-in. The parameter that wants a number takes the number that the text
//! of it starts with -- "65abc" is 65 -- and the hex of the v1 is a number:
//! "0x10" is 16.
//!
//! The built-in that reads the number of its arguments by itself is not in
//! this table: the rule of it is the one of the built-in, and the rule of the
//! table is not its own. The math of the v1 and the `Chr` are such, and the
//! module of the math of the v1 reads the numbers by itself.

/// The parameters that want a number, by the name of the built-in and the
/// places of the parameters.
///
/// The places are the ones of the v1: the command of the v1 writes into a
/// variable of the script, and that variable is a parameter of its own.
const THE_NUMBERS: &[(&str, &[u8])] = &[
    // The place where the search begins, and the occurrence that the caller
    // wants.
    ("InStr", &[3, 4]),
    // The number of the parts that the caller wants.
    ("StrSplit", &[3]),
    // The place where the text begins, and the number of the characters.
    ("SubStr", &[1, 2]),
    // The capacity that the caller asks for.
    ("VarSetCapacity", &[1]),
];

/// Whether the parameter at this place of this built-in wants a number.
pub(crate) fn wants_a_number(name: &str, index: usize) -> bool {
    THE_NUMBERS
        .iter()
        .any(|(built_in, places)| *built_in == name && places.contains(&(index as u8)))
}
