//! The language and the runtime, against a fake desktop.
//!
//! No SSH, no box, no ydotool: the desktop is a fake that records the
//! calls, so these tests are about what the language means, not about what
//! the desktop does. The desktop part is verified against the real box
//! separately.
//!
//! These are a port of the archived Python tests, and the two
//! implementations have to agree.

use ahk::desktop::WindowAnswer;
use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, TitleMatch, Windows,
};
use ahk::runtime::{Report, Runtime, Value};
use ahk::{Node, parse};
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    calls: Vec<(String, String)>,
    pixels: Vec<((i64, i64), String)>,
    /// Where the pointer is.
    pointer: (i64, i64),
    /// The windows the fake knows about, and the one that is active.
    windows: Vec<String>,
    active: Option<String>,
    /// When it is set, the window questions cannot be answered.
    window_answer_unknown: bool,
}

impl Fake {
    fn types(&self) -> Vec<String> {
        self.calls
            .iter()
            .filter(|(kind, _)| kind == "type")
            .map(|(_, value)| value.clone())
            .collect()
    }
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}

impl Files for Fake {
    fn list_files(
        &mut self,
        folder: &str,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        self.calls
            .push(("list_files".into(), format!("{folder}|{pattern}|{mode}")));
        // The pattern of v2 is the tail of the name: "*.txt" is every
        // name that ends with ".txt".
        let wanted = pattern.strip_prefix('*').unwrap_or(pattern);
        Ok(["/work/one.txt", "/work/two.txt", "/work/notes.log"]
            .into_iter()
            .filter(|file| wanted.is_empty() || file.ends_with(wanted))
            .map(|file| file.to_string())
            .collect())
    }
    fn working_directory(&mut self) -> Result<String, String> {
        self.calls.push(("working_directory".into(), String::new()));
        Ok("/work".to_string())
    }
}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        self.calls.push(("type".into(), text.into()));
        Ok(format!("typed {text:?}"))
    }
    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        let mut spelled = modifiers.join("+");
        if !spelled.is_empty() {
            spelled.push('+');
        }
        spelled.push_str(name);
        self.calls.push(("key".into(), spelled.clone()));
        Ok(format!("pressed {spelled}"))
    }
    fn key_down(&mut self, name: &str) -> Result<String, String> {
        self.calls.push(("key_down".into(), name.into()));
        Ok(format!("{name} is down"))
    }
    fn key_up(&mut self, name: &str) -> Result<String, String> {
        self.calls.push(("key_up".into(), name.into()));
        Ok(format!("{name} is up"))
    }
}

impl Pointer for Fake {
    fn mouse_down(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        self.calls.push(("mouse_down".into(), button.into()));
        Ok(format!("{button} is down"))
    }
    fn mouse_up(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        self.calls.push(("mouse_up".into(), button.into()));
        Ok(format!("{button} is up"))
    }
    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
        self.calls
            .push(("click".into(), format!("{x},{y},{button},{count}")));
        Ok(format!("clicked ({x}, {y})"))
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.pointer = (x, y);
        self.calls.push(("move".into(), format!("{x},{y}")));
        Ok(format!("moved to ({x}, {y})"))
    }
    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        Ok(self.pointer)
    }
}

impl Screen for Fake {
    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String> {
        self.calls.push((
            "capture".into(),
            format!("{x},{y},{width},{height},{filename}"),
        ));
        Ok(format!("/tmp/{filename}"))
    }
    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("pixel".into(), format!("{x},{y}")));
        Ok(self
            .pixels
            .iter()
            .find(|(at, _)| *at == (x, y))
            .map(|(_, colour)| colour.clone())
            .unwrap_or_else(|| "#000000".into()))
    }
    fn pixel_search(
        &mut self,
        _x1: i64,
        _y1: i64,
        _x2: i64,
        _y2: i64,
        colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        Ok(self
            .pixels
            .iter()
            .find(|(_, value)| value == colour)
            .map(|(at, _)| *at))
    }
}

impl Windows for Fake {
    fn window_active(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        if self.window_answer_unknown {
            return Ok(WindowAnswer::Unknown("this desktop cannot tell".into()));
        }
        // The window that has the focus is a window that is there.
        if let Some(active) = &self.active {
            if mode.matches(active, title)? {
                return Ok(WindowAnswer::Active);
            }
        }
        for window in &self.windows {
            if mode.matches(window, title)? {
                return Ok(WindowAnswer::NotActive);
            }
        }
        // A window that is not there is not the active one.
        Ok(WindowAnswer::Absent)
    }
    fn window_exists(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        if self.window_answer_unknown {
            return Ok(WindowAnswer::Unknown("this desktop cannot list".into()));
        }
        if let Some(active) = &self.active {
            if mode.matches(active, title)? {
                return Ok(WindowAnswer::Active);
            }
        }
        for window in &self.windows {
            if mode.matches(window, title)? {
                return Ok(WindowAnswer::NotActive);
            }
        }
        // A window that is not there is not there.
        Ok(WindowAnswer::Absent)
    }
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        Ok(self.active.clone())
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        self.calls.push(("run".into(), command.into()));
        Ok((0, format!("ran {command}\n"), String::new()))
    }
}

impl Native for Fake {}

fn run(source: &str) -> (Report, Fake) {
    let mut desktop = Fake::default();
    desktop.pixels.push(((10, 20), "#ff8800".into()));
    let statements = parse(source).expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_source(source)
        .run(&statements);
    (report, desktop)
}

/// The same, with a timeout of its own.
fn run_with_timeout(source: &str, timeout: Duration) -> (Report, Fake) {
    let mut desktop = Fake::default();
    desktop.pixels.push(((10, 20), "#ff8800".into()));
    let statements = parse(source).expect("parses");
    let report = Runtime::new(&mut desktop, timeout)
        .with_source(source)
        .run(&statements);
    (report, desktop)
}

/// The same, but speaking the dialect the caller names.
fn run_in(source: &str, dialect: &'static dyn ahk::dialect::Dialect) -> (Report, Fake) {
    let mut desktop = Fake::default();
    let statements = ahk::parse_with(source, dialect).expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect)
        .with_source(source)
        .run(&statements);
    (report, desktop)
}

// -- #Requires ---------------------------------------------------------------
//
// Measured on the binaries of both dialects, and on the tree after the
// change: the line names the dialect the script needs -- `v1` or `v2` -- and
// nothing else. The dialects are names and not sub-versions, and the tree has
// no second name of its own to put in the line: the project is called `ahk`,
// and a name made up for the directive would be a name to keep in step with
// the first.
//
// The line still tells the two implementations apart, because mainline
// AutoHotkey reads the first word as the name of the interpreter: measured on
// the binary of v2.0.31, `#Requires v2` is refused with "This script requires
// v2. Current interpreter: AutoHotkey v2.0.31 64-bit.".

#[test]
fn the_dialect_alone_is_the_requirement() {
    let (report, _) = run("#Requires v2\nFileAppend \"ok\", \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn the_tree_answers_to_the_mainline_name_as_well() {
    // For a script that is meant to run in both: the mainline binary accepts
    // this line too.
    let (report, _) = run("#Requires AutoHotkey v2\nFileAppend \"ok\", \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn the_mainline_name_alone_is_accepted() {
    // Measured on the binary of v2.0.31: the name without a dialect asks for
    // the interpreter and not for a dialect, and it runs.
    let (report, _) = run("#Requires AutoHotkey\nFileAppend \"ok\", \"*\"");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_other_dialect_is_refused() {
    let (report, _) = run("#Requires v1");
    assert!(!report.ok);
    let error = report.error.as_deref().unwrap_or("");
    assert!(error.contains("This interpreter is v2."), "{error}");
}

#[test]
fn a_dialect_that_does_not_exist_is_refused() {
    let (report, _) = run("#Requires v3");
    assert!(!report.ok);
    let error = report.error.as_deref().unwrap_or("");
    assert!(error.contains("This script requires v3."), "{error}");
}

#[test]
fn the_invented_name_of_the_tree_is_not_a_name_of_the_tree() {
    // `RustAhk` was a candidate for the line and it is not the name of the
    // project. It is refused like any other word that is not the dialect.
    let (report, _) = run("#Requires RustAhk v2");
    assert!(!report.ok);
    let error = report.error.as_deref().unwrap_or("");
    assert!(error.contains("This script requires RustAhk v2."), "{error}");
}

#[test]
fn an_empty_requires_says_the_parameter_is_required() {
    let (report, _) = run("#Requires");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Parameter #1 required"));
}

#[test]
fn a_requires_with_a_third_word_is_refused() {
    let (report, _) = run("#Requires AutoHotkey v2 extra");
    assert!(!report.ok);
}

#[test]
fn the_v1_dialect_is_met_under_the_v1_dialect() {
    let (report, _) = run_in("#Requires v1\nFileAppend, ok, *", ahk::dialect::v1());
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn the_v2_dialect_is_refused_under_the_v1_dialect() {
    let (report, _) = run_in("#Requires v2", ahk::dialect::v1());
    assert!(!report.ok);
    let error = report.error.as_deref().unwrap_or("");
    assert!(error.contains("This interpreter is v1."), "{error}");
}

#[test]
fn the_version_of_the_interpreter_is_met() {
    // The line of mainline AutoHotkey names the version and not the dialect.
    // Measured on the binary of v2.0.19 on 2026-11-09: all the forms below
    // are met by the 2.0.19, and a version that is written without the `v`
    // is the same version as the one with it.
    for requirement in [
        "#Requires AutoHotkey v2.0",
        "#Requires AutoHotkey v2.0.19",
        "#Requires AutoHotkey v2.0.19+",
        "#Requires AutoHotkey 2.0.19",
        "#Requires AutoHotkey >=2.0",
        "#Requires AutoHotkey >2.0",
        "#Requires AutoHotkey <2.1",
        "#Requires AutoHotkey <=2.0.19",
        "#Requires AutoHotkey =2.0.19",
    ] {
        let (report, _) = run(&format!("{requirement}\nFileAppend \"ok\", \"*\""));
        assert!(report.ok, "{requirement}: {:?}", report.error);
        assert_eq!(report.output, vec!["ok"]);
    }
}

#[test]
fn a_later_version_is_refused() {
    for requirement in [
        "#Requires AutoHotkey v2.0.20",
        "#Requires AutoHotkey v2.1",
        "#Requires AutoHotkey >=2.1",
        "#Requires AutoHotkey <>2.0.19",
        "#Requires AutoHotkey =2.0",
    ] {
        let (report, _) = run(requirement);
        assert!(!report.ok, "{requirement} is met, and it is not a version of this tree");
    }
}

#[test]
fn a_requirement_of_another_major_version_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the v2 does not meet a
    // requirement of the v1, although 2.0.19 is the later version. The
    // dialects are not versions of one another.
    let (report, _) = run("#Requires AutoHotkey v1.1");
    assert!(!report.ok);
    let error = report.error.as_deref().unwrap_or("");
    assert!(error.contains("This script requires AutoHotkey v1.1."), "{error}");
}

#[test]
fn a_send_types_the_text() {
    let (report, desktop) = run("Send \"hello\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["hello"]);
}

#[test]
fn a_send_with_a_key_presses_it() {
    let (report, desktop) = run("Send \"a{Enter}b\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["a", "b"]);
    assert!(desktop.calls.contains(&("key".into(), "Enter".into())));
}

#[test]
fn an_assignment_sets_a_variable() {
    let (report, desktop) = run("x := 41 + 1\nSend \"%x%\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["42"]);
}

#[test]
fn a_concatenation_uses_the_dot() {
    let (report, desktop) = run("a := \"x\"\nb := a . \"y\"\nSend b");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["xy"]);
}

#[test]
fn an_if_takes_the_branch_that_holds() {
    let (report, desktop) =
        run("x := 5\nIf x > 3 {\n  Send \"big\"\n} Else {\n  Send \"small\"\n}");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["big"]);
}

#[test]
fn an_if_with_no_else_does_nothing_when_false() {
    let (report, desktop) = run("x := 1\nIf x > 3 {\n  Send \"big\"\n}");
    assert!(report.ok);
    assert!(desktop.types().is_empty());
}

#[test]
fn a_while_runs_until_the_condition_breaks() {
    let (report, desktop) = run("i := 0\nWhile i < 3 {\n  i := i + 1\n}\nSend \"%i%\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_loop_runs_its_body_n_times() {
    let (report, desktop) = run("Loop 3 {\n  Send \"x\"\n}");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["x", "x", "x"]);
}

#[test]
fn a_break_leaves_the_loop() {
    let (report, desktop) =
        run("i := 0\nLoop 10 {\n  i := i + 1\n  If i = 2 {\n    Break\n  }\n}\nSend \"%i%\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["2"]);
}

#[test]
fn a_continue_skips_the_rest_of_the_body() {
    let (report, desktop) =
        run("i := 0\nLoop 3 {\n  i := i + 1\n  If i < 3 {\n    Continue\n  }\n  Send \"%i%\"\n}");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_run_carries_the_output() {
    let (report, _desktop) = run("Run \"echo hello\"");
    assert!(report.ok);
    assert!(
        report
            .output
            .iter()
            .any(|line| line.contains("ran echo hello"))
    );
}

#[test]
fn the_power_binds_tighter_than_the_unary_minus() {
    // Measured on the binary of v2.0.19 on 2026-10-28: `-2 ** 2` is -4
    // there, and not 4.
    let (report, _desktop) = run("x := -2 ** 2");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(-4)));
}

#[test]
fn a_pixel_get_reads_the_colour() {
    let (report, _desktop) = run("PixelGetColor 10, 20");
    assert!(report.ok);
    assert_eq!(
        report.variables.get("pixel"),
        Some(&Value::Text("#ff8800".into()))
    );
}

#[test]
fn a_screenshot_captures_the_region() {
    let (report, desktop) = run("Screenshot 0, 0, 100, 50, \"shot.png\"");
    assert!(report.ok);
    assert!(
        desktop
            .calls
            .contains(&("capture".into(), "0,0,100,50,shot.png".into()))
    );
}

#[test]
fn a_click_clicks_at_the_position() {
    let (report, desktop) = run("Click 40, 50");
    assert!(report.ok);
    assert!(
        desktop
            .calls
            .contains(&("click".into(), "40,50,left,1".into()))
    );
}

#[test]
fn a_msg_is_collected_as_output() {
    let (report, _desktop) = run("MsgBox \"the answer\"");
    assert_eq!(report.output, vec!["the answer"]);
}

#[test]
fn a_directive_that_is_already_the_case_is_accepted() {
    let (report, _desktop) = run("#NoTrayIcon\n#SingleInstance\n#ErrorStdOut\nMsgBox \"ok\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn a_requires_that_is_met_is_accepted() {
    let (report, _desktop) = run("#Requires AutoHotkey v2\nMsgBox \"ok\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn a_requires_for_the_dialect_of_the_run_is_accepted() {
    let (report, _desktop) = run("#Requires v2\nMsgBox \"ok\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ok"]);
}

#[test]
fn a_requires_that_is_not_met_fails_the_script() {
    let (report, _desktop) = run("#Requires AutoHotkey v1\nMsgBox \"ok\"");
    assert!(!report.ok);
    assert!(
        report.error.as_deref().unwrap_or("").contains("requires"),
        "{:?}",
        report.error
    );
}

#[test]
fn a_directive_this_runtime_does_not_carry_fails_with_the_reason() {
    let (report, _desktop) = run("#UseHook\nMsgBox \"ok\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the hotkeys are not carried by this runtime.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_loop_that_parses_a_text_runs_the_body_once_for_each_piece() {
    let (report, desktop) = run("Loop Parse \"a,b,c\", \",\" {\n  Send \"%A_LoopField%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a", "b", "c"]);
}

#[test]
fn a_loop_that_parses_a_text_counts_the_pieces() {
    let (report, _desktop) = run("Loop Parse \"a,b\", \",\" {\n  Send \"%A_Index%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    let (report, _desktop) =
        run("Loop Parse \"a,b\", \",\" {\n  last := A_Index\n}\nSend \"%last%\"");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_loop_over_the_registry_fails_with_the_reason() {
    let (report, _desktop) = run("Loop Reg \"HKLM\\\\*\" {\n}");
    assert!(!report.ok);
    assert!(
        report.error.as_deref().unwrap_or("").contains("Windows"),
        "{:?}",
        report.error
    );
}

#[test]
fn a_string_is_read_where_it_is_written() {
    // The name is the name of the scope that is writing the string, so
    // a string a function returns still knows its own parameter.
    let (report, desktop) =
        run("Greet(name) {\n  return \"hello %name%\"\n}\nSend Greet(\"world\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["hello world"]);
}

#[test]
fn a_property_can_be_in_a_string() {
    let (report, desktop) = run("class Point {\n  x := 3\n}\np := Point()\nSend \"%p.x%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_builtin_is_a_value() {
    let (report, desktop) = run("f := Abs\nSend \"%f.Name%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["Abs"]);
}

#[test]
fn a_function_in_a_variable_can_be_called() {
    let (report, desktop) = run("f := Abs\nSend f.Call(3)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_function_knows_how_many_parameters_it_wants() {
    let (report, desktop) = run("f := Abs\nSend \"%f.MinParams% %f.MaxParams%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1 1"]);
}

#[test]
fn a_builtin_is_a_builtin() {
    // `IsBuiltIn` is a property of v2, and not a method.
    let (report, desktop) = run("f := Abs\nSend f.IsBuiltIn");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

#[test]
fn the_is_built_in_of_a_function_is_not_a_method() {
    let (report, _) = run("f := Abs\nMsgBox f.IsBuiltIn()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Func\" has no method named \"IsBuiltIn\".")
    );
}

#[test]
fn an_object_knows_what_it_has() {
    let (report, desktop) = run(
        "class Point {\n  x := 3\n  Show() {\n    return \"x\"\n  }\n}\n\
         p := Point()\n\
         Send p.HasProp(\"x\") . \" \" . p.HasProp(\"y\") . \" \" . p.HasMethod(\"Show\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1 0 1"]);
}

#[test]
fn an_object_knows_the_class_it_is_of() {
    let (report, desktop) = run(
        "class Base {\n}\nclass Child extends Base {\n}\nclass Other {\n}\nc := Child()\n\
         Send c.HasBase(Base.Prototype) . \" \" . c.HasBase(Other.Prototype)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1 0"]);
}

#[test]
fn a_property_can_be_defined_and_taken_away() {
    let (report, desktop) = run(
        "class Box {\n}\nb := Box()\nb.DefineProp(\"x\", {Value: 7})\ny := b.x\n\
         z := b.DeleteProp(\"x\")\nSend \"%y% %z%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["7 7"]);
}

#[test]
fn an_object_that_is_called_is_asked_through_call() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the object is
    // asked through its `Call` method.
    let (report, desktop) = run("class Adder {\n  Call(a, b) {\n    return a + b\n  }\n}\n\
         add := Adder()\nSend add(2, 3)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["5"]);
}

#[test]
fn an_object_without_a_call_method_says_so() {
    let (report, _) = run("class Plain {\n}\n\
         p := Plain()\nMsgBox p(2, 3)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Plain\" has no method named \"Call\".")
    );
}

#[test]
fn a_method_that_is_not_there_is_asked_through_the_call_of_the_class() {
    // The `__Call` of v2 takes the name of the method and the
    // parameters of the call.
    let (report, desktop) = run(
        "class Adder {\n  __Call(name, params) {\n    return \"%name% %params.Length%\"\n  }\n}\n\
         add := Adder()\nSend add.Unknown(2, 3)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["Unknown 2"]);
}

#[test]
fn a_class_has_a_method_and_a_property() {
    let (report, desktop) = run(
        "class Point {\n  x := 3\n  Greet() {\n    return \"hello\"\n  }\n}\n\
         p := Point()\nSend p.Greet()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["hello"]);
}

#[test]
fn a_property_of_an_instance_can_be_read() {
    let (report, _desktop) = run("class Point {\n  x := 3\n}\np := Point()\nSend \"%p.x%\"");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_property_of_an_instance_can_be_written() {
    let (report, desktop) =
        run("class Point {\n  x := 3\n}\np := Point()\np.x := 7\nSend \"%p.x%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["7"]);
}

#[test]
fn a_method_sees_the_object_it_was_called_on() {
    let (report, desktop) = run(
        "class Point {\n  x := 3\n  Show() {\n    return \"x is %this.x%\"\n  }\n}\n\
         p := Point()\nSend p.Show()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["x is 3"]);
}

#[test]
fn a_class_can_extend_another_one() {
    let (report, desktop) = run("class Base {\n  Who() {\n    return \"base\"\n  }\n}\n\
         class Child extends Base {\n  Who() {\n    return \"child\"\n  }\n}\n\
         c := Child()\nSend c.Who()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["child"]);
}

#[test]
fn a_method_of_the_class_it_extends_is_there() {
    let (report, desktop) = run("class Base {\n  Who() {\n    return \"base\"\n  }\n}\n\
         class Child extends Base {\n}\nc := Child()\nSend c.Who()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["base"]);
}

#[test]
fn a_new_may_build_the_object() {
    let (report, desktop) = run(
        "class Point {\n  __New(x) {\n    this.x := x\n  }\n  x := 0\n}\n\
         p := Point(9)\nSend \"%p.x%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["9"]);
}

#[test]
fn a_get_is_asked_when_the_property_is_not_there() {
    let (report, desktop) = run(
        "class Box {\n  __Get(name, params) {\n    return \"asked for %name%\"\n  }\n}\n\
         b := Box()\nSend b.Whatever",
    );
    assert!(report.ok, "{:?}", report.error);
    // The name is the name the script wrote.
    assert_eq!(desktop.types(), vec!["asked for Whatever"]);
}

#[test]
fn a_set_is_told_when_the_property_is_written() {
    let (report, desktop) = run(
        "class Box {\n  __Set(name, params, value) {\n    this.seen := name\n    this.written := value\n  }\n\
         seen := \"\"\n  written := \"\"\n}\n\
         b := Box()\nb.thing := 1\nSend b.seen \" \" b.written",
    );
    assert!(report.ok, "{:?}", report.error);
    // The value that is written goes with the name.
    assert_eq!(desktop.types(), vec!["thing 1"]);
}

#[test]
fn a_get_takes_the_parameters_of_the_access() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the brackets of
    // the access are the parameters of the meta-function.
    let (report, desktop) = run(
        "class Box {\n  __Get(name, params) {\n    return \"%name% %params.Length%\"\n  }\n}\n\
         b := Box()\nSend b.Whatever[1]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["Whatever 1"]);
}

#[test]
fn a_get_handles_the_whole_access() {
    // The answer of the meta-function is the answer of the access: the
    // brackets are its parameters and not an index of its answer.
    let (report, desktop) = run(
        "class Box {\n  __Get(name, params) {\n    return \"the answer\"\n  }\n}\n\
         b := Box()\nSend b.Whatever[1]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["the answer"]);
}

#[test]
fn a_set_takes_the_value_that_is_written() {
    let (report, desktop) = run(
        "class Box {\n  __Set(name, params, value) {\n    this.written := value\n  }\n\
         written := \"\"\n}\n\
         b := Box()\nb.thing := 7\nSend b.written",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["7"]);
}

#[test]
fn a_function_is_a_value() {
    // The name of a function is the name the script wrote, and not the
    // name it is looked up by.
    let (report, desktop) = run("Add(a, b) {\n  return a + b\n}\nf := Add\nSend f.Name");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["Add"]);
}

#[test]
fn a_parameter_may_carry_a_default() {
    let (report, desktop) = run("Add(a, b := 2) {\n  return a + b\n}\nSend Add(1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_method_called_with_too_few_arguments_fails_with_the_reason() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _desktop) =
        run("class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
    assert_eq!(report.error_line, Some(7));
}

#[test]
fn a_method_called_with_too_many_arguments_fails_with_the_reason() {
    let (report, _desktop) =
        run("class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method(1, 2)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.")
    );
    assert_eq!(report.error_line, Some(7));
}

#[test]
fn a_method_without_parameters_runs_when_none_are_given() {
    let (report, desktop) = run(
        "class Thing {\n  Method() {\n    return \"the answer\"\n  }\n}\nt := Thing()\nSend t.Method()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["the answer"]);
}

#[test]
fn a_method_takes_the_default_of_a_parameter_that_is_not_given() {
    let (report, desktop) = run(
        "class Thing {\n  Method(a := \"the default\") {\n    return a\n  }\n}\nt := Thing()\nSend t.Method()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["the default"]);
}

#[test]
fn a_parameter_may_take_the_rest_of_the_arguments() {
    let (report, desktop) = run("Add(a, rest*) {\n  return a + rest.Length\n}\nSend Add(1, 2, 3)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_call_with_too_few_arguments_fails_with_the_reason() {
    let (report, _desktop) = run("Add(a, b) {\n  return a + b\n}\nAdd(1)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too few parameters passed to function.\n     Specifically: Add")
    );
    assert_eq!(report.error_line, Some(4));
}

#[test]
fn a_call_with_too_many_arguments_fails_with_the_reason() {
    let (report, _desktop) = run("Add(a) {\n  return a\n}\nAdd(1, 2)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.\n     Specifically: Add")
    );
    assert_eq!(report.error_line, Some(4));
}

#[test]
fn a_function_that_is_held_in_a_variable_can_be_called() {
    let (report, desktop) = run("Add(a, b) {\n  return a + b\n}\nf := Add\nSend f(2, 3)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["5"]);
}

#[test]
fn a_method_that_is_not_there_fails_with_the_reason() {
    let (report, _desktop) = run("class Point {\n}\np := Point()\np.Nothing()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Point\" has no method named \"Nothing\".")
    );
}

#[test]
fn a_variable_that_was_never_set_is_an_error() {
    let (report, _desktop) = run("x := nope");
    assert!(!report.ok);
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn an_unknown_name_in_a_string_is_left_alone() {
    let (report, desktop) = run("Send \"%nope%\"");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["%nope%"]);
}

#[test]
fn a_division_by_zero_is_an_error() {
    let (report, _desktop) = run("x := 1 / 0");
    assert!(!report.ok);
}

#[test]
fn an_endless_loop_is_stopped() {
    let (report, _desktop) = run("Loop 100000 {\n  Send \"x\"\n}");
    assert!(!report.ok);
    assert!(report.error.as_deref().unwrap_or("").contains("more than"));
}

#[test]
fn a_while_body_runs_at_most_a_thousand_times() {
    let (report, _desktop) = run("while (true) {\n}");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the loop ran more than 1000 times")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_script_runs_at_most_twenty_thousand_statements() {
    // A counted `Loop` has no guard of its own: the step count is what stops
    // it, which is why the message is about statements and not about turns.
    let (report, _desktop) = run("Loop 100000 {\n  Send \"x\"\n}");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the script ran more than 20000 statements")
    );
    assert!(report.error_line.is_some());
}

#[test]
fn a_script_that_outlives_its_timeout_is_stopped() {
    let (report, _desktop) =
        run_with_timeout("Sleep 200\nMsgBox \"late\"", Duration::from_millis(50));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the script ran for more than 0.05s")
    );
    // The timeout is not a statement: it stops the script between two of
    // them, so it does not know a line, and it says so by leaving it out.
    assert_eq!(report.error_line, None);
}

#[test]
fn every_step_is_reported() {
    let (report, _desktop) = run("Send \"a\"\nSend \"b\"");
    assert_eq!(report.steps.len(), 2);
    assert_eq!(report.steps[0].op, "Send");
    assert_eq!(report.steps[1].op, "Send");
    assert!(report.steps.iter().all(|step| step.ok && step.line > 0));
}

#[test]
fn the_statements_of_a_script_keep_their_order() {
    let statements = parse("x := 1\nSend \"a\"\nSleep 100").expect("parses");
    let names: Vec<&str> = statements.iter().map(ahk::runtime::op_name).collect();
    assert_eq!(names, vec!["Assign", "Send", "Sleep"]);
    assert!(matches!(statements[0], Node::Assign { .. }));
}

// -- the basic syntax -----------------------------------------------------------

#[test]
fn an_else_if_is_taken_when_the_first_one_is_false() {
    let (report, desktop) = run(
        "x := 2\nIf x = 1 {\n  Send \"one\"\n} Else If x = 2 {\n  Send \"two\"\n} Else {\n  Send \"other\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["two"]);
}

#[test]
fn an_else_if_chain_takes_only_the_first_branch_that_holds() {
    let (report, desktop) = run(
        "x := 5\nIf x = 1 {\n  Send \"one\"\n} Else If x = 5 {\n  Send \"two\"\n} Else If x = 5 {\n  Send \"three\"\n} Else {\n  Send \"other\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["two"]);
}

#[test]
fn a_switch_runs_the_case_that_matches() {
    let (report, desktop) = run(
        "x := 2\nSwitch x {\n  Case 1:\n    Send \"one\"\n  Case 2:\n    Send \"two\"\n  Default:\n    Send \"other\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["two"]);
}

#[test]
fn a_switch_without_a_match_runs_the_default() {
    let (report, desktop) =
        run("x := 9\nSwitch x {\n  Case 1:\n    Send \"one\"\n  Default:\n    Send \"other\"\n}");
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["other"]);
}

#[test]
fn a_function_returns_a_value() {
    let (report, desktop) =
        run("Add(a, b) {\n  return a + b\n}\ntotal := Add(2, 3)\nSend \"%total%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["5"]);
}

#[test]
fn a_function_can_be_called_as_a_statement() {
    let (report, _desktop) = run("Greet(name) {\n  MsgBox \"hello %name%\"\n}\nGreet(\"world\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello world"]);
}

#[test]
fn a_function_may_be_defined_after_the_call() {
    let (report, desktop) =
        run("text := Later()\nSend \"%text%\"\nLater() {\n  return \"here\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["here"]);
}

#[test]
fn a_return_leaves_the_function_at_once() {
    let (report, desktop) =
        run("text := First()\nSend \"%text%\"\nFirst() {\n  return \"one\"\n  Send \"never\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one"]);
}

#[test]
fn a_variable_assigned_inside_a_function_is_local() {
    let (report, _desktop) =
        run("x := \"outside\"\nSet() {\n  x := \"inside\"\n}\nSet()\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["outside"]);
}

#[test]
fn exitapp_ends_the_script_and_carries_a_code() {
    let (report, desktop) = run("Send \"before\"\nExitApp 3\nSend \"after\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.exit_code, Some(3));
    assert_eq!(desktop.types(), vec!["before"]);
}

// -- the reference operator --------------------------------------------------------

#[test]
fn a_parameter_written_by_reference_is_an_alias_of_the_variable_of_the_caller() {
    // The ByRef marker of v2: the parameter and the variable of the
    // caller are the same thing.
    let (report, _desktop) = run("Swap(&a, &b) {\n  tmp := a\n  a := b\n  b := tmp\n}\n\
         a := \"one\"\nb := \"two\"\nSwap(&a, &b)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Text("two".into())));
    assert_eq!(report.variables.get("b"), Some(&Value::Text("one".into())));
}

#[test]
fn a_method_may_have_a_parameter_written_by_reference() {
    let (report, _desktop) = run(
        "class Counter {\n  Total(&value) {\n    value := value + 1\n  }\n}\n\
         c := Counter()\nn := 5\nc.Total(&n)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("n"), Some(&Value::Int(6)));
}

#[test]
fn a_parameter_written_by_reference_wants_a_reference() {
    let (report, _desktop) = run("Show(&x) {\n  out := x\n}\nShow(5)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of Show requires a variable reference, but received an Integer.")
    );
}

#[test]
fn a_parameter_written_by_reference_that_the_call_leaves_out_is_a_local_one() {
    // The default of v2 is a value of its own: the parameter is a local
    // one when the call does not give it a reference.
    let (report, _desktop) = run("Show(&x := \"default\") {\n  return x\n}\nout := Show()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("out"),
        Some(&Value::Text("default".into()))
    );
}

#[test]
fn a_reference_is_not_a_value() {
    let (report, _desktop) = run("x := 1\ny := &x\nout := \"v=\" . y");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got a VarRef.")
    );
}

#[test]
fn a_plain_parameter_that_is_given_a_reference_holds_it() {
    // The reference is a value of its own, and the parameter that is
    // not written `&x` holds it.
    let (report, _desktop) = run("Show(x) {\n  out := \"in=\" . x\n}\ny := \"orig\"\nShow(&y)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got a VarRef.")
    );
}

#[test]
fn a_plain_parameter_that_is_given_a_reference_writes_its_own_copy() {
    let (report, _desktop) = run("Show(x) {\n  x := \"written\"\n}\ny := \"orig\"\nShow(&y)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("orig".into())));
}

#[test]
fn a_reference_where_the_built_in_wants_a_value_fails() {
    // The built-ins that write into a variable of the caller want a
    // reference in that place and nowhere else. Measured on the binary of
    // v2.0.19 on 2026-10-31: the first parameter of the `SplitPath`
    // wants a text, and the reference is refused with the name of it.
    let (report, _desktop) = run("x := \"C:\\file.txt\"\nSplitPath &x, &dir");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of SplitPath requires a String, but received a VarRef.")
    );
}

#[test]
fn the_reference_operator_wants_a_variable() {
    let (report, _desktop) = run("class Box {\n  Prop := 1\n}\nb := Box()\nr := &b.Prop");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("\"&\" requires a variable."));
}

#[test]
fn a_built_in_variable_cannot_have_its_reference_taken() {
    let (report, _desktop) = run("r := &A_ScreenWidth");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "This built-in variable cannot have its reference taken.\n     Specifically: A_ScreenWidth"
        )
    );
}

#[test]
fn a_function_cannot_have_its_reference_taken() {
    let (report, _desktop) = run("r := &Abs");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This Func cannot have its reference taken.\n     Specifically: Abs")
    );
}

// -- the focus check ------------------------------------------------------------

#[test]
fn a_loop_over_the_files_takes_the_pattern_of_v2() {
    // v2 takes the pattern first and the mode second, and the folder is
    // the one the script runs in.
    let (report, desktop) = run("Loop Files \"*.txt\" {\n  Send \"%A_LoopFileName%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one.txt", "two.txt"]);
    assert!(
        desktop
            .calls
            .contains(&("working_directory".to_string(), String::new())),
        "{:?}",
        desktop.calls
    );
    assert!(
        desktop
            .calls
            .contains(&("list_files".to_string(), "/work|*.txt|".to_string())),
        "{:?}",
        desktop.calls
    );
}

#[test]
fn a_loop_over_the_files_writes_the_variables_of_v2() {
    let (report, _desktop) = run(concat!(
        "Loop Files \"*.txt\" {\n",
        "  name := A_LoopFileName\n",
        "  ext := A_LoopFileExt\n",
        "  dir := A_LoopFileDir\n",
        "  full := A_LoopFileFullPath\n",
        "  path := A_LoopFilePath\n",
        "}\n",
        "out := name . \"|\" . ext . \"|\" . dir . \"|\" . full . \"|\" . path\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("out"),
        Some(&Value::Text(
            "two.txt|txt|/work|/work/two.txt|/work/two".into()
        ))
    );
}

#[test]
fn the_short_names_of_the_files_are_refused_with_the_reason() {
    // The short names of the files are a thing of Windows, and this box has
    // none: the script that reads the name is told that, and the tree does
    // not make up a name for the file.
    //
    // Measured on the binary of AutoHotkey v2.0.19 on 2026-10-31: the
    // `A_LoopFileShortName` is one of the variables of the loop of the files.
    let (report, _desktop) = run("Loop Files \"*.txt\" {\n  short := A_LoopFileShortName\n}\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "the short names of the files are a thing of Windows, and this desktop is not Windows."
        )
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_loop_over_the_files_takes_the_comma_of_v1_too() {
    // v2 takes the comma after the kind of the loop, and does not need
    // it: both forms are the same loop.
    let (report, desktop) = run("Loop Files, \"*.txt\" {\n  Send \"%A_LoopFileName%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one.txt", "two.txt"]);
}

#[test]
fn a_loop_over_the_files_takes_the_mode_of_v2() {
    let (report, desktop) = run("Loop Files \"*.txt\", \"R\" {\n  Send \"%A_LoopFileName%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        desktop
            .calls
            .contains(&("list_files".to_string(), "/work|*.txt|R".to_string())),
        "{:?}",
        desktop.calls
    );
}

#[test]
fn a_loop_over_the_files_takes_no_more_than_two_parameters() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the command is
    // refused when the script loads, and the first parameter is named
    // with it.
    let error = parse("Loop Files \"*.txt\", \"R\", \"x\" {\n}\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Files\" accepts at most 2 parameters.\n     Specifically: Loop Files \"*.txt\""
    );
}

#[test]
fn a_recursion_that_never_ends_stops_with_the_reason() {
    // v2.0.19 stops at about 1105 nested calls, with "Function recursion
    // limit exceeded." The tree stops before its own stack is gone, and
    // says the same thing.
    let (report, _desktop) = run("F(k) {\n  return F(k + 1)\n}\nF(1)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Function recursion limit exceeded.")
    );
}

#[test]
fn two_values_side_by_side_are_one_value() {
    // The implicit concatenation of v2: measured on the binary of
    // v2.0.19 on 2026-10-28, `"a" 1 + 2` is "a3" and `1 + 2 "d"` is
    // "3d": the concatenation binds like the dot, and the dot is looser
    // than the arithmetic.
    let (report, _desktop) = run("x := \"a\" 1 + 2\ny := \"a\" \"b\" \"c\"\nz := 1 + 2 \"d\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("a3".into())));
    assert_eq!(report.variables.get("y"), Some(&Value::Text("abc".into())));
    assert_eq!(report.variables.get("z"), Some(&Value::Text("3d".into())));
}

#[test]
fn the_assignment_operators_of_v2_are_there() {
    // The bitwise and the shift forms of v2 are `&=`, `|=`, `^=`,
    // `<<=`, `>>=` and `>>>=`.
    let (report, _desktop) = run(concat!(
        "a := 6\na |= 2\n",
        "\nb := 6\nb &= 2\n",
        "\nc := 6\nc ^= 2\n",
        "\nd := 6\nd <<= 2\n",
        "\ne := 6\ne >>= 1\n",
        "\nf := 6\nf >>>= 1\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(6)));
    assert_eq!(report.variables.get("b"), Some(&Value::Int(2)));
    assert_eq!(report.variables.get("c"), Some(&Value::Int(4)));
    assert_eq!(report.variables.get("d"), Some(&Value::Int(24)));
    assert_eq!(report.variables.get("e"), Some(&Value::Int(3)));
    assert_eq!(report.variables.get("f"), Some(&Value::Int(3)));
}

#[test]
fn the_match_operator_of_v2_is_the_tilde_one() {
    // v2 spells the match `~=`, and it answers the place of the match:
    // it is zero when the pattern is not there.
    let (report, _desktop) = run(concat!(
        "a := \"a1\" ~= \"\\d\"\n",
        "\nb := \"abc\" ~= \"\\d\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(2)));
    assert_eq!(report.variables.get("b"), Some(&Value::Int(0)));
}

#[test]
fn a_list_can_be_picked_by_its_position() {
    let (report, _desktop) = run("MouseGetPos &x, &y\nfound := [10, 20]\nfx := found[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));
    assert_eq!(report.variables.get("fx"), Some(&Value::Int(10)));
}

#[test]
fn a_text_cannot_be_picked_by_its_position() {
    // Measured on the binary of v2.0.19 on 2026-10-30: a text has no
    // item, and `SubStr` is the way to take a letter out of one.
    let (report, _desktop) = run("letter := \"hello\"[2]");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"String\" has no property named \"__Item\".")
    );
}

#[test]
fn a_letter_is_taken_out_of_a_text_with_substr() {
    let (report, _desktop) = run("letter := SubStr(\"hello\", 2, 1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("letter"),
        Some(&Value::Text("e".into()))
    );
}

#[test]
fn a_position_that_is_not_there_is_an_error() {
    let (report, _desktop) = run("letter := \"hello\"[99]");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"String\" has no property named \"__Item\".")
    );
}

#[test]
fn a_modifier_before_a_key_is_a_modifier() {
    let (report, desktop) = run("Send \"^a\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("key".to_string(), "ctrl+a".to_string())]
    );
}

#[test]
fn a_modifier_on_its_own_is_text() {
    let (report, desktop) = run("Send \"^\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["^"]);
}

#[test]
fn a_modifier_in_braces_is_text() {
    let (report, desktop) = run("Send \"{+}\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["+"]);
}

#[test]
fn a_key_that_goes_down_stays_down() {
    let (report, desktop) = run("Send \"{a down}\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("key_down".to_string(), "a".to_string())]
    );
}

#[test]
fn a_key_that_comes_up_is_told_to() {
    let (report, desktop) = run("Send \"{a up}\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("key_up".to_string(), "a".to_string())]);
}

#[test]
fn a_mouse_button_can_be_held_down() {
    let (report, desktop) = run("Send \"{RButton down}\"\nSend \"{RButton up}\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![
            ("mouse_down".to_string(), "right".to_string()),
            ("mouse_up".to_string(), "right".to_string())
        ]
    );
}

#[test]
fn a_click_can_have_a_count() {
    let (report, desktop) = run("Click 10, 20, 2");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10,20,left,2".to_string())]
    );
}

#[test]
fn a_click_can_name_the_button_and_the_count() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the button of a
    // click is a value of the dialect, and not a word of the language:
    // the word `right` is a variable of the script.
    let (report, desktop) = run("right := \"right\"\nClick 10, 20, right, 3");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10,20,right,3".to_string())]
    );
}

#[test]
fn a_number_alone_is_the_number_of_clicks() {
    // The place is two numbers, so a number on its own is the number of
    // clicks: `Click 2` is a double click where the pointer is.
    let (report, desktop) = run("Click 2");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "0,0,left,2".to_string())]
    );
}

#[test]
fn the_components_of_a_click_may_be_in_one_string() {
    let (report, desktop) = run("Click \"100 200 Right\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "100,200,right,1".to_string())]
    );
}

#[test]
fn a_click_that_would_hold_the_button_fails_with_the_reason() {
    let (report, _desktop) = run("Click \"Down\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the pressing of the button is not carried yet.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_mouse_button_can_be_held_at_a_place() {
    let (report, desktop) =
        run("MouseMove 100, 200\nSend \"{LButton down}\"\nSend \"{LButton up}\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls.len(), 3);
}

#[test]
fn mouse_get_pos_says_where_the_pointer_is() {
    // v2 writes the place of the pointer into the variables of the
    // caller.
    let (report, _desktop) = run("MouseMove 120, 340\nMouseGetPos &x, &y");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(120)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(340)));
}

#[test]
fn a_pixel_search_finds_the_colour() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the command takes
    // seven parameters, the first two are references, and it answers one
    // when the colour is there.
    let (report, _desktop) = run("found := PixelSearch(&x, &y, 0, 0, 100, 100, \"#ff8800\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("found"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("x"), Some(&Value::Int(10)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(20)));
}

#[test]
fn a_pixel_search_that_finds_nothing_answers_zero() {
    let (report, _desktop) = run("found := PixelSearch(&x, &y, 0, 0, 100, 100, \"#ffffff\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("found"), Some(&Value::Int(0)));
    // The place of v2 is blank when the colour is not there.
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

#[test]
fn win_active_says_whether_the_window_is_active() {
    let mut desktop = Fake {
        active: Some("Steam".into()),
        ..Default::default()
    };
    let statements = parse("active := WinActive(\"Steam\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("active"), Some(&Value::Int(1)));
}

/// A window that is there but is not in front is not the active one.
#[test]
fn win_active_of_a_window_that_is_not_in_front_is_false() {
    let mut desktop = Fake {
        windows: vec!["Steam".into(), "Firefox".into()],
        active: Some("Steam".into()),
        ..Default::default()
    };
    let statements = parse("active := WinActive(\"Firefox\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("active"), Some(&Value::Int(0)));
}

/// A window that is not there is not the active one.
///
/// This is an answer and not a failure: the desktop said that the window
/// is not there, and that is the end of the question.
#[test]
fn win_active_of_a_window_that_is_not_there_is_false() {
    let mut desktop = Fake {
        windows: vec!["Steam".into()],
        active: Some("Steam".into()),
        ..Default::default()
    };
    let statements = parse("active := WinActive(\"Firefox\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("active"), Some(&Value::Int(0)));
}

/// The turn of the cases that change the mode of the titles.
///
/// The mode is a setting of the crate, and the cases of one file run in
/// threads: a case that reads the mode while another writes it would see the
/// mode of the other one.
static THE_TITLE_MODE_TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// The mode of the matching of the titles is a setting of the script,
/// and the desktop is told it.
#[test]
fn the_mode_of_the_matching_of_the_titles_is_the_one_of_the_script() {
    let _turn = THE_TITLE_MODE_TURN
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let mut desktop = Fake {
        windows: vec!["Steam Client".into()],
        ..Default::default()
    };
    let statements = parse(
        "SetTitleMatchMode(3)\nthere := WinExist(\"Steam\")\nSetTitleMatchMode(1)\nstarts := WinExist(\"Steam\")\nSetTitleMatchMode(2)\ninside := WinExist(\"Steam\")\nSetTitleMatchMode(\"RegEx\")\npattern := WinExist(\"Steam .*\")\nfolded := WinExist(\"i)steam .*\")\nSetTitleMatchMode(2)",
    )
    .expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    // The mode 3 wants the title to be the text, and it is not.
    assert_eq!(report.variables.get("there"), Some(&Value::Int(0)));
    // The mode 1 wants the title to start with the text, and it does.
    assert_eq!(report.variables.get("starts"), Some(&Value::Int(1)));
    // The mode 2 wants the title to have the text in it.
    assert_eq!(report.variables.get("inside"), Some(&Value::Int(1)));
    // The pattern is a regular expression, and the `i)` at the front of it
    // takes the case away.
    assert_eq!(report.variables.get("pattern"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("folded"), Some(&Value::Int(1)));
}

/// A window that is there is there.
#[test]
fn win_exist_says_that_a_window_that_is_there_is_there() {
    let mut desktop = Fake {
        windows: vec!["Steam".into()],
        active: Some("Steam".into()),
        ..Default::default()
    };
    let statements = parse("there := WinExist(\"Steam\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("there"), Some(&Value::Int(1)));
}

/// A window that is not there is not there.
#[test]
fn win_exist_of_a_window_that_is_not_there_is_false() {
    let mut desktop = Fake {
        windows: vec!["Steam".into()],
        active: Some("Steam".into()),
        ..Default::default()
    };
    let statements = parse("there := WinExist(\"Firefox\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("there"), Some(&Value::Int(0)));
}

/// The title of the window that has the focus.
#[test]
fn win_get_title_says_the_title_of_the_window_that_has_the_focus() {
    let mut desktop = Fake {
        windows: vec!["Steam".into(), "Firefox".into()],
        active: Some("Firefox".into()),
        ..Default::default()
    };
    let statements = parse("title := WinGetTitle()").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("title"),
        Some(&Value::Text("Firefox".into()))
    );
}

/// A desktop with no window in front has no title to give.
#[test]
fn win_get_title_without_a_window_in_front_is_empty() {
    let mut desktop = Fake::default();
    let statements = parse("title := WinGetTitle()").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("title"),
        Some(&Value::Text(String::new()))
    );
}

#[test]
fn a_window_question_that_cannot_be_answered_fails_the_script() {
    let mut desktop = Fake {
        window_answer_unknown: true,
        ..Default::default()
    };
    let statements = parse("active := WinActive(\"Steam\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    let error = report.error.unwrap_or_default();
    assert!(error.contains("cannot answer"), "{error}");
}

// -- the builtins ---------------------------------------------------------------

#[test]
fn the_numbers_have_their_functions() {
    let (report, _desktop) = run(
        "a := Abs(-3)\nb := Sqrt(16)\nc := Mod(7, 3)\nd := Floor(2.7)\ne := Ceil(2.1)\nf := Round(2.345, 2)\ng := Min(3, 1, 2)\nh := Max(3, 1, 2)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(3)));
    // AutoHotkey v2 always answers a float for a square root.
    assert_eq!(report.variables.get("b"), Some(&Value::Float(4.0)));
    assert_eq!(report.variables.get("c"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("d"), Some(&Value::Int(2)));
    assert_eq!(report.variables.get("e"), Some(&Value::Int(3)));
    // Measured on the binary of v2.0.19 on 2026-11-09: the `Round` that
    // takes a number of decimals answers a String of exactly that many
    // digits, and not a Float.
    assert_eq!(report.variables.get("f"), Some(&Value::Text("2.35".into())));
    assert_eq!(report.variables.get("g"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("h"), Some(&Value::Int(3)));
}

#[test]
fn the_is_operator_says_whether_an_object_is_of_a_class() {
    let (report, _desktop) = run(
        "class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\n\
         a := b is Beta\nc := b is Alpha\nd := b is Object\ne := 3 is Object",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("c"), Some(&Value::Int(1)));
    // Every object is an Object: it is the root of the chain.
    assert_eq!(report.variables.get("d"), Some(&Value::Int(1)));
    // A number is not an object.
    assert_eq!(report.variables.get("e"), Some(&Value::Int(0)));
}

#[test]
fn a_class_is_a_class_and_an_object_and_not_an_instance_of_itself() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `C is Class` is 1,
    // `C is Object` is 1 and `C is C` is 0 -- a class is not an instance of
    // itself -- and a prototype is not an instance of its own class either.
    let (report, _desktop) = run(
        "class Alpha {\n}\nm := Map\n\
         a := Alpha is Class\nb := Alpha is Object\nc := Alpha is Alpha\n\
         d := m is Class\ne := m is Object\nf := Alpha.Prototype is Alpha",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("b"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("c"), Some(&Value::Int(0)));
    // The class of a class type of the runtime is the same kind.
    assert_eq!(report.variables.get("d"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("e"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("f"), Some(&Value::Int(0)));
}

#[test]
fn the_is_operator_wants_a_class() {
    let (report, _desktop) = run("class Alpha {\n}\nb := Alpha()\na := b is \"Alpha\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Class but got a String.")
    );
}

#[test]
fn a_class_has_a_prototype() {
    let (report, _desktop) = run("class Alpha {\n}\np := Alpha.Prototype\nt := Type(Alpha)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("t"),
        Some(&Value::Text("Class".into()))
    );
    match report.variables.get("p") {
        Some(Value::Object(object)) => assert_eq!(object.borrow().class, "prototype"),
        other => panic!("the prototype is {other:?}"),
    }
}

#[test]
fn the_text_has_its_functions() {
    let (report, _desktop) = run(
        "a := StrLen(\"hello\")\nb := SubStr(\"hello\", 2)\nc := StrReplace(\"hello\", \"l\", \"L\")\nd := InStr(\"hello\", \"ll\")\ne := StrUpper(\"hi\")\nf := Format(\"{1} of {2}\", 2, 3)\ng := Format(\"{1:.2}\", 2.345)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(5)));
    // AutoHotkey counts the positions from one.
    assert_eq!(report.variables.get("b"), Some(&Value::Text("ello".into())));
    assert_eq!(
        report.variables.get("c"),
        Some(&Value::Text("heLLo".into()))
    );
    assert_eq!(report.variables.get("d"), Some(&Value::Int(3)));
    assert_eq!(report.variables.get("e"), Some(&Value::Text("HI".into())));
    assert_eq!(
        report.variables.get("f"),
        Some(&Value::Text("2 of 3".into()))
    );
    // Measured on the binary of v2.0.19 on 2026-11-09: the type of the
    // format is `s` when it is not written, and the precision of a String is
    // its length, so `{1:.2}` of 2.345 is the first two characters.
    assert_eq!(report.variables.get("g"), Some(&Value::Text("2.".into())));
}

#[test]
fn a_function_with_the_name_of_a_builtin_wins() {
    let (report, desktop) = run(
        "StrUpper(text) {\n  return \"mine\"\n}\nresult := StrUpper(\"hi\")\nSend \"%result%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["mine"]);
}

#[test]
fn a_builtin_variable_can_be_in_a_string() {
    // The built-in variables are names of the runtime, so a name in a
    // string finds them.
    let (report, desktop) = run("Send \"[%A_AhkVersion%]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["[2.0.19]"]);
}

#[test]
fn a_builtin_variable_of_the_date_is_answered_in_a_string() {
    // The clock of the host knows the date, and the name of the date is
    // answered with the fourteen digits of v2.
    let (report, desktop) = run("Send \"%A_Now%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types().len(), 1);
    assert_eq!(desktop.types()[0].len(), 14);
}

#[test]
fn an_item_of_a_map_may_be_written() {
    let (report, desktop) = run("m := Map()\nm[\"a\"] := 1\nv := m[\"a\"]\nSend \"%v%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

#[test]
fn an_item_of_an_array_may_be_written() {
    let (report, desktop) = run("a := Array(0)\na[1] := 5\nv := a[1]\nSend \"%v%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["5"]);
}

#[test]
fn an_item_of_an_array_that_is_not_there_may_not_be_written() {
    // An array grows with `Push` and with `InsertAt`, and not with an
    // assignment: the index has to be an item that is there.
    let (report, _desktop) = run("a := Array(0)\na[3] := 5");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Invalid index."));
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn an_item_of_a_class_of_the_script_may_not_be_written() {
    // v2.0.19 asks only the `__Item` property of the item, and the
    // meta-functions answer for the properties that are not there.
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _desktop) = run("class Box {\n}\nb := Box()\nb[\"a\"] := 1");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Box\" has no property named \"__Item\".")
    );
}

#[test]
fn a_built_in_may_be_written_as_a_call_statement() {
    // The parentheses may be left out when the answer is not needed, and
    // the name is at the start of the line. The answer is discarded.
    let (report, _desktop) = run("Abs 5");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_function_of_the_script_may_be_written_as_a_call_statement() {
    let (report, desktop) = run("Greet() {\n    Send \"hello\"\n}\nGreet");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["hello"]);
}

#[test]
fn a_name_that_is_neither_a_built_in_nor_a_function_is_a_variable_of_the_script() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a call
    // statement is not refused at the load. It is a variable of the
    // script, and v2 reads it: a variable that has no value is a variable
    // that has no value.
    let (report, _desktop) = run("Nonsense 1");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_value_that_is_not_an_object_cannot_be_called() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that
    // cannot be called is refused with the kind of it, and the failure is
    // of the class `MethodError`.
    let (report, _desktop) = run("x := 5\nx");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no method named \"Call\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_built_in_variable_that_is_a_number_cannot_be_called_either() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value of a
    // built-in variable is the value the call asks for.
    let (report, _desktop) = run("A_HotkeyInterval");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no method named \"Call\".")
    );
}

#[test]
fn the_object_literal_of_v2_makes_an_object() {
    let (report, desktop) = run("x := {a: 1, b: 2}\nSend \"%x.a%%x.b%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["12"]);
}

#[test]
fn the_empty_object_literal_is_an_object_of_nothing() {
    let (report, desktop) = run("x := {}\nSend \"%x.__Class%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["Object"]);
}

#[test]
fn the_names_of_the_properties_of_an_object_do_not_know_their_case() {
    let (report, desktop) = run("x := {a: 1}\nSend \"%x.A%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

#[test]
fn the_name_of_a_property_of_an_object_literal_must_be_a_name() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the script does
    // not load at all.
    let error = parse("x := {\"a b\": 1}").expect_err("does not parse");
    assert_eq!(error.message, "Invalid property name in object literal.");
    assert_eq!(error.line, 1);
}

// -- a body that has no head, and a chain of bases that closes -----------------
//
// Measured on the binaries of v2.0.31 and of v1.1.37.02 on 2026-11-09. The
// two versions changed the answer of the first group: v2.0.19 let the line
// pass and ran nothing, and the changelog of v2.0.20 records the fix. The
// second group is the fix of v2.0.27, and the v1 runs the same script, so the
// rule is the dialect's.

#[test]
fn the_else_without_an_if_is_refused_by_the_v2() {
    let error = parse("else\n    FileAppend \"ran\", \"*\"\n").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Else\"");
}

#[test]
fn the_catch_without_a_try_is_refused_by_the_v2() {
    let error = parse("catch\n    FileAppend \"ran\", \"*\"\n").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Catch\"");
}

#[test]
fn the_finally_without_a_try_is_refused_by_the_v2() {
    let error = parse("finally\n    FileAppend \"ran\", \"*\"\n").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Finally\"");
}

#[test]
fn the_v1_says_what_the_body_was_missing() {
    // The v1 does not name the word it found: it says which head is absent.
    for (source, message) in [
        ("else\n    FileAppend, ran, *\n", "ELSE with no matching IF"),
        ("catch\n    FileAppend, ran, *\n", "CATCH with no matching TRY"),
        (
            "finally\n    FileAppend, ran, *\n",
            "FINALLY with no matching TRY or CATCH",
        ),
    ] {
        let error = ahk::parse_with(source, ahk::dialect::v1()).expect_err("does not parse");
        assert_eq!(error.message, message);
    }
}

#[test]
fn the_bodies_that_have_a_head_still_run() {
    // The refusal is for the word on its own: the legitimate forms reach
    // their own head and are untouched by it.
    let (report, _) = run(
        "if (1)\n    FileAppend \"a\", \"*\"\nelse\n    FileAppend \"b\", \"*\"\n\
         try {\n    FileAppend \"t\", \"*\"\n} catch {\n    FileAppend \"c\", \"*\"\n}\
         finally {\n    FileAppend \"f\", \"*\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a", "t", "f"]);
}

#[test]
fn a_class_that_extends_itself_is_refused() {
    let error = parse("class A extends A\n{\n}\n").expect_err("does not parse");
    assert_eq!(error.message, "Invalid base class.\n     Specifically: A");
}

#[test]
fn the_reason_names_the_class_and_not_the_spelling_of_the_base() {
    let error = parse("class A extends a\n{\n}\n").expect_err("does not parse");
    assert_eq!(error.message, "Invalid base class.\n     Specifically: A");
}

#[test]
fn a_pair_that_is_each_others_base_is_refused() {
    let error =
        parse("class A extends B\n{\n}\nclass B extends A\n{\n}\n").expect_err("does not parse");
    assert_eq!(error.message, "Invalid base class.\n     Specifically: B");
}

#[test]
fn a_trio_that_closes_on_itself_is_refused() {
    let error = parse("class A extends B\n{\n}\nclass B extends C\n{\n}\nclass C extends A\n{\n}\n")
        .expect_err("does not parse");
    assert_eq!(error.message, "Invalid base class.\n     Specifically: C");
}

#[test]
fn a_chain_of_bases_that_does_not_close_is_a_class() {
    let (report, _) = run("class A\n{\n}\nclass B extends A\n{\n}\nFileAppend \"ran\", \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ran"]);
}

#[test]
fn the_v1_runs_a_class_that_extends_itself() {
    // Measured on the binary of v1.1.37.02 on 2026-11-09: the class is its
    // own base and the script runs.
    let statements = ahk::parse_with(
        "class A extends A\n{\n}\nFileAppend, ran, *\n",
        ahk::dialect::v1(),
    )
    .expect("the v1 parses it");
    let mut desktop = Fake::default();
    let report = Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(ahk::dialect::v1())
        .run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ran"]);
}

#[test]
fn a_property_of_an_object_literal_needs_its_colon() {
    let error = parse("x := {a 1}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
    assert_eq!(error.line, 1);
}

/// The literal of an object of the v2 is a list of parameters, and the number
/// of them has to be even.
///
/// Measured on the binary of v2.0.19 on 2026-11-04: the literal is a list of
/// parameters: the key of a property and its value are two, and a parameter
/// that is in blank is one. The `x := {, 1}` is an object of no property, and
/// the `x := {, b: 1}`, the `x := {, 1, 2}`, the `x := {b: 1, , c: 2}`, the
/// `x := {b: 1, c}` and the `x := {1, 2}` are refused with "Missing
/// "propertyname:" in object literal.".
#[test]
fn the_literal_of_an_object_of_the_v2_is_a_list_of_parameters() {
    let (report, _) = run("x := {, 1}\nFileAppend ObjOwnPropCount(x), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0".to_string()]);
    let error = parse("x := {, b: 1}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
    let error = parse("x := {, 1, 2}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
    let error = parse("x := {b: 1, , c: 2}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
    let error = parse("x := {b: 1, c}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
    let error = parse("x := {1, 2}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
}

/// The blank at the end of the literal of an object of the v2 is accepted.
///
/// Measured on the binary of v2.0.19 on 2026-11-04: the `x := {a: 1,}` is
/// an object of one property, and the `x := {,}` and the `x := {a: 1,,}` are
/// objects of no property. The v1 refuses the comma at the end of the
/// literal with "Blank parameter".
#[test]
fn the_blank_at_the_end_of_the_literal_of_the_v2_is_accepted() {
    let (report, _) = run("x := {a: 1,}\nFileAppend ObjOwnPropCount(x), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1".to_string()]);
    let (report, _) = run("x := {,}\nFileAppend ObjOwnPropCount(x), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0".to_string()]);
    let (report, _) = run("x := {a: 1,,}\nFileAppend ObjOwnPropCount(x), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1".to_string()]);
}

/// The value of the literal of an object of the v2 that has no name is
/// evaluated, and it is dropped.
///
/// Measured on the binary of v2.0.19 on 2026-11-04: the `x := {, f()}` calls
/// f, and the object of it has no property.
#[test]
fn the_value_of_the_literal_of_the_v2_that_has_no_name_is_evaluated_and_dropped() {
    let (report, _) = run(
        "n := 0\nf() {\n    global n\n    n := n + 1\n    return 0\n}\n\
         x := {, f()}\nFileAppend n, \"*\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1".to_string()]);
}

#[test]
fn the_name_of_a_property_of_an_object_literal_may_be_a_keyword() {
    let (report, desktop) = run("x := {class: 1}\nSend \"%x.class%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

#[test]
fn a_comma_at_the_end_of_a_list_is_allowed() {
    // Measured on the binary of v2.0.19 on 2026-10-30: v2 takes a comma at
    // the end of the list of an array, of an object and of the arguments of
    // a call.
    let (report, _desktop) = run(
        "a := [1, 2,]\nx := {a: 1,}\ny := StrLen(\"ab\",)\nMsgBox \"%a.Length%\"\nMsgBox \"%x.a%\"\nMsgBox \"%y%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2", "1", "2"]);
}

#[test]
fn the_arrow_function_of_v2_is_a_function() {
    let (report, _desktop) = run("f := (a) => a + 1\nMsgBox f(1)");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the arrow
    // function of v2 is a function, and it takes the parameters it names.
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_arrow_function_without_parentheses_is_a_function() {
    let (report, _desktop) = run("f := a => a + 1\nMsgBox f(1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_arrow_function_closes_over_the_scope_where_it_was_written() {
    let (report, _desktop) =
        run("Outer() {\n    n := 10\n    f := (a) => a + n\n    return f(1)\n}\nMsgBox Outer()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["11"]);
}

#[test]
fn the_function_that_is_written_inside_another_one_closes_over_it() {
    let (report, _desktop) = run(
        "Outer() {\n    n := 10\n    Inner() {\n        return n\n    }\n    return Inner()\n}\nMsgBox Outer()",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: a function that is
    // written inside another one sees the variables of the one around it.
    assert_eq!(report.output, vec!["10"]);
}

#[test]
fn the_function_that_is_written_inside_another_one_is_there_before_it_is_defined() {
    let (report, _desktop) = run(
        "Outer() {\n    return Inner()\n    Inner() {\n        return \"inner\"\n    }\n}\nMsgBox Outer()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["inner"]);
}

#[test]
fn the_closure_writes_the_variable_of_the_scope_it_closes_over() {
    let (report, _desktop) = run(
        "Outer() {\n    n := 10\n    Inner() {\n        n := 20\n    }\n    Inner()\n    return n\n}\nMsgBox Outer()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["20"]);
}

#[test]
fn the_assignment_of_v2_is_an_expression() {
    let (report, _desktop) = run("x := (y := 1)\nMsgBox x\nMsgBox y");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the assignment of
    // v2 is an expression, and its value is the value that was written.
    assert_eq!(report.output, vec!["1", "1"]);
}

#[test]
fn the_equal_sign_of_v1_is_not_the_assignment_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the script does
    // not load at all.
    let error = parse("x = 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Syntax error. Did you mean to use \":=\"?\n     Specifically: x = 1"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn a_parameter_name_that_is_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name that
    // begins with a number is refused the way the lexer refuses it.
    let error = parse("f(1) {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This variable name starts with a number, which is not allowed:\n\"1\""
    );
}

#[test]
fn a_parameter_name_that_is_a_text_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name that is
    // missing is named, and the line of the script says where it was.
    let error = parse("f(\"x\") {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing parameter name.\n     Specifically: f(\"x\") {"
    );
}

#[test]
fn a_parameter_name_that_is_a_reserved_word_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let error = parse("f(if) {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a variable name:\n\"if\""
    );
}

#[test]
fn a_parameter_name_that_is_a_comma_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let error = parse("f(a, b,) {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing parameter name.\n     Specifically: f(a, b,) {"
    );
}

#[test]
fn a_class_name_that_is_a_reserved_word_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let error = parse("class if {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a class name:\n\"if\""
    );
}

#[test]
fn a_class_name_that_is_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let error = parse("class 1 {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This class name starts with a number, which is not allowed:\n\"1\""
    );
}

#[test]
fn a_block_that_is_not_closed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the brace that is
    // not closed is named, and the block that is empty names the
    // construct that opened it.
    let error = parse("f(a) {\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: f(a) {");
}

#[test]
fn a_class_that_is_not_closed_names_the_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class that was
    // open is the one that is named with the brace that is not closed.
    let error = parse("class Foo {\n    x := 1\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: Foo");
}

#[test]
fn a_switch_that_is_not_closed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: v2 looks for the
    // brace that closes the switch before it reads the cases. The switch
    // that has a statement in it names nothing.
    let error = parse("Switch 1 {\n    x := 1\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"");
    // Measured on the binary of v2.0.19 on 2026-11-01: the switch that is
    // empty names the statement of it.
    let error = parse("Switch 1 {").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"}\"\n     Specifically: Switch 1 {"
    );
}

#[test]
fn a_case_needs_its_colon() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the colon of a case
    // is not optional, and the line of the script says where it was.
    let error = parse("Switch 1 {\n    case 1\n        x := 1\n}").expect_err("does not parse");
    assert_eq!(error.message, "Missing \":\"\n     Specifically: case 1");
}

#[test]
fn a_statement_that_is_not_a_case_is_refused() {
    let error = parse("Switch 1 {\n    x := 1\n}").expect_err("does not parse");
    assert_eq!(error.message, "Expected Case/Default.");
}

#[test]
fn a_bracket_that_is_not_closed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let error = parse("x := (1\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \")\"\n     Specifically: x := (1");
}

#[test]
fn a_token_that_is_not_wanted_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the token is named,
    // and the line of the script says where it was.
    let error = parse("x := 1 )").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected \")\"\n     Specifically: x := 1 )"
    );
}

#[test]
fn the_static_variable_of_a_class_is_the_class_that_has_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a variable that
    // belongs to the class is the class that has it, and an instance does
    // not see it.
    let (report, _desktop) = run("class Thing {\n    static x := 1\n}\nMsgBox Thing.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    let (report, _desktop) = run("class Thing {\n    static x := 1\n}\nt := Thing()\nMsgBox t.x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Thing\" has no property named \"x\".")
    );
}

#[test]
fn the_class_of_v2_is_initialised_when_it_is_first_used() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class is
    // initialised when it is first used, and the `__New` that is static is
    // the one that runs: once.
    let (report, _desktop) = run(
        "count := 0\nclass Thing {\n    static __New() {\n        global count\n        count := count + 1\n    }\n}\nt := Thing()\nu := Thing()\nMsgBox count",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_method_of_a_class_is_read_only() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let (report, _desktop) = run(
        "class Thing {\n    static Say() {\n        return \"said\"\n    }\n}\nThing.Say := () => 1",
    );
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Property is read-only."));
}

#[test]
fn the_concatenation_of_an_object_is_refused() {
    let (report, _desktop) = run("x := {}\nMsgBox \"a\" . x");
    assert!(!report.ok);
    // Measured on the binary of v2.0.19 on 2026-10-31: the concatenation
    // wants a text or a number, and the message names the kind of the value
    // that arrived.
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got an Object.")
    );
}

#[test]
fn the_concatenation_of_an_array_is_refused() {
    let (report, _desktop) = run("x := [1]\nMsgBox \"a\" . x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got an Array.")
    );
}

#[test]
fn the_concatenation_of_a_function_is_refused() {
    let (report, _desktop) = run("x := () => 1\nMsgBox \"a\" . x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got a Func.")
    );
}

#[test]
fn the_concatenation_of_a_number_is_a_text() {
    let (report, _desktop) = run("MsgBox \"a\" 1");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a1"]);
}

#[test]
fn the_maths_refuses_an_object() {
    let (report, _desktop) = run("x := {}\nMsgBox 1 + x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got an Object.")
    );
}

#[test]
fn the_maths_refuses_an_array() {
    let (report, _desktop) = run("x := [1]\nMsgBox 1 + x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got an Array.")
    );
}

#[test]
fn the_maths_refuses_a_text_that_is_not_a_number() {
    let (report, _desktop) = run("x := \"a\"\nMsgBox 1 + x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got a String.")
    );
}

#[test]
fn the_parenthesis_may_open_an_argument_of_a_command() {
    let (report, _desktop) = run("MsgBox (1 + 2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_name_and_a_parenthesis_with_a_space_are_not_a_call() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the space before
    // the parenthesis tells a call from the concatenation of v2.
    let (report, _desktop) = run("x := 1\nMsgBox x (2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["12"]);
}

#[test]
fn the_whole_number_of_the_bitwise_operations_is_a_number_first() {
    let (report, _desktop) = run("MsgBox 1 & \"2\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_number_with_a_point_is_not_a_whole_number() {
    let (report, _desktop) = run("MsgBox 1 & 1.5");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected an Integer but got a Float.")
    );
}

#[test]
fn the_hex_of_v2_is_a_whole_number() {
    let (report, desktop) = run("x := 0xE9\nSend \"%x%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["233"]);
}

#[test]
fn the_hex_of_v2_wraps_the_way_the_machine_does() {
    let (report, desktop) = run("x := 0xFFFFFFFFFFFFFFFF\nSend \"%x%\"");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the hex of v2 is
    // a number of the machine, and it wraps the way the machine does.
    assert_eq!(desktop.types(), vec!["-1"]);
}

#[test]
fn a_number_with_an_exponent_is_a_float() {
    let (report, desktop) = run("x := 1e3\nSend \"%x%\"");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the exponent makes
    // the number a float, and a float keeps its point.
    assert_eq!(desktop.types(), vec!["1000.0"]);
}

#[test]
fn a_number_with_a_point_is_a_float() {
    let (report, desktop) = run("x := 3.0\nSend \"%x%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3.0"]);
}

#[test]
fn a_name_that_begins_with_a_number_does_not_parse() {
    // The check is a check of the load, the way v2 has it: the script
    // does not run at all.
    let error = parse("x := 12abc").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This variable name starts with a number, which is not allowed:\n\"12abc\""
    );
    assert_eq!(error.line, 1);
}

#[test]
fn an_item_may_be_written_with_an_operator() {
    let (report, desktop) =
        run("m := Map()\nm[\"a\"] := 1\nm[\"a\"] += 2\nv := m[\"a\"]\nSend \"%v%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3"]);
}

// -- the array and the for loop ---------------------------------------------------

#[test]
fn an_array_literal_is_an_array() {
    let (report, desktop) = run("a := [\"x\", \"y\"]\nSend \"%a.Length%\"\nSend a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["2", "x"]);
}

#[test]
fn an_empty_array_literal_is_an_array_of_nothing() {
    let (report, desktop) = run("a := []\nSend \"%a.Length%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["0"]);
}

#[test]
fn an_array_literal_holds_the_values_it_is_given() {
    let (report, desktop) = run("x := 2\na := [x + 1, \"n\" . x]\nSend a[1]\nSend a[2]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["3", "n2"]);
}

#[test]
fn a_for_walks_the_values_of_an_array() {
    let (report, desktop) = run("for v in [\"a\", \"b\"] {\n  Send v\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a", "b"]);
}

#[test]
fn a_for_with_two_variables_walks_the_places_and_the_values() {
    let (report, desktop) = run("for i, v in [\"a\", \"b\"] {\n  Send \"%i%:%v%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1:a", "2:b"]);
}

#[test]
fn a_for_walks_the_keys_of_a_map() {
    // The enumerator of a map gives the key, and the value under it
    // when the loop wants two.
    let (report, desktop) = run("m := Map()\nm.Set(\"a\", 1)\nfor v in m {\n  Send v\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a"]);
}

#[test]
fn a_for_with_two_variables_walks_the_keys_and_the_values_of_a_map() {
    let (report, desktop) =
        run("m := Map()\nm.Set(\"a\", 1)\nfor k, v in m {\n  Send \"%k%=%v%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a=1"]);
}

#[test]
fn a_for_may_leave_the_key_out() {
    let (report, desktop) = run("m := Map()\nm.Set(\"a\", 1)\nfor , v in m {\n  Send v\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

#[test]
fn a_for_counts_its_turns() {
    let (report, desktop) = run("for v in [\"a\", \"b\"] {\n  Send \"%A_Index%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1", "2"]);
}

#[test]
fn a_break_leaves_a_for() {
    let (report, desktop) =
        run("for v in [\"a\", \"b\", \"c\"] {\n  If v = \"b\" {\n    Break\n  }\n  Send v\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a"]);
}

#[test]
fn a_continue_skips_the_rest_of_a_for() {
    let (report, desktop) =
        run("for v in [\"a\", \"b\"] {\n  If v = \"a\" {\n    Continue\n  }\n  Send v\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["b"]);
}

#[test]
fn a_call_statement_may_take_an_array() {
    // The space is the reason: `f[1, 2]` is the item of the value `f`,
    // and `f [1, 2]` is a call whose argument is the array.
    let (report, _desktop) = run("f(a) {\n  return a.Length\n}\nf [1, 2]");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_for_gives_the_variables_back() {
    let (report, desktop) = run("v := \"before\"\nfor v in [\"a\"] {\n}\nSend v");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["before"]);
}

#[test]
fn a_for_leaves_no_variable_behind() {
    // A name that had no value before the loop has no value after it.
    let (report, _desktop) = run("for v in [\"a\"] {\n}\nSend v");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
}

#[test]
fn a_for_walks_what_a_class_of_the_script_enumerates() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the enumerator of
    // v2 is something that can be called, and a function of the script is
    // one: it writes the values into the variables of the loop.
    let (report, desktop) = run(
        "values := [\"a\", \"b\"]\nindex := 0\nNext(&v) {\n  global values, index\n  if index >= values.Length\n    return false\n  v := values[index + 1]\n  index += 1\n  return true\n}\nclass Bag {\n  __Enum(n) {\n    return Next\n  }\n}\nfor v in Bag() {\n  Send v\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a", "b"]);
}

#[test]
fn an_enumerator_that_is_a_list_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the enumerator of
    // v2 is something that can be called, and a list is not one.
    let (report, _desktop) =
        run("class Bag {\n  __Enum(n) {\n    return [\"a\", \"b\"]\n  }\n}\nfor v in Bag() {\n}");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Value not enumerable."));
}

#[test]
fn an_enumerator_that_is_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is
    // not an object at all is a mismatch of the type.
    let (report, _desktop) =
        run("class Bag {\n  __Enum(n) {\n    return 5\n  }\n}\nfor v in Bag() {\n}");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Type mismatch."));
}

#[test]
fn a_for_gives_the_variables_back_inside_a_function() {
    let (report, desktop) =
        run("f() {\n  v := \"before\"\n  for v in [\"a\"] {\n  }\n  return v\n}\nSend f()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["before"]);
}

#[test]
fn a_for_over_a_value_that_cannot_be_enumerated_fails_with_the_reason() {
    let (report, _desktop) = run("for v in 42 {\n}");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Value not enumerable."));
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_for_over_an_object_without_an_enumerator_fails_with_the_reason() {
    let (report, _desktop) = run("class Box {\n}\nfor v in Box() {\n}");
    assert!(!report.ok);
    let error = report.error.unwrap_or_default();
    assert_eq!(error, "Value not enumerable.");
}

#[test]
fn a_for_that_runs_too_long_fails_with_the_reason() {
    let (report, _desktop) = run("a := []\nLoop 1001 {\n  a.Push(1)\n}\nfor v in a {\n}");
    assert!(!report.ok);
    let error = report.error.unwrap_or_default();
    assert!(
        error.contains("the loop ran more than 1000 times"),
        "{error}"
    );
}

// -- the error handling --------------------------------------------------------

#[test]
fn a_try_catches_the_error_of_the_runtime() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  MsgBox NoSuchVariable\n",
        "} catch {\n",
        "  MsgBox \"caught\"\n",
        "}\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["caught", "after"]);
}

#[test]
fn a_variable_of_a_function_that_has_no_value_is_a_local_one() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) =
        run("Helper()\n\nHelper() {\n    for answer in [\"a\"] {\n    }\n    MsgBox answer\n}");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
}

#[test]
fn a_catch_binds_the_value_that_was_thrown() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  MsgBox NoSuchVariable\n",
        "} catch as e {\n",
        "  MsgBox e.Message\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["This global variable has not been assigned a value."]
    );
}

#[test]
fn the_else_of_a_try_runs_when_nothing_was_thrown() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  MsgBox \"in the try\"\n",
        "} catch {\n",
        "  MsgBox \"in the catch\"\n",
        "} else {\n",
        "  MsgBox \"in the else\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["in the try", "in the else"]);
}

#[test]
fn the_finally_of_a_try_always_runs() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  MsgBox \"in the try\"\n",
        "} catch {\n",
        "  MsgBox \"in the catch\"\n",
        "} finally {\n",
        "  MsgBox \"in the finally\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["in the try", "in the finally"]);
}

#[test]
fn a_thrown_value_is_not_an_error_and_a_catch_of_error_does_not_take_it() {
    // Measured on the binary of v2.0.19 on 2026-10-29: a catch without a
    // class takes the class `Error`, and a text that is thrown is not one.
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  throw \"the error of the script\"\n",
        "} catch {\n",
        "  MsgBox \"in the catch\"\n",
        "}\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Unhandled exception.\n     Specifically: the error of the script")
    );
}

#[test]
fn a_catch_of_any_takes_a_thrown_value() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  throw \"the error of the script\"\n",
        "} catch Any as e {\n",
        "  MsgBox e\n",
        "}\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the error of the script", "after"]);
}

#[test]
fn the_message_of_an_uncaught_error_is_the_message_of_the_error() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the dialog of an
    // uncaught error shows the message of the error, and the report of the
    // runner carries the same text, so a tool that reads it learns what the
    // script threw instead of a generic line. A value that is not an error
    // keeps the generic line.
    let (report, _desktop) = run("f() {\n    throw Error(\"boom\")\n}\nf()");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("boom"));
    let (report, _desktop) = run("throw ValueError(\"bad value\")");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("bad value"));
    let (report, _desktop) = run("throw {a: 1}");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Unhandled exception."));
}

#[test]
fn a_throw_without_a_value_throws_again_what_the_catch_took() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  throw \"the error of the script\"\n",
        "} catch Any as e {\n",
        "  throw\n",
        "}\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Unhandled exception.\n     Specifically: the error of the script")
    );
}

#[test]
fn the_error_of_the_runtime_that_nobody_catches_is_the_error_of_the_run() {
    let (report, _desktop) = run("MsgBox NoSuchVariable\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
}

#[test]
fn a_catch_of_a_class_that_the_tree_does_not_know_fails_with_the_reason() {
    let (report, _desktop) = run(concat!(
        "try {\n",
        "  MsgBox NoSuchVariable\n",
        "} catch NoSuchClass {\n",
        "  MsgBox \"caught\"\n",
        "}\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the class of a catch is not a class of this runtime: NoSuchClass")
    );
}

#[test]
fn the_names_of_the_variables_are_the_names_of_the_dialect() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the dialect does
    // not know the case of the names.
    let (report, _) = run("x := 1\nMsgBox X\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_variable_written_in_another_case_is_the_same_variable() {
    let (report, _) = run("x := 1\nX := 2\nMsgBox x\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn a_parameter_read_in_another_case_is_the_same_parameter() {
    let (report, _) = run("f(p) {\n  return P\n}\nMsgBox f(\"the parameter\")\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the parameter"]);
}

#[test]
fn a_property_that_is_not_there_names_the_type_of_the_value() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) = run("x := 5\nMsgBox x.nothing");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no property named \"nothing\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_method_that_is_not_there_names_the_type_of_the_value() {
    let (report, _) = run("x := 5\nMsgBox x.nothing()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no method named \"nothing\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn an_item_that_is_not_there_names_the_type_of_the_value() {
    let (report, _) = run("x := 5\nMsgBox x[1]");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no property named \"__Item\".")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn the_refusal_of_a_text_names_the_type_of_the_value() {
    let (report, _) = run("x := \"abc\"\nMsgBox x.nothing");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"String\" has no property named \"nothing\".")
    );
}

#[test]
fn the_refusal_of_an_object_names_the_class_of_the_script() {
    let (report, _) = run("class Thing {\n}\nt := Thing()\nMsgBox t.nothing");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Thing\" has no property named \"nothing\".")
    );
}

#[test]
fn the_refusal_of_a_class_type_names_the_class() {
    let (report, _) = run("m := Map()\nMsgBox m.nothing");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Map\" has no property named \"nothing\".")
    );
}

#[test]
fn the_write_of_a_property_that_is_not_there_names_the_type() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the write is
    // refused the same way the read is.
    let (report, _) = run("x := 5\nx.nothing := 2");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no property named \"nothing\".")
    );
}

#[test]
fn the_write_of_an_item_that_is_not_there_names_the_type() {
    let (report, _) = run("x := 5\nx[1] := 2");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no property named \"__Item\".")
    );
}

#[test]
fn the_write_of_a_property_of_a_class_of_the_script_makes_the_property() {
    // Measured on the binary of v2.0.19 on 2026-10-30: v2 does not refuse
    // the write of a property that the class does not declare, and the
    // property is there afterwards.
    let (report, _) = run("class Thing {\n}\nt := Thing()\nt.nothing := 2\nMsgBox t.nothing");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn a_method_of_the_script_is_a_value() {
    // Measured on the binary of v2.0.19 on 2026-10-30: a method is
    // handed over as a function whose first parameter is `this`.
    let (report, _) = run(
        "class Thing {\n  Method() {\n    return \"the answer\"\n  }\n}\nt := Thing()\nf := t.Method\nMsgBox \"%f.Name% %f.MinParams% %f.MaxParams%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Thing.Prototype.Method 1 1"]);
}

#[test]
fn a_name_that_is_a_statement_calls_the_method() {
    // Measured on the binary of v2.0.19 on 2026-10-29: a name that is a
    // statement is a call, and the parentheses may be left out.
    let (report, _) =
        run("class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
}

#[test]
fn a_name_that_is_a_statement_of_a_property_says_that_it_is_a_method() {
    // Measured on the binary of v2.0.19 on 2026-10-29: `m.Count` is a
    // call of a method, and `Count` is a property.
    let (report, _) = run("m := Map()\nm.Count");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Map\" has no method named \"Count\".")
    );
}

#[test]
fn an_item_that_is_a_statement_is_read() {
    // Measured on the binary of v2.0.19 on 2026-10-29: `m["a"]` reads the
    // item, and the value it leaves behind is not used.
    let (report, _) = run("m := Map()\nm[\"a\"]");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Item has no value."));
}

#[test]
fn the_call_of_a_function_is_a_value() {
    // Measured on the binary of v2.0.19 on 2026-10-29.
    let (report, _) = run("f := Abs.Call\nMsgBox f.Name");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Func.Prototype.Call"]);
}

#[test]
fn a_method_held_in_a_variable_is_called_with_the_object() {
    let (report, desktop) = run(
        "class Thing {\n  Method() {\n    return \"the answer\"\n  }\n}\nt := Thing()\nf := t.Method\nSend f(t)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["the answer"]);
}

#[test]
fn a_method_held_in_a_variable_without_the_object_fails() {
    let (report, _) = run(
        "class Thing {\n  Method() {\n    return \"the answer\"\n  }\n}\nt := Thing()\nf := t.Method\nSend f()",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
}

#[test]
fn the_method_of_the_class_is_not_a_value() {
    // A class is a value of its own kind, and v2 does not hand its
    // methods over.
    let (report, _) = run("class Thing {\n  Method() {\n    return 1\n  }\n}\nMsgBox Thing.Method");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Class\" has no property named \"Method\".")
    );
}

#[test]
fn a_function_says_whether_it_is_built_in() {
    // `IsBuiltIn` and `IsVariadic` are properties of v2, and not
    // methods.
    let (report, _) = run("f := Abs\nMsgBox \"%f.IsBuiltIn%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn an_error_carries_its_message() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) =
        run("try {\n    throw Error(\"the message\")\n} catch as e {\n    MsgBox e.Message\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the message"]);
}

#[test]
fn an_error_carries_what_and_extra() {
    let (report, _) = run(
        "try {\n    throw Error(\"the message\", \"the what\", \"the extra\")\n} catch as e {\n    MsgBox \"%e.What% %e.Extra%\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the what the extra"]);
}

#[test]
fn an_error_thrown_in_a_function_names_it() {
    let (report, _) = run(
        "try {\n    Helper()\n} catch as e {\n    MsgBox e.What\n}\n\nHelper() {\n    throw Error(\"the message\")\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Helper"]);
}

#[test]
fn the_extra_of_a_missing_parameter_is_its_name() {
    let (report, _) = run(
        "class Thing {\n  Method(a) {\n    return a\n  }\n}\ntry {\n    t := Thing()\n    t.Method()\n} catch as e {\n    MsgBox e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a"]);
}

#[test]
fn the_extra_of_too_many_arguments_is_the_method() {
    let (report, _) = run(
        "class Thing {\n  Method(a) {\n    return a\n  }\n}\ntry {\n    t := Thing()\n    t.Method(1, 2)\n} catch as e {\n    MsgBox e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Thing.Prototype.Method"]);
}

#[test]
fn the_init_of_a_class_runs_when_the_instance_is_made() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) = run(concat!(
        "class Thing {\n",
        "  __Init() {\n",
        "    MsgBox \"init\"\n",
        "  }\n",
        "}\n",
        "t := Thing()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["init"]);
}

#[test]
fn the_init_of_a_class_runs_before_the_new() {
    let (report, _) = run(concat!(
        "class Thing {\n",
        "  __New() {\n",
        "    MsgBox \"new\"\n",
        "  }\n",
        "  __Init() {\n",
        "    MsgBox \"init\"\n",
        "  }\n",
        "}\n",
        "t := Thing()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["init", "new"]);
}

#[test]
fn the_init_of_the_derived_replaces_the_one_of_the_base() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  __Init() {\n",
        "    MsgBox \"base init\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  __Init() {\n",
        "    MsgBox \"derived init\"\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["derived init"]);
}

#[test]
fn the_init_of_the_base_runs_when_the_derived_has_none() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  __Init() {\n",
        "    MsgBox \"base init\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "}\n",
        "d := Derived()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["base init"]);
}

#[test]
fn the_init_of_the_base_runs_when_the_derived_calls_super() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  __Init() {\n",
        "    MsgBox \"base init\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  __Init() {\n",
        "    MsgBox \"derived init\"\n",
        "    super.__Init()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["derived init", "base init"]);
}

#[test]
fn the_variables_of_the_base_are_not_assigned_when_the_derived_has_an_init() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  value := \"the value of the base\"\n",
        "}\n",
        "class Derived extends Base {\n",
        "  __Init() {\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.value\n",
    ));
    assert!(!report.ok);
    assert!(
        report.error.as_deref().unwrap_or("").contains("value"),
        "{:?}",
        report.error
    );
}

#[test]
fn the_variables_of_the_base_are_assigned_when_the_derived_calls_super() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  value := \"the value of the base\"\n",
        "}\n",
        "class Derived extends Base {\n",
        "  __Init() {\n",
        "    super.__Init()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.value\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the value of the base"]);
}

#[test]
fn the_init_of_a_class_runs_for_every_instance() {
    let (report, _) = run(concat!(
        "class Thing {\n",
        "  __Init() {\n",
        "    MsgBox \"init\"\n",
        "  }\n",
        "}\n",
        "a := Thing()\n",
        "b := Thing()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["init", "init"]);
}

#[test]
fn the_init_of_a_class_does_not_run_when_the_class_is_only_referenced() {
    let (report, _) = run(concat!(
        "class Thing {\n",
        "  __Init() {\n",
        "    MsgBox \"init\"\n",
        "  }\n",
        "}\n",
        "x := Thing\n",
        "MsgBox \"after the reference\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["after the reference"]);
}

#[test]
fn a_super_calls_the_method_of_the_base() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  m() {\n",
        "    return \"the derived and \" . super.m()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the derived and the base"]);
}

#[test]
fn a_super_gives_the_base_the_same_this() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return this.value\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  m() {\n",
        "    this.value := \"the value\"\n",
        "    return super.m()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the value"]);
}

#[test]
fn a_super_in_a_grandchild_asks_the_middle() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Middle extends Base {\n",
        "  m() {\n",
        "    return \"the middle and \" . super.m()\n",
        "  }\n",
        "}\n",
        "class Derived extends Middle {\n",
        "  m() {\n",
        "    return \"the derived and \" . super.m()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["the derived and the middle and the base"]
    );
}

#[test]
fn a_super_reads_the_property_of_the_base_through_its_getter() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  __Get(name, params) {\n",
        "    return \"the value of \" . name\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  m() {\n",
        "    return super.value\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the value of value"]);
}

#[test]
fn a_super_asks_the_base_for_the_item() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  __Item(key) {\n",
        "    return \"the item \" . key\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  m() {\n",
        "    return super[\"value\"]\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the item value"]);
}

#[test]
fn a_super_is_the_call_of_the_base() {
    // `super()` is `super.call()`.
    let (report, _) = run(concat!(
        "class Base {\n",
        "  Call(x) {\n",
        "    return \"called with \" . x\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  m() {\n",
        "    return super(\"the argument\")\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["called with the argument"]);
}

#[test]
fn a_super_in_a_static_method_asks_the_base() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  static m() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  static m() {\n",
        "    return super.m()\n",
        "  }\n",
        "}\n",
        "MsgBox Derived.m()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the base"]);
}

#[test]
fn a_base_that_has_not_the_method_refuses_with_the_name_of_the_class() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  n() {\n",
        "    return super.n()\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.n()\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Derived\" has no method named \"n\".")
    );
}

#[test]
fn a_class_without_a_base_refuses_with_its_own_name() {
    let (report, _) = run(concat!(
        "class Alone {\n",
        "  m() {\n",
        "    return super.m()\n",
        "  }\n",
        "}\n",
        "a := Alone()\n",
        "MsgBox a.m()\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Alone\" has no method named \"m\".")
    );
}

#[test]
fn a_base_that_has_not_the_property_refuses_with_the_name_of_the_class() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  n() {\n",
        "    return super.value\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.n()\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Derived\" has no property named \"value\".")
    );
}

#[test]
fn an_inherited_method_asks_the_base_of_the_class_that_defined_it() {
    let (report, _) = run(concat!(
        "class Base {\n",
        "  m() {\n",
        "    return super.n()\n",
        "  }\n",
        "  n() {\n",
        "    return \"the base\"\n",
        "  }\n",
        "}\n",
        "class Derived extends Base {\n",
        "  n() {\n",
        "    return \"the derived\"\n",
        "  }\n",
        "}\n",
        "d := Derived()\n",
        "MsgBox d.m()\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Derived\" has no method named \"n\".")
    );
}

#[test]
fn a_global_in_a_function_writes_the_global_of_the_script() {
    // Measured on the binary of v2.0.19 on 2026-10-30.
    let (report, _) = run(concat!(
        "g := 0\n",
        "f() {\n",
        "  global g\n",
        "  g := 1\n",
        "  return g\n",
        "}\n",
        "MsgBox f() \", \" g\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1, 1"]);
}

#[test]
fn a_global_with_a_value_writes_the_global_of_the_script() {
    let (report, _) = run("global g := 1\nMsgBox g\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_local_in_a_function_is_local() {
    let (report, _) = run(concat!(
        "g := 1\n",
        "f() {\n",
        "  local l := 2\n",
        "  return l\n",
        "}\n",
        "MsgBox f() \", \" g\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2, 1"]);
}

#[test]
fn a_function_that_assumes_the_scope_of_the_script_writes_the_global() {
    // The bare `global` of the first line of a function is the mode of the
    // function.
    let (report, _) = run(concat!(
        "g := 0\n",
        "f() {\n",
        "  global\n",
        "  g := 1\n",
        "  return g\n",
        "}\n",
        "MsgBox f() \", \" g\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1, 1"]);
}

#[test]
fn a_local_in_a_function_that_assumes_the_scope_of_the_script_is_local() {
    let (report, _) = run(concat!(
        "g := 0\n",
        "f() {\n",
        "  global\n",
        "  local l := 2\n",
        "  g := 1\n",
        "  return l \" \" g\n",
        "}\n",
        "MsgBox f() \", \" g\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 1, 1"]);
}

#[test]
fn a_declaration_inside_a_block_takes_effect_for_the_whole_function() {
    let (report, _) = run(concat!(
        "g := 0\n",
        "f() {\n",
        "  if 1 {\n",
        "    global g\n",
        "  }\n",
        "  g := 1\n",
        "  return g\n",
        "}\n",
        "MsgBox f() \", \" g\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1, 1"]);
}

#[test]
fn a_declaration_may_name_several_variables() {
    let (report, _) = run(concat!(
        "a := 0\n",
        "b := 0\n",
        "f() {\n",
        "  global a, b := 1\n",
        "  a := 2\n",
        "  return a \" \" b\n",
        "}\n",
        "MsgBox f() \", \" a \" \" b\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 1, 2 1"]);
}

#[test]
fn the_call_of_the_call_method_of_a_value_names_that_method() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the failure of
    // the call of the `Call` method of a value carries the name of that
    // method in the `What`, and the name of the callee in the `Extra`.
    let (report, _desktop) = run(
        "try {\n    f := Abs\n    f.Call()\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too few parameters passed to function. | Func.Prototype.Call | Abs"]
    );
}

#[test]
fn the_call_of_a_value_carries_no_name_of_a_method() {
    let (report, _desktop) = run(
        "try {\n    f := Abs\n    x := f()\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Too few parameters passed to function. |  | Abs"]
    );
}

#[test]
fn every_value_carries_its_class() {
    // Measured on the binary of v2.0.19 on 2026-10-30: every value
    // carries its class, and the `__Class` of it is the name of it.
    let (report, _desktop) = run(
        "whole := 5\nfrac := 5.5\ntext := \"text\"\nlist := [1]\nMsgBox whole.__Class \" \" frac.__Class \" \" text.__Class \" \" list.__Class",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Integer Float String Array"]);
}

#[test]
fn the_class_of_an_object_is_the_class_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a class
    // is a name of the script, and the variable may not have it.
    let (report, _desktop) = run(
        "class Thing {\n}\ninstance := Thing()\nm := Map()\nMsgBox instance.__Class \" \" m.__Class \" \" Abs.__Class",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Thing Map Func"]);
}

#[test]
fn the_class_of_a_class_is_a_class() {
    let (report, _desktop) = run("class Thing {\n}\nMsgBox Thing.__Class \" \" Map.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Class Class"]);
}

#[test]
fn the_class_of_a_prototype_is_the_class_it_is_the_prototype_of() {
    let (report, _desktop) =
        run("class Thing {\n}\nMsgBox Thing.Prototype.__Class \" \" Map.Prototype.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Thing Map"]);
}

#[test]
fn the_class_of_a_value_that_is_not_an_object_cannot_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the property is
    // read-only, and the name of it goes in the `Extra`.
    let (report, _desktop) = run(
        "try {\n    x := 5\n    x.__Class := \"Map\"\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Property is read-only. |  | __Class"]);
}

#[test]
fn the_class_of_an_object_can_be_written_and_it_shadows_the_class() {
    let (report, _desktop) = run("m := Map()\nm.__Class := \"X\"\nMsgBox m.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["X"]);
}

#[test]
fn the_member_of_the_answer_of_a_call_is_read() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer of a
    // call is a value, and the members of it are read the way the members
    // of any other value are.
    let (report, _desktop) = run("x := StrLen(\"abc\").__Class\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Integer"]);
}

#[test]
fn the_item_of_the_answer_of_a_call_is_read() {
    let (report, _desktop) = run("m := Map()\nm.Set(\"a\", 1)\nMsgBox m.Get(\"a\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_assignment_of_a_call_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the assignment of
    // an expression is refused at the load, and the text of it is named.
    let error = parse("StrLen(\"abc\") := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: := 1"
    );
}

#[test]
fn the_operator_that_wants_a_number_may_not_follow_a_literal_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the script does not
    // load, and the text that is named is the operator and the rest of the
    // expression that holds it.
    let error = parse("x := \"a\" + 0").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected operator following literal string.\n     Specifically: + 0"
    );
    assert_eq!(error.line, 1);
}

#[test]
fn the_text_that_is_named_is_the_operator_and_the_rest_of_the_expression() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the expression that
    // holds the operator is the one of the statement, and the group is not
    // the end of it.
    let error = parse("x := (\"a\" + 0) + 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected operator following literal string.\n     Specifically: + 0) + 1"
    );
}

#[test]
fn the_command_of_v2_carries_the_parenthesis_that_closes_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the command is a call
    // whose parentheses are not written, and the text that is named carries
    // the one that closes it.
    let error = parse("StrLen \"a\" + 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected operator following literal string.\n     Specifically: + 1)"
    );
}

#[test]
fn the_concatenation_of_two_literal_texts_is_a_literal_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the concatenation of
    // two literal texts is a literal text, and the concatenation of a text
    // and a number is not.
    let error = parse("x := \"a\" \"b\" + 0").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected operator following literal string.\n     Specifically: + 0"
    );
}

#[test]
fn a_group_is_not_a_literal_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value of a group
    // is not a constant, and v2 refuses it at the run and not at the load.
    let (report, _desktop) = run("x := (\"a\") + 0");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got a String.")
    );
}

#[test]
fn the_body_of_a_block_may_be_one_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the body of a block
    // may be one statement, and the braces may be absent.
    let (report, _desktop) = run("if (1)\n    x := 1\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_body_of_a_block_without_braces_is_one_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the body is the one
    // statement that follows, and the rest of the line is outside of it.
    let (report, _desktop) = run("if (0)\n    MsgBox \"a\"\nMsgBox \"b\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["b"]);
}

#[test]
fn the_body_of_an_if_may_not_be_on_the_line_of_the_if() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `if` names its
    // own word.
    let error = parse("if (1) x := 1").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"If\"");
}

#[test]
fn the_body_of_an_else_may_be_on_the_line_of_the_else() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `else` takes the
    // body on its line and on the next one.
    let (report, _desktop) = run("if (0)\n    x := 1\nelse x := 2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn a_block_that_has_no_body_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `if` needs a
    // body, and it names its own word.
    let error = parse("if (1)").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"If\"");
}

#[test]
fn the_condition_of_an_if_is_not_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `if` needs a
    // condition, and it says so in its own words.
    let error = parse("if\n    x := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"If\" requires at least 1 parameter.\n     Specifically: if"
    );
}

#[test]
fn the_class_needs_its_braces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class names the
    // line of it.
    let error = parse("class Thing\n    x := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"{\"\n     Specifically: class Thing"
    );
}

#[test]
fn a_method_needs_its_braces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the method names the
    // line of it.
    let error = parse("class Thing {\n    M()\n        x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid method, class or property definition.\n     Specifically: M()"
    );
}

#[test]
fn the_base_of_an_object_is_another_object_and_not_a_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the base is an
    // object, and a name is refused with the reason.
    let (report, _desktop) = run("o := {}\nMsgBox HasBase(o, \"Object\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected an object but got a String.")
    );
}

#[test]
fn the_base_of_an_object_is_the_prototype_of_its_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the question is
    // whether the base of the object is the other object, and the chain of
    // the bases is walked.
    let (report, _desktop) = run(
        "m := Map()\nMsgBox HasBase(m, Map.Prototype) \" \" HasBase(m, Object.Prototype) \" \" HasBase(m, Array.Prototype)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1 0"]);
}

#[test]
fn a_value_that_is_not_an_object_has_no_base() {
    let (report, _desktop) = run("MsgBox HasBase(5, Object.Prototype)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_class_has_no_base_of_itself() {
    let (report, _desktop) = run("MsgBox HasBase(Map, Map.Prototype)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_methods_of_the_base_object_are_the_methods_of_every_object() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the methods of the
    // base object are inherited.
    let (report, _desktop) =
        run("o := {}\nMsgBox HasMethod(o, \"OwnProps\") \" \" HasMethod(o, \"Nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn the_question_with_no_name_asks_whether_the_value_is_a_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class has the
    // methods of the base object, and an instance has the methods of its
    // class.
    let (report, _desktop) =
        run("m := Map()\nMsgBox HasMethod(Map) \" \" HasMethod(m) \" \" HasMethod({})");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0 0"]);
}

#[test]
fn the_name_of_a_class_is_not_the_name_of_a_variable() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a class
    // is a name of the script, and it may not be the output variable of an
    // assignment. The refusal is at the load.
    let error = parse("class Foo {\n}\nfoo := 5").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Class cannot be used as an output variable.\n     Specifically: Foo"
    );
}

#[test]
fn the_name_of_a_class_is_not_written_by_a_compound_assignment() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the compound
    // assignment says that the class cannot be assigned a value.
    let error = parse("class Foo {\n}\nfoo .= \"x\"").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Class cannot be assigned a value.\n     Specifically: Foo"
    );
}

#[test]
fn the_name_of_a_class_of_the_runtime_is_not_the_name_of_a_variable() {
    let error = parse("Map := 5").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Class cannot be used as an output variable.\n     Specifically: Map"
    );
}

#[test]
fn a_class_may_not_be_named_like_a_function() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a name may be
    // declared only once, and the message names the kind of the one that is
    // already there.
    let error = parse("f() {\n}\nclass f {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This class declaration conflicts with an existing Func.\n     Specifically: f"
    );
}

#[test]
fn a_function_may_not_be_named_like_a_class() {
    let error = parse("class f {\n}\nf() {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This function declaration conflicts with an existing Class.\n     Specifically: f"
    );
}

#[test]
fn a_name_may_be_declared_only_once() {
    let error = parse("class f {\n}\nclass f {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This class declaration conflicts with an existing Class.\n     Specifically: f"
    );
}

#[test]
fn a_local_that_is_declared_is_not_the_global() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `local g` and a
    // global `g` are two names, and the local is the one that the function
    // reads.
    let (report, _desktop) = run("g := \"G\"\nf() {\n    local g\n    return g\n}\nf()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
}

#[test]
fn a_local_that_is_declared_may_hold_its_own_value() {
    let (report, _desktop) = run(concat!(
        "g := \"G\"\n",
        "f() {\n    local g\n    g := \"L\"\n    return g\n}\n",
        "MsgBox f()",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["L"]);
}

#[test]
fn a_local_may_not_hold_the_class_of_its_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a class
    // is a name of the script, and a local that has that name may not hold
    // the class or an instance of it.
    let (report, _desktop) = run("class Foo {\n}\nf() {\n    foo := Foo()\n    return foo\n}\nf()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "This local variable has not been assigned a value. A global declaration inside the function may be required."
        )
    );
}

#[test]
fn a_local_that_is_declared_may_not_hold_the_class_of_its_name_either() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a local that is
    // declared says the same words without the hint.
    let (report, _desktop) =
        run("class Foo {\n}\nf() {\n    local foo\n    foo := Foo()\n    return foo\n}\nf()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
}

#[test]
fn every_object_has_the_property_of_the_class_of_it() {
    let (report, _desktop) =
        run("m := Map()\nMsgBox m.HasProp(\"__Class\") \" \" m.HasOwnProp(\"__Class\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn the_concat_of_a_variable_that_has_no_value_is_an_empty_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the concat reads a
    // variable that has no value as an empty text, and the write makes it.
    let (report, _desktop) = run("s .= \"x\"\nMsgBox s");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x"]);
}

#[test]
fn the_concat_of_a_local_that_has_no_value_is_an_empty_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the same is true of
    // a local of a function.
    let (report, _desktop) = run("f() {\n    s .= \"x\"\n    return s\n}\nMsgBox f()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["x"]);
}

#[test]
fn the_add_of_a_variable_that_has_no_value_is_an_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the concat is the
    // one operator that reads a variable that has no value as an empty
    // text. The others want a value that is there.
    let (report, _desktop) = run("n += 2");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
}

#[test]
fn the_compound_assignment_of_a_property_that_is_not_there_is_an_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the compound
    // operators read the property before they write it, and a property that
    // is not there is an error that names it.
    let (report, _desktop) = run("o := {}\no.x .= \"y\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Object\" has no property named \"x\".")
    );
}

#[test]
fn the_compound_assignment_of_a_property_that_is_there_is_written() {
    let (report, _desktop) = run("o := {}\no.x := \"a\"\no.x .= \"b\"\nMsgBox o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ab"]);
}

#[test]
fn a_member_of_a_class_that_is_not_a_definition_names_the_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the member that is
    // not a definition is refused, and the line is named.
    let error = parse("class Thing {\n    x .= \"y\"\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid method, class or property definition.\n     Specifically: x .= \"y\""
    );
}

#[test]
fn a_member_of_a_class_that_is_not_a_name_names_the_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a member that is not
    // a name is not a valid definition either.
    let error = parse("class Thing {\n    1 + 2\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid method, class or property definition.\n     Specifically: 1 + 2"
    );
}

#[test]
fn a_member_of_a_class_that_is_a_name_alone_is_a_line_with_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a member that is a
    // name and nothing else is a line that has no action in it.
    let error = parse("class Thing {\n    x\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: x"
    );
}

#[test]
fn a_goto_goes_to_the_label_and_the_run_goes_on() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `Goto` jumps to
    // the label and continues the run.
    let (report, _desktop) =
        run("if (true) {\n    Goto later\n    MsgBox \"skipped\"\n}\nlater:\nMsgBox \"done\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["done"]);
}

#[test]
fn a_goto_that_leaves_a_block_goes_on_outside_it() {
    let (report, _desktop) = run("if (true) {\n    Goto later\n}\nlater:\nMsgBox \"done\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["done"]);
}

#[test]
fn the_name_of_a_label_does_not_know_its_case() {
    let (report, _desktop) = run("if (true) {\n    Goto DONE\n}\ndone:\nMsgBox \"done\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["done"]);
}

#[test]
fn a_goto_to_a_label_that_is_not_there_fails_with_the_reason() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the load refuses the
    // goto whose label is not in the scope, and it names the label.
    let error = parse("if (true) {\n    Goto nowhere\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Label not found in current scope.\n     Specifically: nowhere"
    );
    assert_eq!(error.line, 2);
}

#[test]
fn a_goto_into_a_block_that_does_not_enclose_it_fails_with_the_reason() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the jump may go to
    // a label of the block it is in, or of a block around it, and to no
    // other.
    let (report, _desktop) = run("if (true) {\n    Goto inner\n}\nif (true) {\n    inner:\n}");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("A Goto must not jump into a block that doesn't enclose it.")
    );
}

#[test]
fn the_failure_of_a_goto_is_not_taken_by_a_catch() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the goto whose label
    // is not there is refused at the load, and a catch does not take it.
    let error = parse("try {\n    Goto nowhere\n} catch Any as e {\n    MsgBox \"caught\"\n}")
        .expect_err("does not parse");
    assert_eq!(
        error.message,
        "Label not found in current scope.\n     Specifically: nowhere"
    );
}

#[test]
fn a_label_of_a_function_is_not_a_label_of_the_script() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the label of a
    // function is not the label of the script, and the load says so.
    let error = parse("f() {\n    Goto inner\n}\nf()\ninner:\nMsgBox \"done\"")
        .expect_err("does not parse");
    assert_eq!(
        error.message,
        "Label not found in current scope.\n     Specifically: inner"
    );
    assert_eq!(error.line, 2);
}

#[test]
fn is_label_answers_whether_the_label_is_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the label is asked
    // in the scope that runs, and the answer is one or zero.
    let (report, _desktop) = run("later:\nMsgBox IsLabel(\"later\") \" \" IsLabel(\"nope\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn is_label_asks_the_scope_that_runs() {
    let (report, _desktop) = run("f() {\n    MsgBox IsLabel(\"later\")\n}\nf()\nlater:");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn is_label_of_an_empty_name_answers_zero() {
    let (report, _desktop) = run("MsgBox IsLabel(\"\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_line_that_ends_with_an_operator_continues_on_the_next_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the line that ends
    // with an operator continues on the next one.
    let (report, _desktop) = run("x :=\n1\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_empty_lines_of_a_continuation_are_not_a_part_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the empty lines
    // between the two are not a part of the continuation.
    let (report, _desktop) = run("x :=\n\n1\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_line_that_begins_with_an_operator_continues_the_one_before_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the line that begins
    // with an operator continues the line before it.
    let (report, _desktop) = run("x := 1\n+ 2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn the_group_may_be_written_in_more_than_one_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the group may be
    // written in more than one line.
    let (report, _desktop) = run("x := (1\n)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_array_may_be_written_in_more_than_one_line() {
    let (report, _desktop) = run("x := [1\n, 2]\nMsgBox x.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_arguments_of_a_call_may_be_written_in_more_than_one_line() {
    let (report, _desktop) = run("x := StrLen(\n\"ab\")\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_operand_that_is_not_there_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the operator that
    // wants the operand names the failure.
    let error = parse("x :=").expect_err("does not parse");
    assert_eq!(error.message, "Missing operand.\n     Specifically: :=");
}

#[test]
fn the_operand_that_is_not_there_names_the_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the brace that
    // closes a block is not an operand that is missing, and the statement
    // it is in is named with it.
    let error = parse("if (x) {\ny :=\n}").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"}\"\n     Specifically: y := }");
    assert_eq!(error.line, 2);
}

#[test]
fn an_operator_that_may_not_begin_a_value_is_refused() {
    let error = parse("x := *").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: *");
}

#[test]
fn an_operator_that_may_not_begin_a_value_names_the_rest_of_the_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text that the
    // refusal names is the rest of the line.
    let error = parse("x := * 2").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: * 2");
}

#[test]
fn the_operator_that_wanted_the_operand_names_the_failure() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the operator that
    // wanted the operand names the failure, and the rest of the line is
    // named with it.
    let error = parse("x := 1 + * 2").expect_err("does not parse");
    assert_eq!(error.message, "Missing operand.\n     Specifically: + * 2");
    let error = parse("x := 1 +").expect_err("does not parse");
    assert_eq!(error.message, "Missing operand.\n     Specifically: +");
}

#[test]
fn the_operator_that_is_the_last_item_of_a_list_names_no_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text of the
    // refusal is not there when the operator is the last item of the
    // arguments of a call or of the items of an array. The group is not a
    // list, and the text is there.
    let error = parse("x := f(*)").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.");
    let error = parse("x := [*]").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.");
    let error = parse("x := (*)").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: *)");
}

#[test]
fn the_dot_that_reaches_into_nothing_names_the_rest_of_the_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dot that does not
    // reach into a value is a "Syntax error." and the rest of the line, from
    // the dot.
    let error = parse("x := a. b").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: . b");
    let error = parse("x := a.").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .");
}

#[test]
fn a_brace_that_closes_a_block_is_not_an_operand() {
    let error = parse("x := )").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \")\"\n     Specifically: x := )");
}

#[test]
fn the_colon_of_the_ternary_needs_its_question() {
    let error = parse("x := :").expect_err("does not parse");
    assert_eq!(error.message, "A \":\" is missing its \"?\"");
}

#[test]
fn the_section_joins_the_line_that_has_code_above_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a section begins with
    // a parenthesis at the start of a line, and it continues the last line
    // that has code. A name that ends that line is separated from the
    // contents of the section by a space.
    let (report, _desktop) = run("x := 1\n(\n2\n)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["12"]);
}

#[test]
fn the_section_joins_the_line_above_the_comments() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the blank lines and
    // the comments between the line and the section are of no account.
    let (report, _desktop) = run("x := 1\n; a comment\n\n(\n2\n)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["12"]);
}

#[test]
fn the_section_that_is_inside_a_text_is_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the section that
    // begins inside a text keeps the quotes as literal characters, and the
    // newlines of the section are kept.
    let (report, _desktop) = run("x := \"\n(\na\nb\n)\"\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a\nb"]);
}

#[test]
fn the_lines_of_a_section_are_spaces_in_an_expression() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the newlines of a
    // section are spaces in an expression.
    let (report, _desktop) = run("x := 1\n(\n2\n3\n)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["123"]);
}

#[test]
fn the_join_option_puts_the_lines_together() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the option `Join`
    // connects the lines without anything between them.
    let (report, _desktop) = run("x := \"\n( Join\na\nb\n)\"\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ab"]);
}

#[test]
fn the_ltrim_option_takes_the_indentation_away() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the option `LTrim`
    // removes the leading whitespace of every line.
    let (report, _desktop) = run("x := \"\n( LTrim\n  a\n  b\n)\"\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a\nb"]);
}

#[test]
fn the_indentation_of_the_first_line_goes_from_the_lines() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the leading
    // whitespace of the lines follows the indentation of the first one, and
    // a line that is indented less is left as it is.
    let (report, _desktop) = run("x := \"\n(\n  a\n b\n    c\n)\"\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a\n b\n  c"]);
}

#[test]
fn the_comments_option_takes_the_comments_away() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the option
    // `Comments` removes the comments of the lines.
    let (report, _desktop) = run("x := \"\n( Comments\na ; a comment\nb\n)\"\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a\nb"]);
}

#[test]
fn the_option_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the option that v2
    // does not know is named, and the script does not load.
    let error = parse("x := \"\n( Nope\na\n)\"").expect_err("does not parse");
    assert_eq!(error.message, "Invalid option.\n     Specifically: Nope");
    assert_eq!(error.line, 2);
}

#[test]
fn the_section_that_is_not_closed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the section that is
    // not closed is named, and the text it merged with the line above is
    // named with it.
    let error = parse("x := 1\n(\n2\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \")\"\n     Specifically: x := 1 2");
}

#[test]
fn a_line_that_begins_with_a_parenthesis_is_an_expression() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a line that begins
    // with a parenthesis is an expression, and the value it leaves behind is
    // not used.
    let (report, _desktop) = run("(2 + 3)\nMsgBox \"done\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["done"]);
}

#[test]
fn the_literal_at_the_start_of_a_statement_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a literal at the
    // start of a statement is a line that holds no action, and the line is
    // named with it.
    let error = parse("2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: 2"
    );
    let error = parse("1 + 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: 1 + 2"
    );
}

#[test]
fn the_value_that_came_too_close_names_the_rest_of_the_line() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value that came
    // too close is named, and the rest of the line is named with it.
    let error = parse("x := \"a\"\"b\"\"c\"").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing space or operator before this.\n     Specifically: \"b\"\"c\""
    );
}

#[test]
fn the_command_carries_the_parenthesis_that_closes_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the command of v2 is
    // a call whose parentheses are not written, and the text that names the
    // operator carries the one that closes it.
    let error = parse("MsgBox \"x\"2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing space or operator before this.\n     Specifically: 2)"
    );
}

#[test]
fn the_bracket_that_is_not_there_names_the_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the bracket that is
    // not there is named with the statement that holds it, and the lines of
    // the statement are written in one.
    let error = parse("FileAppend \"[\" (\nabc").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \")\"\n     Specifically: FileAppend \"[\" ( abc"
    );
}

#[test]
fn the_escape_of_a_space_is_a_space() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `s is a space, and
    // it is allowed wherever `t is.
    let (report, _desktop) = run("MsgBox \"a`sb\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a b"]);
}

#[test]
fn the_increment_may_be_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the increment may
    // be a statement of its own, and it writes to the value that is before
    // it.
    let (report, _desktop) = run("x := 1\nx++\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    let (report, _desktop) = run("x := 1\n++x\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_increment_writes_to_a_property_and_to_an_item() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the increment of a
    // property and of an item writes to them.
    let (report, _desktop) = run("x := {a: 1}\nx.a++\nMsgBox x.a");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    let (report, _desktop) = run("x := [1]\nx[1]++\nMsgBox x[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    let (report, _desktop) = run("x := Map()\nx[\"a\"] := 1\nx[\"a\"]++\nMsgBox x[\"a\"]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_postfix_answers_the_value_of_before() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the postfix
    // answers the value that was there, and the prefix answers the one that
    // is there.
    let (report, _desktop) = run("x := 1\ny := x++\nMsgBox y\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "2"]);
    let (report, _desktop) = run("x := 1\ny := ++x\nMsgBox y\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2", "2"]);
}

#[test]
fn the_increment_of_a_value_that_cannot_be_written_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the increment of a
    // value that cannot be written is refused at the load, and the source
    // of the operator is named with it.
    let error = parse("x := StrLen(\"ab\")++").expect_err("does not parse");
    assert_eq!(error.message, "Invalid assignment.\n     Specifically: ++");
    let error = parse("x := ++2").expect_err("does not parse");
    assert_eq!(error.message, "Invalid assignment.\n     Specifically: ++2");
    let error = parse("++2").expect_err("does not parse");
    assert_eq!(error.message, "Invalid assignment.\n     Specifically: ++2");
}

#[test]
fn the_number_at_the_left_of_an_assignment_is_refused_at_the_parse() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the number that is
    // the left side of an assignment is refused at the parse, and the text
    // it names is the number.
    let error = parse("1 := 2").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: 1");
    let error = parse("1e3 := 2").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: 1e3");
    let error = parse("0x10 := 2").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: 0x10");
}

#[test]
fn the_number_that_carries_a_point_is_refused_by_the_assignment() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the number that
    // carries a point is not refused at the parse: the assignment is the one
    // that is refused, and the text it names is the operator and the value.
    let error = parse("1.5 := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: := 2"
    );
    let error = parse("2.0 := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: := 2"
    );
    let error = parse("1.5 := 2 + 3").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: := 2 + 3"
    );
}

#[test]
fn the_compound_assignment_refuses_the_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the compound
    // operators refuse the number that is their left side, and the text
    // they name is the operator and the value.
    let error = parse("1 += 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: += 2"
    );
    let error = parse("1e3 -= 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: -= 2"
    );
    let error = parse("1.5 *= 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: *= 2"
    );
}

#[test]
fn the_number_with_a_point_at_its_end_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the point at the end
    // of the number is the dot of a member that is not there, and the line
    // holds no action.
    let error = parse("1. := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: 1. := 2"
    );
}

#[test]
fn the_array_at_the_start_of_a_statement_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the array at the
    // start of a statement is a line that holds no action, and the line is
    // named with it.
    let error = parse("[1] := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: [1] := 2"
    );
    let error = parse("[1]").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: [1]"
    );
}

#[test]
fn a_static_keeps_its_value_between_the_calls() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static of a
    // function is the same one in every call of it.
    let (report, _desktop) =
        run("F() {\nstatic count := 0\ncount++\nreturn count\n}\nMsgBox F()\nMsgBox F()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "2"]);
}

#[test]
fn the_value_of_a_static_declaration_runs_once() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value of the
    // declaration runs the first time that declaration is reached.
    let (report, _desktop) =
        run("F() {\nstatic count := 5\ncount++\nreturn count\n}\nMsgBox F()\nMsgBox F()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["6", "7"]);
    // The declaration that the function does not reach on the first call
    // runs its value on the call that reaches it.
    let (report, _desktop) = run(
        "F(n) {\nif n = 0\nreturn \"none\"\nstatic count := 5\nreturn count\n}\nMsgBox F(0)\nMsgBox F(1)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["none", "5"]);
}

#[test]
fn a_static_belongs_to_the_function_that_declares_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: two functions have
    // two statics, and the static of a recursion is the same one in every
    // turn of it.
    let (report, _desktop) = run(
        "F() {\nstatic count := 1\ncount++\nreturn count\n}\nG() {\nstatic count := 10\ncount++\nreturn count\n}\nMsgBox F()\nMsgBox F()\nMsgBox G()\nMsgBox G()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2", "3", "11", "12"]);
    let (report, _desktop) = run(
        "F(n) {\nstatic count := 0\ncount++\nif n = 0\nreturn count\nreturn F(n - 1)\n}\nMsgBox F(2)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_static_shadows_a_global_inside_the_function() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static of a
    // function is not the global of the same name.
    let (report, _desktop) = run(
        "global count := 1\nF() {\nstatic count := 2\nreturn count\n}\nMsgBox F()\nMsgBox count",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2", "1"]);
}

#[test]
fn a_static_of_a_method_is_the_same_one_in_every_call() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the static of a
    // method belongs to the method, and not to the instance.
    let (report, _desktop) = run(
        "class C {\nCount() {\nstatic n := 0\nn++\nreturn n\n}\n}\nMsgBox C().Count()\nMsgBox C().Count()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1", "2"]);
}

#[test]
fn a_static_without_a_value_is_a_declaration() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `static` with
    // no value declares the name, and the assignment that follows writes
    // it.
    let (report, _desktop) = run("F() {\nstatic count\ncount := 1\nreturn count\n}\nMsgBox F()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    // The bare `static` inside a function is accepted.
    let (report, _desktop) = run("F() {\nstatic\nreturn 1\n}\nMsgBox F()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_static_outside_a_function_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `static` at
    // the top level has no function to live in, and the whole declaration
    // is named with it.
    let error = parse("static count := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected declaration.\n     Specifically: static count := 1"
    );
}

#[test]
fn a_static_that_fights_with_a_parameter_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `static` may
    // not name a parameter of the function.
    let error = parse("F(count) {\nstatic count := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This static variable declaration conflicts with an existing parameter.\n     Specifically: count"
    );
}

#[test]
fn a_number_may_begin_with_a_point() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the numbers of v2
    // may begin with a point, and `.5` is the number 0.5.
    let (report, _desktop) = run("x := .5\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0.5"]);
    let (report, _desktop) = run("x := .5e2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["50.0"]);
    let (report, _desktop) = run("x := .5 + 1\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1.5"]);
    let (report, _desktop) = run("x := (.5)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0.5"]);
    let (report, _desktop) = run("x := [.5, 1]\nMsgBox x[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0.5"]);
    let (report, _desktop) = run("x := StrLen(.5)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    let (report, _desktop) = run("x := {a: .5}\nMsgBox x.a");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0.5"]);
}

#[test]
fn the_point_of_a_member_is_not_the_point_of_a_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the point that
    // follows a value is the dot of the member, and the number that begins
    // with a point is a number.
    let (report, _desktop) = run("x := 1.5\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1.5"]);
    let (report, _desktop) = run("x := {5: 1}\nMsgBox x.5");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_name_of_a_property_may_be_a_whole_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // property may be a whole number, and the number with a point is not a
    // name.
    let (report, _desktop) = run("x := {5: 1}\nMsgBox x.5");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    let (report, _desktop) = run("x := {0x10: 1}\nMsgBox x.0x10");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    let error = parse("x := {1.5: 1}").expect_err("does not parse");
    assert_eq!(error.message, "Invalid property name in object literal.");
}

#[test]
fn a_statement_that_begins_with_an_operator_is_a_line_that_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a statement that
    // begins with an operator is a line that holds no action, and the text
    // that is named is the line itself.
    let error = parse("-1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: -1"
    );
    let error = parse("*x").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: *x"
    );
    let error = parse("&x").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: &x"
    );
    let error = parse("== 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: == 1"
    );
    // The statement that carries an assignment after the operator is
    // refused with the whole line.
    let error = parse("-x := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: -x := 2"
    );
    let error = parse("!x := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: !x := 2"
    );
}

#[test]
fn the_operators_that_want_a_word_of_their_own_are_named_apart() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the equal wants the
    // operator of an assignment, the assignment wants a name on its left,
    // the comma separates the parameters of a call, the question asks a
    // question that is not there, and the double colon is the separator of
    // a hotkey.
    let error = parse("= 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Syntax error. Did you mean to use \":=\"?\n     Specifically: = 1"
    );
    let error = parse(":= 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\":=\" requires that parameter #1 be non-blank."
    );
    let error = parse("+= 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: += 1"
    );
    let error = parse(", 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Function calls require a space or \"(\".  Use comma only between parameters.\n     Specifically: , 1"
    );
    let error = parse("? 1").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: ? 1");
    let error = parse(":: 1").expect_err("does not parse");
    assert_eq!(error.message, "Invalid hotkey.\n     Specifically: :: 1");
}

#[test]
fn the_dot_is_the_member_of_a_value_that_is_not_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dot with a name
    // after it is the member of a value that is not there, and the v2 names
    // the call that it wanted to make.
    let error = parse(".x").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .x()");
    let error = parse(".x 1").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .x(1)");
    let error = parse(".x := 2").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .x := 2");
    // The dot alone is a line that holds no action.
    let error = parse(".").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: ."
    );
    // The dot in the place of an operand names the rest of the line.
    let error = parse("x := 1 + .").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .");
    let error = parse("f(.)").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: .)");
}

#[test]
fn the_line_that_begins_with_an_operator_continues_the_one_before_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the line that begins
    // with an operator continues the one before it, and the variable is
    // read before it is assigned.
    let (report, _desktop) = run("x := 1\n-x");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    // The assignment that the continuation makes is the one of the
    // operator: `x := 5 - (x := 2)` leaves x with 3.
    let (report, _desktop) = run("x := 5\n-x := 2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_loop_of_a_kind_without_its_parameter_is_refused_at_the_load() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the command of a
    // kind checks its parameters when the script loads, and the parameter
    // that is not there is named.
    let error = parse("Loop Read {\n  x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 required\n     Specifically: Loop Read "
    );
    let error = parse("Loop Parse {\n  x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 required\n     Specifically: Loop Parse "
    );
    let error = parse("Loop Files {\n  x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 required\n     Specifically: Loop Files "
    );
    let error = parse("Loop Reg {\n  x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 required\n     Specifically: Loop Reg "
    );
    // The name of the kind is the one that the script wrote.
    let error = parse("Loop read {\n  x := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 required\n     Specifically: Loop read "
    );
}

#[test]
fn the_loop_of_a_kind_names_itself_when_it_is_not_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the list of the
    // parameters that is empty says so, and the statement that is not
    // complete names itself.
    let error = parse("Loop Read,").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Read\" requires at least 1 parameter.\n     Specifically: Loop Read,"
    );
    let error = parse("Loop Read, \"a\"").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Loop Read\"");
    let error = parse("Loop Parse, \"a\"").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Loop Parse\"");
    // The word that follows the `Loop` is the count of the loop when the
    // line ends there, and the loop of a count is named "Loop".
    let error = parse("Loop Read").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Loop\"");
    let error = parse("Loop Parse").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Loop\"");
    let error = parse("Loop Files").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Loop\"");
}

#[test]
fn the_loop_of_a_kind_takes_the_parameters_that_it_takes() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `Loop Read`,
    // `Loop Files` and `Loop Reg` take two parameters, and `Loop Parse`
    // takes three. The one that is written too many is named with the
    // first of them.
    let error = parse("Loop Read, \"a\", \"b\", \"c\" {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Read\" accepts at most 2 parameters.\n     Specifically: Loop Read, \"a\""
    );
    let error = parse("Loop Parse, \"a\", \"b\", \"c\", \"d\" {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Parse\" accepts at most 3 parameters.\n     Specifically: Loop Parse, \"a\""
    );
    let error = parse("Loop Files, \"a\", \"b\", \"c\" {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Files\" accepts at most 2 parameters.\n     Specifically: Loop Files, \"a\""
    );
    let error = parse("Loop Reg, \"a\", \"b\", \"c\" {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Loop Reg\" accepts at most 2 parameters.\n     Specifically: Loop Reg, \"a\""
    );
}

#[test]
fn the_count_of_the_loop_is_not_a_name_of_a_kind() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a comma after the
    // count says that the type of the loop is not a valid one.
    let error = parse("Loop 5, 6 {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid loop type.\n     Specifically: Loop 5, 6 {"
    );
    let error = parse("Loop Foo, 5 {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid loop type.\n     Specifically: Loop Foo, 5 {"
    );
    // The comma of a call is not there.
    let error = parse("Loop,").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Function calls require a space or \"(\".  Use comma only between parameters.\n     Specifically: Loop,"
    );
}

#[test]
fn a_loop_without_a_count_goes_on_until_a_break() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the loop without
    // a count goes on until a break.
    let (report, _desktop) =
        run("i := 0\nLoop {\n  i := i + 1\n  If i = 3 {\n    Break\n  }\n}\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
}

#[test]
fn a_loop_that_runs_away_is_stopped_by_the_guard_rail() {
    // The loop of v2 has no bound of its own, and the tree has one: the
    // count of the statements.
    let (report, _desktop) = run("Loop {\n}");
    assert!(!report.ok);
    assert!(
        report
            .error
            .as_deref()
            .unwrap_or("")
            .contains("ran more than"),
        "{:?}",
        report.error
    );
    assert_eq!(report.error_line, Some(1));
}

// -- the `until` of the loop of v2 -------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09, with the probes under
// `egm-review/keysharp-audit/wp1/`: the `until` follows the body of the
// loop -- on the line of the closing brace or on the line that comes next,
// with blank lines and comments between them -- and the condition is read
// after every turn of the body. The loop ends when the condition is true,
// and the count of the loop is the other end of it.

#[test]
fn the_until_of_the_loop_ends_it_after_the_body() {
    // `accept_basic.ahk` A1: the loop without a count ends on the third
    // turn.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n} Until i >= 3\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    // A3: the word does not know its case.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n} UNTIL i >= 3\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    // A7: the space between the closing brace and the word is not needed.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n}Until i >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_until_of_the_loop_is_a_condition_of_v2() {
    // A4: the `=` of v2 is a comparison and not an assignment.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n} until i = 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // A5: the condition is a call of a function of the script.
    let (report, _desktop) =
        run("f() {\n  return true\n}\ni := 0\nLoop {\n  i += 1\n} until f()\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    // A6: the condition reads A_Index.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n} until A_Index >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // A14 and A15: the condition is an expression of two operands, and the
    // function of the script may be one of them.
    let (report, _desktop) = run("i := 0\nLoop {\n  i += 1\n} until (i >= 2 and i < 10)\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    let (report, _desktop) = run(
        "f() {\n  return true\n}\ni := 0\nLoop {\n  i += 1\n} until (i > 5 or f())\nMsgBox i",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    // P7: the condition may write a variable.
    let (report, _desktop) = run(
        "x := 0\ny := 0\nLoop {\n  x += 1\n} until (y := x) >= 2\nMsgBox y",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_count_of_the_loop_is_the_other_end_of_the_until() {
    // A2 and A9: the until ends the loop before the count does.
    let (report, _desktop) = run("i := 0\nLoop 10 {\n  i += 1\n} Until i > 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    // A9b: the count ends the loop before the until does.
    let (report, _desktop) = run("i := 0\nLoop 2 {\n  i += 1\n} Until i > 5\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_until_of_the_loop_may_be_on_the_line_after_the_closing_brace() {
    // `accept_nextline.ahk` B1: the word on the line after the brace.
    let (report, _desktop) = run("i := 0\nLoop 3\n{\n  i += 1\n}\nUntil i >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // B2: a blank line between them.
    let (report, _desktop) = run("i := 0\nLoop 3\n{\n  i += 1\n}\n\nUntil i >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // B3: a comment on the line of the closing brace.
    let (report, _desktop) =
        run("i := 0\nLoop 3\n{\n  i += 1\n} ; the body is done\nUntil i >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // B4: a comment after the condition.
    let (report, _desktop) =
        run("i := 0\nLoop 3\n{\n  i += 1\n}\nUntil i >= 2 ; the condition\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // B5 and N2: the content of the block on the line of the brace that
    // opens it, and the until on the line of the closing brace.
    let (report, _desktop) = run("i := 0\nLoop 3\n{ i += 1\n} Until i >= 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_until_of_the_loop_follows_a_body_that_has_no_braces() {
    // F1 and F2: the body is one statement on the line after the loop.
    let (report, _desktop) = run("i := 0\nLoop 10\n  i += 1\nUntil i > 2\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    let (report, _desktop) = run("i := 0\nLoop 10\n  i += 2\nUntil i >= 4\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn the_until_of_the_loop_is_read_after_a_turn_that_used_continue() {
    // A10: the count of the loop and the turn that continued, and the
    // condition that is read after it. The loop without a count is the
    // same rule in v2 (A11 answers 2), and the runtime does not read the
    // until after a `continue` there yet.
    let (report, _desktop) = run(
        "i := 0\nLoop 10 {\n  i += 1\n  if (i = 2) {\n    continue\n  }\n} Until i = 2\nMsgBox i",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn a_break_leaves_the_loop_before_the_until_is_read() {
    // A13: the `break` leaves the loop, and the condition is not read.
    let (report, _desktop) =
        run("x := 0\ni := 0\nLoop {\n  i += 1\n  break\n} Until (x := 1)\nMsgBox i \"|\" x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1|0"]);
}

#[test]
fn a_loop_of_zero_turns_does_not_read_the_until() {
    // A12: the loop whose count is zero never reads the condition.
    let (report, _desktop) = run("x := 0\nLoop 0 {\n} Until (x := 1)\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn the_until_of_the_loop_binds_the_loop_that_it_follows() {
    // E1: the until of the inner loop is the one read there, and the
    // until of the outer loop is read after each turn of the outer body.
    let (report, _desktop) = run(
        "outer := 0\ninner := 0\nLoop 3 {\n  Loop 2 {\n    inner += 1\n  } Until true\n  outer += 1\n} Until outer >= 2\nMsgBox outer \"|\" inner",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2|2"]);
    // E2: the outer loop has no until, and it counts its turns.
    let (report, _desktop) = run(
        "outer := 0\ninner := 0\nLoop 3 {\n  Loop 2 {\n    inner += 1\n  } Until true\n  outer += 1\n}\nMsgBox outer \"|\" inner",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3|3"]);
}

#[test]
fn the_until_of_the_loop_needs_a_condition_and_names_the_word_as_written() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the word with no
    // condition is refused with this message, and the text that it names
    // is the word as the script wrote it.
    let error = parse("Loop 1\n{\n} Until").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Until\" requires at least 1 parameter.\n     Specifically: Until"
    );
    assert_eq!(error.line, 3);
    let error = parse("Loop 1\n{\n} until").expect_err("does not parse");
    assert_eq!(
        error.message,
        "\"Until\" requires at least 1 parameter.\n     Specifically: until"
    );
}

#[test]
fn the_until_of_the_while_is_not_a_condition_of_the_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the `while` refuses
    // the word with "Unexpected \"Until\"", and the line that it names is
    // the line of the word. The `for` takes it, and the node of the `for`
    // of this tree has no field for it yet.
    let error = parse("i := 0\nWhile i < 5 {\n  i += 1\n}\nUntil i >= 3").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Until\"");
    assert_eq!(error.line, 5);
}

#[test]
fn the_word_until_is_reserved_in_v2_and_not_in_a_property() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the word is not the
    // name of a variable, and it is the name of a property.
    let error = parse("until := 2").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a variable name:\n\"until\""
    );
    let (report, _desktop) = run("o := {until: 5}\nMsgBox o.until");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5"]);
}

#[test]
fn the_until_of_the_loop_of_a_kind_is_parsed() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the loops of a kind
    // take the until as well -- `Loop Parse "a,b,c", "," { } Until i >= 2`
    // ends on the second field. The parser attaches it; the runtime does
    // not read it for the kinds yet.
    assert!(parse("Loop Parse \"a,b,c\", \",\" {\n  i += 1\n} Until i >= 2").is_ok());
    assert!(parse("Loop Files \"*.txt\" {\n} Until true").is_ok());
}

#[test]
fn the_until_of_the_v1_is_still_the_statement_after_the_loop() {
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    // `until` of the v1 is the condition of the loop, and the v1 reads it
    // from the statement after the body. The v2 change does not touch it.
    let (report, _desktop) = run_in(
        "i := 0\nLoop 3 {\ni := i + 1\n}\nUntil i == 2\nFileAppend, [%i%], *",
        ahk::dialect::v1(),
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[2]".to_string()]);
    // The `until` that no loop takes is still refused when the load is
    // over.
    let error = ahk::parse_with("Until 1", ahk::dialect::v1()).expect_err("does not parse");
    assert_eq!(error.message, "UNTIL with no matching LOOP");
}

#[test]
fn the_fields_of_the_loop_that_parses_are_the_characters_of_the_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delimiters
    // that are not there make every character of the text a field of its
    // own, and the spaces are fields too.
    let (report, desktop) = run("Loop Parse \"a b\" {\n  Send \"%A_LoopField%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a", " ", "b"]);
    // The characters that are not wanted at the ends of a field are taken
    // away from them.
    let (report, desktop) =
        run("Loop Parse \" a , b \", \",\", \" \" {\n  Send \"%A_LoopField%\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a", "b"]);
}

#[test]
fn the_variables_of_the_loops_are_there_before_the_first_loop() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the index is zero
    // and the others are empty.
    let (report, _desktop) = run("MsgBox \"[\" A_Index \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[0]"]);
    let (report, _desktop) = run("MsgBox \"[\" A_LoopField \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
    let (report, _desktop) = run("MsgBox \"[\" A_LoopReadLine \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_capacity_of_a_string_that_has_no_value_is_the_one_that_was_asked_for() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a variable that
    // has no value is a text that is empty, and the capacity of it is
    // the one that was asked for.
    let (report, _desktop) = run("MsgBox VarSetStrCapacity(&x, 100)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["100"]);
    // Measured on the binary of v2.0.19 on 2026-11-09: the capacity of a
    // text is the size class AHK keeps for it, and the "hello" of five
    // characters is of the class of seven. The tree keeps the class as
    // well since the block of the string memory.
    let (report, _desktop) = run("x := \"hello\"\nMsgBox VarSetStrCapacity(&x)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn an_object_is_not_written_as_a_text() {
    // Measured on the binary of v2.0.19 on 2026-10-31: v2 reads the
    // property "Ptr" of the value that it writes, and an object has
    // none.
    let (report, _desktop) = run("FileAppend Map(), \"*\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Map\" has no property named \"Ptr\".")
    );
    let (report, _desktop) = run("FileAppend [1, 2], \"*\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Array\" has no property named \"Ptr\".")
    );
}

#[test]
fn a_class_is_a_class_of_its_own_kind_and_a_prototype_is_of_its_own() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the refusal of
    // v2 names the class "Class" and the prototype "Prototype".
    let (report, _desktop) = run("FileAppend Gui, \"*\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Class\" has no property named \"Ptr\".")
    );
    let (report, _desktop) = run("FileAppend Gui.Prototype, \"*\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Prototype\" has no property named \"Ptr\".")
    );
    // The parameter of a built-in names the kind of the value that
    // arrived the same way.
    let (report, _desktop) = run("MsgBox Gui");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of MsgBox requires a String, but received a Class.")
    );
}

#[test]
fn the_stack_of_an_error_names_the_frames_of_the_call() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every frame is a
    // line of its own -- the file, the line, the name of the frame and
    // the source of the line -- and the last line names the section that
    // starts the script. The script of the runner has no file of its own,
    // and the file of every frame is empty.
    let (report, _desktop) = run(
        "f() {\n  Throw Error(\"boom\")\n}\ntry {\n  f()\n} catch Any as err {\n  MsgBox err.Stack\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![" (2) : [f] Throw Error(\"boom\")\n (5) : [] f()\n> Auto-execute"]
    );
}

#[test]
fn the_stack_of_an_error_of_the_section_that_starts_the_script() {
    // The section that starts the script is the last frame of the stack,
    // and it has no name.
    let (report, _desktop) =
        run("try {\n  Throw Error(\"boom\")\n} catch Any as err {\n  MsgBox err.Stack\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![" (2) : [] Throw Error(\"boom\")\n> Auto-execute"]
    );
}

#[test]
fn the_stack_of_a_failure_of_the_run_names_the_line_that_failed() {
    // A failure that the script did not throw has a stack too.
    let (report, _desktop) =
        run("try {\n  x := 1 / 0\n} catch Any as err {\n  MsgBox err.Stack\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![" (2) : [] x := 1 / 0\n> Auto-execute"]);
}

#[test]
fn the_stack_names_every_frame_of_the_call() {
    // The frames come from the inside out, and the name of a method is
    // the one that v2 gives it.
    let (report, _desktop) = run(
        "g() {\n  f()\n}\nf() {\n  h()\n}\nh() {\n  Throw Error(\"boom\")\n}\ntry {\n  g()\n} catch Any as err {\n  MsgBox err.Stack\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            " (8) : [h] Throw Error(\"boom\")\n (5) : [f] h()\n (2) : [g] f()\n (11) : [] g()\n> Auto-execute"
        ]
    );
}

#[test]
fn the_member_of_the_answer_of_a_call_is_read_in_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer of a call
    // is a value, and the member of it is read the way the member of any
    // other value is. The statement reads it and leaves the value behind.
    let (report, _desktop) = run("Map().Count");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_method_of_the_answer_of_a_call_is_called_in_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the method of the
    // answer of a call is called when the statement is the call of it.
    let (report, _desktop) = run(r#"Map().HasOwnProp("a")"#);
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_write_to_a_member_of_a_call_is_refused_when_the_property_is_read_only() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the write of a
    // property that may not be written is refused at the run, with
    // "Property is read-only.", the name of the property in the Extra, and
    // nothing in the What.
    let (report, _desktop) = run(r#"try {
    Map().Count := 1
} catch Any as e {
    MsgBox e.Message " | " e.What " | " e.Extra
}"#);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Property is read-only. |  | Count"]);
}

#[test]
fn the_call_itself_may_not_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer of a call
    // is not a value that may be written, and the refusal is at the load,
    // with the operator and the value named.
    let error = parse("Map() := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid assignment.\n     Specifically: := 1"
    );
    let error = parse("Map()++").expect_err("does not parse");
    assert_eq!(error.message, "Invalid assignment.\n     Specifically: ++");
}

#[test]
fn the_item_of_the_answer_of_a_call_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the item of the
    // answer of a call is an item, and it may be written.
    let (report, _desktop) = run(r#"Map()["a"] := 1"#);
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_property_of_the_answer_of_a_constructor_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the property of the
    // answer of a constructor is a property of the instance, and it may be
    // written.
    let (report, _desktop) = run("class T {\n    P := 3\n}\nT().P := 1");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn the_hash_that_is_not_a_directive_is_an_illegal_character() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the hash is a
    // directive only at the start of a line, and anywhere else it is an
    // illegal character in an expression. The text that is named is the
    // expression, and the command of v2 carries the parenthesis that closes
    // it.
    let error = parse("x := 1 #y").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: #y"
    );
    let error = parse("MsgBox #y").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: #y)"
    );
}

#[test]
fn the_hash_that_follows_a_name_is_a_line_that_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the hash that
    // follows a name is a part of it, and the line that holds it holds no
    // action.
    let error = parse("x#y").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: x#y"
    );
    let error = parse("x#y := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: x#y := 1"
    );
}

#[test]
fn the_directive_that_is_not_known_is_a_line_that_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a name that the v2
    // does not know is not a directive at all, and the refusal is at the
    // load.
    let error = parse("#Foo").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: #Foo"
    );
    let error = parse("#Foo Bar").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: #Foo Bar"
    );
}

#[test]
fn the_double_deref_reads_the_variable_that_the_name_spells() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the double deref of
    // v2 is a name that the script builds -- the value of the expression is
    // the name of the variable whose value is the answer.
    let (report, _) = run("b := 42\nx := \"b\"\nMsgBox %x%");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["42"]);
}

#[test]
fn the_dynamic_name_reads_the_variable_whose_name_is_built() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the pieces of the
    // name are the text that is written and the values of the expressions,
    // and the space between two of them is what tells that the name is
    // over.
    let (report, _) = run("MyArray1 := \"A\"\ni := 1\nMsgBox MyArray%i%");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["A"]);
}

#[test]
fn the_dynamic_name_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name that the
    // script builds may be written to, and the variable that it names is
    // the one that changes.
    let (report, _) = run("MyArray1 := \"A\"\ni := 1\nMyArray%i% := \"B\"\nMsgBox MyArray1");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["B"]);
}

#[test]
fn the_double_deref_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name that the
    // script builds may be written to.
    let (report, _) = run("b := 1\nx := \"b\"\n%x% := 7\nMsgBox b");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn the_name_that_is_not_a_variable_fails_with_its_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // dynamic variable reference is the name of a variable, and the one
    // that is not there fails with "Variable not found." and the name in
    // the `Extra`.
    let (report, _) = run(
        "try {\n    x := \"nope\"\n    MsgBox %x%\n} catch as e {\n    MsgBox e.Message\n    MsgBox e.Extra\n    MsgBox e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Variable not found.", "nope", "Error"]);
}

#[test]
fn the_name_of_a_dynamic_reference_is_not_a_path() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // dynamic variable reference is a name, and not a path of names:
    // `%m.a%` is the variable `m.a`, and it is not there.
    let (report, _) = run(
        "try {\n    m := Map()\n    x := \"m.a\"\n    MsgBox %x%\n} catch as e {\n    MsgBox e.Message\n    MsgBox e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Variable not found.", "m.a"]);
}

#[test]
fn the_dynamic_member_reads_the_property_that_the_name_spells() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // member may be built by the script: `m.%x%` is the member whose name
    // is the value of `x`.
    let (report, _) = run("m := {a: 1}\nx := \"a\"\nMsgBox m.%x%");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_dynamic_member_may_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the member whose
    // name the script builds may be written to.
    let (report, _) = run("m := {a: 1}\nx := \"a\"\nm.%x% := 7\nMsgBox m.a");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn the_dynamic_method_is_called_by_its_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `m.%x%()` is the
    // method whose name is the value of `x`, and the one that is not there
    // fails with the name of it.
    let (report, _) = run(
        "try {\n    m := {a: 1}\n    x := \"a\"\n    m.%x%()\n} catch as e {\n    MsgBox e.Message\n    MsgBox e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "This value of type \"Object\" has no method named \"a\".",
            "MethodError"
        ]
    );
}

#[test]
fn the_dynamic_write_of_a_global_from_a_function_wants_a_global_declaration() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a function that
    // writes a name of the script says that it wants a global declaration,
    // and the failure is of the class `Error`.
    let (report, _) = run(
        "b := 1\ntry {\n    Write()\n} catch as e {\n    MsgBox e.Message\n    MsgBox e.Extra\n    MsgBox e.__Class\n}\n\nWrite() {\n    x := \"b\"\n    %x% := 7\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "This dynamic assignment requires a \"global\" declaration.",
            "b",
            "Error"
        ]
    );
}

#[test]
fn the_double_deref_that_does_not_close_is_refused_at_the_load() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name that does
    // not close is refused at the load, and the text that is named is the
    // rest of the line, from the name.
    let error = parse("x := %a").expect_err("does not parse");
    assert_eq!(error.message, "Missing ending \"%\"\n     Specifically: %a");
    let error = parse("x := a%b").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing ending \"%\"\n     Specifically: a%b"
    );
}

#[test]
fn the_percent_sign_that_is_inside_a_name_is_a_syntax_error() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the percent sign
    // that is inside a name that the script builds is not a value, and v2
    // says "Syntax error." and the rest of the line, from it.
    let error = parse("x := %%y%%").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: %y%%");
}

#[test]
fn the_double_deref_alone_is_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a statement that
    // begins with a double deref is a name that the script builds, and the
    // value it leaves behind is not used.
    let (report, _) = run("b := 42\nx := \"b\"\n%x%");
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output.is_empty());
}

#[test]
fn the_dynamic_name_of_a_function_is_the_function() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of a
    // dynamic variable reference is looked up the way a name that is
    // written is looked up: a function of the script is a value too.
    let (report, _) = run("f() {\n    return 1\n}\nx := \"f\"\nMsgBox %x%.__Class");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Func"]);
}

#[test]
fn the_bracket_that_is_not_closed_is_the_failure() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the bracket that is
    // not closed is the failure, and not the operand that is not there
    // inside it: `x := f(1,` is a "Missing ")" and not a "Missing
    // operand.".
    let error = parse("x := f(1,").expect_err("does not parse");
    assert_eq!(error.message, "Missing \")\"\n     Specifically: x := f(1,");
    let error = parse("x := [").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"]\"\n     Specifically: x := [");
    let error = parse("x := {").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: x := {");
    let error = parse("x := (1 +").expect_err("does not parse");
    assert_eq!(error.message, "Missing \")\"\n     Specifically: x := (1 +");
}

#[test]
fn the_brace_that_closes_the_block_is_named_before_the_bracket() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a brace that closes
    // a block is named before the bracket that is not closed, and the text
    // that is named is the statement and the line of the failure.
    let error = parse("try {\n    x := f(1\n} catch as err {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \")\" before \"}\"\n     Specifically: x := f(1 } catch as err {"
    );
}

#[test]
fn the_button_of_a_click_is_a_value_and_not_a_word() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `Click 10, 20,
    // right` reads the variable `right`. The v2 refuses the script at the
    // load when that variable has no value, and the tree refuses it at the
    // run.
    let (report, _desktop) = run("Click 10, 20, right");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
}

#[test]
fn the_button_of_a_click_may_be_the_value_of_a_variable() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value of the
    // variable is the name of the button, and not the name of the
    // variable.
    let (report, desktop) = run("right := \"left\"\nClick 10, 20, right");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10,20,left,1".to_string())]
    );
}

#[test]
fn the_name_of_a_property_of_an_object_may_be_built() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `{%name%: 1}` is
    // the property whose name is the value of `name`.
    let (report, _desktop) = run("name := \"a\"\nx := {%name%: 1}\nFileAppend x.a, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1".to_string()]);
}

#[test]
fn a_brace_that_opens_an_argument_of_a_command_opens_an_object() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `Send {Enter}` is
    // refused at the load, because the brace opens the literal of an
    // object and the name of a property is not there.
    let error = parse("Send {Enter}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Missing \"propertyname:\" in object literal."
    );
}

#[test]
fn the_button_of_a_mouse_click_is_the_first_parameter() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the button of a
    // `MouseClick` is the first parameter, and it is a text that is not
    // spread. `MouseClick 10, 20` is refused, and the "10" is the button.
    let (report, _desktop) = run("MouseClick 10, 20");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of MouseClick is invalid.")
    );
}

#[test]
fn the_button_of_a_mouse_click_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let (report, _desktop) = run("MouseClick \"nonsense\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of MouseClick is invalid.")
    );
}

#[test]
fn the_button_of_a_mouse_click_is_not_spread() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the words of the
    // button are not the words of a click.
    let (report, _desktop) = run("MouseClick \"R D\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Parameter #1 of MouseClick is invalid.")
    );
}

#[test]
fn the_value_of_the_button_that_failed_is_in_the_extra() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value that
    // failed is in the `Extra`, with the space at the end of it cut.
    let (report, _desktop) =
        run("try {\n    MouseClick \"Left \"\n} catch as err {\n    MsgBox err.Extra\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Left".to_string()]);
}

#[test]
fn a_mouse_click_takes_the_place_and_the_count() {
    let (report, desktop) = run("MouseClick \"Right\", 10, 20, 3");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10,20,right,3".to_string())]
    );
}

#[test]
fn a_mouse_click_with_no_button_is_the_left_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a button that is
    // not there is the left one.
    let (report, desktop) = run("MouseClick");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "0,0,left,1".to_string())]
    );
}

#[test]
fn the_wheel_of_a_mouse_click_is_not_carried_yet() {
    // Measured on the binary of v2.0.19 on 2026-10-31: v2 takes the wheel
    // of a `MouseClick`, and the desktop of the tree does not.
    let (report, _desktop) = run("MouseClick \"WheelUp\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the wheel is not carried yet.")
    );
}

#[test]
fn the_fourth_button_of_a_mouse_click_is_not_carried_yet() {
    let (report, _desktop) = run("MouseClick \"X1\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the fourth button is not carried yet.")
    );
}

#[test]
fn the_value_that_the_is_refuses_is_in_the_extra() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value that
    // arrived goes in the `Extra` of the failure, and the `What` is empty.
    let (report, _desktop) =
        run("try {\n    x := 1 is 2\n} catch as err {\n    MsgBox err.Extra\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2".to_string()]);
}

#[test]
fn the_value_of_the_list_is_the_last_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value of a
    // parenthesized list is the last of its items.
    let (report, _desktop) = run("x := (\"a\", \"b\")\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["b"]);
}

#[test]
fn every_item_of_the_list_runs() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every item of a
    // parenthesized list runs.
    let (report, _desktop) =
        run("i := 0\nf() {\n    global i\n    i := i + 1\n}\nx := (f(), f())\nMsgBox i");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_name_with_a_space_and_a_parenthesis_is_a_command() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parenthesis
    // that is spaced is not the list of the arguments of a call, and the
    // group is one parameter of the command.
    let (report, _desktop) = run("One(a) {\n    MsgBox a\n}\nOne (\"a\", \"b\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["b"]);
}

#[test]
fn the_command_with_a_group_and_a_text_has_two_parameters() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameters of
    // a command are the expressions that the commas separate.
    let (report, _desktop) = run("Two(a, b) {\n    MsgBox a\n    MsgBox b\n}\nTwo (\"a\"), \"b\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a", "b"]);
}

#[test]
fn the_commas_at_the_end_of_a_command_add_nothing() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the commas at the
    // end of the parameters of a command add nothing.
    let (report, _desktop) = run("One(a) {\n    MsgBox a\n}\nOne (\"a\"),");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a"]);
}

#[test]
fn the_member_with_a_space_and_a_parenthesis_is_a_command() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parenthesis
    // that is spaced is not the list of the arguments of a call, and the
    // group is one parameter of the command.
    let (report, _desktop) = run(
        "class Thing {\n    Method(a) {\n        MsgBox a\n    }\n}\nt := Thing()\nt.Method (\"a\", \"b\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["b"]);
}

#[test]
fn the_member_with_a_space_is_not_a_call_in_an_expression() {
    // Measured on the binary of v2.0.19 on 2026-10-31: in an expression the
    // parenthesis that is spaced is the value that follows the member, and
    // the concatenation of v2 takes the two.
    let (report, _desktop) = run(
        "class Thing {\n    Method(a) {\n        MsgBox a\n    }\n}\nt := Thing()\nx := t.Method (\"a\")",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got a Func.")
    );
}

#[test]
fn the_parameter_that_is_not_there_is_a_missing_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameter
    // that the command does not give a value to is a missing one, and the
    // call is refused with "Missing a required parameter.".
    let (report, _desktop) = run("Three(a, b, c) {\n    MsgBox a\n}\nThree (\"a\"),, \"c\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
    assert_eq!(report.error_line, Some(4));
}

#[test]
fn the_name_that_the_script_builds_is_incremented_the_way_a_variable_is() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `y%x%++` answers the
    // old value and stores the new one, and `++y%x%` answers the new one.
    let (report, _desktop) = run(concat!(
        "x := 1\n",
        "y1 := 5\n",
        "a := y%x%++\n",
        "b := ++y%x%\n",
        "FileAppend(a \"|\" b \"|\" y1, \"*\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5|7|7"]);
}

#[test]
fn the_hotkey_of_the_v2_is_a_line_of_its_own() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the hotkey of the v2
    // is a line whose name is followed by two colons, and the action of it is
    // the statement of the line, the block of the same line, the block of the
    // line that comes after it, or any statement (`return`). The `#` of the
    // Win modifier and the `$` of the hook are the first characters of the
    // line, and the fake desktop of the tests cannot watch the keyboard: the
    // script runs and says so in a note, because v2 has no failure of the load
    // for a hotkey.
    let (report, _desktop) = run(concat!(
        "F1::MsgBox \"one\"\n",
        "F2::{\n    MsgBox \"two\"\n}\n",
        "F3::\n{\n    MsgBox \"three\"\n}\n",
        "F4::return\n",
        "^!a::MsgBox \"chord\"\n",
        "F6 Up::MsgBox \"up\"\n",
        "#z::MsgBox \"hash\"\n",
        "$F5::MsgBox \"hook\"\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The action of a hotkey does not run in this runner, and the statement
    // after the definitions does.
    assert_eq!(report.output, vec!["after"]);
    assert!(
        report.notes.iter().any(|note| note.contains("not watched")),
        "{:?}",
        report.notes
    );
}

#[test]
fn the_hotkey_of_the_v2_refuses_the_shapes_the_binary_refuses() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the name that is
    // declared twice is "Duplicate hotkey.", the name that is not a key is
    // "Invalid key name.", and the braces that do not end the line are "This
    // line does not contain a recognized action.".
    let error = parse("F1::MsgBox \"a\"\nF1::MsgBox \"b\"\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Duplicate hotkey.\n     Specifically: F1"
    );
    let error = parse("zzz::MsgBox \"a\"\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid key name.\n     Specifically: zzz"
    );
    let error = parse("F1::{}\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: {}"
    );
    let error = parse("F1::{ MsgBox \"x\" }\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: { MsgBox \"x\" }"
    );
}

#[test]
fn the_hotstring_of_the_v2_reads_the_options_of_the_hotstring_directive() {
    // Measured on the binary of v2.0.19 on 2026-11-10: `#Hotstring X0` turns
    // the code mode of the hotstrings off, so a replacement that ends with `{`
    // is text and not a block, and the statements after the definition are the
    // statements of the script.
    let (report, _desktop) = run(concat!(
        "#Hotstring X0\n",
        ":T:brace::{\n",
        "\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["after"]);
    // And `#Hotstring X` turns the code mode back on: the block is the action.
    let (report, _desktop) = run(concat!(
        "#Hotstring X0\n",
        "#Hotstring X\n",
        "::block::{\n",
        "    MsgBox \"in the block\"\n",
        "}\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["after"]);
}

#[test]
fn the_array_literal_takes_the_items_of_a_spread() {
    // Measured on the binary of v2.0.19 on 2026-11-10: the last item of an
    // array literal may carry a star, and the items of the value it names take
    // its place -- `[a*]` is the items of `a`, the way a `for` loop walks
    // them: an array gives its items, a map its keys, and a class with an
    // `__Enum` its own. The star in the middle is "Missing operand." with the
    // text from the star, and so is a star after a trailing comma.
    let (report, _desktop) = run(concat!(
        "a := [1, 2]\n",
        "b := [a*]\n",
        "c := [0, a*]\n",
        "m := Map(\"k\", 1)\n",
        "d := [m*]\n",
        "class Empty {\n",
        "    static __Enum(n) => (&c) => false\n",
        "}\n",
        "none := [Empty*]\n",
        "MsgBox b.Length \":\" c.Length \":\" d.Length \":\" none.Length\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2:3:1:0"]);
    // The star in the middle is the refusal of the binary, and it is named
    // from the star.
    let failure = parse("a := [1, 2]\nb := [a*, 9]\n").expect_err("refused");
    assert_eq!(
        failure.message,
        "Missing operand.\n     Specifically: *, 9]"
    );
}

#[test]
fn the_hotif_directive_sets_the_condition_of_the_hotkeys() {
    // Measured on the binary of v2.0.19 on 2026-11-10: `#HotIf` takes an
    // expression, and the hotkeys that follow it carry the condition until
    // the next `#HotIf`; the empty directive clears it. The forms this
    // runtime carries are the four window questions with a literal title, and
    // `#HotIfTimeout` is accepted (the question of this runtime answers at
    // once, so the timeout changes nothing).
    let (report, _desktop) = run(concat!(
        "#HotIf WinActive(\"a window\")\n",
        "F1::MsgBox \"one\"\n",
        "#HotIf WinExist(\"a window\")\n",
        "F2::MsgBox \"two\"\n",
        "#HotIf\n",
        "F3::MsgBox \"three\"\n",
        "#HotIfTimeout 100\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["after"]);
    // A condition that is not one of the four is refused rather than
    // registered without its condition.
    let (report, _desktop) = run("#HotIf 1\nF1::MsgBox \"x\"\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "The condition of #HotIf is not carried by this runtime: only the WinActive, WinExist, WinNotActive and WinNotExist forms with a literal title are."
        )
    );
}

#[test]
fn the_exitapp_of_the_v2_takes_its_parentheses() {
    // Measured on the binary of v2.0.19 on 2026-11-10: `ExitApp()` is a call
    // of the function with no code, and `ExitApp(3)` one that ends the script
    // with the three: the empty pair is not the syntax error of a group.
    let (report, _desktop) = run("MsgBox \"before\"\nExitApp()\nMsgBox \"after\"\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["before"]);
    assert_eq!(report.exit_code, Some(0));
    let (report, _desktop) = run("ExitApp(3)\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.exit_code, Some(3));
}

#[test]
fn the_hotstring_of_the_v2_is_a_line_of_raw_text() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `::btw::by the way` is
    // a hotstring, the text of it may hold the two colons again (`::1:::2`),
    // and the backtick escapes the colon (`::5`:::6`). The line without the
    // second pair of colons is "Invalid hotkey." with the line itself.
    let (report, _desktop) = run(concat!(
        "::btw::by the way\n",
        "::1:::2\n",
        "::5`:::6\n",
        ":*:st::starred\n",
        ":?:qm::question\n",
        ":*?:both::both of them\n",
        ":*:i\"q::I \"quote\"\n",
        ":?:; btu::; but\n",
        "::text1::\n",
        "(\n",
        "line one\n",
        "line two\n",
        ")\n",
        "::mf4::{\n",
        "    MsgBox \"x\"\n",
        "}\n",
        "::mf5::\n",
        "{\n",
        "    MsgBox \"y\"\n",
        "}\n",
        "MsgBox \"after\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["after"]);
    let error = parse("::btw\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid hotkey.\n     Specifically: ::btw"
    );
    // Measured on the binary of v2.0.19 on 2026-11-10: the line without the
    // second pair of colons and the one with an empty trigger are
    // "Invalid hotkey." with the line, and a replacement that is empty and has
    // no continuation section is "Hotkey or hotstring is missing its opening
    // brace.".
    let error = parse(":btw::x\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid hotkey.\n     Specifically: :btw::x"
    );
    let error = parse(":*::x\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Invalid hotkey.\n     Specifically: :*::x"
    );
    let error = parse("::btw::\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Hotkey or hotstring is missing its opening brace."
    );
}

#[test]
fn a_text_that_holds_a_hexadecimal_number_is_a_number() {
    // Measured on the binary of v2.0.19 on 2026-11-09: "0x1" + 0 is 1,
    // "0X1F" + 0 is 31, "-0x10" + 0 is -16, " 0x10 " + 0 is 16, `~"0x1"` is
    // -2, and "0x10" * 1 is 16. A text that holds a whole decimal number adds
    // as a whole number while "1e3" is a float. "0x" and "0xzz" hold no
    // number and are the "Expected a Number but got a String." of any other
    // text.
    let (report, _desktop) = run(concat!(
        "a := \"0x1\"\n",
        "b := \"0X1F\"\n",
        "c := \"-0x10\"\n",
        "d := \" 0x10 \"\n",
        "e := ~a\n",
        "f := b + 0\n",
        "g := c + 0\n",
        "h := d + 0\n",
        "i := a * 2\n",
        "t := \"2\"\n",
        "j := t + 1\n",
        "u := \"1e3\"\n",
        "k := u + 0\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    for (name, number) in [
        ("e", -2),
        ("f", 31),
        ("g", -16),
        ("h", 16),
        ("i", 2),
        ("j", 3),
    ] {
        assert_eq!(report.variables.get(name), Some(&Value::Int(number)), "{name}");
    }
    assert_eq!(
        report.variables.get("k"),
        Some(&Value::Float(1000.0))
    );
}

#[test]
fn a_text_that_is_not_a_hexadecimal_number_is_not_a_number() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the prefix with no
    // digit and a digit that is not a hexadecimal one are not numbers.
    let (report, _desktop) = run("a := \"0x\"\nb := a + 0");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got a String.")
    );
    let (report, _desktop) = run("a := \"0xzz\"\nb := a + 0");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a Number but got a String.")
    );
}

#[test]
fn a_loop_may_carry_an_else_that_runs_when_it_took_no_turn() {
    // Measured on the binary of v2.0.19 on 2026-11-09: every loop kind --
    // the counted one, the one without a count, `while`, `for`, `Files` and
    // `Parse` -- may carry an `else`, and the `else` runs exactly when the
    // loop took no turn. The `else` may come after a brace block or after the
    // one statement of the body.
    let (report, _desktop) = run(concat!(
        "seen := \"\"\n",
        "Loop 0 {\n",
        "    seen .= \"L0-body,\"\n",
        "} else {\n",
        "    seen .= \"L0-else,\"\n",
        "}\n",
        "Loop 1 {\n",
        "    seen .= \"L1-body,\"\n",
        "} else {\n",
        "    seen .= \"L1-else,\"\n",
        "}\n",
        "i := 0\n",
        "While (i < 0) {\n",
        "    seen .= \"W-body,\"\n",
        "} else {\n",
        "    seen .= \"W-else,\"\n",
        "}\n",
        "For x in [] {\n",
        "    seen .= \"F-body,\"\n",
        "} else {\n",
        "    seen .= \"F-else,\"\n",
        "}\n",
        "Loop Files \"zzz-nothing-*.txt\" {\n",
        "    seen .= \"LF-body,\"\n",
        "} else {\n",
        "    seen .= \"LF-else,\"\n",
        "}\n",
        "Loop Parse \"\" {\n",
        "    seen .= \"P-body,\"\n",
        "} else {\n",
        "    seen .= \"P-else,\"\n",
        "}\n",
        "Loop 0\n",
        "    seen .= \"S-body,\"\n",
        "else\n",
        "    seen .= \"S-else,\"\n",
        "FileAppend seen, \"*\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["L0-else,L1-body,W-else,F-else,LF-else,P-else,S-else,"]
    );
}

#[test]
fn the_optional_parameter_may_be_left_out() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `f(a, b?)` marks `b`
    // optional, and a call that leaves it out leaves it unset -- `IsSet(b)`
    // is zero -- while a call that gives it a value sets it. The method of a
    // class takes the optional parameter the same way.
    let (report, _desktop) = run(concat!(
        "f(a, b?) {\n",
        "    return IsSet(b) ? \"set:\" b : \"unset\"\n",
        "}\n",
        "class C {\n",
        "    m(a?, b?) {\n",
        "        return IsSet(a) \"|\" IsSet(b)\n",
        "    }\n",
        "}\n",
        "FileAppend(f(1) \"|\" f(1, 2) \"|\" C().m() \"|\" C().m(1, 2), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["unset|set:2|0|0|1|1"]);
}

#[test]
fn a_reading_of_the_optional_parameter_that_is_left_out_is_unset() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a read of the
    // parameter that the call left out fails the way a read of an unset
    // local fails, and no warning is raised for it.
    let (report, _desktop) = run(concat!(
        "f(a?) {\n",
        "    try {\n",
        "        return a\n",
        "    } catch as e {\n",
        "        return Type(e) \": \" e.Message\n",
        "    }\n",
        "}\n",
        "FileAppend(f(), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["UnsetError: This local variable has not been assigned a value."]
    );
}

#[test]
fn the_optional_parameter_has_the_refusals_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the optional one may
    // not carry a default -- `f(a, b? := 2)` is "Missing comma" -- and a
    // required one may not come after it -- `f(a?, b)` is "Parameter default
    // required.", named after the required parameter.
    let error = parse("f(a, b? := 2) {\n    return b\n}\n").expect_err("does not parse");
    assert!(
        error.message.starts_with("Missing comma"),
        "{:?}",
        error.message
    );
    let error = parse("f(a?, b) {\n    return b\n}\n").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter default required.\n     Specifically: b"
    );
    // The arity of the call counts the required parameters alone.
    let (report, _desktop) = run("f(a, b?) {\n    return b\n}\nf()");
    assert!(!report.ok);
    assert!(
        report
            .error
            .as_deref()
            .unwrap_or_default()
            .starts_with("Too few parameters passed to function."),
        "{:?}",
        report.error
    );
}

#[test]
fn the_parameter_that_is_not_there_takes_its_default() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a parameter that
    // has a default takes it, and one that has none is a missing one.
    let (report, _desktop) =
        run("WithDefault(a, b := \"the default\") {\n    MsgBox b\n}\nWithDefault (\"a\"),,");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the default"]);
}

#[test]
fn the_parameter_that_is_not_there_keeps_its_place() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the place of the
    // parameter that is not there is kept, and the number of the arguments
    // is checked before the parameters are bound.
    let (report, _desktop) = run("Two(a, b) {\n    MsgBox a\n}\nTwo (\"a\"),, \"b\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.\n     Specifically: Two")
    );
}

#[test]
fn the_value_that_is_not_there_in_a_group_is_refused_at_the_load() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the value that is
    // not there in the list of a group is a "Syntax error.", and the comma
    // that follows it is named.
    let error = parse("x := (\"a\",, \"c\")").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: , \"c\")");
}

#[test]
fn the_expression_of_a_statement_goes_on() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the expression of a
    // statement begins with the call, and the operators that follow it are
    // a part of it. The value of it is thrown away.
    let (report, _desktop) = run("f() {\n    MsgBox \"[f]\"\n    return 3\n}\nf() + 1");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[f]"]);
}

#[test]
fn the_concatenation_of_a_statement_is_a_part_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the concatenation of
    // v2 is a part of the expression of a statement, and it wants a String.
    let (report, _desktop) = run(
        "class Thing {\n    Method() {\n        return \"m\"\n    }\n}\nt := Thing()\nf() {\n    return t.Method\n}\nf() \"a\"",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Expected a String but got a Func.")
    );
}

#[test]
fn the_comma_ends_the_expression_of_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the expression of a
    // statement ends at the comma, and the rest of the line is the list of
    // v1: it is evaluated, and its value is thrown away.
    let (report, _desktop) =
        run("f() {\n    MsgBox \"[f]\"\n}\ng() {\n    MsgBox \"[g]\"\n}\nf(), g()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[f]", "[g]"]);
}

#[test]
fn the_assignment_that_is_followed_by_a_comma_keeps_its_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the expression of the
    // assignment ends at the comma, and the value of it is the one of the
    // expression.
    let (report, _desktop) = run("x := 1, 2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_member_that_has_parentheses_is_an_expression() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the member that has
    // parentheses is an expression, and the comma ends it: the rest of the
    // line is the list of v1, and it is evaluated.
    let (report, _desktop) = run(
        "class Thing {\n    Method() {\n        MsgBox \"[m]\"\n    }\n}\nt := Thing()\ng() {\n    MsgBox \"[g]\"\n}\nt.Method(), g()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[m]", "[g]"]);
}

#[test]
fn the_member_that_is_a_command_carries_its_arguments() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the member that has
    // no parentheses is a command of v2, and the rest of the line is the
    // list of its arguments.
    let (report, _desktop) =
        run("class Thing {\n    Method(a) {\n        MsgBox a\n    }\n}\nt := Thing()\nt.Method 7");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7"]);
}

#[test]
fn the_parameters_of_the_command_of_a_member_are_separated_by_the_comma() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the space is the
    // concatenation of v2, and the comma is what separates the arguments.
    let (report, _desktop) = run(
        "class Thing {\n    Method(a, b) {\n        MsgBox a \",\" b\n    }\n}\nt := Thing()\nt.Method 7, 8",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7,8"]);
}

#[test]
fn the_parameter_that_is_not_there_in_a_method_is_a_missing_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameter that
    // the command does not give a value to keeps its place, and the method
    // is refused with "Missing a required parameter.".
    let (report, _desktop) = run(
        "class Thing {\n    Method(a, b) {\n        MsgBox a\n    }\n}\nt := Thing()\nt.Method , 7",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
}

#[test]
fn the_line_that_ends_with_a_comma_continues_on_the_next_one() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the line that ends
    // with a comma continues on the next one, and the list of v1 goes on
    // with it.
    let (report, _desktop) = run("x := 1,\n2\nMsgBox x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_number_of_the_arguments_is_checked_before_the_values() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
    // arguments of a call is checked before the arguments are evaluated: v2
    // refuses the script when it loads, and the refusal of the arity comes
    // before the failure of an argument.
    let (report, _desktop) = run("One(a) {\n    MsgBox a\n}\nOne(\"a\", nothingHere)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.\n     Specifically: One")
    );
}

#[test]
fn the_number_of_the_arguments_of_a_method_is_checked_before_the_values() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the number of the
    // arguments of a method is checked before the arguments are evaluated,
    // and the failure carries the name of the method in the \"Extra\".
    let (report, _desktop) = run(
        "class Thing {\n    Method(a) {\n        MsgBox a\n    }\n}\nt := Thing()\nt.Method(\"a\", nothingHere)",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.")
    );
}

#[test]
fn the_built_in_that_takes_the_parameter_that_is_not_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameter that
    // is not there keeps its place in the list of the arguments of a call,
    // and the built-in takes it for the value that is not there.
    let (report, _desktop) = run("FileAppend Format(\"{}{}\", 1,, 2), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_built_in_with_the_parameter_that_is_not_there_at_the_end() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameter that
    // is not there and that is the last one is dropped: the commas at the
    // end of the list add nothing.
    let (report, _desktop) = run("FileAppend Format(\"{}{}\", 1,), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1{}"]);
}

#[test]
fn the_built_in_with_the_parameter_that_is_not_there_alone_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the parameter that
    // is not there and that is the last one is dropped, and the built-in
    // that wants one is refused with the number of the arguments.
    let (report, _desktop) = run("FileAppend StrLen(,), \"*\"");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too few parameters passed to function.\n     Specifically: StrLen")
    );
}

#[test]
fn the_comma_at_the_end_of_the_list_of_the_arguments_adds_nothing_in_the_v2() {
    // Measured on the binary of AutoHotkey v2.0.19 on 2026-11-04: the comma
    // at the end of the list of the arguments adds nothing: the `Add(1,)` is
    // a call of one argument. The v1 counts it, and refuses the call when
    // the list is full.
    let (report, _desktop) = run("Add(a) {\n  return a\n}\nFileAppend Add(1,), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1".to_string()]);
}

#[test]
fn the_object_literal_that_is_written_in_more_than_one_line() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the literal of an
    // object may be written in more than one line: the property may be on
    // the line that follows the brace that opens it, and the brace that
    // closes it may be on a line of its own.
    let (report, _desktop) = run("x := {a: 1\n}\nFileAppend x.a, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    let (report, _desktop) = run("x := {a: 1\n}\nFileAppend x.a, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_block_that_opens_a_line_runs_its_statements() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the brace that opens
    // a line is a block of its own, and the statements between the braces
    // run.
    let (report, _desktop) = run("{\nx := 1\n}\nFileAppend x, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    // The content may be on the line of the brace that opens it.
    let (report, _desktop) = run("{x := 2\n}\nFileAppend x, \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_member_of_a_number_is_read_at_the_run() {
    // Measured on the binary of v2.0.19 on 2026-11-01: `1.abc` is the
    // number 1 and the member that follows it. The script loads, and it
    // fails at the run.
    let (report, _desktop) = run("x := 1.abc");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"Integer\" has no property named \"abc\".")
    );
}

#[test]
fn the_definition_may_be_written_with_the_fat_arrow() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the body of a
    // definition may be an expression, and the fat arrow is what says so:
    // `f(a) => a + 1` is a function that answers the expression of it.
    let (report, _desktop) = run("f(a) => a + 1\nFileAppend f(1), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
    // The parameters are the ones of a function, and the default of one of
    // them is a default.
    let (report, _desktop) = run("f(a, b := 2) => a + b\nFileAppend f(1), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3"]);
    // The method of a class may be written the same way.
    let (report, _desktop) = run("class X {\n    m() => 1\n}\nFileAppend X().m(), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
    // The arrow may be on the line of the parameters or on the one after it.
    let (report, _desktop) = run("f(a)\n=> a + 1\nFileAppend f(1), \"*\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2"]);
}

#[test]
fn the_case_of_a_switch_may_name_many_values() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a case may name
    // many values, and it matches when the subject of the switch is one of
    // them.
    let (report, _desktop) = run(
        "x := 2\nswitch (x) {\n    case 1, 2:\n        FileAppend \"one_or_two=\", \"*\"\n    default:\n        FileAppend \"other=\", \"*\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one_or_two="]);
    let (report, _desktop) = run(
        "x := 9\nswitch (x) {\n    case 1, 2:\n        FileAppend \"one_or_two=\", \"*\"\n    default:\n        FileAppend \"other=\", \"*\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["other="]);
    // The comma at the end of the list adds nothing.
    let (report, _desktop) =
        run("x := 2\nswitch (x) {\n    case 1, 2,:\n        FileAppend \"one_or_two=\", \"*\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one_or_two="]);
}

// -- the words of the v2 ---------------------------------------------------------

#[test]
fn the_reserved_word_refuses_with_the_reason_of_the_word() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the reason belongs
    // to the word, and it is the same in every place where the word is
    // refused.
    let error = parse("if := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a variable name:\n\"if\""
    );
    let error = parse("true := 1").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: true");
    let error = parse("in := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected reserved word.\n     Specifically: in"
    );
}

#[test]
fn the_value_of_a_reserved_word_refuses_with_the_reason_of_the_word() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the reason is the
    // one of the word, in the value as well as in the name.
    let error = parse("x := if").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a variable name:\n\"if\""
    );
    let error = parse("x := and").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: and");
    let error = parse("x := in").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Unexpected reserved word.\n     Specifically: in"
    );
}

#[test]
fn the_name_of_a_function_may_not_be_a_reserved_word() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    // function may not be a word that the language reserves, and the
    // refusal names the function.
    let error = parse("true() {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a function name:\n\"true\""
    );
    let error = parse("in() {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "The following reserved word must not be used as a function name:\n\"in\""
    );
    // The words that the v2 does not reserve are names.
    assert!(parse("until() {\n}").is_ok());
    assert!(parse("catch() {\n}").is_ok());
    assert!(parse("exitapp() {\n}").is_ok());
}

#[test]
fn the_words_that_the_v2_does_not_reserve_are_names() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the `elseif`, the
    // `default` and the `extends` are names in v2, and the branch of an `if`
    // is written with two words.
    assert!(parse("elseif := 1").is_ok());
    assert!(parse("default := 1").is_ok());
    assert!(parse("extends := 1").is_ok());
    assert!(parse("class default {\n}").is_ok());
    assert!(parse("class elseif {\n}").is_ok());
    assert!(parse("class extends {\n}").is_ok());
    assert!(parse("if 0 {\n} else if 1 {\n}").is_ok());
}

#[test]
fn the_output_variable_of_an_assignment_may_not_be_a_function() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of a
    // function of the machine may not be the output variable of an
    // assignment, and the plain assignment and the compound ones say it in
    // their own words.
    let error = parse("MsgBox := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Func cannot be used as an output variable.\n     Specifically: MsgBox"
    );
    let error = parse("MsgBox += 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Func cannot be assigned a value.\n     Specifically: MsgBox"
    );
    let error = parse("exitapp := 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Func cannot be used as an output variable.\n     Specifically: exitapp"
    );
}

#[test]
fn a_local_may_be_named_after_a_function_of_the_machine() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the refusal is the
    // one of the global scope, and a local one may carry the name.
    assert!(parse("f() {\nMsgBox := 1\n}").is_ok());
    assert!(parse("f() {\nclass := 1\n}").is_ok());
    // The name that is declared global is a global one.
    let error = parse("f() {\nglobal MsgBox\nMsgBox := 1\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Func cannot be used as an output variable.\n     Specifically: MsgBox"
    );
}

#[test]
fn a_parameter_may_be_named_after_a_function_or_a_class() {
    // Measured on the binary of v2.0.19 on 2026-11-01: `f(exitapp)` and
    // `f(class)` are both of them.
    assert!(parse("f(exitapp) {\n}").is_ok());
    assert!(parse("f(class) {\n}").is_ok());
}

#[test]
fn the_name_of_a_class_may_be_the_name_of_a_function() {
    // Measured on the binary of v2.0.19 on 2026-11-01: `class ExitApp { }`
    // is a class, and the `Class` is a class of the dialect and a class of
    // the script may not have its name.
    assert!(parse("class exitapp {\n}").is_ok());
    let error = parse("class class {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This class declaration conflicts with an existing Class.\n     Specifically: Class"
    );
}

#[test]
fn the_default_of_a_switch_is_a_name_and_a_colon() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the label of the
    // default case is the name `default` and a colon, and the name alone in
    // the body of the switch is not a label.
    let statements = parse("switch 1 {\ndefault:\n}").expect("parses");
    let Node::Switch { cases, .. } = &statements[0] else {
        panic!("a switch")
    };
    assert_eq!(cases.len(), 1);
    assert!(cases[0].values.is_empty());
    let error = parse("switch 1 {\ndefault\n}").expect_err("does not parse");
    assert_eq!(error.message, "Expected Case/Default.");
}

#[test]
fn a_case_may_name_no_value() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the values of a case
    // may be none, and the colon is what says where the label of the case
    // ends.
    let statements = parse("switch 1 {\ncase:\n}").expect("parses");
    let Node::Switch { cases, .. } = &statements[0] else {
        panic!("a switch")
    };
    assert_eq!(cases.len(), 1);
    assert!(cases[0].values.is_empty());
    let error = parse("switch 1 {\ncase\n}").expect_err("does not parse");
    assert_eq!(error.message, "Missing \":\"\n     Specifically: case");
}

#[test]
fn the_elseif_is_a_name_and_not_a_branch() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the `elseif` is a
    // name in v2, and a script may write to it.
    let (report, desktop) = run("elseif := \"one\"\nSend elseif");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one"]);
}

#[test]
fn the_default_in_the_body_of_a_case_is_a_statement() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the label of the
    // default case is the name and a colon, and the name alone in the body
    // of a case is a statement.
    let (report, desktop) = run("default := \"one\"\nswitch 1 {\ncase 1:\nSend default\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one"]);
}

#[test]
fn a_return_that_carries_a_value_outside_a_function_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a return that carries
    // a value outside a function is refused at the load, and the reason
    // comes before the one of the expression that it carries.
    let error = parse("return 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Return's parameter should be blank except inside a function."
    );
    assert_eq!(error.line, 1);
}

#[test]
fn a_return_inside_a_function_carries_the_value() {
    // Measured on the binary of v2.0.19 on 2026-11-01: a return inside a
    // function may carry a value.
    let (report, _) = run("f() {\n    return 1\n}\nf()");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_break_that_carries_a_parenthesis_is_a_break() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the v2 takes the
    // parenthesis of a break and of a continue, and it ignores what is
    // inside it.
    let (report, _) = run("Loop 2 {\n    break(1)\n}");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_break_with_a_parenthesis_outside_a_loop_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the refusal of a break
    // that is not inside a loop comes after the parenthesis.
    let error = parse("break()").expect_err("does not parse");
    assert_eq!(error.message, "Break/Continue must be enclosed by a Loop.");
    assert_eq!(error.line, 1);
}

#[test]
fn a_try_that_carries_a_parenthesis_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the try takes no
    // parenthesis, and the dialect names the statement that carries it.
    let error = parse("try() {\n}").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: try() {");
    assert_eq!(error.line, 1);
}

#[test]
fn a_finally_that_carries_a_parenthesis_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the finally takes no
    // parenthesis, and the dialect names the statement that carries it.
    let error = parse("try {\n}\nfinally()").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: finally()");
    assert_eq!(error.line, 3);
}

#[test]
fn an_else_that_carries_a_parenthesis_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the else takes no
    // parenthesis, and the dialect names the statement that carries it.
    let error = parse("if (1) {\n}\nelse()").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: else()");
    assert_eq!(error.line, 3);
}

#[test]
fn a_switch_without_a_subject_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the switch needs a
    // subject, and it names itself when there is none.
    let error = parse("switch").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Switch\"");
    assert_eq!(error.line, 1);
}

#[test]
fn a_condition_that_is_empty_is_refused_after_the_body() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the v2 reads the body
    // of the construct before it reads the condition, and the refusal of a
    // condition that is empty comes after the one of the body.
    let error = parse("if()").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"If\"");
    let error = parse("if()\n{\n}").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: )");
}

#[test]
fn a_for_whose_list_is_empty_names_the_text_that_it_read() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the v2 names the text
    // that it read, and the text of the for ends at the parenthesis that
    // closes the list.
    let error = parse("for() {\n}").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: for(");
    let error = parse("for(1) {\n}").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: for(1");
}

#[test]
fn the_values_of_a_case_that_are_empty_name_the_closing_parenthesis() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the text that is
    // named is the closing parenthesis and not the rest of the line.
    let error = parse("switch 1 {\n    case(): x := 1\n}").expect_err("does not parse");
    assert_eq!(error.message, "Syntax error.\n     Specifically: )");
    assert_eq!(error.line, 2);
}

#[test]
fn the_catch_that_has_no_body_names_itself() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the catch needs a
    // body, and the construct that has none says so in its own words.
    let error = parse("try {\n}\ncatch").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Catch\"");
    assert_eq!(error.line, 3);
}

#[test]
fn the_finally_that_has_no_body_names_itself() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the finally needs a
    // body too.
    let error = parse("try {\n}\nfinally").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Finally\"");
    assert_eq!(error.line, 3);
}

#[test]
fn the_else_that_has_no_body_names_itself() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the else needs a
    // body as well.
    let error = parse("if (1) {\n}\nelse").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Else\"");
    assert_eq!(error.line, 3);
}

#[test]
fn the_else_of_a_try_needs_a_catch_before_it() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the else of a try
    // needs a catch before it, and it names itself when there is none.
    let error = parse("try {\n}\nelse {\n}").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"Else\"");
    // The catch before it is what makes the difference.
    let statements = parse("try {\n}\ncatch {\n}\nelse {\n}").expect("parses");
    assert_eq!(statements.len(), 1);
}

#[test]
fn the_content_of_a_block_may_be_on_the_line_of_the_brace_that_opens_it() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the content may be
    // there when the brace is at the start of its line.
    let statements = parse("if (1)\n{ x := 1\n}").expect("parses");
    assert_eq!(statements.len(), 1);
    // And the construct refuses it when the brace is not at the start of
    // its line.
    let error = parse("if (1) { x := 1 }").expect_err("does not parse");
    assert_eq!(error.message, "Unexpected \"If\"");
}

#[test]
fn the_body_of_a_try_that_is_on_one_line_says_that_the_brace_is_missing() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the block that is on
    // the line of the construct says that the brace is missing.
    let error = parse("try { x := 1 }").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"");
    // And the block that is empty is not a block at all.
    let error = parse("try {}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: }"
    );
}

#[test]
fn the_block_that_is_not_closed_names_the_construct_that_opened_it() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the block that is
    // empty names the construct that opened it, and the block that has
    // content names nothing.
    let error = parse("if (1) {").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"\n     Specifically: if (1) {");
    let error = parse("if (1) {\n    x := 1").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"");
}

#[test]
fn the_goto_whose_parenthesis_is_spaced_looks_for_the_whole_text() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the parenthesis
    // that is spaced is a part of the text of the name of the label.
    let error = parse("Goto (nowhere)").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Label not found in current scope.\n     Specifically: (nowhere)"
    );
}

#[test]
fn the_name_of_a_member_that_is_a_text_is_an_illegal_character() {
    // Measured on the binary of v2.0.19 on 2026-11-02: the name of a member
    // may not be a text: the `a.'b'` is refused with "Illegal character in
    // expression." and the text is named as it was written.
    let error = parse("a := 1\nx := a.'b'").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Illegal character in expression.\n     Specifically: 'b'"
    );
}

#[test]
fn the_statement_that_begins_with_a_member_whose_name_is_not_a_name_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-11-02: the statement that
    // begins with a member whose name is not a name holds no action: the
    // `a.'b' := 5` is refused with "This line does not contain a recognized
    // action." and the whole line is named.
    let error = parse("a := 1\na.'b' := 5").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: a.'b' := 5"
    );
}

#[test]
fn the_statement_that_begins_with_a_member_whose_name_is_not_a_name_and_a_sum_holds_no_action() {
    // Measured on the binary of v2.0.19 on 2026-11-02: the `a.'b' + 1` is a
    // statement that holds no action, and the whole line is named.
    let error = parse("a := 1\na.'b' + 1").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This line does not contain a recognized action.\n     Specifically: a.'b' + 1"
    );
}

/// The mode of the loop of the files is a set of letters, and the letter that
/// is not one of them is refused when the loop runs.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the mode is the
/// "R" that walks into the folders, the "D" and the "K" that give the
/// folders, the "F" and the "V" that give the files, and the spaces between
/// them are ignored. The "A" is not one of them.
#[test]
fn the_mode_of_the_loop_of_the_files_carries_the_letters_of_v2() {
    let (report, _desktop) = run("Loop Files \"*.txt\", \"A\" {\n}\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "the mode of a `Loop Files` is the \"R\", the \"D\", the \"F\", the \"V\" or the \"K\", and the letter \"A\" is not one of them."
        )
    );
    assert_eq!(report.error_line, Some(1));
    // The spaces between the letters are ignored.
    let (report, desktop) =
        run("Loop Files \"*.txt\", \"D R\" {\n  Send \"%A_LoopFileName%\"\n}\n");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        desktop
            .calls
            .contains(&("list_files".to_string(), "/work|*.txt|D R".to_string())),
        "{:?}",
        desktop.calls
    );
}

/// The `#Warn` takes the type of the warning and the mode of it.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the type that
/// is not one of them is refused at the load with "Parameter #1 invalid." and
/// the source of the line up to the comma, and the mode that is not one of
/// them with "Parameter #2 invalid." and the text that was written.
#[test]
fn the_type_of_the_warn_that_is_not_one_of_them_is_refused() {
    let error = parse("#Warn Nope").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #Warn Nope"
    );
    assert_eq!(error.line, 1);
}

/// The text that the refusal of the type names is the source of the line.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the case of
/// the script is the case of the text -- "#warn nope" -- and the spaces of
/// the line are the spaces of the text -- "#Warn    Nope".
#[test]
fn the_type_of_the_warn_is_named_with_the_source_of_the_line() {
    let error = parse("#warn nope").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #warn nope"
    );
    let error = parse("#WARN NOPE").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #WARN NOPE"
    );
    let error = parse("#Warn    Nope").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #Warn    Nope"
    );
    // The text stops at the comma.
    let error = parse("#Warn Nope , StdOut").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #Warn Nope"
    );
}

/// The mode of the `#Warn` that is not one of them is refused.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the text that
/// the refusal names is the text that comes after the comma, and it is the
/// whole of it -- "StdOut, Extra".
#[test]
fn the_mode_of_the_warn_that_is_not_one_of_them_is_refused() {
    let error = parse("#Warn All, StdOut, Extra").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #2 invalid.\n     Specifically: StdOut, Extra"
    );
    let error = parse("#Warn All, StdOut Extra").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #2 invalid.\n     Specifically: StdOut Extra"
    );
}

/// A type that is empty is not a type, and the mode is the text that comes
/// after the comma.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the "#Warn ,
/// StdOut" is not a failure, and the "#Warn ,Nope" is a refusal of the mode.
#[test]
fn the_empty_type_of_the_warn_is_not_a_type() {
    parse("#Warn , StdOut").expect("parses");
    parse("#Warn All").expect("parses");
    let error = parse("#Warn ,Nope").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #2 invalid.\n     Specifically: Nope"
    );
}

/// The line that never runs is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning
/// is of the line that comes right after the statement that leaves the level,
/// and the mode of the "StdOut" writes it to the standard output.
#[test]
fn the_line_that_never_runs_is_warned_about() {
    let (report, _desktop) = run("#Warn Unreachable, StdOut\nreturn\nx := 1\ny := 2\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 3);
    assert_eq!(
        report.warnings[0].message,
        "This line will never execute, due to Return preceding it."
    );
    assert_eq!(
        report.output,
        vec![
            "(3) : ==> Warning: This line will never execute, due to Return preceding it."
                .to_string()
        ]
    );
}

/// The warning is of the level of the statement that leaves it.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the break of
/// a loop warns the line that comes after it in the body of the loop, and the
/// return of a block does not warn the line that comes after the block.
#[test]
fn the_warning_of_the_unreachable_line_is_of_its_own_level() {
    let (report, _desktop) = run("#Warn Unreachable, StdOut\nloop 3 {\nbreak\nx := 1\n}\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 4);
    assert_eq!(
        report.warnings[0].message,
        "This line will never execute, due to Break preceding it."
    );
    let (report, _desktop) = run("#Warn Unreachable, StdOut\nif 1 {\nreturn\n}\nx := 1\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

/// The label that is the target of the jump is not warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the `Goto`
/// warns the line that comes after it, and the label is the line that the
/// jump comes to.
#[test]
fn the_label_that_is_the_target_of_the_jump_is_not_warned_about() {
    let (report, _desktop) = run("#Warn Unreachable, StdOut\ngoto later\nx := 1\nlater:\ny := 2\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 3);
    assert_eq!(
        report.warnings[0].message,
        "This line will never execute, due to Goto preceding it."
    );
}

/// The warning of the unreachable line is off when the directive says so.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the mode of
/// the "Off" is no warning at all.
#[test]
fn the_warning_of_the_unreachable_line_is_off_when_the_directive_says_so() {
    let (report, _desktop) = run("#Warn Unreachable, Off\nreturn\nx := 1\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

/// The variable that is read and never written is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning
/// is given before the failure of the run, and the text of it names the kind
/// of the variable.
#[test]
fn the_variable_that_is_read_and_never_written_is_warned_about() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nx := nope\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 2);
    assert_eq!(
        report.warnings[0].message,
        "This global variable appears to never be assigned a value.\n     Specifically: nope"
    );
    assert_eq!(
        report.output,
        vec!["(2) : ==> Warning: This global variable appears to never be assigned a value.\n     Specifically: nope".to_string()]
    );
}

/// The local variable that is read and never written is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the text of
/// the warning of a local says "This local variable appears to never be
/// assigned a value.".
#[test]
fn the_local_variable_that_is_read_and_never_written_is_warned_about() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nf() {\n  x := nope\n}\nf()\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 3);
    assert_eq!(
        report.warnings[0].message,
        "This local variable appears to never be assigned a value.\n     Specifically: nope"
    );
}

/// The variable that is read and never written is warned about when the
/// script is read.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning is
/// given when the script is read, and not when the variable is read: the
/// `if 0 { x := nope }` is warned about, and the read of it never happens.
#[test]
fn the_variable_that_is_never_run_is_warned_about() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nif 0 {\n  x := nope\n}\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 3);
    assert_eq!(
        report.warnings[0].message,
        "This global variable appears to never be assigned a value.\n     Specifically: nope"
    );
}

/// The variable that is written after the read is not warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the name that
/// the scope writes is a local of it, and the warning is not given for it.
#[test]
fn the_variable_that_is_written_after_the_read_is_not_warned_about() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nx := nope\nnope := 5\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

/// The name that the script has seen is a global one.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the script reads
/// the name and the function reads it after, and there is one warning only.
#[test]
fn the_name_that_the_script_has_seen_is_not_warned_about_again() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nx := nope\nf() {\n  y := x\n}\nf()\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 2);
}

/// The variable of a class that is read and never written is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the body of the
/// class is a scope of its own, and the name that it reads and that the script
/// does not write is a local of it.
#[test]
fn the_variable_of_a_class_that_is_never_written_is_warned_about() {
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nclass A {\n  y := nope\n}\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 3);
    assert_eq!(
        report.warnings[0].message,
        "This local variable appears to never be assigned a value.\n     Specifically: nope"
    );
}

/// The type of the warning that the v2 does not know is refused.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the types of
/// the v1 are not types of the v2, and the directive that names one of them
/// is refused at the load.
#[test]
fn the_type_of_the_warn_that_the_v2_does_not_know_is_refused() {
    let error = parse("#Warn UseUnsetGlobal, StdOut").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Parameter #1 invalid.\n     Specifically: #Warn UseUnsetGlobal"
    );
}

/// The type of the warning that the tree carries is not named in the report.
///
/// The report names the types of the warning that the tree does not carry, and
/// the tree carries them all: the note is only there for the types that are
/// missing.
#[test]
fn the_type_of_the_warn_that_the_tree_carries_is_not_named_in_the_report() {
    let (report, _desktop) = run("#Warn LocalSameAsGlobal, StdOut\nx := 1\n");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        !report
            .notes
            .iter()
            .any(|note| note.contains("LocalSameAsGlobal")),
        "{:?}",
        report.notes
    );
}

/// The local variable that has the name of a global one is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the warning
/// is given once, when the script is read, and the text of it names the
/// variable and the function of it -- "x  (in function f)". The line of it is
/// the line of the write of the local.
#[test]
fn the_local_that_has_the_name_of_a_global_is_warned_about() {
    let (report, _desktop) =
        run("#Warn LocalSameAsGlobal, StdOut\nx := 1\nf() {\n  x := 2\n}\nf()\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 4);
    assert_eq!(
        report.warnings[0].message,
        "This local variable has the same name as a global variable.\n     Specifically: x  (in function f)"
    );
    assert_eq!(
        report.output,
        vec!["(4) : ==> Warning: This local variable has the same name as a global variable.\n     Specifically: x  (in function f)".to_string()]
    );
}

/// The name that is only read is not a local of the v2.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the v2 reads
/// the scope of the script when the name is only read, and the name that it
/// only reads is not a local of it.
#[test]
fn the_local_that_is_only_read_is_not_warned_about() {
    let (report, _desktop) =
        run("#Warn LocalSameAsGlobal, StdOut\nx := 1\nf() {\n  y := x\n}\nf()\n");
    assert!(report.ok, "{:?}", report.error);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

/// The local of a method that has the name of a global one is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the name of the
/// method is "A.Prototype.m" -- the class, the prototype and the method -- and
/// the line of the warning is the line of the write of the local.
#[test]
fn the_local_of_a_method_that_has_the_name_of_a_global_is_warned_about() {
    let (report, _desktop) =
        run("#Warn LocalSameAsGlobal, StdOut\nx := 1\nclass A {\n  m() {\n    x := 2\n  }\n}\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 5);
    assert_eq!(
        report.warnings[0].message,
        "This local variable has the same name as a global variable.\n     Specifically: x  (in function A.Prototype.m)"
    );
}

/// The local of a getter that has the name of a global one is warned about.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the name of the
/// getter is "A.Prototype.p.get", and the line of the warning is the line of
/// the write of the local.
#[test]
fn the_local_of_a_getter_that_has_the_name_of_a_global_is_warned_about() {
    let (report, _desktop) = run(
        "#Warn LocalSameAsGlobal, StdOut\nx := 1\nclass A {\n  p {\n    get {\n      x := 2\n    }\n  }\n}\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0].line, 6);
    assert_eq!(
        report.warnings[0].message,
        "This local variable has the same name as a global variable.\n     Specifically: x  (in function A.Prototype.p.get)"
    );
}

/// The local that is declared global is not a local.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the local that
/// the function declares global is not warned about.
#[test]
fn the_local_that_is_declared_global_is_not_warned_about() {
    let (report, _desktop) =
        run("#Warn LocalSameAsGlobal, StdOut\nx := 1\nf() {\n  global x\n  x := 2\n}\nf()\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    // The local without a global is not warned about either.
    let (report, _desktop) = run("#Warn LocalSameAsGlobal, StdOut\nf() {\n  x := 2\n}\nf()\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

/// The body of the getter is a scope of the v2.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the `return` of the body
/// of the getter is the value of the property, and the name that the body
/// writes is a local of it.
#[test]
fn the_body_of_the_getter_is_a_scope_of_the_v2() {
    let (report, desktop) = run(
        "class Thing {\n  p {\n    get {\n      return 1\n    }\n  }\n}\ninst := Thing()\nSend inst.p",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["1"]);
}

/// The getter that is on the same line as the name is refused.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the script does not load,
/// and the member is named whole: "p { get { return 1 } }".
#[test]
fn the_getter_that_is_on_the_same_line_as_the_name_is_refused() {
    let error =
        parse("class Thing {\n  p { get {\n      return 1\n  } }\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid method, class or property definition.\n     Specifically: p { get { return 1 } }"
    );
    assert_eq!(error.line, 2);
}

/// The body of the getter that is on the line of the member is refused.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the script does not load,
/// and the line of the member is named: "get { return 1 }".
#[test]
fn the_body_of_the_getter_that_is_on_the_line_of_the_member_is_refused() {
    let error =
        parse("class Thing {\n  p {\n    get { return 1 }\n  }\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "Not a valid property getter/setter.\n     Specifically: get { return 1 }"
    );
    assert_eq!(error.line, 3);
}

/// The setter of the v2 writes the value that is given to it.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the body of the setter is
/// a scope, and the name "value" is the value that is written.
#[test]
fn the_setter_of_the_v2_writes_the_value_that_is_given_to_it() {
    let (report, desktop) = run(
        "class Thing {\n  p {\n    set {\n      this.stored := value\n    }\n  }\n}\ninst := Thing()\ninst.p := 7\nSend inst.stored",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["7"]);
}

/// The value of the setter is a parameter of it, and it is not warned about.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the name "value" is the
/// value that is written, and the v2 does not warn about it.
#[test]
fn the_value_of_the_setter_is_not_warned_about_in_the_v2() {
    let (report, _desktop) = run(
        "#Warn VarUnset, StdOut\nclass Thing {\n  p {\n    set {\n      this.stored := value\n    }\n  }\n}\ninst := Thing()\ninst.p := 7\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
}

/// The `this` of a class is not a variable.
///
/// Measured on the binary of v2.0.19 on 2026-11-05: the `this` of the body of
/// a class, the one of a method, the one of a static method and the one of a
/// getter are not warned about, and the `this` of a function is.
#[test]
fn the_this_of_a_class_is_not_a_variable_of_the_v2() {
    let (report, _desktop) =
        run("#Warn VarUnset, StdOut\nclass Thing {\n  m() {\n    this.x := 1\n  }\n}\n");
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    let (report, _desktop) = run(
        "#Warn VarUnset, StdOut\nclass Thing {\n  p {\n    get {\n      this.x := 1\n    }\n  }\n}\n",
    );
    assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    // The `this` of a function is a variable that has no value.
    let (report, _desktop) = run("#Warn VarUnset, StdOut\nf() {\n  y := this\n}\n");
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(
        report.warnings[0].message,
        "This local variable appears to never be assigned a value.\n     Specifically: this"
    );
}

// -- break N, continue N, and the spread of a call ---------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09: `break N`/`continue N`
// name the level of the loop that the flow leaves or continues, the level is
// an integer literal that is refused at the load when it is zero or above the
// count of the loops around it, and a `switch` is transparent to a break and
// to a continue. `f(arr*)` spreads every item of the value of the last
// argument into the arguments of the call; the spread is enumerated the way a
// `for` loop enumerates, and a middle spread is refused at the load with
// "Missing operand.".

#[test]
fn a_break_of_two_leaves_the_two_loops() {
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "Loop 2 {\n",
        "    Loop 2 {\n",
        "        out .= A_Index\n",
        "        break 2\n",
        "    }\n",
        "    out .= \"x\"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_continue_of_two_continues_the_outer_loop() {
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "Loop 3 {\n",
        "    Loop 3 {\n",
        "        If A_Index = 2\n",
        "            continue 2\n",
        "        out .= \"in\" A_Index \" \"\n",
        "    }\n",
        "    out .= \"after\" A_Index \" \"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["in1 in1 in1 "]);
}

#[test]
fn a_plain_break_and_a_plain_continue_are_the_level_one() {
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "Loop 2 {\n",
        "    Loop 2 {\n",
        "        out .= A_Index\n",
        "        break 1\n",
        "    }\n",
        "    out .= \"-\"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1-1-"]);
}

#[test]
fn a_level_of_zero_is_refused() {
    let error = parse("Loop 2 {\n    break 0\n}").expect_err("does not parse");
    assert_eq!(error.message, "Parameter #1 invalid.");
    assert_eq!(error.line, 2);
    let error = parse("Loop 2 {\n    continue 0\n}").expect_err("does not parse");
    assert_eq!(error.message, "Parameter #1 invalid.");
    assert_eq!(error.line, 2);
}

#[test]
fn a_level_above_the_loops_around_it_is_refused() {
    let error = parse("Loop 2 {\n    break 2\n}").expect_err("does not parse");
    assert_eq!(error.message, "Parameter #1 invalid.");
    assert_eq!(error.line, 2);
    let error = parse("Loop 2 {\n    Loop 2 {\n        continue 3\n    }\n}")
        .expect_err("does not parse");
    assert_eq!(error.message, "Parameter #1 invalid.");
    assert_eq!(error.line, 3);
}

#[test]
fn a_level_outside_a_loop_names_the_enclosure() {
    // The refusal of the enclosure comes before the level: the real binary
    // answers "Break/Continue must be enclosed by a Loop." for a break of any
    // level that is not inside one.
    let error = parse("break 2").expect_err("does not parse");
    assert_eq!(error.message, "Break/Continue must be enclosed by a Loop.");
}

#[test]
fn a_break_of_a_level_inside_a_switch_leaves_the_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the switch does not
    // take a break of its own -- a case body ends at the next case -- and the
    // break goes on to the loop that holds the switch.
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "Loop 2 {\n",
        "    switch A_Index {\n",
        "        case 1:\n",
        "            out .= \"one-\"\n",
        "            break\n",
        "        default:\n",
        "            out .= \"other-\"\n",
        "    }\n",
        "    out .= \"after\" A_Index \"-\"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one-"]);
    // A level of two inside a switch inside one loop is above the one loop
    // around it: the switch does not count as a level.
    let error = parse("Loop 2 {\n    switch 1 {\n        case 1:\n            break 2\n    }\n}")
        .expect_err("does not parse");
    assert_eq!(error.message, "Parameter #1 invalid.");
}

#[test]
fn a_level_comes_out_of_a_parse_loop_and_a_while_loop() {
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "Loop 2 {\n",
        "    Loop Parse \"a,b,c\", \",\" {\n",
        "        out .= A_LoopField\n",
        "        If A_LoopField = \"b\"\n",
        "            break 2\n",
        "    }\n",
        "    out .= \"x\"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["ab"]);
    let (report, _desktop) = run(concat!(
        "out := \"\"\n",
        "i := 0\n",
        "Loop 3 {\n",
        "    i += 1\n",
        "    j := 0\n",
        "    while (j < 3) {\n",
        "        j += 1\n",
        "        If j = 2\n",
        "            continue 2\n",
        "        out .= i \".\" j \" \"\n",
        "    }\n",
        "    out .= \"after-\"\n",
        "}\n",
        "FileAppend(out, \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1.1 2.1 3.1 "]);
}

#[test]
fn the_spread_of_a_call_spreads_the_array() {
    let (report, _desktop) = run(concat!(
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "arr := [1, 2]\n",
        "FileAppend(F(arr*), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1/2"]);
}

#[test]
fn the_spread_of_a_call_goes_through_a_value_and_a_variadic() {
    let (report, _desktop) = run(concat!(
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "Sum(rest*) {\n",
        "    s := 0\n",
        "    for v in rest\n",
        "        s += v\n",
        "    return s\n",
        "}\n",
        "arr := [1, 2]\n",
        "g := F\n",
        "FileAppend(g(arr*) \"|\" Sum(arr*) \"|\" Abs([-3]*), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1/2|3|3"]);
}

#[test]
fn the_spread_of_a_map_gives_its_keys() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the spread enumerates
    // the value the way a `for` loop does, and the `for` over a map walks its
    // keys.
    let (report, _desktop) = run(concat!(
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "m := Map(\"a\", 1, \"b\", 2)\n",
        "FileAppend(F(m*), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a/b"]);
}

#[test]
fn the_spread_of_an_empty_array_leaves_the_call_without_a_value() {
    let (report, _desktop) = run(concat!(
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "arr := []\n",
        "FileAppend(F(arr*), \"*\")",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
}

#[test]
fn a_spread_of_a_value_that_cannot_be_enumerated_is_refused() {
    let (report, _desktop) = run(concat!(
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "x := 5\n",
        "FileAppend(F(x*), \"*\")",
    ));
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Value not enumerable."));
}

#[test]
fn a_middle_spread_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-09: only the last
    // argument may carry the star, and the refusal names the rest of the
    // line from the star -- also inside the list of another call.
    let error = parse("F(a, b) {\n    return a\n}\narr := [1, 2]\nF(arr*, 9)")
        .expect_err("does not parse");
    assert!(
        error.message.starts_with("Missing operand.\n     Specifically: *, 9)"),
        "{:?}",
        error.message
    );
    let error = parse("F(a, b) {\n    return a\n}\narr := [1, 2]\nEmit(F(arr*, 9))")
        .expect_err("does not parse");
    assert!(
        error.message.starts_with("Missing operand."),
        "{:?}",
        error.message
    );
}

#[test]
fn the_spread_of_a_method_and_of_a_constructor_works() {
    let (report, _desktop) = run(concat!(
        "class C {\n",
        "    __New(a, b) {\n",
        "        this.v := a \"-\" b\n",
        "    }\n",
        "    m(a, b) {\n",
        "        return a \"+\" b\n",
        "    }\n",
        "}\n",
        "arr := [4, 5]\n",
        "inst := C(arr*)\n",
        "FileAppend(inst.v \"|\" inst.m(arr*), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4-5|4+5"]);
}

#[test]
fn a_spread_is_a_value_where_a_reference_is_wanted() {
    // Measured on the binary of v2.0.19 on 2026-11-09: an item of an array is
    // not a variable reference, and a spread argument is a value.
    let (report, _desktop) = run(concat!(
        "Swap(&x, &y) {\n",
        "    t := x\n",
        "    x := y\n",
        "    y := t\n",
        "}\n",
        "a := 1\n",
        "b := 2\n",
        "arr := [a, b]\n",
        "Swap(arr*)",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "Parameter #1 of Swap requires a variable reference, but received an Integer."
        )
    );
}

#[test]
fn the_spread_of_a_class_enumerates_its_values() {
    let (report, _desktop) = run(concat!(
        "class E {\n",
        "    __Enum(n) {\n",
        "        return [7, 8].__Enum(n)\n",
        "    }\n",
        "}\n",
        "F(a, b) {\n",
        "    return a \"/\" b\n",
        "}\n",
        "FileAppend(F(E()*), \"*\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7/8"]);
}
#[test]
fn the_mode_of_the_title_is_a_setting_of_the_script() {
    let _turn = THE_TITLE_MODE_TURN
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // Measured on the binary of v2.0.19 on 2026-11-09: the command may be
    // written as a statement without the parentheses, and the mode that it
    // sets is the one that the built-in variable `A_TitleMatchMode` answers.
    let (report, _desktop) = run(concat!(
        "SetTitleMatchMode 3\n",
        "m := A_TitleMatchMode\n",
        "SetTitleMatchMode 2\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("m"), Some(&Value::Int(3)));
}

#[test]
fn the_mode_of_the_title_may_be_the_word_of_the_patterns() {
    let _turn = THE_TITLE_MODE_TURN
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // Measured on the binary of v2.0.19 on 2026-11-09: the word "RegEx" is the
    // mode of the patterns, and the built-in variable answers the word.
    let (report, _desktop) = run(concat!(
        "SetTitleMatchMode \"RegEx\"\n",
        "m := A_TitleMatchMode\n",
        "SetTitleMatchMode 2\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("m"), Some(&Value::Text("RegEx".into())));
}
#[test]
fn a_hotkey_of_a_suspend_exempt_section_is_exempt() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `#SuspendExempt` covers
    // the hotkeys that come after it, and `#SuspendExempt False` stops
    // covering them; `Suspend 1` turns the state on and `Suspend 0` turns it
    // off. The runner has no keyboard, so the state is what is observable.
    let (report, _desktop) = run(concat!(
        "#SuspendExempt\n",
        "F1::return\n",
        "#SuspendExempt False\n",
        "F2::return\n",
        "Suspend 1\n",
        "a := A_IsSuspended\n",
        "Suspend 0\n",
        "b := A_IsSuspended\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("b"), Some(&Value::Int(0)));
}

#[test]
fn a_suspend_exempt_that_is_not_a_boolean_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a value that is not a
    // boolean is the load failure "Parameter #1 invalid.".
    let (report, _desktop) = run("#SuspendExempt Bogus\n");
    assert_eq!(report.error.as_deref(), Some("Parameter #1 invalid."));
}