//! The language and the runtime, against a fake desktop.
//!
//! No SSH, no box, no ydotool: the desktop is a fake that records the
//! calls, so these tests are about what the language means, not about what
//! the desktop does. The desktop part is verified against the real box
//! separately.
//!
//! These are a port of the Python tests in `yoko-ahk-mcp`, and the two
//! implementations have to agree.

use ahk::runtime::{Desktop, Report, Runtime, Value, WindowAnswer};
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

impl Desktop for Fake {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        self.calls.push(("type".into(), text.into()));
        Ok(format!("typed {text:?}"))
    }

    fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
        self.calls.push(("key".into(), name.into()));
        Ok(format!("pressed {name}"))
    }

    fn click(&mut self, x: i64, y: i64, button: &str) -> Result<String, String> {
        self.calls.push(("click".into(), format!("{x},{y},{button}")));
        Ok(format!("clicked ({x}, {y})"))
    }

    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        self.calls.push(("run".into(), command.into()));
        Ok((0, format!("ran {command}\n"), String::new()))
    }

    fn capture(&mut self, x: i64, y: i64, width: i64, height: i64, filename: &str) -> Result<String, String> {
        self.calls.push(("capture".into(), format!("{x},{y},{width},{height},{filename}")));
        Ok(format!("/tmp/{filename}"))
    }

    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("pixel".into(), format!("{x},{y}")));
        Ok(self.pixels.iter().find(|(at, _)| *at == (x, y)).map(|(_, colour)| colour.clone()).unwrap_or_else(|| "#000000".into()))
    }

    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.pointer = (x, y);
        self.calls.push(("move".into(), format!("{x},{y}")));
        Ok(format!("moved to ({x}, {y})"))
    }

    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        Ok(self.pointer)
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

    fn window_active(&mut self, title: &str) -> Result<WindowAnswer, String> {
        if self.window_answer_unknown {
            return Ok(WindowAnswer::Unknown("this desktop cannot tell".into()));
        }
        Ok(match &self.active {
            Some(active) if active.contains(title) => WindowAnswer::Active,
            _ => WindowAnswer::NotActive,
        })
    }

    fn window_exists(&mut self, title: &str) -> Result<WindowAnswer, String> {
        if self.window_answer_unknown {
            return Ok(WindowAnswer::Unknown("this desktop cannot list".into()));
        }
        Ok(if self.windows.iter().any(|window| window.contains(title)) {
            WindowAnswer::Active
        } else {
            WindowAnswer::NotActive
        })
    }

    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        Ok(self.active.clone())
    }
}

fn run(source: &str) -> (Report, Fake) {
    let mut desktop = Fake::default();
    desktop.pixels.push(((10, 20), "#ff8800".into()));
    let statements = parse(source).expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
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
fn a_pixel_get_reads_the_colour() {
    let (report, _desktop) = run("PixelGet 10, 20");
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
    assert!(desktop.calls.contains(&("click".into(), "40,50,left".into())));
}

#[test]
fn a_msg_is_collected_as_output() {
    let (report, _desktop) = run("Msg \"the answer\"");
    assert_eq!(report.output, vec!["the answer"]);
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
        "x := 2\nSwitch x {\n  Case 1 {\n    Send \"one\"\n  }\n  Case 2 {\n    Send \"two\"\n  }\n  Default {\n    Send \"other\"\n  }\n}",
    );
    assert!(report.ok);
    assert_eq!(desktop.types(), vec!["two"]);
}

#[test]
fn a_switch_without_a_match_runs_the_default() {
    let (report, desktop) = run(
        "x := 9\nSwitch x {\n  Case 1 {\n    Send \"one\"\n  }\n  Default {\n    Send \"other\"\n  }\n}",
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
    let (report, _desktop) = run("Greet(name) {\n  Msg \"hello %name%\"\n}\nGreet(\"world\")");
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
    let (report, _desktop) = run("x := \"outside\"\nSet() {\n  x := \"inside\"\n}\nSet()\nMsg x");
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

// -- the focus check ------------------------------------------------------------

#[test]
fn a_list_can_be_picked_by_its_position() {
    let (report, _desktop) = run(
        "pos := MouseGetPos()\nx := pos[1]\ny := pos[2]\nfound := PixelSearch(0, 0, 10, 10, \"#ff8800\")\nfx := found[1]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Int(0)));
    assert_eq!(report.variables.get("y"), Some(&Value::Int(0)));
    assert_eq!(report.variables.get("fx"), Some(&Value::Int(10)));
}

#[test]
fn a_text_can_be_picked_by_its_position() {
    let (report, _desktop) = run("letter := \"hello\"[2]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("letter"), Some(&Value::Text("e".into())));
}

#[test]
fn a_position_that_is_not_there_is_an_error() {
    let (report, _desktop) = run("letter := \"hello\"[99]");
    assert!(!report.ok);
}

#[test]
fn mouse_get_pos_says_where_the_pointer_is() {
    let (report, _desktop) = run("MouseMove 120, 340\npos := MouseGetPos()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("mouse_x"), Some(&Value::Int(120)));
    assert_eq!(report.variables.get("mouse_y"), Some(&Value::Int(340)));
    assert_eq!(
        report.variables.get("pos"),
        Some(&Value::List(vec![Value::Int(120), Value::Int(340)]))
    );
}

#[test]
fn a_pixel_search_finds_the_colour() {
    let (report, _desktop) = run("found := PixelSearch(0, 0, 100, 100, \"#ff8800\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("found"),
        Some(&Value::List(vec![Value::Int(10), Value::Int(20)]))
    );
}

#[test]
fn a_pixel_search_that_finds_nothing_is_an_empty_list() {
    let (report, _desktop) = run("found := PixelSearch(0, 0, 100, 100, \"#ffffff\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("found"), Some(&Value::List(Vec::new())));
}

#[test]
fn win_active_says_whether_the_window_is_active() {
    let mut desktop = Fake { active: Some("Steam".into()), ..Default::default() };
    let statements = parse("active := WinActive(\"Steam\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("active"), Some(&Value::Bool(true)));
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
    assert_eq!(report.variables.get("b"), Some(&Value::Int(4)));
    assert_eq!(report.variables.get("c"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("d"), Some(&Value::Int(2)));
    assert_eq!(report.variables.get("e"), Some(&Value::Int(3)));
    assert_eq!(report.variables.get("f"), Some(&Value::Float(2.35)));
    assert_eq!(report.variables.get("g"), Some(&Value::Int(1)));
    assert_eq!(report.variables.get("h"), Some(&Value::Int(3)));
}

#[test]
fn the_text_has_its_functions() {
    let (report, _desktop) = run(
        "a := StrLen(\"hello\")\nb := SubStr(\"hello\", 2)\nc := StrReplace(\"hello\", \"l\", \"L\")\nd := InStr(\"hello\", \"ll\")\ne := Upper(\"hi\")\nf := Format(\"{0} of {1}\", 2, 3)\ng := Format(\"{0:.2}\", 2.345)",
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
    let (report, desktop) = run("Upper(text) {\n  return \"mine\"\n}\nresult := Upper(\"hi\")\nSend \"%result%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.types(), vec!["mine"]);
}
