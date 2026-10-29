//! The built-ins, against a fake desktop.
//!
//! The cases are the language ones: what a built-in answers, and what it
//! says when it is given the wrong thing. The wording of the failures is
//! the wording of AutoHotkey v2, because that is what a script sees.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Machine, Native, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::dll::{encode_number, NativeCall, NativeResult, NativeType, NativeValue};
use ahk::runtime::{Report, Runtime, Value};
use ahk::parse;
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    calls: Vec<(String, String)>,
    /// The native call the script made, if it made one.
    native: Option<NativeCall>,
    /// The answer the fake gives to a native call.
    answer: Option<NativeResult>,
    /// The numbers the fake writes into the arguments that are the
    /// address of a variable, in the order the arguments were given.
    writes: Vec<f64>,
    /// The text on the clipboard of the fake.
    clipboard: String,
}

impl Desktop for Fake {}

impl Sound for Fake {
    fn sound_volume(&mut self, device: &str) -> Result<f64, String> {
        self.calls.push(("volume".into(), device.into()));
        Ok(50.0)
    }
    fn sound_set_volume(&mut self, device: &str, volume: f64) -> Result<(), String> {
        self.calls.push(("set-volume".into(), format!("{device} {volume}")));
        Ok(())
    }
    fn sound_muted(&mut self, device: &str) -> Result<bool, String> {
        self.calls.push(("mute".into(), device.into()));
        Ok(false)
    }
    fn sound_set_muted(&mut self, device: &str, muted: bool) -> Result<(), String> {
        self.calls.push(("set-mute".into(), format!("{device} {muted}")));
        Ok(())
    }
    fn sound_play(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("play".into(), path.into()));
        Ok(format!("played {path}"))
    }
    fn sound_beep(&mut self, frequency: f64, duration: u64) -> Result<String, String> {
        self.calls.push(("beep".into(), format!("{frequency} {duration}")));
        Ok("beeped".to_string())
    }
}
impl Machine for Fake {}

impl Windows for Fake {}

impl Files for Fake {
    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        self.calls.push(("working-dir".into(), directory.into()));
        Ok(format!("the working directory is now {directory}"))
    }
    fn file_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(path.ends_with(".txt"))
    }
    fn dir_exists(&mut self, path: &str) -> Result<bool, String> {
        Ok(path == "/tmp")
    }
    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        Ok(if path.ends_with(".txt") { vec!["A".to_string()] } else { vec!["D".to_string()] })
    }
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Ok(vec!["one".to_string(), "two".to_string()])
    }
    fn append_file(&mut self, _path: &str, text: &str) -> Result<String, String> {
        Ok(text.to_string())
    }
}

impl Clipboard for Fake {
    fn clipboard_read(&mut self) -> Result<String, String> {
        self.calls.push(("clipboard-read".into(), String::new()));
        Ok(self.clipboard.clone())
    }
    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        self.calls.push(("clipboard-write".into(), text.into()));
        self.clipboard = text.to_string();
        Ok(text.to_string())
    }
}

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
        self.calls.push(("click".into(), format!("{x} {y}")));
        Ok(format!("clicked ({x}, {y})"))
    }
    fn mouse_down(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls.push(("down".into(), format!("{x:?} {y:?} {button}")));
        Ok("down".to_string())
    }
    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls.push(("up".into(), format!("{x:?} {y:?} {button}")));
        Ok("up".to_string())
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("move".into(), format!("{x} {y}")));
        Ok("moved".to_string())
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

impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, String> {
        self.native = Some(call.clone());
        // The bytes the callee left in the memory of an address come
        // back into the argument: the argument is the way back.
        let mut writes = self.writes.iter();
        for argument in call.arguments.iter_mut() {
            if !argument.address {
                continue;
            }
            let Some(number) = writes.next() else { continue };
            argument.value = NativeValue::Bytes(encode_number(*number, argument.kind));
        }
        Ok(self.answer.clone().unwrap_or(NativeResult {
            kind: call.returns,
            value: NativeValue::Integer(0),
        }))
    }
}

/// A desktop that cannot make a native call.
///
/// It writes the capability, but not the method: the default of the
/// trait says that this desktop cannot do it.
#[derive(Debug, Default)]
struct Blind;

impl Desktop for Blind {}
impl Sound for Blind {}
impl Machine for Blind {}

impl Windows for Blind {}

impl Files for Blind {}

impl Clipboard for Blind {}

impl Timers for Blind {}

impl Hotkeys for Blind {}

impl Native for Blind {}

impl Keyboard for Blind {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Pointer for Blind {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Screen for Blind {
    fn capture(&mut self, _x: i64, _y: i64, _width: i64, _height: i64, _filename: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Processes for Blind {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

/// Run a script and answer with its report.
fn run(source: &str) -> Report {
    let mut desktop = Fake::default();
    run_on(source, &mut desktop)
}

/// Run a script against a desktop and answer with its report.
///
/// The desktop is the one of the caller, so that a case can look at
/// what the call carried.
fn run_on(source: &str, desktop: &mut Fake) -> Report {
    let statements = parse(source).expect("parses");
    Runtime::new(desktop, Duration::from_secs(10)).run(&statements)
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
fn the_volume_of_a_device_is_a_number_between_zero_and_a_hundred() {
    let report = run("volume := SoundGetVolume()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(float_of(&report, "volume"), 50.0);
}

#[test]
fn the_volume_of_a_device_can_be_set() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume 30", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("set-volume".to_string(), " 30".to_string())]);
}

#[test]
fn a_sign_in_front_of_the_volume_adjusts_it() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume \"+10\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The volume of the device is asked first, and the sign adds ten to
    // it: 50 becomes 60.
    assert_eq!(
        desktop.calls,
        vec![("volume".to_string(), String::new()), ("set-volume".to_string(), " 60".to_string())]
    );
}

#[test]
fn a_volume_that_is_not_a_percentage_fails_the_script() {
    let report = run("SoundSetVolume \"loud\"");
    let failure = failure_of(&report);
    assert!(failure.contains("percentage between -100 and 100"), "{failure}");
}

#[test]
fn the_mute_of_a_device_is_a_number() {
    let report = run("muted := SoundGetMute()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "muted"), 0);
}

#[test]
fn a_device_can_be_muted_and_unmuted() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetMute 1", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("set-mute".to_string(), " true".to_string())]);
}

#[test]
fn a_minus_sign_in_front_of_the_volume_adjusts_it_down() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume \"-10\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The volume of the device is 50, and the sign takes ten from it.
    assert_eq!(
        desktop.calls,
        vec![("volume".to_string(), String::new()), ("set-volume".to_string(), " 40".to_string())]
    );
}

#[test]
fn a_component_that_the_box_does_not_have_fails_the_script() {
    let report = run("SoundGetVolume(\"Microphone\")");
    let failure = failure_of(&report);
    assert!(failure.contains("one volume per device"), "{failure}");
}

#[test]
fn a_tone_is_a_frequency_and_a_duration() {
    let mut desktop = Fake::default();
    let report = run_on("SoundBeep 750, 500", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("beep".to_string(), "750 500".to_string())]);
}

#[test]
fn a_tone_has_the_defaults_of_v2() {
    let mut desktop = Fake::default();
    let report = run_on("SoundBeep", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The frequency of v2 is 523 and its duration is 150 milliseconds.
    assert_eq!(desktop.calls, vec![("beep".to_string(), "523 150".to_string())]);
}

#[test]
fn a_frequency_that_is_not_a_tone_fails_the_script() {
    let report = run("SoundBeep 1");
    let failure = failure_of(&report);
    assert!(failure.contains("between 37 and 32767"), "{failure}");
}

#[test]
fn a_sound_file_is_played() {
    let mut desktop = Fake::default();
    let report = run_on("SoundPlay \"/tmp/a.wav\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("play".to_string(), "/tmp/a.wav".to_string())]);
}

#[test]
fn a_sound_that_the_desktop_cannot_answer_fails_the_script() {
    let statements = parse("volume := SoundGetVolume()").expect("parses");
    let mut desktop = Blind;
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    let failure = failure_of(&report);
    assert!(failure.contains("cannot"), "{failure}");
}

#[test]
fn the_names_of_the_sound_that_are_windows_fail_with_the_reason() {
    let report = run("SoundGetName()");
    assert_eq!(failure_of(&report), "this is a name of Windows, and this desktop is not Windows.");
}

#[test]
fn a_string_in_title_case_has_the_first_letter_of_each_word_up() {
    let report = run("a := StrTitle(\"This is a test.\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("This Is A Test.".into()));
}

#[test]
fn the_title_case_lowercases_the_rest_of_the_word() {
    let report = run("a := StrTitle(\"GONE with the WIND\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Gone With The Wind".into()));
}

#[test]
fn the_versions_are_compared_by_their_numbers() {
    let report = run("a := VerCompare(\"1.20.0\", \"1.3\")");
    assert!(report.ok, "{:?}", report.error);
    // The numbers are compared one by one, and twenty is bigger than three.
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_version_with_a_pre_release_is_older_than_the_same_version() {
    let report = run(
        "a := VerCompare(\"2.0-a137\", \"2.0\")\nb := VerCompare(\"2.0-a137\", \"2.0-a136\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), -1);
    assert_eq!(int_of(&report, "b"), 1);
}

#[test]
fn the_second_version_may_carry_an_operator() {
    let report = run("a := VerCompare(\"2.0.1\", \">=2.0\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_label_is_not_there() {
    // The tree has no labels: there is nothing to jump to.
    let report = run("a := IsLabel(\"there\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 0);
}

#[test]
fn the_working_directory_of_the_script_can_be_changed() {
    let mut desktop = Fake::default();
    let report = run_on("SetWorkingDir \"there\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("working-dir".to_string(), "there".to_string())]);
}

#[test]
fn the_click_of_v2_takes_the_button_first() {
    let mut desktop = Fake::default();
    let report = run_on("MouseClick \"Right\", 10, 20", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(desktop.calls, vec![("click".to_string(), "10 20".to_string())]);
}

#[test]
fn a_drag_goes_down_in_one_place_and_comes_up_in_another() {
    let mut desktop = Fake::default();
    let report = run_on("MouseClickDrag \"Left\", 10, 20, 30, 40", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![
            ("down".to_string(), "Some(10) Some(20) left".to_string()),
            ("move".to_string(), "30 40".to_string()),
            ("up".to_string(), "None None left".to_string()),
        ]
    );
}

#[test]
fn a_name_of_windows_fails_with_the_reason() {
    let report = run("StrGet(\"a\")");
    assert_eq!(failure_of(&report), "this is a name of Windows, and this desktop is not Windows.");
}

#[test]
fn a_hotstring_that_v2_would_refuse_fails_with_the_reason_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-28: the name of a
    // hotstring is written between two of the same character, and the
    // name is checked before the action.
    let report = run("Hotstring(\"a\", \"b\")");
    assert_eq!(failure_of(&report), "Parameter #1 of Hotstring is invalid.");
}

#[test]
fn the_date_is_answered_the_way_v2_answers_it() {
    // The date of v2 is a number of fourteen digits, and the unit comes
    // last: DateAdd("20260101", 1, "d") is the second of January.
    let report = run("a := DateAdd(\"20260101\", 1, \"d\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("20260102000000".into()));
}

#[test]
fn a_unit_that_v2_does_not_have_fails_with_the_reason_of_v2() {
    let report = run("DateAdd(\"20260101\", 1, \"days\")");
    assert_eq!(failure_of(&report), "Parameter #3 of DateAdd is invalid.");
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
    // v2 answers with the attributes and not with a number.
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("D".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("".into()));
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

/// `ObjBindMethod` is a name of v2, and not a method of a class: the
/// tables that were read out of the binary had it in a table of methods,
/// and the tree carried the reason of that mistake. The name is a
/// function of the object model, and the tree does not have the bound
/// functions of v2 yet.
#[test]
fn a_name_that_the_tree_does_not_carry_yet_says_so() {
    let report = run("class Box {}\nb := Box()\na := ObjBindMethod(b, \"x\")");
    assert_eq!(failure_of(&report), "this runtime does not carry that name yet.");
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

// -- the clipboard ----------------------------------------------------------------

#[test]
fn a_wait_for_the_clipboard_that_finds_data_answers_one() {
    let mut desktop = Fake { clipboard: "the text".to_string(), ..Default::default() };
    let report = run_on("MsgBox ClipWait(0.25)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_wait_for_the_clipboard_that_times_out_answers_zero() {
    let mut desktop = Fake::default();
    let report = run_on("MsgBox ClipWait(0.25)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_wait_for_the_clipboard_with_no_timeout_waits_for_the_data() {
    let mut desktop = Fake { clipboard: "the text".to_string(), ..Default::default() };
    let report = run_on("MsgBox ClipWait()", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_wait_for_the_data_of_any_kind_fails_with_the_reason_of_the_text() {
    let mut desktop = Fake::default();
    let report = run_on("MsgBox ClipWait(0.25, 1)", &mut desktop);
    assert_eq!(
        failure_of(&report),
        "the clipboard of this runtime carries text only, and it cannot wait for data of any kind."
    );
}

#[test]
fn a_desktop_that_cannot_read_the_clipboard_fails_the_wait_with_the_reason() {
    let mut desktop = Blind;
    let statements = parse("ClipWait(0.25)").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert_eq!(failure_of(&report), "this desktop cannot read the clipboard");
}

#[test]
fn each_name_of_the_shape_of_the_runtime_refuses_with_the_reason_of_its_own_thing() {
    // The fourteen names of the shape shared one reason, and that reason
    // hid fourteen different things.
    for (name, reason) in [
        (
            "Critical",
            "this runtime runs one script in one thread, and it cannot make it critical.",
        ),
        (
            "Thread",
            "this runtime runs one script in one thread, and it cannot start another.",
        ),
        (
            "Pause",
            "this runtime cannot wait for a key to continue: the script ends when it ends.",
        ),
        (
            "Persistent",
            "this runtime is a script that ends when it ends, and it cannot be kept alive.",
        ),
        (
            "Reload",
            "this runtime runs one script once, and it cannot run itself again.",
        ),
        ("Suspend", "this runtime cannot stop itself: it would never end."),
        (
            "ListLines",
            "the lines are listed in a window of v2, and this runtime has no window to show them in.",
        ),
        (
            "ListVars",
            "the variables are listed in a window of v2, and this runtime has no window to show them in.",
        ),
        (
            "InputBox",
            "the question of the user is asked in a window of v2, and this runtime has no window to ask in.",
        ),
        (
            "Download",
            "the download is made by the desktop, and this desktop does not reach the network.",
        ),
        (
            "LoadPicture",
            "the picture is read by the desktop, and this desktop does not read the images of the machine.",
        ),
        (
            "ImageSearch",
            "the search of a picture is made by the desktop, and this desktop does not read the images of the machine.",
        ),
    ] {
        let report = run(name);
        assert_eq!(failure_of(&report), reason, "{name}");
        // The reason of the name is the one that the surface says.
        assert_eq!(ahk::builtins::refusal_of(name), Some(reason), "{name}");
    }
}

#[test]
fn the_function_of_the_exit_takes_the_reason_and_the_code() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the callback of
    // the exit takes the reason of the exit and the code.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason \", \" ExitCode\n",
        "}\n",
        "MsgBox \"the script ends\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the script ends", "Exit, 0"]);
}

#[test]
fn the_function_of_the_exit_runs_when_the_script_stops_with_a_code() {
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason \", \" ExitCode\n",
        "}\n",
        "Exit 3\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Exit, 3"]);
    assert_eq!(report.exit_code, Some(3));
}

#[test]
fn the_function_of_the_exit_runs_when_the_script_fails() {
    // The end of the script is the end of the script, whatever the reason
    // it ended, and the reason is the one of v2.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason\n",
        "}\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.output, vec!["Error"]);
}

#[test]
fn a_function_of_the_exit_that_fails_fails_the_run() {
    let report = run("OnExit Bye\nBye(Reason, Code) {\n  MsgBox NoSuchVariable\n}\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
    // The line is the line of the statement that failed, inside the
    // function.
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_function_of_the_exit_that_fails_when_the_script_already_failed_is_a_note() {
    // The error of the script is the error of the run, and the failure of
    // the function is not lost.
    let report = run(
        "OnExit Bye\nBye(Reason, Code) {\n  MsgBox NoSuchVariable\n}\nMsgBox NoSuchVariable\n",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert!(
        report.notes.iter().any(|note| note.contains("the function of the exit failed")),
        "{:?}",
        report.notes
    );
}

#[test]
fn a_text_is_not_a_callback_and_refuses_with_the_reason_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the callback is a
    // function object, and a text is refused.
    let report = run("OnExit(\"Bye\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of OnExit requires an Object, but received a String. Specifically: Bye"
    );
}

#[test]
fn a_value_that_cannot_be_called_is_not_a_callback() {
    // v2 calls the value, and a value that cannot be called has no method
    // named "Call".
    let report = run("OnExit Map()");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Object\" has no method named \"Call\"."
    );
}

#[test]
fn a_prototype_that_does_not_take_the_two_parameters_is_not_a_callback() {
    // Measured on the binary of v2.0.19 on 2026-10-29.
    let report = run("OnExit Bye\nBye() {\n}\n");
    assert_eq!(failure_of(&report), "Invalid callback function.");
}

#[test]
fn a_callback_that_takes_the_rest_of_the_arguments_is_a_callback() {
    // The asterisk takes what is left, and it fits.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode*) {\n",
        "  MsgBox ExitReason\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Exit"]);
}

#[test]
fn the_callback_of_the_exit_is_required() {
    let report = run("OnExit()");
    assert_eq!(failure_of(&report), "Too few parameters passed to function. Specifically: OnExit");
}

#[test]
fn the_order_of_the_callbacks_is_one_minus_one_or_zero() {
    let report = run("OnExit Bye, 5\nBye(Reason, Code) {\n}\n");
    assert_eq!(failure_of(&report), "Parameter #2 of OnExit is invalid. Specifically: 5");
}

#[test]
fn an_order_that_is_not_a_number_refuses_with_the_reason_of_v2() {
    let report = run("OnExit Bye, \"x\"\nBye(Reason, Code) {\n}\n");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of OnExit requires a Number, but received a String. Specifically: x"
    );
}

#[test]
fn the_order_is_cut_to_a_whole_number() {
    // Measured on the binary of v2.0.19 on 2026-10-30: 1.5 is 1.
    let report = run("OnExit Bye, 1.5\nBye(Reason, Code) {\n  MsgBox \"bye\"\n}\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["bye"]);
}

#[test]
fn the_callback_that_is_asked_not_to_be_called_is_removed() {
    let report = run(concat!(
        "OnExit Bye\n",
        "OnExit Bye, 0\n",
        "Bye(Reason, Code) {\n",
        "  MsgBox \"bye\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

#[test]
fn the_callbacks_run_in_the_order_the_script_asked_for() {
    // One is after the ones that are registered, and minus one is before
    // them.
    let report = run(concat!(
        "First(Reason, Code) {\n  MsgBox \"first\"\n}\n",
        "Second(Reason, Code) {\n  MsgBox \"second\"\n}\n",
        "OnExit First\n",
        "OnExit Second\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["first", "second"]);
}

#[test]
fn a_callback_that_asks_to_run_before_the_others_runs_before_them() {
    let report = run(concat!(
        "First(Reason, Code) {\n  MsgBox \"first\"\n}\n",
        "Second(Reason, Code) {\n  MsgBox \"second\"\n}\n",
        "OnExit First\n",
        "OnExit Second, -1\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["second", "first"]);
}

#[test]
fn the_same_callback_registered_twice_runs_once() {
    // v2 keeps one entry for a callback.
    let report = run(concat!(
        "Bye(Reason, Code) {\n  MsgBox \"bye\"\n}\n",
        "OnExit Bye\n",
        "OnExit Bye\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["bye"]);
}

#[test]
fn a_function_of_the_exit_that_answers_a_number_keeps_the_script_alive() {
    // v2: the exit does not happen, and the callbacks that come after it
    // are not called. This runtime has no way to keep a script alive, so
    // the run ends and the note says so.
    let report = run(concat!(
        "Bye(Reason, Code) {\n  return 1\n}\n",
        "OnExit Bye\n",
        "Exit 3\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.exit_code, None);
    assert!(
        report.notes.iter().any(|note| note.contains("keep the script alive")),
        "{:?}",
        report.notes
    );
}

#[test]
fn the_function_of_the_error_takes_the_value_and_the_mode() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the mode of a
    // failure of the runtime is "Return".
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox Mode\n}\n",
        "OnError Handler\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert_eq!(report.output, vec!["Return"]);
}

#[test]
fn the_function_of_the_error_takes_the_value_that_was_thrown() {
    // A value that the script threw is not a continuable failure: the
    // mode is "Exit".
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox Mode \", \" Thrown\n}\n",
        "OnError Handler\n",
        "throw \"the error of the script\"\n",
    ));
    assert!(!report.ok);
    assert_eq!(report.output, vec!["Exit, the error of the script"]);
}

#[test]
fn a_function_of_the_error_that_answers_one_hides_the_failure() {
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  return 1\n}\n",
        "OnError Handler\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(
        report.notes.iter().any(|note| note.contains("hid the failure")),
        "{:?}",
        report.notes
    );
}

#[test]
fn a_function_of_the_error_that_answers_minus_one_lets_the_thread_continue() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the thread goes on
    // where the failure happened.
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  return -1\n}\n",
        "OnError Handler\n",
        "Fail()\n",
        "MsgBox \"after the call\"\n",
        "Fail() {\n",
        "  MsgBox NoSuchVariable\n",
        "  MsgBox \"inside the function\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["inside the function", "after the call"]);
}

#[test]
fn the_functions_of_the_error_are_not_called_while_a_try_is_active() {
    // v2: the catch of the script takes the failure first.
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox \"the handler\"\n}\n",
        "OnError Handler\n",
        "try {\n",
        "  MsgBox NoSuchVariable\n",
        "} catch {\n",
        "  MsgBox \"in the catch\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["in the catch"]);
}

#[test]
fn the_function_of_the_error_is_removed_when_the_order_is_zero() {
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox \"the handler\"\n}\n",
        "OnError Handler\n",
        "OnError Handler, 0\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

#[test]
fn a_script_that_writes_the_clipboard_finds_what_it_wrote_when_it_reads_it_again() {
    let mut desktop = Fake::default();
    let report = run_on("A_Clipboard := \"the new text\"\nMsgBox A_Clipboard", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the new text"]);
    assert_eq!(desktop.clipboard, "the new text");
}

// -- the native call, against a desktop that records it ------------------------

#[test]
fn the_library_and_the_function_are_read_out_of_the_first_parameter() {
    let mut desktop = Fake::default();
    let report = run_on("DllCall(\"kernel32\\GetLastError\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.library, "kernel32");
    assert_eq!(call.function, "GetLastError");
}

#[test]
fn the_types_of_the_arguments_are_read_in_pairs() {
    let mut desktop = Fake::default();
    let report = run_on("DllCall(\"f\", \"Int\", 5, \"Str\", \"hello\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.arguments.len(), 2);
    assert_eq!(call.arguments[0].kind, NativeType::Int);
    assert_eq!(call.arguments[0].value, NativeValue::Integer(5));
    assert_eq!(call.arguments[1].kind, NativeType::Str);
    // A text goes out as its bytes: two a character, and the nul at the end.
    assert_eq!(
        call.arguments[1].value,
        NativeValue::Bytes(vec![b'h', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, 0, 0])
    );
}

#[test]
fn the_type_of_the_answer_is_the_last_parameter_when_the_rest_is_odd() {
    let mut desktop = Fake::default();
    let report = run_on("DllCall(\"f\", \"Int\", 5, \"Double\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.returns, NativeType::Double);
    assert_eq!(call.arguments.len(), 1);
}

#[test]
fn the_answer_of_the_desktop_comes_back_to_the_script() {
    let mut desktop = Fake {
        answer: Some(NativeResult { kind: NativeType::Int, value: NativeValue::Integer(42) }),
        ..Default::default()
    };
    let report = run_on("a := DllCall(\"f\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(42));
}

#[test]
fn a_star_at_the_end_of_a_type_is_the_address_of_a_variable() {
    // The fake writes into the memory of the address, the way the box does.
    let mut desktop = Fake { writes: vec![42.0], ..Default::default() };
    let report = run_on("counter := 0\nDllCall(\"f\", \"Int64*\", &counter)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "counter"), 42);
}

#[test]
fn a_type_that_is_not_a_type_fails() {
    let report = run("DllCall(\"f\", \"Nope\", 5)");
    assert_eq!(failure_of(&report), "Invalid arg type.");
}

#[test]
fn a_type_of_the_answer_that_is_not_a_type_fails() {
    let report = run("DllCall(\"f\", \"Int\", 5, \"Nope\")");
    assert_eq!(failure_of(&report), "Invalid return type.");
}

#[test]
fn a_number_where_a_text_is_wanted_fails() {
    let report = run("DllCall(\"f\", 5, 5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn a_text_where_a_number_is_wanted_fails() {
    let report = run("DllCall(\"f\", \"Int\", \"hello\")");
    assert_eq!(failure_of(&report), "Expected a Number but got a String.");
}

#[test]
fn a_first_parameter_that_is_not_a_text_fails() {
    let report = run("DllCall(5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn a_desktop_that_cannot_make_a_native_call_fails_with_the_reason() {
    let mut desktop = Blind;
    let statements = parse("DllCall(\"f\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert_eq!(failure_of(&report), "this desktop cannot make a native call.");
}

#[test]
fn the_failure_of_a_built_in_names_the_built_in() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the failure of a
    // built-in carries the name of it in the `What`, and the value of the
    // parameter that failed in the `Extra`.
    let report = run(
        "try {\n    x := Chr(-1)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Parameter #1 of Chr is invalid. | Chr | -1"]);
}

#[test]
fn the_failure_of_a_built_in_that_wants_a_number_carries_the_value() {
    let report = run(
        "try {\n    x := Round(\"abc\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of Round requires a Number, but received a String. | Round | abc"]
    );
}

#[test]
fn the_failure_of_a_built_in_is_of_the_class_of_v2() {
    let report = run(
        "try {\n    x := Chr(-1)\n} catch Any as e {\n    MsgBox e is ValueError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}
