//! The label that a command of the v1 names.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
//! `GroupAdd` names a label of the script in its fourth parameter, the
//! `Hotkey` in its second and the `SetTimer` in its first, and the label
//! that is not there is refused when the script loads.

use crate::dialect::TheLabelOfACommand;

/// The label that a command of the v1 names.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
/// `GroupAdd` names a label in its fourth parameter -- the label of the
/// subroutine that the group runs when it has no window -- the `Hotkey` in
/// its second -- the action of the key -- and the `SetTimer` in its first --
/// the body of the timer.
///
/// The action of a hotkey and the body of a timer may be a function of the
/// script, and the name of the function is accepted there: the `Hotkey,
/// ^!a, MyFunc` is accepted when the script has such a function, and the
/// `SetTimer, MyFunc, 1000` too. The function with arguments is not a
/// function that the v1 names: the `Hotkey, ^!a, MyFunc(1)` is refused with
/// "Target label does not exist.". The `GroupAdd` names a label and nothing
/// else: the `GroupAdd, g, t, x, MyFunc` is refused when the script has such
/// a function and no such a label.
pub(crate) fn the_label_of_the_command(name: &str) -> Option<TheLabelOfACommand> {
    if name.eq_ignore_ascii_case("GroupAdd") {
        return Some(TheLabelOfACommand::Label(3));
    }
    if name.eq_ignore_ascii_case("Hotkey") {
        return Some(TheLabelOfACommand::LabelOrFunction(1));
    }
    if name.eq_ignore_ascii_case("SetTimer") {
        return Some(TheLabelOfACommand::LabelOrFunction(0));
    }
    None
}
