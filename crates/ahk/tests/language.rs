//! The language and the runtime, against a fake desktop.
//!
//! No SSH, no box, no ydotool: the desktop is a fake that records the
//! calls, so these tests are about what the language means, not about what
//! the desktop does. The desktop part is verified against the real box
//! separately.
//!
//! These are a port of the archived Python tests, and the two
//! implementations have to agree.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, TitleMatch, Windows,
};
use ahk::desktop::WindowAnswer;
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
    fn list_files(&mut self, folder: &str, pattern: &str, mode: &str) -> Result<Vec<String>, String> {
        self.calls.push(("list_files".into(), format!("{folder}|{pattern}|{mode}")));
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
    fn mouse_down(&mut self, _x: Option<i64>, _y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls.push(("mouse_down".into(), button.into()));
        Ok(format!("{button} is down"))
    }
    fn mouse_up(&mut self, _x: Option<i64>, _y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls.push(("mouse_up".into(), button.into()));
        Ok(format!("{button} is up"))
    }
    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
        self.calls.push(("click".into(), format!("{x},{y},{button},{count}")));
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
    fn capture(&mut self, x: i64, y: i64, width: i64, height: i64, filename: &str) -> Result<String, String> {
        self.calls.push(("capture".into(), format!("{x},{y},{width},{height},{filename}")));
        Ok(format!("/tmp/{filename}"))
    }
    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("pixel".into(), format!("{x},{y}")));
        Ok(self.pixels.iter().find(|(at, _)| *at == (x, y)).map(|(_, colour)| colour.clone()).unwrap_or_else(|| "#000000".into()))
    }
    fn pixel_search(
        &mut self,
        _x1: i64,
        _y1: i64,
        _x2: i64,
        _y2: i64,
        colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        Ok(self.pixels.iter().find(|(_, value)| value == colour).map(|(at, _)| *at))
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
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    (report, desktop)
}

/// The same, with a timeout of its own.
fn run_with_timeout(source: &str, timeout: Duration) -> (Report, Fake) {
    let mut desktop = Fake::default();
    desktop.pixels.push(((10, 20), "#ff8800".into()));
    let statements = parse(source).expect("parses");
    let report = Runtime::new(&mut desktop, timeout).run(&statements);
    (report, desktop)
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
    let (report, desktop) = run("x := 5\nIf x > 3 {\n  Send \"big\"\n} Else {\n  Send \"small\"\n}");
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
    let (report, desktop) = run(
        "i := 0\nLoop 10 {\n  i := i + 1\n  If i = 2 {\n    Break\n  }\n}\nSend \"%i%\"",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["2"]);
}

#[test]
fn a_continue_skips_the_rest_of_the_body() {
    let (report, desktop) = run(
        "i := 0\nLoop 3 {\n  i := i + 1\n  If i < 3 {\n    Continue\n  }\n  Send \"%i%\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["3"]);
}

#[test]
fn a_run_carries_the_output() {
    let (report, _desktop) = run("Run \"echo hello\"");
    assert!(report.ok);
    assert!(report.output.iter().any(|line| line.contains("ran echo hello")));
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
    assert_eq!(report.variables.get("pixel"), Some(&Value::Text("#ff8800".into())));
}

#[test]
fn a_screenshot_captures_the_region() {
    let (report, desktop) = run("Screenshot 0, 0, 100, 50, \"shot.png\"");
    assert!(report.ok);
    assert!(desktop.calls.contains(&("capture".into(), "0,0,100,50,shot.png".into())));
}

#[test]
fn a_click_clicks_at_the_position() {
    let (report, desktop) = run("Click 40, 50");
    assert!(report.ok);
    assert!(desktop.calls.contains(&("click".into(), "40,50,left,1".into())));
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
    let (report, _desktop) = run("#Requires AutoHotkey v2.0.19\nMsgBox \"ok\"");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_requires_that_is_not_met_fails_the_script() {
    let (report, _desktop) = run("#Requires AutoHotkey v1\nMsgBox \"ok\"");
    assert!(!report.ok);
    assert!(report.error.as_deref().unwrap_or("").contains("requires"), "{:?}", report.error);
}

#[test]
fn a_directive_this_runtime_does_not_carry_fails_with_the_reason() {
    let (report, _desktop) = run("#UseHook\nMsgBox \"ok\"");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the hotkeys are not carried by this runtime."));
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
    let (report, _desktop) = run("Loop Parse \"a,b\", \",\" {\n  last := A_Index\n}\nSend \"%last%\"");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_loop_over_the_registry_fails_with_the_reason() {
    let (report, _desktop) = run("Loop Reg \"HKLM\\\\*\" {\n}");
    assert!(!report.ok);
    assert!(report.error.as_deref().unwrap_or("").contains("Windows"), "{:?}", report.error);
}

#[test]
fn a_string_is_read_where_it_is_written() {
    // The name is the name of the scope that is writing the string, so
    // a string a function returns still knows its own parameter.
    let (report, desktop) = run("Greet(name) {\n  return \"hello %name%\"\n}\nSend Greet(\"world\")");
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
    let (report, desktop) = run(
        "class Adder {\n  Call(a, b) {\n    return a + b\n  }\n}\n\
         add := Adder()\nSend add(2, 3)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["5"]);
}

#[test]
fn an_object_without_a_call_method_says_so() {
    let (report, _) = run(
        "class Plain {\n}\n\
         p := Plain()\nMsgBox p(2, 3)",
    );
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
    let (report, desktop) = run(
        "class Point {\n  x := 3\n}\np := Point()\np.x := 7\nSend \"%p.x%\"",
    );
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
    let (report, desktop) = run(
        "class Base {\n  Who() {\n    return \"base\"\n  }\n}\n\
         class Child extends Base {\n  Who() {\n    return \"child\"\n  }\n}\n\
         c := Child()\nSend c.Who()",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["child"]);
}

#[test]
fn a_method_of_the_class_it_extends_is_there() {
    let (report, desktop) = run(
        "class Base {\n  Who() {\n    return \"base\"\n  }\n}\n\
         class Child extends Base {\n}\nc := Child()\nSend c.Who()",
    );
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
    let (report, _desktop) = run("class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Missing a required parameter.")
    );
    assert_eq!(report.error_line, Some(7));
}

#[test]
fn a_method_called_with_too_many_arguments_fails_with_the_reason() {
    let (report, _desktop) = run("class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method(1, 2)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function.")
    );
    assert_eq!(report.error_line, Some(7));
}

#[test]
fn a_method_without_parameters_runs_when_none_are_given() {
    let (report, desktop) = run("class Thing {\n  Method() {\n    return \"the answer\"\n  }\n}\nt := Thing()\nSend t.Method()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["the answer"]);
}

#[test]
fn a_method_takes_the_default_of_a_parameter_that_is_not_given() {
    let (report, desktop) = run("class Thing {\n  Method(a := \"the default\") {\n    return a\n  }\n}\nt := Thing()\nSend t.Method()");
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
        Some("Too few parameters passed to function. Specifically: Add")
    );
    assert_eq!(report.error_line, Some(4));
}

#[test]
fn a_call_with_too_many_arguments_fails_with_the_reason() {
    let (report, _desktop) = run("Add(a) {\n  return a\n}\nAdd(1, 2)");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("Too many parameters passed to function. Specifically: Add")
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
    assert_eq!(report.error.as_deref(), Some("the loop ran more than 1000 times"));
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_script_runs_at_most_twenty_thousand_statements() {
    // A counted `Loop` has no guard of its own: the step count is what stops
    // it, which is why the message is about statements and not about turns.
    let (report, _desktop) = run("Loop 100000 {\n  Send \"x\"\n}");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the script ran more than 20000 statements"));
    assert!(report.error_line.is_some());
}

#[test]
fn a_script_that_outlives_its_timeout_is_stopped() {
    let (report, _desktop) = run_with_timeout("Sleep 200\nMsgBox \"late\"", Duration::from_millis(50));
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the script ran for more than 0.05s"));
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
        "x := 2\nIf x = 1 {\n  Send \"one\"\n} ElseIf x = 2 {\n  Send \"two\"\n} Else {\n  Send \"other\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["two"]);
}

#[test]
fn an_else_if_chain_takes_only_the_first_branch_that_holds() {
    let (report, desktop) = run(
        "x := 5\nIf x = 1 {\n  Send \"one\"\n} ElseIf x = 5 {\n  Send \"two\"\n} ElseIf x = 5 {\n  Send \"three\"\n} Else {\n  Send \"other\"\n}",
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
    let (report, desktop) = run(
        "x := 9\nSwitch x {\n  Case 1:\n    Send \"one\"\n  Default:\n    Send \"other\"\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["other"]);
}

#[test]
fn a_function_returns_a_value() {
    let (report, desktop) = run("Add(a, b) {\n  return a + b\n}\ntotal := Add(2, 3)\nSend \"%total%\"");
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
    let (report, desktop) = run("text := Later()\nSend \"%text%\"\nLater() {\n  return \"here\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["here"]);
}

#[test]
fn a_return_leaves_the_function_at_once() {
    let (report, desktop) = run(
        "text := First()\nSend \"%text%\"\nFirst() {\n  return \"one\"\n  Send \"never\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["one"]);
}

#[test]
fn a_variable_assigned_inside_a_function_is_local() {
    let (report, _desktop) = run("x := \"outside\"\nSet() {\n  x := \"inside\"\n}\nSet()\nMsgBox x");
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
    let (report, _desktop) = run(
        "Swap(&a, &b) {\n  tmp := a\n  a := b\n  b := tmp\n}\n\
         a := \"one\"\nb := \"two\"\nSwap(&a, &b)",
    );
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
        Some(
            "Parameter #1 of Show requires a variable reference, but received an Integer."
        )
    );
}

#[test]
fn a_parameter_written_by_reference_that_the_call_leaves_out_is_a_local_one() {
    // The default of v2 is a value of its own: the parameter is a local
    // one when the call does not give it a reference.
    let (report, _desktop) = run("Show(&x := \"default\") {\n  return x\n}\nout := Show()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("out"), Some(&Value::Text("default".into())));
}

#[test]
fn a_reference_is_not_a_value() {
    let (report, _desktop) = run("x := 1\ny := &x\nout := \"v=\" . y");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a String but got a VarRef."));
}

#[test]
fn a_plain_parameter_that_is_given_a_reference_holds_it() {
    // The reference is a value of its own, and the parameter that is
    // not written `&x` holds it.
    let (report, _desktop) = run("Show(x) {\n  out := \"in=\" . x\n}\ny := \"orig\"\nShow(&y)");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a String but got a VarRef."));
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
        Some("This built-in variable cannot have its reference taken. Specifically: A_ScreenWidth")
    );
}

#[test]
fn a_function_cannot_have_its_reference_taken() {
    let (report, _desktop) = run("r := &Abs");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This Func cannot have its reference taken. Specifically: Abs")
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
        desktop.calls.contains(&("working_directory".to_string(), String::new())),
        "{:?}",
        desktop.calls
    );
    assert!(
        desktop.calls.contains(&("list_files".to_string(), "/work|*.txt|".to_string())),
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
        Some(&Value::Text("two.txt|txt|/work|/work/two.txt|/work/two".into()))
    );
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
        desktop.calls.contains(&("list_files".to_string(), "/work|*.txt|R".to_string())),
        "{:?}",
        desktop.calls
    );
}

#[test]
fn a_loop_over_the_files_takes_no_more_than_two_parameters() {
    let (report, _desktop) = run("Loop Files \"*.txt\", \"R\", \"x\" {\n}\n");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("\"Loop Files\" accepts at most 2 parameters."));
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
    let (report, _desktop) = run(
        "MouseGetPos &x, &y\nfound := [10, 20]\nfx := found[1]",
    );
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
    assert_eq!(report.variables.get("letter"), Some(&Value::Text("e".into())));
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
    assert_eq!(desktop.calls, vec![("key".to_string(), "ctrl+a".to_string())]);
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
    assert_eq!(desktop.calls, vec![("key_down".to_string(), "a".to_string())]);
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
    assert_eq!(desktop.calls, vec![("click".to_string(), "10,20,left,2".to_string())]);
}

#[test]
fn a_click_can_name_the_button_and_the_count() {
    let (report, desktop) = run("Click 10, 20, right, 3");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("click".to_string(), "10,20,right,3".to_string())]);
}

#[test]
fn a_number_alone_is_the_number_of_clicks() {
    // The place is two numbers, so a number on its own is the number of
    // clicks: `Click 2` is a double click where the pointer is.
    let (report, desktop) = run("Click 2");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("click".to_string(), "0,0,left,2".to_string())]);
}

#[test]
fn the_components_of_a_click_may_be_in_one_string() {
    let (report, desktop) = run("Click \"100 200 Right\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("click".to_string(), "100,200,right,1".to_string())]);
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
    let (report, desktop) = run(
        "MouseMove 100, 200\nSend \"{LButton down}\"\nSend \"{LButton up}\"",
    );
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
    let (report, _desktop) =
        run("found := PixelSearch(&x, &y, 0, 0, 100, 100, \"#ff8800\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("found"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("x"), Some(&Value::Int(10)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(20)));
}

#[test]
fn a_pixel_search_that_finds_nothing_answers_zero() {
    let (report, _desktop) =
        run("found := PixelSearch(&x, &y, 0, 0, 100, 100, \"#ffffff\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("found"), Some(&Value::Int(0)));
    // The place of v2 is blank when the colour is not there.
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
    assert_eq!(report.variables.get("y"), Some(&Value::Text(String::new())));
}

#[test]
fn win_active_says_whether_the_window_is_active() {
    let mut desktop = Fake { active: Some("Steam".into()), ..Default::default() };
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

/// The mode of the matching of the titles is a setting of the script,
/// and the desktop is told it.
#[test]
fn the_mode_of_the_matching_of_the_titles_is_the_one_of_the_script() {
    let mut desktop = Fake { windows: vec!["Steam Client".into()], ..Default::default() };
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
    assert_eq!(report.variables.get("title"), Some(&Value::Text("Firefox".into())));
}

/// A desktop with no window in front has no title to give.
#[test]
fn win_get_title_without_a_window_in_front_is_empty() {
    let mut desktop = Fake::default();
    let statements = parse("title := WinGetTitle()").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("title"), Some(&Value::Text(String::new())));
}

#[test]
fn a_window_question_that_cannot_be_answered_fails_the_script() {
    let mut desktop = Fake { window_answer_unknown: true, ..Default::default() };
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
    assert_eq!(report.variables.get("f"), Some(&Value::Float(2.35)));
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
fn the_is_operator_wants_a_class() {
    let (report, _desktop) = run("class Alpha {\n}\nb := Alpha()\na := b is \"Alpha\"");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a Class but got a String."));
}

#[test]
fn a_class_has_a_prototype() {
    let (report, _desktop) = run("class Alpha {\n}\np := Alpha.Prototype\nt := Type(Alpha)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("t"), Some(&Value::Text("Class".into())));
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
    assert_eq!(report.variables.get("c"), Some(&Value::Text("heLLo".into())));
    assert_eq!(report.variables.get("d"), Some(&Value::Int(3)));
    assert_eq!(report.variables.get("e"), Some(&Value::Text("HI".into())));
    assert_eq!(report.variables.get("f"), Some(&Value::Text("2 of 3".into())));
    assert_eq!(report.variables.get("g"), Some(&Value::Text("2.35".into())));
}

#[test]
fn a_function_with_the_name_of_a_builtin_wins() {
    let (report, desktop) = run("StrUpper(text) {\n  return \"mine\"\n}\nresult := StrUpper(\"hi\")\nSend \"%result%\"");
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
    let (report, _desktop) = run(
        "class Box {\n}\nb := Box()\nb[\"a\"] := 1",
    );
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

#[test]
fn a_property_of_an_object_literal_needs_its_colon() {
    let error = parse("x := {a 1}").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"propertyname:\" in object literal.");
    assert_eq!(error.line, 1);
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
    let (report, _desktop) = run(
        "Outer() {\n    n := 10\n    f := (a) => a + n\n    return f(1)\n}\nMsgBox Outer()",
    );
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
    // Measured on the binary of v2.0.19 on 2026-10-31: the brace that is
    // not closed is named, and nothing else: the line of the script is
    // not named with it.
    let error = parse("f(a) {\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"");
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
    // brace that closes the switch before it reads the cases.
    let error = parse("Switch 1 {\n    x := 1\n").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"}\"");
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
    assert_eq!(error.message, "Unexpected \")\"\n     Specifically: x := 1 )");
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
    assert_eq!(report.error.as_deref(), Some("Expected a String but got an Object."));
}

#[test]
fn the_concatenation_of_an_array_is_refused() {
    let (report, _desktop) = run("x := [1]\nMsgBox \"a\" . x");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a String but got an Array."));
}

#[test]
fn the_concatenation_of_a_function_is_refused() {
    let (report, _desktop) = run("x := () => 1\nMsgBox \"a\" . x");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a String but got a Func."));
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
    assert_eq!(report.error.as_deref(), Some("Expected a Number but got an Object."));
}

#[test]
fn the_maths_refuses_an_array() {
    let (report, _desktop) = run("x := [1]\nMsgBox 1 + x");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a Number but got an Array."));
}

#[test]
fn the_maths_refuses_a_text_that_is_not_a_number() {
    let (report, _desktop) = run("x := \"a\"\nMsgBox 1 + x");
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Expected a Number but got a String."));
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
    assert_eq!(report.error.as_deref(), Some("Expected an Integer but got a Float."));
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
    let (report, desktop) = run("m := Map()\nm[\"a\"] := 1\nm[\"a\"] += 2\nv := m[\"a\"]\nSend \"%v%\"");
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
    let (report, desktop) = run("m := Map()\nm.Set(\"a\", 1)\nfor k, v in m {\n  Send \"%k%=%v%\"\n}");
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
    let (report, desktop) = run(
        "for v in [\"a\", \"b\", \"c\"] {\n  If v = \"b\" {\n    Break\n  }\n  Send v\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["a"]);
}

#[test]
fn a_continue_skips_the_rest_of_a_for() {
    let (report, desktop) = run(
        "for v in [\"a\", \"b\"] {\n  If v = \"a\" {\n    Continue\n  }\n  Send v\n}",
    );
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
    let (report, _desktop) = run(
        "class Bag {\n  __Enum(n) {\n    return [\"a\", \"b\"]\n  }\n}\nfor v in Bag() {\n}",
    );
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Value not enumerable."));
}

#[test]
fn an_enumerator_that_is_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is
    // not an object at all is a mismatch of the type.
    let (report, _desktop) = run(
        "class Bag {\n  __Enum(n) {\n    return 5\n  }\n}\nfor v in Bag() {\n}",
    );
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("Type mismatch."));
}

#[test]
fn a_for_gives_the_variables_back_inside_a_function() {
    let (report, desktop) = run("f() {\n  v := \"before\"\n  for v in [\"a\"] {\n  }\n  return v\n}\nSend f()");
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
    assert!(error.contains("the loop ran more than 1000 times"), "{error}");
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
    let (report, _) = run(
        "Helper()\n\nHelper() {\n    for answer in [\"a\"] {\n    }\n    MsgBox answer\n}",
    );
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
        Some("Unhandled exception. Specifically: the error of the script")
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
        Some("Unhandled exception. Specifically: the error of the script")
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
    let (report, _) = run(
        "class Thing {\n  Method(a) {\n    return a\n  }\n}\nt := Thing()\nt.Method",
    );
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
    let (report, _) = run(
        "try {\n    throw Error(\"the message\")\n} catch as e {\n    MsgBox e.Message\n}",
    );
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
    assert_eq!(report.output, vec!["the derived and the middle and the base"]);
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
        "number := 5\nfloat := 5.5\ntext := \"text\"\nlist := [1]\nMsgBox number.__Class \" \" float.__Class \" \" text.__Class \" \" list.__Class",
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
    let (report, _desktop) = run(
        "class Thing {\n}\nMsgBox Thing.Prototype.__Class \" \" Map.Prototype.__Class",
    );
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
    assert_eq!(error.message, "Invalid assignment. Specifically: := 1");
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
    assert_eq!(report.error.as_deref(), Some("Expected a Number but got a String."));
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
    assert_eq!(error.message, "\"If\" requires at least 1 parameter.\n     Specifically: if");
}

#[test]
fn the_class_needs_its_braces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the class names the
    // line of it.
    let error = parse("class Thing\n    x := 1").expect_err("does not parse");
    assert_eq!(error.message, "Missing \"{\"\n     Specifically: class Thing");
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
    let (report, _desktop) = run("m := Map()\nMsgBox HasBase(m, Map.Prototype) \" \" HasBase(m, Object.Prototype) \" \" HasBase(m, Array.Prototype)");
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
    let (report, _desktop) = run("o := {}\nMsgBox HasMethod(o, \"OwnProps\") \" \" HasMethod(o, \"Nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 0"]);
}

#[test]
fn the_question_with_no_name_asks_whether_the_value_is_a_class() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a class has the
    // methods of the base object, and an instance has the methods of its
    // class.
    let (report, _desktop) = run("m := Map()\nMsgBox HasMethod(Map) \" \" HasMethod(m) \" \" HasMethod({})");
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
        "This Class cannot be used as an output variable. Specifically: Foo"
    );
}

#[test]
fn the_name_of_a_class_is_not_written_by_a_compound_assignment() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the compound
    // assignment says that the class cannot be assigned a value.
    let error = parse("class Foo {\n}\nfoo .= \"x\"").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Class cannot be assigned a value. Specifically: Foo"
    );
}

#[test]
fn the_name_of_a_class_of_the_runtime_is_not_the_name_of_a_variable() {
    let error = parse("Map := 5").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This Class cannot be used as an output variable. Specifically: Map"
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
        "This class declaration conflicts with an existing Func. Specifically: f"
    );
}

#[test]
fn a_function_may_not_be_named_like_a_class() {
    let error = parse("class f {\n}\nf() {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This function declaration conflicts with an existing Class. Specifically: f"
    );
}

#[test]
fn a_name_may_be_declared_only_once() {
    let error = parse("class f {\n}\nclass f {\n}").expect_err("does not parse");
    assert_eq!(
        error.message,
        "This class declaration conflicts with an existing Class. Specifically: f"
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
    let (report, _desktop) = run("class Foo {\n}\nf() {\n    local foo\n    foo := Foo()\n    return foo\n}\nf()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
}

#[test]
fn every_object_has_the_property_of_the_class_of_it() {
    let (report, _desktop) = run(
        "m := Map()\nMsgBox m.HasProp(\"__Class\") \" \" m.HasOwnProp(\"__Class\")",
    );
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
