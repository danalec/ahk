//! The parameters of the v1 that want a number.
//!
//! Measured on the CHM of AutoHotkey v1.1.37.02 and on the binary on
//! 2026-11-02: the v1 does not check the kind of the arguments of a
//! built-in. The parameter that wants a number takes the number that the text
//! of it starts with -- "65abc" is 65 -- and the hex of the v1 is a number:
//! "0x10" is 16.
//!
//! The documentation of the v1 says which parameter is a number: "Numeric
//! parameters accept a literal number or an expression, and can be identified
//! by phrasing like 'This parameter can be an expression.'".
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
    // The commands of the v1.
    //
    // The number of the seconds that the command waits, and the number of
    // what it waits for.
    ("ClipWait", &[0, 1]),
    // The value of the arithmetic of a variable of the script.
    ("EnvAdd", &[1]),
    ("EnvDiv", &[1]),
    ("EnvMult", &[1]),
    ("EnvSub", &[1]),
    // The code that the script leaves.
    ("Exit", &[0]),
    ("ExitApp", &[0]),
    // Whether the file is written over the one that is there.
    ("FileCopy", &[2]),
    ("FileInstall", &[2]),
    ("FileMove", &[2]),
    // The place of the mouse, the number of the clicks and the speed.
    ("MouseClick", &[1, 2, 3, 4]),
    ("MouseClickDrag", &[1, 2, 3, 4, 5]),
    ("MouseMove", &[0, 1, 2]),
    // The place of the pixel.
    ("PixelGetColor", &[1, 2]),
    // The bounds of the number, and the seed of the generator.
    ("Random", &[1, 2, 3]),
    // The level of the keyboard.
    ("SendLevel", &[0]),
    // The speed of the mouse.
    ("SetDefaultMouseSpeed", &[0]),
    // The delay of the keyboard, and the duration of the press.
    ("SetKeyDelay", &[0, 1]),
    // The delay of the mouse.
    ("SetMouseDelay", &[0]),
    // The delay of the script.
    ("Sleep", &[0]),
    // The frequency of the sound, and its duration.
    ("SoundBeep", &[0, 1]),
    // The functions of the v1.
    //
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
///
/// The name of the built-in may come in any case: the parser writes it in
/// lower case, and the table of the built-ins writes it as it is spelled.
pub(crate) fn wants_a_number(name: &str, index: usize) -> bool {
    THE_NUMBERS.iter().any(|(built_in, places)| {
        built_in.eq_ignore_ascii_case(name) && places.contains(&(index as u8))
    })
}
