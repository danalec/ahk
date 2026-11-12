//! The lock keys of the box.
//!
//! `CapsLock`, `NumLock` and `ScrollLock` are the keys of the keyboard whose
//! state is a switch and not a hold: the state of the key is the state of the
//! LED of the keyboard, and the kernel publishes that LED under
//! `/sys/class/leds`.
//!
//! Measured on the box on 2026-11-09: the physical keyboard of the box has
//! three LED devices, `/sys/class/leds/input1::capslock/brightness`,
//! `input1::numlock` and `input1::scrolllock`, and each of them holds "0" or
//! "1". The compositor is what turns them: the key goes to the compositor,
//! the compositor keeps the lock state and writes the LED.
//!
//! The state of a key is therefore read here, and the *change* of it is the
//! work of the keyboard of the box: the box sends the lock key, and this
//! module is what says whether the switch took.

/// The LED devices of the machine, by the name of the lock key.
///
/// The name of the file is `input<N>::<lock>`, and the lock is the lowercase
/// name of the key: "capslock", "numlock", "scrolllock".
fn the_led_of(lock: &str) -> Option<std::path::PathBuf> {
    let wanted = format!("::{}", lock.to_ascii_lowercase());
    let entries = std::fs::read_dir("/sys/class/leds").ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.ends_with(&wanted) {
            continue;
        }
        let brightness = entry.path().join("brightness");
        if brightness.exists() {
            return Some(brightness);
        }
    }
    None
}

/// Whether a lock key is on.
///
/// The answer is the LED of the key: "1" is on and "0" is off. A machine with
/// no LED of the key cannot say, and says why instead of guessing.
pub fn is_on(lock: &str) -> Result<bool, String> {
    let Some(path) = the_led_of(lock) else {
        return Err(format!(
            "this machine has no LED of the {lock} to read, and the state of a lock key is the \
             state of its LED."
        ));
    };
    let text = std::fs::read_to_string(&path).map_err(|error| the_message_of(&error))?;
    Ok(text.trim() == "1")
}

/// The message of a failure of the machine, the way the tree says it: the
/// code of the system in front of what it says.
fn the_message_of(error: &std::io::Error) -> String {
    let text = error.to_string();
    let text = text.split(" (os error ").next().unwrap_or(&text);
    match error.raw_os_error() {
        Some(code) => format!("({code}) {text}"),
        None => text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::is_on;

    /// The box has the LEDs of the three lock keys, and the state of each of
    /// them is a number of the LED.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_lock_keys_of_the_box_have_a_state() {
        for lock in ["CapsLock", "NumLock", "ScrollLock"] {
            // The state of a lock key is the state of the LED of the key,
            // and the LED is a device of the machine: the box has the
            // three of them, and a machine of a CI has none. The answer
            // is the state where the machine has the LED, and the reason
            // of the machine where it has not -- never a state that was
            // made up.
            match is_on(lock) {
                Ok(state) => {
                    // The test does not change the state of the session:
                    // it only reads it.
                    let _ = state;
                }
                Err(reason) => {
                    assert!(
                        reason.contains("no LED of the"),
                        "the reason of a machine with no LED names the LED: {reason}"
                    );
                }
            }
        }
    }

    /// A machine with no LED of the key says so, and does not answer a state
    /// it does not know.
    #[test]
    fn a_lock_key_that_is_not_there_has_no_state() {
        let answer = is_on("NoSuchLock");
        assert!(answer.is_err(), "a lock that is not there has no state");
    }
}
