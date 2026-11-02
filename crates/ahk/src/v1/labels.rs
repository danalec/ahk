//! The label that a command of the v1 names, and the label that a goto of
//! the v1 names.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
//! `GroupAdd` names a label of the script in its fourth parameter, the
//! `Hotkey` in its second and the `SetTimer` in its first, and the label
//! that is not there is refused when the script loads.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

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

/// The refusal of a goto of the v1.
///
/// The name is the name of the label that the goto names. The chain of the
/// goto is the chain of the blocks that enclose it, and the labels are the
/// labels of the script with the chain of the blocks of each of them.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the label
/// that is not there at all is refused when the script loads, and the
/// complaint of the v1 is "Target label does not exist." with the label after
/// it. The jump that may not be made is refused too: the label of a block
/// that does not enclose the jump is "A Goto/Gosub must not jump into a
/// block that doesn't enclose it.", and the jump that leaves a function is
/// "Cannot jump from inside a function to outside.".
pub(crate) fn the_refusal_of_a_goto(
    name: &str,
    chain: &[crate::parser::TheBlock],
    labels: &[(String, Vec<crate::parser::TheBlock>)],
) -> Option<String> {
    // The label that is not there at all.
    let Some((_, chain_of_the_label)) = labels
        .iter()
        .find(|(label, _)| label.eq_ignore_ascii_case(name))
    else {
        return Some(format!(
            "Target label does not exist.\n     Specifically: {name}"
        ));
    };
    // The jump that enters a block that does not enclose it: the chain of
    // the blocks of the label has to be the beginning of the chain of the
    // blocks of the jump.
    if !the_chain_of_the_label_is_the_beginning(chain_of_the_label, chain) {
        return Some(
            "A Goto/Gosub must not jump into a block that doesn't enclose it.".to_string(),
        );
    }
    // The jump that leaves the function: the label has to be inside the
    // function that the jump is in.
    let function = chain.iter().rev().find(|block| block.function).map(|block| block.id);
    if let Some(function) = function {
        let inside = chain_of_the_label
            .iter()
            .any(|block| block.function && block.id == function);
        if !inside {
            return Some("Cannot jump from inside a function to outside.".to_string());
        }
    }
    None
}

/// Whether the chain of the blocks of the label is the beginning of the chain
/// of the blocks of the jump.
fn the_chain_of_the_label_is_the_beginning(
    chain_of_the_label: &[crate::parser::TheBlock],
    chain_of_the_goto: &[crate::parser::TheBlock],
) -> bool {
    if chain_of_the_label.len() > chain_of_the_goto.len() {
        return false;
    }
    chain_of_the_label
        .iter()
        .zip(chain_of_the_goto.iter())
        .all(|(label, jump)| label.id == jump.id)
}
