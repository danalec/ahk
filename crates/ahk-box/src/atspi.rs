//! The accessibility bus of the desktop.
//!
//! AT-SPI is a D-Bus protocol, and that is what makes it worth reading: it is
//! the one description of the windows and the controls of them that a Wayland
//! session keeps, because the compositor is on the same bus. Measured on the
//! box on 2026-11-09: `org.a11y.Bus` runs, the compositor owns
//! `org.freedesktop.a11y.Manager`, and the registry at
//! `/org/a11y/atspi/accessible/root` lists the applications of the session.
//!
//! # Why `busctl` and not a crate
//!
//! The tree declares no D-Bus dependency, and it already shells out to `grim`,
//! `zenity`, `df` and `notify-send`. `busctl` is a tool of the machine like
//! them, and `--json=short` gives an answer that a few string operations read:
//!
//! ```text
//! {"type":"a(so)","data":[[[":1.1","/org/a11y/atspi/accessible/root"], ...]]}
//! {"type":"s","data":"xdg-desktop-portal-gtk"}
//! ```
//!
//! # What the tree looks like
//!
//! Measured on the box: an application's root has windows as its children, and
//! the `Name` of a window is the **title of the window**. That is the thing
//! that makes `ControlGetText(Control, WinTitle)` possible without correlating
//! a Wayland toplevel to a bus name: the title is in the tree already.
//!
//! Not every node has the `Text` interface -- a container does not -- so the
//! walk asks for the text of a node and keeps going when it is not there.

use std::process::Command;

use ahk::desktop::ControlAnswer;

/// One node of the tree: the connection that holds it, and its object path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    /// The bus name of the application, as AT-SPI gives it: `:1.4`.
    pub bus: String,
    /// The object path of the node.
    pub path: String,
}

/// The address of the accessibility bus, or why the session has none.
pub fn address() -> Result<String, String> {
    let output = Command::new("busctl")
        .args([
            "--user",
            "call",
            "org.a11y.Bus",
            "/org/a11y/bus",
            "org.a11y.Bus",
            "GetAddress",
        ])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err("this session has no accessibility bus, so it describes no control".to_string());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    // The answer is `s "unix:path=/run/user/1000/at-spi/bus_0"`.
    text.split('"')
        .nth(1)
        .map(str::to_string)
        .ok_or_else(|| "the accessibility bus gave no address".to_string())
}

/// The quoted strings of an answer, after its `data`.
///
/// `busctl --json=short` writes `{"type": ..., "data": ...}` and nothing else
/// that is quoted, so the strings of the data are the answer.
fn the_strings_of(answer: &str) -> Vec<String> {
    let Some((_, data)) = answer.split_once("\"data\":") else {
        return Vec::new();
    };
    data.split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// The children of a node.
pub fn children(address: &str, node: &Node) -> Result<Vec<Node>, String> {
    let answer = call(
        address,
        &node.bus,
        &node.path,
        "org.a11y.atspi.Accessible.GetChildren",
        &[],
    )?;
    let strings = the_strings_of(&answer);
    // The answer is a list of pairs, a bus and a path each.
    Ok(strings
        .chunks_exact(2)
        .map(|pair| Node {
            bus: pair[0].clone(),
            path: pair[1].clone(),
        })
        .collect())
}

/// The name of a node.
pub fn name(address: &str, node: &Node) -> String {
    let Ok(answer) = get_property(address, node, "org.a11y.atspi.Accessible", "Name") else {
        return String::new();
    };
    the_strings_of(&answer).into_iter().next().unwrap_or_default()
}

/// The text of a node, when the node has the interface of a text.
///
/// A container has none, and a node that has none is not a failure: the walk
/// keeps going.
pub fn text(address: &str, node: &Node) -> Option<String> {
    let answer = call(
        address,
        &node.bus,
        &node.path,
        "org.a11y.atspi.Text.GetText",
        &["uu", "0", "4096"],
    )
    .ok()?;
    the_strings_of(&answer).into_iter().next()
}

/// The text of a control of a window.
///
/// The window is found by its **title**, which is the `Name` of a window in the
/// tree, and the control is found by its name inside that window. The two
/// failures are told apart, because v2 tells them apart: a title that no window
/// has is not the same fact as a window that has no such control.
pub fn control_text(
    address: &str,
    title: &str,
    control: &str,
) -> Result<ControlAnswer, String> {
    let root = Node {
        bus: "org.a11y.atspi.Registry".to_string(),
        path: "/org/a11y/atspi/accessible/root".to_string(),
    };
    let mut the_window_is_there = false;
    for application in children(address, &root)? {
        // The window is a child of the application, and its name is its title.
        for window in children(address, &application).unwrap_or_default() {
            if !name(address, &window).contains(title) {
                continue;
            }
            the_window_is_there = true;
            if let Some(found) = the_control(address, &window, control, 0) {
                return Ok(ControlAnswer::Found(found));
            }
        }
    }
    if the_window_is_there {
        Ok(ControlAnswer::NoSuchControl)
    } else {
        Ok(ControlAnswer::NoSuchWindow)
    }
}

/// The text of the first node under one that has this name.
///
/// The walk is bounded in both ways -- the depth and the number of nodes -- so
/// that a tree of a program that never ends does not hold the script.
fn the_control(address: &str, node: &Node, control: &str, depth: usize) -> Option<String> {
    const DEPTH: usize = 8;
    const NODES: usize = 200;
    if depth > DEPTH {
        return None;
    }
    let children = children(address, node).unwrap_or_default();
    for child in children.iter().take(NODES) {
        // A control is named by its name, and a label of a dialog has none: its
        // text is the only name it has. Measured on the box on 2026-11-09, the
        // label of a `zenity` dialog answers an empty `Name` and the text of
        // the dialog through `Text`.
        let named = name(address, child);
        if named.contains(control) {
            // Measured on the box on 2026-11-09 with a GTK3 window: a `label` is
            // named with its text, and it has **no** `Text` interface --
            // `Text.GetText` on one answers `impl_GetText: assertion
            // 'ATK_IS_TEXT (user_data)' failed`. So the name of a control that
            // has no text is the text of it.
            return Some(text(address, child).unwrap_or(named));
        }
        if let Some(text) = text(address, child) {
            if text.contains(control) {
                return Some(text);
            }
        }
        if let Some(found) = the_control(address, child, control, depth + 1) {
            return Some(found);
        }
    }
    None
}

/// A method of a node.
fn call(address: &str, bus: &str, path: &str, method: &str, args: &[&str]) -> Result<String, String> {
    let mut command = Command::new("busctl");
    // `busctl call` takes the interface and the method as two arguments, and
    // the caller writes them as one name. Passing the whole name as the
    // interface is what made `busctl` answer "Too few arguments.".
    let (interface, method) = method.rsplit_once('.').unwrap_or(("", method));
    let mut owned: Vec<String> = vec![
        format!("--address={address}"),
        "--json=short".to_string(),
        "call".to_string(),
        bus.to_string(),
        path.to_string(),
        interface.to_string(),
        method.to_string(),
    ];
    owned.extend(args.iter().map(|argument| argument.to_string()));
    for argument in &owned {
        command.arg(argument);
    }
    let output = command.output().map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// A property of a node.
fn get_property(address: &str, node: &Node, interface: &str, property: &str) -> Result<String, String> {
    let output = Command::new("busctl")
        .args([
            &format!("--address={address}"),
            "--json=short",
            "get-property",
            &node.bus,
            &node.path,
            interface,
            property,
        ])
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
