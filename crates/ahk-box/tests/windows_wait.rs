//! The waits of the tree, against the machine itself.
//!
//! A wait asks the machine about a window until the answer is the one that
//! is wanted. The case of this file is the title that is empty, which is
//! the title of no window at all: the v1 and the v2 ask for the last found
//! window, and the wait of this runtime does not have one.

#![cfg(windows)]

use ahk::desktop::{TitleMatch, WaitFor, Windows};
use ahk_box::windows::{gui, WindowsDesktop};

/// The window of the script, on the screen.
///
/// The window is there while the wait runs, so that the answer of the wait
/// cannot be the one of a machine that has no window at all.
fn a_window_on_the_screen() -> ahk_box::windows::gui::TheWindow {
    let mut window = gui::open_of_the_v1("the probe of the wait").expect("the window is made");
    gui::the_window_is_shown(&mut window, true);
    window
}

/// The wait with no title is the wait for no window at all.
///
/// Measured on the binary of v1.1.37.02: the `WinWaitClose` with no title
/// answers at once and leaves the `ErrorLevel` at zero. Measured on the
/// binary of v2.0.32: the `WinWaitClose()` with no title answers `1` at
/// once.
#[test]
fn the_wait_with_no_title_is_the_wait_for_no_window_at_all() {
    let _window = a_window_on_the_screen();
    let mut desktop = WindowsDesktop::new();
    let answer = desktop
        .window_wait("", TitleMatch::Exact, WaitFor::Close, 2_000, &[])
        .expect("the wait answers");
    assert_eq!(answer, Some(String::new()));
}

/// The wait that finds a window answers the identifier of it.
///
/// Measured on the binary of v2.0.32: the `WinWait("Magnifier")` answers
/// `1053368`, the identifier of the window.
#[test]
fn the_wait_that_finds_a_window_answers_the_identifier_of_it() {
    let window = a_window_on_the_screen();
    let mut desktop = WindowsDesktop::new();
    let answer = desktop
        .window_wait(
            "the probe of the wait",
            TitleMatch::Exact,
            WaitFor::Present,
            2_000,
            &[],
        )
        .expect("the wait answers");
    assert_eq!(answer, Some((window.handle as i64).to_string()));
}

/// The wait with no title does not wait for a window to come.
///
/// Measured on the binary of v2.0.32: the `WinWait()` with no title waits
/// for a window that never comes. The v1 waits the same way.
#[test]
fn the_wait_with_no_title_does_not_find_a_window() {
    let _window = a_window_on_the_screen();
    let mut desktop = WindowsDesktop::new();
    let answer = desktop
        .window_wait("", TitleMatch::Exact, WaitFor::Present, 500, &[])
        .expect("the wait answers");
    assert_eq!(answer, None);
}
