//! The window of a script, drawn by the helper, over a pipe.
//!
//! The language crate does no I/O and holds no toolkit, so a window of the
//! runtime is drawn by a program of its own -- `helper/ahk-gui.py` -- and this
//! is the pipe to it. One line of JSON in, one line of JSON out:
//!
//! ```text
//! in   {"op": "window", "title": "ahk-gui-probe"}
//! out  {"ok": true}
//! in   {"op": "add", "kind": "Edit", "text": "the field"}
//! out  {"ok": true, "id": 1, "classnn": "Edit1"}
//! in   {"op": "title"}
//! out  {"ok": true, "title": "ahk-gui-probe"}
//! in   {"op": "title", "set": "renamed"}
//! out  {"ok": true, "title": "renamed"}
//! in   {"op": "destroy"}
//! out  {"ok": true}
//! ```
//!
//! # Why a helper and not a binding
//!
//! The tree already reaches the desktop by running the tools of the box --
//! `grim`, `zenity`, `busctl`, `df` -- and a toolkit is the same kind of thing. A
//! binding would put GTK inside the runner and a `Gtk.main` loop inside a runtime
//! that deliberately has neither, and would commit the tree to GTK for good. The
//! helper keeps the language crate free of both, and it is one file.
//!
//! # The path of the helper is the caller's
//!
//! `open` takes it, so that this module does not guess where a file of the tree
//! is at run time. The convention of the tree is `helper/` beside the sources
//! (the raven's helper is there too), and the caller that knows its own layout
//! is the one that should say.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// The pipe to the program that draws the window.
#[derive(Debug)]
pub struct TheWindow {
    /// The program. Dropping this kills it, and the window goes with it -- the
    /// helper ends its loop when its input closes.
    program: Child,
    input: ChildStdin,
    /// The answers of the helper, read by a thread of their own so that a wait for
    /// one can be given up on. **A helper that stops answering must not stop the
    /// runner**: the timeout of a run is checked between statements, and a read
    /// that never returns is inside one.
    lines: std::sync::mpsc::Receiver<String>,
}

/// Read the answers of the helper on a thread of their own.
///
/// The thread owns the reader, so the wait for an answer becomes a receive with a
/// timeout instead of a read that cannot be given up on.
fn the_lines_of(output: ChildStdout) -> std::sync::mpsc::Receiver<String> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut reader = BufReader::new(output);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if sender.send(line.trim_end().to_string()).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });
    receiver
}

impl TheWindow {
    /// Start the helper.
    ///
    /// The helper is read on the machine it runs on: `python3` with the file of
    /// the tree, which is what the box has. A machine without it answers why.
    pub fn open(helper: &Path) -> Result<Self, String> {
        let mut program = Command::new("python3")
            .arg(helper)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("the window helper could not be started: {error}"))?;
        let input = program
            .stdin
            .take()
            .ok_or("the window helper took no input")?;
        let output = program
            .stdout
            .take()
            .ok_or("the window helper gave no output")?;
        Ok(Self {
            program,
            input,
            lines: the_lines_of(output),
        })
    }

    /// Send one command and answer the line the helper says back.
    ///
    /// The command is the JSON of one of the five operations, and the answer is
    /// the JSON the helper wrote, unread by this module: what a field of it
    /// means is the business of the class that asked.
    pub fn send(&mut self, command: &str) -> Result<String, String> {
        self.input
            .write_all(the_line(command).as_bytes())
            .map_err(|error| format!("the window helper took no command: {error}"))?;
        self.input
            .flush()
            .map_err(|error| format!("the window helper took no command: {error}"))?;
        let answer = self
            .lines
            .recv_timeout(std::time::Duration::from_secs(5))
            .map_err(|_| "the window helper stopped answering".to_string())?;
        if answer.is_empty() {
            return Err("the window helper ended before it answered".to_string());
        }
        Ok(answer)
    }

    /// Take the window away, and let the helper go.
    pub fn close(mut self) -> Result<(), String> {
        // The input closing is what ends the helper's loop, and its window.
        let _ = self.send("{\"op\": \"quit\"}");
        let _ = self.program.kill();
        Ok(())
    }
}

/// One command as the line of the pipe.
///
/// The command carries its own braces; this adds the line break that ends it,
/// and nothing else.
fn the_line(command: &str) -> String {
    format!("{command}\n")
}

#[cfg(test)]
mod tests {
    use super::{TheWindow, the_line};

    #[test]
    fn a_command_is_one_line_of_the_pipe() {
        assert_eq!(the_line("{\"op\": \"quit\"}"), "{\"op\": \"quit\"}\n");
    }

    #[test]
    fn the_line_is_the_command_and_one_break() {
        let line = the_line("{}");
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
    }

    /// The live half of the transport, and it is ignored by default.
    ///
    /// It needs three things this workstation does not have together: a
    /// graphical session for GTK, the helper of the tree, and the real
    /// `python3` with `gi`. The box has all three, and the run is:
    ///
    /// ```text
    /// AHK_GUI_HELPER=~/ahk/crates/ahk-box/helper/ahk-gui.py \
    ///     cargo test -p ahk-box -- --ignored
    /// ```
    #[test]
    #[ignore = "needs a graphical session and the helper; run it on the box"]
    fn the_helper_starts_and_answers() {
        let path =
            std::env::var("AHK_GUI_HELPER").expect("AHK_GUI_HELPER is the path of the helper");
        let mut window = TheWindow::open(std::path::Path::new(&path)).expect("the helper starts");
        let answer = window
            .send("{\"op\": \"window\", \"title\": \"ahk-transport-probe\"}")
            .expect("the helper answers");
        assert!(answer.contains("\"ok\": true"), "the helper said {answer}");
        window.close().expect("the helper goes");
    }
}
