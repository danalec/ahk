//! The built-ins, against a fake desktop.
//!
//! The cases are the language ones: what a built-in answers, and what it
//! says when it is given the wrong thing. The wording of the failures is
//! the wording of AutoHotkey v2, because that is what a script sees.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, FileAnswer, FileOptions, Files, Hotkeys, Image, ImageOptions,
    ImageSearch, InputAnswer, InputOptions, Keyboard, Machine, Native, Network, PictureOptions,
    Pictures, Pointer, Processes, Screen, Sound, Timers, Windows,
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
impl Pictures for Fake {
    fn load_picture(&mut self, filename: &str, _options: &PictureOptions) -> Result<Option<u32>, String> {
        self.calls.push(("load-picture".into(), filename.to_string()));
        Ok(Some(7))
    }
    fn image_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        image: &Image,
        _options: &ImageOptions,
    ) -> Result<ImageSearch, String> {
        self.calls.push(("image-search".into(), format!("{x1},{y1},{x2},{y2}")));
        match image {
            Image::File(name) if name.is_empty() => Ok(ImageSearch::NotLoadable),
            _ => Ok(ImageSearch::Found(10, 20)),
        }
    }
}
impl Network for Fake {
    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        self.calls.push(("download".into(), format!("{url}|{file}")));
        Ok(file.to_string())
    }
}

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
impl Dialogs for Fake {
    fn input_box(
        &mut self,
        prompt: &str,
        title: &str,
        options: &InputOptions,
        default: &str,
    ) -> Result<InputAnswer, String> {
        self.calls.push((
            "input-box".into(),
            format!("{prompt}|{title}|{default}|{:?}", options.timeout),
        ));
        Ok(InputAnswer { value: "the answer".into(), result: "OK".into() })
    }

    fn file_select(
        &mut self,
        options: &FileOptions,
        root: &str,
        title: &str,
        filter: &str,
    ) -> Result<FileAnswer, String> {
        self.calls.push(("file-select".into(), format!("{root}|{title}|{filter}")));
        if options.multiple {
            Ok(FileAnswer::Many(vec!["/tmp/one.txt".into(), "/tmp/two.txt".into()]))
        } else {
            Ok(FileAnswer::One("/tmp/chosen.txt".into()))
        }
    }

    fn dir_select(&mut self, starting: &str, options: i64, prompt: &str) -> Result<String, String> {
        self.calls.push(("dir-select".into(), format!("{starting}|{options}|{prompt}")));
        Ok("/tmp/chosen".into())
    }
}

impl Windows for Fake {}

impl Files for Fake {
    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        self.calls.push(("working-dir".into(), directory.into()));
        Ok(format!("the working directory is now {directory}"))
    }
    fn attributes_of_the_pattern(&mut self, pattern: &str, mode: &str) -> Result<Vec<String>, String> {
        self.calls.push((format!("pattern {mode}"), pattern.into()));
        if mode.contains('D') {
            return Ok(if pattern == "/tmp" { vec!["D".to_string()] } else { Vec::new() });
        }
        Ok(if pattern.ends_with(".txt") { vec!["A".to_string()] } else { Vec::new() })
    }
    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        Ok(if path.ends_with(".txt") { vec!["A".to_string()] } else { vec!["D".to_string()] })
    }
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Ok(vec!["one".to_string(), "two".to_string()])
    }
    fn read_the_bytes_of_a_file(&mut self, path: &str) -> Result<Vec<u8>, String> {
        self.calls.push(("bytes-of-file".into(), path.into()));
        Ok(b"one\r\ntwo\r\n".to_vec())
    }
    fn append_file(&mut self, path: &str, bytes: &[u8], mark: &[u8]) -> Result<String, String> {
        self.calls.push((
            "append-file".into(),
            format!("{path} {} {}", String::from_utf8_lossy(bytes), String::from_utf8_lossy(mark)),
        ));
        Ok(bytes.len().to_string())
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
impl Pictures for Blind {}
impl Network for Blind {}
impl Sound for Blind {}
impl Machine for Blind {}
impl Dialogs for Blind {}

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
fn the_pattern_of_the_files_is_asked_of_the_desktop() {
    let mut desktop = Fake::default();
    let report = run_on("a := FileExist(\"*.txt\")\nb := DirExist(\"/tmp\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // v2 answers with the attributes of the first entry that matches, and
    // a folder is asked for with the mode "D".
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("D".into()));
    assert_eq!(
        desktop.calls,
        vec![
            ("pattern ".to_string(), "*.txt".to_string()),
            ("pattern D".to_string(), "/tmp".to_string()),
        ]
    );
}

#[test]
fn the_content_of_a_file_is_the_text_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer is the
    // text of the file and not an array of its lines.
    let report = run("text := FileRead(\"a.txt\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "text"), Value::Text("one\r\ntwo\r\n".into()));
}

#[test]
fn the_read_of_a_file_takes_the_options_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the second
    // parameter is the options of the read: the size of the part that is
    // read, the raw bytes and the linefeed.
    let report = run(
        "part := FileRead(\"a.txt\", \"m5\")\nsize := StrLen(part)\nraw := FileRead(\"a.txt\", \"RAW\")\nbytes := raw.Size\nlines := FileRead(\"a.txt\", \"`n\")\ncount := StrLen(lines)\nMsgBox \"%size% %bytes% %count%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5 10 8"]);
}

#[test]
fn an_option_of_the_read_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the message is
    // "Invalid option.", the class of the failure is a ValueError, and the
    // option that was not known is in the Extra.
    let report = run(
        "try {\n    FileRead(\"a.txt\", \"bogus\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid option. | ValueError | FileRead | bogus"]);
}

#[test]
fn the_write_of_a_file_takes_the_options_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the third parameter
    // is the options of the write: the linefeed puts a carriage return
    // before every linefeed that has none.
    let mut desktop = Fake::default();
    let report = run_on("FileAppend(\"a`nb\", \"a.txt\", \"`n\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("append-file".to_string(), "a.txt a\r\nb ".to_string())]
    );
}

#[test]
fn an_option_of_the_write_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the message is
    // "Invalid option.", the class of the failure is a ValueError, and the
    // option that was not known is in the Extra.
    let report = run(
        "try {\n    FileAppend(\"a\", \"a.txt\", \"m5000\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid option. | ValueError | FileAppend | m5000"]);
}

#[test]
fn a_text_is_cut_at_the_delimiters() {
    let report = run(
        "pieces := StrSplit(\"a,b,c\", \",\")\n\
         kind := Type(pieces)\nlength := pieces.Length\nfirst := pieces[1]",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer of
    // `StrSplit` is an array of v2, and not a list of the tree.
    assert_eq!(value_of(&report, "kind"), Value::Text("Array".into()));
    assert_eq!(int_of(&report, "length"), 3);
    assert_eq!(value_of(&report, "first"), Value::Text("a".into()));
}

#[test]
fn the_delimiters_of_a_text_are_literals() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text is cut
    // where a delimiter is, and a delimiter is a literal and not a set of
    // characters.
    let report = run(
        "one := StrSplit(\"aXXb\", \"XX\")\n\
         two := StrSplit(\"a.b.c\", \"..\")\n\
         first := one[1] . \",\" . one[2]\n\
         second := two[1]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b".into()));
    assert_eq!(value_of(&report, "second"), Value::Text("a.b.c".into()));
}

#[test]
fn the_delimiters_of_a_text_may_be_an_array() {
    let report = run(
        "pieces := StrSplit(\"a.b-c\", [\".\", \"-\"])\n\
         first := pieces[1] . \",\" . pieces[2] . \",\" . pieces[3]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b,c".into()));
}

#[test]
fn the_characters_to_omit_are_cut_from_the_pieces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the characters to
    // omit are cut from the beginning and the end of every piece.
    let report = run(
        "pieces := StrSplit(\"a,b,c\", \",\", \"ac\")\n\
         first := \"[\" pieces[1] \"]\" . \"[\" pieces[2] \"]\" . \"[\" pieces[3] \"]\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("[][b][]".into()));
}

#[test]
fn the_number_of_the_pieces_says_how_many_times_the_text_is_cut() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text is cut a
    // maximum of MaxParts-1 times, and the rest of the text is the last
    // piece.
    let report = run(
        "two := StrSplit(\"a,b,c\", \",\", \"\", 2)\n\
         one := StrSplit(\"a,b,c\", \",\", \"\", 1)\n\
         none := StrSplit(\"a,b,c\", \",\", \"\", 0)\n\
         first := two[1] . \",\" . two[2] . \" \" . one[1] . \" \" . none.Length",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b,c a,b,c 0".into()));
}

#[test]
fn a_blank_delimiter_makes_a_piece_of_every_character() {
    // Measured on the binary of v2.0.19 on 2026-10-31: when the
    // delimiters are blank every character of the text is a piece, and
    // the characters to omit are not pieces at all.
    let report = run(
        "pieces := StrSplit(\"a b\", \"\", \" \")\n\
         first := pieces[1] . pieces[2]",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("ab".into()));
}

#[test]
fn a_text_that_is_empty_has_no_piece() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a text that is empty
    // has no piece at all, and the piece that is empty is not a piece.
    let report = run("pieces := StrSplit(\"\", \",\")\nnumber := pieces.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "number"), 0);
}

#[test]
fn the_delimiters_that_are_not_a_text_are_refused() {
    let report = run("pieces := StrSplit(\"a,b\", Map())");
    assert_eq!(failure_of(&report), "Parameter #2 of StrSplit is invalid.");
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
        "Parameter #2 of SplitPath requires a variable reference, but received an Integer."
    );
}

#[test]
fn an_object_can_be_asked_about() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    // class is a name of the script, and the variable may not have it.
    let report = run(concat!(
        "class Box {\n  x := 1\n}\n",
        "thing := Box()",
        "\na := HasProp(thing, \"x\")",
        "\nb := HasProp(thing, \"y\")",
        "\nc := HasBase(thing, Box.Prototype)",
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
        "Parameter #1 of IsSetRef requires a variable reference, but received an Integer."
    );
}

#[test]
fn the_commands_of_the_settings_answer_with_the_value_that_was_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every one of them
    // answers the value that was there before. The script sets the mode of
    // the titles first, so the answer does not depend on what another case
    // left behind, and it puts the mode back the way v2 starts it.
    let report = run(
        "SetTitleMatchMode(3)\na := SetTitleMatchMode(1)\nb := A_TitleMatchMode\nSetTitleMatchMode(\"RegEx\")\nc := SetTitleMatchMode(2)\nd := A_TitleMatchMode\ne := SetTitleMatchMode(\"Slow\")\nf := SetTitleMatchMode(\"Fast\")\ng := DetectHiddenWindows(true)\nh := DetectHiddenText(false)\ni := SetWinDelay(200)\nSetTitleMatchMode(2)\nMsgBox \"%a% %b% %c% %d% %e% %f% %g% %h% %i%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 1 RegEx 2 Fast Slow 0 1 100"]);
}

#[test]
fn the_commands_of_the_delay_of_the_controls_and_of_the_view_of_the_registry_answer_with_the_value_that_was_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every one of them
    // answers the value that was there before, and the view of the
    // registry is always a text.
    let report = run("a := SetControlDelay(50)\nb := SetRegView(32)\nMsgBox \"%a% %b%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["20 Default"]);
}

#[test]
fn a_delay_of_the_controls_that_is_not_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay is a
    // number, and a text that holds no number is refused.
    let report = run(
        "try {\n    SetControlDelay(\"x\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetControlDelay requires a Number, but received a String. | TypeError | SetControlDelay | x"]
    );
}

#[test]
fn a_delay_of_the_controls_that_is_out_of_the_range_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay goes from
    // -1 to the biggest number of thirty-two bits.
    let report = run(
        "try {\n    SetControlDelay(-2)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetControlDelay is invalid. | ValueError | SetControlDelay | -2"]
    );
}

#[test]
fn a_view_of_the_registry_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the view is 32, 64
    // or the word "Default".
    let report = run(
        "try {\n    SetRegView(33)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetRegView is invalid. | ValueError | SetRegView | 33"]
    );
}

#[test]
fn a_word_that_is_neither_the_mode_nor_the_speed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode is a
    // number from 1 to 3, the word "RegEx", or the speed -- "Fast" or
    // "Slow".
    let report = run(
        "try {\n    SetTitleMatchMode(\"Bogus\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | Bogus"]
    );
}

#[test]
fn a_mode_of_the_titles_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode is a number
    // from 1 to 3, and the others are refused.
    let report = run(
        "try {\n    SetTitleMatchMode(5)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | 5"]);
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
fn the_kind_of_a_value_is_named_in_the_message() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the kind of a value
    // is the name of its class.
    for (source, kind) in [
        ("a := Abs({})", "an Object"),
        ("a := Abs([1])", "an Array"),
        ("a := Abs(Map())", "a Map"),
        ("a := Abs(() => 1)", "a Func"),
    ] {
        let report = run(source);
        assert_eq!(
            failure_of(&report),
            format!("Parameter #1 of Abs requires a Number, but received {kind}."),
            "{source}"
        );
    }
}

#[test]
fn the_value_of_a_failure_is_empty_when_it_has_no_text() {
    let report = run(
        "try {\n    x := Abs({})\n} catch Any as e {\n    MsgBox \"[\" e.Extra \"]\"\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
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
        "class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\nx := ObjGetBase(b)",
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
        "class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\n\
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
        "class Box {\n  x := 1\n}\nb := Box()\n\
         names := ObjOwnProps(b)\ncount := ObjOwnPropCount(b)\nhas := ObjHasOwnProp(b, \"x\")\n\
         kind := Type(names)\n\
         first := \"\"\nnames.Call(&first)\n\
         after := \"\"\nnames.Call(&after)",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-31: the own properties
    // of v2 are an enumerator, and it gives the name when the caller
    // wants one value.
    assert_eq!(value_of(&report, "kind"), Value::Text("Enumerator".into()));
    assert_eq!(value_of(&report, "first"), Value::Text("x".into()));
    assert_eq!(value_of(&report, "after"), Value::Text("".into()));
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(int_of(&report, "has"), 1);
}

#[test]
fn the_state_of_a_map_is_not_a_property_of_it() {
    let report = run("m := Map()\nnames := ObjOwnProps(m)\nkind := Type(names)\ncount := ObjOwnPropCount(m)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "kind"), Value::Text("Enumerator".into()));
    assert_eq!(int_of(&report, "count"), 0);
}

#[test]
fn the_name_that_counts_the_references_is_not_one_of_a_script() {
    let report = run("class Box {\n}\nb := Box()\na := ObjAddRef(b)");
    assert_eq!(failure_of(&report), "Parameter #1 of ObjAddRef is invalid.");
}

#[test]
fn a_plain_value_is_not_a_descriptor() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the second
    // parameter of `DefineProp` is a descriptor, and a value that is not
    // one is refused with the reason.
    let report = run("o := {}\no.DefineProp(\"x\", 1)");
    assert_eq!(failure_of(&report), "Parameter #2 of Object.Prototype.DefineProp is invalid.");
}

#[test]
fn an_object_without_a_member_is_not_a_descriptor() {
    let report = run("o := {}\no.DefineProp(\"x\", {y: 1})");
    assert_eq!(failure_of(&report), "Parameter #2 of Object.Prototype.DefineProp is invalid.");
}

#[test]
fn a_descriptor_of_a_value_holds_the_value() {
    let report = run("o := {}\no.DefineProp(\"x\", {Value: 5})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(5));
}

#[test]
fn the_answer_of_a_defined_property_is_the_object() {
    let report = run("o := {}\nr := o.DefineProp(\"x\", {Value: 5})\na := r == o");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_descriptor_of_a_getter_is_asked_for_the_value() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => 42})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(42));
}

#[test]
fn the_getter_of_a_descriptor_is_asked_with_the_object() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => Type(f)})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Object".into()));
}

#[test]
fn a_descriptor_of_a_setter_is_asked_for_the_write() {
    let report = run("o := {}\no.DefineProp(\"x\", {Set: (f, v) => 0})\no.x := 3");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_property_without_a_setter_is_read_only() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => 42})\no.x := 3");
    assert_eq!(failure_of(&report), "Property is read-only.");
}

#[test]
fn a_descriptor_of_a_call_is_called() {
    let report = run("o := {}\no.DefineProp(\"x\", {Call: (f, a) => a * 2})\na := o.x(5)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(10));
}

#[test]
fn the_descriptor_that_comes_back_is_an_object() {
    let report = run(
        "o := {}\no.DefineProp(\"x\", {Value: 5})\n\
         a := Type(o.GetOwnPropDesc(\"x\")) . \" \" . o.GetOwnPropDesc(\"x\").Value",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Object 5".into()));
}

#[test]
fn the_descriptor_of_a_property_that_is_not_there_is_refused() {
    let report = run("o := {}\na := o.GetOwnPropDesc(\"nope\")");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Object\" has no property named \"nope\"."
    );
}

#[test]
fn the_take_away_of_a_property_answers_its_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `DeleteProp`
    // answers the value of the property that is taken away.
    let report = run("o := {}\no.DefineProp(\"x\", {Value: 7})\na := o.DeleteProp(\"x\")\nb := o.DeleteProp(\"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(7));
    assert_eq!(value_of(&report, "b"), Value::Text("".into()));
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
    let report = run("class Box {\n}\nb := Box()\na := ObjBindMethod(b, \"x\")");
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
        "\ne := StrCompare(\"A\", \"a\", true)",
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
fn the_case_sense_wants_a_boolean() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the sense of the case
    // is a boolean, and a text is not a value that the parameter can take.
    for (source, message) in [
        ("a := StrCompare(\"a\", \"A\", \"On\")", "Parameter #3 of StrCompare is invalid."),
        ("a := StrReplace(\"a\", \"a\", \"b\", \"On\")", "Parameter #4 of StrReplace is invalid."),
        ("a := InStr(\"a\", \"a\", \"On\")", "Parameter #3 of InStr is invalid."),
        ("a := StrCompare(\"a\", \"A\", 3)", "Parameter #3 of StrCompare is invalid."),
        ("a := StrReplace(\"a\", \"a\", \"b\", 3)", "Parameter #4 of StrReplace is invalid."),
        ("a := InStr(\"a\", \"a\", 1.0)", "Parameter #3 of InStr is invalid."),
    ] {
        let report = run(source);
        assert_eq!(failure_of(&report), message, "{source}");
    }
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
        "Too few parameters passed to function.\n     Specifically: Format"
    );
}

#[test]
fn in_str_says_where_the_needle_is() {
    let report = run(concat!(
        "a := InStr(\"hello\", \"ll\")",
        "\nb := InStr(\"hello\", \"z\")",
        "\nc := InStr(\"hello\", \"l\", 1)",
        "\nd := InStr(\"hello\", \"l\", true, 4)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    // Measured on the binary of v2.0.19 on 2026-10-31: the third parameter is
    // the sense of the case and the fourth is the place where the search
    // begins.
    assert_eq!(value_of(&report, "c"), Value::Int(3));
    assert_eq!(value_of(&report, "d"), Value::Int(4));
}

#[test]
fn in_str_counts_the_occurrences() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the fifth parameter is
    // the occurrence that the caller wants, and zero is not one.
    let report = run(concat!(
        "a := InStr(\"abcabc\", \"b\", true, 1, 1)",
        "\nb := InStr(\"abcabc\", \"b\", true, 1, 2)",
        "\nc := InStr(\"abcabc\", \"b\", true, 1, 3)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(2));
    assert_eq!(value_of(&report, "b"), Value::Int(5));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
}

#[test]
fn the_occurrence_of_the_needle_wants_a_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let report = run("a := InStr(\"abcabc\", \"b\", true, 1, 0)");
    assert_eq!(failure_of(&report), "Parameter #5 of InStr is invalid.");
    let report = run("a := InStr(\"abcabc\", \"b\", true, 1, \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of InStr requires a Number, but received a String."
    );
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
        "Parameter #5 of StrReplace requires a variable reference, but received an Integer."
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
fn str_replace_with_an_empty_needle_changes_nothing() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a needle that is
    // empty changes nothing, and the text comes back as it is.
    let report = run("a := StrReplace(\"hello\", \"\", \"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("hello".into()));
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
        "Too few parameters passed to function.\n     Specifically: SubStr"
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
        "class Box {\n}\nb := Box()\n\
         a := Type(b)\nc := Type(Box)\nd := Type(Map())\ne := Type(Box.Prototype)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Box".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("Map".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("Prototype".into()));
}

#[test]
fn a_reference_is_a_value_of_its_own_kind() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `Type` of a
    // reference names its kind, and the predicates answer about it.
    let report = run(concat!(
        "y := 1\n",
        "a := Type(&y)\n",
        "b := IsObject(&y)\n",
        "c := IsInteger(&y)\n",
        "d := IsNumber(&y)\n",
        "e := IsFloat(&y)\n",
        "f := IsSetRef(&y)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("VarRef".into()));
    // A reference is an object of its own kind.
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(0));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
}

#[test]
fn the_questions_about_a_value_answer_a_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is
    // not an object has no method and no property, and the answer is
    // zero and not a refusal. The one property that every value has is
    // its class.
    let report = run(concat!(
        "y := 1\n",
        "a := HasMethod(&y)\n",
        "b := HasMethod(1)\n",
        "c := HasProp(&y, \"nothing\")\n",
        "d := HasProp(&y, \"__Class\")\n",
        "e := HasProp([1, 2], \"Length\")\n",
        "f := HasMethod([1, 2], \"Push\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(0));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(1));
    // The array answers about its own properties and methods.
    assert_eq!(value_of(&report, "e"), Value::Int(1));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
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
        "Parameter #4 of RegExReplace requires a variable reference, but received an Integer."
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
    for (name, source, reason) in [
        (
            "Critical",
            "Critical",
            "this runtime runs one script in one thread, and it cannot make it critical.",
        ),
        (
            "Thread",
            // v2 refuses a `Thread` that carries no argument, and the
            // refusal comes before the reason of this runtime: measured on
            // the binary of v2.0.19 on 2026-10-31.
            "Thread(1)",
            "this runtime runs one script in one thread, and it cannot start another.",
        ),
        (
            "Pause",
            "Pause",
            "this runtime cannot wait for a key to continue: the script ends when it ends.",
        ),
        (
            "Persistent",
            "Persistent",
            "this runtime is a script that ends when it ends, and it cannot be kept alive.",
        ),
        (
            "Reload",
            "Reload",
            "this runtime runs one script once, and it cannot run itself again.",
        ),
        ("Suspend", "Suspend", "this runtime cannot stop itself: it would never end."),
        (
            "ListLines",
            "ListLines",
            "the lines are listed in a window of v2, and this runtime has no window to show them in.",
        ),
        (
            "ListVars",
            "ListVars",
            "the variables are listed in a window of v2, and this runtime has no window to show them in.",
        ),
    ] {
        let report = run(source);
        assert_eq!(failure_of(&report), reason, "{name}");
        // The reason of the name is the one that the surface says.
        assert_eq!(ahk::builtins::refusal_of(name), Some(reason), "{name}");
    }
}

#[test]
fn a_command_says_how_many_parameters_it_wants() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a command whose
    // parameters are all needed is refused at the load, and the refusal
    // is the one of the count.
    let report = run("PixelGetColor()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: PixelGetColor"
    );
}

#[test]
fn a_command_says_when_there_are_too_many_parameters() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `MsgBox` takes
    // three parameters, and a fourth is refused.
    let report = run("MsgBox(\"a\", \"b\", \"c\", \"d\")");
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: MsgBox"
    );
}

#[test]
fn a_parameter_that_may_not_be_left_out_is_named() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `FileGetAttrib`
    // declares that its parameter may be left out, and the command needs
    // it. The refusal names the parameter and not the count.
    let report = run("FileGetAttrib()");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of FileGetAttrib must not be omitted in this case."
    );
}

#[test]
fn the_drag_names_the_parameter_it_needs() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the drag needs the
    // place where it ends, and it names the first one that is missing.
    let report = run("MouseClickDrag()");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of MouseClickDrag must not be omitted in this case."
    );
    let report = run("MouseClickDrag(\"Left\", 1, 2, 3)");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of MouseClickDrag must not be omitted in this case."
    );
}

#[test]
fn the_failure_of_an_omitted_parameter_carries_no_name() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the failure of a
    // parameter that may not be left out has an empty `What` and an empty
    // `Extra`: the message carries the whole of it.
    let report = run(concat!(
        "try\n",
        "  FileGetAttrib()\n",
        "catch as e\n",
        "  MsgBox e.What \"|\" e.Extra \"|\" e.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["||Parameter #1 of FileGetAttrib must not be omitted in this case."]
    );
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
        "Parameter #1 of OnExit requires an Object, but received a String."
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
    assert_eq!(failure_of(&report), "Too few parameters passed to function.\n     Specifically: OnExit");
}

#[test]
fn the_order_of_the_callbacks_is_one_minus_one_or_zero() {
    let report = run("OnExit Bye, 5\nBye(Reason, Code) {\n}\n");
    assert_eq!(failure_of(&report), "Parameter #2 of OnExit is invalid.\n     Specifically: 5");
}

#[test]
fn an_order_that_is_not_a_number_refuses_with_the_reason_of_v2() {
    let report = run("OnExit Bye, \"x\"\nBye(Reason, Code) {\n}\n");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of OnExit requires a Number, but received a String."
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
fn the_base_of_an_object_of_a_class_type_of_the_runtime_cannot_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of the
    // class is there, and the base of the object cannot be replaced.
    for source in [
        "m := Map()\nObjSetBase(m, Object())",
        "a := [1, 2]\nObjSetBase(a, Object())",
        "b := Buffer(4)\nObjSetBase(b, Object())",
        "f := Abs\nObjSetBase(f, Object())",
        "m := Map()\nm.Base := Object()",
    ] {
        let report = run(&format!(
            "try {{\n    {source}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}\n"
        ));
        assert!(report.ok, "{:?}", report.error);
        assert_eq!(report.output, vec!["Invalid base. | ObjSetBase | "], "the base of {source}");
    }
    // The base of a plain object, of an error and of an instance of a class
    // of the script can be written.
    for source in [
        "o := Object()\nObjSetBase(o, Object())",
        "e := Error(\"x\")\nObjSetBase(e, Object())",
        "class C {\n}\no := C()\nObjSetBase(o, Object())",
    ] {
        let report = run(source);
        assert!(report.ok, "{:?}", report.error);
    }
    // The value that is not an object goes in the `Extra` of the refusal.
    let report = run(
        "try {\n    ObjSetBase(Map(), 1)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Expected an Object but got an Integer. | ObjSetBase | 1"]
    );
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

#[test]
fn the_file_append_writes_to_the_console_when_the_name_is_the_star() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the star is the
    // standard output of the machine, and the answer of the command is
    // an empty text.
    let report = run("x := FileAppend(\"hello\", \"*\")\nMsgBox \"[%x%]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello", "[]"]);
}

#[test]
fn the_file_open_of_the_star_opens_the_console() {
    // The writes go to the standard output, the reads answer an empty
    // text, and the length of the file is the number of the characters
    // that were written to it.
    let report = run(
        "f := FileOpen(\"*\", \"w\")\nwritten := f.Write(\"hello\")\nMsgBox \"%written% %f.Length% %f.Pos% %f.AtEOF%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello", "5 5 5 1"]);
}

#[test]
fn the_read_of_the_console_answers_an_empty_text() {
    let report = run("f := FileOpen(\"*\", \"r\")\ntext := f.Read()\nMsgBox \"[%text%]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

// -- the dialogs ----------------------------------------------------------------

#[test]
fn the_input_box_answers_an_object_with_the_text_and_the_result() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer of
    // the InputBox is an object, and it carries the text of the user in
    // its Value and the way the dialog ended in its Result.
    let report = run(concat!(
        "ib := InputBox(\"the prompt\", \"the title\", \"T10\", \"the default\")\n",
        "a := ib.Value\n",
        "b := ib.Result\n",
        "c := ib.__Class\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("the answer".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("OK".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Object".into()));
}

#[test]
fn the_input_box_carries_what_it_was_asked() {
    let mut desktop = Fake::default();
    let report = run_on("InputBox(\"the prompt\", \"the title\", \"T10\", \"the default\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("input-box".to_string(), "the prompt|the title|the default|Some(10.0)".to_string())]
    );
}

#[test]
fn the_input_box_of_too_many_arguments_fails_the_way_v2_does() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the InputBox
    // takes four parameters, and the fifth is too many.
    let report = run("InputBox(\"a\", \"b\", \"c\", \"d\", \"e\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: InputBox"
    );
}

#[test]
fn an_option_of_the_input_box_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.29 on 2026-10-29: the message is
    // "Invalid option.", the class of the failure is a ValueError, and
    // the option that was not known is in the Extra.
    let report = run(
        "try {\n    InputBox(\"a\", \"b\", \"z9\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid option. | ValueError | InputBox | z9"]);
}

#[test]
fn the_file_select_answers_the_path_of_what_was_chosen() {
    let report = run("a := FileSelect(\"D\", \"/tmp\", \"the title\")\nMsgBox \"%a%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/tmp/chosen.txt"]);
}

#[test]
fn the_file_select_of_many_files_answers_an_array() {
    let report = run("a := FileSelect(\"M\")\nMsgBox a.Length \" \" a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 /tmp/one.txt"]);
}

#[test]
fn the_file_select_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("FileSelect(\"a\", \"b\", \"c\", \"d\", \"e\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: FileSelect"
    );
}

#[test]
fn an_option_of_the_file_select_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the message is
    // "Parameter #1 of FileSelect is invalid." and the options are in
    // the Extra of the failure.
    let report = run(
        "try {\n    FileSelect(\"Q\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of FileSelect is invalid. | ValueError | FileSelect | Q"]
    );
}

#[test]
fn the_filter_of_the_file_select_is_not_there_when_a_folder_is_asked_for() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the filter of
    // the FileSelect is refused when the user chooses a folder.
    let report = run(
        "try {\n    FileSelect(\"D\", \"\", \"\", \"Text (*.txt)\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Parameter #4 of FileSelect is invalid. | Text (*.txt)"]);
}

#[test]
fn the_dir_select_answers_the_path_of_the_folder() {
    let report = run("a := DirSelect(\"/tmp\", 3, \"the prompt\")\nMsgBox \"%a%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/tmp/chosen"]);
}

#[test]
fn the_dir_select_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("DirSelect(\"a\", \"b\", \"c\", \"d\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: DirSelect"
    );
}

#[test]
fn the_options_of_the_dir_select_are_a_number() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the options of
    // the DirSelect are a number, and a text that is not one is
    // refused.
    let report = run(
        "try {\n    DirSelect(\"a\", \"b\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of DirSelect requires a Number, but received a String. | TypeError | DirSelect | b"]
    );
}



#[test]
fn the_load_picture_answers_the_number_of_the_picture() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer is
    // the number that identifies the picture, and the type of the
    // handle is written in the third parameter.
    let mut desktop = Fake::default();
    let report = run_on(
        "t := 9\nx := LoadPicture(\"the.png\", \"w10\", &t)\nMsgBox x \" | \" t",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7 | 0"]);
    assert_eq!(
        desktop.calls,
        vec![("load-picture".to_string(), "the.png".to_string())]
    );
}

#[test]
fn the_load_picture_without_arguments_fails_the_way_v2_does() {
    let report = run("x := LoadPicture()");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: LoadPicture"
    );
}

#[test]
fn the_load_picture_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("x := LoadPicture(\"a\", \"b\", &t, \"d\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: LoadPicture"
    );
}

#[test]
fn the_image_search_answers_one_and_writes_the_place() {
    let mut desktop = Fake::default();
    let report = run_on(
        "x := 0\ny := 0\nz := ImageSearch(&x, &y, 1, 2, 3, 4, \"the.png\")\nMsgBox z \" | \" x \" | \" y",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 | 10 | 20"]);
    assert_eq!(
        desktop.calls,
        vec![("image-search".to_string(), "1,2,3,4".to_string())]
    );
}

#[test]
fn the_image_search_wants_two_variables() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the first two
    // parameters want a variable.
    let report = run(
        "z := ImageSearch(0, 0, 0, 0, 10, 10, \"the.png\")\n",
    );
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ImageSearch requires a VarRef, but received an Integer."
    );
}

#[test]
fn the_image_search_of_a_picture_that_cannot_be_loaded_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the message is
    // "Parameter #7 of ImageSearch is invalid." and the name of the
    // picture is in the Extra.
    let report = run(
        "try {\n    x := 0\n    y := 0\n    ImageSearch(&x, &y, 0, 0, 10, 10, \"\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #7 of ImageSearch is invalid. | ValueError | ImageSearch"]
    );
}

#[test]
fn the_image_search_of_too_few_arguments_fails_the_way_v2_does() {
    let report = run("x := 0\ny := 0\nz := ImageSearch(&x, &y, 0, 0, 10, 10)");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: ImageSearch"
    );
}

#[test]
fn the_image_search_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("x := 0\ny := 0\nz := ImageSearch(&x, &y, 0, 0, 10, 10, \"a\", \"b\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: ImageSearch"
    );
}

#[test]
fn a_desktop_that_does_not_read_the_images_says_so() {
    let mut desktop = Blind;
    let statements = parse("x := LoadPicture(\"the.png\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the picture is read by the desktop, and this desktop does not read the images of the machine.")
    );
}

#[test]
fn the_download_writes_the_file_and_answers_nothing() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer of the
    // command is an empty text, and the file is written where the script
    // said.
    let mut desktop = Fake::default();
    let report = run_on(
        "x := Download(\"https://example.com/file.txt\", \"/tmp/file.txt\")\nMsgBox \"[\" x \"]\"",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
    assert_eq!(
        desktop.calls,
        vec![("download".to_string(), "https://example.com/file.txt|/tmp/file.txt".to_string())]
    );
}

#[test]
fn the_download_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("Download(\"a\", \"b\", \"c\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: Download"
    );
}

#[test]
fn the_download_without_arguments_fails_the_way_v2_does() {
    let report = run("Download()");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: Download"
    );
}

#[test]
fn a_desktop_that_does_not_reach_the_network_says_so() {
    let mut desktop = Blind;
    let statements = parse("Download(\"https://example.com/file.txt\", \"/tmp/file.txt\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the download is made by the desktop, and this desktop does not reach the network.")
    );
}

// -- the names that the tree does not carry ------------------------------------------------

#[test]
fn a_name_that_the_tree_does_not_carry_says_the_kind_of_the_parameter_first() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect says
    // the kind that the parameter wants before it says anything else.
    let report = run("x := 1\na := StrPut(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrPut requires a String, but received a VarRef."
    );
}

#[test]
fn a_name_that_the_tree_does_not_carry_wants_a_number_where_the_dialect_wants_one() {
    let report = run("x := 1\na := Shutdown(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Shutdown requires a Number, but received a VarRef."
    );
}

#[test]
fn a_value_of_the_wrong_kind_is_refused_before_the_reason_of_the_machine() {
    let report = run("a := StrPut([])");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrPut requires a String, but received an Array."
    );
}

#[test]
fn the_check_that_the_body_of_a_function_makes_says_its_own_words() {
    // The `StrPtr` checks the kind of its argument in its body, and the
    // words are not the ones of a parameter that declares its kind.
    let report = run("x := 1\na := StrPtr(&x)");
    assert_eq!(failure_of(&report), "Expected a String but got a VarRef.");
}

#[test]
fn a_parameter_that_wants_a_reference_says_it() {
    let report = run("a := CaretGetPos(5, 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of CaretGetPos requires a variable reference, but received an Integer."
    );
}

#[test]
fn an_empty_text_is_named_in_the_message_of_the_check() {
    let report = run("a := ObjOwnProps(\"\")");
    assert_eq!(failure_of(&report), "Expected an Object but got an empty string.");
}

#[test]
fn a_name_that_the_tree_does_not_carry_says_the_reason_of_the_machine_when_the_kind_fits() {
    let report = run("a := StrPut(\"x\", \"y\")");
    assert_eq!(
        failure_of(&report),
        "this is a name of Windows, and this desktop is not Windows."
    );
}

#[test]
fn the_kind_of_a_value_comes_before_the_parameter_that_may_not_be_left_out() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect
    // converts the arguments before the body of the function runs.
    let report = run("a := ListViewGetContent([])");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ListViewGetContent requires a String, but received an Array."
    );
}

#[test]
fn the_parameter_that_may_not_be_left_out_is_a_value_that_is_not_valid() {
    let report = run(
        "try {\n    a := ListViewGetContent(\"x\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of ListViewGetContent must not be omitted in this case. | ValueError"]
    );
}

#[test]
fn a_desktop_that_cannot_ask_the_user_says_so() {
    // The question is the machine's, and a machine that has no window
    // for it refuses with the reason.
    let mut desktop = Blind;
    let statements = parse("InputBox(\"the prompt\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this desktop cannot ask the user for a text")
    );
}

#[test]
fn the_name_of_a_file_that_is_empty_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the commands that
    // act on the name of a file refuse it when it is empty, and the value
    // that failed goes in the `Extra` as "empty string".
    let cases = [
        ("FileDelete \"\"", 1, "FileDelete"),
        ("FileRead \"\"", 1, "FileRead"),
        ("FileMove \"\", \"b.txt\"", 1, "FileMove"),
        ("FileMove \"a.txt\", \"\"", 2, "FileMove"),
        ("FileCopy \"\", \"b.txt\"", 1, "FileCopy"),
        ("FileCopy \"a.txt\", \"\"", 2, "FileCopy"),
        ("FileGetSize \"\"", 1, "FileGetSize"),
        ("FileGetTime \"\"", 1, "FileGetTime"),
        ("FileGetAttrib \"\"", 1, "FileGetAttrib"),
        ("DirCopy \"\", \"b\"", 1, "DirCopy"),
        ("DirCopy \"a\", \"\"", 2, "DirCopy"),
        ("DirMove \"\", \"b\"", 1, "DirMove"),
        ("DirMove \"a\", \"\"", 2, "DirMove"),
        ("FileSetTime \"a.txt\", \"\", \"b.txt\"", 2, "FileSetTime"),
        ("FileAppend \"a\", \"\"", 2, "FileAppend"),
    ];
    for (call, number, name) in cases {
        let report = run(&format!(
            "try {{\n    {call}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}"
        ));
        assert!(report.ok, "{call}: {:?}", report.error);
        assert_eq!(
            report.output,
            vec![format!("Parameter #{number} of {name} is invalid. | {name} | empty string")],
            "{call}"
        );
    }
}

#[test]
fn the_name_of_the_file_of_the_attributes_is_the_second_one_and_the_message_names_the_first() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of the file
    // is the second parameter of the `FileSetAttrib`, and the message of v2
    // names the first one, which is the attributes. The value of the first
    // one goes in the `Extra`.
    let report = run(
        "try {\n    FileSetAttrib(\"R\", \"\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of FileSetAttrib is invalid. | FileSetAttrib | R"]
    );
}

#[test]
fn the_file_append_without_the_name_of_the_file_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of the file
    // of a `FileAppend` may be left out only inside the body of a `Loop
    // Read` that has a file of its own.
    let report = run(
        "try {\n    FileAppend(\"a\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileAppend must not be omitted in this case. | FileAppend | "]
    );
}

#[test]
fn the_file_append_without_the_name_writes_to_the_file_of_the_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the text of a
    // `FileAppend` that has no name of its own goes to the file of the
    // `Loop Read` that runs.
    let mut desktop = Fake::default();
    let report = run_on(
        "Loop Read \"in.txt\", \"out.txt\" {\n    FileAppend \"x\"\n}",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![
            ("append-file".to_string(), "out.txt x ".to_string()),
            ("append-file".to_string(), "out.txt x ".to_string()),
        ]
    );
}

#[test]
fn the_file_append_without_the_name_is_refused_in_a_loop_without_a_file() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the loop that reads
    // has no file of its own, and the name of the file may not be left out.
    let report = run(
        "try {\n    Loop Read \"in.txt\" {\n        FileAppend \"x\"\n    }\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileAppend must not be omitted in this case. | FileAppend"]
    );
}
