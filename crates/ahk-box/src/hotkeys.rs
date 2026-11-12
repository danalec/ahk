//! The hotkeys of the box.
//!
//! The keyboard of the box writes; the hotkeys read. The device that is
//! read is the physical keyboard of the machine: `/dev/input/eventN` is
//! opened, the events are read, and the state of the keys is kept.
//!
//! The device of the box itself is not read: a hotkey that fires on the
//! events of the keyboard of the box is a hotkey that fires on what the
//! script types, and that is a loop. The devices of the machine are in
//! `/proc/bus/input/devices`, and the block of the physical keyboard says
//! which event node it is. The virtual ones -- the device of the box among
//! them -- are skipped: their `Sysfs` is under `/devices/virtual/`, and
//! their `Phys` is empty.
//!
//! The device may also be named: the environment variable
//! `AHK_HOTKEY_DEVICE` says which one to read, and the watcher reads it
//! whatever it is. The variable may hold the path of the device or its
//! name, and a name is looked for in the devices of the machine -- the
//! virtual ones included, because the keyboard of the box is one of them
//! and its node is not known before the run.
//!
//! That is how the hotkeys are measured on the box: the variable points
//! the watcher at the device that the box writes, and a script that types
//! with `Send` fires its own hotkey.
//!
//! The watcher is here for the whole of the run, and the box runs one
//! script at a time: the functions of this module are the ones the
//! desktop of the box calls.

use ahk::hotkeys::Spec;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

// The device is opened with `O_NONBLOCK`, and the flag is a Unix thing:
// the trait that carries it only exists there.
#[cfg(target_os = "linux")]
use std::os::unix::fs::OpenOptionsExt;

/// The name of the environment variable that says which device to watch.
pub const DEVICE_VARIABLE: &str = ahk::hotkeys::DEVICE_VARIABLE;

/// The reason of a box that is not Linux.
#[cfg(not(target_os = "linux"))]
const NOT_LINUX: &str = "the hotkeys of this box watch a Linux device; this is not Linux";

/// The kind of the events of the kernel.
const EV_KEY: u16 = 1;

/// The codes of the modifiers of the keyboard.
const LEFTCTRL: u16 = 29;
const RIGHTCTRL: u16 = 97;
const LEFTSHIFT: u16 = 42;
/// The right shift: the table of the tree has no code for it, and the
/// kernel numbers it 54.
const RIGHTSHIFT: u16 = 54;
const LEFTALT: u16 = 56;
const RIGHTALT: u16 = 100;
const LEFTMETA: u16 = 125;
const RIGHTMETA: u16 = 126;

/// The size of one event of the kernel.
const EVENT_SIZE: usize = 24;

/// The time of an event of the kernel.
#[repr(C)]
struct Timeval {
    seconds: i64,
    microseconds: i64,
}

/// The event of the kernel: the time, the kind, the code and the value.
#[repr(C)]
struct InputEvent {
    time: Timeval,
    kind: u16,
    code: u16,
    value: i32,
}

/// A hotkey the script registered.
struct Entry {
    /// The name of the hotkey, as the script spelled it.
    name: String,
    /// The spelling of it, read by the language.
    spec: Spec,
}

/// A hotstring the script registered.
struct Hotstring {
    /// The name of it, as the script spelled it.
    name: String,
    /// The text that triggers it.
    text: String,
}

/// The watcher of the keyboard of the machine.
pub struct Watcher {
    /// The device the events are read from.
    ///
    /// The watcher of the cases has no device: the matching is what they
    /// are about.
    device: Option<File>,
    /// The reason the device could not be read, when it could not be.
    failure: Option<String>,
    /// The codes of the keys that are down.
    down: Vec<u16>,
    /// The hotkeys the script registered.
    hotkeys: Vec<Entry>,
    /// The hotstrings the script registered.
    hotstrings: Vec<Hotstring>,
    /// The text that was typed since the last hotstring.
    typed: String,
    /// The hotkey that was pressed since the last question.
    fired: Option<String>,
}

impl Watcher {
    /// The watcher of the keyboard of the machine.
    ///
    /// The device is not opened here: it is opened when the first events
    /// are wanted. The keyboard of the box is made when the script types
    /// for the first time, and a watcher that wanted a device that is not
    /// there yet would say that the hotkeys cannot be watched.
    pub fn open() -> Result<Self, String> {
        Ok(Watcher {
            device: None,
            failure: None,
            down: Vec::new(),
            hotkeys: Vec::new(),
            hotstrings: Vec::new(),
            typed: String::new(),
            fired: None,
        })
    }

    /// A watcher with no device, for the cases that are about the
    /// matching.
    #[cfg(test)]
    fn bare() -> Self {
        Watcher {
            device: None,
            failure: None,
            down: Vec::new(),
            hotkeys: Vec::new(),
            hotstrings: Vec::new(),
            typed: String::new(),
            fired: None,
        }
    }

    /// Read the events that are waiting, and answer what they say.
    pub fn read_events(&mut self) {
        let events = self.read();
        for event in &events {
            self.handle(event);
        }
    }

    /// The events that are waiting at the device.
    fn read(&mut self) -> Vec<InputEvent> {
        if self.device.is_none() {
            let path = match device_path() {
                Ok(path) => path,
                Err(reason) => {
                    self.failure = Some(reason);
                    return Vec::new();
                }
            };
            match open_device(&path) {
                Ok(device) => self.device = Some(device),
                Err(reason) => {
                    self.failure = Some(reason);
                    return Vec::new();
                }
            }
        }
        let Some(device) = self.device.as_mut() else {
            return Vec::new();
        };
        let mut buffer = [0u8; EVENT_SIZE * 64];
        let mut events: Vec<InputEvent> = Vec::new();
        loop {
            let read = match device.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => read,
                // The device is not blocking: there is nothing to read
                // now, and that is not a failure.
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(_) => break,
            };
            let mut offset = 0;
            while offset + EVENT_SIZE <= read {
                let event = unsafe {
                    std::ptr::read_unaligned(buffer.as_ptr().add(offset) as *const InputEvent)
                };
                offset += EVENT_SIZE;
                events.push(event);
            }
        }
        events
    }

    /// What one event of the kernel says.
    fn handle(&mut self, event: &InputEvent) {
        if event.kind != EV_KEY {
            return;
        }
        match event.value {
            // The key went down.
            1 => {
                if !self.down.contains(&event.code) {
                    self.down.push(event.code);
                }
                self.key_down(event.code);
            }
            // The key came up.
            0 => {
                self.down.retain(|held| *held != event.code);
                self.key_up(event.code);
            }
            // The key repeats: it is already down.
            _ => {}
        }
    }

    /// The hotkey that the key that went down is, if it is one.
    fn key_down(&mut self, code: u16) {
        for entry in &self.hotkeys {
            if entry.spec.up {
                continue;
            }
            if entry.spec.code != code {
                continue;
            }
            if !self.modifiers_hold(&entry.spec) {
                continue;
            }
            self.fired = Some(entry.name.clone());
            break;
        }
        // The text that was typed may be the text of a hotstring.
        if let Some(character) = self.character_of(code) {
            self.typed.push(character);
            self.hotstring_fired();
        }
    }

    /// The hotkey that the key that came up is, if it is one.
    fn key_up(&mut self, code: u16) {
        for entry in &self.hotkeys {
            if !entry.spec.up {
                continue;
            }
            if entry.spec.code != code {
                continue;
            }
            if !self.modifiers_hold(&entry.spec) {
                continue;
            }
            self.fired = Some(entry.name.clone());
            break;
        }
    }

    /// The hotstring that the text that was typed is, if it is one.
    ///
    /// The text of a hotstring is eaten: the hotstring takes it out of
    /// the way, the way v2 does it.
    fn hotstring_fired(&mut self) {
        for hotstring in &self.hotstrings {
            if self.typed.len() < hotstring.text.len() {
                continue;
            }
            let start = self.typed.len() - hotstring.text.len();
            if !self.typed[start..].eq_ignore_ascii_case(&hotstring.text) {
                continue;
            }
            self.fired = Some(self.typed[start..].to_string());
            self.typed.truncate(start);
            break;
        }
    }

    /// Whether the modifiers the hotkey wants are down.
    ///
    /// A hotkey with the `*` prefix fires for any combination of the
    /// modifiers: the ones it names have to be down, and the ones it does
    /// not name may be. A hotkey without it fires only when the modifiers
    /// that are down are the ones it names.
    fn modifiers_hold(&self, spec: &Spec) -> bool {
        for modifier in &spec.modifiers {
            if !self.down.contains(&code_of_modifier(modifier)) {
                return false;
            }
        }
        if spec.custom {
            return true;
        }
        for held in &self.down {
            if !is_modifier(*held) {
                continue;
            }
            let name = name_of_modifier(*held);
            if !spec.modifiers.contains(&name) {
                return false;
            }
        }
        true
    }

    /// The character the key that went down typed, if it typed one.
    fn character_of(&self, code: u16) -> Option<char> {
        let shifted = self.down.contains(&LEFTSHIFT) || self.down.contains(&RIGHTSHIFT);
        characters().get(&(code, shifted)).copied()
    }
}

/// The characters the keys of the layout type.
///
/// The table of the language says which key types a character, and the
/// watcher asks the other way round.
fn characters() -> &'static BTreeMap<(u16, bool), char> {
    static CHARACTERS: OnceLock<BTreeMap<(u16, bool), char>> = OnceLock::new();
    CHARACTERS.get_or_init(|| {
        let mut table: BTreeMap<(u16, bool), char> = BTreeMap::new();
        for code in 0x20u8..0x7f {
            let character = code as char;
            if let Some((key, shifted)) = ahk::keys::character_stroke(character) {
                table.entry((key, shifted)).or_insert(character);
            }
        }
        table
    })
}

/// The code of a modifier, by the name the runtime gives it.
fn code_of_modifier(name: &str) -> u16 {
    match name {
        "rctrl" => RIGHTCTRL,
        "shift" | "lshift" => LEFTSHIFT,
        "rshift" => RIGHTSHIFT,
        "alt" | "lalt" => LEFTALT,
        "ralt" => RIGHTALT,
        "win" | "lwin" => LEFTMETA,
        "rwin" => RIGHTMETA,
        _ => LEFTCTRL,
    }
}

/// The name of a modifier, by the name the runtime gives it.
fn name_of_modifier(code: u16) -> String {
    match code {
        RIGHTCTRL => "rctrl".to_string(),
        LEFTSHIFT => "shift".to_string(),
        RIGHTSHIFT => "rshift".to_string(),
        LEFTALT => "alt".to_string(),
        RIGHTALT => "ralt".to_string(),
        LEFTMETA => "win".to_string(),
        RIGHTMETA => "rwin".to_string(),
        _ => "ctrl".to_string(),
    }
}

/// Whether a code is one of the modifiers.
fn is_modifier(code: u16) -> bool {
    matches!(
        code,
        LEFTCTRL | RIGHTCTRL | LEFTSHIFT | RIGHTSHIFT | LEFTALT | RIGHTALT | LEFTMETA | RIGHTMETA
    )
}

/// The name of a key, the way the runtime names it.
fn name_of(code: u16) -> String {
    if is_modifier(code) {
        return name_of_modifier(code);
    }
    ahk::keys::EVDEV
        .iter()
        .find(|(_, key_code)| *key_code == code)
        .map(|(name, _)| name.to_lowercase())
        .unwrap_or_default()
}

/// The watcher of the keyboard.
///
/// The box runs one script at a time, and the watcher belongs to the
/// machine: it is opened when the first hotkey is registered.
static WATCHER: Mutex<Option<Watcher>> = Mutex::new(None);

/// The watcher, opened when it is not there yet.
fn watcher() -> Result<MutexGuard<'static, Option<Watcher>>, String> {
    let mut guard = WATCHER
        .lock()
        .map_err(|_| "the watcher of the keyboard is broken.".to_string())?;
    if guard.is_none() {
        *guard = Some(Watcher::open()?);
    }
    Ok(guard)
}

/// The watcher of the keyboard, and the keyboard it watches.
fn watching() -> Result<MutexGuard<'static, Option<Watcher>>, String> {
    watcher()
}

/// Register a hotkey with the watcher.
///
/// The name is the spelling of the script. A name that is a hotstring is
/// a text that fires a function when it is typed, and everything else is
/// a key.
pub fn register(name: &str) -> Result<String, String> {
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    if let Some(text) = ahk::hotkeys::hotstring_text(name) {
        watcher
            .hotstrings
            .retain(|hotstring| hotstring.name != name);
        watcher.hotstrings.push(Hotstring {
            name: name.to_string(),
            text,
        });
        return Ok(name.to_string());
    }
    let spec = ahk::hotkeys::parse(name)?;
    watcher.hotkeys.retain(|entry| entry.name != name);
    watcher.hotkeys.push(Entry {
        name: name.to_string(),
        spec,
    });
    Ok(name.to_string())
}

/// Take a hotkey away from the watcher.
pub fn unregister(name: &str) -> Result<String, String> {
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    watcher.hotkeys.retain(|entry| entry.name != name);
    watcher
        .hotstrings
        .retain(|hotstring| hotstring.name != name);
    Ok(name.to_string())
}

/// The hotkeys that are registered.
pub fn list() -> Result<Vec<String>, String> {
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    let mut names: Vec<String> = watcher
        .hotkeys
        .iter()
        .map(|entry| entry.name.clone())
        .collect();
    names.extend(
        watcher
            .hotstrings
            .iter()
            .map(|hotstring| hotstring.name.clone()),
    );
    Ok(names)
}

/// The hotkey that was pressed since the last question.
///
/// The name of a hotkey is the name the script gave it, and the name of
/// a hotstring is the text that was typed.
pub fn fired() -> Result<Option<String>, String> {
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    watcher.read_events();
    if let Some(reason) = watcher.failure.take() {
        return Err(reason);
    }
    Ok(watcher.fired.take())
}

/// Whether a key is down right now.
pub fn key_is_down(name: &str) -> Result<bool, String> {
    let code = ahk::keys::resolve_name(name)
        .map_err(|_| format!("the keyboard of this box has no key named {name}."))?;
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    watcher.read_events();
    Ok(watcher.down.contains(&code))
}

/// The keys that are down right now.
pub fn keys_down() -> Result<Vec<String>, String> {
    let mut guard = watching()?;
    let watcher = guard
        .as_mut()
        .ok_or_else(|| "the keyboard of this box cannot be watched.".to_string())?;
    watcher.read_events();
    Ok(watcher.down.iter().map(|code| name_of(*code)).collect())
}

/// The device the events of the keyboard are read from.
///
/// The variable may hold the path of the device or its name: a path that
/// is there is the device, and a name is looked for in the devices of the
/// machine.
fn device_path() -> Result<PathBuf, String> {
    match std::env::var(DEVICE_VARIABLE) {
        Ok(name) if !name.is_empty() => {
            let path = PathBuf::from(&name);
            if path.exists() {
                return Ok(path);
            }
            device_named(&name).ok_or_else(|| {
                format!("the device {name:?} is not one of the devices of the machine.")
            })
        }
        _ => device_of_the_machine(),
    }
}

/// The event node of the device with this name.
///
/// The text is the one of `/proc/bus/input/devices`, and the name is the
/// one of the `N: Name=` field. The virtual devices are not skipped: the
/// keyboard of the box is one of them, and it is the one that the
/// measurement watches.
#[cfg(target_os = "linux")]
fn device_named(name: &str) -> Option<PathBuf> {
    let text = std::fs::read_to_string("/proc/bus/input/devices").ok()?;
    let wanted = name.to_lowercase();
    for block in text.split("\n\n") {
        if field(block, "N: Name=").to_lowercase() != wanted {
            continue;
        }
        let handlers = field(block, "H: Handlers=");
        let event = handlers
            .split_whitespace()
            .find(|word| word.starts_with("event"))?;
        return Some(PathBuf::from(format!("/dev/input/{event}")));
    }
    None
}

/// The device the events of the keyboard are read from, on a box that is
/// not Linux.
#[cfg(not(target_os = "linux"))]
fn device_named(_name: &str) -> Option<PathBuf> {
    None
}

/// The device the events of the keyboard are read from.
///
/// The devices of the machine are in `/proc/bus/input/devices`, and the
/// block of the physical keyboard says which event node it is.
#[cfg(target_os = "linux")]
fn device_of_the_machine() -> Result<PathBuf, String> {
    let text = std::fs::read_to_string("/proc/bus/input/devices")
        .map_err(|error| format!("the devices of the machine cannot be read: {error}"))?;
    match device_in(&text) {
        Some(event) => Ok(PathBuf::from(format!("/dev/input/{event}"))),
        None => Err(
            "this machine has no keyboard to watch: the devices of /proc/bus/input/devices say so."
                .to_string(),
        ),
    }
}

/// The device of the keyboard, on a box that is not Linux.
#[cfg(not(target_os = "linux"))]
fn device_of_the_machine() -> Result<PathBuf, String> {
    Err(NOT_LINUX.to_string())
}

/// The event node of the physical keyboard, in the devices of the
/// machine.
///
/// The text is the one of `/proc/bus/input/devices`. The keyboard of the
/// machine is the one whose name says it is a keyboard, whose `Phys` is
/// not empty and whose `Sysfs` is not under `/devices/virtual/`: the
/// devices that the box makes are virtual, and a hotkey that fires on
/// them is a hotkey that fires on what the script types.
#[cfg(any(target_os = "linux", test))]
fn device_in(text: &str) -> Option<String> {
    for block in text.split("\n\n") {
        let name = field(block, "N: Name=");
        let phys = field(block, "P: Phys=");
        let sysfs = field(block, "S: Sysfs=");
        let handlers = field(block, "H: Handlers=");
        if !name.to_lowercase().contains("keyboard") {
            continue;
        }
        if sysfs.starts_with("/devices/virtual/") || phys.is_empty() {
            continue;
        }
        let event = handlers
            .split_whitespace()
            .find(|word| word.starts_with("event"))?;
        return Some(event.to_string());
    }
    None
}

/// The value of a field of a block of the devices.
#[cfg(any(target_os = "linux", test))]
fn field(block: &str, prefix: &str) -> String {
    for line in block.lines() {
        if let Some(rest) = line.strip_prefix(prefix) {
            return rest.trim().trim_matches('"').to_string();
        }
    }
    String::new()
}

/// The device the events are read from.
#[cfg(target_os = "linux")]
fn open_device(path: &Path) -> Result<File, String> {
    std::fs::OpenOptions::new()
        .read(true)
        // The device is not blocking: the watcher asks it for the events
        // that are there and comes back.
        .custom_flags(crate::sys::O_NONBLOCK)
        .open(path)
        .map_err(|error| format!("the keyboard of this box cannot be read: {error}"))
}

/// The device the events are read from, on a box that is not Linux.
#[cfg(not(target_os = "linux"))]
fn open_device(_path: &Path) -> Result<File, String> {
    Err(NOT_LINUX.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The devices of the box, measured on 2026-10-28.
    const DEVICES: &str = "\
I: Bus=0011 Vendor=0001 Product=0001 Version=ab41
N: Name=\"AT Translated Set 2 keyboard\"
P: Phys=isa0060/serio0
S: Sysfs=/devices/platform/i8042/serio0/input/input1
U: uniq=
H: Handlers=sysrq kbd mouse0 event1
B: PROP=0

I: Bus=0019 Vendor=0000 Product=0001 Version=0000
N: Name=\"libvirtualhid Keyboard\"
P: Phys=
S: Sysfs=/devices/virtual/input/input21
U: uniq=
H: Handlers=sysrq kbd event21
B: PROP=0

I: Bus=0019 Vendor=0000 Product=0006 Version=0000
N: Name=\"yubikey YubiKey OTP+FIDO+CCID\"
P: Phys=usb-0000:00:14.0-2/input0
S: Sysfs=/devices/pci0000:00/0000:00:14.0/usb1/1-2/input/input22
U: uniq=
H: Handlers=sysrq kbd event22
B: PROP=0
";

    /// The keyboard of the machine is the one that is not virtual.
    #[test]
    fn the_physical_keyboard_is_the_one_that_is_watched() {
        assert_eq!(device_in(DEVICES), Some("event1".to_string()));
    }

    /// A device that is not a keyboard is not the one.
    #[test]
    fn a_device_that_is_not_a_keyboard_is_not_watched() {
        let devices = DEVICES.replace("AT Translated Set 2 keyboard", "Some Mouse");
        assert_eq!(device_in(&devices), None);
    }

    /// The virtual keyboards are skipped: one of them is the device that
    /// the box makes, and a hotkey that fires on it fires on what the
    /// script types.
    #[test]
    fn the_virtual_keyboards_are_skipped() {
        let devices = DEVICES.replace(
            "N: Name=\"AT Translated Set 2 keyboard\"\nP: Phys=isa0060/serio0\nS: Sysfs=/devices/platform/i8042/serio0/input/input1",
            "N: Name=\"Virtual Keyboard\"\nP: Phys=\nS: Sysfs=/devices/virtual/input/input21",
        );
        assert_eq!(device_in(&devices), None);
    }

    /// A hotkey fires when the key it names goes down with its modifiers.
    #[test]
    fn a_hotkey_fires_when_the_key_goes_down() {
        let mut watcher = Watcher::bare();
        watcher.hotkeys.push(Entry {
            name: "^a".to_string(),
            spec: ahk::hotkeys::parse("^a").unwrap(),
        });
        watcher.handle(&event(LEFTCTRL, 1));
        watcher.handle(&event(30, 1));
        assert_eq!(watcher.fired, Some("^a".to_string()));
    }

    /// A hotkey that names a modifier does not fire without it.
    #[test]
    fn a_hotkey_without_its_modifier_does_not_fire() {
        let mut watcher = Watcher::bare();
        watcher.hotkeys.push(Entry {
            name: "^a".to_string(),
            spec: ahk::hotkeys::parse("^a").unwrap(),
        });
        watcher.handle(&event(30, 1));
        assert_eq!(watcher.fired, None);
    }

    /// A hotkey with the `*` prefix fires with the modifiers that are
    /// down.
    #[test]
    fn a_hotkey_with_the_star_prefix_fires_with_more_modifiers() {
        let mut watcher = Watcher::bare();
        watcher.hotkeys.push(Entry {
            name: "*a".to_string(),
            spec: ahk::hotkeys::parse("*a").unwrap(),
        });
        watcher.handle(&event(LEFTCTRL, 1));
        watcher.handle(&event(30, 1));
        assert_eq!(watcher.fired, Some("*a".to_string()));
    }

    /// The ` Up` variant of a hotkey fires when the key comes up.
    #[test]
    fn the_up_variant_fires_when_the_key_comes_up() {
        let mut watcher = Watcher::bare();
        watcher.hotkeys.push(Entry {
            name: "a Up".to_string(),
            spec: ahk::hotkeys::parse("a Up").unwrap(),
        });
        watcher.handle(&event(30, 1));
        assert_eq!(watcher.fired, None);
        watcher.handle(&event(30, 0));
        assert_eq!(watcher.fired, Some("a Up".to_string()));
    }

    /// A hotstring fires when the text that was typed is its text.
    #[test]
    fn a_hotstring_fires_when_the_text_is_typed() {
        let mut watcher = Watcher::bare();
        watcher.hotstrings.push(Hotstring {
            name: ":btw:".to_string(),
            text: "btw".to_string(),
        });
        for code in [48, 20, 17] {
            watcher.handle(&event(code, 1));
            watcher.handle(&event(code, 0));
        }
        assert_eq!(watcher.fired, Some("btw".to_string()));
    }

    /// The text of a hotstring is eaten, and the text that follows it is
    /// the text of the next one.
    #[test]
    fn the_text_of_a_hotstring_is_eaten() {
        let mut watcher = Watcher::bare();
        watcher.hotstrings.push(Hotstring {
            name: ":btw:".to_string(),
            text: "btw".to_string(),
        });
        for code in [48, 20, 17, 48] {
            watcher.handle(&event(code, 1));
            watcher.handle(&event(code, 0));
        }
        assert_eq!(watcher.fired, Some("btw".to_string()));
        assert_eq!(watcher.typed, "b".to_string());
    }

    /// An event of the kernel, for the cases.
    fn event(code: u16, value: i32) -> InputEvent {
        InputEvent {
            time: Timeval {
                seconds: 0,
                microseconds: 0,
            },
            kind: EV_KEY,
            code,
            value,
        }
    }
}
