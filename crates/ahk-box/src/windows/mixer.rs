//! The sound of the era of XP: the mixer API of winmm.
//!
//! The volume of the modern tree is the COM of Vista -- the
//! *MMDeviceEnumerator class* and the `IAudioEndpointVolume` of the
//! device. That class is not registered on Windows XP (measured on the
//! two guests of the lab: the answer is `0x80040154`), so on XP the
//! tree asks the mixer of winmm instead, which is the way of the v1
//! and is of Windows 2000.
//!
//! The mixer is the shape the v1 was built on: a device has lines, a
//! line has a component type (`MIXERLINE_COMPONENTTYPE_*` -- the very
//! numbers of `topology.rs`, which the v1 map of the components
//! already uses), and a line has controls of volume and of mute. The
//! volume of a control is a number between its bounds, and the tree
//! answers it between zero and a hundred, the way the v1 does.
//!
//! The functions here mirror the public ones of `audio.rs`, and
//! `audio.rs` calls them when the class of Vista is not there.

use core::ffi::c_void;

use windows_sys::Win32::Media::Audio::{
    mixerClose, mixerGetControlDetailsW, mixerGetDevCapsW, mixerGetLineControlsW,
    mixerGetLineInfoW, mixerGetNumDevs, mixerOpen, mixerSetControlDetails, CALLBACK_NULL,
    HMIXER, MIXERCONTROLDETAILS, MIXERCONTROLDETAILS_BOOLEAN, MIXERCONTROLDETAILS_UNSIGNED,
    MIXERCONTROLW, MIXERCONTROL_CONTROLTYPE_MUTE, MIXERCONTROL_CONTROLTYPE_VOLUME,
    MIXERCAPSW, MIXERLINECONTROLSW, MIXERLINEW, MIXER_GETCONTROLDETAILSF_VALUE,
    MIXER_GETLINECONTROLSF_ONEBYTYPE, MIXER_GETLINEINFOF_COMPONENTTYPE,
    MIXER_SETCONTROLDETAILSF_VALUE,
};

use ahk::desktop::{SoundControl, SoundFailure};

use super::audio::{is_the_master, the_component_and_the_control, the_component_type_of};
use super::topology::{DST_SPEAKERS, SRC_UNDEFINED};

/// Whether the mixer is the way of the sound on this machine.
///
/// The class of Vista is asked of the registry once: when it is not
/// there, the machine is of the era of XP and the mixer answers.
pub fn the_mixer_is_the_way() -> bool {
    static THE_WAY: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *THE_WAY.get_or_init(|| super::com::the_class_of_the_devices_of_the_sound().is_err())
}

/// The mixer of the device that the script names.
///
/// The number of the device is its place in the list of the mixers of
/// the machine, and the count begins at one (the way of the v1); the
/// empty name is the first mixer, the one of the system.
fn the_mixer_of(device: &str) -> Result<HMIXER, SoundFailure> {
    let number = device.trim();
    let index = if number.is_empty() {
        0
    } else {
        match number.parse::<u32>() {
            Ok(number) => number.saturating_sub(1),
            Err(_) => 0,
        }
    };
    if index >= unsafe { mixerGetNumDevs() } {
        return Err(SoundFailure::NoMixer);
    }
    let mut mixer: HMIXER = core::ptr::null_mut();
    let answer = unsafe { mixerOpen(&mut mixer, index, 0, 0, CALLBACK_NULL) };
    if answer != 0 || mixer.is_null() {
        return Err(SoundFailure::NoMixer);
    }
    Ok(mixer)
}

/// The line of the component type that the script names.
fn the_line_of(mixer: HMIXER, component: u32) -> Result<MIXERLINEW, SoundFailure> {
    let mut line: MIXERLINEW = unsafe { core::mem::zeroed() };
    line.cbStruct = core::mem::size_of::<MIXERLINEW>() as u32;
    line.dwComponentType = component;
    let answer = unsafe {
        mixerGetLineInfoW(
            mixer,
            &mut line,
            MIXER_GETLINEINFOF_COMPONENTTYPE as u32,
        )
    };
    if answer != 0 {
        return Err(SoundFailure::NoComponent);
    }
    Ok(line)
}

/// The control of volume or of mute of a line.
fn the_control_of(mixer: HMIXER, line: &MIXERLINEW, mute: bool) -> Result<MIXERCONTROLW, SoundFailure> {
    let mut control: MIXERCONTROLW = unsafe { core::mem::zeroed() };
    let mut controls: MIXERLINECONTROLSW = unsafe { core::mem::zeroed() };
    controls.cbStruct = core::mem::size_of::<MIXERLINECONTROLSW>() as u32;
    controls.dwLineID = line.dwLineID;
    controls.Anonymous.dwControlType = if mute {
        MIXERCONTROL_CONTROLTYPE_MUTE
    } else {
        MIXERCONTROL_CONTROLTYPE_VOLUME
    };
    controls.cControls = 1;
    controls.cbmxctrl = core::mem::size_of::<MIXERCONTROLW>() as u32;
    controls.pamxctrl = &mut control;
    let answer = unsafe {
        mixerGetLineControlsW(
            mixer,
            &mut controls,
            MIXER_GETLINECONTROLSF_ONEBYTYPE as u32,
        )
    };
    if answer != 0 {
        return Err(SoundFailure::NoControl);
    }
    Ok(control)
}

/// The volume of a control, between zero and a hundred.
///
/// The mixer gives the volume between the bounds of the control, and
/// the tree answers it between zero and a hundred, the way of the v1.
fn the_volume_of(mixer: HMIXER, control: &MIXERCONTROLW) -> Result<f64, SoundFailure> {
    let mut value = MIXERCONTROLDETAILS_UNSIGNED { dwValue: 0 };
    let mut details: MIXERCONTROLDETAILS = unsafe { core::mem::zeroed() };
    details.cbStruct = core::mem::size_of::<MIXERCONTROLDETAILS>() as u32;
    details.dwControlID = control.dwControlID;
    details.cChannels = 1;
    details.cbDetails = core::mem::size_of::<MIXERCONTROLDETAILS_UNSIGNED>() as u32;
    details.paDetails = (&mut value as *mut MIXERCONTROLDETAILS_UNSIGNED).cast::<c_void>();
    let answer = unsafe {
        mixerGetControlDetailsW(mixer, &mut details, MIXER_GETCONTROLDETAILSF_VALUE as u32)
    };
    if answer != 0 {
        return Err(SoundFailure::NoSetting);
    }
    let bounds = unsafe { control.Bounds.Anonymous1 };
    let lowest = bounds.lMinimum;
    let highest = bounds.lMaximum;
    let width = (highest as i64) - (lowest as i64);
    if width <= 0 {
        return Ok(0.0);
    }
    let level = (value.dwValue as i64 - lowest as i64) as f64 / width as f64;
    Ok(level * 100.0)
}

/// Write the volume of a control, from a number between zero and a hundred.
fn the_volume_is_set(mixer: HMIXER, control: &MIXERCONTROLW, volume: f64) -> Result<(), SoundFailure> {
    let bounds = unsafe { control.Bounds.Anonymous1 };
    let lowest = bounds.lMinimum;
    let highest = bounds.lMaximum;
    let width = (highest as i64) - (lowest as i64);
    let level = volume.clamp(0.0, 100.0) / 100.0;
    let mut value = MIXERCONTROLDETAILS_UNSIGNED {
        dwValue: (lowest as i64 + (level * width as f64) as i64) as u32,
    };
    let mut details: MIXERCONTROLDETAILS = unsafe { core::mem::zeroed() };
    details.cbStruct = core::mem::size_of::<MIXERCONTROLDETAILS>() as u32;
    details.dwControlID = control.dwControlID;
    details.cChannels = 1;
    details.cbDetails = core::mem::size_of::<MIXERCONTROLDETAILS_UNSIGNED>() as u32;
    details.paDetails = (&mut value as *mut MIXERCONTROLDETAILS_UNSIGNED).cast::<c_void>();
    let answer = unsafe {
        mixerSetControlDetails(mixer, &details, MIXER_SETCONTROLDETAILSF_VALUE as u32)
    };
    if answer != 0 {
        return Err(SoundFailure::NoChange);
    }
    Ok(())
}

/// Whether a control of mute is muted.
fn the_mute_of(mixer: HMIXER, control: &MIXERCONTROLW) -> Result<bool, SoundFailure> {
    let mut value = MIXERCONTROLDETAILS_BOOLEAN { fValue: 0 };
    let mut details: MIXERCONTROLDETAILS = unsafe { core::mem::zeroed() };
    details.cbStruct = core::mem::size_of::<MIXERCONTROLDETAILS>() as u32;
    details.dwControlID = control.dwControlID;
    details.cChannels = 1;
    details.cbDetails = core::mem::size_of::<MIXERCONTROLDETAILS_BOOLEAN>() as u32;
    details.paDetails = (&mut value as *mut MIXERCONTROLDETAILS_BOOLEAN).cast::<c_void>();
    let answer = unsafe {
        mixerGetControlDetailsW(mixer, &mut details, MIXER_GETCONTROLDETAILSF_VALUE as u32)
    };
    if answer != 0 {
        return Err(SoundFailure::NoSetting);
    }
    Ok(value.fValue != 0)
}

/// Mute a control, or unmute it.
fn the_mute_is_set(mixer: HMIXER, control: &MIXERCONTROLW, muted: bool) -> Result<(), SoundFailure> {
    let mut value = MIXERCONTROLDETAILS_BOOLEAN {
        fValue: i32::from(muted),
    };
    let mut details: MIXERCONTROLDETAILS = unsafe { core::mem::zeroed() };
    details.cbStruct = core::mem::size_of::<MIXERCONTROLDETAILS>() as u32;
    details.dwControlID = control.dwControlID;
    details.cChannels = 1;
    details.cbDetails = core::mem::size_of::<MIXERCONTROLDETAILS_BOOLEAN>() as u32;
    details.paDetails = (&mut value as *mut MIXERCONTROLDETAILS_BOOLEAN).cast::<c_void>();
    let answer = unsafe {
        mixerSetControlDetails(mixer, &details, MIXER_SETCONTROLDETAILSF_VALUE as u32)
    };
    if answer != 0 {
        return Err(SoundFailure::NoChange);
    }
    Ok(())
}

/// The control of a component, the way of the two dialects.
///
/// The `named_master` is the difference of the two: the v1 treats the
/// `Master` and the `Speakers` as the master of the device, and the v2
/// does not -- only the empty text is the master. The master of the
/// mixer is the line of the speakers that plays.
fn the_control_of_a_component_with(
    component: &str,
    control: &str,
    device: &str,
    named_master: bool,
) -> Result<SoundControl, SoundFailure> {
    let (flow, is_the_mute) = the_component_and_the_control(component, control)?;
    let mixer = the_mixer_of(device)?;
    let answer = (|| {
        let is_master = if named_master {
            is_the_master(component)
        } else {
            component.trim().is_empty()
        };
        // The v2 does not know the named master: only the empty text.
        if !named_master && !component.trim().is_empty() && is_the_master(component) {
            return Err(SoundFailure::NoComponent);
        }
        let wanted = if flow == 0 && is_master {
            DST_SPEAKERS
        } else {
            let wanted = the_component_type_of(component);
            if wanted == SRC_UNDEFINED {
                return Err(SoundFailure::NoComponent);
            }
            wanted
        };
        let line = the_line_of(mixer, wanted)?;
        let control = the_control_of(mixer, &line, is_the_mute)?;
        if is_the_mute {
            Ok(SoundControl::Switch(the_mute_of(mixer, &control)?))
        } else {
            Ok(SoundControl::Volume(the_volume_of(mixer, &control)?))
        }
    })();
    unsafe { mixerClose(mixer) };
    answer
}

/// The volume of a component of a device, between zero and a hundred.
pub fn the_volume_of_the_component(device: &str, component: &str) -> Result<f64, String> {
    match the_control_of_a_component_with(component, "Volume", device, false) {
        Ok(SoundControl::Volume(volume)) => Ok(volume),
        Ok(SoundControl::Switch(_)) => Err("the control of the component is not the volume".into()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// Write the volume of a component of a device.
pub fn the_volume_of_the_component_is_set(
    device: &str,
    component: &str,
    volume: f64,
) -> Result<(), String> {
    match the_control_of_a_component_is_set(component, "Volume", volume, device) {
        Ok(()) => Ok(()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// Whether a component of a device is muted.
pub fn the_mute_of_the_component(device: &str, component: &str) -> Result<bool, String> {
    match the_control_of_a_component_with(component, "Mute", device, false) {
        Ok(SoundControl::Switch(muted)) => Ok(muted),
        Ok(SoundControl::Volume(_)) => Err("the control of the component is not the mute".into()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// Mute a component of a device, or unmute it.
pub fn the_mute_of_the_component_is_set(
    device: &str,
    component: &str,
    muted: bool,
) -> Result<(), String> {
    match the_control_of_a_component_is_set(component, "Mute", if muted { 1.0 } else { 0.0 }, device) {
        Ok(()) => Ok(()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// The control of a component, the way of the v1.
pub fn the_control_of_a_component(
    component: &str,
    control: &str,
    device: &str,
) -> Result<SoundControl, SoundFailure> {
    the_control_of_a_component_with(component, control, device, true)
}

/// Write the value of a control of a component of a device.
pub fn the_control_of_a_component_is_set(
    component: &str,
    control: &str,
    value: f64,
    device: &str,
) -> Result<(), SoundFailure> {
    let (_flow, is_the_mute) = the_component_and_the_control(component, control)?;
    let mixer = the_mixer_of(device)?;
    let answer = (|| {
        let wanted = the_component_type_of(component);
        let wanted = if wanted == SRC_UNDEFINED {
            DST_SPEAKERS
        } else {
            wanted
        };
        let line = the_line_of(mixer, wanted)?;
        let control = the_control_of(mixer, &line, is_the_mute)?;
        if is_the_mute {
            the_mute_is_set(mixer, &control, value > 0.0)
        } else {
            the_volume_is_set(mixer, &control, value)
        }
    })();
    unsafe { mixerClose(mixer) };
    answer
}

/// The name of the mixer of that number, if the machine has one.
fn the_name_of_the_mixer(index: u32) -> Option<String> {
    if index >= unsafe { mixerGetNumDevs() } {
        return None;
    }
    let mut caps: MIXERCAPSW = unsafe { core::mem::zeroed() };
    let answer = unsafe {
        mixerGetDevCapsW(index as usize, &mut caps, core::mem::size_of::<MIXERCAPSW>() as u32)
    };
    if answer != 0 {
        return None;
    }
    // The name is a wide text of the machine; the array is packed, so
    // it is copied before it is read.
    let name = caps.szPname;
    let end = name.iter().position(|&c| c == 0).unwrap_or(name.len());
    Some(String::from_utf16_lossy(&name[..end]))
}

/// The names of the devices of the sound: the one that plays and the
/// one that records.
///
/// The mixer of the era of XP is one device that plays and records at
/// once, so the two names are the name of the first mixer.
pub fn the_names_of_the_devices() -> Result<(String, String), String> {
    let name = the_name_of_the_mixer(0).unwrap_or_default();
    Ok((name.clone(), name))
}

/// The name of the device that the script names, if the machine has one.
pub fn the_name_of_the_device_of_the_name(name: &str) -> Result<Option<String>, String> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(the_name_of_the_mixer(0));
    }
    if let Ok(number) = name.parse::<u32>() {
        return Ok(the_name_of_the_mixer(number.saturating_sub(1)));
    }
    let count = unsafe { mixerGetNumDevs() };
    for index in 0..count {
        if let Some(candidate) = the_name_of_the_mixer(index) {
            if candidate.to_lowercase().starts_with(&name.to_lowercase()) {
                return Ok(Some(candidate));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mixer_of_the_machine_opens() {
        // The machine of the tree has a mixer: the first opens, and a
        // number that the machine does not have refuses.
        if unsafe { mixerGetNumDevs() } == 0 {
            return;
        }
        let mixer = the_mixer_of("").expect("the first mixer opens");
        unsafe { mixerClose(mixer) };
        assert!(the_mixer_of("9999").is_err());
    }

    #[test]
    fn the_master_of_the_mixer_has_a_volume() {
        if unsafe { mixerGetNumDevs() } == 0 {
            return;
        }
        let answer = the_control_of_a_component("", "Volume", "");
        // The machine of the lab may have no volume control; the
        // answer is a volume or a refusal of the machine, never a
        // panic.
        match answer {
            Ok(SoundControl::Volume(volume)) => assert!((0.0..=100.0).contains(&volume)),
            Ok(SoundControl::Switch(_)) => panic!("the master answered a switch"),
            Err(_) => {}
        }
    }
}
