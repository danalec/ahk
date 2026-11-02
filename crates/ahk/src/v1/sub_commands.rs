//! The sub-commands of the commands of the v1 that the v1 checks when the
//! script loads.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
//! command whose parameter names a sub-command refuses the sub-command that
//! the v1 does not know when the script loads: "Parameter #N invalid." with
//! the text of the parameter after it, on a line of its own. The
//! sub-command that the v1 knows has rules of its own, and the complaint of
//! the v1 is one of three:
//!
//! - "Parameter #N must be blank in this case." -- the parameter that the
//!   sub-command does not take.
//! - "Parameter #N must not be blank in this case." -- the parameter that
//!   the sub-command needs.
//! - "Parameter #N and beyond should be omitted in this case." -- the
//!   parameter that the sub-command does not take, and the ones after it.
//!
//! The complaint names the text of the parameter when it is not empty, and
//! the `WinSet` does not name it at all. The complaint of the parameter that
//! may not be blank never names it.
//!
//! The sub-command may carry the name of the window before it -- the
//! "MyGui:Show" of the `Gui` -- and it may be a list of options that begins
//! with a sign -- the "+Resize" of the `Gui`. The name of the window is not
//! a sub-command, and the list of options is not one either.

use alloc::format;
use alloc::string::{String, ToString};

/// The refusal of a sub-command of a command of the v1.
///
/// The name is the name of the command as the script wrote it, and the
/// parameters are the texts that the script wrote. `None` means that the
/// command is not one that the v1 checks, or that the sub-command is one
/// that the v1 wants.
pub(crate) fn the_refusal_of_a_sub_command(name: &str, parameters: &[String]) -> Option<String> {
    // The command whose rule is its own comes first.
    if let Some(rule) = THE_OWN.iter().find(|rule| rule.name.eq_ignore_ascii_case(name)) {
        return (rule.check)(parameters);
    }
    let rule = SUB_COMMANDS.iter().find(|rule| rule.name.eq_ignore_ascii_case(name))?;
    the_refusal_of_the_sub_command(rule, parameters)
}

/// A command of the v1 whose rule is not the rule of a sub-command.
struct TheOwn {
    /// The name of the command, as the table of the v1 spells it.
    name: &'static str,
    /// The check of the parameters.
    check: fn(&[String]) -> Option<String>,
}

/// The commands of the v1 whose rule is not the rule of a sub-command.
const THE_OWN: &[TheOwn] = &[
    TheOwn { name: "Process", check: the_process },
    TheOwn { name: "SoundGet", check: the_sound },
    TheOwn { name: "SoundSet", check: the_sound },
    TheOwn { name: "RegRead", check: the_reg_read },
    TheOwn { name: "RegWrite", check: the_reg_write },
    TheOwn { name: "RegDelete", check: the_reg_delete },
];

/// The rule of a command of the v1 whose parameter names a sub-command.
struct Rule {
    /// The name of the command, as the table of the v1 spells it.
    name: &'static str,
    /// The place of the sub-command.
    place: usize,
    /// Whether the sub-command may carry the name of a window before it,
    /// separated by a colon.
    carries_a_name: bool,
    /// Whether the sub-command may be blank.
    allows_a_blank: bool,
    /// What the v1 wants when the sub-command is a list of options that
    /// begins with a sign.
    options: &'static [TheWant],
    /// The sub-commands that the v1 knows.
    sub_commands: &'static [SubCommand],
}

/// A sub-command of a command of the v1.
struct SubCommand {
    /// The name of the sub-command, as the v1 spells it.
    name: &'static str,
    /// What the sub-command wants of the parameters that follow it.
    wants: &'static [TheWant],
}

/// What a sub-command wants of the parameters that follow it.
struct TheWant {
    /// The place of the parameter.
    place: usize,
    /// The complaint of the v1 when the parameter is not the one that the
    /// sub-command wants.
    complaint: TheComplaint,
}

/// The complaint of the v1 about a parameter that the sub-command does not
/// want.
#[derive(Clone, Copy)]
enum TheComplaint {
    /// "Parameter #N must be blank in this case." with the text of the
    /// parameter after it.
    Blank,
    /// "Parameter #N must be blank in this case." without the text of the
    /// parameter.
    BlankWithoutTheText,
    /// "Parameter #N must not be blank in this case."
    NotBlank,
    /// "Parameter #N and beyond should be omitted in this case." with the
    /// text of the parameter after it.
    Omitted,
}

/// The commands of the v1 whose parameter names a sub-command.
const SUB_COMMANDS: &[Rule] = &[
    Rule {
        name: "Control",
        place: 0,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Add", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Delete", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Style", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ExStyle", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ShowDropDown", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "HideDropDown", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "TabLeft", wants: &[] },
            SubCommand { name: "TabRight", wants: &[] },
            SubCommand { name: "Choose", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ChooseString", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "EditPaste", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Enable", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Disable", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Show", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Hide", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Check", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Uncheck", wants: &[TheWant { place: 1, complaint: TheComplaint::Blank }] },
        ],
    },
    Rule {
        name: "ControlGet",
        place: 1,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Checked", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Enabled", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Visible", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Tab", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Choice", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "LineCount", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "CurrentLine", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "CurrentCol", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Selected", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Style", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "ExStyle", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "Hwnd", wants: &[TheWant { place: 2, complaint: TheComplaint::Blank }] },
            SubCommand { name: "FindString", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Line", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "List", wants: &[] },
        ],
    },
    Rule {
        name: "Drive",
        place: 0,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Lock", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Unlock", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Label", wants: &[TheWant { place: 1, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Eject", wants: &[] },
        ],
    },
    Rule {
        name: "DriveGet",
        place: 1,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "List", wants: &[] },
            SubCommand { name: "StatusCD", wants: &[] },
            SubCommand { name: "Capacity", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "FileSystem", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Label", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Serial", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Type", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Status", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
        ],
    },
    Rule {
        name: "Gui",
        place: 0,
        carries_a_name: true,
        allows_a_blank: false,
        // The list of options of the `Gui` takes no parameter after it.
        options: &[TheWant { place: 1, complaint: TheComplaint::Omitted }],
        sub_commands: &[
            SubCommand { name: "New", wants: &[] },
            SubCommand { name: "Add", wants: &[] },
            SubCommand { name: "Show", wants: &[] },
            SubCommand { name: "Submit", wants: &[] },
            SubCommand { name: "Cancel", wants: &[] },
            SubCommand { name: "Hide", wants: &[] },
            SubCommand { name: "Destroy", wants: &[] },
            SubCommand { name: "Font", wants: &[] },
            SubCommand { name: "Color", wants: &[] },
            SubCommand { name: "Margin", wants: &[] },
            SubCommand { name: "Menu", wants: &[] },
            SubCommand { name: "Minimize", wants: &[] },
            SubCommand { name: "Maximize", wants: &[] },
            SubCommand { name: "Restore", wants: &[] },
            SubCommand { name: "Flash", wants: &[] },
            SubCommand { name: "Default", wants: &[] },
            SubCommand { name: "Tab", wants: &[] },
        ],
    },
    Rule {
        name: "GuiControl",
        place: 0,
        carries_a_name: true,
        allows_a_blank: true,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Text", wants: &[] },
            SubCommand { name: "Move", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "MoveDraw", wants: &[] },
            SubCommand { name: "Focus", wants: &[] },
            SubCommand { name: "Disable", wants: &[] },
            SubCommand { name: "Enable", wants: &[] },
            SubCommand { name: "Hide", wants: &[] },
            SubCommand { name: "Show", wants: &[] },
            SubCommand { name: "Choose", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ChooseString", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Font", wants: &[] },
        ],
    },
    Rule {
        name: "GuiControlGet",
        place: 1,
        carries_a_name: true,
        allows_a_blank: true,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Pos", wants: &[] },
            SubCommand { name: "Focus", wants: &[
                TheWant { place: 2, complaint: TheComplaint::Blank },
                TheWant { place: 3, complaint: TheComplaint::Blank },
            ] },
            SubCommand { name: "FocusV", wants: &[
                TheWant { place: 2, complaint: TheComplaint::Blank },
                TheWant { place: 3, complaint: TheComplaint::Blank },
            ] },
            SubCommand { name: "Enabled", wants: &[] },
            SubCommand { name: "Visible", wants: &[] },
            SubCommand { name: "Hwnd", wants: &[] },
            SubCommand { name: "Name", wants: &[] },
        ],
    },
    Rule {
        name: "Menu",
        place: 1,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "Add", wants: &[] },
            SubCommand { name: "Insert", wants: &[] },
            SubCommand { name: "Delete", wants: &[] },
            SubCommand { name: "DeleteAll", wants: &[] },
            SubCommand { name: "Rename", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Check", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Uncheck", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ToggleCheck", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Enable", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Disable", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "ToggleEnable", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "Default", wants: &[] },
            SubCommand { name: "NoDefault", wants: &[TheWant { place: 2, complaint: TheComplaint::Omitted }] },
            SubCommand { name: "Standard", wants: &[TheWant { place: 2, complaint: TheComplaint::Omitted }] },
            SubCommand { name: "NoStandard", wants: &[TheWant { place: 2, complaint: TheComplaint::Omitted }] },
            SubCommand { name: "Icon", wants: &[] },
            SubCommand { name: "NoIcon", wants: &[] },
            SubCommand { name: "Tip", wants: &[] },
            SubCommand { name: "Show", wants: &[] },
            SubCommand { name: "Color", wants: &[] },
            SubCommand { name: "Click", wants: &[TheWant { place: 2, complaint: TheComplaint::NotBlank }] },
            SubCommand { name: "MainWindow", wants: &[TheWant { place: 2, complaint: TheComplaint::Omitted }] },
            SubCommand { name: "NoMainWindow", wants: &[TheWant { place: 2, complaint: TheComplaint::Omitted }] },
            SubCommand { name: "UseErrorLevel", wants: &[] },
        ],
    },
    Rule {
        name: "SysGet",
        place: 1,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "MonitorCount", wants: &[] },
            SubCommand { name: "MonitorPrimary", wants: &[] },
            SubCommand { name: "Monitor", wants: &[] },
            SubCommand { name: "MonitorWorkArea", wants: &[] },
            SubCommand { name: "MonitorName", wants: &[] },
        ],
    },
    Rule {
        name: "WinGet",
        place: 1,
        carries_a_name: false,
        allows_a_blank: true,
        options: &[],
        sub_commands: &[
            SubCommand { name: "ID", wants: &[] },
            SubCommand { name: "IDLast", wants: &[] },
            SubCommand { name: "PID", wants: &[] },
            SubCommand { name: "ProcessName", wants: &[] },
            SubCommand { name: "ProcessPath", wants: &[] },
            SubCommand { name: "Count", wants: &[] },
            SubCommand { name: "List", wants: &[] },
            SubCommand { name: "MinMax", wants: &[] },
            SubCommand { name: "ControlList", wants: &[] },
            SubCommand { name: "ControlListHwnd", wants: &[] },
            SubCommand { name: "Transparent", wants: &[] },
            SubCommand { name: "TransColor", wants: &[] },
            SubCommand { name: "Style", wants: &[] },
            SubCommand { name: "ExStyle", wants: &[] },
        ],
    },
    Rule {
        name: "WinSet",
        place: 0,
        carries_a_name: false,
        allows_a_blank: false,
        options: &[],
        sub_commands: &[
            SubCommand { name: "AlwaysOnTop", wants: &[] },
            SubCommand { name: "Bottom", wants: &[TheWant { place: 1, complaint: TheComplaint::BlankWithoutTheText }] },
            SubCommand { name: "Top", wants: &[TheWant { place: 1, complaint: TheComplaint::BlankWithoutTheText }] },
            SubCommand { name: "Disable", wants: &[TheWant { place: 1, complaint: TheComplaint::BlankWithoutTheText }] },
            SubCommand { name: "Enable", wants: &[TheWant { place: 1, complaint: TheComplaint::BlankWithoutTheText }] },
            SubCommand { name: "Redraw", wants: &[TheWant { place: 1, complaint: TheComplaint::BlankWithoutTheText }] },
            SubCommand { name: "Style", wants: &[] },
            SubCommand { name: "ExStyle", wants: &[] },
            SubCommand { name: "Region", wants: &[] },
            SubCommand { name: "Transparent", wants: &[] },
            SubCommand { name: "TransColor", wants: &[] },
        ],
    },
];

/// The refusal of the sub-command of a command of the v1.
fn the_refusal_of_the_sub_command(rule: &Rule, parameters: &[String]) -> Option<String> {
    let text = the_text_at(parameters, rule.place);
    let trimmed = text.trim();
    // The name of the window may come before the sub-command.
    let name = if rule.carries_a_name {
        match trimmed.split_once(':') {
            Some((_, rest)) => rest.trim(),
            None => trimmed,
        }
    } else {
        trimmed
    };
    // The list of options is not a sub-command.
    if name.starts_with('+') || name.starts_with('-') {
        return the_refusal_of_the_wants(rule.options, parameters);
    }
    if name.is_empty() && rule.allows_a_blank {
        return None;
    }
    let Some(sub) = rule.sub_commands.iter().find(|sub| sub.name.eq_ignore_ascii_case(name)) else {
        return Some(the_refusal_of_an_invalid_parameter(rule.place, trimmed));
    };
    the_refusal_of_the_wants(sub.wants, parameters)
}

/// The refusal of the parameters that a sub-command wants.
fn the_refusal_of_the_wants(wants: &[TheWant], parameters: &[String]) -> Option<String> {
    for want in wants {
        match want.complaint {
            TheComplaint::Blank => {
                let text = the_text_at(parameters, want.place).trim();
                if text.is_empty() {
                    continue;
                }
                return Some(format!(
                    "Parameter #{} must be blank in this case.\n     Specifically: {text}",
                    want.place + 1
                ));
            }
            TheComplaint::BlankWithoutTheText => {
                if the_text_at(parameters, want.place).trim().is_empty() {
                    continue;
                }
                return Some(format!("Parameter #{} must be blank in this case.", want.place + 1));
            }
            TheComplaint::NotBlank => {
                if the_text_at(parameters, want.place).trim().is_empty() {
                    return Some(format!(
                        "Parameter #{} must not be blank in this case.",
                        want.place + 1
                    ));
                }
            }
            TheComplaint::Omitted => {
                // The parameter and the ones after it have to be left out.
                let mut omitted = false;
                for place in want.place..parameters.len() {
                    if !the_text_at(parameters, place).trim().is_empty() {
                        omitted = true;
                    }
                }
                if !omitted {
                    continue;
                }
                let text = the_text_at(parameters, want.place).trim();
                if text.is_empty() {
                    return Some(format!(
                        "Parameter #{} and beyond should be omitted in this case.",
                        want.place + 1
                    ));
                }
                return Some(format!(
                    "Parameter #{} and beyond should be omitted in this case.\n     Specifically: {text}",
                    want.place + 1
                ));
            }
        }
    }
    None
}

/// The refusal of a parameter that the v1 does not want.
///
/// The text is named after it when it is not empty.
fn the_refusal_of_an_invalid_parameter(place: usize, text: &str) -> String {
    if text.is_empty() {
        return format!("Parameter #{} invalid.", place + 1);
    }
    format!("Parameter #{} invalid.\n     Specifically: {text}", place + 1)
}

/// Whether the parameter of this place of this command has to be there.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// sub-command of a command of the v1 has to be there, and the complaint of
/// the v1 is the one of the parameter that may not be blank: "\"X\"
/// requires that parameter #N be non-blank.". The commands whose
/// sub-command may be left out are the `GuiControl`, the `GuiControlGet`,
/// the `WinGet`, the `RegRead` and the `SoundGet`.
pub(crate) fn the_parameter_may_not_be_blank(name: &str, place: usize) -> bool {
    let wanted = if name.eq_ignore_ascii_case("Control")
        || name.eq_ignore_ascii_case("Drive")
        || name.eq_ignore_ascii_case("Gui")
        || name.eq_ignore_ascii_case("Process")
        || name.eq_ignore_ascii_case("WinSet")
    {
        1
    } else if name.eq_ignore_ascii_case("ControlGet")
        || name.eq_ignore_ascii_case("Menu")
        || name.eq_ignore_ascii_case("SysGet")
    {
        2
    } else {
        return false;
    };
    place == wanted
}

/// The text of the parameter at this place, or the empty text.
fn the_text_at(texts: &[String], place: usize) -> &str {
    texts.get(place).map(String::as_str).unwrap_or("")
}

/// The `Process` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// sub-command is the "Exist", the "Close", the "Priority", the "Wait" or
/// the "WaitClose". The "Exist" wants nothing, the "Close", the "Wait" and
/// the "WaitClose" want the name of the process, and the "Priority" wants
/// the level of it in the third parameter. The sub-command that the v1 does
/// not know wants the name of the process too, and the complaint of the v1
/// names it when it is there.
fn the_process(parameters: &[String]) -> Option<String> {
    let sub = the_text_at(parameters, 0).trim();
    match sub.to_lowercase().as_str() {
        "exist" => None,
        "close" | "wait" | "waitclose" => {
            if the_text_at(parameters, 1).trim().is_empty() {
                return Some("Parameter #2 must not be blank in this case.".to_string());
            }
            None
        }
        "priority" => {
            let level = the_text_at(parameters, 2).trim();
            if level.is_empty() {
                return Some("Parameter #3 invalid.".to_string());
            }
            if !level.contains('%') && !is_a_level_of_the_priority(level) {
                return Some(the_refusal_of_an_invalid_parameter(2, level));
            }
            None
        }
        _ => {
            // The v1 checks the number of the parameters before it names the
            // sub-command that it does not know.
            if parameters.len() < 2 {
                return Some("Parameter #2 must not be blank in this case.".to_string());
            }
            Some(the_refusal_of_an_invalid_parameter(0, sub))
        }
    }
}

/// Whether the text is a level of the priority of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// levels are the "Low", the "BelowNormal", the "Normal", the "AboveNormal"
/// and the "High", and the v1 compares the beginning of the text with them:
/// the "HighNormal" and the "High, extra" are the "High". The case of the
/// word does not matter. The "Idle" is not a level of the v1.
fn is_a_level_of_the_priority(text: &str) -> bool {
    let text = text.trim().to_lowercase();
    ["low", "belownormal", "normal", "abovenormal", "high"]
        .iter()
        .any(|level| text.starts_with(level))
}

/// The sound of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// second parameter is the component of the sound -- the "Master" or the
/// "Microphone" -- and the third is the control of it -- the "Volume", the
/// "Mute", the "Bass", the "Treble" or the "Pan". The case of the word does
/// not matter. The control that is not there is the one of the v1, and the
/// text that carries a percent sign is a variable, and the v1 does not know
/// its value when the script loads.
fn the_sound(parameters: &[String]) -> Option<String> {
    let component = the_text_at(parameters, 1).trim();
    if component.is_empty() || component.contains('%') {
        return None;
    }
    if !matches!(component.to_lowercase().as_str(), "master" | "microphone") {
        return Some(the_refusal_of_an_invalid_parameter(1, component));
    }
    let control = the_text_at(parameters, 2).trim();
    if control.is_empty() || control.contains('%') {
        return None;
    }
    if !is_a_control_of_the_sound(control) {
        return Some(the_refusal_of_an_invalid_parameter(2, control));
    }
    None
}

/// Whether the text is a control of the sound of the v1.
fn is_a_control_of_the_sound(text: &str) -> bool {
    matches!(
        text.trim().to_lowercase().as_str(),
        "volume" | "mute" | "bass" | "treble" | "pan"
    )
}

/// The `RegRead` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the
/// second parameter is the key of the registry, and it has to begin with
/// one of the roots of the registry.
fn the_reg_read(parameters: &[String]) -> Option<String> {
    the_refusal_of_a_key(parameters, 1)
}

/// The `RegWrite` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the first
/// parameter is the kind of the value -- the "REG_SZ", the "REG_EXPAND_SZ",
/// the "REG_MULTI_SZ", the "REG_DWORD" or the "REG_BINARY" -- and the second
/// is the key of the registry.
fn the_reg_write(parameters: &[String]) -> Option<String> {
    let kind = the_text_at(parameters, 0).trim();
    if !kind.is_empty() && !kind.contains('%') && !is_a_kind_of_a_value_of_the_registry(kind) {
        return Some(the_refusal_of_an_invalid_parameter(0, kind));
    }
    the_refusal_of_a_key(parameters, 1)
}

/// The `RegDelete` of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the first
/// parameter is the key of the registry.
fn the_reg_delete(parameters: &[String]) -> Option<String> {
    the_refusal_of_a_key(parameters, 0)
}

/// Whether the text is a kind of a value of the registry of the v1.
fn is_a_kind_of_a_value_of_the_registry(text: &str) -> bool {
    matches!(
        text.trim().to_lowercase().as_str(),
        "reg_sz" | "reg_expand_sz" | "reg_multi_sz" | "reg_dword" | "reg_binary"
    )
}

/// The refusal of a key of the registry of the v1.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-02: the key
/// has to begin with one of the roots of the registry -- the "HKCU", the
/// "HKLM", the "HKCR", the "HKU", the "HKCC", the "HKEY_CURRENT_USER",
/// the "HKEY_LOCAL_MACHINE", the "HKEY_CLASSES_ROOT", the "HKEY_USERS" or
/// the "HKEY_CURRENT_CONFIG" -- and the case of the letters does not
/// matter. The key that carries a percent sign is a variable, and the v1
/// does not know its value when the script loads.
fn the_refusal_of_a_key(parameters: &[String], place: usize) -> Option<String> {
    let key = the_text_at(parameters, place).trim();
    if key.is_empty() || key.contains('%') {
        return None;
    }
    let root = key.split('\\').next().unwrap_or(key);
    if is_a_root_of_the_registry(root) {
        return None;
    }
    Some(the_refusal_of_an_invalid_parameter(place, key))
}

/// Whether the text is a root of the registry of the v1.
fn is_a_root_of_the_registry(text: &str) -> bool {
    matches!(
        text.trim().to_lowercase().as_str(),
        "hkcu"
            | "hklm"
            | "hkcr"
            | "hku"
            | "hkcc"
            | "hkey_current_user"
            | "hkey_local_machine"
            | "hkey_classes_root"
            | "hkey_users"
            | "hkey_current_config"
    )
}
