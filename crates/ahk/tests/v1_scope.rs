//! The scope of the variables of the v1 inside a function.
//!
//! The v1 has no `Assume-Local`: a name that a function writes is the name
//! of the function, and not the name of the script -- unless a `global`
//! declaration outside of any function declared it. The CHM of the
//! v1.1.37.02 (`Functions.htm`, "Global Variables") calls such a name a
//! "super-global" and says that it "takes effect for all functions by
//! default (excluding force-local functions)".
//!
//! Every case of this file was measured on the binary of AutoHotkey
//! v1.1.37.02, and the probe of each one is written in the
//! comment of it. The probes went to
//! the probe of the measurement and the answer of them to a
//! file next to it.
//!
//! The cases run against a fake desktop: the scope of a variable is the
//! business of the runtime, and not of the machine.

use ahk::desktop::{
    Clipboard, Com, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
    Pointer, Processes, Screen, Sound, Timers, Windows,
};
use ahk::dialect;
use ahk::runtime::{Report, Runtime};
use std::time::Duration;

/// A desktop that does nothing: the cases of the scope do not need the
/// machine, and the methods that a capability does not have answer that
/// they do not have it.
#[derive(Debug, Default)]
struct Fake;

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}
impl Clipboard for Fake {}
impl Timers for Fake {}
impl Hotkeys for Fake {}
impl Native for Fake {}
impl Com for Fake {}
impl Files for Fake {}
impl Windows for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        Ok(format!("typed {text}"))
    }
    fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(format!("pressed {name}"))
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok("clicked".into())
    }
    fn mouse_down(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        Ok(format!("{button} is down"))
    }
    fn mouse_up(
        &mut self,
        _x: Option<i64>,
        _y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        Ok(format!("{button} is up"))
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
        Ok(String::new())
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, format!("ran {command}"), String::new()))
    }
}

/// Run a script of the v1 and answer with the report.
fn run(script: &str) -> Report {
    let statements = ahk::parse_with(script, dialect::v1()).expect("the script of the v1 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::v1())
        .run(&statements)
}

/// Run a script of the v2 and answer with the report.
///
/// The v2 has no super-global: the name of a function is the name of the
/// function, and the tree keeps it that way.
fn run_the_v2(script: &str) -> Report {
    let statements = ahk::parse_with(script, dialect::v2()).expect("the script of the v2 parses");
    let mut desktop = Fake;
    Runtime::new(&mut desktop, Duration::from_secs(10))
        .with_dialect(dialect::v2())
        .run(&statements)
}

// -- the super-global of the script -------------------------------------------------

#[test]
fn the_name_that_a_global_declaration_declares_is_the_global_one() {
    // The probe:
    //
    // ```ahk
    // Global gvA := 1
    // gvB := 2
    // f1() {
    //     gvA := 42
    //     gvB := 43
    // }
    // f1()
    // ```
    //
    // The answer of the v1.1.37.02: "A=42 B=2". The name
    // that the `Global` declaration of the top level declared is the global
    // one, and the name that the top level only wrote is the local one of
    // the function: a global that exists is not enough.
    let report = run(
        "Global gvA := 1\n\
         gvB := 2\n\
         f1() {\n\
         \x20   gvA := 42\n\
         \x20   gvB := 43\n\
         }\n\
         f1()\n\
         text := \"A=\" gvA \" B=\" gvB\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["A=42 B=2"]);
}

#[test]
fn the_name_that_no_global_has_is_the_local_one_of_the_function() {
    // The probe:
    //
    // ```ahk
    // f1() {
    //     gnew := 7
    //     return "in=" gnew
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=7 out=". The
    // outside of the function sees nothing.
    let report = run(
        "f1() {\n\
         \x20   gnew := 7\n\
         \x20   return \"in=\" gnew\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gnew\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=7 out="]);
}

#[test]
fn the_super_global_is_the_global_one_in_a_function_that_another_one_calls() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // outer() {
    //     inner()
    // }
    // inner() {
    //     gv := 42
    // }
    // outer()
    // ```
    //
    // The answer of the v1.1.37.02: "gv=42". The name of the
    // function that writes it is the global one, and the call of it does
    // not change that.
    let report = run(
        "Global gv := 1\n\
         outer() {\n\
         \x20   inner()\n\
         \x20   return\n\
         }\n\
         inner() {\n\
         \x20   gv := 42\n\
         \x20   return\n\
         }\n\
         outer()\n\
         text := \"gv=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["gv=42"]);
}

#[test]
fn the_function_that_another_function_made_a_global_of_is_not_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // f2() {
    //     Global gvC
    //     gvC := 5
    // }
    // f1() {
    //     gvC := 42
    // }
    // f2()
    // f1()
    // ```
    //
    // The answer of the v1.1.37.02: "gvC=5". The `Global` of
    // the function `f2` makes the name global for that function and not
    // for the whole script: the `f1` writes a local of its own.
    let report = run(
        "f2() {\n\
         \x20   Global gvC\n\
         \x20   gvC := 5\n\
         \x20   return\n\
         }\n\
         f1() {\n\
         \x20   gvC := 42\n\
         \x20   return\n\
         }\n\
         f2()\n\
         f1()\n\
         text := \"gvC=\" gvC\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["gvC=5"]);
}

// -- the declarations of the function ------------------------------------------------

#[test]
fn the_global_declaration_of_the_function_makes_the_name_global() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1() {
    //     Global gv
    //     gv := 42
    //     return "in=" gv
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=42 out=42".
    let report = run(
        "Global gv := 1\n\
         f1() {\n\
         \x20   Global gv\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" gv\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=42 out=42"]);
}

#[test]
fn the_global_declaration_of_the_function_makes_the_name_global_when_no_global_exists() {
    // The probe:
    //
    // ```ahk
    // f1() {
    //     Global gnew2
    //     gnew2 := 5
    //     return "in=" gnew2
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=5 out=5". The
    // global is made, and the outside of the function sees it.
    let report = run(
        "f1() {\n\
         \x20   Global gnew2\n\
         \x20   gnew2 := 5\n\
         \x20   return \"in=\" gnew2\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gnew2\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=5 out=5"]);
}

#[test]
fn the_local_declaration_of_the_function_wins_over_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1() {
    //     Local gv
    //     gv := 42
    //     return "in=" gv
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=42 out=1". The
    // local is the local one, and the global keeps its value.
    let report = run(
        "Global gv := 1\n\
         f1() {\n\
         \x20   Local gv\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" gv\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=42 out=1"]);
}

#[test]
fn the_name_that_the_function_reads_before_it_writes_it_is_the_global_one() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1() {
    //     s := "read=" gv
    //     gv := 42
    //     return s
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=read=1 out=42". The
    // read sees the value of the global, and the write goes to it.
    let report = run(
        "Global gv := 1\n\
         f1() {\n\
         \x20   s := \"read=\" gv\n\
         \x20   gv := 42\n\
         \x20   return s\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=read=1 out=42"]);
}

#[test]
fn the_static_of_the_function_is_the_static_of_the_function() {
    // The probe:
    //
    // ```ahk
    // Global sv := 1
    // f1() {
    //     static sv := 5
    //     sv += 1
    //     return "in=" sv
    // }
    // r1 := f1()
    // r2 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=6 r2=in=7 out=1".
    // The static keeps its value between the calls, and the global of the
    // script keeps its own.
    let report = run(
        "Global sv := 1\n\
         f1() {\n\
         \x20   static sv := 5\n\
         \x20   sv += 1\n\
         \x20   return \"in=\" sv\n\
         }\n\
         r1 := f1()\n\
         r2 := f1()\n\
         text := \"r1=\" r1 \" r2=\" r2 \" out=\" sv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=6 r2=in=7 out=1"]);
}

// -- the modes of the function --------------------------------------------------------

#[test]
fn the_force_local_mode_does_not_reach_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1() {
    //     local
    //     gv := 42
    //     return "in=" gv
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=42 out=1". The
    // bare `local` of the first line of the body is the force-local mode,
    // and the super-global of the script is not reachable in it.
    let report = run(
        "Global gv := 1\n\
         f1() {\n\
         \x20   local\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" gv\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1 \" out=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=42 out=1"]);
}

#[test]
fn the_parameter_wins_over_the_global_of_the_script() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1(gv) {
    //     gv := 42
    //     return "in=" gv
    // }
    // r1 := f1(5)
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=42 out=1". The
    // parameter is the variable of the function, and the global of the
    // script keeps its value.
    let report = run(
        "Global gv := 1\n\
         f1(gv) {\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" gv\n\
         }\n\
         r1 := f1(5)\n\
         text := \"r1=\" r1 \" out=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=42 out=1"]);
}

#[test]
fn the_parameter_is_local_in_the_assume_global_mode() {
    // The probe:
    //
    // ```ahk
    // f1(a) {
    //     global
    //     a := 42
    //     b := 43
    //     return "in=" a " " b
    // }
    // r1 := f1(5)
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=42 43 a= b=43".
    // The mode assumes the scope of the script for every name but the
    // parameters, and the parameter is the variable of the function.
    let report = run(
        "f1(a) {\n\
         \x20   global\n\
         \x20   a := 42\n\
         \x20   b := 43\n\
         \x20   return \"in=\" a \" \" b\n\
         }\n\
         r1 := f1(5)\n\
         text := \"r1=\" r1 \" a=\" a \" b=\" b\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=42 43 a= b=43"]);
}

#[test]
fn the_parameter_is_local_in_the_force_local_mode() {
    // The probe:
    //
    // ```ahk
    // Global gv := 1
    // f1(a) {
    //     local
    //     gv := 42
    //     return "in=" a " " gv
    // }
    // r1 := f1(5)
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=5 42 a= gv=1".
    let report = run(
        "Global gv := 1\n\
         f1(a) {\n\
         \x20   local\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" a \" \" gv\n\
         }\n\
         r1 := f1(5)\n\
         text := \"r1=\" r1 \" a=\" a \" gv=\" gv\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in=5 42 a= gv=1"]);
}

// -- the names that the script builds ------------------------------------------------

#[test]
fn the_name_that_the_script_builds_is_the_name_of_a_variable_that_is_not_there() {
    // The probe:
    //
    // ```ahk
    // Global pPen4 := "global"
    // f1() {
    //     s := "in=" pPen%4%
    //     return s
    // }
    // r1 := f1()
    // ```
    //
    // The answer of the v1.1.37.02: "r1=in=". The name that
    // the script builds is not the name "pPen4": the text that is between
    // the percent signs is the name of a variable and not an expression, and
    // the "4" is not the name of a variable, so the name that the pieces
    // make is the "pPen" -- and there is no variable of that name.
    //
    // The probe of the measurement is
    // the probe of the measurement,
    // and the answer of it went to the file next to it.
    //
    // The write of such a name is the local of the function as well: the
    // `Global d1 := 2` of the top level of the script and the `d%1% := 43`
    // of a function leave the global at 2. The tree does not carry that
    // rule yet, and the case of the write is left to the work that will.
    let report = run(
        "Global pPen4 := \"global\"\n\
         f1() {\n\
         \x20   s := \"in=\" pPen%4%\n\
         \x20   return s\n\
         }\n\
         r1 := f1()\n\
         text := \"r1=\" r1\n\
         FileAppend, %text%, *\n",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["r1=in="]);
}

// -- the v2 keeps what it has ---------------------------------------------------------

#[test]
fn the_v2_keeps_the_name_of_the_function_local() {
    // The v2 has no `global` declaration outside of a function, and the
    // name of a function is the name of the function: the tree keeps the
    // rule of the v2, and the change of the v1 does not touch it.
    let report = run_the_v2(
        "gv := 1\n\
         f1() {\n\
         \x20   gv := 42\n\
         \x20   return \"in=\" gv\n\
         }\n\
         r1 := f1()\n",
    );
    assert!(report.ok, "{:?}", report.error);
    // The global of the script keeps the value it had.
    assert_eq!(
        report.variables.get("gv").map(|value| value.to_text()),
        Some("1".to_string())
    );
}
