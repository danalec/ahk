//! The machine, the session and the user.
//!
//! The runtime does no I/O of its own: the names of the machine -- the
//! user, the computer, the shell, the folders, the version of the
//! operating system, the language, the rights of the script and the idle
//! times -- are asked of the desktop, and the box is the desktop.
//!
//! The box is a Linux machine, and the answers are read where Linux keeps
//! them: the environment of the session, the files of the XDG base
//! directories and the release of the kernel. The values were measured on
//! the box on 2026-10-29: the user is `danalec`, the computer is
//! `pod158c`, the shell is `/usr/bin/zsh`, and the version of the
//! operating system is the release of the kernel.
//!
//! The two things that only Linux has -- the release of the kernel and the
//! effective user id -- are behind a gate, and the other systems answer
//! with the reason they cannot answer.

use std::path::Path;

use ahk::desktop::{Folders, Idle};

/// The name of the user that launched the session.
///
/// The session carries it in `USER`: the box has one session and one user,
/// and the shell of the session sets the name.
pub fn user_name() -> Result<String, String> {
    from_the_environment("USER", "name of the user")
}

/// The name of the computer, as the network sees it.
///
/// The kernel knows it, and the kernel is the one that answers even when
/// the session carries no `HOSTNAME`: measured on the box on 2026-10-29, a
/// run that comes from `ssh` has no `HOSTNAME` in its environment, and the
/// computer is `pod158c`.
pub fn computer_name() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    if let Some(name) = the_name_of_the_machine() {
        return Ok(name);
    }
    from_the_environment("HOSTNAME", "name of the computer")
}

/// The name of the machine, as the kernel keeps it.
#[cfg(target_os = "linux")]
fn the_name_of_the_machine() -> Option<String> {
    let mut buffer = [0u8; 256];
    let asked =
        unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut libc::c_char, buffer.len()) };
    if asked != 0 {
        return None;
    }
    let end = buffer
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(buffer.len());
    let name = String::from_utf8_lossy(&buffer[..end]).trim().to_string();
    if name.is_empty() { None } else { Some(name) }
}

/// The program that runs a command of the session.
///
/// v2 calls it `A_ComSpec`, and on the machine of the box it is the shell
/// of the session: `/usr/bin/zsh`.
pub fn shell() -> Result<String, String> {
    from_the_environment("SHELL", "shell")
}

/// A value of the environment of the session.
///
/// A variable the session does not carry is the reason of the failure, and
/// the reason says which one it was.
fn from_the_environment(name: &str, what: &str) -> Result<String, String> {
    match std::env::var(name) {
        Ok(value) if !value.is_empty() => Ok(value),
        _ => Err(format!(
            "the session does not carry the {what}: the environment has no {name}."
        )),
    }
}

/// The version of the operating system.
///
/// On this machine the version of the operating system is the release of
/// the kernel, and the kernel keeps it in `/proc/sys/kernel/osrelease`.
pub fn os_version() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        let release = std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map_err(|error| format!("the release of the kernel cannot be read: {error}"))?;
        Ok(release.trim().to_string())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err("the version of the operating system is the release of the kernel, and this is not Linux."
            .to_string())
    }
}

/// The language of the session.
///
/// The session carries it in `LC_ALL`, `LANG` or `LANGUAGE`, in that order
/// of preference, and the file of the system is the last word:
/// `/etc/locale.conf` has `LANG=en_US.UTF-8` on the box.
pub fn language() -> Result<String, String> {
    for name in ["LC_ALL", "LANG", "LANGUAGE"] {
        if let Ok(value) = std::env::var(name) {
            if !value.is_empty() {
                return Ok(value);
            }
        }
    }
    if let Some(locale) = the_locale_of_the_system() {
        return Ok(locale);
    }
    Err(
        "the session does not carry the language: the environment has no LC_ALL, LANG or \
         LANGUAGE, and /etc/locale.conf is not there."
            .to_string(),
    )
}

/// The language of the system, from `/etc/locale.conf`.
fn the_locale_of_the_system() -> Option<String> {
    let text = std::fs::read_to_string("/etc/locale.conf").ok()?;
    for line in text.lines() {
        let Some((name, value)) = line.split_once('=') else {
            continue;
        };
        if name.trim() != "LANG" {
            continue;
        }
        let value = value.trim().trim_matches('"');
        if !value.is_empty() {
            return Some(value.to_string());
        }
    }
    None
}

/// Whether the script runs with the rights of an administrator.
///
/// On Linux the rights of an administrator are the effective user id: the
/// root is the one that is zero. Measured on the box on 2026-10-29, the
/// user of the session is the 1000, and a script of that session is not
/// elevated.
pub fn is_admin() -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        Ok(unsafe { libc::geteuid() == 0 })
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(
            "the rights of an administrator are the effective user id, and this is not Linux."
                .to_string(),
        )
    }
}

/// The folders of the machine.
///
/// The folders are the ones of the XDG base directories. A folder the
/// machine does not have is not in the answer: the tree asks the machine,
/// and the machine does not have what is not there.
pub fn folders() -> Result<Folders, String> {
    let home = std::env::var("HOME").ok().filter(|value| !value.is_empty());
    let data = std::env::var("XDG_DATA_HOME")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| home.as_ref().map(|home| format!("{home}/.local/share")));
    let config = std::env::var("XDG_CONFIG_HOME")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| home.as_ref().map(|home| format!("{home}/.config")));
    let temp = std::env::var("TMPDIR")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| Some("/tmp".to_string()));
    Ok(Folders {
        app_data: on_the_disk(data.clone()),
        app_data_common: on_the_disk(Some("/usr/share".to_string())),
        desktop: on_the_disk(desktop_of_the_user(config.as_deref(), home.as_deref())),
        // The machine has no desktop for everybody: the desktop of the
        // session is the one of the user.
        desktop_common: None,
        start_menu: on_the_disk(data.as_ref().map(|data| format!("{data}/applications"))),
        start_menu_common: on_the_disk(Some("/usr/share/applications".to_string())),
        // The machine has no programs folder: the start menu of the user
        // is the folder of the applications.
        programs: None,
        programs_common: None,
        startup: on_the_disk(config.as_ref().map(|config| format!("{config}/autostart"))),
        startup_common: on_the_disk(Some("/etc/xdg/autostart".to_string())),
        my_documents: on_the_disk(documents_of_the_user(config.as_deref(), home.as_deref())),
        temp: on_the_disk(temp),
    })
}

/// The desktop of the user, as `xdg-user-dir-update` wrote it.
fn desktop_of_the_user(config: Option<&str>, home: Option<&str>) -> Option<String> {
    user_dir(config, home, "XDG_DESKTOP_DIR").or_else(|| home.map(|home| format!("{home}/Desktop")))
}

/// The documents of the user, as `xdg-user-dir-update` wrote them.
fn documents_of_the_user(config: Option<&str>, home: Option<&str>) -> Option<String> {
    user_dir(config, home, "XDG_DOCUMENTS_DIR")
        .or_else(|| home.map(|home| format!("{home}/Documents")))
}

/// A folder of the user, as the file of the user directories writes it.
///
/// The file is `user-dirs.dirs`, and the tool that writes it is
/// `xdg-user-dir-update`.
fn user_dir(config: Option<&str>, home: Option<&str>, key: &str) -> Option<String> {
    let config = config?;
    let text = std::fs::read_to_string(format!("{config}/user-dirs.dirs")).ok()?;
    the_user_dir(&text, home, key)
}

/// A folder of the user, as the text of the file of the user directories
/// writes it.
///
/// The file is written with the home of the user in it:
/// `XDG_DESKTOP_DIR="$HOME/Desktop"`.
fn the_user_dir(text: &str, home: Option<&str>, key: &str) -> Option<String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        let Some((name, value)) = trimmed.split_once('=') else {
            continue;
        };
        if name.trim() != key {
            continue;
        }
        let value = value.trim().trim_matches('"');
        if !value.is_empty() {
            return Some(expand_the_home(value, home));
        }
    }
    None
}

/// A path with the home of the user put in it.
fn expand_the_home(value: &str, home: Option<&str>) -> String {
    let Some(home) = home else {
        return value.to_string();
    };
    if let Some(rest) = value.strip_prefix("${HOME}") {
        return format!("{home}{rest}");
    }
    if let Some(rest) = value.strip_prefix("$HOME") {
        return format!("{home}{rest}");
    }
    value.to_string()
}

/// A folder that is on the disk.
///
/// A folder the machine does not have is not in the answer.
fn on_the_disk(path: Option<String>) -> Option<String> {
    let path = path?;
    if Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

/// How long the session has been idle, in milliseconds.
///
/// The box has no idle monitor: measured on 2026-10-29, the XDG portal of
/// the box does not publish the `org.freedesktop.portal.Idle` interface,
/// and there is no idle time to read.
pub fn idle_times() -> Result<Idle, String> {
    Err(
        "the XDG portal of this box does not publish the Idle interface, and the session has no \
         idle monitor."
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::the_user_dir;

    /// The file of the user directories is written with the home of the
    /// user in it, and the home is put in it.
    #[test]
    fn a_folder_of_the_user_carries_the_home() {
        let text =
            "# This file is written by xdg-user-dir-update\nXDG_DESKTOP_DIR=\"$HOME/Desktop\"\n";
        assert_eq!(
            the_user_dir(text, Some("/home/danalec"), "XDG_DESKTOP_DIR").as_deref(),
            Some("/home/danalec/Desktop")
        );
    }

    /// A key that the file does not carry is not a folder.
    #[test]
    fn a_key_that_is_not_there_is_not_a_folder() {
        let text = "XDG_DESKTOP_DIR=\"$HOME/Desktop\"\n";
        assert_eq!(
            the_user_dir(text, Some("/home/danalec"), "XDG_DOCUMENTS_DIR"),
            None
        );
    }
}
