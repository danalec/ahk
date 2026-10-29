//! The `Hotkey` class.
//!
//! A hotkey is a key the script watches for. The box watches the keyboard
//! of the machine, and the object of the class is the hotkey itself: it
//! holds the options of the hotkey and the state of the last time it
//! ran, and the callback of the hotkey receives it.
//!
//! The state of the object lives in the property table of the object,
//! under one key, and only this module touches it.

use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::classes::object_with;
use crate::hotkeys::{self, Kind};
use crate::runtime::{Runtime, RuntimeError, RuntimeFailure, Value};

/// The names of the `Hotkey` class, with the thing each one does.
///
/// The names are the ones the binary gives the class, read out of the
/// surface table of v2.0.19.
const HOTKEY: &[(&str, &str)] = &[
    ("__new", "make the hotkey"),
    ("keyopt", "change the options of the hotkey"),
    ("start", "start watching for the key"),
    ("stop", "stop watching for the key"),
    ("wait", "wait for the key"),
    ("backspaceisundo", "read or set whether the backspace key undoes"),
    ("casesensitive", "read or set whether the key is matched by its case"),
    ("findanywhere", "read or set whether the key is found anywhere"),
    ("notifynontext", "read or set whether a key that is not text is announced"),
    ("visiblenontext", "read or set whether a key that is not text is shown"),
    ("visibletext", "read or set whether a text key is shown"),
    ("endkey", "read which key ended the hotkey"),
    ("endmods", "read which modifiers ended the hotkey"),
    ("endreason", "read why the hotkey ended"),
    ("inprogress", "read whether the hotkey is running"),
    ("input", "read the text the hotkey was given"),
    ("match", "read the text the hotkey matched"),
    ("minsendlevel", "read or set the lowest input level of the hotkey"),
    ("onchar", "read or set the character handler of the hotkey"),
    ("onend", "read or set the end handler of the hotkey"),
    ("onkeydown", "read or set the key-down handler of the hotkey"),
    ("onkeyup", "read or set the key-up handler of the hotkey"),
    ("timeout", "read or set the timeout of the hotkey"),
];

/// The names of the `Hotkey` class.
///
/// The names are the ones the binary gives the class, and they are the
/// whole surface of it.
pub(crate) fn names() -> Vec<&'static str> {
    HOTKEY.iter().map(|(name, _)| *name).collect()
}

/// The key the state of a hotkey lives under.
///
/// The key has a space in it, so that a script cannot reach the inside
/// of a hotkey by name.
const STATE: &str = "hotkey state";

/// The class of the object that carries the state of a hotkey.
const STATE_CLASS: &str = "hotkey-state";

/// The state of a hotkey: its options and what happened to it.
#[derive(Debug, Clone)]
struct State {
    name: String,
    function: String,
    enabled: bool,
    options: String,
    case_sensitive: bool,
    find_anywhere: bool,
    notify_non_text: bool,
    visible_non_text: bool,
    visible_text: bool,
    backspace_is_undo: bool,
    min_send_level: i64,
    timeout: i64,
    on_char: String,
    on_end: String,
    on_key_down: String,
    on_key_up: String,
    input: String,
    matched: String,
    end_key: String,
    end_mods: String,
    end_reason: String,
    in_progress: bool,
}

impl State {
    /// The state of a hotkey that was just made.
    fn new(name: &str, function: &str, options: &str) -> Self {
        State {
            name: name.to_string(),
            function: function.to_string(),
            enabled: true,
            options: options.to_string(),
            // The defaults of v2: the hotkey is not case sensitive, it is
            // found anywhere, and the text it types is shown.
            case_sensitive: false,
            find_anywhere: false,
            notify_non_text: false,
            visible_non_text: true,
            visible_text: true,
            backspace_is_undo: true,
            min_send_level: 0,
            timeout: 0,
            on_char: String::new(),
            on_end: String::new(),
            on_key_down: String::new(),
            on_key_up: String::new(),
            input: String::new(),
            matched: String::new(),
            end_key: String::new(),
            end_mods: String::new(),
            end_reason: String::new(),
            in_progress: false,
        }
    }
}

/// The object a `Hotkey` makes.
///
/// The object is the hotkey: it is registered with the desktop, and the
/// function of the script that it runs receives it.
pub(crate) fn construct(
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let name = match arguments.first() {
        Some(Value::Text(text)) => text.clone(),
        _ => return Err(fail("Parameter #1 of Hotkey is invalid.", line)),
    };
    // The action of a hotkey is a function of the script. A hotkey
    // without one is a hotkey of nothing, and v2 says so.
    let Some(action) = arguments.get(1) else {
        return Err(fail("Nonexistent hotkey.", line));
    };
    let (function, takes_name) = hotkeys::action_of(action, line)?;
    let options = match arguments.get(2) {
        Some(value) => value.to_text(),
        None => String::new(),
    };
    let state = State::new(&name, &function, &options);
    let object = object_with("hotkey", vec![(STATE, stored(&state))]);
    let shape = hotkeys::Shape {
        kind: Kind::Hotkey,
        name,
        function,
        takes_name,
        options,
        object: Some(object.clone()),
    };
    hotkeys::register(rt, shape, line)?;
    Ok(object)
}

/// A method of a `Hotkey`.
///
/// `None` means the method is not one of this class's: the runtime then
/// says the value has no such method.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let method = method.to_lowercase();
    match method.as_str() {
        "__new" => Some(made_again(this, line)),
        "keyopt" => Some(keyopt(this, arguments, rt, line)),
        "start" => Some(turn(this, rt, line, true)),
        "stop" => Some(turn(this, rt, line, false)),
        "wait" => Some(wait(this, arguments, rt, line)),
        _ => None,
    }
}

/// A property of a `Hotkey`.
///
/// `None` means the name is not a property of this class: the runtime then
/// looks in the property table of the object.
pub(crate) fn property(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match property_of(name, this, line) {
        Ok(Some(value)) => Some(Ok(value)),
        Ok(None) => None,
        Err(failure) => Some(Err(failure)),
    }
}

/// A property of a `Hotkey`, read.
///
/// `None` is the name of something that is not a property of the class:
/// the runtime then looks in the property table of the object.
fn property_of(name: &str, this: &Value, line: u32) -> Result<Option<Value>, RuntimeFailure> {
    let name = name.to_lowercase();
    let state = state_of(this, line)?;
    let value = match name.as_str() {
        "endkey" => Value::Text(state.end_key.clone()),
        "endmods" => Value::Text(state.end_mods.clone()),
        "endreason" => Value::Text(state.end_reason.clone()),
        "inprogress" => Value::Int(state.in_progress as i64),
        "input" => Value::Text(state.input.clone()),
        "match" => Value::Text(state.matched.clone()),
        "minsendlevel" => Value::Int(state.min_send_level),
        "timeout" => Value::Int(state.timeout),
        "casesensitive" => Value::Int(state.case_sensitive as i64),
        "findanywhere" => Value::Int(state.find_anywhere as i64),
        "notifynontext" => Value::Int(state.notify_non_text as i64),
        "visiblenontext" => Value::Int(state.visible_non_text as i64),
        "visibletext" => Value::Int(state.visible_text as i64),
        "backspaceisundo" => Value::Int(state.backspace_is_undo as i64),
        "onchar" => Value::Text(state.on_char.clone()),
        "onend" => Value::Text(state.on_end.clone()),
        "onkeydown" => Value::Text(state.on_key_down.clone()),
        "onkeyup" => Value::Text(state.on_key_up.clone()),
        _ => return Ok(None),
    };
    Ok(Some(value))
}

/// A property of a `Hotkey` that is written.
///
/// `None` means the class does not own the name: the runtime then
/// writes it in the property table of the object.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    match set_property_of(name, this, value, line) {
        Ok(true) => Some(Ok(())),
        Ok(false) => None,
        Err(failure) => Some(Err(failure)),
    }
}

/// A property of a `Hotkey` that is written.
///
/// `false` is the name of something that is not a property of the class:
/// the runtime then writes it in the property table of the object.
fn set_property_of(
    name: &str,
    this: &Value,
    value: Value,
    line: u32,
) -> Result<bool, RuntimeFailure> {
    let name = name.to_lowercase();
    let mut state = state_of(this, line)?;
    match name.as_str() {
        "minsendlevel" => state.min_send_level = number_of(&value, line)?,
        "timeout" => state.timeout = number_of(&value, line)?,
        "casesensitive" => state.case_sensitive = truthy(&value),
        "findanywhere" => state.find_anywhere = truthy(&value),
        "notifynontext" => state.notify_non_text = truthy(&value),
        "visiblenontext" => state.visible_non_text = truthy(&value),
        "visibletext" => state.visible_text = truthy(&value),
        "backspaceisundo" => state.backspace_is_undo = truthy(&value),
        "onchar" => state.on_char = text_of(&value),
        "onend" => state.on_end = text_of(&value),
        "onkeydown" => state.on_key_down = text_of(&value),
        "onkeyup" => state.on_key_up = text_of(&value),
        // What happened to the hotkey is not something a script writes.
        "endkey" | "endmods" | "endreason" | "inprogress" | "input" | "match" => {
            return Err(fail(
                &format!("the {name} of a hotkey is what happened to it, and a script cannot write it."),
                line,
            ))
        }
        _ => return Ok(false),
    }
    restore(this, &state);
    Ok(true)
}

/// The item of a `Hotkey`: the `__Item` of the class.
///
/// `None` means the class has no `__Item` of its own, and it has none:
/// the binary gives the class no `__Item` row.
#[allow(dead_code)]
pub(crate) fn item(
    _key: &Value,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    None
}

/// The options of the hotkey, read or written.
///
/// The options of v2 are the letters B, I, P, S and T, a letter may carry
/// a number, and the words On, Off and Toggle turn the hotkey on and off.
fn keyopt(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let mut state = state_of(this, line)?;
    let Some(options) = arguments.first() else {
        return Ok(Value::Text(state.options.clone()));
    };
    let options = options.to_text();
    hotkeys::check_options(&options, line)?;
    state.options = options.clone();
    restore(this, &state);
    for option in options.split_whitespace() {
        match option.to_lowercase().as_str() {
            "on" => {
                turn(this, rt, line, true)?;
            }
            "off" => {
                turn(this, rt, line, false)?;
            }
            "toggle" => {
                let on = !state.enabled;
                turn(this, rt, line, on)?;
            }
            _ => {}
        }
    }
    Ok(Value::Text(options))
}

/// Turn the hotkey on or off.
fn turn(this: &Value, rt: &mut Runtime, line: u32, on: bool) -> Result<Value, RuntimeFailure> {
    let Some(entry) = entry_of(rt, this) else {
        return Err(fail("this value is not a hotkey of the script.", line));
    };
    hotkeys::set_enabled(rt, &entry, on)?;
    let mut state = state_of(this, line)?;
    state.enabled = on;
    restore(this, &state);
    Ok(Value::Text(String::new()))
}

/// Wait for the hotkey to be pressed.
///
/// The wait of v2 ends when the hotkey is pressed, and it may wait for
/// ever. The tree waits between the statements of the script, and a wait
/// that ends nowhere would be a wait that never ends: the wait takes a
/// timeout, and it says so when it is given none.
fn wait(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let timeout = match arguments.first() {
        Some(value) => value
            .to_number()
            .ok_or_else(|| fail("the timeout of a wait is a number of milliseconds.", line))?,
        None => 0.0,
    };
    if timeout <= 0.0 {
        return Err(fail(
            "this runtime cannot wait for a hotkey without a timeout: the wait does not end between the statements of the script.",
            line,
        ));
    }
    let state = state_of(this, line)?;
    let deadline = rt.now_ms() + timeout as u64;
    loop {
        let fired = rt
            .desktop()
            .fired_hotkey()
            .map_err(|reason| fail(&reason, line))?;
        if let Some(fired) = fired {
            if fired == state.name {
                return Ok(Value::Text(String::new()));
            }
        }
        if rt.now_ms() >= deadline {
            return Ok(Value::Text(String::new()));
        }
        rt.sleep_ms(10);
    }
}

/// A hotkey that is already made cannot be made again.
fn made_again(_this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    Err(fail("a hotkey that is already made cannot be made again.", line))
}

/// The state of a hotkey, as the property table holds it.
fn stored(state: &State) -> Value {
    object_with(
        STATE_CLASS,
        vec![
            ("name", Value::Text(state.name.clone())),
            ("function", Value::Text(state.function.clone())),
            ("enabled", Value::Int(state.enabled as i64)),
            ("options", Value::Text(state.options.clone())),
            ("case_sensitive", Value::Int(state.case_sensitive as i64)),
            ("find_anywhere", Value::Int(state.find_anywhere as i64)),
            ("notify_non_text", Value::Int(state.notify_non_text as i64)),
            ("visible_non_text", Value::Int(state.visible_non_text as i64)),
            ("visible_text", Value::Int(state.visible_text as i64)),
            ("backspace_is_undo", Value::Int(state.backspace_is_undo as i64)),
            ("min_send_level", Value::Int(state.min_send_level)),
            ("timeout", Value::Int(state.timeout)),
            ("on_char", Value::Text(state.on_char.clone())),
            ("on_end", Value::Text(state.on_end.clone())),
            ("on_key_down", Value::Text(state.on_key_down.clone())),
            ("on_key_up", Value::Text(state.on_key_up.clone())),
            ("input", Value::Text(state.input.clone())),
            ("matched", Value::Text(state.matched.clone())),
            ("end_key", Value::Text(state.end_key.clone())),
            ("end_mods", Value::Text(state.end_mods.clone())),
            ("end_reason", Value::Text(state.end_reason.clone())),
            ("in_progress", Value::Int(state.in_progress as i64)),
        ],
    )
}

/// The state of a hotkey, read out of the property table.
fn state_of(this: &Value, line: u32) -> Result<State, RuntimeFailure> {
    let Value::Object(object) = this else {
        return Err(fail("this value is not a hotkey.", line));
    };
    let object = object.borrow();
    let Some(state) = object.properties.get(STATE) else {
        return Err(fail("this value is not a hotkey of the script.", line));
    };
    let Value::Object(state) = state else {
        return Err(fail("this value is not a hotkey of the script.", line));
    };
    let state = state.borrow();
    let text = |name: &str| {
        state
            .properties
            .get(name)
            .map(|value| value.to_text())
            .unwrap_or_default()
    };
    let number = |name: &str| {
        state
            .properties
            .get(name)
            .and_then(|value| value.to_number())
            .unwrap_or(0.0) as i64
    };
    let flag = |name: &str| number(name) != 0;
    Ok(State {
        name: text("name"),
        function: text("function"),
        enabled: flag("enabled"),
        options: text("options"),
        case_sensitive: flag("case_sensitive"),
        find_anywhere: flag("find_anywhere"),
        notify_non_text: flag("notify_non_text"),
        visible_non_text: flag("visible_non_text"),
        visible_text: flag("visible_text"),
        backspace_is_undo: flag("backspace_is_undo"),
        min_send_level: number("min_send_level"),
        timeout: number("timeout"),
        on_char: text("on_char"),
        on_end: text("on_end"),
        on_key_down: text("on_key_down"),
        on_key_up: text("on_key_up"),
        input: text("input"),
        matched: text("matched"),
        end_key: text("end_key"),
        end_mods: text("end_mods"),
        end_reason: text("end_reason"),
        in_progress: flag("in_progress"),
    })
}

/// Put the state back into the object.
fn restore(this: &Value, state: &State) {
    let Value::Object(object) = this else { return };
    object.borrow_mut().properties.insert(STATE.to_string(), stored(state));
}

/// The entry of the registry that carries this object.
fn entry_of(rt: &mut Runtime, this: &Value) -> Option<hotkeys::Hotkey> {
    let Value::Object(object) = this else { return None };
    rt.hotkeys()
        .iter()
        .find(|entry| match &entry.object {
            Some(Value::Object(other)) => Rc::ptr_eq(object, other),
            _ => false,
        })
        .cloned()
}

/// The value as a number.
fn number_of(value: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    value
        .to_number()
        .map(|number| number as i64)
        .ok_or_else(|| fail("this value is not a number.", line))
}

/// Whether the value counts as true.
fn truthy(value: &Value) -> bool {
    value.truthy()
}

/// The value as text.
fn text_of(value: &Value) -> String {
    value.to_text()
}

/// A failure of a hotkey, with the message of v2.
fn fail(message: &str, line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: message.to_string(),
        line,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::desktop::{Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers, Windows};
    use crate::runtime::Clock;
    use core::time::Duration;

    /// A desktop that remembers the hotkeys it was given.
    #[derive(Default)]
    struct Fake {
        registered: Vec<String>,
        unregistered: Vec<String>,
    }

    impl Keyboard for Fake {
        fn type_text(&mut self, text: &str) -> Result<String, String> {
            Ok(text.to_string())
        }

        fn press_key(&mut self, name: &str, _modifiers: &[String]) -> Result<String, String> {
            Ok(name.to_string())
        }
    }

    impl Pointer for Fake {
        fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
            Ok(String::new())
        }
    }

    impl Screen for Fake {
        fn capture(&mut self, _x: i64, _y: i64, _w: i64, _h: i64, _filename: &str) -> Result<String, String> {
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

    impl Windows for Fake {}
    impl Files for Fake {}
    impl Clipboard for Fake {}
    impl Timers for Fake {}

    impl Hotkeys for Fake {
        fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
            self.registered.push(name.to_string());
            Ok(name.to_string())
        }

        fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {
            self.unregistered.push(name.to_string());
            Ok(name.to_string())
        }

        fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
            Ok(None)
        }
    }

    impl Native for Fake {}
    impl Desktop for Fake {}

    /// A clock that does not move.
    struct TheClock;

    impl Clock for TheClock {
        fn now_ms(&self) -> u64 {
            0
        }

        fn sleep_ms(&mut self, _millis: u64) {}
    }

    /// The object of a hotkey that was made, and the runtime it lives in.
    fn made(script: &str) -> (Value, Fake) {
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::default();
        let mut clock = TheClock;
        let report = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        (Value::Text(String::new()), desktop)
    }

    /// The class answers every name of the binary.
    #[test]
    fn every_name_of_the_binary_is_carried() {
        let carried: Vec<&str> = super::HOTKEY.iter().map(|(name, _)| *name).collect();
        for name in names_of("hotkey") {
            assert!(carried.contains(&name.as_str()), "the Hotkey class does not carry {name}");
        }
        for name in &carried {
            let binary = names_of("hotkey");
            assert!(
                binary.iter().any(|entry| entry == name),
                "the Hotkey class carries {name}, and the binary does not"
            );
        }
    }

    /// The names the binary gives a class.
    fn names_of(class: &str) -> Vec<String> {
        let surface = include_str!("../../reference/ahk-v2.0.19.surface.tsv");
        let mut names: Vec<String> = Vec::new();
        for line in surface.lines() {
            let mut fields = line.trim_end().split('\t');
            let kind = fields.next().unwrap_or("");
            let owner = fields.next().unwrap_or("");
            let name = fields.next().unwrap_or("");
            if kind == "method" && owner.to_lowercase() == class {
                names.push(name.to_lowercase());
            }
        }
        names
    }

    /// A hotkey that is made is registered with the desktop.
    #[test]
    fn a_hotkey_that_is_made_is_registered() {
        let (_object, desktop) = made("Callback(hk) {\n}\nHotkey(\"^a\", Callback)\n");
        assert_eq!(desktop.registered, vec!["^a".to_string()]);
    }

    /// The options of a hotkey are read back, and the properties of it
    /// are the ones of v2.
    #[test]
    fn the_options_of_a_hotkey_are_read_back() {
        let script = "Callback(hk) {\n}\nhk := Hotkey(\"^a\", Callback, \"S\")\nopt := hk.KeyOpt()\ntimeout := hk.Timeout\nvisible := hk.VisibleText\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::default();
        let mut clock = TheClock;
        let report = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(report.variables.get("opt"), Some(&Value::Text("S".to_string())));
        assert_eq!(report.variables.get("timeout"), Some(&Value::Int(0)));
        assert_eq!(report.variables.get("visible"), Some(&Value::Int(1)));
    }

    /// A hotkey that is stopped is not watched any more.
    #[test]
    fn a_hotkey_that_is_stopped_is_not_watched() {
        let script = "Callback(hk) {\n}\nhk := Hotkey(\"^a\", Callback)\nhk.Stop()\n";
        let statements = crate::parse(script).expect("the script parses");
        let mut desktop = Fake::default();
        let mut clock = TheClock;
        let report = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
        assert!(report.ok, "the script failed: {:?}", report.error);
        assert_eq!(desktop.registered, vec!["^a".to_string()]);
        assert_eq!(desktop.unregistered, vec!["^a".to_string()]);
    }
}
