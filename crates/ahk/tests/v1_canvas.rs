//! The windows of the v1 that a script of the v1 paints into.
//!
//! A script of the v1 does not draw its picture in the window of
//! the script: it makes four windows of their own -- the picture, the
//! thumbnails, the selector and the infos -- and it makes them the
//! children of the window of the script with the `SetParent` of the
//! machine.
//!
//! The cases of this file are about what the tree asks of the machine
//! when the script writes the options of those windows: the extended
//! style of them (`+E0x...`), the ordinary style of them (`+0x...`),
//! the owner of them (`+Owner`) and the parent of them (`+Parent`).
//!
//! The facts of the v1 that the cases carry were measured on the
//! binary of AutoHotkey v1.1.37.02, and the probes are
//! in the probe of the measurement (the
//! styles of the windows of the v1) and in
//! the probe of the measurement (the caption,
//! the resize and the owner).

use ahk::desktop::Desktop;
use ahk::dialect;
use ahk::runtime::{Report, Runtime};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::Duration;

/// The turn of the tests: the settings of the script live in a static,
/// and the runtime writes the defaults of its dialect when it is made.
/// Two cases that run a script cannot run at the same time, or the one
/// that reads a setting reads the one that the other wrote.
static THE_TURN: Mutex<()> = Mutex::new(());

/// A desktop that answers the questions of the windows of the v1 and
/// that remembers what it was asked.
struct TheFake {
    /// What the machine was asked, in the order of the questions.
    calls: RefCell<Vec<String>>,
    /// The windows of the v1, by the name that the script gave them.
    guis: RefCell<BTreeMap<String, bool>>,
}

/// The handle that the fake gives to the window of a name.
///
/// The handle is the number of the window and the length of its name,
/// so that every window of the fake has a handle of its own.
fn the_handle_of(name: &str) -> i64 {
    2000 + name.len() as i64
}

impl ahk::desktop::Keyboard for TheFake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl ahk::desktop::Pointer for TheFake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}

impl ahk::desktop::Screen for TheFake {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Ok(String::new())
    }

    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok(String::new())
    }
}

impl ahk::desktop::Processes for TheFake {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

impl ahk::desktop::Windows for TheFake {
    fn gui_list_modify(
        &mut self,
        _number: u64,
        _row: i64,
        _cells: &[String],
    ) -> Result<i64, String> {
        Ok(0)
    }

    // -- the windows of the v1 ----------------------------------------------------

    fn v1_gui_open(&mut self, name: &str, title: &str) -> Result<String, String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_open {name} {title}"));
        self.guis.borrow_mut().insert(name.to_string(), true);
        Ok(name.to_string())
    }

    fn v1_gui_is_open(&mut self, name: &str) -> Result<bool, String> {
        Ok(self.guis.borrow().get(name).copied().unwrap_or(false))
    }

    fn v1_gui_window_handle(&mut self, window: &str) -> Result<i64, String> {
        Ok(the_handle_of(window))
    }

    fn v1_gui_option(
        &mut self,
        _window: &str,
        name: &str,
        value: Option<&str>,
        on: bool,
    ) -> Result<(), String> {
        self.calls.borrow_mut().push(format!(
            "gui_option {name} {on} {}",
            value.unwrap_or_default()
        ));
        Ok(())
    }

    fn v1_gui_style(
        &mut self,
        _window: &str,
        extended: bool,
        style: u32,
        on: bool,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_style {extended} {style} {on}"));
        Ok(())
    }

    fn v1_gui_owner(
        &mut self,
        _window: &str,
        owner: Option<&str>,
        parent: bool,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push(format!("gui_owner {} {parent}", owner.unwrap_or_default()));
        Ok(())
    }
}

impl ahk::desktop::Files for TheFake {}
impl ahk::desktop::Clipboard for TheFake {}
impl ahk::desktop::Timers for TheFake {}
impl ahk::desktop::Hotkeys for TheFake {}
impl ahk::desktop::Native for TheFake {}
impl ahk::desktop::Com for TheFake {}
impl ahk::desktop::Pictures for TheFake {}
impl ahk::desktop::Network for TheFake {}
impl ahk::desktop::Sound for TheFake {}
impl ahk::desktop::Machine for TheFake {}
impl ahk::desktop::Dialogs for TheFake {}

impl Desktop for TheFake {}

/// Run a script of the v1 and answer with the report and with what the
/// machine was asked.
fn run_and_the_calls(script: &str) -> (Report, Vec<String>) {
    let _turn = THE_TURN.lock().unwrap_or_else(|poison| poison.into_inner());
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = TheFake {
        calls: RefCell::new(Vec::new()),
        guis: RefCell::new(BTreeMap::new()),
    };
    let report = Runtime::new(&mut desktop, Duration::from_secs(60))
        .with_dialect(dialect::v1())
        .run(&statements);
    let calls = desktop.calls.into_inner();
    (report, calls)
}

/// The canvas of a script of the v1, the way the script writes it.
///
/// Measured on the binary of v1.1.37.02: the line is
/// accepted, and the window that it makes has the extended style
/// `0x80020` -- the `WS_EX_LAYERED` and the `WS_EX_TRANSPARENT` -- and
/// the ordinary style `0x8C0A0000`.
const THE_CANVAS: &str =
    "Gui, PVgdiPic: -DPIScale +E0x20 +Disabled -Caption +E0x80000 +hwndhGDIwin +OwnerPVwin";

/// The extended style of the canvas of a script of the v1 is
/// written in the extended style of the window.
///
/// Measured on the binary of v1.1.37.02: the `+E0x20` of
/// a window of the v1 adds the `WS_EX_TRANSPARENT` (the `0x20`) to the
/// extended style of it, and the `+E0x80000` adds the `WS_EX_LAYERED`
/// (the `0x80000`). The extended style of the window of the script
/// goes from `0x100` to `0x80020`.
#[test]
fn the_extended_style_of_the_canvas_is_written_in_the_extended_style() {
    let (report, calls) = run_and_the_calls(THE_CANVAS);
    assert!(report.ok, "{report:?}");
    // The `WS_EX_TRANSPARENT` and the `WS_EX_LAYERED`.
    assert!(
        calls.contains(&"gui_style true 32 true".to_string()),
        "{calls:?}"
    );
    assert!(
        calls.contains(&"gui_style true 524288 true".to_string()),
        "{calls:?}"
    );
}

/// The ordinary style of the canvas of a script of the v1 is
/// written in the ordinary style of the window.
///
/// Measured on the binary of v1.1.37.02: the `+0x40000000`
/// of a window of the v1 adds the `WS_CHILD` (the `0x40000000`) to the
/// ordinary style of it: the style of the window goes from `0x84CA0000`
/// to `0xC4CA0000`.
#[test]
fn the_ordinary_style_of_the_canvas_is_written_in_the_ordinary_style() {
    let (report, calls) = run_and_the_calls("Gui, PVgdiPic: +0x40000000");
    assert!(report.ok, "{report:?}");
    assert!(
        calls.contains(&"gui_style false 1073741824 true".to_string()),
        "{calls:?}"
    );
}

/// The style of the canvas of a script of the v1 is taken away when
/// the sign of it is a minus.
///
/// Measured on the binary of v1.1.37.02: the `-E0x20` of
/// a window of the v1 takes the `WS_EX_TRANSPARENT` away from the
/// extended style of it.
#[test]
fn a_style_with_a_minus_is_taken_away_from_the_window() {
    let (report, calls) = run_and_the_calls("Gui, PVgdiPic: -E0x20");
    assert!(report.ok, "{report:?}");
    assert!(
        calls.contains(&"gui_style true 32 false".to_string()),
        "{calls:?}"
    );
}

/// The caption of the canvas of a script of the v1 is taken away.
///
/// Measured on the binary of v1.1.37.02: the `-Caption`
/// of a window of the v1 takes the `WS_CAPTION`, the `WS_THICKFRAME`
/// and the `WS_MAXIMIZEBOX` away from the ordinary style of it, and it
/// takes the `WS_EX_WINDOWEDGE` and the `WS_EX_CLIENTEDGE` away from
/// the extended style of it: the ordinary style of the window goes
/// from `0x84CA0000` to `0x840A0000` and its extended style from
/// `0x100` to `0`.
#[test]
fn the_caption_of_the_canvas_is_taken_away() {
    let (report, calls) = run_and_the_calls("Gui, PVgdiPic: -Caption");
    assert!(report.ok, "{report:?}");
    assert!(
        calls.contains(&"gui_option caption false ".to_string()),
        "{calls:?}"
    );
}

/// The canvas of a script of the v1 is owned by the window of the
/// script.
///
/// Measured on the binary of v1.1.37.02: the `+Owner` of
/// a window of the v1 puts the handle of the other window in the
/// `GWLP_HWNDPARENT` of it, and the `+Parent` of a window of the v1
/// does the same and takes the `WS_POPUP` away and gives it the
/// `WS_CHILD`.
#[test]
fn the_canvas_is_owned_by_the_window_of_the_script() {
    let (report, calls) = run_and_the_calls(THE_CANVAS);
    assert!(report.ok, "{report:?}");
    assert!(
        calls.contains(&"gui_owner PVwin false".to_string()),
        "{calls:?}"
    );
}

/// The parent of a window of the v1 is not the owner of it.
///
/// The `+Parent` and the `+Owner` are two different things: the
/// `+Owner` makes the window owned by the other one, and the `+Parent`
/// makes it the child of the other one. Measured on the binary of
/// v1.1.37.02: the `+Parent` of a window of the v1 takes
/// the ordinary style of it from `0x84CA0000` to `0x44CA0000` -- the
/// `WS_POPUP` goes away and the `WS_CHILD` comes.
#[test]
fn the_parent_of_a_window_is_not_the_owner_of_it() {
    let (report, calls) = run_and_the_calls("Gui, PVgdiPic: +ParentPVwin");
    assert!(report.ok, "{report:?}");
    assert!(
        calls.contains(&"gui_owner PVwin true".to_string()),
        "{calls:?}"
    );
}

/// The handle of the canvas of a script of the v1 reaches the
/// script.
///
/// A script of the v1 paints its picture with GDI+ into the
/// canvas, and it needs the handle of it to do so: the `+hwndhGDIwin`
/// writes the handle of the window into the variable of the script.
#[test]
fn the_handle_of_the_canvas_reaches_the_script() {
    let (report, calls) = run_and_the_calls(THE_CANVAS);
    assert!(report.ok, "{report:?}");
    // The report of a run writes the names of the variables in lower
    // case; the script keeps the case that it wrote.
    let handle = report
        .variables
        .get("hgdiwin")
        .expect("the variable of the handle of the canvas");
    assert_eq!(
        handle.to_text(),
        // The v1 writes the handle of a window as a hexadecimal text
        // with the "0x" in front of it (source/var.cpp of v1.1.37.02,
        // `Var::AssignHWND`).
        format!("0x{:x}", the_handle_of("PVgdiPic")),
        "the handle of the canvas is the handle of the window of the machine"
    );
    // The canvas is a window of the script, and the script made it.
    assert!(
        calls.contains(&"gui_open PVgdiPic ".to_string()),
        "{calls:?}"
    );
}

/// The window of the script is made before the canvas of it.
///
/// A script of the v1 makes the window of the script first, and the
/// canvas after it, and the `+Owner` of the canvas names the window of
/// the script.
#[test]
fn the_window_of_the_script_is_made_before_the_canvas() {
    let (report, calls) = run_and_the_calls(
        "Gui, PVwin: +hwndPVhwnd\nGui, PVgdiPic: +E0x20 +hwndhGDIwin +OwnerPVwin",
    );
    assert!(report.ok, "{report:?}");
    let open_of_the_script = calls
        .iter()
        .position(|call| call == "gui_open PVwin ")
        .expect("the window of the script is made");
    let open_of_the_canvas = calls
        .iter()
        .position(|call| call == "gui_open PVgdiPic ")
        .expect("the canvas is made");
    assert!(
        open_of_the_script < open_of_the_canvas,
        "the window of the script is made first: {calls:?}"
    );
}
