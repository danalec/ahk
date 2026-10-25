//! The language and the runtime, against a fake desktop.
//!
//! No SSH, no box, no ydotool: the desktop is a fake that records the
//! calls, so these tests are about what the language means, not about what
//! the desktop does. The desktop part is verified against the real box
//! separately.
//!
//! These are a port of the Python tests in `yoko-ahk-mcp`, and the two
//! implementations have to agree.

use ahk::runtime::{Desktop, Report, Runtime, Value};
use ahk::{Node, parse};
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    calls: Vec<(String, String)>,
    pixels: Vec<((i64, i64), String)>,
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
