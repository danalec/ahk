//! The sound of the machine.
//!
//! The machine has a speaker, and `Beep` is the way to make it speak:
//! the call takes the frequency of the sound and how long it lasts, and
//! it is of Windows 2000. The call blocks the thread of the script for
//! the length of the sound, and the tree says so.
//!
//! The volume of the machine is the volume of the device of the sound:
//! the tree answers it with the endpoint API of the sound, and the
//! devices of the machine are the ones that it publishes.

use windows_sys::Win32::System::Diagnostics::Debug::Beep;

/// The machine speaks.
///
/// The frequency is in the hertz and the duration in the milliseconds.
/// The answer of the tree is the text of the sound, the way the v2
/// answers it.
pub fn sound_beep(frequency: f64, duration_ms: u64) -> Result<String, String> {
    let frequency = if frequency <= 0.0 {
        0x800
    } else {
        frequency as u32
    };
    unsafe {
        if Beep(frequency, duration_ms as u32) == 0 {
            return Err("the machine did not speak.".to_string());
        }
    }
    Ok(format!("BEEP {frequency} {duration_ms}"))
}

/// A file of the machine is played.
///
/// The machine plays the file by its own: the name of it is the name of
/// the file, and the sound of the machine goes on while the script goes
/// on.
pub fn a_file_is_played(path: &str) -> Result<String, String> {
    use windows_sys::Win32::Media::Audio::{PlaySoundW, SND_ASYNC, SND_FILENAME, SND_NODEFAULT};
    // O arquivo que nao esta nao toca, e o `PlaySoundW` do modo
    // assincrono nao diz que falhou: medido no binario do v2.0.31 em
    // O `SoundPlay("zzz.wav")` recusa com "Failed".
    if !std::path::Path::new(path).is_file() {
        return Err(format!("the machine did not play the file {path}."));
    }
    let wide = super::sys::Wide::of(path);
    unsafe {
        if PlaySoundW(
            wide.as_ptr(),
            std::ptr::null_mut(),
            SND_FILENAME | SND_ASYNC | SND_NODEFAULT,
        ) == 0
        {
            return Err(format!("the machine did not play the file {path}."));
        }
    }
    Ok(path.to_string())
}
