//! The hotkeys of the script.
//!
//! The script registers the hotkeys it wants, and the runtime runs the
//! function of the one that was pressed. The desktop owns the watching of
//! the keyboard: the runtime asks it, between the statements of the
//! script, which hotkey was pressed, and the desktop answers with the
//! name of the hotkey it was given.
//!
//! The spelling of a hotkey is v2's: `^!+` for the modifiers, the name of
//! the key and the `Up` at the end. The cases at the end of this file pin
//! it, and the messages of the failures are the ones of v2.0.19, measured
//! on the binary on 2026-10-28.

use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::ast::Node;
use crate::desktop::WindowAnswer;
use crate::keys;
use crate::runtime::{Route, Runtime, RuntimeError, RuntimeFailure, Value};

/// The name of the environment variable that says which device to watch.
///
/// The device of the keyboard is found by itself when the variable is not
/// set: the box has one physical keyboard, and the devices of the machine
/// say which one it is. The variable is there for the measurement: a
/// script that types with `Send` fires its own hotkey when the watcher
/// reads the device that the box writes.
pub const DEVICE_VARIABLE: &str = "AHK_HOTKEY_DEVICE";

/// What an entry of the registry is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A hotkey: a key the script watches for.
    Hotkey,
    /// A hotstring: a text the script watches for.
    Hotstring,
    /// The state that the hotkeys of the script share.
    ///
    /// The entry is not a hotkey: it is there so that a script that sets a
    /// condition before it has a hotkey keeps the condition.
    State,
}

/// A hotkey of the script.
#[derive(Debug, Clone)]
pub struct Hotkey {
    /// The name of the hotkey, as the script spelled it.
    pub name: String,
    /// The function the hotkey runs.
    pub function: String,
    /// Whether the hotkey is on.
    pub enabled: bool,
    /// What the entry is.
    pub kind: Kind,
    /// The line the hotkey was registered on.
    pub line: u32,
    /// The object of the class, which the callback receives.
    pub object: Option<Value>,
    /// The state the hotkeys of the script share.
    pub shared: Rc<RefCell<Shared>>,
    /// The condition of the hotkey, set by `HotIf`.
    pub condition: Option<Condition>,
    /// Whether the callback of the hotkey takes the name of it.
    ///
    /// The callback of v2 always receives the name of the hotkey, and a
    /// callback that cannot take it is refused. The tree serves the two:
    /// a callback that takes nothing runs without the name.
    pub takes_name: bool,
    /// The action of a hotkey of the v1.
    ///
    /// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the
    /// v1 writes a hotkey in a line of its own, and the action of it is the
    /// label that comes after the line, or the command that the line
    /// carries. The action is the body of the hotkey, and it runs with the
    /// scope of the script.
    pub body: Option<Vec<Node>>,
}

/// The hotkey that is registered: what the script asked for.
pub struct Shape {
    /// Whether it is a hotkey or a hotstring.
    pub kind: Kind,
    /// The name of it, as the script spelled it.
    pub name: String,
    /// The function it runs.
    pub function: String,
    /// Whether the callback takes the name of it.
    pub takes_name: bool,
    /// The options of it.
    pub options: String,
    /// The object of the class, which the callback receives.
    pub object: Option<Value>,
}

/// The state the hotkeys of one script share.
///
/// The runtime has one registry of hotkeys and no room for anything else,
/// so the state that is not of one hotkey -- the hotkey that ran last, the
/// condition of the next one -- lives in the registry, and every entry
/// holds the same state.
#[derive(Debug, Default)]
pub struct Shared {
    /// The hotkey that ran last, and when it ran.
    pub prior: Option<(String, u64)>,
    /// The hotkey that is running, and when it started.
    pub current: Option<(String, u64)>,
    /// The condition of the next hotkey, set by `HotIf`.
    pub condition: Option<Condition>,
}

/// The condition of a hotkey.
///
/// `HotIf` sets the condition of the next hotkey that is registered, and
/// the window variants of it ask about a window. The question of a window
/// is asked when the hotkey is pressed, and not when the condition is
/// set: that is the way v2 does it, and the way a script that watches a
/// window expects it.
#[derive(Debug, Clone)]
pub enum Condition {
    /// A function of the script: it is called when the hotkey is pressed.
    Function(String),
    /// The window with this title has to be the active one.
    WindowActive(String),
    /// The window with this title has to be there.
    WindowExists(String),
    /// The window with this title has not to be the active one.
    WindowNotActive(String),
    /// The window with this title has not to be there.
    WindowNotExist(String),
}

/// The spelling of a hotkey: what the script wrote.
#[derive(Debug, Clone)]
pub struct Spec {
    /// The modifiers that have to be down, by the name the runtime uses.
    pub modifiers: Vec<String>,
    /// The name of the key.
    pub key: String,
    /// The code of the key, the way the events of the machine carry it.
    pub code: u16,
    /// Whether the hotkey is the key coming up.
    pub up: bool,
    /// Whether the hotkey fires in the middle of a word.
    pub tilde: bool,
    /// Whether the hotkey fires for any combination of the modifiers.
    pub custom: bool,
}

/// The spelling of a hotkey.
///
/// Measured on the binary of v2.0.19 on 2026-10-28: `^` is the control,
/// `!` the alt, `+` the shift and `#` the windows key, `<` and `>` say
/// which side of the key, `~` and `*` are the two prefixes, and the ` Up`
/// at the end is the key coming up. A name that is not a key is refused
/// with "Invalid key name.", and a spelling that is not one key with
/// "Invalid hotkey.".
pub fn parse(spec: &str) -> Result<Spec, String> {
    let mut modifiers: Vec<String> = Vec::new();
    let mut tilde = false;
    let mut custom = false;
    let mut rest = spec;
    // The prefixes and the modifiers are at the front.
    let mut marks: Vec<char> = Vec::new();
    while let Some(first) = rest.chars().next() {
        match first {
            '~' if modifiers.is_empty() && !tilde => {
                tilde = true;
                rest = &rest[1..];
            }
            '*' if modifiers.is_empty() && !custom => {
                custom = true;
                rest = &rest[1..];
            }
            // The $ of the spelling asks for the keyboard hook, and the
            // hook of the tree watches every key: the mark is taken and it
            // changes nothing.
            '$' if modifiers.is_empty() => {
                rest = &rest[1..];
            }
            '<' | '>' => {
                marks.push(first);
                rest = &rest[1..];
            }
            '^' | '!' | '+' | '#' => {
                marks.push(first);
                rest = &rest[1..];
            }
            _ => break,
        }
    }
    for (index, mark) in marks.iter().enumerate() {
        if !matches!(mark, '^' | '!' | '+' | '#') {
            continue;
        }
        let left = index > 0 && marks[index - 1] == '<';
        let right = index + 1 < marks.len() && marks[index + 1] == '>';
        for name in sides(*mark, left, right) {
            modifiers.push(name);
        }
    }
    // The ` Up` at the end is the key coming up. v2 has no ` Down`: the
    // spelling is refused with "Invalid hotkey.".
    let mut up = false;
    if rest.len() > 3 && rest[rest.len() - 3..].eq_ignore_ascii_case(" up") {
        up = true;
        rest = &rest[..rest.len() - 3];
    }
    // A lone modifier is the key itself: `^` is the control key.
    if rest.is_empty() {
        let Some(modifier) = modifiers.pop() else {
            return Err("Invalid key name.".to_string());
        };
        if modifiers.is_empty() {
            let code =
                keys::resolve_name(&modifier).map_err(|_| "Invalid key name.".to_string())?;
            return Ok(Spec {
                modifiers,
                key: modifier,
                code,
                up,
                tilde,
                custom,
            });
        }
        return Err("Invalid key name.".to_string());
    }
    if rest.contains(' ') {
        return Err("Invalid hotkey.".to_string());
    }
    let code = keys::resolve_name(rest).map_err(|_| "Invalid key name.".to_string())?;
    Ok(Spec {
        modifiers,
        key: rest.to_string(),
        code,
        up,
        tilde,
        custom,
    })
}

/// The names of the modifier, by the side of the key it is on.
///
/// The runtime names the four modifiers `ctrl`, `alt`, `shift` and `win`,
/// and the right ones have an `r` in front of them.
fn sides(marker: char, left: bool, right: bool) -> Vec<String> {
    let (name, right_name) = match marker {
        '^' => ("ctrl", "rctrl"),
        '!' => ("alt", "ralt"),
        '+' => ("shift", "rshift"),
        _ => ("win", "rwin"),
    };
    match (left, right) {
        (true, true) => vec![name.to_string(), right_name.to_string()],
        (true, false) => vec![name.to_string()],
        (false, true) => vec![right_name.to_string()],
        (false, false) => vec![name.to_string()],
    }
}

/// The text of a hotstring, when the name is one.
///
/// A hotstring of v2 is written between two of the same character, and
/// the character is not part of the text: `:btw:` is the text `btw`. The
/// text of a plain name is nothing, and the name is a hotkey.
pub fn hotstring_text(name: &str) -> Option<String> {
    let mut chars = name.chars();
    let first = chars.next()?;
    let last = name.chars().last()?;
    if first != last || first.is_alphanumeric() || name.chars().count() < 3 {
        return None;
    }
    let inner: String = name
        .chars()
        .skip(1)
        .take(name.chars().count() - 2)
        .collect();
    Some(inner)
}

/// The hotkey that was pressed, and the function that runs for it.
///
/// The desktop says which hotkey was pressed, and the runtime runs the
/// function of the script that the hotkey was registered with. The
/// callback of v2 receives the name of the hotkey, and the tree gives it
/// the same.
pub(crate) fn run_fired(rt: &mut Runtime<'_>) -> Result<(), RuntimeFailure> {
    // A script with no hotkey has nothing to run, and the desktop is not
    // asked at all: a desktop that cannot watch the keyboard is not a
    // failure of a script that wants no hotkey.
    if !rt.hotkeys().iter().any(|entry| entry.kind != Kind::State) {
        return Ok(());
    }
    let fired = match rt.desktop().fired_hotkey() {
        Ok(Some(name)) => name,
        Ok(None) => return Ok(()),
        Err(reason) => return Err(fail(&reason, 0)),
    };
    let Some(entry) = rt
        .hotkeys()
        .iter()
        .find(|entry| entry.matches(&fired))
        .cloned()
    else {
        return Ok(());
    };
    if !entry.enabled {
        return Ok(());
    }
    if !condition_holds(rt, &entry)? {
        return Ok(());
    }
    // The hotkey that ran last is the one that ran before this one, and
    // the time of it is when it started.
    {
        let mut shared = entry.shared.borrow_mut();
        shared.prior = shared.current.clone();
        shared.current = Some((entry.name.clone(), rt.now_ms()));
    }
    // The action of a hotkey of the v1 is the body that comes with it, and
    // it runs with the scope of the script and not with the scope of a
    // function of it.
    if let Some(body) = &entry.body {
        rt.run_the_body_of_a_hotkey(body)?;
        return Ok(());
    }
    if entry.takes_name {
        rt.call_named(
            &entry.function,
            &[Value::Text(fired)],
            entry.line,
            Route::Written,
        )?;
    } else {
        rt.call_function_by_name(&entry.function, entry.line)?;
    }
    Ok(())
}

/// The hotkey of the v1: the line that defines it, and its action.
///
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-03: the hotkey
/// of the v1 is registered when the section of the script that runs reaches
/// it, and the action of it is the body that comes with it. The spelling of
/// the v1 is not the spelling of v2, and the desktop is the one that says
/// whether it can watch the key.
pub(crate) fn register_the_hotkey_of_the_v1(
    rt: &mut Runtime<'_>,
    name: &str,
    body: Vec<Node>,
    line: u32,
) -> Result<(), RuntimeFailure> {
    // The desktop watches the keyboard. A desktop that cannot watch says
    // so, and the script is told why: a hotkey that is registered and
    // never fires is a hotkey that does not work.
    rt.desktop()
        .register_hotkey(name)
        .map_err(|reason| fail(&reason, line))?;
    let shared = shared_of(rt);
    rt.hotkeys().push(Hotkey {
        name: name.to_string(),
        function: String::new(),
        enabled: true,
        kind: Kind::Hotkey,
        line,
        object: None,
        shared,
        condition: None,
        takes_name: false,
        body: Some(body),
    });
    Ok(())
}

/// The hotkey of the v2 that the script writes in a line of its own.
///
/// Measured on the binary of v2.0.19 on 2026-11-09: `F1::MsgBox "hi"` is a
/// hotkey whose action is the statement of the line, and `F1::{ ... }` one
/// whose action is the block. A desktop that cannot watch the keyboard does
/// not make the script fail: the v2 has no failure of the load for it, and a
/// script of this tree is still a script -- the note says why the hotkey is
/// not there, and the rest of it runs.
pub(crate) fn register_the_hotkey_of_the_v2(
    rt: &mut Runtime<'_>,
    name: &str,
    body: Vec<Node>,
    line: u32,
) -> Result<(), RuntimeFailure> {
    if let Err(reason) = rt.desktop().register_hotkey(name) {
        rt.note(format!(
            "the hotkey {name} is not watched on this desktop: {reason}"
        ));
        return Ok(());
    }
    let shared = shared_of(rt);
    rt.hotkeys().push(Hotkey {
        name: name.to_string(),
        function: String::new(),
        enabled: true,
        kind: Kind::Hotkey,
        line,
        object: None,
        shared,
        condition: None,
        takes_name: false,
        body: Some(body),
    });
    Ok(())
}

/// Whether the entry answers the name the desktop gave.
///
/// A hotkey answers to its own name, and a hotstring to the text that was
/// typed: the text of a hotstring is matched without its case, and the
/// name of the hotkey is not.
impl Hotkey {
    /// Whether the entry is the one the desktop named.
    fn matches(&self, fired: &str) -> bool {
        match self.kind {
            Kind::Hotstring => match hotstring_text(&self.name) {
                Some(text) => text.eq_ignore_ascii_case(fired),
                None => false,
            },
            _ => self.name == fired,
        }
    }
}

/// Whether the condition of the hotkey lets it run.
///
/// A question the machine cannot answer fails with the reason: a hotkey
/// that waits for a window the desktop cannot see does not run, and the
/// script is told why.
fn condition_holds(rt: &mut Runtime<'_>, entry: &Hotkey) -> Result<bool, RuntimeFailure> {
    let Some(condition) = entry.condition.clone() else {
        return Ok(true);
    };
    // The mode of the matching of the titles is a setting of the script,
    // and it is handed to the desktop with the question.
    let mode = crate::settings::title_match_mode();
    match condition {
        Condition::Function(name) => {
            let value = rt.call_named(&name, &[], entry.line, Route::Written)?;
            Ok(value.truthy())
        }
        Condition::WindowActive(title) => match rt.desktop().window_active(&title, mode) {
            Ok(WindowAnswer::Active) => Ok(true),
            // A window that is not there is not the active one.
            Ok(WindowAnswer::NotActive) | Ok(WindowAnswer::Absent) => Ok(false),
            Ok(WindowAnswer::Unknown(reason)) => Err(fail(&reason, entry.line)),
            Err(reason) => Err(fail(&reason, entry.line)),
        },
        Condition::WindowExists(title) => match rt.desktop().window_exists(&title, mode) {
            Ok(WindowAnswer::Active) | Ok(WindowAnswer::NotActive) => Ok(true),
            // A window that is not there is not there.
            Ok(WindowAnswer::Absent) => Ok(false),
            Ok(WindowAnswer::Unknown(reason)) => Err(fail(&reason, entry.line)),
            Err(reason) => Err(fail(&reason, entry.line)),
        },
        Condition::WindowNotActive(title) => match rt.desktop().window_active(&title, mode) {
            Ok(WindowAnswer::Active) => Ok(false),
            // A window that is not there is not the active one, and so it
            // is not active.
            Ok(WindowAnswer::NotActive) | Ok(WindowAnswer::Absent) => Ok(true),
            Ok(WindowAnswer::Unknown(reason)) => Err(fail(&reason, entry.line)),
            Err(reason) => Err(fail(&reason, entry.line)),
        },
        Condition::WindowNotExist(title) => match rt.desktop().window_exists(&title, mode) {
            Ok(WindowAnswer::Active) | Ok(WindowAnswer::NotActive) => Ok(false),
            // A window that is not there is not there.
            Ok(WindowAnswer::Absent) => Ok(true),
            Ok(WindowAnswer::Unknown(reason)) => Err(fail(&reason, entry.line)),
            Err(reason) => Err(fail(&reason, entry.line)),
        },
    }
}

/// Register a hotkey of the script.
///
/// The name is the spelling of the script, the function is the name of
/// the function of the script that the hotkey runs, and the object is the
/// one of the class, which the callback receives.
pub(crate) fn register(
    rt: &mut Runtime<'_>,
    shape: Shape,
    line: u32,
) -> Result<(), RuntimeFailure> {
    let Shape {
        kind,
        name,
        function,
        takes_name,
        options,
        object,
    } = shape;
    // The spelling has to be one that v2 accepts.
    match kind {
        Kind::Hotkey => {
            parse(&name).map_err(|reason| fail(&reason, line))?;
        }
        Kind::Hotstring => {
            if hotstring_text(&name).is_none() {
                return Err(fail("Parameter #1 of Hotstring is invalid.", line));
            }
        }
        Kind::State => {}
    }
    check_options(&options, line)?;
    // The options may turn the hotkey off, and a hotkey that is off is
    // not one the desktop watches.
    let mut enabled = true;
    for option in options.split_whitespace() {
        match option.to_lowercase().as_str() {
            "off" => enabled = false,
            "toggle" => enabled = !enabled,
            _ => {}
        }
    }
    // The desktop watches the keyboard. A desktop that cannot watch says
    // so, and the script is told why: a hotkey that is registered and
    // never fires is a hotkey that does not work.
    if enabled {
        rt.desktop()
            .register_hotkey(&name)
            .map_err(|reason| fail(&reason, line))?;
    }
    // The condition of `HotIf` belongs to the next hotkey, and not to the
    // one after it.
    let shared = shared_of(rt);
    let condition = shared.borrow_mut().condition.take();
    rt.hotkeys().push(Hotkey {
        name: name.clone(),
        function: function.clone(),
        enabled,
        kind,
        line,
        object,
        shared,
        condition,
        takes_name,
        body: None,
    });
    Ok(())
}

/// The state the hotkeys of the script share.
///
/// The registry keeps it in an entry of its own, so that a script that
/// sets a condition before it has a hotkey keeps the condition.
fn shared_of(rt: &mut Runtime<'_>) -> Rc<RefCell<Shared>> {
    if let Some(entry) = rt.hotkeys().iter().find(|entry| entry.kind == Kind::State) {
        return entry.shared.clone();
    }
    let shared = Rc::new(RefCell::new(Shared::default()));
    rt.hotkeys().push(Hotkey {
        name: String::new(),
        function: String::new(),
        enabled: false,
        kind: Kind::State,
        line: 0,
        object: None,
        shared: shared.clone(),
        condition: None,
        takes_name: false,
        body: None,
    });
    shared
}

/// The state the hotkeys of the script share, when there is one.
fn shared_state(rt: &mut Runtime<'_>) -> Option<Rc<RefCell<Shared>>> {
    rt.hotkeys()
        .iter()
        .find(|entry| entry.kind == Kind::State)
        .map(|entry| entry.shared.clone())
}

/// The options of a hotkey.
///
/// Measured on the binary of v2.0.19 on 2026-10-28: the letters are B, I,
/// P, S and T, a letter may carry a number, and the words On, Off and
/// Toggle turn the hotkey on and off. Anything else is "Parameter #3 of
/// Hotkey is invalid.".
pub(crate) fn check_options(options: &str, line: u32) -> Result<(), RuntimeFailure> {
    for option in options.split_whitespace() {
        let lowered = option.to_lowercase();
        if lowered == "on" || lowered == "off" || lowered == "toggle" {
            continue;
        }
        let mut rest = lowered.as_str();
        while !rest.is_empty() {
            let letter = rest.chars().next().unwrap_or(' ');
            rest = &rest[letter.len_utf8()..];
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            rest = &rest[digits.len()..];
            let known = match letter {
                'b' | 'i' | 'p' | 's' => true,
                // The threads of a hotkey are a number, and not a letter.
                't' => !digits.is_empty(),
                _ => false,
            };
            if !known {
                return Err(fail("Parameter #3 of Hotkey is invalid.", line));
            }
        }
    }
    Ok(())
}

/// The function a hotkey runs.
///
/// The action of a hotkey is a function of the script, and v2 refuses
/// anything else: a text that is not one of the three words is
/// "Parameter #2 of Hotkey is invalid.", and a function that cannot take
/// the name of the hotkey is "Invalid callback function.".
///
/// Measured on the binary of v2.0.19 on 2026-10-28: `Hotkey "a", "b"`
/// is refused, and so is `Hotkey "a", "MyFunc"` when the script has such
/// a function.
pub(crate) fn action_of(value: &Value, line: u32) -> Result<(String, bool), RuntimeFailure> {
    let Value::Object(object) = value else {
        return Err(fail("Parameter #2 of Hotkey is invalid.", line));
    };
    let object = object.borrow();
    if object.class != "func" {
        return Err(fail("Parameter #2 of Hotkey is invalid.", line));
    }
    let builtin = object
        .value_of("builtin")
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    if builtin != 0.0 {
        return Err(fail("Invalid callback function.", line));
    }
    let max = object
        .value_of("maxparams")
        .and_then(|value| value.to_number())
        .unwrap_or(0.0);
    // The callback of a hotkey receives the name of it, and it may not
    // take anything else.
    if max > 1.0 {
        return Err(fail("Invalid callback function.", line));
    }
    let name = object
        .value_of("name")
        .map(|value| value.to_text())
        .unwrap_or_default();
    Ok((name, max >= 1.0))
}

/// The action of a hotstring.
///
/// The action of a hotstring may also be a text: v2 takes the name of a
/// label of the script, and the tree has no labels -- the text is the name
/// of a function of the script, and a hotstring of a function that the
/// script does not have is a hotstring of nothing.
///
/// Measured on the binary of v2.0.19 on 2026-10-28: `Hotstring ":btw:",
/// "b"` is taken, and `Hotstring ":btw:", 123` is taken as well.
pub(crate) fn hotstring_action_of(
    value: &Value,
    line: u32,
) -> Result<(String, bool), RuntimeFailure> {
    match value {
        Value::Text(text) => Ok((text.clone(), false)),
        Value::Int(number) => Ok((number.to_string(), false)),
        other => action_of(other, line),
    }
}

/// The `Hotkey` command.
///
/// The command registers a hotkey: the key it watches for and the
/// function it runs. The class of the same name does the same thing and
/// gives the object of the hotkey back, and the runtime asks the class
/// types before it asks this table.
pub fn hotkey(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = text_argument(values.first(), "Parameter #1 of Hotkey is invalid.", line)?;
    // A hotkey without an action is a hotkey of nothing, and v2 says so.
    let Some(action) = values.get(1).cloned() else {
        return Err(RuntimeError {
            message: "Nonexistent hotkey.".to_string(),
            line,
        });
    };
    let options = match values.get(2) {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    // The words On, Off and Toggle are not a function: they turn a
    // hotkey that is already there on and off.
    if let Value::Text(word) = &action {
        match word.to_lowercase().as_str() {
            "on" | "off" | "toggle" => {
                return toggle(rt, &name, word, line);
            }
            _ => {}
        }
    }
    let (function, takes_name) =
        action_of(&action, line).map_err(|failure| as_error(failure, line))?;
    let shape = Shape {
        kind: Kind::Hotkey,
        name,
        function,
        takes_name,
        options,
        object: None,
    };
    register(rt, shape, line).map_err(|failure| as_error(failure, line))?;
    Ok(Value::Text(String::new()))
}

/// Turn a hotkey that is already there on or off.
fn toggle(rt: &mut Runtime<'_>, name: &str, word: &str, line: u32) -> Result<Value, RuntimeError> {
    let lowered = word.to_lowercase();
    let Some(entry) = rt
        .hotkeys()
        .iter()
        .find(|entry| entry.name == name)
        .cloned()
    else {
        return Err(RuntimeError {
            message: "Nonexistent hotkey.".to_string(),
            line,
        });
    };
    let on = match lowered.as_str() {
        "on" => true,
        "off" => false,
        _ => !entry.enabled,
    };
    set_enabled(rt, &entry, on).map_err(|failure| as_error(failure, line))?;
    Ok(Value::Text(String::new()))
}

/// Turn a hotkey on or off, and tell the desktop about it.
///
/// A hotkey that is already in the state it is asked for is not told to
/// the desktop again: v2 starts a hotkey that is running, and it does
/// nothing.
pub(crate) fn set_enabled(
    rt: &mut Runtime<'_>,
    entry: &Hotkey,
    on: bool,
) -> Result<(), RuntimeFailure> {
    if entry.enabled == on {
        return Ok(());
    }
    if on {
        rt.desktop()
            .register_hotkey(&entry.name)
            .map_err(|reason| fail(&reason, entry.line))?;
    } else {
        rt.desktop()
            .unregister_hotkey(&entry.name)
            .map_err(|reason| fail(&reason, entry.line))?;
    }
    for hotkey in rt.hotkeys().iter_mut() {
        if hotkey.kind == entry.kind && hotkey.name == entry.name {
            hotkey.enabled = on;
        }
    }
    Ok(())
}

/// The `Hotstring` command.
///
/// A hotstring is a text that fires a function when it is typed. The text
/// of v2 is written between two of the same character, and the character
/// is not part of it.
pub fn hotstring(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let text = match values.first() {
        Some(Value::Text(text)) => text.clone(),
        Some(_) => {
            return Err(RuntimeError {
                message:
                    "Parameter #1 of Hotstring requires a String, but received something else."
                        .to_string(),
                line,
            });
        }
        None => {
            return Err(RuntimeError {
                message: "Parameter #1 of Hotstring is invalid.".to_string(),
                line,
            });
        }
    };
    // v2 says which parameter is wrong first, and the name of the
    // hotstring comes before the action: measured on the binary of
    // v2.0.19 on 2026-10-28, `Hotstring "btw", "b"` is "Parameter #1 of
    // Hotstring is invalid.".
    if hotstring_text(&text).is_none() {
        return Err(RuntimeError {
            message: "Parameter #1 of Hotstring is invalid.".to_string(),
            line,
        });
    }
    let options = match values.get(2) {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    // A hotstring without an action runs nothing, and v2 takes it.
    let (function, takes_name) = match values.get(1) {
        Some(action) => {
            hotstring_action_of(action, line).map_err(|failure| as_error(failure, line))?
        }
        None => (String::new(), false),
    };
    let shape = Shape {
        kind: Kind::Hotstring,
        name: text,
        function,
        takes_name,
        options,
        object: None,
    };
    register(rt, shape, line).map_err(|failure| as_error(failure, line))?;
    Ok(Value::Text(String::new()))
}

/// The `HotIf` command.
///
/// The condition of a hotkey is a function of the script, and it is
/// called when the hotkey is pressed. The command without an argument
/// takes the condition away.
pub fn hotif(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let shared = shared_of(rt);
    match values.first() {
        None => {
            shared.borrow_mut().condition = None;
        }
        Some(value) => {
            let Value::Object(object) = value else {
                return Err(RuntimeError {
                    message: "Parameter #1 must match an existing #HotIf expression.".to_string(),
                    line,
                });
            };
            let object = object.borrow();
            if object.class != "func" {
                return Err(RuntimeError {
                    message: "Parameter #1 must match an existing #HotIf expression.".to_string(),
                    line,
                });
            }
            let name = object
                .value_of("name")
                .map(|value| value.to_text())
                .unwrap_or_default();
            shared.borrow_mut().condition = Some(Condition::Function(name));
        }
    }
    Ok(Value::Text(String::new()))
}

/// The `HotIfWin...` commands.
///
/// The variant says which window the condition is about: 0 is the active
/// one, 1 is a window that is there, 2 is the active one that must not
/// be there, and 3 is a window that must not be there.
pub fn hotif_window(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    variant: u8,
) -> Result<Value, RuntimeError> {
    let title = text_argument(
        values.first(),
        "Parameter #1 must match an existing #HotIf expression.",
        line,
    )?;
    let condition = match variant {
        0 => Condition::WindowActive(title),
        1 => Condition::WindowExists(title),
        2 => Condition::WindowNotActive(title),
        _ => Condition::WindowNotExist(title),
    };
    let shared = shared_of(rt);
    shared.borrow_mut().condition = Some(condition);
    Ok(Value::Text(String::new()))
}

/// The `GetKeyState` command.
///
/// The command says whether a key is down right now. v2.0.19 takes one
/// parameter only: the second one is "Parameter #2 of GetKeyState is
/// invalid.".
pub fn get_key_state(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if values.len() > 1 {
        return Err(RuntimeError {
            message: "Parameter #2 of GetKeyState is invalid.".to_string(),
            line,
        });
    }
    let name = text_argument(
        values.first(),
        "Parameter #1 of GetKeyState is invalid.",
        line,
    )?;
    if keys::resolve_name(&name).is_err() {
        return Err(RuntimeError {
            message: "Parameter #1 of GetKeyState is invalid.".to_string(),
            line,
        });
    }
    match rt.desktop().key_is_down(&name) {
        Ok(down) => Ok(Value::Int(down as i64)),
        Err(reason) => Err(RuntimeError {
            message: reason,
            line,
        }),
    }
}

/// The `KeyHistory` command.
///
/// The command of v2 opens a window with the keys that were pressed. The
/// box has no window to open, and the script is told why.
pub fn key_history(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message:
            "this desktop cannot show the history of the keys: it has no window to show it in."
                .to_string(),
        line,
    })
}

/// The `ListHotkeys` command.
///
/// The command of v2 opens a window with the hotkeys of the script. The
/// box has no window to open, and the script is told why.
pub fn list_hotkeys(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Err(RuntimeError {
        message:
            "this desktop cannot show the list of the hotkeys: it has no window to show it in."
                .to_string(),
        line,
    })
}

/// The `InstallKeybdHook` and `InstallMouseHook` commands.
///
/// The hooks of v2 are installed when a hotkey needs them, and the
/// commands say that they are there. The box watches the keyboard from
/// the first hotkey, and the mouse is not watched at all.
pub fn install_hook(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text(String::new()))
}

/// The value of a built-in variable of the hotkeys.
///
/// The names are the ones of v2, without the `A_` that the table of the
/// variables of the runtime carries. The three numbers are the ones of
/// v2.0.19, measured on the binary on 2026-10-28.
pub fn variable(name: &str, rt: &mut Runtime<'_>) -> Option<Result<Value, RuntimeError>> {
    let spelled = name.to_lowercase();
    let name = spelled.strip_prefix("a_").unwrap_or(spelled.as_str());
    let value = match name {
        "hotkeyinterval" => Value::Int(2000),
        // Measured on the binary of v2.0.19 on 2026-11-01: the timeout of
        // the change of a hotkey is fifty until a script writes another
        // one. The name is `A_HotkeyModifierTimeout`: the tree had measured
        // a name that does not exist, and it said that the variable had no
        // value.
        "hotkeymodifiertimeout" => Value::Int(50),
        "maxhotkeysperinterval" => Value::Int(70),
        "priorhotkey" => match shared_state(rt) {
            Some(shared) => match &shared.borrow().prior {
                Some((name, _)) => Value::Text(name.clone()),
                None => Value::Text(String::new()),
            },
            None => Value::Text(String::new()),
        },
        // The tree does not keep the key that was pressed last, and v2
        // answers with an empty text until there is one.
        "priorkey" => Value::Text(String::new()),
        "thishotkey" => match shared_state(rt) {
            Some(shared) => match &shared.borrow().current {
                Some((name, _)) => Value::Text(name.clone()),
                None => Value::Text(String::new()),
            },
            None => Value::Text(String::new()),
        },
        "timesincepriorhotkey" => match shared_state(rt) {
            Some(shared) => {
                let shared = shared.borrow();
                match &shared.prior {
                    Some((_, when)) => Value::Int(rt.now_ms() as i64 - *when as i64),
                    None => Value::Text(String::new()),
                }
            }
            None => Value::Text(String::new()),
        },
        "timesincethishotkey" => match shared_state(rt) {
            Some(shared) => {
                let shared = shared.borrow();
                match &shared.current {
                    Some((_, when)) => Value::Int(rt.now_ms() as i64 - *when as i64),
                    None => Value::Text(String::new()),
                }
            }
            None => Value::Text(String::new()),
        },
        _ => return None,
    };
    Some(Ok(value))
}

/// The text of an argument, or the failure of v2.
fn text_argument(value: Option<&Value>, message: &str, line: u32) -> Result<String, RuntimeError> {
    match value {
        Some(Value::Text(text)) => Ok(text.clone()),
        _ => Err(RuntimeError {
            message: message.to_string(),
            line,
        }),
    }
}

/// A failure of a hotkey, with the message of v2.
fn fail(message: &str, line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: message.to_string(),
        line,
    })
}

/// A failure of a built-in, as the table of the built-ins wants it.
fn as_error(failure: RuntimeFailure, line: u32) -> RuntimeError {
    match failure {
        RuntimeFailure::Error(error) => error,
        RuntimeFailure::Detailed(error, _) => error,
        _ => RuntimeError {
            message: "the hotkey could not be registered.".to_string(),
            line,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{
        Clipboard, Desktop, Dialogs, Files, Hotkeys, Keyboard, Machine, Native, Network, Pictures,
        Pointer, Processes, Screen, Sound, Timers, TitleMatch, WindowAnswer, Windows,
    };
    use crate::runtime::Clock;
    use alloc::string::String;
    use alloc::vec::Vec;
    use core::time::Duration;

    /// A desktop that says which hotkey was pressed.
    ///
    /// The fake fires the first hotkey it was given, once: that is what
    /// a keyboard does when a key is pressed.
    struct Fake {
        fired: bool,
        registered: Vec<String>,
        down: Vec<String>,
    }

    impl Fake {
        fn new() -> Self {
            Fake {
                fired: false,
                registered: Vec::new(),
                down: Vec::new(),
            }
        }
    }

    impl Keyboard for Fake {
        fn type_text(&mut self, text: &str) -> Result<String, String> {
            Ok(text.to_string())
        }

        fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
            Ok(name.to_string())
        }

        fn key_is_down(&mut self, name: &str) -> Result<bool, String> {
            Ok(self.down.iter().any(|key| key == name))
        }
    }

    impl Pointer for Fake {
        fn click(
            &mut self,
            _x: i64,
            _y: i64,
            _button: &str,
            _count: u32,
        ) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Screen for Fake {
        fn capture(
            &mut self,
            _x: i64,
            _y: i64,
            _w: i64,
            _h: i64,
            _filename: &str,
        ) -> Result<String, String> {
            Ok(String::new())
        }

        fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
            Ok("#000000".to_string())
        }
    }

    impl Processes for Fake {
        fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
            Ok((0, String::new(), String::new()))
        }
    }

    impl Windows for Fake {
        fn window_active(
            &mut self,
            _title: &str,
            _mode: TitleMatch,
        ) -> Result<WindowAnswer, String> {
            // The fake has no window at all: every window is absent.
            Ok(WindowAnswer::Absent)
        }
        fn window_exists(
            &mut self,
            _title: &str,
            _mode: TitleMatch,
        ) -> Result<WindowAnswer, String> {
            Ok(WindowAnswer::Absent)
        }
    }
    impl Files for Fake {}
    impl Clipboard for Fake {}
    impl Timers for Fake {}

    impl Hotkeys for Fake {
        fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
            self.registered.push(name.to_string());
            Ok(name.to_string())
        }

        fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {
            Ok(name.to_string())
        }

        fn hotkeys(&mut self) -> Result<Vec<String>, String> {
            Ok(self.registered.clone())
        }

        fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
            if self.fired {
                return Ok(None);
            }
            self.fired = true;
            Ok(self.registered.first().cloned())
        }
    }

    impl Native for Fake {}
    impl Desktop for Fake {}
    impl Pictures for Fake {}
    impl Network for Fake {}
    impl Sound for Fake {}
    impl Machine for Fake {}
    impl Dialogs for Fake {}

    /// A clock that does not move.
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            0
        }

        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// The spelling of `^!a` is the two modifiers and the key.
    #[test]
    fn the_modifiers_are_read_from_the_front() {
        let spec = parse("^!a").unwrap();
        assert_eq!(spec.modifiers, vec!["ctrl".to_string(), "alt".to_string()]);
        assert_eq!(spec.key, "a");
        assert_eq!(spec.code, 30);
        assert!(!spec.up);
    }

    /// The shift is a `+` and the windows key is a `#`.
    #[test]
    fn the_shift_and_the_windows_key_are_read() {
        assert_eq!(
            parse("^+a").unwrap().modifiers,
            vec!["ctrl".to_string(), "shift".to_string()]
        );
        assert_eq!(parse("#a").unwrap().modifiers, vec!["win".to_string()]);
    }

    /// The `<` and the `>` say which side of the key the modifier is on.
    #[test]
    fn the_sides_of_the_modifiers_are_read() {
        assert_eq!(parse("<^a").unwrap().modifiers, vec!["ctrl".to_string()]);
        assert_eq!(parse("^>a").unwrap().modifiers, vec!["rctrl".to_string()]);
        assert_eq!(
            parse("<^>a").unwrap().modifiers,
            vec!["ctrl".to_string(), "rctrl".to_string()]
        );
    }

    /// The `~` and the `*` are the two prefixes, and they come first.
    #[test]
    fn the_prefixes_are_read() {
        let spec = parse("~^a").unwrap();
        assert!(spec.tilde);
        assert!(!spec.custom);
        let spec = parse("*^a").unwrap();
        assert!(spec.custom);
        assert!(!spec.tilde);
    }

    /// The ` Up` at the end is the key coming up.
    #[test]
    fn the_up_suffix_is_read() {
        let spec = parse("a Up").unwrap();
        assert!(spec.up);
        assert_eq!(spec.key, "a");
        let spec = parse("^a Up").unwrap();
        assert!(spec.up);
        assert_eq!(spec.modifiers, vec!["ctrl".to_string()]);
    }

    /// v2 has no ` Down`: the spelling is refused.
    #[test]
    fn the_down_suffix_is_refused() {
        assert_eq!(parse("a Down").unwrap_err(), "Invalid hotkey.");
    }

    /// A name that is not a key is refused.
    #[test]
    fn a_name_that_is_not_a_key_is_refused() {
        assert_eq!(parse("Foo").unwrap_err(), "Invalid key name.");
        assert_eq!(parse("").unwrap_err(), "Invalid key name.");
    }

    /// Two keys are not one hotkey.
    #[test]
    fn two_keys_are_refused() {
        assert_eq!(parse("a b").unwrap_err(), "Invalid hotkey.");
    }

    /// A lone modifier is the key itself.
    #[test]
    fn a_lone_modifier_is_the_key() {
        let spec = parse("^").unwrap();
        assert_eq!(spec.key, "ctrl");
        assert_eq!(spec.code, 29);
    }

    /// The options of v2 are the letters B, I, P, S and T, and the three
    /// words that turn a hotkey on and off.
    #[test]
    fn the_options_are_read() {
        assert!(check_options("B", 1).is_ok());
        assert!(check_options("S", 1).is_ok());
        assert!(check_options("P", 1).is_ok());
        assert!(check_options("I", 1).is_ok());
        assert!(check_options("T2", 1).is_ok());
        assert!(check_options("On", 1).is_ok());
        assert!(check_options("Off", 1).is_ok());
        assert!(check_options("Toggle", 1).is_ok());
        assert!(check_options("SS", 1).is_ok());
    }

    /// An option that v2 does not know is refused with its message.
    #[test]
    fn an_option_that_v2_does_not_know_is_refused() {
        let failure = check_options("X", 1).unwrap_err();
        let RuntimeFailure::Error(error) = failure else {
            panic!("the failure is not an error");
        };
        assert_eq!(error.message, "Parameter #3 of Hotkey is invalid.");
    }

    /// The text of a hotstring is between two of the same character.
    #[test]
    fn the_text_of_a_hotstring_is_read() {
        assert_eq!(hotstring_text(":btw:"), Some("btw".to_string()));
        assert_eq!(hotstring_text("abc"), None);
        assert_eq!(hotstring_text("~a~"), Some("a".to_string()));
        assert_eq!(hotstring_text("(a)"), None);
    }

    /// The name of a hotstring is checked before its action.
    #[test]
    fn the_name_of_a_hotstring_is_checked_before_its_action() {
        // Measured on the binary of v2.0.19 on 2026-10-28.
        let script = "Hotstring(\"btw\", \"b\")\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(!report.ok, "the script was expected to fail");
        assert_eq!(
            report.error.unwrap(),
            "Parameter #1 of Hotstring is invalid."
        );
    }

    /// The action of a hotstring may be a text: it is the name of a
    /// function of the script.
    #[test]
    fn the_action_of_a_hotstring_may_be_a_text() {
        let script = "Callback(hk) {\n    MsgBox hk\n}\nHotstring(\":btw:\", \"Callback\")\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(desktop.registered, vec![":btw:".to_string()]);
    }

    /// A hotkey that is registered is watched by the desktop, and the
    /// function of the script runs when the hotkey is pressed.
    ///
    /// The callback of v2 receives the name of the hotkey, and the tree
    /// gives it the same.
    #[test]
    fn the_hotkey_that_was_pressed_runs_its_function() {
        let script = "Callback(hk) {\n    MsgBox hk\n}\nHotkey(\"^a\", Callback)\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(desktop.registered, vec!["^a".to_string()]);
        assert_eq!(report.output, vec!["^a".to_string()]);
    }

    /// A script with no hotkey never asks the desktop about one.
    #[test]
    fn a_script_with_no_hotkey_asks_nothing() {
        let script = "MsgBox \"hello\"\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert!(desktop.registered.is_empty());
    }

    /// A window that is not there is not there: the hotkey that asks for
    /// it does not run.
    #[test]
    fn a_hotkey_of_a_window_that_is_not_there_does_not_run() {
        let script =
            "Callback(hk) {\n    MsgBox hk\n}\nHotIfWinExist \"Steam\"\nHotkey(\"^a\", Callback)\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(desktop.registered, vec!["^a".to_string()]);
        assert!(
            report.output.is_empty(),
            "the hotkey ran: {:?}",
            report.output
        );
    }

    /// A window that is not there is not the active one, and so it is not
    /// active: the hotkey that asks for the opposite runs.
    #[test]
    fn a_hotkey_of_a_window_that_is_not_active_runs() {
        let script = "Callback(hk) {\n    MsgBox hk\n}\nHotIfWinNotActive \"Steam\"\nHotkey(\"^a\", Callback)\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let report =
            Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(report.output, vec!["^a".to_string()]);
    }

    /// The numbers of v2 are the ones the binary has.
    #[test]
    fn the_numbers_of_the_hotkeys_are_the_ones_of_v2() {
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        assert_eq!(
            variable("hotkeyinterval", &mut rt).unwrap().unwrap(),
            Value::Int(2000)
        );
        assert_eq!(
            variable("maxhotkeysperinterval", &mut rt).unwrap().unwrap(),
            Value::Int(70)
        );
        // Measured on the binary of v2.0.19 on 2026-11-01: the timeout of
        // the change of a hotkey is fifty until a script writes another one.
        assert_eq!(
            variable("hotkeymodifiertimeout", &mut rt).unwrap().unwrap(),
            Value::Int(50)
        );
    }

    /// The variables of the hotkeys carry the state of the last one that
    /// ran.
    #[test]
    fn the_variables_carry_the_state_of_the_hotkeys() {
        let mut desktop = Fake::new();
        let mut clock = TheClock;
        let mut rt = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10));
        // A script that has not run a hotkey has no hotkey to name.
        assert_eq!(
            variable("thishotkey", &mut rt).unwrap().unwrap(),
            Value::Text(String::new())
        );
        assert_eq!(
            variable("priorhotkey", &mut rt).unwrap().unwrap(),
            Value::Text(String::new())
        );
        assert_eq!(
            variable("timesincepriorhotkey", &mut rt).unwrap().unwrap(),
            Value::Text(String::new())
        );
        let shared = shared_of(&mut rt);
        shared.borrow_mut().current = Some(("^a".to_string(), 0));
        shared.borrow_mut().prior = Some(("a".to_string(), 0));
        assert_eq!(
            variable("thishotkey", &mut rt).unwrap().unwrap(),
            Value::Text("^a".to_string())
        );
        assert_eq!(
            variable("priorhotkey", &mut rt).unwrap().unwrap(),
            Value::Text("a".to_string())
        );
        assert_eq!(
            variable("timesincethishotkey", &mut rt).unwrap().unwrap(),
            Value::Int(0)
        );
    }
}
