//! The `File` class, against a fake desktop.
//!
//! The fake keeps its files in memory, so these cases are about what
//! the class means and not about what the disk does. The box serves
//! the same methods, and the box is checked by hand.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures, Pointer, Processes,
    Screen, Sound, Timers, Windows,
};
use ahk::runtime::{Report, Runtime};
use ahk::parse;
use std::collections::BTreeMap;
use std::time::Duration;

/// A desktop whose files are in memory.
#[derive(Debug, Default)]
struct Fake {
    /// The files of the fake, by path.
    disk: BTreeMap<String, Vec<u8>>,
    /// The files that are open, by handle.
    open: BTreeMap<i64, String>,
    /// The handle the next file gets.
    next: i64,
}

impl Fake {
    /// A desktop with one file on it.
    fn with_file(path: &str, text: &str) -> Fake {
        Fake::with_bytes(path, text.as_bytes())
    }

    /// A desktop with one file on it, with the bytes it has.
    fn with_bytes(path: &str, bytes: &[u8]) -> Fake {
        let mut disk = BTreeMap::new();
        disk.insert(path.to_string(), bytes.to_vec());
        Fake { disk, open: BTreeMap::new(), next: 0 }
    }

    /// The text of a file of the fake.
    fn text(&self, path: &str) -> String {
        String::from_utf8_lossy(self.disk.get(path).map(|file| file.as_slice()).unwrap_or(&[])).to_string()
    }

    /// The bytes of a file of the fake.
    fn bytes(&self, path: &str) -> Vec<u8> {
        self.disk.get(path).cloned().unwrap_or_default()
    }
}

impl Desktop for Fake {}
impl Pictures for Fake {}
impl Network for Fake {}
impl Sound for Fake {}
impl Machine for Fake {}
impl Dialogs for Fake {}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Windows for Fake {}

impl Files for Fake {
    fn open_file(&mut self, path: &str, mode: &str) -> Result<i64, String> {
        match mode {
            "r" => {
                if !self.disk.contains_key(path) {
                    // The machine says the code of the system in front of
                    // what it says, and v2 reads it that way.
                    return Err("(2) The system cannot find the file specified.".to_string());
                }
            }
            "w" => {
                self.disk.insert(path.to_string(), Vec::new());
            }
            "a" | "rw" | "r+" => {
                self.disk.entry(path.to_string()).or_default();
            }
            other => return Err(format!("the mode {other:?} is not one this desktop knows")),
        }
        let handle = self.next;
        self.next += 1;
        self.open.insert(handle, path.to_string());
        Ok(handle)
    }

    fn read_file_at(&mut self, handle: i64, pos: i64, count: i64) -> Result<Vec<u8>, String> {
        let path = self
            .open
            .get(&handle)
            .cloned()
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        let bytes = self.disk.get(&path).cloned().unwrap_or_default();
        let start = pos.max(0) as usize;
        if start >= bytes.len() {
            return Ok(Vec::new());
        }
        let end = (start + count.max(0) as usize).min(bytes.len());
        Ok(bytes[start..end].to_vec())
    }

    fn write_file_at(&mut self, handle: i64, pos: i64, bytes: &[u8]) -> Result<usize, String> {
        let path = self
            .open
            .get(&handle)
            .cloned()
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        let file = self.disk.entry(path).or_default();
        let start = pos.max(0) as usize;
        if file.len() < start {
            file.resize(start, 0);
        }
        let end = start + bytes.len();
        if file.len() < end {
            file.resize(end, 0);
        }
        file[start..end].copy_from_slice(bytes);
        Ok(bytes.len())
    }

    fn file_size(&mut self, handle: i64) -> Result<i64, String> {
        let path = self
            .open
            .get(&handle)
            .cloned()
            .ok_or_else(|| format!("the file {handle} is not open"))?;
        Ok(self.disk.get(&path).map(|file| file.len() as i64).unwrap_or(0))
    }

    fn close_file(&mut self, handle: i64) -> Result<(), String> {
        self.open.remove(&handle);
        Ok(())
    }
}

impl Keyboard for Fake {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }

    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Pointer for Fake {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
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
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

impl Native for Fake {}

/// Run a script against a desktop and answer the report of it.
fn run(desktop: &mut Fake, source: &str) -> Report {
    let statements = parse(source).expect("parses");
    Runtime::new(desktop, Duration::from_secs(10)).run(&statements)
}

/// Run a script against a desktop with one file on it.
fn run_with_file(path: &str, text: &str, source: &str) -> (Report, Fake) {
    let mut desktop = Fake::with_file(path, text);
    let report = run(&mut desktop, source);
    (report, desktop)
}

#[test]
fn a_file_reads_a_line() {
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := FileOpen(\"notes.txt\", \"r\")\nline := f.ReadLine()\nMsgBox \"%line%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one"]);
}

#[test]
fn a_file_reads_a_number_of_characters() {
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := FileOpen(\"notes.txt\", \"r\")\ntext := f.Read(3)\nMsgBox \"%text%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one"]);
}

#[test]
fn a_file_reads_the_rest_of_it() {
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := FileOpen(\"notes.txt\", \"r\")\ntext := f.Read()\nMsgBox \"%text%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one\ntwo\n"]);
}

#[test]
fn a_file_writes_and_reads_back() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.WriteLine(\"hello\")\nf.WriteLine(\"world\")\nf.Close()\ng := FileOpen(\"out.txt\", \"r\")\nfirst := g.ReadLine()\nsecond := g.ReadLine()\nMsgBox \"%first% / %second%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello / world"]);
    assert_eq!(desktop.text("out.txt"), "hello\nworld\n");
}

#[test]
fn a_file_writes_without_a_newline() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Write(\"hello\")\nf.Close()\ng := FileOpen(\"out.txt\", \"r\")\ntext := g.Read()\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello"]);
    assert_eq!(desktop.text("out.txt"), "hello");
}

#[test]
fn a_file_that_is_opened_to_be_appended_to_starts_at_its_end() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Write(\"one\")\nf.Close()\ng := FileOpen(\"out.txt\", \"a\")\ng.Write(\"two\")\ng.Close()\nh := FileOpen(\"out.txt\", \"r\")\ntext := h.Read()\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["onetwo"]);
}

#[test]
fn a_file_reports_its_length_and_its_position() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nf.Read(3)\nMsgBox \"%f.Length% %f.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["8 3"]);
}

#[test]
fn a_file_knows_when_it_is_at_its_end() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nbefore := f.AtEOF\nf.Read(8)\nafter := f.AtEOF\nMsgBox \"%before% %after%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0 1"]);
}

#[test]
fn a_file_seeks() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nmoved := f.Seek(4)\nMsgBox \"%moved% %f.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 4"]);
}

#[test]
fn a_file_writes_a_number_and_reads_it_back() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"nums.bin\", \"w\")\nwritten := f.WriteUInt(42)\nf.Close()\ng := FileOpen(\"nums.bin\", \"r\")\nnumber := g.ReadUInt()\nMsgBox \"%written% %number%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4 42"]);
}

#[test]
fn a_file_reads_the_bytes_into_a_buffer() {
    // v2.0.19 has no way to read a byte out of a buffer: the buffer is
    // memory, and the reads of bytes belong to the file.
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"raw.bin\", \"w\")\nf.RawWrite(\"abc\")\nf.Close()\ng := FileOpen(\"raw.bin\", \"r\")\nc := Buffer(3)\ncount := g.RawRead(c)\nMsgBox \"%count% %g.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 3"]);
    // Measured on the binary of v2.0.19 on 2026-10-30: the raw write of
    // a text writes the bytes of the UTF-16, and not the bytes of the
    // encoding of the file.
    assert_eq!(desktop.text("raw.bin"), "a\0b\0c\0");
}

#[test]
fn a_file_writes_a_string_as_the_bytes_of_it() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.RawWrite(\"abc\")\nf.Close()\ng := FileOpen(\"out.txt\", \"r\")\ntext := g.Read()\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // The raw write of a text writes the bytes of the UTF-16: the text
    // of the file is read in the encoding of the file, and the bytes
    // that are there are the ones of the UTF-16.
    assert_eq!(report.output, vec!["a\0b\0c\0"]);
    assert_eq!(desktop.text("out.txt"), "a\0b\0c\0");
}

#[test]
fn a_closed_file_answers_an_empty_text() {
    // Measured on the binary of v2.0.19 on 2026-10-30: a read of a file
    // that is closed answers an empty text, and it does not fail.
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nf.Close()\nline := f.ReadLine()\nMsgBox \"%line%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![""]);
}

#[test]
fn a_closed_file_answers_the_number_of_what_it_wrote() {
    // Measured on the binary of v2.0.19 on 2026-10-30: a write of a file
    // that is closed answers the number of characters it wrote, and the
    // position of the file answers minus one.
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nf.Close()\nwritten := f.Write(\"abc\")\nMsgBox \"%written% %f.Pos% %f.Length% %f.AtEOF%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 -1 0 1"]);
}

#[test]
fn a_file_that_is_not_there_fails() {
    let mut desktop = Fake::default();
    let report = run(&mut desktop, "f := FileOpen(\"nope.txt\", \"r\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("(2) The system cannot find the file specified.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_read_past_the_end_of_a_file_fails() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nf.Read(4)\nnumber := f.ReadUInt()",
    );
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the read is past the end of the file."));
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_file_has_an_encoding() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nMsgBox \"%f.Encoding%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: a file of the
    // default is a file of the code page of the machine.
    assert_eq!(report.output, vec!["CP1252"]);
}

#[test]
fn a_file_of_utf_8_has_the_name_of_utf_8() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\", \"UTF-8\")\nMsgBox \"%f.Encoding%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["UTF-8"]);
}

#[test]
fn a_file_may_change_its_encoding() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Encoding := \"UTF-16\"\nf.Write(\"hi\")\nf.Close()\ng := FileOpen(\"out.txt\", \"r\", \"UTF-16\")\ntext := g.Read(2)\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hi"]);
}

#[test]
fn an_encoding_that_is_not_carried_fails() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nf.Encoding := \"CP437\"",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "the encoding \"CP437\" is not one this runtime carries: it knows the code page of the machine (CP1252), UTF-8 and UTF-16."
        )
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_file_of_the_code_page_writes_the_bytes_of_the_code_page() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Write(Chr(0xE9))\nf.Close()",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the bytes of a
    // file of the default are the bytes of the code page of the machine.
    assert_eq!(desktop.bytes("out.txt"), vec![0xE9]);
}

#[test]
fn a_character_the_code_page_does_not_have_is_a_question_mark() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Write(Chr(0x65E5))\nf.Close()",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: a character the
    // code page does not have is a question mark.
    assert_eq!(desktop.bytes("out.txt"), vec![0x3F]);
}

#[test]
fn a_file_of_utf_8_begins_with_the_mark() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\", \"UTF-8\")\nf.Write(\"a\")\nf.Close()",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-29: a file of UTF-8
    // begins with the mark of three bytes.
    assert_eq!(desktop.bytes("out.txt"), vec![0xEF, 0xBB, 0xBF, b'a']);
}

#[test]
fn a_file_of_utf_8_raw_has_no_mark() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\", \"UTF-8-RAW\")\nf.Write(\"a\")\nf.Close()",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-29: the raw files
    // have no mark.
    assert_eq!(desktop.bytes("out.txt"), vec![b'a']);
}

#[test]
fn the_mark_of_a_file_is_skipped_when_the_file_is_opened() {
    let mut desktop = Fake::with_bytes("notes.txt", &[0xEF, 0xBB, 0xBF, b'a']);
    let report = run(
        &mut desktop,
        "f := FileOpen(\"notes.txt\", \"r\")\npos := f.Pos\ntext := f.Read()\nMsgBox \"%pos% / %text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the position of a
    // file with a mark goes over the mark when the file is opened.
    assert_eq!(report.output, vec!["3 / a"]);
}

#[test]
fn the_mark_of_a_file_decides_the_encoding_of_it() {
    let mut desktop = Fake::with_bytes("notes.txt", &[0xEF, 0xBB, 0xBF, b'a']);
    let report = run(
        &mut desktop,
        "f := FileOpen(\"notes.txt\", \"r\", \"CP1252\")\nMsgBox \"%f.Encoding%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the bytes of a
    // file decide the encoding of it, whatever the script asked for.
    assert_eq!(report.output, vec!["UTF-8"]);
}

#[test]
fn a_file_that_is_appended_to_keeps_the_encoding_of_its_head() {
    let mut desktop = Fake::with_bytes("notes.txt", &[0xEF, 0xBB, 0xBF, b'a']);
    let report = run(
        &mut desktop,
        "f := FileOpen(\"notes.txt\", \"a\")\nMsgBox \"%f.Encoding% / %f.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-30: the encoding of a
    // file that is appended to is the one of its head.
    assert_eq!(report.output, vec!["UTF-8 / 4"]);
}

#[test]
fn a_file_has_a_handle() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nMsgBox \"%f.Handle%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_file_may_be_made_longer() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Length := 4\nf.Close()\ng := FileOpen(\"out.txt\", \"r\")\nMsgBox \"%g.Length%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_file_that_is_made_shorter_fails() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := FileOpen(\"out.txt\", \"w\")\nf.Write(\"hello\")\nf.Length := 2",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this desktop cannot make a file shorter: it cannot cut one.")
    );
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_file_has_the_methods_of_the_base_object() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nmethod := f.HasMethod(\"ReadLine\")\nprop := f.HasProp(\"Length\")\nbase := f.HasBase(\"Object\")\nMsgBox \"%method% %prop% %base%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1 1"]);
}

#[test]
fn the_inside_of_a_file_is_not_a_property_of_it() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := FileOpen(\"notes.txt\", \"r\")\nprops := f.OwnProps()\ninside := f.HasOwnProp(\"file state\")\nMsgBox \"%props% %inside%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![" 0"]);
}

#[test]
fn the_failure_of_a_file_that_is_not_there_is_of_the_class_of_the_system() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the failure of
    // the machine is of the class `OSError`, and its message is the code
    // of the system in front of what it says.
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "try {\n    f := FileOpen(\"nope.txt\", \"r\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n    MsgBox e is OSError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "(2) The system cannot find the file specified. | FileOpen",
            "1",
        ]
    );
}

#[test]
fn the_failure_of_a_mode_that_is_not_one_is_of_the_class_of_a_value() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "try {\n    f := FileOpen(\"nope.txt\", \"zz\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n    MsgBox e is ValueError\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid parameter(s). | FileOpen", "1"]);
}

#[test]
fn a_file_has_no_unsigned_number_of_sixty_four_bits() {
    // Measured on the binary of v2.0.19 on 2026-10-30: v2 has no
    // `ReadUInt64` and no `WriteUInt64`, and the tree does not carry them
    // either.
    let mut desktop = Fake::default();
    let report = run(&mut desktop, "f := FileOpen(\"out.txt\", \"w\")\nx := f.ReadUInt64()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"File\" has no method named \"ReadUInt64\".")
    );
}

#[test]
fn a_file_is_not_made_again() {
    let mut desktop = Fake::default();
    let report = run(&mut desktop, "f := FileOpen(\"out.txt\", \"w\")\nx := f.__New()");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This value of type \"File\" has no method named \"__New\".")
    );
}

#[test]
fn a_file_is_not_made() {
    // Measured on the binary of v2.0.19 on 2026-10-30: v2 does not make
    // a file either, and the call goes through the base object. The file
    // is made by the `FileOpen` built-in.
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "try {\n    x := File(\"out.txt\", \"w\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Invalid base. | Object.Call | "]);
}
