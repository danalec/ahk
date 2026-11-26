//! The criteria of the titles: the parts of a text that name a window.
//!
//! The criteria are a grammar of their own, and the grammar was
//! measured on the binaries of AutoHotkey v2.0.31 and v1.1.37.02. The probes are in `egm-review/compare_the_criteria.py`,
//! `egm-review/compare_the_criteria_of_the_v1.py` and
//! `egm-review/compare_the_edges_of_the_criteria.py`.
//!
//! The cases here are about the grammar itself: where a criterion
//! starts, where its value ends, and what a window has to answer.

use ahk::desktop::criteria::WindowCriteria;
use ahk::desktop::TitleMatch;

/// The window that the criteria are asked of.
struct AWindow {
    title: &'static str,
    class: &'static str,
    exe: &'static str,
    id: i64,
    pid: i64,
    /// Whether the window has the focus.
    active: bool,
}

impl AWindow {
    /// A window of the Notepad, of the python, with the identifier and
    /// the process that the cases use.
    fn notepad() -> AWindow {
        AWindow {
            title: "Untitled - Notepad",
            class: "Notepad",
            exe: "notepad.exe",
            id: 4331538,
            pid: 9001,
            active: false,
        }
    }

    /// Whether the window answers the criteria.
    fn answers(&self, criteria: &WindowCriteria) -> bool {
        criteria
            .matches(
                self.title,
                self.class,
                self.exe,
                self.id,
                self.pid,
                self.active,
                TitleMatch::Contains,
            )
            .unwrap_or(false)
    }
}

#[test]
fn the_title_that_is_the_a_alone_is_the_window_that_has_the_focus() {
    // Measured on the binary of v1.1.37.02: the `WinExist("A")` answers
    // the handle of the window that has the focus. Measured on the binary
    // of v2.0.32: the `WinExist("A")` answers the identifier of that
    // window.
    let criteria = WindowCriteria::parse("A");
    assert!(criteria.the_active_window);
    let mut window = AWindow::notepad();
    assert!(!window.answers(&criteria));
    window.active = true;
    assert!(window.answers(&criteria));
    // The `a` is the same thing, and a title that carries a criterion
    // beside it is a plain title.
    assert!(WindowCriteria::parse("a").the_active_window);
    assert!(!WindowCriteria::parse("A ahk_class Notepad").the_active_window);
}

#[test]
fn a_text_without_a_criterion_is_a_title() {
    let criteria = WindowCriteria::parse("Untitled - Notepad");
    assert_eq!(criteria.title, "Untitled - Notepad");
    assert!(criteria.class.is_none());
    assert!(criteria.exe.is_none());
    assert!(criteria.id.is_none());
    assert!(criteria.pid.is_none());
}

#[test]
fn the_title_is_the_text_that_is_before_the_first_criterion() {
    let criteria = WindowCriteria::parse("Untitled - Notepad ahk_class Notepad");
    assert_eq!(criteria.title, "Untitled - Notepad");
    assert_eq!(criteria.class.as_deref(), Some("Notepad"));
}

#[test]
fn a_criterion_that_is_in_the_middle_of_a_word_is_a_title() {
    // Measured on the binary of v2.0.31: `xxahk_class
    // Notepad` is a title and not a criterion.
    let criteria = WindowCriteria::parse("xxahk_class Notepad");
    assert_eq!(criteria.title, "xxahk_class Notepad");
    assert!(criteria.class.is_none());
}

#[test]
fn the_name_of_a_criterion_is_read_without_the_case() {
    let criteria = WindowCriteria::parse("AHK_CLASS Notepad");
    assert_eq!(criteria.class.as_deref(), Some("Notepad"));
    assert_eq!(criteria.title, "");
}

#[test]
fn the_value_of_a_criterion_may_have_spaces() {
    let criteria = WindowCriteria::parse("ahk_class ahk Test Class");
    assert_eq!(criteria.class.as_deref(), Some("ahk Test Class"));
}

#[test]
fn the_space_that_is_before_a_criterion_is_not_a_title() {
    // Measured on the binary of v2.0.31: the space that
    // separates the criterion from the text belongs to the text, and not
    // to the criterion.
    let criteria = WindowCriteria::parse("Untitled ahk_class Notepad");
    assert_eq!(criteria.title, "Untitled");
}

#[test]
fn a_space_that_is_before_a_criterion_is_not_a_title_either() {
    // Medido no binario do v2.0.31: `Untitled  ahk_class
    // Notepad` -- o titulo e `Untitled ` e o criterio e `ahk_class`.
    let criteria = WindowCriteria::parse("Untitled  ahk_class Notepad");
    assert_eq!(criteria.title, "Untitled ");
    assert_eq!(criteria.class.as_deref(), Some("Notepad"));
}

#[test]
fn the_space_that_is_after_the_name_of_a_criterion_is_not_a_value() {
    let criteria = WindowCriteria::parse("ahk_class    Notepad");
    assert_eq!(criteria.class.as_deref(), Some("Notepad"));
}

#[test]
fn the_space_that_is_at_the_end_of_the_text_is_a_value() {
    // Measured on the binary of v2.0.31: `ahk_class
    // Notepad ` does not find the window of the `Notepad`, because the
    // space at the end is part of the value.
    let criteria = WindowCriteria::parse("ahk_class Notepad ");
    assert_eq!(criteria.class.as_deref(), Some("Notepad "));
    let window = AWindow::notepad();
    assert!(!window.answers(&criteria));
}

#[test]
fn the_last_criterion_of_a_kind_is_the_one_that_counts() {
    let criteria = WindowCriteria::parse("ahk_class A ahk_class B");
    assert_eq!(criteria.class.as_deref(), Some("B"));
}

#[test]
fn the_value_of_a_criterion_ends_where_the_next_one_starts() {
    let criteria = WindowCriteria::parse("ahk_class Notepad ahk_exe notepad.exe");
    assert_eq!(criteria.class.as_deref(), Some("Notepad"));
    assert_eq!(criteria.exe.as_deref(), Some("notepad.exe"));
}

#[test]
fn the_class_is_compared_with_the_case() {
    // Measured on the binary of v2.0.31: `ahk_class
    // notepad` does not find the window of the class `Notepad`.
    let window = AWindow::notepad();
    assert!(window.answers(&WindowCriteria::parse("ahk_class Notepad")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_class notepad")));
}

#[test]
fn the_executable_is_compared_without_the_case() {
    let window = AWindow::notepad();
    assert!(window.answers(&WindowCriteria::parse("ahk_exe notepad.exe")));
    assert!(window.answers(&WindowCriteria::parse("ahk_exe NOTEPAD.EXE")));
}

#[test]
fn the_executable_wants_the_name_of_the_file() {
    // Measured on the binary of v2.0.31: `ahk_exe python`
    // does not find the window of the `python.exe`.
    let window = AWindow {
        exe: "python.exe",
        ..AWindow::notepad()
    };
    assert!(window.answers(&WindowCriteria::parse("ahk_exe python.exe")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_exe python")));
}

#[test]
fn the_identifier_names_the_window() {
    let window = AWindow::notepad();
    assert!(window.answers(&WindowCriteria::parse("ahk_id 4331538")));
    assert!(window.answers(&WindowCriteria::parse("ahk_id 0x421812")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_id 1")));
}

#[test]
fn an_identifier_that_is_not_a_number_names_no_window() {
    // Measured on the binary of v2.0.31: `ahk_id abc`
    // finds no window at all.
    let window = AWindow::notepad();
    assert!(!window.answers(&WindowCriteria::parse("ahk_id abc")));
}

#[test]
fn the_process_names_the_window() {
    let window = AWindow::notepad();
    assert!(window.answers(&WindowCriteria::parse("ahk_pid 9001")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_pid 1")));
}

#[test]
fn a_criterion_that_has_no_value_names_no_window() {
    let window = AWindow::notepad();
    assert!(!window.answers(&WindowCriteria::parse("ahk_class")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_exe")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_id")));
    assert!(!window.answers(&WindowCriteria::parse("ahk_pid")));
}

#[test]
fn a_window_that_has_no_title_answers_the_criteria_of_the_class() {
    let window = AWindow {
        title: "",
        ..AWindow::notepad()
    };
    assert!(window.answers(&WindowCriteria::parse("ahk_class Notepad")));
    // The empty title is a title, and a title that does not match the
    // criterion of the title.
    assert!(!window.answers(&WindowCriteria::parse("ahk_class Notepad x")));
}

#[test]
fn the_criteria_are_asked_together() {
    let window = AWindow::notepad();
    let criteria = WindowCriteria::parse("Notepad ahk_class Notepad ahk_exe notepad.exe");
    assert!(window.answers(&criteria));
    // The title that does not match already makes the window not
    // answer.
    let criteria = WindowCriteria::parse("Outpad ahk_class Notepad");
    assert!(!window.answers(&criteria));
}
