//! The built-ins, against a fake desktop.
//!
//! The cases are the language ones: what a built-in answers, and what it
//! says when it is given the wrong thing. The wording of the failures is
//! the wording of AutoHotkey v2, because that is what a script sees.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Drive, FileAnswer, FileOptions, Files, Hotkeys, Image, ImageOptions,
    ImageSearch, InputAnswer, InputOptions, Keyboard, Machine, Monitor, Native, Network,
    PictureOptions, Pictures, Pointer, Processes, Screen, Sound, Timers, WindowInfo, Windows,
};
use ahk::dll::{NativeCall, NativeFailure, NativeResult, NativeType, NativeValue, encode_number};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// The turn of the cases that change the mode of the titles.
///
/// The mode is a setting of the crate, and the cases of one file run in
/// threads: a case that reads the mode while another writes it would see the
/// mode of the other one. The cases that change it take this turn.
static THE_TITLE_MODE_TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
    /// Whether the caps lock of the fake is on.
    lock_on: bool,
    /// The turns of the lock keys that the script asked for.
    locks: Vec<(String, bool)>,
    /// The files the script recycled, in the order it asked for them.
    recycled: Vec<String>,
    /// Whether the trash of the fake was emptied.
    trash_emptied: bool,
    /// The texts the script sent to the debugger.
    debugged: Vec<String>,
}

impl Desktop for Fake {}
impl Pictures for Fake {
    fn load_picture(
        &mut self,
        filename: &str,
        _options: &PictureOptions,
    ) -> Result<Option<u32>, String> {
        self.calls
            .push(("load-picture".into(), filename.to_string()));
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
        self.calls
            .push(("image-search".into(), format!("{x1},{y1},{x2},{y2}")));
        match image {
            Image::File(name) if name.is_empty() => Ok(ImageSearch::NotLoadable),
            _ => Ok(ImageSearch::Found(10, 20)),
        }
    }
}
impl Network for Fake {
    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        self.calls
            .push(("download".into(), format!("{url}|{file}")));
        Ok(file.to_string())
    }
}

impl Sound for Fake {
    fn sound_volume(&mut self, device: &str) -> Result<f64, String> {
        self.calls.push(("volume".into(), device.into()));
        Ok(50.0)
    }
    fn sound_set_volume(&mut self, device: &str, volume: f64) -> Result<(), String> {
        self.calls
            .push(("set-volume".into(), format!("{device} {volume}")));
        Ok(())
    }
    fn sound_muted(&mut self, device: &str) -> Result<bool, String> {
        self.calls.push(("mute".into(), device.into()));
        Ok(false)
    }
    fn sound_set_muted(&mut self, device: &str, muted: bool) -> Result<(), String> {
        self.calls
            .push(("set-mute".into(), format!("{device} {muted}")));
        Ok(())
    }
    fn sound_play(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("play".into(), path.into()));
        Ok(format!("played {path}"))
    }
    fn sound_beep(&mut self, frequency: f64, duration: u64) -> Result<String, String> {
        self.calls
            .push(("beep".into(), format!("{frequency} {duration}")));
        Ok("beeped".to_string())
    }
}
impl Machine for Fake {
    // The addresses of the fake: the shape the binary answers, measured on
    // 2026-11-09 -- the texts of the addresses of the machine, and not the
    // address of the loopback.
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        Ok(vec!["172.26.48.1".into(), "192.168.0.45".into()])
    }
    fn output_debug(&mut self, text: &str) -> Result<(), String> {
        self.debugged.push(text.to_string());
        Ok(())
    }
    // The drive of the fake: the root of the machine is a fixed volume of the
    // format ext4 with a serial and two sizes, and every other name is a
    // path that is not there. The shapes are the ones measured on the binary
    // of v2.0.19 on 2026-11-09.
    fn drive(&mut self, path: &str) -> Result<Option<Drive>, String> {
        if path == "/" {
            return Ok(Some(Drive {
                drive_type: "Fixed".into(),
                filesystem: "ext4".into(),
                label: String::new(),
                serial: 42,
                free_megabytes: 100,
                capacity_megabytes: 200,
                status: "Ready".into(),
            }));
        }
        Err("(2) No such file or directory".to_string())
    }
    fn drive_type(&mut self, path: &str) -> Result<Option<String>, String> {
        Ok((path == "/").then(|| "Fixed".to_string()))
    }
    fn drive_status(&mut self, path: &str) -> Result<Option<String>, String> {
        Ok((path == "/").then(|| "Ready".to_string()))
    }
}
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
        Ok(InputAnswer {
            value: "the answer".into(),
            result: "OK".into(),
        })
    }

    fn file_select(
        &mut self,
        options: &FileOptions,
        root: &str,
        title: &str,
        filter: &str,
    ) -> Result<FileAnswer, String> {
        self.calls
            .push(("file-select".into(), format!("{root}|{title}|{filter}")));
        if options.multiple {
            Ok(FileAnswer::Many(vec![
                "/tmp/one.txt".into(),
                "/tmp/two.txt".into(),
            ]))
        } else {
            Ok(FileAnswer::One("/tmp/chosen.txt".into()))
        }
    }

    fn dir_select(&mut self, starting: &str, options: i64, prompt: &str) -> Result<String, String> {
        self.calls.push((
            "dir-select".into(),
            format!("{starting}|{options}|{prompt}"),
        ));
        Ok("/tmp/chosen".into())
    }
}

impl Windows for Fake {
    // The windows of the fake: three of them, and the first one is the active
    // one. The ids and the class names are the shapes measured on the binary
    // of v2.0.19 on 2026-11-09: an id of the runtime, a title, and the class
    // of the window -- the application id of the compositor on the box.
    fn windows(&mut self) -> Result<Vec<WindowInfo>, String> {
        Ok(vec![
            WindowInfo {
                id: 101,
                title: "Steam".into(),
                class: "steam_app".into(),
                active: true,
            },
            WindowInfo {
                id: 102,
                title: "Firefox".into(),
                class: "firefox".into(),
                active: false,
            },
            WindowInfo {
                id: 103,
                title: "Download".into(),
                class: "chromium".into(),
                active: false,
            },
        ])
    }
}

impl Files for Fake {
    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        self.calls.push(("working-dir".into(), directory.into()));
        Ok(format!("the working directory is now {directory}"))
    }
    fn attributes_of_the_pattern(
        &mut self,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        self.calls.push((format!("pattern {mode}"), pattern.into()));
        if mode.contains('D') {
            return Ok(if pattern == "/tmp" {
                vec!["D".to_string()]
            } else {
                Vec::new()
            });
        }
        Ok(if pattern.ends_with(".txt") {
            vec!["A".to_string()]
        } else {
            Vec::new()
        })
    }
    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        Ok(if path.ends_with(".txt") {
            vec!["A".to_string()]
        } else {
            vec!["D".to_string()]
        })
    }
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Ok(vec!["one".to_string(), "two".to_string()])
    }
    fn read_the_bytes_of_a_file(&mut self, path: &str) -> Result<Vec<u8>, String> {
        self.calls.push(("bytes-of-file".into(), path.into()));
        Ok(b"one\r\ntwo\r\n".to_vec())
    }
    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("delete-file".into(), path.into()));
        Ok(String::new())
    }
    fn create_dir(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("create-dir".into(), path.into()));
        Ok(String::new())
    }
    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        self.calls
            .push(("copy-file".into(), format!("{from}|{to}|{overwrite}")));
        // The machine says "Failed" when the file that is copied is not
        // there, and the number of the files that failed goes in the
        // `Extra` of the failure.
        if from.contains("nope") {
            return Err("Failed".to_string());
        }
        Ok(format!("copied the file {from} to {to}"))
    }
    fn move_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        self.calls
            .push(("move-file".into(), format!("{from}|{to}|{overwrite}")));
        if from.contains("nope") {
            return Err("Failed".to_string());
        }
        Ok(format!("moved the file {from} to {to}"))
    }
    fn working_directory(&mut self) -> Result<String, String> {
        Ok("/tmp".to_string())
    }
    // The files of a pattern of the fake: the two files of the pattern, as
    // the full paths the desktop answers with. The pattern of v2 is the one
    // of the last part of the path, and the mode "D" is the folders.
    fn list_files(
        &mut self,
        folder: &str,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        self.calls
            .push(("list-files".into(), format!("{folder}|{pattern}|{mode}")));
        if mode.contains('D') {
            return Ok(Vec::new());
        }
        let (where_it_is, name) = match pattern.rfind('/') {
            Some(at) => (&pattern[..=at], &pattern[at + 1..]),
            None => ("", pattern),
        };
        let where_it_is = if where_it_is.is_empty() {
            format!("{folder}/")
        } else {
            where_it_is.to_string()
        };
        if !name.contains('*') && !name.contains('?') {
            return Ok(Vec::new());
        }
        let extension = name.trim_start_matches('*');
        Ok(["a", "b"]
            .iter()
            .map(|stem| format!("{where_it_is}{stem}{extension}"))
            .collect())
    }
    fn append_file(&mut self, path: &str, bytes: &[u8], mark: &[u8]) -> Result<String, String> {
        self.calls.push((
            "append-file".into(),
            format!(
                "{path} {} {}",
                String::from_utf8_lossy(bytes),
                String::from_utf8_lossy(mark)
            ),
        ));
        Ok(bytes.len().to_string())
    }
    // The trash of the fake: the files the script recycled are recorded, and
    // a path that is not there is the failure "Failed", which is what the
    // binary answers: measured on it on 2026-11-09.
    fn recycle(&mut self, pattern: &str) -> Result<(), String> {
        if pattern.contains("missing") {
            return Err("Failed".to_string());
        }
        self.recycled.push(pattern.to_string());
        Ok(())
    }
    fn empty_trash(&mut self, _drive: Option<&str>) -> Result<(), String> {
        self.trash_emptied = true;
        Ok(())
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
    // The caps lock of the fake is a switch, and the turns of the keys that
    // the script asked for are recorded: the state of a lock key is the state
    // of the LED of the keyboard, and the shape is the one measured on the
    // binary of v2.0.19 on 2026-11-09.
    fn lock_is_on(&mut self, _lock: &str) -> Result<bool, String> {
        Ok(self.lock_on)
    }
    fn set_lock(&mut self, lock: &str, on: bool) -> Result<(), String> {
        self.locks.push((lock.to_string(), on));
        self.lock_on = on;
        Ok(())
    }
}

impl Pointer for Fake {
    fn click(&mut self, x: i64, y: i64, _button: &str, _count: u32) -> Result<String, String> {
        self.calls.push(("click".into(), format!("{x} {y}")));
        Ok(format!("clicked ({x}, {y})"))
    }
    fn mouse_down(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        self.calls
            .push(("down".into(), format!("{x:?} {y:?} {button}")));
        Ok("down".to_string())
    }
    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls
            .push(("up".into(), format!("{x:?} {y:?} {button}")));
        Ok("up".to_string())
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("move".into(), format!("{x} {y}")));
        Ok("moved".to_string())
    }
}

impl Screen for Fake {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        filename: &str,
    ) -> Result<String, String> {
        Ok(format!("/tmp/{filename}"))
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
    // The monitors of the fake: two of them, side by side on a screen of
    // 3840 by 2160, and the second one is the primary one. The shapes are
    // the ones measured on the binary of v2.0.19 on 2026-11-09: the right
    // and the bottom are one past the last column and the last row, and a
    // monitor has a name.
    fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        Ok(vec![
            Monitor {
                name: "\\\\.\\DISPLAY1".into(),
                left: 0,
                top: 0,
                right: 1920,
                bottom: 2160,
            },
            Monitor {
                name: "\\\\.\\DISPLAY2".into(),
                left: 1920,
                top: 0,
                right: 3840,
                bottom: 2160,
            },
        ])
    }
    fn primary_monitor(&mut self) -> Result<usize, String> {
        Ok(2)
    }
    fn monitor_work_area(
        &mut self,
        monitor: usize,
    ) -> Result<Option<(i64, i64, i64, i64)>, String> {
        // The bar of the taskbar is at the bottom of the primary monitor.
        Ok(match monitor {
            1 => Some((0, 0, 1920, 2160)),
            2 => Some((1920, 0, 3840, 2112)),
            _ => None,
        })
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, format!("ran {command}\n"), String::new()))
    }
    fn env_get(&mut self, name: &str) -> Result<Option<String>, String> {
        Ok(Some(format!("the value of {name}")))
    }
    fn env_set(&mut self, _name: &str, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }
    // The processes of the fake: the pid 4242 is "bash", the pid 4243 is
    // "stays" -- a process that is still there at every timeout -- and
    // anything else is not there. The process of the script itself is 99.
    //
    // The answers are the shapes measured on the binary of v2.0.19 on
    // 2026-11-09: a process that is not there is zero for the questions that
    // answer a pid, and a failure the caller turns into "Target process not
    // found." for the two that answer a text.
    fn process_exist(&mut self, which: Option<&str>) -> Result<u32, String> {
        Ok(match which {
            None => 99,
            Some("4242") | Some("bash") => 4242,
            Some("4243") | Some("stays") => 4243,
            _ => 0,
        })
    }
    fn process_name(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        Ok(match which {
            None => Some("ahk".to_string()),
            Some("4242") | Some("bash") => Some("bash".to_string()),
            Some("4243") | Some("stays") => Some("stays".to_string()),
            _ => None,
        })
    }
    fn process_path(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        Ok(match which {
            None => Some("/usr/bin/ahk".to_string()),
            Some("4242") | Some("bash") => Some("/usr/bin/bash".to_string()),
            Some("4243") | Some("stays") => Some("/usr/bin/stays".to_string()),
            _ => None,
        })
    }
    fn process_parent(&mut self, which: Option<&str>) -> Result<u32, String> {
        Ok(match which {
            None => 1,
            Some("4242") | Some("bash") => 1,
            Some("4243") | Some("stays") => 2,
            _ => 0,
        })
    }
    fn process_close(&mut self, which: &str) -> Result<u32, String> {
        Ok(match which {
            "4242" | "bash" => 4242,
            "4243" | "stays" => 4243,
            _ => 0,
        })
    }
    fn process_wait(&mut self, which: &str, _timeout: Option<f64>) -> Result<u32, String> {
        Ok(match which {
            "4242" | "bash" => 4242,
            "4243" | "stays" => 4243,
            _ => 0,
        })
    }
    fn process_wait_close(&mut self, which: &str, _timeout: Option<f64>) -> Result<u32, String> {
        // The process that stays is still there at the timeout; the others
        // are gone, and a process that never existed is gone too.
        Ok(match which {
            "4243" | "stays" => 4243,
            _ => 0,
        })
    }
    fn process_set_priority(
        &mut self,
        _priority: &str,
        which: Option<&str>,
    ) -> Result<Option<u32>, String> {
        Ok(match which {
            None => Some(99),
            Some("4242") | Some("bash") => Some(4242),
            _ => None,
        })
    }
}

impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        self.native = Some(call.clone());
        // The bytes the callee left in the memory of an address come
        // back into the argument: the argument is the way back.
        let mut writes = self.writes.iter();
        for argument in call.arguments.iter_mut() {
            if !argument.address {
                continue;
            }
            let Some(number) = writes.next() else {
                continue;
            };
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
    report
        .variables
        .get(name)
        .cloned()
        .unwrap_or_else(|| panic!("{name} has no value"))
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
    assert_eq!(
        desktop.calls,
        vec![("set-volume".to_string(), " 30".to_string())]
    );
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
        vec![
            ("volume".to_string(), String::new()),
            ("set-volume".to_string(), " 60".to_string())
        ]
    );
}

#[test]
fn a_volume_that_is_not_a_percentage_fails_the_script() {
    let report = run("SoundSetVolume \"loud\"");
    let failure = failure_of(&report);
    assert!(
        failure.contains("percentage between -100 and 100"),
        "{failure}"
    );
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
    assert_eq!(
        desktop.calls,
        vec![("set-mute".to_string(), " true".to_string())]
    );
}

#[test]
fn a_minus_sign_in_front_of_the_volume_adjusts_it_down() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume \"-10\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The volume of the device is 50, and the sign takes ten from it.
    assert_eq!(
        desktop.calls,
        vec![
            ("volume".to_string(), String::new()),
            ("set-volume".to_string(), " 40".to_string())
        ]
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
    assert_eq!(
        desktop.calls,
        vec![("beep".to_string(), "750 500".to_string())]
    );
}

#[test]
fn a_tone_has_the_defaults_of_v2() {
    let mut desktop = Fake::default();
    let report = run_on("SoundBeep", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The frequency of v2 is 523 and its duration is 150 milliseconds.
    assert_eq!(
        desktop.calls,
        vec![("beep".to_string(), "523 150".to_string())]
    );
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
    assert_eq!(
        desktop.calls,
        vec![("play".to_string(), "/tmp/a.wav".to_string())]
    );
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
    assert_eq!(
        failure_of(&report),
        "this is a name of Windows, and this desktop is not Windows."
    );
}

#[test]
fn a_string_in_title_case_has_the_first_letter_of_each_word_up() {
    let report = run("a := StrTitle(\"This is a test.\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("This Is A Test.".into())
    );
}

#[test]
fn the_title_case_lowercases_the_rest_of_the_word() {
    let report = run("a := StrTitle(\"GONE with the WIND\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("Gone With The Wind".into())
    );
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
    let report =
        run("a := VerCompare(\"2.0-a137\", \"2.0\")\nb := VerCompare(\"2.0-a137\", \"2.0-a136\")");
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
    assert_eq!(
        desktop.calls,
        vec![("working-dir".to_string(), "there".to_string())]
    );
}

#[test]
fn the_click_of_v2_takes_the_button_first() {
    let mut desktop = Fake::default();
    let report = run_on("MouseClick \"Right\", 10, 20", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10 20".to_string())]
    );
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
    let report = run("ControlGetText(\"x\")");
    assert_eq!(
        failure_of(&report),
        "this is a name of Windows, and this desktop is not Windows."
    );
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
    // Measured on the binary of v2.0.19 on 2026-11-09: the unit is read by
    // its first letter and the case does not matter, so "days" is days and
    // only a letter that names no unit is refused.
    let report = run("DateAdd(\"20260101\", 1, \"weeks\")");
    assert_eq!(failure_of(&report), "Parameter #3 of DateAdd is invalid.");
}

#[test]
fn the_unit_of_a_date_is_read_by_its_first_letter() {
    // Measured on the binary of v2.0.19 on 2026-11-09: "days" is read the
    // way "d" is, and the case of the name does not matter.
    let report = run(concat!(
        "a := DateAdd(\"20260101\", 1, \"days\")",
        "\nb := DateDiff(\"20260102\", \"20260101\", \"Days\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("20260102000000".into()));
    assert_eq!(value_of(&report, "b"), Value::Int(1));
}

#[test]
fn the_environment_is_asked_of_the_desktop() {
    let report = run("a := EnvGet(\"HOME\")\nb := EnvSet(\"HOME\", \"there\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("the value of HOME".into())
    );
    assert_eq!(value_of(&report, "b"), Value::Text("there".into()));
}

#[test]
fn the_process_family_is_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the answer of each
    // question of the family, with the fake of the tests in the place of the
    // machine -- the pid 4242 is "bash", the pid 4243 is "stays", and a
    // process that is not there is zero.
    let report = run(concat!(
        "a := ProcessExist()\n",
        "b := ProcessExist(\"bash\")\n",
        "c := ProcessExist(4242)\n",
        "d := ProcessExist(\"nosuch\")\n",
        "e := ProcessExist(999999)\n",
        "f := ProcessGetName(\"bash\")\n",
        "g := ProcessGetPath(4242)\n",
        "h := ProcessGetParent(\"bash\")\n",
        "i := ProcessGetParent(\"nosuch\")\n",
        "j := ProcessClose(\"bash\")\n",
        "k := ProcessClose(\"nosuch\")\n",
        "l := ProcessWait(\"bash\", 1)\n",
        "m := ProcessWait(\"nosuch\", 0.01)\n",
        "n := ProcessWaitClose(\"bash\", 1)\n",
        "o := ProcessWaitClose(\"stays\", 1)\n",
        "p := ProcessSetPriority(\"High\", \"bash\")\n",
        "q := ProcessSetPriority(\"Normal\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 99);
    assert_eq!(int_of(&report, "b"), 4242);
    assert_eq!(int_of(&report, "c"), 4242);
    assert_eq!(int_of(&report, "d"), 0);
    assert_eq!(int_of(&report, "e"), 0);
    assert_eq!(value_of(&report, "f"), Value::Text("bash".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("/usr/bin/bash".into()));
    assert_eq!(int_of(&report, "h"), 1);
    assert_eq!(int_of(&report, "i"), 0);
    assert_eq!(int_of(&report, "j"), 4242);
    assert_eq!(int_of(&report, "k"), 0);
    assert_eq!(int_of(&report, "l"), 4242);
    assert_eq!(int_of(&report, "m"), 0);
    // A process that closes is zero, and a process that is still there at
    // the timeout is its pid.
    assert_eq!(int_of(&report, "n"), 0);
    assert_eq!(int_of(&report, "o"), 4243);
    assert_eq!(int_of(&report, "p"), 4242);
    assert_eq!(int_of(&report, "q"), 99);
}

#[test]
fn a_process_that_is_not_there_is_the_failure_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the name and the path
    // of a process that is not there are the failure "Target process not
    // found.", which the class of is `TargetError`.
    let report = run(concat!(
        "try {\n",
        "    a := ProcessGetName(999999)\n",
        "} catch as e {\n",
        "    MsgBox Type(e) \"|\" e.Message\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["TargetError|Target process not found."]
    );
    let report = run("a := ProcessGetPath(999999)");
    assert_eq!(failure_of(&report), "Target process not found.");
    let report = run("a := ProcessSetPriority(\"Normal\", 999999)");
    assert_eq!(failure_of(&report), "Target process not found.");
}

#[test]
fn the_process_family_refuses_what_v2_refuses() {
    // Measured on the binary of v2.0.19 on 2026-11-09: every parameter of the
    // family is a text, the timeout is a number, and the priority is one of
    // the six words -- the first letter of it is what is read.
    let report = run("a := ProcessExist(Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ProcessExist requires a String, but received a Map."
    );
    let report = run("a := ProcessWait(\"bash\", Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of ProcessWait requires a Number, but received a Map."
    );
    let report = run("a := ProcessSetPriority(3)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ProcessSetPriority is invalid."
    );
    // The binary reads the first letter of the word only, so a word that
    // starts with a letter of the six is a call of that class.
    let report = run("a := ProcessSetPriority(\"Bogus\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 99);
}

#[test]
fn the_screen_is_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the number of SysGet
    // is a metric of GetSystemMetrics, and the ones about the screen are the
    // ones this desktop can answer. The fake has two monitors, side by side,
    // and the second one is the primary one.
    let report = run(concat!(
        "a := SysGet(0)\n",
        "b := SysGet(1)\n",
        "c := SysGet(16)\n",
        "d := SysGet(76)\n",
        "e := SysGet(77)\n",
        "f := SysGet(78)\n",
        "g := SysGet(79)\n",
        "h := SysGet(80)\n",
        "i := SysGet(81)\n",
        "j := SysGet(1000)\n",
        "k := SysGet(-1)\n",
        "l := SysGet(\"1\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The width and the height of the primary monitor.
    assert_eq!(int_of(&report, "a"), 1920);
    assert_eq!(int_of(&report, "b"), 2160);
    assert_eq!(int_of(&report, "c"), 1920);
    // The virtual screen: the box around the two monitors.
    assert_eq!(int_of(&report, "d"), 0);
    assert_eq!(int_of(&report, "e"), 0);
    assert_eq!(int_of(&report, "f"), 3840);
    assert_eq!(int_of(&report, "g"), 2160);
    // The number of the monitors, and the metric of the format.
    assert_eq!(int_of(&report, "h"), 2);
    assert_eq!(int_of(&report, "i"), 1);
    // A number that is not a metric of the system is zero, the way the
    // binary answers it, and a text that holds a number is a number.
    assert_eq!(int_of(&report, "j"), 0);
    assert_eq!(int_of(&report, "k"), 0);
    assert_eq!(int_of(&report, "l"), 2160);
}

#[test]
fn a_sysget_metric_of_windows_is_refused_with_the_reason() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the metric 4 of the
    // binary is the height of the caption of a window, and this desktop has
    // no window manager of that kind to ask: the answer is the reason, and
    // not a wrong number.
    let report = run("a := SysGet(4)");
    assert_eq!(
        failure_of(&report),
        "SysGet(4) is a metric of the window manager of Windows, and this desktop does not have one."
    );
    let report = run("a := SysGet(Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of SysGet requires a Number, but received a Map."
    );
}

#[test]
fn the_output_parameters_of_the_monitor_reads_want_a_reference() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the four output
    // parameters of the two monitor reads are values of the kind VarRef, and
    // a number there is refused with the name of the built-in in the `What`
    // and the number in the `Extra`.
    let report = run("MonitorGet(1, 9, 9, 9, 9)");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of MonitorGet requires a VarRef, but received an Integer."
    );
    let report = run("MonitorGetWorkArea(1, 9, 9, 9, 9)");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of MonitorGetWorkArea requires a VarRef, but received an Integer."
    );
}

#[test]
fn the_monitors_are_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the rectangle of a
    // monitor is left, top, right and bottom, and the right and the bottom
    // are one past the last column and the last row; the monitors count from
    // one, and the monitor that a call does not name is the primary one.
    let report = run(concat!(
        "count := MonitorGetCount()\n",
        "primary := MonitorGetPrimary()\n",
        "name0 := MonitorGetName()\n",
        "name2 := MonitorGetName(2)\n",
        "l1 := 0, t1 := 0, r1 := 0, b1 := 0\n",
        "ok1 := MonitorGet(1, &l1, &t1, &r1, &b1)\n",
        "rect1 := l1 \"|\" t1 \"|\" r1 \"|\" b1\n",
        "l2 := 0, t2 := 0, r2 := 0, b2 := 0\n",
        "ok2 := MonitorGet(2, &l2, &t2, &r2, &b2)\n",
        "rect2 := l2 \"|\" t2 \"|\" r2 \"|\" b2\n",
        "wl := 0, wt := 0, wr := 0, wb := 0\n",
        "okw := MonitorGetWorkArea(2, &wl, &wt, &wr, &wb)\n",
        "work := wl \"|\" wt \"|\" wr \"|\" wb\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "count"), 2);
    assert_eq!(int_of(&report, "primary"), 2);
    // The monitor that a call does not name is the primary one.
    assert_eq!(value_of(&report, "name0"), Value::Text("\\\\.\\DISPLAY2".into()));
    assert_eq!(value_of(&report, "name2"), Value::Text("\\\\.\\DISPLAY2".into()));
    assert_eq!(int_of(&report, "ok1"), 1);
    assert_eq!(value_of(&report, "rect1"), Value::Text("0|0|1920|2160".into()));
    assert_eq!(int_of(&report, "ok2"), 1);
    assert_eq!(
        value_of(&report, "rect2"),
        Value::Text("1920|0|3840|2160".into())
    );
    assert_eq!(int_of(&report, "okw"), 1);
    assert_eq!(
        value_of(&report, "work"),
        Value::Text("1920|0|3840|2112".into())
    );
}

#[test]
fn a_monitor_that_is_not_there_is_refused_with_the_parameter() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a monitor that is not
    // there is "Parameter #1 of MonitorGet is invalid." with the number in
    // the `Extra`, a value where a variable is wanted is refused with the
    // kind it got, and a monitor number that is not a number is refused with
    // the parameter that wants one.
    let report = run("a := MonitorGet(3)");
    assert_eq!(failure_of(&report), "Parameter #1 of MonitorGet is invalid.");
    let report = run("a := MonitorGet(1, 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of MonitorGet requires a VarRef, but received an Integer."
    );
    let report = run("a := MonitorGet(Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of MonitorGet requires a Number, but received a Map."
    );
    let report = run("a := MonitorGetName(Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of MonitorGetName requires a Number, but received a Map."
    );
}

#[test]
fn the_addresses_of_the_machine_are_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the answer is an
    // array of texts, one for each address of the machine, and the address of
    // the loopback is not one of them.
    let report = run(concat!(
        "a := SysGetIPAddresses()\n",
        "b := Type(a)\n",
        "c := a.Length\n",
        "d := a[1]\n",
        "e := a[2]\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "b"), Value::Text("Array".into()));
    assert_eq!(int_of(&report, "c"), 2);
    assert_eq!(value_of(&report, "d"), Value::Text("172.26.48.1".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("192.168.0.45".into()));
}

#[test]
fn the_lock_keys_are_turned_where_the_state_is_not_what_was_asked_for() {
    // Measured on the binary of v2.0.19 on 2026-11-09, with the conversion of
    // the state read beside it: "On" and "1" turn the state on, "Off" and "0"
    // turn it off, a word it does not know -- and the empty text, and a
    // missing parameter -- leave the state alone and are not a failure, and
    // every form answers the empty text.
    let mut desktop = Fake::default();
    let report = run_on(
        concat!(
            "a := SetCapsLockState(\"On\")\n",
            "b := SetNumLockState(\"Off\")\n",
            "c := SetScrollLockState(1)\n",
            "d := SetCapsLockState(0)\n",
            "e := SetCapsLockState(\"Bogus\")\n",
            "f := SetCapsLockState()\n",
            "g := SetCapsLockState(\"AlwaysOn\")\n",
        ),
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    for name in ["a", "b", "c", "d", "e", "f", "g"] {
        assert_eq!(value_of(&report, name), Value::Text(String::new()), "{name}");
    }
    // The turns of the keys are the ones the states asked for: the bogus word,
    // the missing parameter and the "AlwaysOn" of a key that is on already
    // leave the switch alone.
    assert_eq!(
        desktop.locks,
        vec![
            ("CapsLock".to_string(), true),
            ("NumLock".to_string(), false),
            ("ScrollLock".to_string(), true),
            ("CapsLock".to_string(), false),
            ("CapsLock".to_string(), true),
        ]
    );
    assert_eq!(
        failure_of(&run("a := SetCapsLockState(Map())")),
        "Parameter #1 of SetCapsLockState requires a String, but received a Map."
    );
}

#[test]
fn the_debugger_gets_the_text_and_the_answer_is_nothing() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `OutputDebug` writes
    // the text to the debugger and answers the empty text.
    let mut desktop = Fake::default();
    let report = run_on("a := OutputDebug(\"hello\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text(String::new()));
    assert_eq!(desktop.debugged, vec!["hello".to_string()]);
    assert_eq!(
        failure_of(&run("a := OutputDebug(Map())")),
        "Parameter #1 of OutputDebug requires a String, but received a Map."
    );
}

#[test]
fn the_file_is_recycled_and_the_trash_is_emptied() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a file that is there
    // goes into the trash and the answer is the empty text, a plain path that
    // is not there is the failure "Failed", a pattern that matches nothing is
    // not a failure, and an empty path is "Parameter #1 of FileRecycle is
    // invalid." with the empty text in the `Extra`.
    let mut desktop = Fake::default();
    let report = run_on(
        concat!(
            "a := FileRecycle(\"notes.txt\")\n",
            "b := FileRecycle(\"nothing-*.txt\")\n",
            "c := FileRecycleEmpty()\n",
            "d := FileRecycleEmpty(\"/\")\n",
        ),
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text(String::new()));
    assert_eq!(
        desktop.recycled,
        vec!["notes.txt".to_string(), "nothing-*.txt".to_string()]
    );
    assert!(desktop.trash_emptied, "the trash was emptied");
    assert_eq!(
        failure_of(&run("a := FileRecycle(\"the-missing-one.txt\")")),
        "Failed"
    );
    let report = run(concat!(
        "try\n",
        "    FileRecycle(\"\")\n",
        "catch as e\n",
        "    x := e.Message \"|\" e.Extra\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "x"),
        Value::Text("Parameter #1 of FileRecycle is invalid.|empty string".into())
    );
}

#[test]
fn the_readings_of_a_drive_are_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the capacity and the
    // free space are counted in megabytes, the serial is a number, the
    // filesystem and the label are texts, a drive that is not there is the
    // empty text for the type and "Invalid" for the state, and the other
    // readings of a drive that is not there are the failure of the machine.
    let report = run(concat!(
        "a := DriveGetType(\"/\")\n",
        "b := DriveGetStatus(\"/\")\n",
        "c := DriveGetFilesystem(\"/\")\n",
        "d := DriveGetLabel(\"/\")\n",
        "e := DriveGetSerial(\"/\")\n",
        "f := DriveGetSpaceFree(\"/\")\n",
        "g := DriveGetCapacity(\"/\")\n",
        "h := DriveGetType(\"/no-such-path\")\n",
        "i := DriveGetStatus(\"/no-such-path\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Fixed".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("Ready".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("ext4".into()));
    assert_eq!(value_of(&report, "d"), Value::Text(String::new()));
    assert_eq!(int_of(&report, "e"), 42);
    assert_eq!(int_of(&report, "f"), 100);
    assert_eq!(int_of(&report, "g"), 200);
    // A path that is not a drive is not a failure for the type and the state.
    assert_eq!(value_of(&report, "h"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "i"), Value::Text("Invalid".into()));
    // The five readings that answer with the failure of the machine say it.
    assert_eq!(
        failure_of(&run("a := DriveGetFilesystem(\"/no-such-path\")")),
        "(2) No such file or directory"
    );
    // A machine without drive letters has no list of them to answer: measured on
    // the binary of v2.0.19 on 2026-11-09, `DriveGetList` answers the letters of
    // the drives of Windows ("BCDEFX" on the machine of the binary), and the
    // volumes of this machine are named by the paths they are mounted at. The
    // name therefore stays a refusal of the table, with the reason of it.
    assert_eq!(
        failure_of(&run("a := DriveGetList()")),
        "this is a name of Windows, and this desktop is not Windows."
    );
    assert_eq!(
        failure_of(&run("a := DriveGetType(Map())")),
        "Parameter #1 of DriveGetType requires a String, but received a Map."
    );
}

#[test]
fn the_suspend_and_the_pause_are_a_state_of_the_run() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `Suspend 1` and
    // `Suspend 0` turn the state off and on, the omitted parameter and the
    // number minus one toggle it, and the parameter is a Number: "On" is the
    // TypeError of a String, and `Suspend 2` is "Parameter #1 ... is invalid."
    // with the number in the `Extra`. The answer of both commands is the empty
    // text. The runner has no keyboard, so the state is what is observable --
    // there is no hotkey to fire here.
    let report = run(concat!(
        "s0 := A_IsSuspended\n",
        "r := Suspend(1)\n",
        "s1 := A_IsSuspended\n",
        "Suspend(0)\n",
        "s2 := A_IsSuspended\n",
        "Suspend()\n",
        "s3 := A_IsSuspended\n",
        "Suspend()\n",
        "s4 := A_IsSuspended\n",
        "Suspend(-1)\n",
        "s5 := A_IsSuspended\n",
        "p0 := A_IsPaused\n",
        "Pause(1)\n",
        "p1 := A_IsPaused\n",
        "Pause(0)\n",
        "p2 := A_IsPaused\n",
        "Pause()\n",
        "p3 := A_IsPaused\n",
        "Pause()\n",
        "p4 := A_IsPaused\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "r"), Value::Text(String::new()));
    assert_eq!(int_of(&report, "s0"), 0);
    assert_eq!(int_of(&report, "s1"), 1);
    assert_eq!(int_of(&report, "s2"), 0);
    assert_eq!(int_of(&report, "s3"), 1);
    assert_eq!(int_of(&report, "s4"), 0);
    assert_eq!(int_of(&report, "s5"), 1);
    assert_eq!(int_of(&report, "p0"), 0);
    assert_eq!(int_of(&report, "p1"), 1);
    assert_eq!(int_of(&report, "p2"), 0);
    assert_eq!(int_of(&report, "p3"), 1);
    assert_eq!(int_of(&report, "p4"), 0);
    // The parameter is a Number, and the words of the v1 are not taken.
    assert_eq!(
        failure_of(&run("Suspend(\"On\")")),
        "Parameter #1 of Suspend requires a Number, but received a String."
    );
    assert_eq!(
        failure_of(&run("Pause(\"Off\")")),
        "Parameter #1 of Pause requires a Number, but received a String."
    );
    // A number that is not one, minus one or zero is invalid, and the number
    // is in the `Extra`.
    let report = run(concat!(
        "try\n",
        "    Suspend(2)\n",
        "catch as e\n",
        "    a := Type(e) \"|\" e.Message \"|\" e.Extra\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("ValueError|Parameter #1 of Suspend is invalid.|2".into())
    );
}

#[test]
fn the_windows_of_the_session_are_asked_of_the_desktop() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `WinGetList` answers
    // an array of the ids as whole numbers, and the empty array when nothing
    // matches; `WinGetCount` the number; and the other three the id or the
    // class of the first, with "Target window not found." when there is none.
    let report = run(concat!(
        "list := WinGetList()\n",
        "kind := Type(list)\n",
        "len := list.Length\n",
        "first := list[1]\n",
        "some := WinGetList(\"Fire\")\n",
        "some_len := some.Length\n",
        "none := WinGetList(\"zzz-nothing-zzz\").Length\n",
        "count := WinGetCount()\n",
        "count_some := WinGetCount(\"Fire\")\n",
        "count_none := WinGetCount(\"zzz-nothing-zzz\")\n",
        "id_first := WinGetID()\n",
        "id_some := WinGetID(\"Fire\")\n",
        "id_last := WinGetIDLast()\n",
        "id_last_some := WinGetIDLast(\"Fire\")\n",
        "class_first := WinGetClass()\n",
        "class_some := WinGetClass(\"Fire\")\n",
        "id_of_id := WinGetID(102)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "kind"), Value::Text("Array".into()));
    assert_eq!(int_of(&report, "len"), 3);
    assert_eq!(int_of(&report, "first"), 101);
    assert_eq!(int_of(&report, "some_len"), 1);
    assert_eq!(int_of(&report, "none"), 0);
    assert_eq!(int_of(&report, "count"), 3);
    assert_eq!(int_of(&report, "count_some"), 1);
    assert_eq!(int_of(&report, "count_none"), 0);
    // The first and the last of the windows, and an id that names one.
    assert_eq!(int_of(&report, "id_first"), 101);
    assert_eq!(int_of(&report, "id_some"), 102);
    assert_eq!(int_of(&report, "id_last"), 103);
    assert_eq!(int_of(&report, "id_last_some"), 102);
    assert_eq!(value_of(&report, "class_first"), Value::Text("steam_app".into()));
    assert_eq!(value_of(&report, "class_some"), Value::Text("firefox".into()));
    assert_eq!(int_of(&report, "id_of_id"), 102);
}

#[test]
fn a_window_that_is_not_there_is_the_target_error_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `WinGetID`,
    // `WinGetIDLast` and `WinGetClass` of a window that is not there are the
    // failure "Target window not found." of the class `TargetError`, and the
    // title that was asked about is in its `Extra`; a window that is named by
    // its id leaves the `Extra` empty.
    let report = run(concat!(
        "try\n",
        "    WinGetID(\"zzz-nothing-zzz\")\n",
        "catch as e\n",
        "    a := Type(e) \"|\" e.Message \"|\" e.Extra\n",
        "try\n",
        "    WinGetID(999)\n",
        "catch as e2\n",
        "    b := Type(e2) \"|\" e2.Extra\n",
        "try\n",
        "    WinGetClass(\"zzz-nothing-zzz\")\n",
        "catch as e3\n",
        "    c := e3.Message\n",
        "try\n",
        "    WinGetIDLast(\"zzz-nothing-zzz\")\n",
        "catch as e4\n",
        "    d := e4.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("TargetError|Target window not found.|zzz-nothing-zzz".into())
    );
    assert_eq!(
        value_of(&report, "b"),
        Value::Text("TargetError|".into())
    );
    assert_eq!(
        value_of(&report, "c"),
        Value::Text("Target window not found.".into())
    );
    assert_eq!(
        value_of(&report, "d"),
        Value::Text("Target window not found.".into())
    );
}

#[test]
fn a_window_is_named_by_its_id_in_the_window_family() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a whole number as the
    // window is its id, and `WinExist` and `WinActive` answer the id of the
    // window when it is there, and zero when it is not.
    let report = run(concat!(
        "exist_title := WinExist(\"Fire\")\n",
        "exist_id := WinExist(102)\n",
        "exist_none := WinExist(999)\n",
        "exist_absent := WinExist(\"zzz-nothing-zzz\")\n",
        "active_title := WinActive(\"Steam\")\n",
        "active_other := WinActive(\"Fire\")\n",
        "active_id := WinActive(101)\n",
        "active_none := WinActive(102)\n",
        "title_of_id := WinGetTitle(102)\n",
        "title_of_title := WinGetTitle(\"Fire\")\n",
        "title_active := WinGetTitle()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "exist_title"), 102);
    assert_eq!(int_of(&report, "exist_id"), 102);
    assert_eq!(int_of(&report, "exist_none"), 0);
    assert_eq!(int_of(&report, "exist_absent"), 0);
    assert_eq!(int_of(&report, "active_title"), 101);
    assert_eq!(int_of(&report, "active_other"), 0);
    assert_eq!(int_of(&report, "active_id"), 101);
    assert_eq!(int_of(&report, "active_none"), 0);
    assert_eq!(value_of(&report, "title_of_id"), Value::Text("Firefox".into()));
    assert_eq!(
        value_of(&report, "title_of_title"),
        Value::Text("Firefox".into())
    );
    assert_eq!(value_of(&report, "title_active"), Value::Text("Steam".into()));
    let report = run("WinGetTitle(999)");
    assert_eq!(failure_of(&report), "Target window not found.");
    // The other parameters of v2 are the text of the window and the two
    // exclusions, and they are not carried.
    assert_eq!(
        failure_of(&run("WinGetID(\"a\", \"b\")")),
        "the text of the window and the exclusions are not carried yet."
    );
}

#[test]
fn the_files_of_a_pattern_are_copied_moved_and_taken_away() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the name of the file
    // of `FileCopy`, `FileMove` and `FileDelete` may be a pattern whose last
    // part holds a `*` (any run of characters) and a `?` (one character), and
    // a destination that ends with a separator, or that is a folder, is the
    // folder the files go into. The wildcards of v2 are in the names of the
    // files -- a title of a window has none, and a `*` of a title is a
    // character of the title.
    let mut desktop = Fake::default();
    let report = run_on(
        concat!(
            "a := FileCopy(\"/tmp/*.txt\", \"/tmp/out/\")\n",
            "b := FileMove(\"/tmp/*.md\", \"/tmp/out/\")\n",
            "c := FileDelete(\"/tmp/*.log\")\n",
        ),
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text(String::new()));
    let copies: Vec<&(String, String)> = desktop
        .calls
        .iter()
        .filter(|(what, _)| what == "copy-file")
        .collect();
    assert_eq!(
        copies,
        vec![
            &(
                "copy-file".to_string(),
                "/tmp/a.txt|/tmp/out/a.txt|false".to_string()
            ),
            &(
                "copy-file".to_string(),
                "/tmp/b.txt|/tmp/out/b.txt|false".to_string()
            ),
        ]
    );
    let moves: Vec<&(String, String)> = desktop
        .calls
        .iter()
        .filter(|(what, _)| what == "move-file")
        .collect();
    assert_eq!(moves.len(), 2, "{:?}", desktop.calls);
    let deletes: Vec<&(String, String)> = desktop
        .calls
        .iter()
        .filter(|(what, _)| what == "delete-file")
        .collect();
    assert_eq!(
        deletes,
        vec![
            &("delete-file".to_string(), "/tmp/a.log".to_string()),
            &("delete-file".to_string(), "/tmp/b.log".to_string()),
        ]
    );
    // The destination may be a pattern too: measured on the binary of
    // v2.0.19 on 2026-11-09, the `*` of the destination is the name of the
    // file without its extension, and the literal after it is the extension
    // the copies take.
    let mut desktop = Fake::default();
    let report = run_on("FileCopy(\"/tmp/*.txt\", \"/tmp/out/*.bak\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let copies: Vec<&(String, String)> = desktop
        .calls
        .iter()
        .filter(|(what, _)| what == "copy-file")
        .collect();
    assert_eq!(
        copies,
        vec![
            &(
                "copy-file".to_string(),
                "/tmp/a.txt|/tmp/out/a.bak|false".to_string()
            ),
            &(
                "copy-file".to_string(),
                "/tmp/b.txt|/tmp/out/b.bak|false".to_string()
            ),
        ]
    );
}

#[test]
fn the_mode_of_the_title_is_the_one_of_v2() {
    let _turn = THE_TITLE_MODE_TURN
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // Measured on the binary of v2.0.19 on 2026-11-09: the command answers
    // the mode that was there before, the mode 1 is "the title starts with
    // the text", the mode 2 "it has the text in it" and the mode 3 "it is
    // the text", the word "RegEx" is the mode of the patterns, and a value
    // that is none of them is "Parameter #1 of SetTitleMatchMode is invalid."
    // with the value in the `Extra`.
    let report = run(concat!(
        "a := SetTitleMatchMode(1)\n",
        "m1 := A_TitleMatchMode\n",
        "b := SetTitleMatchMode(3)\n",
        "c := SetTitleMatchMode(\"RegEx\")\n",
        "d := A_TitleMatchMode\n",
        "e := SetTitleMatchMode(\"Fast\")\n",
        "f := SetTitleMatchMode(\"Slow\")\n",
        "g := A_TitleMatchMode\n",
        "try\n",
        "    SetTitleMatchMode(\"Bogus\")\n",
        "catch as e1\n",
        "    bad := Type(e1) \"|\" e1.Message \"|\" e1.Extra\n",
        "try\n",
        "    SetTitleMatchMode(4)\n",
        "catch as e2\n",
        "    bad4 := e2.Extra\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 2);
    assert_eq!(int_of(&report, "m1"), 1);
    assert_eq!(int_of(&report, "b"), 1);
    assert_eq!(int_of(&report, "c"), 3);
    assert_eq!(value_of(&report, "d"), Value::Text("RegEx".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("Fast".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("Fast".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("RegEx".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("RegEx".into()));
    assert_eq!(
        value_of(&report, "bad"),
        Value::Text("ValueError|Parameter #1 of SetTitleMatchMode is invalid.|Bogus".into())
    );
    assert_eq!(value_of(&report, "bad4"), Value::Text("4".into()));
}

#[test]
fn the_title_of_a_window_is_matched_the_way_the_mode_says() {
    let _turn = THE_TITLE_MODE_TURN
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    // Measured on the binary of v2.0.19 on 2026-11-09: the mode 1 wants the
    // title to start with the text, the mode 2 to have it in it and the mode
    // 3 to be the text; the `RegEx` mode reads the text as a pattern; and the
    // `*` and the `?` of a title are characters of the title, and not
    // wildcards -- v2.0.19 has no wildcard in a window title.
    let report = run(concat!(
        "SetTitleMatchMode(2)\n",
        "c_start := WinExist(\"Ste\")\n",
        "c_middle := WinExist(\"tea\")\n",
        "c_star := WinExist(\"Steam*\")\n",
        "c_question := WinExist(\"Ste?m\")\n",
        "SetTitleMatchMode(1)\n",
        "s_start := WinExist(\"Ste\")\n",
        "s_middle := WinExist(\"tea\")\n",
        "SetTitleMatchMode(3)\n",
        "e_exact := WinExist(\"Steam\")\n",
        "e_start := WinExist(\"Ste\")\n",
        "SetTitleMatchMode(\"RegEx\")\n",
        "r_start := WinExist(\"^Ste\")\n",
        "r_any := WinExist(\"Fir.*\")\n",
        "r_bad := WinExist(\"(unclosed\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "c_start"), 101);
    assert_eq!(int_of(&report, "c_middle"), 101);
    // The wildcards of v2 are in the names of the files, and not in a title.
    assert_eq!(int_of(&report, "c_star"), 0);
    assert_eq!(int_of(&report, "c_question"), 0);
    assert_eq!(int_of(&report, "s_start"), 101);
    assert_eq!(int_of(&report, "s_middle"), 0);
    assert_eq!(int_of(&report, "e_exact"), 101);
    assert_eq!(int_of(&report, "e_start"), 0);
    assert_eq!(int_of(&report, "r_start"), 101);
    assert_eq!(int_of(&report, "r_any"), 102);
    // A pattern that does not compile is not a failure of the question about
    // a window: measured on the binary of v2.0.19 on 2026-11-09, the answer
    // is that no window matches, while a direct `RegExMatch` fails.
    assert_eq!(int_of(&report, "r_bad"), 0);
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
    let report = run_on(
        "a := FileExist(\"*.txt\")\nb := DirExist(\"/tmp\")",
        &mut desktop,
    );
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
    assert_eq!(
        value_of(&report, "text"),
        Value::Text("one\r\ntwo\r\n".into())
    );
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
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | FileRead | bogus"]
    );
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
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | FileAppend | m5000"]
    );
}

#[test]
fn a_text_is_cut_at_the_delimiters() {
    let report = run("pieces := StrSplit(\"a,b,c\", \",\")\n\
         kind := Type(pieces)\nlength := pieces.Length\nfirst := pieces[1]");
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
    let report = run("one := StrSplit(\"aXXb\", \"XX\")\n\
         two := StrSplit(\"a.b.c\", \"..\")\n\
         first := one[1] . \",\" . one[2]\n\
         second := two[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b".into()));
    assert_eq!(value_of(&report, "second"), Value::Text("a.b.c".into()));
}

#[test]
fn the_delimiters_of_a_text_may_be_an_array() {
    let report = run("pieces := StrSplit(\"a.b-c\", [\".\", \"-\"])\n\
         first := pieces[1] . \",\" . pieces[2] . \",\" . pieces[3]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b,c".into()));
}

#[test]
fn the_characters_to_omit_are_cut_from_the_pieces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the characters to
    // omit are cut from the beginning and the end of every piece.
    let report = run("pieces := StrSplit(\"a,b,c\", \",\", \"ac\")\n\
         first := \"[\" pieces[1] \"]\" . \"[\" pieces[2] \"]\" . \"[\" pieces[3] \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("[][b][]".into()));
}

#[test]
fn the_number_of_the_pieces_says_how_many_times_the_text_is_cut() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text is cut a
    // maximum of MaxParts-1 times, and the rest of the text is the last
    // piece.
    let report = run("two := StrSplit(\"a,b,c\", \",\", \"\", 2)\n\
         one := StrSplit(\"a,b,c\", \",\", \"\", 1)\n\
         none := StrSplit(\"a,b,c\", \",\", \"\", 0)\n\
         first := two[1] . \",\" . two[2] . \" \" . one[1] . \" \" . none.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "first"),
        Value::Text("a,b,c a,b,c 0".into())
    );
}

#[test]
fn a_blank_delimiter_makes_a_piece_of_every_character() {
    // Measured on the binary of v2.0.19 on 2026-10-31: when the
    // delimiters are blank every character of the text is a piece, and
    // the characters to omit are not pieces at all.
    let report = run("pieces := StrSplit(\"a b\", \"\", \" \")\n\
         first := pieces[1] . pieces[2]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("ab".into()));
}

#[test]
fn a_text_that_is_empty_has_no_piece() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a text that is empty
    // has no piece at all, and the piece that is empty is not a piece.
    let report = run("pieces := StrSplit(\"\", \",\")\ncount := pieces.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "count"), 0);
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
        "SplitPath \"C:\\Users\\<user>\\file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("file.txt|C:\\Users\\<user>|txt|file|C:".into())
    );
}

#[test]
fn a_path_with_a_drive_and_no_backslash_keeps_the_drive_out_of_the_name() {
    // Measured on the binary of v2.0.19: the drive is cut off the path
    // before the name is, and the slash is not a separator of a file.
    let report = run(concat!(
        "SplitPath \"C:/Users/<user>/file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("/Users/<user>/file.txt|C:|txt|/Users/<user>/file|C:".into())
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
fn is_set_says_whether_the_variable_has_a_value() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `u := unset` takes
    // the value of the variable away, a variable that was never written has
    // none either, and a variable with a value is one -- in a function too,
    // through the `global` declaration.
    let report = run(concat!(
        "u := unset\n",
        "w := 5\n",
        "Probe() {\n",
        "    global w\n",
        "    return IsSet(w)\n",
        "}\n",
        "a := IsSet(u)\n",
        "b := IsSet(w)\n",
        "c := IsSet(never_written)\n",
        "d := Probe()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 0);
    assert_eq!(int_of(&report, "b"), 1);
    assert_eq!(int_of(&report, "c"), 0);
    assert_eq!(int_of(&report, "d"), 1);
}

#[test]
fn is_set_refuses_a_value_that_is_not_a_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the argument of
    // `IsSet` is a variable, and a value that is not one is refused with
    // "IsSet requires a variable." -- at the load in the binary, and at the
    // run in the tree, which checks the call when it runs.
    let report = run("a := IsSet(1)");
    assert_eq!(failure_of(&report), "IsSet requires a variable.");
    let report = run("x := 1\na := IsSet(&x)");
    assert_eq!(failure_of(&report), "IsSet requires a variable.");
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
fn the_commands_of_the_delay_of_the_controls_and_of_the_view_of_the_registry_answer_with_the_value_that_was_there()
 {
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
        vec![
            "Parameter #1 of SetControlDelay requires a Number, but received a String. | TypeError | SetControlDelay | x"
        ]
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
        vec![
            "Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | Bogus"
        ]
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
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | 5"]
    );
}

#[test]
fn the_mode_of_the_keys_is_the_one_of_the_box() {
    let report = run("SendMode \"Input\"");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("keyboard of the box")),
        "{:?}",
        report.notes
    );
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
    assert!(
        !typed.is_empty(),
        "the timer did not run: {:?}",
        report.steps
    );
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
    let report =
        run("try {\n    x := Abs({})\n} catch Any as e {\n    MsgBox \"[\" e.Extra \"]\"\n}");
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
fn round_with_decimals_answers_the_text_of_them() {
    // Measured on the binary of v2.0.19 on 2026-11-09: with a positive
    // number of decimals the answer is a text of exactly that many digits
    // after the point, the zeros kept.
    let report = run(concat!(
        "a := Round(2.567, 2)",
        "\nb := Round(1.005, 2)",
        "\nc := Round(2.345, 2)",
        "\nd := Round(2.567, 3)",
        "\ne := Round(2, 2)",
        "\nf := Round(-2.567, 2)",
        "\ng := Round(2.5, 2)",
        "\nh := Round(0.0005, 2)",
        "\ni := Round(-1.005, 2)",
        "\nj := Round(123.456, 2)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2.57".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("1.00".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("2.35".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("2.567".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("2.00".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("-2.57".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("2.50".into()));
    assert_eq!(value_of(&report, "h"), Value::Text("0.00".into()));
    assert_eq!(value_of(&report, "i"), Value::Text("-1.00".into()));
    assert_eq!(value_of(&report, "j"), Value::Text("123.46".into()));
}

#[test]
fn a_round_of_decimals_is_a_text_of_the_length_of_the_decimals() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the answer of a
    // round with decimals is a String, and its length is the digits of the
    // number, the point and the decimals.
    let report = run(concat!(
        "a := Type(Round(2.567, 2))",
        "\nb := StrLen(Round(2.567, 2))",
        "\nc := StrLen(Round(1.005, 2))",
        "\nd := Type(Round(2.5))",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("String".into()));
    assert_eq!(value_of(&report, "b"), Value::Int(4));
    assert_eq!(value_of(&report, "c"), Value::Int(4));
    assert_eq!(value_of(&report, "d"), Value::Text("Integer".into()));
}

#[test]
fn a_round_without_decimals_is_a_whole_number() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the number is
    // rounded away from zero on the half, and the answer is an Integer when
    // the decimals are omitted, zero or negative.
    let report = run(concat!(
        "a := Round(2.4)",
        "\nb := Round(2.5)",
        "\nc := Round(-2.5)",
        "\nd := Round(2.5, 0)",
        "\ne := Round(1043.22, -1)",
        "\nf := Round(-0.5)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(2));
    assert_eq!(value_of(&report, "b"), Value::Int(3));
    assert_eq!(value_of(&report, "c"), Value::Int(-3));
    assert_eq!(value_of(&report, "d"), Value::Int(3));
    assert_eq!(value_of(&report, "e"), Value::Int(1040));
    assert_eq!(value_of(&report, "f"), Value::Int(-1));
}

#[test]
fn a_round_reads_a_number_the_way_the_binary_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a text that holds a
    // number is one, and the number of the decimals is a whole number.
    let report = run(concat!(
        "a := Round(\"2.567\", 2)",
        "\nb := Round(2.567, \"2\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2.57".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("2.57".into()));
    let report = run("a := Round(\"x\", 2)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Round requires a Number, but received a String."
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
    let report =
        run("class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\nx := ObjGetBase(b)");
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
    let report = run("class Box {\n  x := 1\n}\nb := Box()\n\
         names := ObjOwnProps(b)\ncount := ObjOwnPropCount(b)\nhas := ObjHasOwnProp(b, \"x\")\n\
         kind := Type(names)\n\
         first := \"\"\nnames.Call(&first)\n\
         after := \"\"\nnames.Call(&after)");
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
    let report = run(
        "m := Map()\nnames := ObjOwnProps(m)\nkind := Type(names)\ncount := ObjOwnPropCount(m)",
    );
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
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Object.Prototype.DefineProp is invalid."
    );
}

#[test]
fn an_object_without_a_member_is_not_a_descriptor() {
    let report = run("o := {}\no.DefineProp(\"x\", {y: 1})");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Object.Prototype.DefineProp is invalid."
    );
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
    let report = run("o := {}\no.DefineProp(\"x\", {Value: 5})\n\
         a := Type(o.GetOwnPropDesc(\"x\")) . \" \" . o.GetOwnPropDesc(\"x\").Value");
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
    let report = run(
        "o := {}\no.DefineProp(\"x\", {Value: 7})\na := o.DeleteProp(\"x\")\nb := o.DeleteProp(\"x\")",
    );
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
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("apple\nfig\npear".into())
    );
}

#[test]
fn sort_puts_a_list_in_order() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the parameter of a
    // `Sort` is a text, and an array is refused with the reason of v2.
    let mut desktop = Fake::default();
    let mut runtime = Runtime::new(&mut desktop, Duration::from_secs(10));
    let sort = ahk::builtins::find("Sort").expect("the tree carries Sort");
    let numbers = Value::List(vec![Value::Int(3), Value::Int(1), Value::Int(2)]);
    let failure = (sort.handler)(&mut runtime, &[numbers], 1, 0)
        .expect_err("an array is not a text");
    assert_eq!(
        failure.message,
        "Parameter #1 of Sort requires a String, but received an Array."
    );
}

#[test]
fn sort_of_a_number_sorts_its_text() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a number is turned
    // into its text and the text is what is sorted, so `Sort(5)` is "5".
    let report = run(concat!(
        "a := Sort(5)",
        "\nb := Sort(2.5)",
        "\nc := Sort(true)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("5".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("2.5".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("1".into()));
}

#[test]
fn sort_of_something_that_is_not_a_list_fails() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a map is not a text.
    let report = run("a := Sort(Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Sort requires a String, but received a Map."
    );
}

#[test]
fn sort_reads_the_options_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-11-09.
    let report = run(concat!(
        "a := Sort(\"233`n40\", \"N\")",
        "\nb := Sort(\"b`na\")",
        "\nc := Sort(\"b`na\", \"R\")",
        "\nd := Sort(\"b`na`nB`nA\", \"C\")",
        "\ne := Sort(\"b`na`nB`nA\", \"CL\")",
        "\nf := Sort(\"b`na`nb\", \"U\")",
        "\ng := Sort(\"b`na`nB`nA\", \"UC\")",
        "\nh := Sort(\"5,3,7,1\", \"N D,\")",
        "\ni := Sort(\"b`na`n\", \"Z\")",
        "\nj := Sort(\"abcd`nabcf`nabce\", \"P3\")",
        "\nk := Sort(\"zab`nbac`nxaa\", \"P2\")",
        "\nl := Sort(\"dir\\b`ndir\\a\", \"\\\")",
        "\nm := Sort(\"A2`nA10`nA1\", \"CLogical\")",
        "\nn := Sort(\"B`na\", \"C1\")",
        "\no := Sort(\"B`na\", \"COn\")",
        "\np := Sort(\"B`na\", \"C0\")",
        "\nq := Sort(\"B`na\", \"COff\")",
        "\nr := Sort(\"B`na\", \"CLocale\")",
        "\ns := Sort(\"b`na\", \"XY\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("40\n233".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a\nb".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("b\na".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("A\nB\na\nb".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("a\nA\nb\nB".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("a\nb".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("A\nB\na\nb".into()));
    assert_eq!(value_of(&report, "h"), Value::Text("1,3,5,7".into()));
    assert_eq!(value_of(&report, "i"), Value::Text("\na\nb".into()));
    assert_eq!(value_of(&report, "j"), Value::Text("abcd\nabce\nabcf".into()));
    assert_eq!(value_of(&report, "k"), Value::Text("xaa\nzab\nbac".into()));
    assert_eq!(value_of(&report, "l"), Value::Text("dir\\a\ndir\\b".into()));
    assert_eq!(value_of(&report, "m"), Value::Text("A1\nA2\nA10".into()));
    assert_eq!(value_of(&report, "n"), Value::Text("B\na".into()));
    assert_eq!(value_of(&report, "o"), Value::Text("B\na".into()));
    assert_eq!(value_of(&report, "p"), Value::Text("a\nB".into()));
    assert_eq!(value_of(&report, "q"), Value::Text("a\nB".into()));
    assert_eq!(value_of(&report, "r"), Value::Text("a\nB".into()));
    assert_eq!(value_of(&report, "s"), Value::Text("a\nb".into()));
}

#[test]
fn sort_numbers_the_items_the_way_the_binary_does() {
    // Measured on the binary of v2.0.19 on 2026-11-09: an item that is not
    // a number counts as zero, and the ties keep the order they came in.
    let report = run(concat!(
        "a := Sort(\"3`n10`n2\", \"N\")",
        "\nb := Sort(\"x`n10`nabc`n2\", \"N\")",
        "\nc := Sort(\"1.5`n1.10`n1.9\", \"N\")",
        "\nd := Sort(\"10`n9`n1\", \"NR\")",
        "\ne := Sort(\"10`n9`n1\", \"NRU\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2\n3\n10".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("x\nabc\n2\n10".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("1.10\n1.5\n1.9".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("10\n9\n1".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("10\n9\n1".into()));
}

#[test]
fn sort_keeps_the_shape_of_the_text_it_was_given() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the trailing
    // delimiter of the list is kept, the carriage return of an item stays
    // where it was, and a list of CRLF lines keeps them.
    let report = run(concat!(
        "a := Sort(\"b`na`n\")",
        "\nb := Sort(\"b`na`r`n\")",
        "\nc := Sort(\"`n\")",
        "\nd := Sort(\"\")",
        "\ne := Sort(\"b`n`na\")",
        "\nf := Sort(\"b`r`na\")",
        "\ng := Sort(\"b`r`na`r`n\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("a\nb\n".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a\r\nb\n".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("\n".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("\na\nb".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("a\r\nb".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("a\r\nb\r\n".into()));
}

#[test]
fn sort_calls_the_callback_with_the_two_items_and_the_distance_between_them() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the third parameter
    // is a comparison function of three parameters, and the sign of the
    // number it answers is the order of the two items.
    let report = run(concat!(
        "ByLength(a, b, offset) {\n",
        "    return StrLen(a) - StrLen(b)\n",
        "}\n",
        "a := Sort(\"ccc`na`nbb\", \"\", ByLength)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("a\nbb\nccc".into()));
}

#[test]
fn sort_refuses_a_third_parameter_that_is_not_a_function() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a value that is not
    // an object is refused, and an object is asked for its `Call` method.
    let report = run("a := Sort(\"b`na\", \"\", 5)");
    assert_eq!(failure_of(&report), "Parameter #3 of Sort is invalid.");
    let report = run("a := Sort(\"b`na\", \"\", [1])");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Array\" has no method named \"Call\"."
    );
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
fn the_case_sense_is_a_number_or_a_name() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the sense of the case
    // is a whole number -- 1 takes the case into the compare and 0 leaves it
    // out -- or one of the names "On", "Off", "Locale" and "Logical", and a
    // float or another number is not a value that the parameter can take.
    let report = run(concat!(
        "a := StrCompare(\"a\", \"A\", \"On\")",
        "\nb := StrReplace(\"aA\", \"a\", \"b\", \"On\")",
        "\nc := InStr(\"a\", \"a\", \"On\")",
        "\nd := StrCompare(\"A\", \"a\", \"Off\")",
        "\ne := InStr(\"A\", \"a\", \"0\")",
        "\nf := StrCompare(\"a\", \"A\", \"Logical\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Text("bA".into()));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(1));
    assert_eq!(value_of(&report, "f"), Value::Int(0));
    for (source, message) in [
        (
            "a := StrCompare(\"a\", \"A\", 3)",
            "Parameter #3 of StrCompare is invalid.",
        ),
        (
            "a := StrReplace(\"a\", \"a\", \"b\", 3)",
            "Parameter #4 of StrReplace is invalid.",
        ),
        (
            "a := InStr(\"a\", \"a\", 1.0)",
            "Parameter #3 of InStr is invalid.",
        ),
        (
            "a := InStr(\"a\", \"a\", \"Logical\")",
            "Parameter #3 of InStr is invalid.",
        ),
        (
            "a := StrReplace(\"a\", \"a\", \"b\", \"Logical\")",
            "Parameter #4 of StrReplace is invalid.",
        ),
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
    // Measured on the binary of v2.0.19 on 2026-11-09: `{{` is a literal
    // brace only when a `}` follows it, so "{{1}}" is the literal brace, the
    // number one and the closing brace, and the placeholder of the second
    // argument is what is filled.
    assert_eq!(value_of(&report, "b"), Value::Text("{2}".into()));
    // The default is the text of the value, and the precision cuts it.
    assert_eq!(value_of(&report, "c"), Value::Text("2.".into()));
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
fn format_reads_the_specifier_the_way_the_binary_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09. The specifier is the
    // one of the C library of the binary: the flags `-+0 #`, a width, a
    // point with a precision, and one type character.
    let report = run(concat!(
        "a := Format(\"{:05d}\", 42)",
        "\nb := Format(\"{:x}\", 255)",
        "\nc := Format(\"{:X}\", 255)",
        "\nd := Format(\"{:o}\", 8)",
        "\ne := Format(\"{:#x}\", 255)",
        "\nf := Format(\"{:#o}\", 8)",
        "\ng := Format(\"{:+d}\", 42)",
        "\nh := Format(\"{: d}\", 42)",
        "\ni := Format(\"{:08d}\", -42)",
        "\nj := Format(\"{:x}\", -1)",
        "\nk := Format(\"{:u}\", -1)",
        "\nl := Format(\"{:.5d}\", 42)",
        "\nm := Format(\"{:.0d}\", 0)",
        "\nn := Format(\"{:-6d}\", 42)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("00042".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("ff".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("FF".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("10".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("0xff".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("010".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("+42".into()));
    assert_eq!(value_of(&report, "h"), Value::Text(" 42".into()));
    assert_eq!(value_of(&report, "i"), Value::Text("-0000042".into()));
    assert_eq!(
        value_of(&report, "j"),
        Value::Text("ffffffffffffffff".into())
    );
    assert_eq!(
        value_of(&report, "k"),
        Value::Text("18446744073709551615".into())
    );
    assert_eq!(value_of(&report, "l"), Value::Text("00042".into()));
    assert_eq!(value_of(&report, "m"), Value::Text("".into()));
    assert_eq!(value_of(&report, "n"), Value::Text("42    ".into()));
}

#[test]
fn format_reads_the_specifier_of_a_float_the_way_the_binary_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09.
    let report = run(concat!(
        "a := Format(\"{:.2f}\", 2.345)",
        "\nb := Format(\"{:8.2f}\", 3.14159)",
        "\nc := Format(\"{:05.2f}\", 3.14159)",
        "\nd := Format(\"{:e}\", 1234.5)",
        "\ne := Format(\"{:E}\", 1234.5)",
        "\nf := Format(\"{:.2e}\", 1234.5)",
        "\ng := Format(\"{:g}\", 1234.5)",
        "\nh := Format(\"{:g}\", 0.00001234)",
        "\ni := Format(\"{:g}\", 1000000)",
        "\nj := Format(\"{:g}\", 0)",
        "\nk := Format(\"{:#g}\", 1.5)",
        "\nl := Format(\"{:a}\", 1.5)",
        "\nm := Format(\"{:A}\", 1.5)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2.35".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("    3.14".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("03.14".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("1.234500e+03".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("1.234500E+03".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("1.23e+03".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("1234.5".into()));
    assert_eq!(value_of(&report, "h"), Value::Text("1.234e-05".into()));
    assert_eq!(value_of(&report, "i"), Value::Text("1e+06".into()));
    assert_eq!(value_of(&report, "j"), Value::Text("0".into()));
    assert_eq!(value_of(&report, "k"), Value::Text("1.50000".into()));
    assert_eq!(
        value_of(&report, "l"),
        Value::Text("0x1.8000000000000p+0".into())
    );
    assert_eq!(
        value_of(&report, "m"),
        Value::Text("0X1.8000000000000P+0".into())
    );
}

#[test]
fn format_reads_the_default_and_the_text_specifiers() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the default is the
    // text of the value, and its precision cuts the text.
    let report = run(concat!(
        "a := Format(\"{}\", 42)",
        "\nb := Format(\"{}\", 2.5)",
        "\nc := Format(\"{}\", 1/3)",
        "\nd := Format(\"{:s}\", 42)",
        "\ne := Format(\"{:5s}\", \"ab\")",
        "\nf := Format(\"{:05s}\", \"ab\")",
        "\ng := Format(\"{:.2}\", 2.345)",
        "\nh := Format(\"{:c}\", 65)",
        "\ni := Format(\"{:p}\", 255)",
        "\nj := Format(\"{:U}\", \"abc\")",
        "\nk := Format(\"{:L}\", \"ABC\")",
        "\nl := Format(\"{:T}\", \"hello world\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("42".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("2.5".into()));
    assert_eq!(
        value_of(&report, "c"),
        Value::Text("0.33333333333333331".into())
    );
    assert_eq!(value_of(&report, "d"), Value::Text("42".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("   ab".into()));
    assert_eq!(value_of(&report, "f"), Value::Text("000ab".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("2.".into()));
    assert_eq!(value_of(&report, "h"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "i"), Value::Text("00000000000000FF".into()));
    assert_eq!(value_of(&report, "j"), Value::Text("ABC".into()));
    assert_eq!(value_of(&report, "k"), Value::Text("abc".into()));
    assert_eq!(value_of(&report, "l"), Value::Text("Hello World".into()));
}

#[test]
fn format_reads_the_number_of_the_argument_of_the_binary() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the number of a
    // placeholder is one for the first argument, a placeholder with no
    // number takes the one after the last one named, and a placeholder that
    // names an argument that is not there is literal text.
    let report = run(concat!(
        "a := Format(\"{1}\", \"a\")",
        "\nb := Format(\"{} {1}\", \"a\", \"b\")",
        "\nc := Format(\"{2}\", \"a\", \"b\")",
        "\nd := Format(\"{0}\", \"a\")",
        "\ne := Format(\"{3}\", \"a\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("a".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a a".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("b".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("{0}".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("{3}".into()));
}

#[test]
fn format_leaves_a_placeholder_it_does_not_read_as_it_was_written() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a spec that is not
    // one of the C library makes the whole placeholder literal text.
    let report = run(concat!(
        "a := Format(\"{:b}\", 5)",
        "\nb := Format(\"{:<6d}\", 42)",
        "\nc := Format(\"{:z}\", 5)",
        "\nd := Format(\"{1.5}\", 5)",
        "\ne := Format(\"a{b\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("{:b}".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("{:<6d}".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("{:z}".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("{1.5}".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("a{b".into()));
}

#[test]
fn format_escapes_a_brace_the_way_the_binary_does() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `{{` is one brace
    // only when a `}` follows it, and `}}` is one brace too.
    let report = run(concat!(
        "a := Format(\"{{}}\", 5)",
        "\nb := Format(\"a{{b}}c\", 5)",
        "\nc := Format(\"{}}\", 5)",
        "\nd := Format(\"{{}\", 5)",
        "\ne := Format(\"}}\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("{}".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a{{b}}c".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("}".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("{".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("}}".into()));
}

#[test]
fn format_refuses_a_value_of_the_wrong_kind() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a specifier that
    // wants a number refuses a text that does not hold one, and the default
    // refuses an object. The number of the parameter is the one of the call,
    // the pattern included.
    let report = run("a := Format(\"{:d}\", \"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Format requires a Number, but received a String."
    );
    let report = run("a := Format(\"{2:d}\", 1, \"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #3 of Format requires a Number, but received a String."
    );
    let report = run("a := Format(\"{}\", Map())");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Format requires a String, but received a Map."
    );
}

#[test]
fn format_takes_the_number_a_text_holds_the_way_the_binary_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09: a text that holds a
    // whole number, a hex number, a fraction or an exponent is that number.
    let report = run(concat!(
        "a := Format(\"{:d}\", \"0x10\")",
        "\nb := Format(\"{:d}\", \"1e3\")",
        "\nc := Format(\"{:d}\", \"1.9\")",
        "\nd := Format(\"{:d}\", \"010\")",
        "\ne := Format(\"{:f}\", \"0x10\")",
        "\nf := Format(\"{:d}\", 1e30)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("16".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("1000".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("1".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("10".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("16.000000".into()));
    // The cast of the machine answers the smallest 64-bit integer for a
    // number that is out of its range.
    assert_eq!(
        value_of(&report, "f"),
        Value::Text("-9223372036854775808".into())
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
fn in_str_reads_a_sense_of_the_case_by_its_name() {
    // Measured on the binary of v2.0.19 on 2026-11-09: "On" and "Off" are
    // taken as they are written, whatever the case of the letters, and
    // "Locale" looks at the letters through the locale.
    let report = run(concat!(
        "a := InStr(\"hello world\", \"o\", \"Off\", 6)",
        "\nb := InStr(\"hello world\", \"O\", \"On\")",
        "\nc := InStr(\"hello world\", \"O\", \"Off\")",
        "\nd := InStr(\"hello world\", \"O\", \"Locale\")",
        "\ne := InStr(\"hello\", \"l\", \"ON\")",
        "\nf := InStr(\"hello\", \"l\", \"oFf\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(8));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(5));
    assert_eq!(value_of(&report, "d"), Value::Int(5));
    assert_eq!(value_of(&report, "e"), Value::Int(3));
    assert_eq!(value_of(&report, "f"), Value::Int(3));
}

#[test]
fn in_str_reads_a_place_from_the_right() {
    // Measured on the binary of v2.0.19 on 2026-11-09, and read out of the
    // `BIF_InStr` of the source: a negative place is `len + 1 + place` and
    // the search runs from the right; when the occurrence is not given with
    // it, the search from the right is the default.
    let report = run(concat!(
        "a := InStr(\"hello\", \"l\", \"On\", -1)",
        "\nb := InStr(\"hello\", \"l\", \"On\", -2)",
        "\nc := InStr(\"hello\", \"h\", \"On\", -5)",
        "\nd := InStr(\"hello\", \"l\", \"On\", -6)",
        "\ne := InStr(\"abcabc\", \"b\", \"On\", -1, -2)",
        "\nf := InStr(\"abcabc\", \"b\", \"On\", 1, -1)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(4));
    assert_eq!(value_of(&report, "b"), Value::Int(4));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(2));
    assert_eq!(value_of(&report, "f"), Value::Int(0));
}

#[test]
fn in_str_counts_the_matches_without_overlapping_them() {
    // Measured on the binary of v2.0.19 on 2026-11-09: after a match the
    // search from the left goes on after it, so the matches do not overlap.
    let report = run(concat!(
        "a := InStr(\"aaaa\", \"aa\", \"On\", 1, 1)",
        "\nb := InStr(\"aaaa\", \"aa\", \"On\", 1, 2)",
        "\nc := InStr(\"aaaa\", \"aa\", \"On\", 1, 3)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Int(3));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
}

#[test]
fn the_place_of_the_search_wants_a_number_and_is_not_zero() {
    // Measured on the binary of v2.0.19 on 2026-11-09.
    let report = run("a := InStr(\"hello\", \"l\", \"On\", 0)");
    assert_eq!(failure_of(&report), "Parameter #4 of InStr is invalid.");
    let report = run("a := InStr(\"hello\", \"l\", \"On\", \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of InStr requires a Number, but received a String."
    );
    // A number written as a text is a number, and a float is cut.
    let report = run(concat!(
        "a := InStr(\"hello\", \"l\", \"On\", \"4\")",
        "\nb := InStr(\"hello\", \"l\", \"On\", 4.9)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(4));
    assert_eq!(value_of(&report, "b"), Value::Int(4));
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
    // Measured on the binary of v2.0.19 on 2026-11-09: a capacity empties
    // the string and answers the capacity the variable has, and a variable
    // that is never asked for one answers the size class of its text --
    // three for "abc", sixty-three for ten characters.
    let report = run(concat!(
        "x := \"abc\"",
        "\na := VarSetStrCapacity(&x, 100)",
        "\ny := \"abc\"",
        "\nb := VarSetStrCapacity(&y)",
        "\nz := \"abcdefghij\"",
        "\nc := VarSetStrCapacity(&z)",
        "\nw := \"hello\"",
        "\nd := VarSetStrCapacity(&w, 0)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 100);
    // The string is emptied when a capacity is given.
    assert_eq!(value_of(&report, "x"), Value::Text(String::new()));
    // Without a capacity the capacity of the text is the answer, and the
    // string is left alone.
    assert_eq!(int_of(&report, "b"), 3);
    assert_eq!(value_of(&report, "y"), Value::Text("abc".into()));
    assert_eq!(int_of(&report, "c"), 63);
    // A zero empties the string, and the capacity of a text that was never
    // asked for is not given back: it is the size class of the text.
    assert_eq!(int_of(&report, "d"), 7);
    assert_eq!(value_of(&report, "w"), Value::Text(String::new()));
}

#[test]
fn var_set_str_capacity_reads_the_capacity_the_way_the_binary_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09: minus one answers
    // the length of the text and leaves the text alone; a negative
    // capacity that is not minus one is an invalid parameter; a capacity
    // that was given is given back by a zero.
    let report = run(concat!(
        "v := \"hello\"",
        "\na := VarSetStrCapacity(&v, -1)",
        "\nb := VarSetStrCapacity(&v, 64)",
        "\nc := VarSetStrCapacity(&v, 0)",
        "\nd := VarSetStrCapacity(&v)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 5);
    assert_eq!(value_of(&report, "v"), Value::Text(String::new()));
    assert_eq!(int_of(&report, "b"), 64);
    assert_eq!(int_of(&report, "c"), 0);
    assert_eq!(int_of(&report, "d"), 0);
    let report = run("a := VarSetStrCapacity(&v, -5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of VarSetStrCapacity is invalid."
    );
}

#[test]
fn str_ptr_answers_an_address_of_a_text() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the address of a
    // text is a number above zero, the same text answers the same address,
    // and a number, an object and a reference are refused with their kind.
    let report = run(concat!(
        "v := \"hello\"",
        "\na := StrPtr(v)",
        "\nb := StrPtr(v)",
        "\nc := StrPtr(\"hello\")",
        "\nd := Type(StrPtr(\"\"))",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(int_of(&report, "a") > 0);
    assert_eq!(int_of(&report, "a"), int_of(&report, "b"));
    assert_eq!(value_of(&report, "d"), Value::Text("Integer".into()));
    let report = run("a := StrPtr(5)");
    assert_eq!(
        failure_of(&report),
        "Expected a String but got an Integer."
    );
    let report = run("v := 1\na := StrPtr(&v)");
    assert_eq!(failure_of(&report), "Expected a String but got a VarRef.");
}

#[test]
fn str_put_writes_the_text_of_a_buffer_and_str_get_reads_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the bytes of the
    // text and the terminator are the answer of a write, the encoding of a
    // call that does not name one is UTF-16, and a target that cannot hold
    // the text is refused.
    let report = run(concat!(
        "buf := Buffer(32)",
        "\na := StrPut(\"abc\", buf, \"UTF-8\")",
        "\nb := StrGet(buf, \"UTF-8\")",
        "\nc := StrPut(\"abc\", \"UTF-8\")",
        "\nd := StrPut(\"abc\")",
        "\ne := StrPut(\"abc\", buf, \"UTF-16\")",
        "\nf := StrGet(buf, \"UTF-16\")",
        "\ng := StrGet(buf)",
        "\nh := StrGet(buf, 2, \"UTF-16\")",
        "\ni := StrGet(buf, \"utf-8\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 4);
    assert_eq!(value_of(&report, "b"), Value::Text("abc".into()));
    assert_eq!(int_of(&report, "c"), 4);
    assert_eq!(int_of(&report, "d"), 8);
    assert_eq!(int_of(&report, "e"), 8);
    assert_eq!(value_of(&report, "f"), Value::Text("abc".into()));
    assert_eq!(value_of(&report, "g"), Value::Text("abc".into()));
    assert_eq!(value_of(&report, "h"), Value::Text("ab".into()));
    // The bytes of a UTF-16 text read as UTF-8 end at the zero of the
    // first unit: the "a" and nothing else.
    assert_eq!(value_of(&report, "i"), Value::Text("a".into()));
}

#[test]
fn the_string_functions_refuse_what_v2_refuses() {
    // Measured on the binary of v2.0.19 on 2026-11-09.
    let report = run("buf := Buffer(16)\na := StrPut(\"abc\", buf, \"UTF-99\")");
    assert_eq!(failure_of(&report), "Invalid Encoding.");
    let report = run("a := StrGet(123)");
    assert_eq!(failure_of(&report), "Parameter #1 of StrGet is invalid.");
    let report = run("a := StrGet(\"abc\")");
    assert_eq!(failure_of(&report), "Parameter #1 invalid.");
    let report = run("a := StrPut(\"abc\", 123)");
    assert_eq!(failure_of(&report), "Parameter #2 of StrPut is invalid.");
    let report = run("buf := Buffer(2)\na := StrPut(\"abcdef\", buf, \"UTF-8\")");
    assert_eq!(failure_of(&report), "Parameter #2 of StrPut is invalid.");
    let report = run("buf := Buffer(16)\na := StrPut(\"abc\", buf, 3)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 6);
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
    let report =
        run("a := SubStr(\"hello\", 2)\nb := SubStr(\"hello\", 2, 2)\nc := SubStr(\"hello\", -2)");
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
fn the_class_of_a_number_converts_its_argument() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the classes of the
    // numbers and of the text are the conversions of the dialect, and the
    // classes themselves stay what they are.
    let report = run(concat!(
        "a := Integer(\"0x1F\")",
        "\nb := Integer(3.9)",
        "\nc := Integer(-3.9)",
        "\nd := Integer(\" 5 \")",
        "\ne := Integer(\"1e3\")",
        "\nf := Integer(\"010\")",
        "\ng := Integer(true)",
        "\nh := Float(\"1.5\")",
        "\ni := Float(2)",
        "\nj := Float(\"0x10\")",
        "\nk := Float(\"1e3\")",
        "\nl := String(42)",
        "\nm := String(1.5)",
        "\nn := String(true)",
        "\no := String(\"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(31));
    assert_eq!(value_of(&report, "b"), Value::Int(3));
    assert_eq!(value_of(&report, "c"), Value::Int(-3));
    assert_eq!(value_of(&report, "d"), Value::Int(5));
    assert_eq!(value_of(&report, "e"), Value::Int(1000));
    assert_eq!(value_of(&report, "f"), Value::Int(10));
    assert_eq!(value_of(&report, "g"), Value::Int(1));
    assert_eq!(value_of(&report, "h"), Value::Float(1.5));
    assert_eq!(value_of(&report, "i"), Value::Float(2.0));
    assert_eq!(value_of(&report, "j"), Value::Float(16.0));
    assert_eq!(value_of(&report, "k"), Value::Float(1000.0));
    assert_eq!(value_of(&report, "l"), Value::Text("42".into()));
    assert_eq!(value_of(&report, "m"), Value::Text("1.5".into()));
    assert_eq!(value_of(&report, "n"), Value::Text("1".into()));
    assert_eq!(value_of(&report, "o"), Value::Text("x".into()));
}

#[test]
fn the_class_of_a_number_is_still_a_class() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the conversion lives
    // in the call of the class, and the value of the name is the class.
    let report = run(concat!(
        "a := Type(Integer)",
        "\nb := Type(Float)",
        "\nc := Type(String)",
        "\nd := Type(Integer.Prototype)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("Prototype".into()));
}

#[test]
fn a_conversion_refuses_a_value_of_the_wrong_kind() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the name of the call
    // is "Integer.Call" and the value goes in the `Extra` of the failure.
    let report = run("a := Integer(\"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Integer.Call requires a Number, but received a String."
    );
    let report = run("a := Float(\"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Float.Call requires a Number, but received a String."
    );
    // The number of the arguments is the one of the class: one.
    let report = run("a := Integer()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function."
    );
    let report = run("a := Integer(1, 2)");
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function."
    );
}

#[test]
fn the_text_of_an_object_asks_for_its_to_string() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `String` of an
    // object calls its `ToString` method, and one that has none says so.
    let report = run("a := String(Map())");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Map\" has no method named \"ToString\"."
    );
    let report = run(concat!(
        "class WithStr {\n",
        "    ToString() {\n",
        "        return \"made\"\n",
        "    }\n",
        "}\n",
        "a := String(WithStr())",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("made".into()));
}

#[test]
fn number_keeps_the_shape_the_number_was_written_with() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `Number("0x10")` is
    // the integer 16 and `Number("1.5")` the float 1.5.
    let report = run(concat!(
        "a := Number(\"0x10\")",
        "\nb := Number(\"1.5\")",
        "\nc := Number(3.9)",
        "\nd := Type(Number(\"0x10\"))",
        "\ne := Type(Number(\"1.5\"))",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(16));
    assert_eq!(value_of(&report, "b"), Value::Float(1.5));
    assert_eq!(value_of(&report, "c"), Value::Float(3.9));
    assert_eq!(value_of(&report, "d"), Value::Text("Integer".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("Float".into()));
}

#[test]
fn the_type_of_an_object_is_the_name_of_its_class() {
    let report = run("class Box {\n}\nb := Box()\n\
         a := Type(b)\nc := Type(Box)\nd := Type(Map())\ne := Type(Box.Prototype)");
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
fn regex_match_writes_the_matches_into_the_output_variable() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the third parameter
    // is the variable that takes the `RegExMatchInfo`, and the number of the
    // groups, the text of the groups and their places are read from it with
    // `Count`, the item and `Len`/`Pos`.
    let report = run(concat!(
        "p := RegExMatch(\"abc123def\", \"(\\d+)\", &m)\n",
        "q := RegExMatch(\"abc123def\", \"(?<year>\\d+)\", &named)\n",
        "r := RegExMatch(\"zzz\", \"(\\d+)\", &miss)\n",
        "FileAppend(p \"|\" m[0] \"|\" m[1] \"|\" m.Count \"|\" m.Len[0] \"|\" m.Pos[0] \"|\" ",
        "named[\"year\"] \"|\" named.Name[1] \"|\" r \"|\" Type(miss), \"*\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The output variable of the miss is the empty string, not unset.
    assert_eq!(report.output, vec!["4|123|123|1|3|4|123|year|0|String"]);
}

#[test]
fn the_members_of_a_match_answer_the_way_v2_answers_them() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `Len`, `Pos` and
    // `Name` take the place or the name of a group between the brackets and
    // answer the whole match without one, `Mark` is the mark of the pattern
    // -- which the engine of the tree does not carry, so it is empty -- and
    // a group that took no part is the empty text at the place zero.
    let report = run(concat!(
        "m := \"\"\n",
        "RegExMatch(\"b\", \"(a)?(b)\", &m)\n",
        "n := \"\"\n",
        "RegExMatch(\"abc123def\", \"(?<year>\\d+)(?<word>def)\", &n)\n",
        "a := m.Count\n",
        "b := m[1] \"|\" m[2] \"|\" m.Len[1] \"|\" m.Pos[1]\n",
        "c := n.Len[\"year\"] \"|\" n.Pos[\"year\"] \"|\" n.Name[1] \"|\" n.Name[2] \"|\" n.Name[0]\n",
        "d := n.Len \"|\" n.Pos \"|\" n.Mark\n",
        "e := n.Len() \"|\" n.Pos(0)\n",
        "f := Type(n) \"|\" n.__Class\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 2);
    assert_eq!(value_of(&report, "b"), Value::Text("|b|0|0".into()));
    assert_eq!(
        value_of(&report, "c"),
        Value::Text("3|4|year|word|".into())
    );
    assert_eq!(value_of(&report, "d"), Value::Text("6|4|".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("6|4".into()));
    assert_eq!("RegExMatchInfo|RegExMatchInfo", value_of(&report, "f").to_text());
}

#[test]
fn the_members_of_a_match_refuse_what_v2_refuses() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the name of a group
    // that is not there is the invalid first parameter of the member that
    // was read, and a place that is not one is the same failure.
    let report = run(concat!(
        "m := \"\"\n",
        "RegExMatch(\"abc123\", \"(?<year>\\d+)\", &m)\n",
        "a := m.Len[\"nope\"]\n",
    ));
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of RegExMatchInfo.Prototype.Len.Get is invalid."
    );
    let report = run(concat!(
        "m := \"\"\n",
        "RegExMatch(\"abc123\", \"(\\d+)\", &m)\n",
        "a := m[5]\n",
    ));
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of RegExMatchInfo.Prototype.__Item.Get is invalid."
    );
    let report = run("a := RegExMatch(\"a\", \"a\", 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #3 of RegExMatch requires a variable reference, but received an Integer."
    );
}

#[test]
fn a_match_is_walked_by_the_groups_of_it() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the keys of the walk
    // are the places of the groups, the named ones under their names, and
    // the values are the texts -- a group that took no part is empty.
    let report = run(concat!(
        "m := \"\"\n",
        "RegExMatch(\"a1\", \"(?<d>\\d+)(x)?\", &m)\n",
        "pairs := \"\"\n",
        "for k, v in m\n",
        "    pairs .= \"[\" k \"=\" v \"]\"\n",
        "values := \"\"\n",
        "for v in m\n",
        "    values .= v\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "pairs"), Value::Text("[0=1][d=1][2=]".into()));
    assert_eq!(value_of(&report, "values"), Value::Text("11".into()));
}

#[test]
fn a_match_is_searched_from_the_place_the_caller_names() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the place counts
    // from one, a place that is zero or negative counts from the end of the
    // text, and a place past the end finds nothing.
    let report = run(concat!(
        "a := 0\n",
        "p1 := RegExMatch(\"abc123\", \"(\\d+)\", &a, 3)\n",
        "v1 := a[0]\n",
        "b := 0\n",
        "p2 := RegExMatch(\"abcabc\", \"b\", &b, -3)\n",
        "v2 := b[0]\n",
        "p3 := RegExMatch(\"abcabc\", \"b\", &c, 0)\n",
        "p4 := RegExMatch(\"abcabc\", \"b\", &d, 100)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "p1"), 4);
    assert_eq!(value_of(&report, "v1"), Value::Text("123".into()));
    assert_eq!(int_of(&report, "p2"), 5);
    assert_eq!(value_of(&report, "v2"), Value::Text("b".into()));
    assert_eq!(int_of(&report, "p3"), 0);
    assert_eq!(int_of(&report, "p4"), 0);
    // A place that is not a number is refused with the parameter.
    let report = run("a := RegExMatch(\"a\", \"a\", &m, \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of RegExMatch requires a Number, but received a String."
    );
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
    let mut desktop = Fake {
        clipboard: "the text".to_string(),
        ..Default::default()
    };
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
    let mut desktop = Fake {
        clipboard: "the text".to_string(),
        ..Default::default()
    };
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
    assert_eq!(
        failure_of(&report),
        "this desktop cannot read the clipboard"
    );
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
            "Persistent",
            "Persistent",
            "this runtime is a script that ends when it ends, and it cannot be kept alive.",
        ),
        (
            "Reload",
            "Reload",
            "this runtime runs one script once, and it cannot run itself again.",
        ),
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
fn the_failure_of_an_omitted_parameter_carries_the_name_of_the_built_in() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the failure of a
    // parameter that may not be left out carries the name of the built-in
    // in the `What`, and an empty `Extra`.
    //
    // The tree carried an empty `What` there, and the case of it pinned
    // the tree and not the binary: the name of the built-in goes in the
    // `What` of every failure of a built-in.
    let report = run(concat!(
        "try\n",
        "  FileGetAttrib()\n",
        "catch as e\n",
        "  MsgBox e.What \"|\" e.Extra \"|\" e.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["FileGetAttrib||Parameter #1 of FileGetAttrib must not be omitted in this case."]
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
    let report =
        run("OnExit Bye\nBye(Reason, Code) {\n  MsgBox NoSuchVariable\n}\nMsgBox NoSuchVariable\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("the function of the exit failed")),
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
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: OnExit"
    );
}

#[test]
fn the_order_of_the_callbacks_is_one_minus_one_or_zero() {
    let report = run("OnExit Bye, 5\nBye(Reason, Code) {\n}\n");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of OnExit is invalid.\n     Specifically: 5"
    );
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
        report
            .notes
            .iter()
            .any(|note| note.contains("keep the script alive")),
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
        report
            .notes
            .iter()
            .any(|note| note.contains("hid the failure")),
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
    let report = run_on(
        "A_Clipboard := \"the new text\"\nMsgBox A_Clipboard",
        &mut desktop,
    );
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
    let report = run_on(
        "DllCall(\"f\", \"Int\", 5, \"Str\", \"hello\")",
        &mut desktop,
    );
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
        answer: Some(NativeResult {
            kind: NativeType::Int,
            value: NativeValue::Integer(42),
        }),
        ..Default::default()
    };
    let report = run_on("a := DllCall(\"f\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(42));
}

#[test]
fn a_star_at_the_end_of_a_type_is_the_address_of_a_variable() {
    // The fake writes into the memory of the address, the way the box does.
    let mut desktop = Fake {
        writes: vec![42.0],
        ..Default::default()
    };
    let report = run_on(
        "counter := 0\nDllCall(\"f\", \"Int64*\", &counter)",
        &mut desktop,
    );
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
        assert_eq!(
            report.output,
            vec!["Invalid base. | ObjSetBase | "],
            "the base of {source}"
        );
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
    assert_eq!(
        failure_of(&report),
        "this desktop cannot make a native call."
    );
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
    assert_eq!(
        report.output,
        vec!["Parameter #1 of Chr is invalid. | Chr | -1"]
    );
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
    let report = run("try {\n    x := Chr(-1)\n} catch Any as e {\n    MsgBox e is ValueError\n}");
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
    let report = run_on(
        "InputBox(\"the prompt\", \"the title\", \"T10\", \"the default\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![(
            "input-box".to_string(),
            "the prompt|the title|the default|Some(10.0)".to_string()
        )]
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
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | InputBox | z9"]
    );
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
    assert_eq!(
        report.output,
        vec!["Parameter #4 of FileSelect is invalid. | Text (*.txt)"]
    );
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
        vec![
            "Parameter #2 of DirSelect requires a Number, but received a String. | TypeError | DirSelect | b"
        ]
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
    let report = run("z := ImageSearch(0, 0, 0, 0, 10, 10, \"the.png\")\n");
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
        Some(
            "the picture is read by the desktop, and this desktop does not read the images of the machine."
        )
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
        vec![(
            "download".to_string(),
            "https://example.com/file.txt|/tmp/file.txt".to_string()
        )]
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
    let statements =
        parse("Download(\"https://example.com/file.txt\", \"/tmp/file.txt\")").expect("parses");
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
    assert_eq!(
        failure_of(&report),
        "Expected an Object but got an empty string."
    );
}

#[test]
fn a_name_that_the_tree_does_not_carry_says_the_reason_of_the_machine_when_the_kind_fits() {
    let report = run("a := StatusBarGetText(1)");
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
            vec![format!(
                "Parameter #{number} of {name} is invalid. | {name} | empty string"
            )],
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

#[test]
fn the_commands_of_the_files_answer_with_an_empty_text() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the answer of the
    // `DirCreate` and of the `FileDelete` is an empty text, and not the
    // name of what they did.
    let report = run("x := DirCreate(\"made\")\nMsgBox \"[\" x \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
    let report = run("x := FileDelete(\"a.txt\")\nMsgBox \"[\" x \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_name_of_the_built_in_is_in_the_failure_of_a_parameter_that_may_not_be_left_out() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the failure of a
    // parameter that may not be left out carries the name of the built-in
    // in the `What`.
    let report = run(
        "try {\n    x := FileSetTime(\"20200101000000\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileSetTime must not be omitted in this case. | FileSetTime"]
    );
}

#[test]
fn the_failure_of_a_copy_carries_the_number_of_the_files_that_failed() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the copy of a file
    // that is not there fails with "Failed", and the `Extra` of the failure
    // is the number of the files that failed.
    let report = run(
        "try {\n    x := FileCopy(\"nope.txt\", \"b.txt\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Failed | FileCopy | 1"]);
}

#[test]
fn the_wildcards_of_the_delete_are_not_carried() {
    // The name of a file of a delete may be a pattern of v2: the files that it
    // names are taken away, and the answer is the empty text. Measured on the
    // binary of v2.0.19 on 2026-11-09.
    let mut desktop = Fake::default();
    let report = run_on("x := FileDelete(\"*.txt\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "x"), Value::Text(String::new()));
    let deletes: Vec<&(String, String)> = desktop
        .calls
        .iter()
        .filter(|(what, _)| what == "delete-file")
        .collect();
    assert_eq!(deletes.len(), 2, "{:?}", desktop.calls);
}
/// The class that the cases of the binding use: a method that takes one
/// argument and one that takes two.
const THE_THING: &str = "class Thing {\n    __New() {\n        this.v := 42\n    }\n    Method(x) {\n        return this.v + x\n    }\n    Two(x, y) {\n        return x + y\n    }\n}\n";

/// The `ObjBindMethod` binds the method of an object.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the value is
/// of the class `BoundFunc`, and the call of it calls the method of the object
/// with the object as its `this`. The name of the method is not checked when
/// the value is made: the `ObjBindMethod(t, "Nope")` is made, and the call of
/// it fails.
#[test]
fn the_obj_bind_method_binds_the_method_of_the_object() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nx := f(8)"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 50);
}

/// The value of a binding is of the class `BoundFunc`, and its name is empty.
#[test]
fn the_value_of_a_binding_is_of_the_class_bound_func() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nthe_class := f.__Class\nthe_name := f.Name\nthe_minimum := f.MinParams"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "the_class"),
        Value::Text("BoundFunc".into())
    );
    assert_eq!(value_of(&report, "the_name"), Value::Text("".into()));
    assert_eq!(int_of(&report, "the_minimum"), 0);
}

/// The name of the method is not checked when the binding is made.
#[test]
fn the_obj_bind_method_does_not_check_the_name_of_the_method() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Nope\")"
    ));
    assert!(report.ok, "{:?}", report.error);

    // The failure comes when the value is called.
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Nope\")\nf()"
    ));
    assert_eq!(
        failure_of(&report),
        "This value of type \"Thing\" has no method named \"Nope\"."
    );
}

/// The call of a binding may go through the `Call` method of it.
#[test]
fn the_call_of_a_binding_may_go_through_the_call_method() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nx := f.Call(8)"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 50);
}

/// The binding of a method of a class type of the runtime.
#[test]
fn the_obj_bind_method_binds_the_method_of_a_class_type() {
    let report =
        run("m := Map()\nm[\"a\"] := 1\nf := ObjBindMethod(m, \"Clear\")\nf()\nn := m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "n"), 0);
}

#[test]
fn the_title_of_a_text_keeps_the_words_of_the_whitespace() {
    // Measured on the binary of v2.0.19 on 2026-11-10: only the whitespace
    // begins a word -- `hello-world` is `Hello-world` and `ab-cd` is `Ab-cd`
    // -- and the first letter of the word is the one in the upper case, so
    // the digits in front of it keep the word waiting (`123abc` is `123Abc`)
    // while the digits inside it do not (`abc123def` is `Abc123def`).
    let report = run(concat!(
        "MsgBox StrTitle(\"hello-world 3rd o'neil\")\n",
        "MsgBox StrTitle(\"mIxEd`tcase`nnext\")\n",
        "MsgBox StrTitle(\"123abc\") \"|\" StrTitle(\"abc123def\") \"|\" StrTitle(\"AB-CD\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Hello-world 3Rd O'neil", "Mixed\tCase\nNext", "123Abc|Abc123def|Ab-cd"]
    );
}

#[test]
fn the_named_codes_of_the_format_of_a_time_are_the_whole_of_the_format() {
    // Measured on the binary of v2.0.19 on 2026-11-10: `WDay` is the number of
    // the day of the week, `YDay` the number of the day of the year and
    // `YWeek` the year and the week of it, as four digits and two; the name is
    // read with any case and without its leading whitespace, and a name that
    // is a part of a longer format is read letter by letter.
    let report = run(concat!(
        "MsgBox FormatTime(\"20200704\", \"WDay\") \" \" FormatTime(\"20200704\", \"WDay\")\n",
        "MsgBox FormatTime(\"20200704\", \"YDay\") \" \" FormatTime(\"20200704\", \" wday\")\n",
        "MsgBox FormatTime(\"20200704\", \"YWeek\")\n",
        "MsgBox FormatTime(\"20200704\", \"A_WDay_B\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7 7", "186 7", "202027", "A_WDa20_B"]);
}
#[test]
fn the_failure_of_mod_is_a_type_error_while_the_same_words_of_fileopen_are_a_value_error() {
    // Measured on the binary of v2.0.19 on 2026-11-10: the class of a failure
    // is not the class of its message alone -- the "Invalid parameter(s)." of
    // a `Mod` is a `TypeError`, and the same words of a `FileOpen` are a
    // `ValueError`.
    let report = run(concat!(
        "try\n",
        "    Mod(\"abc\", 2)\n",
        "catch as e\n",
        "    MsgBox Type(e) \"|\" e.Message \"|\" e.What\n",
        "try\n",
        "    FileOpen(\"nothing.txt\", \"z\")\n",
        "catch as e\n",
        "    MsgBox Type(e) \"|\" e.Message \"|\" e.What\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "TypeError|Invalid parameter(s).|Mod",
            "ValueError|Invalid parameter(s).|FileOpen",
        ]
    );
}
#[test]
fn the_absent_value_is_no_argument_to_the_builtin() {
    // Measured on the binary of v2.0.19 on 2026-11-10: `StrReplace` writes the
    // number of the swaps into the fifth parameter, the limit is the sixth, the
    // fourth is the sense of the case, and an argument that is `unset` -- or a
    // hole between two commas -- is no argument at all: the case is the one of
    // the dialect, the number is written, and the limit is all of the swaps.
    let report = run(concat!(
        "n := 0\n",
        "y := StrReplace(\"a,b,c,d,e,f\", \",\", \".\", unset, &n)\n",
        "MsgBox y \" \" n\n",
        "m := 0\n",
        "y := StrReplace(\"a,b,c,d,e,f\", \",\", \".\", , &m, 3)\n",
        "MsgBox y \" \" m\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["a.b.c.d.e.f 5", "a.b.c.d,e,f 3"]
    );
}
#[test]
fn a_text_is_counted_and_cut_in_the_code_units_of_utf16() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a surrogate code unit
    // comes back as it is -- `StrLen(Chr(0xD83D))` is one and `Ord` of it is
    // 0xD83D -- a character above the plane of BMP is two units and its code
    // point is the code of it, and the second unit of the pair is a text of its
    // own: `Ord(SubStr(Chr(0x1F600), 2))` is 0xDE00.
    let report = run(concat!(
        "MsgBox StrLen(Chr(0xD83D)) \" \" StrLen(Chr(0x1F600))\n",
        "MsgBox Ord(Chr(0xD83D)) \" \" Ord(Chr(0x1F600)) \" \" Ord(SubStr(Chr(0x1F600), 2))\n",
        "MsgBox StrLen(\"hello\") \" \" Ord(\"\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["1 2", "55357 128512 56832", "5 0"]
    );
}