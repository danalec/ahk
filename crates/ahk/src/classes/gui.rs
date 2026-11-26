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
    (
        "getclientpos",
        "read where the client area of the window is",
    ),
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
    (
        "focusedctrl",
        "read which control of the window has the focus",
    ),
    ("backcolor", "read or set the back colour of the window"),
    ("marginx", "read or set the left margin of the window"),
    ("marginy", "read or set the top margin of the window"),
    ("menubar", "read or set the menu bar of the window"),
];

/// The names of the `Gui.Control` class that are methods.
///
/// Measured on the binary of v2.0.31: the class of a
/// control owns nine methods, and the `Add`, the `Choose` and the
/// `Delete` are not among them -- they are the names of the class of a
/// list of items, the `Gui.List`, and the classes of a list box, of a
/// combo box, of a drop-down list and of a tab derive from it.
const CONTROL_METHODS: &[(&str, &str)] = &[
    ("Focus", "give the control the focus"),
    ("GetPos", "read where the control is"),
    ("Move", "move the control"),
    ("OnCommand", "give the control a command handler"),
    ("OnEvent", "give the control an event handler"),
    ("OnNotify", "give the control a notify handler"),
    ("Opt", "change the options of the control"),
    ("Redraw", "redraw the control"),
    ("SetFont", "set the font of the control"),
];

/// The names of the `Gui.Control` class that are properties.
///
/// Measured on the binary of v2.0.31: the class of a
/// control owns ten properties, and they are the ones of every control.
const CONTROL_PROPERTIES: &[(&str, &str)] = &[
    ("ClassNN", "read the class and the number of the control"),
    ("Enabled", "read or set whether the control is enabled"),
    ("Focused", "read whether the control has the focus"),
    ("Gui", "read the window the control is in"),
    ("Hwnd", "give the handle of the control"),
    ("Name", "read or set the name of the control"),
    ("Text", "read or set the text of the control"),
    ("Type", "read the type of the control"),
    ("Value", "read or set the value of the control"),
    ("Visible", "read or set whether the control is visible"),
];

/// The names of the `Gui.List` class, with the thing each one does.
///
/// Measured on the binary of v2.0.31: the class of a list
/// of items owns three names, and the `Gui.ListBox`, the `Gui.ComboBox`,
/// the `Gui.DDL` and the `Gui.Tab` derive from it.
const LIST: &[(&str, &str)] = &[
    ("Add", "add an item to the list"),
    ("Choose", "choose an item of the list"),
    ("Delete", "delete an item of the list"),
];

/// The names of the `Gui.ListView` class, with the thing each one does.
///
/// Measured on the binary of v2.0.31: the class of a list
/// view owns eleven names, and the `Get`, the `GetChild`, the
/// `GetParent`, the `GetPrev` and the `GetSelection` are not among them
/// -- they are the names of the class of a tree, the `Gui.TreeView`.
const LISTVIEW: &[(&str, &str)] = &[
    ("Add", "add a row to the list"),
    ("Delete", "delete a row from the list"),
    ("DeleteCol", "delete a column of the list"),
    ("GetCount", "count the rows of the list"),
    ("GetNext", "read the row after a row of the list"),
    ("GetText", "read the text of a row of the list"),
    ("Insert", "insert a row into the list"),
    ("InsertCol", "insert a column into the list"),
    ("Modify", "change a row of the list"),
    ("ModifyCol", "change a column of the list"),
    ("SetImageList", "give the list an image list"),
];

/// The names of the `Gui.TreeView` class, with the thing each one does.
///
/// Measured on the binary of v2.0.31: the class of a tree
/// owns twelve names, and the `SetIcon`, the `SetParts` and the `SetText`
/// are not among them -- the binary has no such methods.
const TREEVIEW: &[(&str, &str)] = &[
    ("Add", "add an item to the tree"),
    ("Delete", "delete an item of the tree"),
    ("Get", "read an item of the tree"),
    ("GetChild", "read the child of an item of the tree"),
    ("GetCount", "count the items of the tree"),
    ("GetNext", "read the item after an item of the tree"),
    ("GetParent", "read the item above an item of the tree"),
    ("GetPrev", "read the item before an item of the tree"),
    ("GetSelection", "read the selected item of the tree"),
    ("GetText", "read the text of an item of the tree"),
    ("Modify", "change an item of the tree"),
    ("SetImageList", "give the tree an image list"),
];

/// The names of the `Gui.StatusBar` class, with the thing each one does.
///
/// Measured on the binary of v2.0.31: the class of a
/// status bar owns three names, and they are the ones of the text of it,
/// of the parts of it and of the icon of it.
const STATUSBAR: &[(&str, &str)] = &[
    ("SetIcon", "set the icon of the bar"),
    ("SetParts", "set the parts of the bar"),
    ("SetText", "set the text of a part of the bar"),
];

/// The names of the `Gui.Menu` class, with the thing each one does.
///
/// The script calls this class `Menu`, and the registry hands the module
/// the two names.
const MENU: &[(&str, &str)] = &[
    ("add", "add an item to the menu bar"),
    ("addstandard", "add the standard items to the menu bar"),
    ("check", "check an item of the menu bar"),
    (
        "clickcount",
        "read or set how many clicks an item of the menu bar takes",
    ),
    (
        "default",
        "read or set which item of the menu bar is the default",
    ),
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
    (
        "toggleenable",
        "toggle whether an item of the menu bar is enabled",
    ),
    ("uncheck", "uncheck an item of the menu bar"),
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
///
/// The `Gui.Menu` is not a class of the dialect: the binaries refuse the
/// name, and the tree carries the methods of the class of the menu bar
/// in the table of the class.
const CLASSES: &[Windows] = &[
    Windows {
        class: "gui",
        name: "Gui",
        surface: GUI,
        reason: "a window is a Windows window, and this desktop is a Wayland session without one",
    },
    Windows {
        class: "menu",
        name: "Menu",
        surface: MENU,
        reason: "a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one",
    },
    Windows {
        class: "gui.menu",
        name: "Menu",
        surface: MENU,
        reason: "a menu is the menu bar of a Windows window, and this desktop is a Wayland session without one",
    },
];

/// A class of the controls of a window.
///
/// The class of a control is a nested class of the class `Gui`, and the
/// binary carries twenty-five of them as values: `Type(Gui.Control)`
/// answers "Class" on the binary of the v2.0.19 and on the binary of the
/// v2.0.31, alike.
///
/// The base of each one and the names it owns are the ones of the binary,
/// and they were read out of it with the instrument beside
/// this table (`scripts/probe-gui-classes.py`).
pub(crate) struct Nested {
    /// The name the registry hands the module.
    pub(crate) class: &'static str,
    /// The name the script calls it.
    pub(crate) name: &'static str,
    /// The class it derives from.
    pub(crate) base: &'static str,
    /// The names the class owns that are methods.
    pub(crate) methods: &'static [(&'static str, &'static str)],
    /// The names the class owns that are properties.
    pub(crate) properties: &'static [(&'static str, &'static str)],
    /// Why it cannot work here.
    pub(crate) reason: &'static str,
}

/// The nested classes of the class `Gui`.
///
/// The reason is the reason of the whole family: a control is a control
/// of a Windows window, and this desktop is a Wayland session without
/// one.
const NESTED: &[Nested] = &[
    Nested {
        class: "gui.control",
        name: "Gui.Control",
        base: "object",
        methods: CONTROL_METHODS,
        properties: CONTROL_PROPERTIES,
        reason: "a control is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.list",
        name: "Gui.List",
        base: "gui.control",
        methods: LIST,
        properties: &[],
        reason: "a list of items is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.listbox",
        name: "Gui.ListBox",
        base: "gui.list",
        methods: &[],
        properties: &[],
        reason: "a list box is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.combobox",
        name: "Gui.ComboBox",
        base: "gui.list",
        methods: &[],
        properties: &[],
        reason: "a combo box is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.ddl",
        name: "Gui.DDL",
        base: "gui.list",
        methods: &[],
        properties: &[],
        reason: "a drop-down list is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.tab",
        name: "Gui.Tab",
        base: "gui.list",
        methods: &[("UseTab", "use the tab of the window the control is in")],
        properties: &[],
        reason: "a tab is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.listview",
        name: "Gui.ListView",
        base: "gui.control",
        methods: LISTVIEW,
        properties: &[],
        reason: "a list view is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.treeview",
        name: "Gui.TreeView",
        base: "gui.control",
        methods: TREEVIEW,
        properties: &[],
        reason: "a tree view is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.statusbar",
        name: "Gui.StatusBar",
        base: "gui.control",
        methods: STATUSBAR,
        properties: &[],
        reason: "a status bar is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.datetime",
        name: "Gui.DateTime",
        base: "gui.control",
        methods: &[("SetFormat", "set the format of the control")],
        properties: &[],
        reason: "a date and time is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.activex",
        name: "Gui.ActiveX",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "an ActiveX is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.button",
        name: "Gui.Button",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a button is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.checkbox",
        name: "Gui.CheckBox",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a check box is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.custom",
        name: "Gui.Custom",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a custom control is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.edit",
        name: "Gui.Edit",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "an edit is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.groupbox",
        name: "Gui.GroupBox",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a group box is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.hotkey",
        name: "Gui.Hotkey",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a hotkey is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.link",
        name: "Gui.Link",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a link is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.monthcal",
        name: "Gui.MonthCal",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a month calendar is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.pic",
        name: "Gui.Pic",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a picture is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.progress",
        name: "Gui.Progress",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a progress bar is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.radio",
        name: "Gui.Radio",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a radio button is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.slider",
        name: "Gui.Slider",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a slider is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.text",
        name: "Gui.Text",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "a text is a control of a Windows window, and this desktop is a Wayland session without one",
    },
    Nested {
        class: "gui.updown",
        name: "Gui.UpDown",
        base: "gui.control",
        methods: &[],
        properties: &[],
        reason: "an up-down is a control of a Windows window, and this desktop is a Wayland session without one",
    },
];

/// The names of the nested classes, in lower case.
///
/// The table of the names of the classes of the runtime is built with them:
/// the question "is this name a class of the runtime?" is asked in every
/// call of a built-in.
pub(crate) fn the_names_of_the_nested() -> impl Iterator<Item = &'static str> {
    NESTED.iter().map(|entry| entry.class)
}

/// The nested class of that name, when the name is one of them.
///
/// The name is the one the registry hands the module: `gui.control`.
pub(crate) fn the_nested(class: &str) -> Option<&'static Nested> {
    // The name is read without a copy of it: the question is asked for every
    // call of a built-in.
    NESTED
        .iter()
        .find(|entry| entry.class.eq_ignore_ascii_case(class))
}

/// The name of a nested class, the way v2 spells it.
///
/// The name has the class in front: `Gui.Control`, and not
/// `Gui.control`.
pub(crate) fn the_spelling_of(class: &str) -> Option<&'static str> {
    the_nested(class).map(|entry| entry.name)
}

/// The nested class of that name, when the name is the one that follows
/// the `Gui`.
///
/// The name of a nested class is a member of the class `Gui`, and the
/// member is the name of the class without the `Gui` in front: the
/// `Control` is the `Gui.Control`.
pub(crate) fn the_nested_of_the_window(name: &str) -> Option<&'static str> {
    let name = name.to_lowercase();
    NESTED
        .iter()
        .find(|entry| {
            entry
                .name
                .strip_prefix("Gui.")
                .map(|rest| rest.to_lowercase() == name)
                .unwrap_or(false)
        })
        .map(|entry| entry.class)
}

/// The nested classes, the way the script spells them.
///
/// The name of a nested class is a member of the class `Gui`, and a
/// script reads it as a class.
pub(crate) fn the_nested_names() -> Vec<&'static str> {
    NESTED.iter().map(|entry| entry.name).collect()
}

/// The names of a class of the windows.
///
/// The names are the ones the binary gives the class, and they are the
/// whole surface of it: a name that is not in them is not a method of
/// the class.
pub(crate) fn names(class: &str) -> Vec<&'static str> {
    let class = class.to_lowercase();
    if let Some(entry) = CLASSES.iter().find(|entry| entry.class == class) {
        return entry
            .surface
            .iter()
            .map(|(name, _)| *name)
            .collect();
    }
    if let Some(entry) = the_nested(&class) {
        return entry
            .methods
            .iter()
            .chain(entry.properties.iter())
            .map(|(name, _)| *name)
            .collect();
    }
    Vec::new()
}

/// The members that the prototype of a class owns, and the ones that it
/// inherits from its base.
///
/// Measured on the binary of v2.0.31: the classes of the
/// controls are a tree -- the `Gui.ListBox`, the `Gui.ComboBox`, the
/// `Gui.DDL` and the `Gui.Tab` derive from the `Gui.List`, and the
/// `Gui.List` and the rest derive from the `Gui.Control` -- and the
/// prototype of a class owns the members of it and the ones of its base.
pub(crate) fn the_members_with_the_base(class: &str) -> Vec<&'static str> {
    let mut names: Vec<&'static str> = Vec::new();
    let mut current = Some(class.to_lowercase());
    while let Some(class) = current {
        let Some(entry) = the_nested(&class) else {
            break;
        };
        for (name, _) in entry.methods.iter().chain(entry.properties.iter()) {
            if !names.iter().any(|other| other.eq_ignore_ascii_case(name)) {
                names.push(name);
            }
        }
        current = Some(entry.base.to_string());
    }
    names
}

/// The methods that the prototype of a class owns, and the ones that it
/// inherits from its base.
///
/// The properties of a class are not methods of it, and the members of
/// the base object are not among them: the runtime answers for those.
pub(crate) fn the_methods_with_the_base(class: &str) -> Vec<&'static str> {
    let mut names: Vec<&'static str> = Vec::new();
    let mut current = Some(class.to_lowercase());
    while let Some(class) = current {
        let Some(entry) = the_nested(&class) else {
            break;
        };
        for (name, _) in entry.methods {
            if !names.iter().any(|other| other.eq_ignore_ascii_case(name)) {
                names.push(name);
            }
        }
        current = Some(entry.base.to_string());
    }
    names
}

/// The object a class of the windows makes.
///
/// The object is real: a script may hold it, and the report says that it
/// is there. What it cannot do is the thing that fails.
pub(crate) fn construct(
    class: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    // A menu is made at once: the `Menu()` creates the menu of the
    // machine and the object holds the number that names it.
    if (class.eq_ignore_ascii_case("menu") || class.eq_ignore_ascii_case("gui.menu"))
        && rt.desktop().carries_the_gui()
    {
        let id = rt.desktop().menu_create().map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        })?;
        return Ok(crate::classes::object_with(
            class,
            alloc::vec![("menu id", Value::Int(id as i64))],
        ));
    }
    // A Gui keeps the script alive after its statements end, the way the
    // original AutoHotkey does.
    if class.eq_ignore_ascii_case("gui") {
        rt.the_script_asked_to_stay(true);
    }
    // The third argument of v2 is the object that receives the events of
    // the window, and v2 checks it before the options.
    if let Some(events) = arguments.get(2) {
        if !matches!(events, Value::Object(_)) {
            return Err(the_type_failure(
                4,
                "Gui.Prototype.__New",
                TypeName::Object,
                events,
                line,
            ));
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
                return Err(the_type_failure(
                    2,
                    "Gui.Prototype.__New",
                    TypeName::String,
                    other,
                    line,
                ));
            }
        }
    }
    Ok(crate::classes::object(class))
}

/// The options that the constructor of the `Gui` takes.
///
/// Measured on the binary of v2.0.19: the options are the
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
/// Measured on the binary of v2.0.19. A word that is a
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
                &Value::Text(name.to_string().into()),
                line,
            ));
        };
        if THE_OPTIONS_THAT_NAME_A_WINDOW.contains(&option) {
            // Measured on the binary of v2.0.31: a bare `Owner` carries no
            // owner and the window is made; an owner or a parent that names a
            // window of the machine is resolved, and the one that is not a
            // window is "Invalid or nonexistent owner or parent window."
            let value = &lower[option.len()..];
            let a_window_is_named = if option.eq_ignore_ascii_case("Owner") {
                !value.is_empty()
            } else {
                true
            };
            if a_window_is_named {
                return Err(RuntimeFailure::Detailed(
                    RuntimeError {
                        message: "Invalid or nonexistent owner or parent window.".to_string(),
                        line,
                    },
                    Detail {
                        what: "Gui.Prototype.__New".to_string(),
                        extra: word.to_string(),
                    },
                ));
            }
            continue;
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
        Detail {
            what: name.to_string(),
            extra: value.to_text(),
        },
    )
}

/// A method of a class of the windows.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    class: &str,
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The methods of a `Gui.ListView`: a row of cells, read and written.
    // Measured on the binary of v2.0.19. The number of the
    // control is the `Hwnd` property the object carries.
    if class.eq_ignore_ascii_case("gui.listview") {
        let number = super::the_number_of_a_control(this).unwrap_or(0);
        let words: Vec<String> = arguments.iter().map(|value| value.to_text()).collect();
        let outcome: Result<Value, String> = match method.to_lowercase().as_str() {
            // `Add(Options, Col1, Col2, ...)`: the options come first.
            "add" => rt
                .desktop()
                .gui_list_add(number, words.get(1..).unwrap_or(&[]))
                .map(Value::Int),
            "getcount" => rt.desktop().gui_list_count(number).map(Value::Int),
            "gettext" => {
                let row = words.first().and_then(|w| w.parse().ok()).unwrap_or(0);
                let col = words.get(1).and_then(|w| w.parse().ok()).unwrap_or(0);
                rt.desktop()
                    .gui_list_text(number, row, col)
                    .map(Value::text)
            }
            "getcontent" => {
                let selected = words
                    .first()
                    .map(|w| w.eq_ignore_ascii_case("selected"))
                    .unwrap_or(false);
                rt.desktop()
                    .gui_list_content(number, selected)
                    .map(Value::text)
            }
            "modify" => {
                let row = words.first().and_then(|w| w.parse().ok()).unwrap_or(0);
                rt.desktop()
                    .gui_list_modify(number, row, words.get(2..).unwrap_or(&[]))
                    .map(Value::Int)
            }
            "getnext" => {
                let start = words.first().and_then(|w| w.parse().ok()).unwrap_or(0);
                rt.desktop().gui_list_next(number, start).map(Value::Int)
            }
            _ => return Some(Err(refused(class, method, line)?)),
        };
        return Some(outcome.map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        }));
    }
    // The methods of a `Gui.TreeView`: items with a parent, walked by ids.
    // Measured on the binary of v2.0.19. `SetText`, `SetIcon`,
    // `SetImageList` and `SetParts` are in the extracted table but the binary
    // has no such methods -- the table was read at the wrong place -- so they
    // keep the refusal below.
    if class.eq_ignore_ascii_case("gui.treeview") {
        let number = super::the_number_of_a_control(this).unwrap_or(0);
        let words: Vec<String> = arguments.iter().map(|value| value.to_text()).collect();
        let number_at =
            |index: usize| -> i64 { words.get(index).and_then(|w| w.parse().ok()).unwrap_or(0) };
        let item = number_at(0);
        let outcome: Result<Value, String> = match method.to_lowercase().as_str() {
            // `Add(Text, Parent, Options)`
            "add" => rt
                .desktop()
                .gui_tree_add(
                    number,
                    words.first().map(String::as_str).unwrap_or(""),
                    number_at(1),
                    words.get(2).map(String::as_str).unwrap_or(""),
                )
                .map(Value::Int),
            "getcount" => rt.desktop().gui_tree_count(number).map(Value::Int),
            "gettext" => rt.desktop().gui_tree_text(number, item).map(Value::text),
            "getparent" | "getchild" | "getnext" | "getprev" => {
                let relation = &method.to_lowercase()[3..];
                rt.desktop()
                    .gui_tree_relation(number, item, relation)
                    .map(Value::Int)
            }
            "getselection" => rt.desktop().gui_tree_selection(number).map(Value::Int),
            "modify" => rt
                .desktop()
                .gui_tree_modify(number, item, words.get(1).map(String::as_str).unwrap_or(""))
                .map(Value::Int),
            "delete" => rt.desktop().gui_tree_delete(number, item).map(Value::Int),
            _ => return Some(Err(refused(class, method, line)?)),
        };
        return Some(outcome.map_err(|reason| {
            RuntimeFailure::Error(RuntimeError {
                message: reason,
                line,
            })
        }));
    }
    // A desktop that makes the windows of the script carries the family:
    // the methods of the `Gui` go to it, and the ones it does not answer
    // keep the refusal below.
    if class.eq_ignore_ascii_case("gui") && rt.desktop().carries_the_gui() {
        if let Some(outcome) = the_gui_method(method, arguments, rt, line) {
            return Some(outcome);
        }
    }
    if (class.eq_ignore_ascii_case("menu") || class.eq_ignore_ascii_case("gui.menu"))
        && rt.desktop().carries_the_gui()
    {
        if let Some(outcome) = the_menu_method(method, this, arguments, rt, line) {
            return Some(outcome);
        }
    }
    Some(Err(refused(class, method, line)?))
}

/// The number that names the menu of a `Menu` object.
fn the_menu_id(this: &Value) -> Option<u64> {
    if let Value::Object(object) = this {
        if let Some(property) = object.borrow().properties.get("menu id") {
            if let Value::Int(number) = property.value {
                return Some(number as u64);
            }
        }
    }
    None
}

/// The methods of the `Menu` that a desktop which makes windows carries.
///
/// `None` means the method is not one of them: the caller keeps the
/// refusal of the family.
fn the_menu_method(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let id = the_menu_id(this)?;
    let lower = method.to_lowercase();
    let fail = |reason: String| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    };
    let text_at = |index: usize| arguments.get(index).map(|v| v.to_text()).unwrap_or_default();
    match lower.as_str() {
        "add" | "insert" => {
            // `Add(ItemName, Callback, Options)`: the callback is a
            // function of the script.
            let name = text_at(0);
            let callback = match arguments.get(1) {
                Some(Value::Object(object)) => object
                    .borrow()
                    .value_of("name")
                    .map(|v| v.to_text())
                    .unwrap_or_default(),
                Some(value) => value.to_text(),
                None => String::new(),
            };
            Some(
                rt.desktop()
                    .menu_add(id, &name, &callback)
                    .map(|_| Value::Int(0))
                    .map_err(fail),
            )
        }
        "delete" => Some(
            rt.desktop()
                .menu_delete(id, &text_at(0))
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "rename" => Some(
            rt.desktop()
                .menu_rename(id, &text_at(0), &text_at(1))
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "enable" => Some(
            rt.desktop()
                .menu_enable(id, &text_at(0), true)
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "disable" => Some(
            rt.desktop()
                .menu_enable(id, &text_at(0), false)
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "check" => Some(
            rt.desktop()
                .menu_check(id, &text_at(0), true)
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "uncheck" => Some(
            rt.desktop()
                .menu_check(id, &text_at(0), false)
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        "togglecheck" => {
            let name = text_at(0);
            let outcome = (|| -> Result<Value, RuntimeFailure> {
                let checked = rt.desktop().menu_is_checked(id, &name).map_err(fail)?;
                rt.desktop()
                    .menu_check(id, &name, !checked)
                    .map(|_| Value::Int(0))
                    .map_err(fail)
            })();
            Some(outcome)
        }
        "toggleenable" => {
            let name = text_at(0);
            let outcome = (|| -> Result<Value, RuntimeFailure> {
                let enabled = rt.desktop().menu_is_enabled(id, &name).map_err(fail)?;
                rt.desktop()
                    .menu_enable(id, &name, !enabled)
                    .map(|_| Value::Int(0))
                    .map_err(fail)
            })();
            Some(outcome)
        }
        "show" => {
            let outcome = (|| -> Result<Value, RuntimeFailure> {
                let callback = rt.desktop().menu_show(id).map_err(fail)?;
                if let Some(name) = callback {
                    if !name.is_empty() {
                        rt.call_named(&name, &[], line, crate::runtime::Route::Written)?;
                    }
                }
                Ok(Value::Int(0))
            })();
            Some(outcome)
        }
        _ => None,
    }
}

/// The kind of control that an `Add` name spells, and its class.
///
/// The `AddText` is the `Text`, the `AddButton` is the `Button`, and so
/// on; the class is the nested class of the control that the language
/// gives it.
fn the_kind_of_an_add(method: &str) -> Option<(&'static str, &'static str)> {
    let answer = match method {
        "addtext" => ("Text", "gui.text"),
        "addbutton" => ("Button", "gui.button"),
        "addcheckbox" => ("CheckBox", "gui.checkbox"),
        "addradio" => ("Radio", "gui.radio"),
        "addgroupbox" => ("GroupBox", "gui.groupbox"),
        "addedit" => ("Edit", "gui.edit"),
        "addlistbox" => ("ListBox", "gui.listbox"),
        "addcombobox" => ("ComboBox", "gui.combobox"),
        "addddl" | "adddropdownlist" => ("DDL", "gui.ddl"),
        "addlistview" => ("ListView", "gui.listview"),
        "addtreeview" => ("TreeView", "gui.treeview"),
        "addstatusbar" => ("StatusBar", "gui.statusbar"),
        "adddatetime" => ("DateTime", "gui.datetime"),
        "addmonthcal" => ("MonthCal", "gui.monthcal"),
        "addhotkey" => ("Hotkey", "gui.hotkey"),
        "addlink" => ("Link", "gui.link"),
        "addtab" | "addtab2" | "addtab3" => ("Tab", "gui.tab"),
        "addpic" | "addpicture" => ("Picture", "gui.pic"),
        "addprogress" => ("Progress", "gui.progress"),
        "addslider" => ("Slider", "gui.slider"),
        "addupdown" => ("UpDown", "gui.updown"),
        "addactivex" => ("ActiveX", "gui.activex"),
        "addcustom" => ("Custom", "gui.custom"),
        _ => return None,
    };
    Some(answer)
}

/// The class of the control that a kind names.
fn the_class_of_a_kind(kind: &str) -> &'static str {
    match kind.to_lowercase().as_str() {
        "text" => "gui.text",
        "button" => "gui.button",
        "checkbox" => "gui.checkbox",
        "radio" => "gui.radio",
        "groupbox" => "gui.groupbox",
        "edit" => "gui.edit",
        "listbox" => "gui.listbox",
        "combobox" => "gui.combobox",
        "ddl" | "dropdownlist" => "gui.ddl",
        "listview" => "gui.listview",
        "treeview" => "gui.treeview",
        "statusbar" => "gui.statusbar",
        "datetime" => "gui.datetime",
        "monthcal" => "gui.monthcal",
        "hotkey" => "gui.hotkey",
        "link" => "gui.link",
        "tab" | "tab2" | "tab3" => "gui.tab",
        "pic" | "picture" => "gui.pic",
        "progress" => "gui.progress",
        "slider" => "gui.slider",
        "updown" => "gui.updown",
        "activex" => "gui.activex",
        "custom" => "gui.custom",
        _ => "gui.control",
    }
}

/// The window of the script is opened when it is needed.
///
/// The `Gui()` makes the object and the window is not made until a
/// member needs it: the `Add` and the `Show` open it. The desktop keeps
/// the one window, and a second open is not asked of it.
fn the_window_is_open(rt: &mut Runtime, line: u32) -> Result<(), RuntimeFailure> {
    if rt.desktop().gui_is_open() {
        return Ok(());
    }
    rt.desktop().gui_open("").map_err(|reason| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    })?;
    Ok(())
}

/// The methods of the `Gui` that a desktop which makes windows carries.
///
/// `None` means the method is not one of them: the caller keeps the
/// refusal of the family.
fn the_gui_method(
    method: &str,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let lower = method.to_lowercase();
    let fail = |reason: String| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    };
    // The `Add` and the `AddText` and the rest: a control is made and the
    // object of it is answered.
    if lower == "add" || the_kind_of_an_add(&lower).is_some() {
        let (kind, text) = if lower == "add" {
            // `Add(ControlType, Options, Text)`.
            let kind = arguments.first().map(|v| v.to_text()).unwrap_or_default();
            let text = arguments.get(2).map(|v| v.to_text()).unwrap_or_default();
            (kind, text)
        } else {
            // `AddText(Options, Text)`.
            let (kind, _) = the_kind_of_an_add(&lower).unwrap();
            let text = arguments.get(1).map(|v| v.to_text()).unwrap_or_default();
            (kind.to_string(), text)
        };
        let outcome = (|| -> Result<Value, RuntimeFailure> {
            the_window_is_open(rt, line)?;
            let (number, classnn) = rt
                .desktop()
                .gui_add(1, &kind, &text)
                .map_err(fail)?;
            let class = the_class_of_a_kind(&kind);
            Ok(crate::classes::object_with(
                class,
                alloc::vec![
                    ("hwnd", Value::Int(number as i64)),
                    ("classnn", Value::Text(classnn.into())),
                    ("type", Value::Text(kind.clone().into())),
                ],
            ))
        })();
        return Some(outcome);
    }
    match lower.as_str() {
        "show" => {
            let outcome = (|| -> Result<Value, RuntimeFailure> {
                the_window_is_open(rt, line)?;
                rt.desktop().gui_show(1).map_err(fail)?;
                Ok(Value::Int(0))
            })();
            Some(outcome)
        }
        "hide" => Some(rt.desktop().gui_hide().map(|_| Value::Int(0)).map_err(fail)),
        "destroy" => Some(
            rt.desktop()
                .gui_destroy(1)
                .map(|_| Value::Int(0))
                .map_err(fail),
        ),
        _ => None,
    }
}

/// A property of a class of the windows.
///
/// `None` means the name is not a property of this class: the runtime
/// then looks in the property table of the object.
pub(crate) fn property(
    class: &str,
    name: &str,
    _this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if class.eq_ignore_ascii_case("gui")
        && rt.desktop().carries_the_gui()
        && name.eq_ignore_ascii_case("title")
    {
        return Some(
            rt.desktop()
                .gui_title(1, None)
                .map(Value::text)
                .map_err(|reason| {
                    RuntimeFailure::Error(RuntimeError {
                        message: reason,
                        line,
                    })
                }),
        );
    }
    // As propriedades de um controle: o texto e o valor sao o conteudo,
    // e o enabled e o visible sao bandeiras de zero e um.
    if let Some(outcome) = the_control_property(class, name, _this, rt, line) {
        return Some(outcome);
    }
    // As propriedades do menu: o handle da maquina e o numero de itens.
    if (class.eq_ignore_ascii_case("menu") || class.eq_ignore_ascii_case("gui.menu"))
        && rt.desktop().carries_the_gui()
    {
        if let Some(id) = the_menu_id(_this) {
            let fail = |reason: String| {
                RuntimeFailure::Error(RuntimeError {
                    message: reason,
                    line,
                })
            };
            match name.to_lowercase().as_str() {
                "handle" => {
                    return Some(
                        rt.desktop().menu_handle(id).map(Value::Int).map_err(fail),
                    )
                }
                "count" => {
                    return Some(rt.desktop().menu_count(id).map(Value::Int).map_err(fail))
                }
                _ => {}
            }
        }
    }
    Some(Err(refused(class, name, line)?))
}

/// A property of a control that a desktop which makes windows carries.
///
/// `None` means the name is not one of them, or the desktop does not
/// make windows: the caller keeps the refusal of the family.
fn the_control_property(
    class: &str,
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !rt.desktop().carries_the_gui() || !class.to_lowercase().starts_with("gui.") {
        return None;
    }
    let number = super::the_number_of_a_control(this)?;
    let lower = name.to_lowercase();
    let fail = |reason: String| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    };
    match lower.as_str() {
        "text" | "value" => Some(
            rt.desktop()
                .gui_value(number, None)
                .map(Value::text)
                .map_err(fail),
        ),
        "enabled" | "visible" => Some(
            rt.desktop()
                .gui_flag(number, &lower, None)
                .map(Value::Int)
                .map_err(fail),
        ),
        _ => None,
    }
}

/// A property of a class of the windows that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    class: &str,
    name: &str,
    _this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    if class.eq_ignore_ascii_case("gui")
        && rt.desktop().carries_the_gui()
        && name.eq_ignore_ascii_case("title")
    {
        let outcome = (|| -> Result<(), RuntimeFailure> {
            the_window_is_open(rt, line)?;
            let text = value.to_text();
            rt.desktop()
                .gui_title(1, Some(&text))
                .map(|_| ())
                .map_err(|reason| {
                    RuntimeFailure::Error(RuntimeError {
                        message: reason,
                        line,
                    })
                })
        })();
        return Some(outcome);
    }
    if let Some(outcome) = the_control_set_property(class, name, _this, &value, rt, line) {
        return Some(outcome);
    }
    Some(Err(refused(class, name, line)?))
}

/// A property of a control that is written, on a desktop that makes
/// windows.
fn the_control_set_property(
    class: &str,
    name: &str,
    this: &Value,
    value: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    if !rt.desktop().carries_the_gui() || !class.to_lowercase().starts_with("gui.") {
        return None;
    }
    let number = super::the_number_of_a_control(this)?;
    let lower = name.to_lowercase();
    let fail = |reason: String| {
        RuntimeFailure::Error(RuntimeError {
            message: reason,
            line,
        })
    };
    match lower.as_str() {
        "text" | "value" => {
            let text = value.to_text();
            Some(
                rt.desktop()
                    .gui_value(number, Some(&text))
                    .map(|_| ())
                    .map_err(fail),
            )
        }
        "enabled" | "visible" => {
            let on = value.to_number().unwrap_or(0.0) != 0.0;
            Some(
                rt.desktop()
                    .gui_flag(number, &lower, Some(on))
                    .map(|_| ())
                    .map_err(fail),
            )
        }
        _ => None,
    }
}

/// The failure a class of the windows gives when it is asked for a name it
/// has.
///
/// `None` means the class has no such name: the runtime then says the
/// value has no such method.
fn refused(class: &str, name: &str, line: u32) -> Option<RuntimeFailure> {
    let class = class.to_lowercase();
    let lower = name.to_lowercase();
    let what = if let Some(entry) = CLASSES.iter().find(|entry| entry.class == class) {
        entry
            .surface
            .iter()
            .find(|entry| entry.0.to_lowercase() == lower)
            .map(|entry| entry.1)
    } else {
        let entry = the_nested(&class)?;
        entry
            .methods
            .iter()
            .chain(entry.properties.iter())
            .find(|entry| entry.0.to_lowercase() == lower)
            .map(|entry| entry.1)
    }?;
    let (spelled, reason) = if let Some(entry) = CLASSES.iter().find(|entry| entry.class == class)
    {
        (entry.name, entry.reason)
    } else {
        let entry = the_nested(&class)?;
        (entry.name, entry.reason)
    };
    Some(RuntimeFailure::Error(RuntimeError {
        message: format!("{} cannot {what}: {reason}.", spelled),
        line,
    }))
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec::Vec;

    use super::{CLASSES, GUI, LIST, LISTVIEW, MENU, NESTED, STATUSBAR, TREEVIEW};

    /// The surface of v2.0.19, read out of the binary.
    const SURFACE: &str = include_str!("../../reference/ahk-v2.0.32.surface.tsv");

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
            ("gui.list", LIST),
            ("gui.listview", LISTVIEW),
            ("gui.menu", MENU),
            ("gui.statusbar", STATUSBAR),
            ("gui.treeview", TREEVIEW),
        ] {
            let carried: Vec<String> = table
                .iter()
                .map(|(name, _)| name.to_lowercase())
                .collect();
            for name in names_of(class) {
                assert!(
                    carried.contains(&name),
                    "{class} does not carry {name}"
                );
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

    /// Every name of the class of a control is carried, and nothing else.
    ///
    /// The class of a control is a nested class of the class `Gui`, and
    /// the table of each one of them was read out of the binary.
    #[test]
    fn every_name_of_a_class_of_a_control_is_carried() {
        for entry in NESTED {
            let carried: Vec<String> = entry
                .methods
                .iter()
                .chain(entry.properties.iter())
                .map(|(name, _)| name.to_lowercase())
                .collect();
            for name in names_of(entry.class) {
                assert!(
                    carried.contains(&name),
                    "{} does not carry {name}",
                    entry.name
                );
            }
            for name in &carried {
                assert!(
                    names_of(entry.class).contains(name),
                    "{} carries {name}, and the binary does not",
                    entry.name
                );
            }
        }
    }

    /// The class of a control owns the names of its base as well.
    ///
    /// Measured on the binary of v2.0.31: the `Gui.ListBox`
    /// derives from the `Gui.List`, and the `Gui.List` from the
    /// `Gui.Control`, and the prototype of a class owns the members of it
    /// and the ones of its base.
    #[test]
    fn a_class_of_a_control_derives_from_its_base() {
        for entry in NESTED {
            let base = super::the_nested(entry.base);
            if entry.base == "object" {
                assert!(base.is_none(), "{} has no base class", entry.name);
                continue;
            }
            let base = base.unwrap_or_else(|| panic!("{} has no base", entry.name));
            assert_eq!(base.class, entry.base);
            let members = super::the_members_with_the_base(entry.class);
            for (name, _) in base.methods.iter().chain(base.properties.iter()) {
                assert!(
                    members.iter().any(|member| member == name),
                    "{} does not have {name} of its base",
                    entry.name
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
            assert!(
                entry.reason.contains("Wayland"),
                "{} says no reason",
                entry.name
            );
        }
    }
}
