//! The `File` class, against a fake desktop.
//!
//! The fake keeps its files in memory, so these cases are about what
//! the class means and not about what the disk does. The box serves
//! the same methods, and the box is checked by hand.

use ahk::desktop::{
    Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers,
    Windows,
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
        let mut disk = BTreeMap::new();
        disk.insert(path.to_string(), text.as_bytes().to_vec());
        Fake { disk, open: BTreeMap::new(), next: 0 }
    }

    /// The text of a file of the fake.
    fn text(&self, path: &str) -> String {
        String::from_utf8_lossy(self.disk.get(path).map(|file| file.as_slice()).unwrap_or(&[])).to_string()
    }
}

impl Desktop for Fake {}

impl Clipboard for Fake {}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Windows for Fake {}

impl Files for Fake {
    fn open_file(&mut self, path: &str, mode: &str) -> Result<i64, String> {
        match mode {
            "r" => {
                if !self.disk.contains_key(path) {
                    return Err(format!("the file {path:?} is not there."));
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
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := File(\"notes.txt\", \"r\")\nline := f.ReadLine()\nMsgBox \"%line%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one"]);
}

#[test]
fn a_file_reads_a_number_of_characters() {
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := File(\"notes.txt\", \"r\")\ntext := f.Read(3)\nMsgBox \"%text%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one"]);
}

#[test]
fn a_file_reads_the_rest_of_it() {
    let (report, _desktop) = run_with_file("notes.txt", "one\ntwo\n", "f := File(\"notes.txt\", \"r\")\ntext := f.Read()\nMsgBox \"%text%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["one\ntwo\n"]);
}

#[test]
fn a_file_writes_and_reads_back() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"out.txt\", \"w\")\nf.WriteLine(\"hello\")\nf.WriteLine(\"world\")\nf.Close()\ng := File(\"out.txt\", \"r\")\nfirst := g.ReadLine()\nsecond := g.ReadLine()\nMsgBox \"%first% / %second%\"",
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
        "f := File(\"out.txt\", \"w\")\nf.Write(\"hello\")\nf.Close()\ng := File(\"out.txt\", \"r\")\ntext := g.Read()\nMsgBox \"%text%\"",
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
        "f := File(\"out.txt\", \"w\")\nf.Write(\"one\")\nf.Close()\ng := File(\"out.txt\", \"a\")\ng.Write(\"two\")\ng.Close()\nh := File(\"out.txt\", \"r\")\ntext := h.Read()\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["onetwo"]);
}

#[test]
fn a_file_reports_its_length_and_its_position() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := File(\"notes.txt\", \"r\")\nf.Read(3)\nMsgBox \"%f.Length% %f.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["8 3"]);
}

#[test]
fn a_file_knows_when_it_is_at_its_end() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := File(\"notes.txt\", \"r\")\nbefore := f.AtEOF\nf.Read(8)\nafter := f.AtEOF\nMsgBox \"%before% %after%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0 1"]);
}

#[test]
fn a_file_seeks() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\ntwo\n",
        "f := File(\"notes.txt\", \"r\")\nmoved := f.Seek(4)\nMsgBox \"%moved% %f.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 4"]);
}

#[test]
fn a_file_writes_a_number_and_reads_it_back() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"nums.bin\", \"w\")\nwritten := f.WriteUInt(42)\nf.Close()\ng := File(\"nums.bin\", \"r\")\nnumber := g.ReadUInt()\nMsgBox \"%written% %number%\"",
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
        "f := File(\"raw.bin\", \"w\")\nf.RawWrite(\"abc\")\nf.Close()\ng := File(\"raw.bin\", \"r\")\nc := Buffer(3)\ncount := g.RawRead(c)\nMsgBox \"%count% %g.Pos%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 3"]);
    assert_eq!(desktop.text("raw.bin"), "abc");
}

#[test]
fn a_file_writes_a_string_as_the_bytes_of_it() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"out.txt\", \"w\")\nf.RawWrite(\"abc\")\nf.Close()\ng := File(\"out.txt\", \"r\")\ntext := g.Read()\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["abc"]);
    assert_eq!(desktop.text("out.txt"), "abc");
}

#[test]
fn a_closed_file_fails() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := File(\"notes.txt\", \"r\")\nf.Close()\nline := f.ReadLine()",
    );
    assert!(!report.ok);
    assert_eq!(report.error.as_deref(), Some("the file is closed."));
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_file_that_is_not_there_fails() {
    let mut desktop = Fake::default();
    let report = run(&mut desktop, "f := File(\"nope.txt\", \"r\")");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the file \"nope.txt\" is not there.")
    );
    assert_eq!(report.error_line, Some(1));
}

#[test]
fn a_read_past_the_end_of_a_file_fails() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := File(\"notes.txt\", \"r\")\nf.Read(4)\nnumber := f.ReadUInt()",
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
        "f := File(\"notes.txt\", \"r\")\nMsgBox \"%f.Encoding%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["UTF-8"]);
}

#[test]
fn a_file_may_change_its_encoding() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"out.txt\", \"w\")\nf.Encoding := \"UTF-16\"\nf.Write(\"hi\")\nf.Close()\ng := File(\"out.txt\", \"r\", \"UTF-16\")\ntext := g.Read(2)\nMsgBox \"%text%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hi"]);
}

#[test]
fn an_encoding_that_is_not_carried_fails() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := File(\"notes.txt\", \"r\")\nf.Encoding := \"CP1252\"",
    );
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the encoding \"CP1252\" is not one this runtime carries: it knows UTF-8 and UTF-16.")
    );
    assert_eq!(report.error_line, Some(2));
}

#[test]
fn a_file_has_a_handle() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := File(\"notes.txt\", \"r\")\nMsgBox \"%f.Handle%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_file_may_be_made_longer() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"out.txt\", \"w\")\nf.Length := 4\nf.Close()\ng := File(\"out.txt\", \"r\")\nMsgBox \"%g.Length%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["4"]);
}

#[test]
fn a_file_that_is_made_shorter_fails() {
    let mut desktop = Fake::default();
    let report = run(
        &mut desktop,
        "f := File(\"out.txt\", \"w\")\nf.Write(\"hello\")\nf.Length := 2",
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
        "f := File(\"notes.txt\", \"r\")\nmethod := f.HasMethod(\"ReadLine\")\nprop := f.HasProp(\"Length\")\nbase := f.HasBase(\"Object\")\nMsgBox \"%method% %prop% %base%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 1 1"]);
}

#[test]
fn the_inside_of_a_file_is_not_a_property_of_it() {
    let (report, _desktop) = run_with_file(
        "notes.txt",
        "one\n",
        "f := File(\"notes.txt\", \"r\")\nprops := f.OwnProps()\ninside := f.HasOwnProp(\"file state\")\nMsgBox \"%props% %inside%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec![" 0"]);
}
