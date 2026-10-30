//! The `Gui` class, and the parts of a window.
//!
//! `Gui` is a window; `Gui.Control`, `Gui.ListView`, `Gui.Menu` and
//! `Gui.StatusBar` are the parts of one. The script calls the menu
//! `Menu`, and the binary calls it `Gui.Menu`: they are one table.
//!
//! All of them are shaped for Windows. A window is a Windows window, and
//! this desktop is a Wayland session without one, so the objects are made
//! -- a script may hold them -- and every name of them answers with the
//! reason it cannot work here. A name that is not in the table is not a
//! method of the class, and the runtime says so.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Detail, Runtime, RuntimeError, RuntimeFailure, TypeName, Value};

/// The names of the `Gui` class, with the thing each one does.
///
/// The names are the ones the binary gives the class, read out of the
/// surface table of v2.0.19.
const GUI: &[(&str, &str)] = &[
    ("__enum", "enumerate the windows"),
    ("__new", "make the window"),
    ("add", "add a control to the window"),
    ("addactivex", "add an ActiveX control to the window"),
    ("addbutton", "add a button to the window"),
    ("addcheckbox", "add a check box to the window"),
    ("addcombobox", "add a combo box to the window"),
    ("addcustom", "add a custom control to the window"),
    ("adddatetime", "add a date and time control to the window"),
    ("addddl", "add a drop-down list to the window"),
    ("adddropdownlist", "add a drop-down list to the window"),
    ("addedit", "add an edit control to the window"),
    ("addgroupbox", "add a group box to the window"),
    ("addhotkey", "add a hotkey control to the window"),
    ("addlink", "add a link to the window"),
    ("addlistbox", "add a list box to the window"),
    ("addlistview", "add a list view to the window"),
    ("addmonthcal", "add a month calendar to the window"),
    ("addpic", "add a picture to the window"),
    ("addpicture", "add a picture to the window"),
    ("addprogress", "add a progress bar to the window"),
    ("addradio", "add a radio button to the window"),
    ("addslider", "add a slider to the window"),
    ("addstatusbar", "add a status bar to the window"),
    ("addtab", "add a tab control to the window"),
    ("addtab2", "add a tab control to the window"),
    ("addtab3", "add a tab control to the window"),
    ("addtext", "add text to the window"),
    ("addtreeview", "add a tree view to the window"),
    ("addupdown", "add an up-down control to the window"),
    ("destroy", "destroy the window"),
    ("flash", "flash the window"),
    ("getclientpos", "read where the client area of the window is"),
    ("getpos", "read where the window is"),
    ("hide", "hide the window"),
    ("maximize", "maximize the window"),
    ("minimize", "minimize the window"),
    ("move", "move the window"),
    ("onevent", "give the window an event handler"),
    ("opt", "change the options of the window"),
    ("restore", "restore the window"),
    ("setfont", "set the font of the window"),
    ("show", "show the window"),
    ("submit", "submit the controls of the window"),
    ("__item", "give the control with that name"),
    ("hwnd", "give the handle of the window"),
    ("title", "read or set the title of the window"),
    ("name", "read or set the name of the window"),
    ("focusedctrl", "read which control of the window has the focus"),
    ("backcolor", "read or set the back colour of the window"),
    ("marginx", "read or set the left margin of the window"),
    ("marginy", "read or set the top margin of the window"),
    ("menubar", "read or set the menu bar of the window"),
];

/// The names of the `Gui.Control` class, with the thing each one does.
const CONTROL: &[(&str, &str)] = &[
    ("focus", "give the control the focus"),
    ("getpos", "read where the control is"),
    ("move", "move the control"),
    ("oncommand", "give the control a command handler"),
    ("onevent", "give the control an event handler"),
    ("onnotify", "give the control a notify handler"),
    ("opt", "change the options of the control"),
    ("redraw", "redraw the control"),
    ("setfont", "set the font of the control"),
    ("classnn", "read the class and the number of the control"),
    ("enabled", "read or set whether the control is enabled"),
    ("focused", "read whether the control has the focus"),
    ("gui", "read the window the control is in"),
    ("hwnd", "give the handle of the control"),
    ("name", "read or set the name of the control"),
    ("text", "read or set the text of the control"),
    ("type", "read the type of the control"),
    ("value", "read or set the value of the control"),
    ("visible", "read or set whether the control is visible"),
    ("add", "add a control to it"),
    ("choose", "choose an item of the control"),
    ("delete", "delete the control"),
    ("usetab", "use the tab of the window the control is in"),
    ("setformat", "set the format of the control"),
];

/// The names of the `Gui.ListView` class, with the thing each one does.
const LISTVIEW: &[(&str, &str)] = &[
    ("add", "add a row to the list"),
    ("delete", "delete a row from the list"),
    ("get", "read a row of the list"),
    ("getchild", "read the child of an item of the list"),
    ("getcount", "count the rows of the list"),
    ("getnext", "read the row after a row of the list"),
    ("getparent", "read the row above a row of the list"),
    ("getprev", "read the row before a row of the list"),
    ("getselection", "read the selected row of the list"),
    ("gettext", "read the text of a row of the list"),
    ("modify", "change a row of the list"),
    ("setimagelist", "give the list an image list"),
    ("seticon", "give a row of the list an icon"),
    ("setparts", "set the columns of the list"),
    ("settext", "set the text of a row of the list"),
];

/// The names of the `Gui.Menu` class, with the thing each one does.
///
/// The script calls this class `Menu`, and the registry hands the module
/// the two names.
const MENU: &[(&str, &str)] = &[
    ("add", "add an item to the menu bar"),
    ("addstandard", "add the standard items to the menu bar"),
    ("check", "check an item of the menu bar"),
    ("clickcount", "read or set how many clicks an item of the menu bar takes"),
    ("default", "read or set which item of the menu bar is the default"),
    ("delete", "delete an item of the menu bar"),
    ("disable", "disable an item of the menu bar"),
    ("enable", "enable an item of the menu bar"),
    ("handle", "give the handle of the menu bar"),
    ("insert", "insert an item into the menu bar"),
    ("rename", "rename an item of the menu bar"),
    ("setcolor", "set the colour of an item of the menu bar"),
    ("seticon", "set the icon of an item of the menu bar"),
    ("show", "show the menu bar"),
    ("togglecheck", "toggle the check of an item of the menu bar"),
    ("toggleenable", "toggle whether an item of the menu bar is enabled"),
    ("uncheck", "uncheck an item of the menu bar"),
];

/// The names of the `Gui.StatusBar` class, with the thing each one does.
const STATUSBAR: &[(&str, &str)] = &[
    ("getnext", "read the part after a part of the bar"),
    ("getcount", "count the parts of the bar"),
    ("gettext", "read the text of a part of the bar"),
    ("add", "add a part to the bar"),
    ("insert", "insert a part into the bar"),
    ("modify", "change a part of the bar"),
    ("delete", "delete a part of the bar"),
    ("insertcol", "insert a column into the bar"),
    ("modifycol", "change a column of the bar"),
    ("deletecol", "delete a column of the bar"),
    ("setimagelist", "give the bar an image list"),
];

/// A class of the windows: the name the registry hands us, the name the
/// script calls it, the names it has, and why it cannot work here.
struct Windows {
    /// The name the registry hands the module.
    class: &'static str,
    /// The name the script calls it.
    name: &'static str,
    /// The names the binary gives it, with the thing each one does.
    surface: &'static [(&'static str, &'static str)],
    /// Why it cannot work here.
    reason: &'static str,
}

/// The classes of the windows.
///
/// The reason is the same for all of them, and it is the reason of the
/// whole family: a window is a Windows window, and this desktop is a
/// Wayland session without one.
const CLASSES: &[Windows] = &[
    Windows { class: "gui", name: "Gui", surface: GUI, reason: "a window is a Windows window, and this desktop is a Wayland session without one" },
    Windows { class: "menu", name: "Menu", surface: MENU, reason: "a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one" },
    Windows { class: "gui.menu", name: "Menu", surface: MENU, reason: "a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one" },
    Windows { class: "gui.control", name: "Gui.Control", surface: CONTROL, reason: "a control is a control of a Windows window, and this desktop is a Wayland session without one" },
    Windows { class: "gui.listview", name: "Gui.ListView", surface: LISTVIEW, reason: "a list view is a control of a Windows window, and this desktop is a Wayland session without one" },
    Windows { class: "gui.statusbar", name: "Gui.StatusBar", surface: STATUSBAR, reason: "a status bar is a control of a Windows window, and this desktop is a Wayland session without one" },
];

/// The names of a class of the windows.
///
/// The names are the ones the binary gives the class, and they are the
/// whole surface of it: a name that is not in them is not a method of
/// the class.
pub(crate) fn names(class: &str) -> Vec<&'static str> {
    let class = class.to_lowercase();
    let Some(entry) = CLASSES.iter().find(|entry| entry.class == class) else {
        return Vec::new();
    };
    entry.surface.iter().map(|(name, _)| *name).collect()
}

/// The object a class of the windows makes.
///
/// The object is real: a script may hold it, and the report says that it
/// is there. What it cannot do is the thing that fails.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    // The third argument of v2 is the object that receives the events of
    // the window, and v2 checks it before the options.
    if let Some(events) = arguments.get(2) {
        if !matches!(events, Value::Object(_)) {
            return Err(the_type_failure(4, "Gui.Prototype.__New", TypeName::Object, events, line));
        }
    }
    // The options are a text, or the number of a window that already
    // exists. A number with a point is a text of options that v2 cannot
    // read.
    if let Some(options) = arguments.first() {
        match options {
            Value::Text(text) => {
                // The options are the dialect's, and the ones that v2 does
                // not know are refused with the reason of the dialect.
                the_options_are_read(text, line)?;
            }
            Value::Int(_) => {}
            Value::Float(_) => {
                return Err(the_failure_of_an_argument(
                    "Invalid option.".to_string(),
                    "Gui.Prototype.__New",
                    options,
                    line,
                ));
            }
            other => {
                return Err(the_type_failure(2, "Gui.Prototype.__New", TypeName::String, other, line));
            }
        }
    }
    Ok(crate::classes::object(class))
}

/// The options that the constructor of the `Gui` takes.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the options are the
/// styles of the window, and the name of one is written in full and it
/// does not know the case -- "Always" is not an option, and "caption" is.
const THE_OPTIONS: [&str; 18] = [
    "AlwaysOnTop",
    "Border",
    "Caption",
    "Disabled",
    "DPIScale",
    "E",
    "LastFound",
    "MaximizeBox",
    "MinimizeBox",
    "MinSize",
    "MaxSize",
    "OwnDialogs",
    "Owner",
    "Parent",
    "Resize",
    "SysMenu",
    "Theme",
    "ToolWindow",
];

/// The options that name a window of the machine.
///
/// The owner of a window and the parent of it are windows, and this
/// desktop has none: the construction is refused with the reason of the
/// machine.
const THE_OPTIONS_THAT_NAME_A_WINDOW: [&str; 2] = ["Owner", "Parent"];

/// The reading of the options of the `Gui`.
///
/// The options are the words of the text, and each one of them may carry
/// a sign -- `+Resize` adds the option and `-Resize` takes it away -- and
/// the name of one may carry its value: `MinSize100x200` is the `MinSize`
/// with the value of it.
///
/// Measured on the binary of v2.0.19 on 2026-10-31. A word that is a
/// number is a value and not an option: `0` and `0x10` are taken, and
/// `12x34` is not. The name of the option that is refused is the word
/// without its sign, and the owner of a window and the parent of it are
/// refused with the reason of the machine.
fn the_options_are_read(options: &str, line: u32) -> Result<(), RuntimeFailure> {
    for word in options.split([' ', '\t']) {
        if word.is_empty() {
            continue;
        }
        let name = word.strip_prefix(['+', '-']).unwrap_or(word);
        if the_number_is_one(name) {
            continue;
        }
        let lower = name.to_lowercase();
        let known = THE_OPTIONS.iter().find(|option| {
            let option = option.to_lowercase();
            lower.starts_with(&option) && the_value_is_one(&lower[option.len()..])
        });
        let Some(option) = known.copied() else {
            return Err(the_failure_of_an_argument(
                "Invalid option.".to_string(),
                "Gui.Prototype.__New",
                &Value::Text(name.to_string()),
                line,
            ));
        };
        if THE_OPTIONS_THAT_NAME_A_WINDOW.contains(&option) {
            let reason = CLASSES
                .iter()
                .find(|entry| entry.class == "gui")
                .map(|entry| entry.reason)
                .unwrap_or("a window is a Windows window");
            return Err(RuntimeFailure::Detailed(
                RuntimeError {
                    message: format!(
                        "Gui cannot take the {} of the window: {reason}.",
                        option.to_lowercase()
                    ),
                    line,
                },
                Detail { what: "Gui.Prototype.__New".to_string(), extra: word.to_string() },
            ));
        }
    }
    Ok(())
}

/// Whether the text is a number of v2: a decimal or a hexadecimal.
fn the_number_is_one(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    match text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        Some(hex) => !hex.is_empty() && hex.chars().all(|c| c.is_ascii_hexdigit()),
        None => text.chars().all(|c| c.is_ascii_digit()),
    }
}

/// Whether the text is the value of an option: nothing, a number, or two
/// numbers with an `x` between them.
fn the_value_is_one(text: &str) -> bool {
    if text.is_empty() || the_number_is_one(text) {
        return true;
    }
    match text.split_once('x') {
        Some((left, right)) => the_number_is_one(left) && the_number_is_one(right),
        None => false,
    }
}

/// The failure of an argument of the constructor that is of the wrong
/// type.
///
/// The number of the parameter of v2 counts the base of the class, and
/// the value that was given goes in the `Extra` of the error.
fn the_type_failure(
    number: usize,
    name: &str,
    wants: TypeName,
    value: &Value,
    line: u32,
) -> RuntimeFailure {
    let message = crate::runtime::requires(number, name, wants, value);
    the_failure_of_an_argument(message, name, value, line)
}

/// The failure of an argument of the constructor.
fn the_failure_of_an_argument(
    message: String,
    name: &str,
    value: &Value,
    line: u32,
) -> RuntimeFailure {
    RuntimeFailure::Detailed(
        RuntimeError { message, line },
        Detail { what: name.to_string(), extra: value.to_text() },
    )
}

/// A method of a class of the windows.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    class: &str,
    method: &str,
    _this: &Value,
    _arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(Err(refused(class, method, line)?))
}

/// A property of a class of the windows.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    class: &str,
    name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(Err(refused(class, name, line)?))
}

/// A property of a class of the windows that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    class: &str,
    name: &str,
    _this: &Value,
    _value: Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    Some(Err(refused(class, name, line)?))
}

/// The failure a class of the windows gives when it is asked for a name it
/// has.
///
/// `None` means the class has no such name: the runtime then says the
/// value has no such method.
fn refused(class: &str, name: &str, line: u32) -> Option<RuntimeFailure> {
    let entry = CLASSES.iter().find(|entry| entry.class == class.to_lowercase())?;
    let what = entry.surface.iter().find(|entry| entry.0 == name.to_lowercase()).map(|entry| entry.1)?;
    Some(RuntimeFailure::Error(RuntimeError {
        message: format!("{} cannot {what}: {}.", entry.name, entry.reason),
        line,
    }))
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use super::{CLASSES, CONTROL, GUI, LISTVIEW, MENU, STATUSBAR};

    /// The surface of v2.0.19, read out of the binary.
    const SURFACE: &str = include_str!("../../reference/ahk-v2.0.19.surface.tsv");

    /// The names the binary gives a class of the windows.
    fn names_of(class: &str) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for line in SURFACE.lines() {
            let mut fields = line.trim_end().split('\t');
            let kind = fields.next().unwrap_or("");
            let owner = fields.next().unwrap_or("");
            let name = fields.next().unwrap_or("");
            if kind == "method" && owner.to_lowercase() == class {
                names.push(name.to_lowercase());
            }
        }
        names
    }

    /// Every name of the binary's table is carried, and nothing else is.
    ///
    /// The table is the whole surface of the class: a name that is not in
    /// it is not a method of the class.
    #[test]
    fn every_name_of_the_binary_is_carried() {
        for (class, table) in [
            ("gui", GUI),
            ("gui.control", CONTROL),
            ("gui.listview", LISTVIEW),
            ("gui.menu", MENU),
            ("gui.statusbar", STATUSBAR),
        ] {
            let carried: Vec<&str> = table.iter().map(|(name, _)| *name).collect();
            for name in names_of(class) {
                assert!(carried.contains(&name.as_str()), "{class} does not carry {name}");
            }
            for name in &carried {
                let binary = names_of(class);
                assert!(
                    binary.iter().any(|entry| entry == name),
                    "{class} carries {name}, and the binary does not"
                );
            }
        }
    }

    /// The class the registry hands decides the table.
    ///
    /// `Focus` is a name of a control and not of a window, and the two
    /// tables have names in common besides.
    #[test]
    fn the_class_the_registry_hands_decides_the_table() {
        assert!(super::refused("gui.control", "focus", 1).is_some());
        assert!(super::refused("gui", "focus", 1).is_none());
        assert!(super::refused("gui", "show", 1).is_some());
        assert!(super::refused("menu", "add", 1).is_some());
        assert!(super::refused("gui.menu", "add", 1).is_some());
    }

    /// Every class of the windows says why it cannot work here.
    #[test]
    fn every_class_says_why_it_cannot_work_here() {
        for entry in CLASSES {
            assert!(!entry.surface.is_empty(), "{} has no names", entry.name);
            assert!(entry.reason.contains("Wayland"), "{} says no reason", entry.name);
        }
    }
}
