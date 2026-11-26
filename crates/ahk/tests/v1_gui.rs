//! The windows of the v1, against a fake desktop.
//!
//! The v1 names its windows, and the commands that work on a window of
//! the script carry its name: the `Gui, Name:` and the `GuiControl,
//! Name:`. The cases of this file are about the answers of these
//! commands: the text of a control, the name of the variable of it, the
//! handle of it, the place of it, the control that has the focus and the
//! state of it.
//!
//! Every case of this file was measured on the binary of AutoHotkey
//! v1.1.37.02, and the probes are in
//! the probe of the measurement, in `gvC2.ahk`,
//! in the probes of the measurement,
//! `gvD5.ahk`, in `gvE1.ahk`, in `gvE2.ahk`, in `gvE3.ahk`, in `gvE4.ahk`,
//! in `gvE5.ahk`, in `gvE6.ahk` and in `gvE7.ahk`, and the answers of
//! them are in the files of the same name with the extension `.txt`.

use ahk::desktop::{
    TheCoordinate, ThePlaceOfTheV1, TheSize, TheStyleOfTheV1,
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::Duration;

/// The turn of the tests: the settings of the script live in a static, and
/// the runtime writes the defaults of its dialect when it is made. Two
/// cases that run a script cannot run at the same time, or the one that
/// reads a setting reads the one that the other wrote.
static THE_TURN: Mutex<()> = Mutex::new(());

/// A desktop that keeps the windows of the v1 in memory.
///
/// The windows of the v1 are named, and each one of them has the controls
/// that the script put in it. The controls carry the class of the
/// machine, the text of them and the name of the variable that they are
/// bound to.
#[derive(Default)]
struct Fake {
    /// The windows of the v1, by the name that the script gave them.
    guis: RefCell<BTreeMap<String, TheWindowOfTheV1>>,
    /// What the machine was asked, in the order of the questions.
    calls: RefCell<Vec<String>>,
    /// The control of the window that has the focus.
    focused: RefCell<Option<(String, u64)>>,
}

/// A window of the v1 of the fake.
#[derive(Default)]
struct TheWindowOfTheV1 {
    /// Whether the window is open.
    open: bool,
    /// The title of the window.
    title: String,
    /// Whether the window is seen.
    shown: bool,
    /// The controls of it, in the order that they were made.
    controls: Vec<TheControlOfTheFake>,
}

/// A control of a window of the v1 of the fake.
#[derive(Default)]
struct TheControlOfTheFake {
    /// The class of the control, the name of the machine.
    classnn: String,
    /// The text that the control carries.
    text: String,
    /// The name of the variable that the control is bound to.
    name: String,
    /// Whether the control is enabled.
    enabled: bool,
    /// Whether the control is seen.
    visible: bool,
    /// The place of the control and the size of it.
    place: (i64, i64, i64, i64),
    /// The items of a combo box or of a list box.
    items: Vec<String>,
    /// The item of a combo box or of a list box that is chosen.
    chosen: usize,
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Sound for Fake {}
impl Dialogs for Fake {}
impl Com for Fake {}
impl Native for Fake {}
impl Timers for Fake {}
impl Hotkeys for Fake {}
impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Err("this fake has no keyboard".into())
    }
}
impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Err("this fake has no pointer".into())
    }
}
impl Processes for Fake {
    fn run(&mut self, _command: &str, _working_dir: &str) -> Result<(i32, String, String), String> {
        Err("this fake has no processes".into())
    }
}
impl Screen for Fake {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Err("this fake has no screen".into())
    }

    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this fake has no screen".into())
    }
}
impl Network for Fake {}
impl Clipboard for Fake {}
impl Files for Fake {}
impl Machine for Fake {
    fn os_version(&mut self) -> Result<String, String> {
        Ok("10.0.26100".to_string())
    }
}

impl Windows for Fake {
    // -- the windows of the v1 ----------------------------------------------------

    fn v1_gui_windows(&mut self) -> Result<Vec<String>, String> {
        Ok(self.guis.borrow().keys().cloned().collect())
    }

    fn v1_gui_open(&mut self, name: &str, title: &str) -> Result<String, String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_open {name} {title}"));
        let mut guis = self.guis.borrow_mut();
        let window = guis.entry(name.to_string()).or_default();
        window.open = true;
        if !title.is_empty() {
            window.title = title.to_string();
        }
        Ok(name.to_string())
    }

    fn v1_gui_is_open(&mut self, name: &str) -> Result<bool, String> {
        Ok(self
            .guis
            .borrow()
            .get(name)
            .map(|window| window.open)
            .unwrap_or(false))
    }

    fn v1_gui_number_of(&mut self, name: &str) -> Result<i64, String> {
        let guis = self.guis.borrow();
        let names: Vec<&String> = guis.keys().collect();
        let place = names
            .iter()
            .position(|other| other.as_str() == name)
            .unwrap_or(0);
        Ok(place as i64 + 1)
    }

    fn v1_gui_add(
        &mut self,
        window: &str,
        kind: &str,
        variable: &str,
        text: &str,
        _style: TheStyleOfTheV1,
    ) -> Result<(u64, String), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_add {window} {kind} {variable} {text}"));
        let mut guis = self.guis.borrow_mut();
        let window = guis.entry(window.to_string()).or_default();
        let number = window.controls.len() as u64 + 1;
        let classnn = format!("{}{}", the_class_of_a_kind(kind), number);
        let items: Vec<String> = text.split('|').map(|item| item.to_string()).collect();
        window.controls.push(TheControlOfTheFake {
            classnn: classnn.clone(),
            text: text.to_string(),
            name: variable.to_string(),
            enabled: true,
            visible: true,
            place: (10, 20, 30, 40),
            items,
            chosen: 0,
        });
        Ok((number, classnn))
    }

    fn v1_gui_place(
        &mut self,
        window: &str,
        number: u64,
        place: ThePlaceOfTheV1,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_place {window} {number} {place:?}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        let (old_x, old_y, old_width, old_height) = control.place;
        let x = match place.x {
            Some(TheCoordinate::Absolute(value)) => value,
            _ => old_x,
        };
        let y = match place.y {
            Some(TheCoordinate::Absolute(value)) => value,
            _ => old_y,
        };
        let width = match place.width {
            Some(TheSize::Absolute(value)) => value,
            _ => old_width,
        };
        let height = match place.height {
            Some(TheSize::Absolute(value)) => value,
            _ => old_height,
        };
        control.place = (x, y, width, height);
        Ok(())
    }

    fn v1_gui_window_place(
        &mut self,
        window: &str,
        x: Option<i64>,
        y: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        centered_x: bool,
        centered_y: bool,
    ) -> Result<(), String> {
        self.calls.borrow_mut().push(format!(
            "gui_window_place {window} {x:?} {y:?} {width:?} {height:?} {centered_x} {centered_y}"
        ));
        Ok(())
    }

    fn v1_gui_number_named(&mut self, window: &str, name: &str) -> Result<i64, String> {
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        for (index, control) in window.controls.iter().enumerate() {
            if !control.name.is_empty() && control.name.eq_ignore_ascii_case(name) {
                return Ok(index as i64 + 1);
            }
        }
        for (index, control) in window.controls.iter().enumerate() {
            if control.classnn.eq_ignore_ascii_case(name) {
                return Ok(index as i64 + 1);
            }
        }
        // The text of the control is the way the v1 names it when it has
        // no variable and no class that matches.
        for (index, control) in window.controls.iter().enumerate() {
            if control.text.eq_ignore_ascii_case(name) {
                return Ok(index as i64 + 1);
            }
        }
        Err(format!("there is no control named {name}"))
    }

    fn v1_gui_classnn(&mut self, window: &str, number: u64) -> Result<String, String> {
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window
            .controls
            .get(number.max(1) as usize - 1)
            .map(|control| control.classnn.clone())
            .ok_or_else(|| format!("there is no control with the number {number}"))
    }

    fn v1_gui_control_name(&mut self, window: &str, number: u64) -> Result<String, String> {
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window
            .controls
            .get(number.max(1) as usize - 1)
            .map(|control| control.name.clone())
            .ok_or_else(|| format!("there is no control with the number {number}"))
    }

    fn v1_gui_handle(&mut self, _window: &str, number: u64) -> Result<i64, String> {
        Ok(1000 + number as i64)
    }

    fn v1_gui_window_handle(&mut self, window: &str) -> Result<i64, String> {
        Ok(2000 + window.len() as i64)
    }

    fn v1_gui_value(
        &mut self,
        window: &str,
        number: u64,
        setting: Option<&str>,
    ) -> Result<String, String> {
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match setting {
            Some(text) => {
                control.text = text.to_string();
                Ok(text.to_string())
            }
            None => Ok(control.text.clone()),
        }
    }

    fn v1_gui_caption(&mut self, window: &str, number: u64) -> Result<String, String> {
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(control.text.clone())
    }

    fn v1_gui_flag(
        &mut self,
        window: &str,
        number: u64,
        what: &str,
        setting: Option<bool>,
    ) -> Result<i64, String> {
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match what {
            "enabled" => {
                if let Some(on) = setting {
                    control.enabled = on;
                }
                Ok(control.enabled as i64)
            }
            "visible" => {
                if let Some(on) = setting {
                    control.visible = on;
                }
                Ok(control.visible as i64)
            }
            other => Err(format!("the tree does not carry the {other} of a control.")),
        }
    }

    fn v1_gui_visible(&mut self, window: &str, number: u64, on: bool) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_visibility {window} {number} {on}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        control.visible = on;
        Ok(())
    }

    fn v1_gui_focus(&mut self, window: &str, number: u64) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_focus {window} {number}"));
        *self.focused.borrow_mut() = Some((window.to_string(), number));
        Ok(())
    }

    fn v1_gui_focused(&mut self, window: &str) -> Result<String, String> {
        let focused = self.focused.borrow().clone();
        let Some((name, number)) = focused else {
            return Ok(String::new());
        };
        if name != window {
            return Ok(String::new());
        }
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        Ok(window
            .controls
            .get(number.max(1) as usize - 1)
            .map(|control| control.classnn.clone())
            .unwrap_or_default())
    }

    fn v1_gui_control_pos(
        &mut self,
        window: &str,
        number: u64,
    ) -> Result<(i64, i64, i64, i64), String> {
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(control.place)
    }

    fn v1_gui_show(&mut self, window: &str, title: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_show {window} {title}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window.shown = true;
        if !title.is_empty() {
            window.title = title.to_string();
        }
        Ok(())
    }

    fn v1_gui_show_without_activating(
        &mut self,
        window: &str,
        title: &str,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_show_without_activating {window} {title}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window.shown = true;
        if !title.is_empty() {
            window.title = title.to_string();
        }
        Ok(())
    }

    fn v1_gui_auto_size(&mut self, window: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_auto_size {window}"));
        Ok(())
    }

    fn v1_gui_hide(&mut self, window: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_hide {window}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window.shown = false;
        Ok(())
    }

    fn v1_gui_destroy(&mut self, window: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_destroy {window}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        window.open = false;
        Ok(())
    }

    fn v1_gui_title(&mut self, window: &str, setting: Option<&str>) -> Result<String, String> {
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        match setting {
            Some(title) => {
                window.title = title.to_string();
                Ok(title.to_string())
            }
            None => Ok(window.title.clone()),
        }
    }

    fn v1_gui_option(
        &mut self,
        window: &str,
        name: &str,
        value: Option<&str>,
        on: bool,
    ) -> Result<(), String> {
        self.calls.borrow_mut().push(format!(
            "gui_option {window} {name} {on} {}",
            value.unwrap_or_default()
        ));
        Ok(())
    }

    fn v1_gui_style(
        &mut self,
        window: &str,
        extended: bool,
        style: u32,
        on: bool,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_style {window} {extended} {style} {on}"));
        Ok(())
    }

    fn v1_gui_owner(
        &mut self,
        window: &str,
        owner: Option<&str>,
        parent: bool,
    ) -> Result<(), String> {
        self.calls.borrow_mut().push(format!(
            "gui_owner {window} {} {parent}",
            owner.unwrap_or_default()
        ));
        Ok(())
    }

    fn v1_gui_choose(
        &mut self,
        window: &str,
        number: u64,
        value: &str,
        by_text: bool,
    ) -> Result<i64, String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_choose {window} {number} {value} {by_text}"));
        let mut guis = self.guis.borrow_mut();
        let Some(window) = guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = window.controls.get_mut(number.max(1) as usize - 1) else {
            return Err(format!("there is no control with the number {number}"));
        };
        // The number of the item is counted from one, and the text of it
        // is the leading part of the text of the item.
        let index = if by_text {
            let wanted = value.to_ascii_lowercase();
            control
                .items
                .iter()
                .position(|item| item.to_ascii_lowercase().starts_with(&wanted))
        } else {
            value.trim().parse::<usize>().ok().filter(|number| *number > 0).map(|number| number - 1)
        };
        let Some(index) = index else {
            return Ok(-1);
        };
        control.chosen = index;
        control.text = control.items[index].clone();
        Ok(index as i64 + 1)
    }

    fn v1_gui_tab(&mut self, window: &str, tab: i64) -> Result<(), String> {
        self.calls.borrow_mut().push(format!("gui_tab {window} {tab}"));
        Ok(())
    }

    fn v1_gui_name_of_handle(&mut self, handle: i64) -> Result<String, String> {
        let guis = self.guis.borrow();
        for name in guis.keys() {
            if 2000 + name.len() as i64 == handle {
                return Ok(name.clone());
            }
        }
        Err(format!("there is no window with the handle {handle}"))
    }

    fn v1_gui_use_window(&mut self, window: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_use_window {window}"));
        Ok(())
    }

    fn v1_gui_color(&mut self, window: &str, color: &str) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_color {window} {color}"));
        Ok(())
    }

    fn v1_gui_margin(&mut self, window: &str, x: i64, y: i64) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_margin {window} {x} {y}"));
        Ok(())
    }

    fn v1_gui_font(
        &mut self,
        window: &str,
        options: &str,
        name: &str,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_font {window} {options} {name}"));
        Ok(())
    }

    fn v1_gui_menu_bar(
        &mut self,
        window: &str,
        _name: &str,
        json: &str,
        _menu: u64,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_menu_bar {window} {json}"));
        Ok(())
    }

    fn v1_gui_submit(&mut self, window: &str) -> Result<Vec<(String, String)>, String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_submit {window}"));
        let guis = self.guis.borrow();
        let Some(window) = guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        Ok(window
            .controls
            .iter()
            .filter(|control| !control.name.is_empty())
            .map(|control| (control.name.clone(), control.text.clone()))
            .collect())
    }

    // -- the list view and the tree view of the v1 --------------------------------

    fn v1_lv_add(&mut self, number: u64, _options: &str, cells: &[String]) -> Result<i64, String> {
        self.calls
            .borrow_mut()
            .push(format!("lv_add {number} {}", cells.join("|")));
        Ok(1)
    }

    fn v1_lv_count(&mut self, number: u64, _mode: &str) -> Result<i64, String> {
        self.calls
            .borrow_mut()
            .push(format!("lv_count {number}"));
        Ok(0)
    }

    fn v1_tv_add(
        &mut self,
        number: u64,
        text: &str,
        _parent: i64,
        _options: &str,
    ) -> Result<i64, String> {
        self.calls
            .borrow_mut()
            .push(format!("tv_add {number} {text}"));
        Ok(1)
    }

    fn v1_tv_count(&mut self, number: u64) -> Result<i64, String> {
        self.calls
            .borrow_mut()
            .push(format!("tv_count {number}"));
        Ok(0)
    }
}

/// The class of the machine of a kind of control, the way the fake names
/// it: the "ClassNN" of the v1 is the class of the machine and the number
/// of the control in the class of it.
fn the_class_of_a_kind(kind: &str) -> String {
    match kind.trim().to_ascii_lowercase().as_str() {
        "button" | "checkbox" | "radio" | "groupbox" => "Button".to_string(),
        "text" | "picture" => "Static".to_string(),
        "edit" => "Edit".to_string(),
        "listbox" => "ListBox".to_string(),
        "combobox" | "ddl" | "dropdownlist" => "ComboBox".to_string(),
        "listview" => "SysListView32".to_string(),
        "treeview" => "SysTreeView32".to_string(),
        "tab" => "SysTabControl32".to_string(),
        other => {
            let mut letters = other.chars();
            match letters.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + letters.as_str(),
                None => String::new(),
            }
        }
    }
}

/// Run a script of the v1 and answer with the report.
fn run(script: &str) -> Report {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake::default();
    Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements)
}

/// Run a script of the v1 and answer with the report and with what the
/// machine was asked.
fn run_and_the_calls(script: &str) -> (Report, Vec<String>) {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake::default();
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    let calls = desktop.calls.into_inner();
    (report, calls)
}

// -- the name of the variable of the Hwnd option ---------------------------------

/// The `Hwnd` option of the `Gui, Add` puts the handle of the control
/// into the variable of the script, and the name of the variable keeps
/// the case that the script wrote.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVwin:
/// Add, Text, x0 y0 w1 h1 BackgroundTrans vPicOnGui1 hwndhPic0, Previous
/// image` writes the handle into `hPic0`, and the report of the names of
/// the script shows the case that the script wrote.
#[test]
fn the_hwnd_option_of_the_gui_add_writes_the_handle_into_the_variable_of_its_name() {
    let report = run("Gui, PVwin: Add, Text, hwndhPic0, Previous image");
    assert!(report.ok, "{:?}", report.error);
    let handle = report.variables.get("hpic0").cloned().unwrap_or(Value::Text("".into()));
    let text = handle.to_text();
    assert!(
        text.starts_with("0x"),
        "the handle of the control is {text}, and the v1 writes it in hexadecimal"
    );
}

/// The `Hwnd` option of the `Gui, Add` accepts the sign in front of the
/// name of the option.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVwin:
/// Add, Edit, vMyEdit2 +hwndMyEdit2Hwnd` writes the handle into
/// `MyEdit2Hwnd`. The CHM of the v1.1.37.02 says of the options of a
/// control: "Note: In the absence of a preceding sign, a plus sign is
/// assumed".
#[test]
fn the_hwnd_option_of_the_gui_add_accepts_the_sign_in_front_of_it() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit2 +hwndMyEdit2Hwnd, Hello");
    assert!(report.ok, "{:?}", report.error);
    let handle = report
        .variables
        .get("myedit2hwnd")
        .cloned()
        .unwrap_or(Value::Text("".into()));
    let text = handle.to_text();
    assert!(
        text.starts_with("0x"),
        "the handle of the control is {text}, and the v1 writes it in hexadecimal"
    );
}

/// The variable of the `Hwnd` option of the `Gui, Add` is the one of the
/// scope of the caller.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVa:
/// Add, Edit, HwndFoo1` of a function that reads `Foo1` after it leaves
/// the handle in the local of the function, and the global of the script
/// has no `Foo1` at all. The CHM of the v1.1.37.02 says of the option:
/// "When within a function, MyEditHwnd is treated as a function dynamic
/// variable."
#[test]
fn the_variable_of_the_hwnd_option_of_the_gui_add_is_the_one_of_the_caller() {
    let report = run(
        "Probe() {\n\
         Gui, PVa: Add, Edit, HwndFoo1\n\
         x := Foo1\n\
         }\n\
         Probe()\n\
         y := 1",
    );
    assert!(report.ok, "{:?}", report.error);
    // The handle is in the local of the function, and the global of the
    // script has no `Foo1`.
    assert!(
        !report.variables.contains_key("foo1"),
        "{:?}",
        report.variables
    );
}

/// The variable of the `Hwnd` option of the `Gui, Add` is the global of
/// the script when there is one of that name.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVa:
/// Add, Edit, HwndgvFoo3` of a function leaves the handle in the global
/// `gvFoo3` of the script.
#[test]
fn the_variable_of_the_hwnd_option_is_the_global_when_there_is_one() {
    let report = run(
        "gvFoo3 := 0\n\
         Probe() {\n\
         Gui, PVa: Add, Edit, HwndgvFoo3\n\
         }\n\
         Probe()",
    );
    assert!(report.ok, "{:?}", report.error);
    let handle = report.variables.get("gvfoo3").cloned().unwrap_or(Value::Text("".into()));
    let text = handle.to_text();
    assert!(
        text.starts_with("0x"),
        "the handle of the control is {text}, and the v1 writes it in hexadecimal"
    );
}

/// The first window that the script makes is the window of the thread.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, MyEdit` of a script that made only the window `PVwin`
/// answers the text of the control of `PVwin`, and the CHM of the
/// v1.1.37.02 says of the windows of a script: "Non-GUI threads use 1 as
/// their default", the one being the first window that the script made.
#[test]
fn the_first_window_that_the_script_makes_is_the_window_of_the_thread() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit, Hello\nGuiControlGet, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("myedit"), Some(&Value::Text("Hello".into())));
}

/// The `+hwnd` option of a window puts the handle of the window into the
/// variable of the script, and the name of the variable keeps the case
/// that the script wrote.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVg:
/// +hwndPVhwndCase` writes the handle into `PVhwndCase`.
#[test]
fn the_hwnd_option_of_a_window_writes_the_handle_into_the_variable_of_its_name() {
    let report = run("Gui, PVg: +hwndPVhwndCase");
    assert!(report.ok, "{:?}", report.error);
    let handle = report
        .variables
        .get("pvhwndcase")
        .cloned()
        .unwrap_or(Value::Text("".into()));
    let text = handle.to_text();
    assert!(
        text.starts_with("0x"),
        "the handle of the window is {text}, and the v1 writes it in hexadecimal"
    );
}

// -- the GuiControlGet of a named window -----------------------------------------

/// The `GuiControlGet` of a named window answers the text of the control.
///
/// Measured on the binary of v1.1.37.02: the `GuiControlGet,
/// out, PVwin:, MyEdit` answers the text of the control.
#[test]
fn the_gui_control_get_of_a_named_window_answers_the_text_of_the_control() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit, Hello\nGuiControlGet, out, PVwin:, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("Hello".into())));
}

/// The `GuiControlGet` with the "Name" answers the name of the variable of
/// the control.
///
/// Measured on the binary of v1.1.37.02: the `GuiControlGet,
/// out, PVwin:Name, MyEdit` answers "MyEdit", and the answer is the
/// empty text when the control has no variable of its own.
#[test]
fn the_gui_control_get_with_the_name_answers_the_variable_of_the_control() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit\nGuiControlGet, out, PVwin:Name, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("MyEdit".into())));
}

/// The `GuiControlGet` with the "Hwnd" answers the handle of the control
/// as a text with the "0x" in front of it.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, out, PVwin:Hwnd, MyEdit` answers "0x9690728".
#[test]
fn the_gui_control_get_with_the_hwnd_answers_the_handle_with_the_zero_x() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit\nGuiControlGet, out, PVwin:Hwnd, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    let Some(Value::Text(answer)) = report.variables.get("out") else {
        panic!("the handle of the control is not a text");
    };
    assert!(answer.starts_with("0x"), "{answer}");
    assert!(answer[2..].chars().all(|one| one.is_ascii_hexdigit()), "{answer}");
}

/// The `GuiControlGet` with the "Pos" writes four variables: the name of
/// the caller with the "X", the "Y", the "W" and the "H" at the end.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, out, PVwin:Pos, MyEdit` writes `outX`, `outY`, `outW`
/// and `outH`.
#[test]
fn the_gui_control_get_with_the_pos_writes_four_variables() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit\nGuiControlGet, out, PVwin:Pos, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("outx"), Some(&Value::Int(10)));
    assert_eq!(report.variables.get("outy"), Some(&Value::Int(20)));
    assert_eq!(report.variables.get("outw"), Some(&Value::Int(30)));
    assert_eq!(report.variables.get("outh"), Some(&Value::Int(40)));
}

/// The `GuiControlGet` with the "Focus" answers the class of the control
/// that has the focus.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, out, PVwin:Focus` answers "Edit1" after the
/// `GuiControl, PVwin: Focus, MyEdit`.
#[test]
fn the_gui_control_get_with_the_focus_answers_the_class_of_the_control() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControl, PVwin: Focus, MyEdit\n\
         GuiControlGet, out, PVwin:Focus",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("Edit1".into())));
}

/// The `GuiControlGet` with the "FocusV" answers the name of the variable
/// of the control that has the focus.
///
/// Measured on the binary of v1.1.37.02: the answer is
/// "MyEdit" when the control has a variable of its own, and the text of
/// the control when it has none: the CHM of the v1.1.37.02 says of the
/// sub-command: "If that control lacks an associated variable, the first
/// 63 characters of the control's text/caption is retrieved instead".
#[test]
fn the_gui_control_get_with_the_focusv_answers_the_variable_of_the_control() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControl, PVwin: Focus, MyEdit\n\
         GuiControlGet, out, PVwin:FocusV",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("MyEdit".into())));
}

/// The `GuiControlGet` with the "FocusV" answers the text of the control
/// that has the focus when the control has no variable of its own.
///
/// Measured on the binary of v1.1.37.02: the button
/// "Press me" without a variable answers "Press me".
#[test]
fn the_gui_control_get_with_the_focusv_answers_the_text_of_the_control() {
    let report = run(
        "Gui, PVwin: Add, Button, , Press me\n\
         GuiControl, PVwin: Focus, Press me\n\
         GuiControlGet, out, PVwin:FocusV",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("Press me".into())));
}

/// The `GuiControlGet` with the "Enabled" and with the "Visible" answers
/// one or zero.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControl, PVwin: Disable, MyEdit` leaves the "Enabled" at zero, and
/// the `GuiControl, PVwin: Hide, MyEdit` leaves the "Visible" at zero.
#[test]
fn the_gui_control_get_answers_the_state_of_the_control() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControlGet, out, PVwin:Enabled, MyEdit\n\
         GuiControlGet, out2, PVwin:Visible, MyEdit\n\
         GuiControl, PVwin: Disable, MyEdit\n\
         GuiControl, PVwin: Hide, MyEdit\n\
         GuiControlGet, out3, PVwin:Enabled, MyEdit\n\
         GuiControlGet, out4, PVwin:Visible, MyEdit",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("out2"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("out3"), Some(&Value::Int(0)));
    assert_eq!(report.variables.get("out4"), Some(&Value::Int(0)));
}

/// The `ControlID` that is left out is the name of the variable of the
/// caller.
///
/// Measured on the binary of v1.1.37.02: the
/// `GuiControlGet, MyEdit` is the same as the `GuiControlGet, MyEdit,,
/// MyEdit`. The CHM of the v1.1.37.02 says: "If blank or omitted, it
/// behaves as though the name of the output variable was specified."
#[test]
fn the_control_id_that_is_left_out_is_the_name_of_the_variable_of_the_caller() {
    let report = run("Gui, PVwin: Add, Edit, vMyEdit, Hello\nGuiControlGet, MyEdit");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("myedit"), Some(&Value::Text("Hello".into())));
}

// -- the GuiControl of a named window --------------------------------------------

/// The `GuiControl` of a named window writes the text of the control.
///
/// Measured on the binary of v1.1.37.02: the `GuiControl,
/// PVwin: Text, MyEdit, Hello there` is read back by the `GuiControlGet,
/// out, PVwin:, MyEdit` as "Hello there".
#[test]
fn the_gui_control_of_a_named_window_writes_the_text_of_the_control() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControl, PVwin: Text, MyEdit, Hello there\n\
         GuiControlGet, out, PVwin:, MyEdit",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("out"),
        Some(&Value::Text("Hello there".into()))
    );
}

/// The `Move` of the `GuiControl` moves the control and changes its size.
///
/// Measured on the binary of v1.1.37.02: the `GuiControl,
/// PVwin: Move, MyEdit, x10 y20 w30 h40` is read back by the
/// `GuiControlGet, out, PVwin:Pos, MyEdit` as the place of ten, twenty,
/// thirty and forty.
#[test]
fn the_move_of_the_gui_control_moves_the_control() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControl, PVwin: Move, MyEdit, x10 y20 w30 h40\n\
         GuiControlGet, out, PVwin:Pos, MyEdit",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("outx"), Some(&Value::Int(10)));
    assert_eq!(report.variables.get("outy"), Some(&Value::Int(20)));
    assert_eq!(report.variables.get("outw"), Some(&Value::Int(30)));
    assert_eq!(report.variables.get("outh"), Some(&Value::Int(40)));
}

/// The `Choose` and the `ChooseString` of the `GuiControl` choose an item
/// of a control.
///
/// Measured on the binary of v1.1.37.02: the `GuiControl,
/// PVwin: Choose, MyList, 2` is read back as "Two", and the `GuiControl,
/// PVwin: ChooseString, MyList, Thr` as "Three".
#[test]
fn the_choose_of_the_gui_control_chooses_an_item() {
    let report = run(
        "Gui, PVwin: Add, DropDownList, vMyList, One|Two|Three\n\
         GuiControl, PVwin: Choose, MyList, 2\n\
         GuiControlGet, out, PVwin:, MyList\n\
         GuiControl, PVwin: ChooseString, MyList, Thr\n\
         GuiControlGet, out2, PVwin:, MyList",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("Two".into())));
    assert_eq!(report.variables.get("out2"), Some(&Value::Text("Three".into())));
}

/// The sub-commands of the `GuiControl` that the v1 knows are the blank
/// one, the "Text", the "Move", the "MoveDraw", the "Focus", the
/// "Disable", the "Enable", the "Hide", the "Show", the "Choose", the
/// "ChooseString" and the "Font", and the "+Redraw" and the "-Redraw" are
/// accepted too.
///
/// Measured on the binary of v1.1.37.02: the script goes
/// on after each one of them.
#[test]
fn the_sub_commands_of_the_gui_control_that_the_v1_knows_are_accepted() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit\n\
         GuiControl, PVwin:, MyEdit, Hello\n\
         GuiControl, PVwin: Text, MyEdit, Hello\n\
         GuiControl, PVwin: Move, MyEdit, x1 y2 w3 h4\n\
         GuiControl, PVwin: MoveDraw, MyEdit, x1 y2 w3 h4\n\
         GuiControl, PVwin: Focus, MyEdit\n\
         GuiControl, PVwin: Disable, MyEdit\n\
         GuiControl, PVwin: Enable, MyEdit\n\
         GuiControl, PVwin: Hide, MyEdit\n\
         GuiControl, PVwin: Show, MyEdit\n\
         GuiControl, PVwin: +Redraw, MyEdit\n\
         GuiControl, PVwin: -Redraw, MyEdit\n\
         x := 1",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("1".into())));
}

// -- the Gui of a named window ---------------------------------------------------

/// The `Show` of a named window takes the place of the window, and the
/// title is the third parameter.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVwin:
/// Show, w640 h480 x10 y20, The title` puts the window at ten and twenty
/// and gives it the title.
#[test]
fn the_show_of_a_named_window_takes_the_place_and_the_title() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVwin: Add, Edit, vMyEdit\nGui, PVwin: Show, w640 h480 x10 y20, The title",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(
        calls.contains(&"gui_window_place PVwin Some(10) Some(20) Some(640) Some(480) false false".to_string()),
        "{calls:?}"
    );
    assert!(
        calls.contains(&"gui_show PVwin The title".to_string()),
        "{calls:?}"
    );
}

/// The `Show` of a named window takes the "AutoSize", the "NoActivate"
/// and the "NA".
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVother:
/// Show, NoActivate AutoSize, The other` and the `Gui, PVthird: Show, NA`
/// are both accepted.
#[test]
fn the_show_of_a_named_window_takes_the_autosize_and_the_noactivate() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVother: Add, Button, , A button\n\
         Gui, PVother: Show, NoActivate AutoSize, The other\n\
         Gui, PVthird: Add, Button, , A button\n\
         Gui, PVthird: Show, NA",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(calls.contains(&"gui_auto_size PVother".to_string()), "{calls:?}");
    assert!(
        calls.contains(&"gui_show_without_activating PVother The other".to_string()),
        "{calls:?}"
    );
    assert!(
        calls.contains(&"gui_show_without_activating PVthird ".to_string()),
        "{calls:?}"
    );
}

/// The `Submit` writes the contents of the controls into the variables of
/// them and hides the window.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVwin:
/// Submit` leaves `MyEdit` with the text of the control, and the window
/// is not seen any more.
#[test]
fn the_submit_of_a_named_window_writes_the_controls_and_hides_the_window() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVwin: Add, Edit, vMyEdit, Hello\nGui, PVwin: Submit",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("myedit"), Some(&Value::Text("Hello".into())));
    assert!(calls.contains(&"gui_submit PVwin".to_string()), "{calls:?}");
    assert!(calls.contains(&"gui_hide PVwin".to_string()), "{calls:?}");
}

/// The `Submit` with the `NoHide` leaves the window where it is.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVwin:
/// Submit, NoHide` does not hide the window.
#[test]
fn the_submit_with_the_nohide_leaves_the_window_where_it_is() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVwin: Add, Edit, vMyEdit, Hello\nGui, PVwin: Submit, NoHide",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("myedit"), Some(&Value::Text("Hello".into())));
    assert!(!calls.contains(&"gui_hide PVwin".to_string()), "{calls:?}");
}

/// The `Default` makes the window that the name names the one of the
/// thread.
///
/// Measured on the binary of v1.1.37.02: after the `Gui,
/// PVwin: Default`, the `GuiControlGet, out, , MyEdit` answers the text
/// of the control of the window `PVwin`.
#[test]
fn the_default_of_a_named_window_makes_it_the_one_of_the_thread() {
    let report = run(
        "Gui, PVwin: Add, Edit, vMyEdit, Hello\n\
         Gui, PVother: Add, Edit, vOtherEdit, Bye\n\
         Gui, PVwin: Default\n\
         GuiControlGet, out, , MyEdit",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("Hello".into())));
}

/// The `ListView` of a named window names the control that the `LV_*`
/// write to.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVlist:
/// ListView, MyList` names the list view of the window, and the `LV_*`
/// write to it.
#[test]
fn the_list_view_of_a_named_window_names_the_control_of_the_lv() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVlist: Add, ListView, vMyList\n\
         Gui, PVlist: ListView, MyList\n\
         LV_Add(\"Row one\")\n\
         count := LV_GetCount()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(
        calls.contains(&"gui_use_window PVlist".to_string()),
        "{calls:?}"
    );
    assert!(calls.contains(&"lv_add 1 Row one".to_string()), "{calls:?}");
}

/// The `TreeView` of a named window names the control that the `TV_*`
/// write to.
///
/// Measured on the binary of v1.1.37.02: the `Gui, PVtree:
/// TreeView, MyTree` names the tree view of the window, and the `TV_*`
/// write to it.
#[test]
fn the_tree_view_of_a_named_window_names_the_control_of_the_tv() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVtree: Add, TreeView, vMyTree\n\
         Gui, PVtree: TreeView, MyTree\n\
         TV_Add(\"Node one\")\n\
         count := TV_GetCount()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(
        calls.contains(&"gui_use_window PVtree".to_string()),
        "{calls:?}"
    );
    assert!(calls.contains(&"tv_add 1 Node one".to_string()), "{calls:?}");
}

