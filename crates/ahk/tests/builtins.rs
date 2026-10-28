//! The built-ins, against a fake desktop.
//!
//! The cases are the language ones: what a built-in answers, and what it
//! says when it is given the wrong thing. The wording of the failures is
//! the wording of AutoHotkey v2, because that is what a script sees.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Pointer, Processes, Screen, Timers, Windows,
};
use ahk::runtime::{Report, Runtime, Value};
use ahk::parse;
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    calls: Vec<(String, String)>,
}

impl Desktop for Fake {}

impl Windows for Fake {}

impl Files for Fake {
    fn file_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(path.ends_with(".txt"))
    }
    fn dir_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(path == "/tmp")
    }
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Ok(vec!["one".to_string(), "two".to_string()])
    }
    fn append_file(&mut self, _path: &str, text: &str) -> Result<String, String> {
        Ok(text.to_string())
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
        Ok(format!("pressed {spelled}"))
    }
}

impl Pointer for Fake {
    fn click(&mut self, x: i64, y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(format!("clicked ({x}, {y})"))
    }
}

impl Screen for Fake {
    fn capture(&mut self, _x: i64, _y: i64, _width: i64, _height: i64, filename: &str) -> Result<String, String> {
        Ok(format!("/tmp/{filename}"))
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, format!("ran {command}\n"), String::new()))
    }
    fn env_get(&mut self, name: &str) -> Result<String, String> {
        Ok(format!("the value of {name}"))
    }
    fn env_set(&mut self, _name: &str, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }
}

/// Run a script and answer with its report.
fn run(source: &str) -> Report {
    let mut desktop = Fake::default();
    let statements = parse(source).expect("parses");
    Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements)
}

/// The value of a variable, as the script left it.
fn value_of(report: &Report, name: &str) -> Value {
    report.variables.get(name).cloned().unwrap_or_else(|| panic!("{name} has no value"))
}

/// The value of a variable, as a whole number.
fn int_of(report: &Report, name: &str) -> i64 {
    match value_of(report, name) {
        Value::Int(number) => number,
        other => panic!("{name} is {other:?} and not a whole number"),
    }
}

/// The value of a variable, as a float.
fn float_of(report: &Report, name: &str) -> f64 {
    match value_of(report, name) {
        Value::Float(number) => number,
        other => panic!("{name} is {other:?} and not a float"),
    }
}

/// The message of the failure, when the script failed.
fn failure_of(report: &Report) -> String {
    assert!(!report.ok, "the script was expected to fail");
    report.error.clone().expect("the failure has a message")
}

#[test]
fn a_name_of_windows_fails_with_the_reason() {
    let report = run("DllCall(\"user32\")");
    assert_eq!(failure_of(&report), "this is a name of Windows, and this desktop is not Windows.");
}

#[test]
fn a_hotkey_fails_with_the_reason() {
    let report = run("Hotstring(\"a\", \"b\")");
    assert_eq!(failure_of(&report), "this runtime has no hotkeys yet.");
}

#[test]
fn a_name_of_the_script_that_the_runtime_cannot_answer_fails_with_the_reason() {
    let report = run("DateAdd(\"20260101\", 1, \"days\")");
    assert_eq!(
        failure_of(&report),
        "the clock of this runtime counts from an arbitrary origin, and it does not know the date."
    );
}

#[test]
fn the_environment_is_asked_of_the_desktop() {
    let report = run("a := EnvGet(\"HOME\")\nb := EnvSet(\"HOME\", \"there\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("the value of HOME".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("there".into()));
}

#[test]
fn the_files_are_asked_of_the_desktop() {
    let report = run(concat!(
        "a := FileExist(\"a.txt\")",
        "\nb := FileExist(\"a.md\")",
        "\nc := DirExist(\"/tmp\")",
        "\nd := DirExist(\"/nowhere\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 0);
    assert_eq!(int_of(&report, "c"), 1);
    assert_eq!(int_of(&report, "d"), 0);
}

#[test]
fn the_lines_of_a_file_are_asked_of_the_desktop() {
    let report = run("lines := FileRead(\"a.txt\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "lines"),
        Value::List(vec![Value::Text("one".into()), Value::Text("two".into())])
    );
}

#[test]
fn a_text_is_cut_at_the_delimiters() {
    let report = run("pieces := StrSplit(\"a,b,c\", \",\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::List(vec![
            Value::Text("a".into()),
            Value::Text("b".into()),
            Value::Text("c".into()),
        ])
    );
}

#[test]
fn a_path_is_cut_into_its_pieces() {
    // v2 writes the pieces into the variables of the caller, and it
    // answers with nothing.
    let report = run(concat!(
        "SplitPath \"C:\\Users\\danalec\\file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("file.txt|C:\\Users\\danalec|txt|file|C:".into())
    );
}

#[test]
fn a_path_with_a_drive_and_no_backslash_keeps_the_drive_out_of_the_name() {
    // Measured on the binary of v2.0.19: the drive is cut off the path
    // before the name is, and the slash is not a separator of a file.
    let report = run(concat!(
        "SplitPath \"C:/Users/danalec/file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("/Users/danalec/file.txt|C:|txt|/Users/danalec/file|C:".into())
    );
}

#[test]
fn a_path_of_a_url_is_cut_at_the_slash() {
    let report = run(concat!(
        "SplitPath \"https://domain.com/images/pic.png\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("pic.png|https://domain.com/images|png|pic|https://domain.com".into())
    );
}

#[test]
fn a_path_that_wants_an_output_variable_says_so() {
    let report = run("SplitPath \"C:\\a.txt\", 5");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of SplitPath requires a variable reference, but received an Integer. Specifically: 5"
    );
}

#[test]
fn an_object_can_be_asked_about() {
    let report = run(concat!(
        "class Box {\n  x := 1\n}\n",
        "box := Box()",
        "\na := HasProp(box, \"x\")",
        "\nb := HasProp(box, \"y\")",
        "\nc := HasBase(box, \"Box\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 0);
    assert_eq!(int_of(&report, "c"), 1);
}

#[test]
fn is_set_ref_says_whether_the_variable_has_a_value() {
    let report = run(concat!(
        "x := 1",
        "\nempty := \"\"",
        "\na := IsSetRef(&x)",
        "\nb := IsSetRef(&empty)",
        "\nc := IsSetRef(&nothere)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 1);
    // A variable that was never given a value is not set.
    assert_eq!(int_of(&report, "c"), 0);
}

#[test]
fn is_set_ref_wants_a_reference() {
    let report = run("a := IsSetRef(1)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of IsSetRef requires a variable reference, but received an Integer. Specifically: 1"
    );
}

#[test]
fn the_mode_of_the_keys_is_the_one_of_the_box() {
    let report = run("SendMode \"Input\"");
    assert!(report.ok, "{:?}", report.error);
    assert!(report.notes.iter().any(|note| note.contains("keyboard of the box")), "{:?}", report.notes);
    let report = run("SendMode \"Event\"");
    assert_eq!(
        failure_of(&report),
        "the keys of this runtime go through the keyboard of the box, and that is the only mode there is."
    );
}

#[test]
fn a_timer_runs_between_the_statements() {
    let report = run(concat!(
        "SetTimer(\"Tick\", 1)",
        "\nTick() {\n  Send \"tick\"\n}",
        "\nSleep 30",
    ));
    assert!(report.ok, "{:?}", report.error);
    let typed: Vec<&String> = report
        .steps
        .iter()
        .filter(|step| step.detail.contains("tick"))
        .map(|step| &step.detail)
        .collect();
    assert!(!typed.is_empty(), "the timer did not run: {:?}", report.steps);
}

#[test]
fn the_trigonometry_answers_a_float() {
    let report = run(concat!(
        "a := Sin(0)",
        "\nb := Cos(0)",
        "\nc := Tan(0)",
        "\nd := ASin(0)",
        "\ne := ACos(1)",
        "\nf := ATan(0)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Float(0.0));
    assert_eq!(value_of(&report, "b"), Value::Float(1.0));
    assert_eq!(value_of(&report, "c"), Value::Float(0.0));
    assert_eq!(value_of(&report, "d"), Value::Float(0.0));
    assert_eq!(value_of(&report, "e"), Value::Float(0.0));
    assert_eq!(value_of(&report, "f"), Value::Float(0.0));
}

#[test]
fn the_trigonometry_wants_a_number() {
    let report = run("a := Sin(\"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Sin requires a Number, but received a String."
    );
}

#[test]
fn the_inverse_trigonometry_fails_outside_the_unit_range() {
    let report = run("a := ACos(2)");
    assert_eq!(failure_of(&report), "Parameter #1 of ACos is invalid.");
}

#[test]
fn the_exponential_and_the_logarithms_answer_a_float() {
    let report = run("a := Exp(0)\nb := Ln(1)\nc := Log(10)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Float(1.0));
    assert_eq!(value_of(&report, "b"), Value::Float(0.0));
    let logarithm = float_of(&report, "c");
    assert!((logarithm - 1.0).abs() < 1e-12, "Log(10) is {logarithm}");
}

#[test]
fn the_logarithm_of_a_negative_number_fails() {
    let report = run("a := Ln(-1)");
    assert_eq!(failure_of(&report), "Parameter #1 of Ln is invalid.");
}

#[test]
fn a_number_that_is_not_a_number_fails_the_way_v2_does() {
    let report = run("a := Abs(\"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received a String."
    );
}

#[test]
fn an_empty_text_is_named_in_the_message() {
    let report = run("a := Abs(\"\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received an empty string."
    );
}

#[test]
fn every_maths_built_in_names_the_type_it_wanted() {
    for (source, name) in [
        ("a := Sqrt(\"x\")", "Sqrt"),
        ("a := Floor(\"x\")", "Floor"),
        ("a := Ceil(\"x\")", "Ceil"),
        ("a := Round(\"x\")", "Round"),
        ("a := Min(\"x\", 2)", "Min"),
        ("a := Max(\"x\", 2)", "Max"),
        ("a := Sin(\"x\")", "Sin"),
        ("a := Cos(\"x\")", "Cos"),
        ("a := Tan(\"x\")", "Tan"),
        ("a := ASin(\"x\")", "ASin"),
        ("a := ACos(\"x\")", "ACos"),
        ("a := ATan(\"x\")", "ATan"),
        ("a := Exp(\"x\")", "Exp"),
        ("a := Ln(\"x\")", "Ln"),
        ("a := Log(\"x\")", "Log"),
    ] {
        let report = run(source);
        assert_eq!(
            failure_of(&report),
            format!("Parameter #1 of {name} requires a Number, but received a String."),
            "{source}"
        );
    }
}

#[test]
fn the_digits_of_a_round_are_a_number_too() {
    let report = run("a := Round(5, \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Round requires a Number, but received a String."
    );
}

#[test]
fn the_modulus_says_that_the_parameters_are_invalid() {
    let report = run("a := Mod(\"x\", 2)");
    assert_eq!(failure_of(&report), "Invalid parameter(s).");
}

#[test]
fn a_text_that_holds_a_number_is_a_number() {
    let report = run("a := Abs(\"3\")\nb := Abs(\"3.5\")\nc := Mod(\"5\", 2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Float(3.5));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
}

#[test]
fn a_reference_is_not_a_number() {
    let report = run("x := 1\na := Abs(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received a VarRef."
    );
}

#[test]
fn a_reference_is_not_a_text_either() {
    let report = run("x := 1\na := StrLen(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrLen requires a String, but received a VarRef."
    );
}

#[test]
fn a_reference_is_not_an_object_either() {
    let report = run("x := 1\na := ObjOwnProps(&x)");
    assert_eq!(failure_of(&report), "Expected an Object but got a VarRef.");
}

#[test]
fn the_base_of_an_object_is_the_prototype_of_its_class() {
    let report = run(
        "class Alpha {}\nclass Beta extends Alpha {}\nb := Beta()\nx := ObjGetBase(b)",
    );
    assert!(report.ok, "{:?}", report.error);
    match value_of(&report, "x") {
        Value::Object(object) => assert_eq!(object.borrow().class, "prototype"),
        other => panic!("the base is {other:?}"),
    }
}

#[test]
fn a_script_may_set_the_base_of_an_object() {
    let report = run(
        "class Alpha {}\nclass Beta extends Alpha {}\nb := Beta()\n\
         old := ObjSetBase(b, Alpha())\nnow := ObjGetBase(b)",
    );
    assert!(report.ok, "{:?}", report.error);
    // An object of a script has no base of its own until it is given one.
    assert_eq!(value_of(&report, "old"), Value::Text(String::new()));
    match value_of(&report, "now") {
        Value::Object(object) => assert_eq!(object.borrow().class, "alpha"),
        other => panic!("the base is {other:?}"),
    }
}

#[test]
fn the_own_properties_of_an_object_are_its_own() {
    let report = run(
        "class Box { x := 1 }\nb := Box()\n\
         names := ObjOwnProps(b)\ncount := ObjOwnPropCount(b)\nhas := ObjHasOwnProp(b, \"x\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "names"), Value::List(vec![Value::Text("x".into())]));
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(int_of(&report, "has"), 1);
}

#[test]
fn the_state_of_a_map_is_not_a_property_of_it() {
    let report = run("m := Map()\nnames := ObjOwnProps(m)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "names"), Value::List(Vec::new()));
}

#[test]
fn the_name_that_counts_the_references_is_not_one_of_a_script() {
    let report = run("class Box {}\nb := Box()\na := ObjAddRef(b)");
    assert_eq!(failure_of(&report), "Parameter #1 of ObjAddRef is invalid.");
}

#[test]
fn the_names_of_com_fail_with_the_reason() {
    let report = run("a := ObjFromPtr(0)");
    assert_eq!(
        failure_of(&report),
        "this is a name of Windows, and this desktop is not Windows."
    );
}

#[test]
fn a_method_of_this_runtime_is_not_bound_to_an_object() {
    let report = run("class Box {}\nb := Box()\na := ObjBindMethod(b, \"x\")");
    assert_eq!(failure_of(&report), "a method of this runtime is not bound to an object.");
}

#[test]
fn mod_takes_the_remainder() {
    let report = run("a := Mod(7, 3)\nb := Mod(7.5, 2)\nc := Mod(-7, 3)");
    assert!(report.ok, "{:?}", report.error);
    // The answer is a whole number when both of the arguments are.
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Float(1.5));
    // The sign of the remainder is the sign of the dividend.
    assert_eq!(value_of(&report, "c"), Value::Int(-1));
}

#[test]
fn mod_by_zero_says_so() {
    let report = run("a := Mod(7, 0)");
    assert_eq!(failure_of(&report), "Divide by zero.");
}

#[test]
fn random_answers_a_number_in_its_range() {
    let report = run("a := Random()\nb := Random(10)\nc := Random(5, 10)");
    assert!(report.ok, "{:?}", report.error);
    let unit = float_of(&report, "a");
    assert!((0.0..1.0).contains(&unit), "Random() is {unit}");
    let below = int_of(&report, "b");
    assert!((0..10).contains(&below), "Random(10) is {below}");
    let between = int_of(&report, "c");
    assert!((5..=10).contains(&between), "Random(5, 10) is {between}");
}

#[test]
fn sort_puts_the_lines_in_order() {
    let report = run("a := Sort(\"pear\" . Chr(10) . \"apple\" . Chr(10) . \"fig\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("apple\nfig\npear".into()));
}

#[test]
fn sort_puts_a_list_in_order() {
    let mut desktop = Fake::default();
    let mut runtime = Runtime::new(&mut desktop, Duration::from_secs(10));
    let sort = ahk::builtins::find("Sort").expect("the tree carries Sort");
    let numbers = Value::List(vec![Value::Int(3), Value::Int(1), Value::Int(2)]);
    let sorted = (sort.handler)(&mut runtime, &[numbers], 1, 0).expect("the list is sorted");
    assert_eq!(sorted, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]));
    let words = Value::List(vec![Value::Text("pear".into()), Value::Text("apple".into())]);
    let sorted = (sort.handler)(&mut runtime, &[words], 1, 0).expect("the list is sorted");
    assert_eq!(sorted, Value::List(vec![Value::Text("apple".into()), Value::Text("pear".into())]));
}

#[test]
fn sort_of_something_that_is_not_a_list_fails() {
    let report = run("a := Sort(5)");
    assert_eq!(failure_of(&report), "Parameter #1 of Sort is invalid.");
}

#[test]
fn chr_and_ord_know_the_characters() {
    let report = run("a := Chr(65)\nb := Ord(\"A\")\nc := Ord(\"\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Int(65));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
}

#[test]
fn chr_of_a_code_that_is_not_a_character_fails() {
    let report = run("a := Chr(-1)");
    assert_eq!(failure_of(&report), "Parameter #1 of Chr is invalid.");
}

#[test]
fn str_compare_says_which_text_comes_first() {
    let report = run(concat!(
        "a := StrCompare(\"a\", \"b\")",
        "\nb := StrCompare(\"b\", \"a\")",
        "\nc := StrCompare(\"a\", \"a\")",
        "\nd := StrCompare(\"A\", \"a\")",
        "\ne := StrCompare(\"A\", \"a\", \"On\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(-1));
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    // The case is not looked at, unless the caller asks for it.
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(-1));
}

#[test]
fn format_puts_the_values_in_the_pattern() {
    let report = run(concat!(
        "a := Format(\"{1} of {2}\", 2, 3)",
        "\nb := Format(\"{{1}}\", 2)",
        "\nc := Format(\"{1:.2}\", 2.345)",
        "\nd := Format(\"{1}\", 2.5)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2 of 3".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("{1}".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("2.35".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("2.5".into()));
}

#[test]
fn format_without_a_pattern_fails() {
    let report = run("a := Format()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function. Specifically: Format"
    );
}

#[test]
fn in_str_says_where_the_needle_is() {
    let report = run(concat!(
        "a := InStr(\"hello\", \"ll\")",
        "\nb := InStr(\"hello\", \"z\")",
        "\nc := InStr(\"hello\", \"l\", 3)",
        "\nd := InStr(\"hello\", \"l\", 1, 4)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(3));
    assert_eq!(value_of(&report, "d"), Value::Int(3));
}

#[test]
fn in_str_with_an_empty_needle_fails() {
    let report = run("a := InStr(\"hello\", \"\")");
    assert_eq!(failure_of(&report), "Parameter #2 of InStr is invalid.");
}

#[test]
fn str_replace_swaps_the_parts() {
    let report = run(concat!(
        "a := StrReplace(\"hello\", \"l\", \"L\")",
        "\nb := StrReplace(\"hello\", \"l\", \"L\", 1)",
        "\nc := StrReplace(\"hello\", \"L\", \"L\")",
        "\nd := StrReplace(\"Hello hello\", \"hello\", \"x\", 0)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("heLLo".into()));
    // The fourth parameter of v2 is the case sense, and it is on when the
    // caller says so.
    assert_eq!(value_of(&report, "b"), Value::Text("heLLo".into()));
    // The case is not looked at, unless the caller asks for it.
    assert_eq!(value_of(&report, "c"), Value::Text("heLLo".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("x x".into()));
}

#[test]
fn str_replace_writes_the_number_of_the_swaps() {
    let report = run(concat!(
        "a := StrReplace(\"aaa\", \"a\", \"b\", true, &count)",
        "\nb := StrReplace(\"aaa\", \"a\", \"b\", false, &other, 1)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("bbb".into()));
    assert_eq!(int_of(&report, "count"), 3);
    // The limit is the sixth parameter, and it says how many of the
    // swaps are made.
    assert_eq!(value_of(&report, "b"), Value::Text("baa".into()));
    assert_eq!(int_of(&report, "other"), 1);
}

#[test]
fn str_replace_wants_a_reference_where_it_writes_the_count() {
    let report = run("a := StrReplace(\"aaa\", \"a\", \"b\", true, 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of StrReplace requires a variable reference, but received an Integer. Specifically: 5"
    );
}

#[test]
fn var_set_str_capacity_answers_the_capacity() {
    let report = run(concat!(
        "x := \"abc\"",
        "\na := VarSetStrCapacity(&x, 100)",
        "\ny := \"abc\"",
        "\nb := VarSetStrCapacity(&y)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 100);
    // The string is emptied when a capacity is given.
    assert_eq!(value_of(&report, "x"), Value::Text(String::new()));
    // Without a capacity the length of the string is the answer, and the
    // string is left alone.
    assert_eq!(int_of(&report, "b"), 3);
    assert_eq!(value_of(&report, "y"), Value::Text("abc".into()));
}

#[test]
fn str_replace_with_an_empty_needle_fails() {
    let report = run("a := StrReplace(\"hello\", \"\", \"x\")");
    assert_eq!(failure_of(&report), "Parameter #2 of StrReplace is invalid.");
}

#[test]
fn the_trims_take_the_characters_off_the_ends() {
    let report = run(concat!(
        "a := Trim(\"  hi  \")",
        "\nb := LTrim(\"  hi  \")",
        "\nc := RTrim(\"  hi  \")",
        "\nd := Trim(\"xxhixx\", \"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("hi".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("hi  ".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("  hi".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("hi".into()));
}

#[test]
fn sub_str_takes_the_part_between_the_places() {
    let report = run("a := SubStr(\"hello\", 2)\nb := SubStr(\"hello\", 2, 2)\nc := SubStr(\"hello\", -2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("ello".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("el".into()));
    // A negative place counts from the end.
    assert_eq!(value_of(&report, "c"), Value::Text("lo".into()));
}

#[test]
fn sub_str_without_a_place_fails() {
    let report = run("a := SubStr(\"hello\")");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function. Specifically: SubStr"
    );
}

#[test]
fn the_predicates_say_what_the_value_is() {
    let report = run(concat!(
        "a := IsAlnum(\"abc123\")",
        "\nb := IsAlpha(\"abc\")",
        "\nc := IsDigit(\"123\")",
        "\nd := IsFloat(\"1.5\")",
        "\ne := IsInteger(\"42\")",
        "\nf := IsLower(\"abc\")",
        "\ng := IsNumber(\"4.2\")",
        "\nh := IsObject(1)",
        "\ni := IsSpace(\" \")",
        "\nj := IsTime(\"20240101\")",
        "\nk := IsUpper(\"ABC\")",
        "\nl := IsXDigit(\"1F\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
    assert_eq!(value_of(&report, "d"), Value::Int(1));
    assert_eq!(value_of(&report, "e"), Value::Int(1));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
    assert_eq!(value_of(&report, "g"), Value::Int(1));
    assert_eq!(value_of(&report, "h"), Value::Int(0));
    assert_eq!(value_of(&report, "i"), Value::Int(1));
    assert_eq!(value_of(&report, "j"), Value::Int(1));
    assert_eq!(value_of(&report, "k"), Value::Int(1));
    assert_eq!(value_of(&report, "l"), Value::Int(1));
}

#[test]
fn the_predicates_that_look_at_the_letters_want_a_text() {
    let report = run("a := IsAlpha(5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn the_predicates_say_no_when_they_should() {
    let report = run(concat!(
        "a := IsAlpha(\"ab1\")",
        "\nb := IsDigit(\"12a\")",
        "\nc := IsFloat(\"12\")",
        "\nd := IsInteger(\"1.5\")",
        "\ne := IsNumber(\"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(0));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(0));
}

#[test]
fn type_says_the_name_of_the_type() {
    let report = run("a := Type(1)\nb := Type(1.5)\nc := Type(\"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Integer".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("Float".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("String".into()));
}

#[test]
fn the_type_of_an_object_is_the_name_of_its_class() {
    let report = run(
        "class Box {}\nb := Box()\n\
         a := Type(b)\nc := Type(Box)\nd := Type(Map())\ne := Type(Box.Prototype)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Box".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("Map".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("Prototype".into()));
}

#[test]
fn regex_match_says_where_the_pattern_is() {
    let report = run(concat!(
        "a := RegExMatch(\"hello\", \"l+\")",
        "\nb := RegExMatch(\"hello\", \"z\")",
        "\nc := RegExMatch(\"hello\", \"(l)(l)\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(3));
}

#[test]
fn regex_match_with_an_output_variable_fails() {
    let report = run("a := RegExMatch(\"hello\", \"l\", \"out\")");
    assert_eq!(failure_of(&report), "the matches of the pattern are not carried into a variable yet.");
}

#[test]
fn regex_replace_swaps_the_matches() {
    let report = run(concat!(
        "a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\")",
        "\nb := RegExReplace(\"a1b2\", \"([0-9])\", \"#$1#\")",
        "\nc := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &count, 1)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("a#b#".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a#1#b#2#".into()));
    // The limit is the fifth parameter of v2, and the number of the swaps
    // goes into the variable of the caller.
    assert_eq!(value_of(&report, "c"), Value::Text("a#b2".into()));
    assert_eq!(int_of(&report, "count"), 1);
}

#[test]
fn regex_replace_looks_at_the_case_when_the_pattern_says_so() {
    let report = run(concat!(
        "a := RegExReplace(\"Hello hello\", \"hello\", \"x\")",
        "\nb := RegExReplace(\"Hello hello\", \"(?i)hello\", \"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The pattern of v2 is exact, and the option `(?i)` is what takes the
    // case away.
    assert_eq!(value_of(&report, "a"), Value::Text("Hello x".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("x x".into()));
}

#[test]
fn regex_replace_starts_where_it_is_told_to() {
    let report = run(concat!(
        "a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &count, 2, 3)",
        "\nb := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &other, 2, 2)",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The sixth parameter of v2 is the place where the search starts, and
    // the text before it is left alone.
    assert_eq!(value_of(&report, "a"), Value::Text("a1b#".into()));
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(value_of(&report, "b"), Value::Text("a#b#".into()));
    assert_eq!(int_of(&report, "other"), 2);
}

#[test]
fn regex_replace_wants_a_reference_where_it_writes_the_count() {
    let report = run("a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of RegExReplace requires a variable reference, but received an Integer. Specifically: 5"
    );
}
