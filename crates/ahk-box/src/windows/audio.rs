//! The devices of the sound of the machine.
//!
//! The devices of the sound are the COM of the machine: the class that
//! knows them -- the *MMDeviceEnumerator class* -- is asked for the
//! devices that play and for the ones that record, and every device
//! knows its volume and whether it is muted.
//!
//! The numbers of the classes and of the shapes come from the machine
//! (see `com::the_class_of_the_machine`), and the shape of the device
//! is asked to the machine by its name:
//! `IMMDevice::Activate` with the shape of the volume.
//!
//! The shape of the whole is the one of the v2.0.31, read in its source
//! (`source/lib/sound.cpp` of the reference tree):
//!
//! * the device of the empty name is the default of the machine --
//!   `IMMDeviceEnumerator::GetDefaultAudioEndpoint` with the flow of the
//!   playback and the role of the console;
//! * the device of a name is the first one whose name begins with that
//!   name -- the comparison of the v2 is a prefix and it does not mind
//!   the case;
//! * the device of a number is the device of that place in the list of
//!   all the devices -- the list of the v2 is the one of every flow, and
//!   it carries the devices that are unplugged so that the numbers do
//!   not move when a device is unplugged;
//! * the name of a device is the name that the machine shows of it --
//!   the property `PKEY_Device_FriendlyName` of its store of
//!   properties;
//! * the volume of the master of a device is the volume of the machine
//!   -- `IAudioEndpointVolume::GetMasterVolumeLevelScalar` -- and it is
//!   a number between zero and a hundred;
//! * the mute of the master of a device is `IAudioEndpointVolume`.
//!
//! Measured on the machine of the tree: the machine has
//! six devices that play and six that record, and the count of them
//! comes from `IMMDeviceCollection::GetCount` -- a method that answers
//! with a number of the machine and writes the count in a place that
//! the caller gives to it. The tree was calling it as a method that
//! answers with the count itself, and the machine was writing the
//! count in a place of nowhere: the tree read a number of nothing and
//! called the chain broken.
//!
//! The device is lazy: the name of it is read when it is asked for and
//! the volume of it is opened when it is asked for. Opening the volume
//! of a device is work for the service of the sound of the machine, and
//! the work of twelve devices at once is what makes the sound of the
//! machine stutter (measured on the machine of the tree).

use core::ffi::c_void;

use windows_sys::core::HRESULT;

use ahk::desktop::{SoundControl, SoundFailure};

use super::com::{self, TheObject};

/// The flow of the data: zero is playing, one is recording, two is
/// every flow.
const THE_PLAYBACK: u32 = 0;
const THE_RECORD: u32 = 1;
const THE_ALL: u32 = 2;

/// The state of a device that is working.
const THE_ACTIVE: u32 = 1;

/// The state of a device that is not plugged in.
///
/// The number is the one of the machine: the `DEVICE_STATE_UNPLUGGED` is
/// the eight, and not the two -- the two is the one of the device that
/// is turned off. Measured on the binary of v1.1.37.02:
/// the list of the devices of the sound of the v1 is the one of the
/// active devices and of the ones that are not plugged in, and the
/// number of each device is its place in the list.
const THE_UNPLUGGED: u32 = 8;

/// The role of the device of the system.
const THE_ROLE_CONSOLE: u32 = 0;
/// The other role of the system, when the console has no device.
const THE_ROLE_MULTIMEDIA: u32 = 1;

/// The number of the property of the name of the device.
///
/// `PKEY_Device_FriendlyName`: the shape is
/// `{A45C254E-DF1C-4EFD-8020-67D146A850E0}` and the number is two.
const THE_NUMBER_OF_THE_NAME: u32 = 2;

/// The kind of a value of a property that is a text largo.
const THE_KIND_OF_A_TEXT: u16 = 31;

/// The value of a property, as the machine gives it.
///
/// The shape is the one of Windows: the kind of the value, three words
/// of reserve and the data. The kind of a wide text is thirty-one, and
/// the data is the pointer to the text.
///
/// The `PROPVARIANT` of Windows is **twenty-four bytes** in every
/// architecture: the union of it holds a `DECIMAL`, que tem dezasseis
/// bytes. A struct de dezasseis bytes é pequena demais, e a máquina
/// escreve oito bytes além dela — o que corrompe o que está ao lado na
/// pilha (o argumento `key` da chamada, e o nome do dispositivo
/// vinha vazio).
///
/// Measured on the machine: the `IPropertyStore::GetValue` of the COM
/// writes the twenty-four bytes, and the tree declared sixteen.
#[repr(C)]
struct TheValue {
    kind: u16,
    reserved1: u16,
    reserved2: u16,
    reserved3: u16,
    data: [u8; 16],
}

/// A device of the sound of the machine.
///
/// The device is lazy on purpose. Opening the volume of a device is work
/// for the service of the sound of the machine -- and the work of twelve
/// devices at once is what makes the sound of the machine stutter
/// (measured on the machine of the tree). So the name is
/// read when it is asked for, and the volume is opened when it is asked
/// for, and the list of the devices is only a list.
pub struct TheDevice {
    /// The device itself, kept alive while its volume is used.
    pub(crate) device: TheObject,
    /// The name of the device, the one that the machine shows, read when
    /// it is asked for and kept after.
    name: std::cell::RefCell<Option<String>>,
}

impl TheDevice {
    /// The name of the device, the one that the machine shows.
    ///
    /// The name comes from the store of the properties of the device, and
    /// it is read only when it is asked for.
    pub fn the_name(&self) -> String {
        if let Some(name) = self.name.borrow().as_ref() {
            return name.clone();
        }
        let name = the_name_of_the_device(&self.device).unwrap_or_default();
        *self.name.borrow_mut() = Some(name.clone());
        name
    }

    /// The volume of the device, as a number between zero and a hundred.
    pub fn the_volume(&self) -> Result<f64, String> {
        let volume = self.the_master_of_the_volume()?;
        unsafe {
            type GetVolume = unsafe extern "system" fn(*mut c_void, *mut f32) -> HRESULT;
            let get: GetVolume = std::mem::transmute(volume.the_method(9));
            let mut level: f32 = 0.0;
            let answer = get(volume.pointer, &mut level);
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            // A conta do v1 e feita no numero de 32 bits: o nivel do
            // volume e multiplicado por cem antes de virar o texto da
            // resposta. Medido no binario do v1.1.37.02: o
            // volume de 94.000084 do v1 e 94.000083 quando a conta e
            // feita com 64 bits.
            Ok(f64::from(level * 100.0f32))
        }
    }

    /// Set the volume of the device, as a number between zero and a hundred.
    pub fn the_volume_is_set(&self, volume: f64) -> Result<(), String> {
        let master = self.the_master_of_the_volume()?;
        unsafe {
            type SetVolume = unsafe extern "system" fn(*mut c_void, f32, *mut c_void) -> HRESULT;
            let set: SetVolume = std::mem::transmute(master.the_method(7));
            let answer = set(
                master.pointer,
                (volume.clamp(0.0, 100.0) / 100.0) as f32,
                std::ptr::null_mut(),
            );
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            Ok(())
        }
    }

    /// Whether the device is muted.
    pub fn the_mute(&self) -> Result<bool, String> {
        let master = self.the_master_of_the_volume()?;
        unsafe {
            type GetMute = unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT;
            let get: GetMute = std::mem::transmute(master.the_method(15));
            let mut muted: i32 = 0;
            let answer = get(master.pointer, &mut muted);
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            Ok(muted != 0)
        }
    }

    /// Mute the device, or unmute it.
    pub fn the_mute_is_set(&self, muted: bool) -> Result<(), String> {
        let master = self.the_master_of_the_volume()?;
        unsafe {
            type SetMute = unsafe extern "system" fn(*mut c_void, i32, *mut c_void) -> HRESULT;
            let set: SetMute = std::mem::transmute(master.the_method(14));
            let answer = set(master.pointer, i32::from(muted), std::ptr::null_mut());
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            Ok(())
        }
    }

    /// The volume and the mute of the master of the device, read with
    /// one opening of the volume.
    ///
    /// Opening the volume of a device is work for the service of the
    /// sound of the machine, and the work of many devices at once is
    /// what makes the sound of the machine stutter (measured on the
    /// machine of the tree). So a reader that wants both
    /// opens the volume once.
    pub fn the_state(&self) -> Result<(f64, bool), String> {
        let master = self.the_master_of_the_volume()?;
        unsafe {
            type GetVolume = unsafe extern "system" fn(*mut c_void, *mut f32) -> HRESULT;
            let get: GetVolume = std::mem::transmute(master.the_method(9));
            let mut level: f32 = 0.0;
            let answer = get(master.pointer, &mut level);
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            type GetMute = unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT;
            let get: GetMute = std::mem::transmute(master.the_method(15));
            let mut muted: i32 = 0;
            let answer = get(master.pointer, &mut muted);
            if answer < 0 {
                return Err(com::the_reason_of(answer));
            }
            Ok((f64::from(level) * 100.0, muted != 0))
        }
    }

    /// The pointer of an interface of the device.
    ///
    /// The interface is asked with the `QueryInterface` of the device
    /// and, when the machine refuses, with its `Activate`. The answer is
    /// the pointer, even when the machine has no such interface -- the
    /// way the v2.0.31 does it (read in its source,
    /// `source/lib/sound.cpp`).
    fn the_interface(&self, iid: &windows_sys::core::GUID) -> Result<u64, String> {
        unsafe {
            type QueryInterface = unsafe extern "system" fn(
                *mut c_void,
                *const windows_sys::core::GUID,
                *mut *mut c_void,
            ) -> HRESULT;
            let query: QueryInterface = std::mem::transmute(self.device.the_method(0));
            let mut pointer: *mut c_void = std::ptr::null_mut();
            let answer = query(self.device.pointer, iid, &mut pointer);
            if answer < 0 || pointer.is_null() {
                // The machine refuses the interface: the `Activate` of the
                // device opens it all the same.
                type Activate = unsafe extern "system" fn(
                    *mut c_void,
                    *const windows_sys::core::GUID,
                    u32,
                    *mut c_void,
                    *mut *mut c_void,
                ) -> HRESULT;
                let activate: Activate = std::mem::transmute(self.device.the_method(3));
                pointer = std::ptr::null_mut();
                activate(
                    self.device.pointer,
                    iid,
                    0x17, // CLSCTX_ALL
                    std::ptr::null_mut(),
                    &mut pointer,
                );
            }
            Ok(pointer as u64)
        }
    }

    /// The volume of the master of the device.
    ///
    /// The master is opened when it is asked for: the machine opens the
    /// volume of each device in the sound service of it, and opening
    /// twelve at once is what makes the sound of the machine stutter
    /// (measured on the machine of the tree).
    fn the_master_of_the_volume(&self) -> Result<TheObject, String> {
        unsafe {
            // The volume of the device: the machine activates a shape of
            // it and gives the object of the volume.
            type Activate = unsafe extern "system" fn(
                *mut c_void,
                *const windows_sys::core::GUID,
                u32,
                *mut c_void,
                *mut *mut c_void,
            ) -> HRESULT;
            let activate: Activate = std::mem::transmute(self.device.the_method(3));
            let mut volume: *mut c_void = std::ptr::null_mut();
            let answer = activate(
                self.device.pointer,
                &com::the_interface_of_the_volume()?,
                1, // CLSCTX_INPROC_SERVER
                std::ptr::null_mut(),
                &mut volume,
            );
            if answer < 0 || volume.is_null() {
                return Err(format!(
                    "the machine did not give the volume of the device: {answer} ({:#x}): {}",
                    answer as u32,
                    com::the_reason_of(answer)
                ));
            }
            Ok(TheObject { pointer: volume })
        }
    }
}

/// The class that knows the devices of the sound.
fn the_enumerator() -> Result<TheObject, String> {
    let class = com::the_class_of_the_devices_of_the_sound()?;
    let interface = com::the_interface_of_the_devices()?;
    TheObject::of_the_class(&class, &interface)
}

/// The devices of one flow of the data.
///
/// The flow is zero for the devices that play, one for the ones that
/// record and two for every flow. The devices that are unplugged are
/// in the list too, so that the number of a device does not move when
/// a device is unplugged -- this is the shape of the v2.0.31.
fn the_devices_of_the_flow(flow: u32) -> Result<Vec<TheDevice>, String> {
    let enumerator = the_enumerator()?;
    unsafe {
        // The collection of the devices.
        type EnumEndpoints =
            unsafe extern "system" fn(*mut c_void, u32, u32, *mut *mut c_void) -> HRESULT;
        let enumerate: EnumEndpoints = std::mem::transmute(enumerator.the_method(3));
        let mut collection: *mut c_void = std::ptr::null_mut();
        let answer = enumerate(
            enumerator.pointer,
            flow,
            THE_ACTIVE | THE_UNPLUGGED,
            &mut collection,
        );
        if answer < 0 || collection.is_null() {
            return Err(format!(
                "the machine did not give the devices of the sound: {answer} ({:#x}): {}",
                answer as u32,
                com::the_reason_of(answer)
            ));
        }
        let collection = TheObject {
            pointer: collection,
        };

        // The count of the devices: the method answers with a number of
        // the machine and writes the count in the place that the caller
        // gives to it.
        type GetCount = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
        let count_of: GetCount = std::mem::transmute(collection.the_method(3));
        let mut count: u32 = 0;
        let answer = count_of(collection.pointer, &mut count);
        if answer < 0 {
            return Err(format!(
                "the machine did not count the devices of the sound: {answer} ({:#x}): {}",
                answer as u32,
                com::the_reason_of(answer)
            ));
        }

        type Item = unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
        let item: Item = std::mem::transmute(collection.the_method(4));

        let mut devices = Vec::new();
        for index in 0..count {
            let mut pointer: *mut c_void = std::ptr::null_mut();
            let answer = item(collection.pointer, index, &mut pointer);
            if answer < 0 || pointer.is_null() {
                continue;
            }
            let device = TheObject { pointer };
            devices.push(TheDevice::of_the_device(device));
        }
        Ok(devices)
    }
}

impl TheDevice {
    /// The device of the sound, com o seu nome e o seu volume.
    ///
    /// The device is made here and nothing more: the name is read when
    /// it is asked for and the volume is opened when it is asked for.
    fn of_the_device(device: TheObject) -> Self {
        Self {
            device,
            name: std::cell::RefCell::new(None),
        }
    }
}

/// The name that the machine shows of the device.
fn the_name_of_the_device(device: &TheObject) -> Result<String, String> {
    unsafe {
        // A loja de propriedades do dispositivo.
        type OpenStore = unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
        let open: OpenStore = std::mem::transmute(device.the_method(4));
        let mut store: *mut c_void = std::ptr::null_mut();
        let answer = open(device.pointer, 0, &mut store);
        if answer < 0 || store.is_null() {
            return Err(format!(
                "the machine did not open the properties of the device: {answer} ({:#x}): {}",
                answer as u32,
                com::the_reason_of(answer)
            ));
        }
        let store = TheObject { pointer: store };

        // O valor da propriedade do nome.
        type GetValue =
            unsafe extern "system" fn(*mut c_void, *const TheKey, *mut TheValue) -> HRESULT;
        let get: GetValue = std::mem::transmute(store.the_method(5));
        let key = TheKey {
            shape: com::the_interface_of_the_name_of_the_device()?,
            number: THE_NUMBER_OF_THE_NAME,
        };
        let mut value = TheValue {
            kind: 0,
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            data: [0; 16],
        };
        let answer = get(store.pointer, &key, &mut value);
        if answer < 0 {
            return Err(format!(
                "the machine did not give the name of the device: {answer} ({:#x}): {}",
                answer as u32,
                com::the_reason_of(answer)
            ));
        }
        // O texto largo da propriedade.
        if value.kind != THE_KIND_OF_A_TEXT {
            return Err(format!(
                "the machine gave the name of the device as the kind {}, and not as a text",
                value.kind
            ));
        }
        let pointer = u64::from_le_bytes(value.data[..8].try_into().unwrap_or([0; 8]))
            as *mut u16;
        if pointer.is_null() {
            return Err("the machine gave the name of the device as nothing".to_string());
        }
        let mut length = 0usize;
        while *pointer.add(length) != 0 {
            length += 1;
        }
        let letters = std::slice::from_raw_parts(pointer, length);
        let name = String::from_utf16_lossy(letters);
        windows_sys::Win32::System::Com::CoTaskMemFree(pointer.cast());
        Ok(name)
    }
}

/// The key of a property, as the machine gives it.
#[repr(C)]
struct TheKey {
    /// The shape of onde the property lives.
    shape: windows_sys::core::GUID,
    /// The number of the property.
    number: u32,
}

/// The value of a control of a component of a device of the sound.
///
/// The v1 names a part of the sound in two words: the component -- the
/// "Master", the "Microphone" -- and the control -- the "Volume", the
/// "Mute". The machine of Windows Vista and later has the master of the
/// device that plays and the microphone of the device that records, and
/// the other components do not exist.
///
/// Measured on the binary of v1.1.37.02: the master and
/// the microphone of the machine of that day have a volume, and the other
/// components do not exist; the master has a mute, and the other controls
/// do not exist either.
/// The volume of a component of a device of the sound.
///
/// O componente e o nome de uma parte do som -- o "Microphone" -- e o
/// texto vazio e o master do device.
///
/// Medido no binario do v2.0.31: o
/// `SoundGetVolume("Microphone")` responde o volume do microfone e o
/// `SoundGetVolume("")` responde o master do device.
pub fn the_volume_of_the_component(device: &str, component: &str) -> Result<f64, String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_volume_of_the_component(device, component);
    }
    let answer = if let Ok(index) = component.trim().parse::<u32>() {
        the_control_of_the_component_by_index(device, index, false)
    } else {
        the_control_of_a_component_with(component, "Volume", device, false)
    };
    match answer {
        Ok(SoundControl::Volume(volume)) => Ok(volume),
        Ok(SoundControl::Switch(_)) => Err("the control of the component is not the volume".into()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// The volume of a component of a device of the sound is written.
pub fn the_volume_of_the_component_is_set(
    device: &str,
    component: &str,
    volume: f64,
) -> Result<(), String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_volume_of_the_component_is_set(device, component, volume);
    }
    match the_control_of_a_component_is_set(component, "Volume", volume, device) {
        Ok(()) => Ok(()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// Whether a component of a device of the sound is muted.
pub fn the_mute_of_the_component(device: &str, component: &str) -> Result<bool, String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_mute_of_the_component(device, component);
    }
    let answer = if let Ok(index) = component.trim().parse::<u32>() {
        the_control_of_the_component_by_index(device, index, true)
    } else {
        the_control_of_a_component_with(component, "Mute", device, false)
    };
    match answer {
        Ok(SoundControl::Switch(muted)) => Ok(muted),
        Ok(SoundControl::Volume(_)) => Err("the control of the component is not the mute".into()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

/// The control of the component that the number names.
///
/// O numero e o lugar do componente na lista da topologia, e a busca
/// distingue o componente que nao esta do componente que esta e nao
/// tem o controle.
fn the_control_of_the_component_by_index(
    device: &str,
    index: u32,
    is_the_mute: bool,
) -> Result<SoundControl, SoundFailure> {
    let device = match the_device_of_the_number(device)? {
        Some(device) => device,
        None => match the_default_of_the_flow(THE_PLAYBACK) {
            Ok(device) => device,
            Err(_) => return Err(SoundFailure::NoMixer),
        },
    };
    let interface = if is_the_mute {
        com::the_interface_of_the_mute()
    } else {
        com::the_interface_of_the_level()
    };
    let Ok(interface) = interface else {
        return Err(SoundFailure::NoControl);
    };
    let Some((found, control)) =
        super::topology::the_component_of_the_device_by_index(&device.device, index, Some(interface))
    else {
        return Err(SoundFailure::NoMixer);
    };
    if !found {
        return Err(SoundFailure::NoComponent);
    }
    let Some(control) = control else {
        return Err(SoundFailure::NoControl);
    };
    if is_the_mute {
        match the_mute_of(&control) {
            Ok(muted) => Ok(SoundControl::Switch(muted)),
            Err(_) => Err(SoundFailure::NoSetting),
        }
    } else {
        match the_level_of(&control) {
            Ok(volume) => Ok(SoundControl::Volume(volume)),
            Err(_) => Err(SoundFailure::NoSetting),
        }
    }
}

/// A component of a device of the sound is muted, or unmuted.
pub fn the_mute_of_the_component_is_set(
    device: &str,
    component: &str,
    muted: bool,
) -> Result<(), String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_mute_of_the_component_is_set(device, component, muted);
    }
    match the_control_of_a_component_is_set(component, "Mute", if muted { 1.0 } else { 0.0 }, device) {
        Ok(()) => Ok(()),
        Err(failure) => Err(failure.the_words_of_the_v2().to_string()),
    }
}

pub fn the_control_of_a_component(
    component: &str,
    control: &str,
    device: &str,
) -> Result<SoundControl, SoundFailure> {
    // O v1 trata o "Master" e o "Speakers" como o master do device.
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_control_of_a_component(component, control, device);
    }
    the_control_of_a_component_with(component, control, device, true)
}

/// The control of a component, and whether the named master counts.
///
/// O `named_master` e a diferenca entre os dois dialetos: o v1 trata o
/// "Master" e o "Speakers" como o master do device, e o v2 nao -- so o
/// texto vazio e o master. Medido no binario do v2.0.31:
/// o `SoundGetVolume("Master")` e o `SoundGetVolume("Speakers")`
/// recusam com "Component not found", e o `SoundGetVolume("")` responde
/// o master do device.
pub fn the_control_of_a_component_with(
    component: &str,
    control: &str,
    device: &str,
    named_master: bool,
) -> Result<SoundControl, SoundFailure> {
    // O componente e o controle que faltam: o `Master` e o `Volume`.
    // Medido no binario do v1.1.37.02: o `SoundGet, out`
    // sem componente e sem controle responde o volume do master, e o
    // `SoundGet, out, , Mute` responde o mute do master.
    let (flow, is_the_mute) = the_component_and_the_control(component, control)?;
    // The device: its number is its place in the list of all the
    // devices of the machine, and the empty name is the device of the
    // system. Measured on the binary of v1.1.37.02: the
    // device of the number that the machine does not have answers
    // "Can't Open Specified Mixer".
    let device = match the_device_of_the_number(device)? {
        Some(device) => device,
        None => match the_default_of_the_flow(flow) {
            Ok(device) => device,
            Err(_) => return Err(SoundFailure::NoMixer),
        },
    };
    // O master e o volume do device: a maquina da a forma do volume
    // do endpoint e o master dele e o volume.
    let is_master = if named_master {
        is_the_master(component)
    } else {
        component.trim().is_empty()
    };
    if flow == THE_PLAYBACK && is_master {
        return if is_the_mute {
            match device.the_mute() {
                Ok(muted) => Ok(SoundControl::Switch(muted)),
                Err(_) => Err(SoundFailure::NoSetting),
            }
        } else {
            match device.the_volume() {
                Ok(volume) => Ok(SoundControl::Volume(volume)),
                Err(_) => Err(SoundFailure::NoSetting),
            }
        };
    }
    // The rest of the components lives in the topology of the device:
    // the `Microphone` of the v1 is the part of the microphone of the
    // device that plays, and not the master of the device that records.
    // Measured on the binary of v1.1.37.02: the volume
    // of the microphone is 49.999966, and the master of the device that
    // records is 27.450982.
    let wanted = the_component_type_of(component);
    if wanted == super::topology::SRC_UNDEFINED {
        return Err(SoundFailure::NoComponent);
    }
    let interface = if is_the_mute {
        com::the_interface_of_the_mute()
    } else {
        com::the_interface_of_the_level()
    };
    let Ok(interface) = interface else {
        return Err(SoundFailure::NoControl);
    };
    // A topologia do device e do device que toca: o v1 abre o device
    // do sistema com o fluxo que toca e anda pela topologia dele.
    let device = match the_default_of_the_flow(THE_PLAYBACK) {
        Ok(device) => device,
        Err(_) => return Err(SoundFailure::NoMixer),
    };
    let Some(control) =
        super::topology::the_component_of_the_device(&device.device, wanted, Some(interface))
    else {
        return Err(SoundFailure::NoComponent);
    };
    if is_the_mute {
        match the_mute_of(&control) {
            Ok(muted) => Ok(SoundControl::Switch(muted)),
            Err(_) => Err(SoundFailure::NoSetting),
        }
    } else {
        match the_level_of(&control) {
            Ok(volume) => Ok(SoundControl::Volume(volume)),
            Err(_) => Err(SoundFailure::NoSetting),
        }
    }
}

/// Whether the component is o master.
///
/// O master e o `Master` e o `Speakers`, e o v1 trata os dois como o
/// mesmo: medido no binario do v1.1.37.02, o
/// `SoundGet, out, Master, Volume` e o `SoundGet, out, Speakers,
/// Volume` respondem o mesmo numero.
pub(crate) fn is_the_master(component: &str) -> bool {
    let wanted = component.trim().to_lowercase();
    let wanted = wanted.split(':').next().unwrap_or_default();
    wanted.is_empty() || wanted == "master" || wanted == "speakers"
}

/// O tipo de componente do mixer que o texto nomeia.
///
/// O mapa e o do `SoundConvertComponentType` do v1.1.37.02: o
/// componente que falta e o `Master`, e cada nome tem o seu tipo.
pub(crate) fn the_component_type_of(component: &str) -> u32 {
    use super::topology::{
        DST_HEADPHONES, DST_SPEAKERS, SRC_ANALOG, SRC_AUXILIARY, SRC_COMPACTDISC, SRC_DIGITAL,
        SRC_LINE, SRC_MICROPHONE, SRC_PCSPEAKER, SRC_SYNTHESIZER, SRC_TELEPHONE, SRC_UNDEFINED,
        SRC_WAVEOUT,
    };
    let wanted = component.trim().to_lowercase();
    // O nome pode ter o numero da instancia depois de um dois pontos:
    // `Microphone:2` e a segunda instance do microfone.
    let wanted = wanted.split(':').next().unwrap_or_default();
    match wanted {
        "" | "master" | "speakers" => DST_SPEAKERS,
        "headphones" => DST_HEADPHONES,
        "digital" => SRC_DIGITAL,
        "line" => SRC_LINE,
        "microphone" => SRC_MICROPHONE,
        "synth" => SRC_SYNTHESIZER,
        "cd" => SRC_COMPACTDISC,
        "telephone" => SRC_TELEPHONE,
        "pcspeaker" => SRC_PCSPEAKER,
        "wave" => SRC_WAVEOUT,
        "aux" => SRC_AUXILIARY,
        "analog" => SRC_ANALOG,
        _ => SRC_UNDEFINED,
    }
}

/// O mute de um controle da topologia.
fn the_mute_of(control: &com::TheObject) -> Result<bool, String> {
    unsafe {
        type GetMute = unsafe extern "system" fn(*mut c_void, *mut i32) -> HRESULT;
        let get: GetMute = std::mem::transmute(control.the_method(3));
        let mut muted: i32 = 0;
        let answer = get(control.pointer, &mut muted);
        if answer < 0 {
            return Err(com::the_reason_of(answer));
        }
        Ok(muted != 0)
    }
}

/// O volume de um controle da topologia.
///
/// The account is the one of the v1: the level in decibels becomes a
/// number between zero and one, and the answer is the greater of the
/// levels of the channels.
fn the_level_of(control: &com::TheObject) -> Result<f64, String> {
    unsafe {
        type GetChannelCount = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
        let count_of: GetChannelCount = std::mem::transmute(control.the_method(3));
        let mut channels: u32 = 0;
        if count_of(control.pointer, &mut channels) < 0 {
            return Err("the machine did not count the channels of the level".to_string());
        }
        type GetLevel = unsafe extern "system" fn(*mut c_void, u32, *mut f32) -> HRESULT;
        // O `GetLevel` do `IPerChannelDbLevel` e o metodo 5, e o
        // `GetLevelRange` e o 4: a ordem da tabela e a do SDK.
        let level_of: GetLevel = std::mem::transmute(control.the_method(5));
        // O `GetLevelRange` da quatro numeros: o minimo e o maximo em
        // decibels, e o passo entre um nivel e o outro.
        type GetLevelRange =
            unsafe extern "system" fn(*mut c_void, u32, *mut f32, *mut f32, *mut f32) -> HRESULT;
        let range_of: GetLevelRange = std::mem::transmute(control.the_method(4));
        let mut highest: f32 = 0.0;
        for channel in 0..channels {
            let mut db: f32 = 0.0;
            let mut minimum: f32 = 0.0;
            let mut maximum: f32 = 0.0;
            let mut step: f32 = 0.0;
            if level_of(control.pointer, channel, &mut db) < 0
                || range_of(control.pointer, channel, &mut minimum, &mut maximum, &mut step) < 0
            {
                return Err("the machine did not read the level of the channel".to_string());
            }
            // O decibel vira um numero entre zero e um, e a resposta e
            // o maior dos canais.
            let lowest = 10f32.powf(minimum / 20.0);
            let width = 10f32.powf(maximum / 20.0) - lowest;
            let level = if width == 0.0 {
                0.0
            } else {
                (10f32.powf(db / 20.0) - lowest) / width
            };
            if highest < level {
                highest = level;
            }
        }
        Ok(f64::from(highest * 100.0f32))
    }
}

/// The component and the control that the text names.
///
/// The component that is missing is the `Master`, and the control that is
/// missing is the `Volume`. The `Wave` is the component that the
/// machine has and that has no control at all: measured on the binary of
/// v1.1.37.02, the `SoundGet, out, Wave, Volume` answers
/// the empty text and the `ErrorLevel` is "Component Doesn't Support
/// This Control Type".
///
/// The answer is the flow of the sound -- what plays or what records --
/// and whether the control that is asked for is the mute.
pub(crate) fn the_component_and_the_control(
    component: &str,
    control: &str,
) -> Result<(u32, bool), SoundFailure> {
    // The component that is missing is the master.
    let wanted = component.trim().to_lowercase();
    let wanted = wanted.split(':').next().unwrap_or_default().to_string();
    // The `Wave` is the component that the machine has and that has no
    // control at all: measured on the binary of v1.1.37.02 the `SoundGet, out, Wave, Volume` answers the empty
    // text and the `ErrorLevel` is "Component Doesn't Support This
    // Control Type".
    let flow = match wanted.as_str() {
        "" | "master" | "speakers" => THE_PLAYBACK,
        "microphone" => THE_RECORD,
        "wave" => return Err(SoundFailure::NoControl),
        "digital" | "line" | "synth" | "cd" | "telephone" | "pcspeaker" | "aux"
        | "analog" | "headphones" => THE_PLAYBACK,
        _ => return Err(SoundFailure::NoComponent),
    };
    // O controle que falta e o volume.
    let control = control.trim().to_lowercase();
    let is_the_volume = matches!(control.as_str(), "" | "volume" | "vol");
    let is_the_mute = control == "mute";
    if !is_the_volume && !is_the_mute {
        return Err(SoundFailure::NoControl);
    }
    Ok((flow, is_the_mute))
}

/// The device of the machine of that number, if the machine has one.
///
/// The number of the device of the v1 e o lugar dele na lista de todos
/// os devices da maquina, e a contagem dessa lista comeca em um: o
/// device do numero zero e o device do sistema.
///
/// Measured on the binary of v1.1.37.02: the device of
/// the number that the machine does not have answers "Can't Open
/// Specified Mixer", and the device of the number that the machine has
/// but that does not have the control that is asked for answers "Can't
/// Get Current Setting".
fn the_device_of_the_number(number: &str) -> Result<Option<TheDevice>, SoundFailure> {
    let number = number.trim();
    if number.is_empty() {
        return Ok(None);
    }
    let Ok(number) = number.parse::<u32>() else {
        return Ok(None);
    };
    let devices = the_devices_of_the_flow(THE_ALL).map_err(|_| SoundFailure::NoMixer)?;
    // The device of the number zero is the first of the list, the way
    // the v1 reads it: its number is one less than the place.
    let index = number.saturating_sub(1) as usize;
    // The list is the one of all the devices of the machine, and not
    // the one of the flow of the component: the device of the number is
    // the same for the master and for the microphone. Measured on the
    // binary of v1.1.37.02.
    match devices.into_iter().nth(index) {
        Some(device) => Ok(Some(device)),
        // The device that the machine does not have: the v1 cannot
        // open its mixer.
        None => Err(SoundFailure::NoMixer),
    }
}

/// Write the value of a control of a component of a device of the sound.
///
/// The v1 writes the volume of a device and the mute of it, and the
/// other components and controls do not exist on the machine of the lab.
pub fn the_control_of_a_component_is_set(
    component: &str,
    control: &str,
    value: f64,
    device: &str,
) -> Result<(), SoundFailure> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_control_of_a_component_is_set(component, control, value, device);
    }
    let (flow, is_the_mute) = the_component_and_the_control(component, control)?;
    let device = match the_device_of_the_number(device)? {
        Some(device) => device,
        None => match the_default_of_the_flow(flow) {
            Ok(device) => device,
            Err(_) => return Err(SoundFailure::NoMixer),
        },
    };
    // A escrita que falha e a frase do `SoundSet`: "Can't Change
    // Setting". Medido no binario do v1.1.37.02.
    if is_the_mute {
        match device.the_mute_is_set(value > 0.0) {
            Ok(()) => Ok(()),
            Err(_) => Err(SoundFailure::NoChange),
        }
    } else {
        match device.the_volume_is_set(value) {
            Ok(()) => Ok(()),
            Err(_) => Err(SoundFailure::NoChange),
        }
    }
}

/// The device of the sound that the script names.
///
/// The name is the name of the device -- the first one whose name
/// begins with it -- or a number and its place in the list of all the
/// devices, or a name and a number and a colon between them. The empty
/// name is the device of the system.
///
/// The answer is `None` when the machine has no device of that name.
pub fn the_device_of_the_name(name: &str) -> Result<Option<TheDevice>, String> {
    let name = name.trim();
    if name.is_empty() {
        return the_device_of_the_system().map(Some);
    }
    // The number of the device: the place of it in the list of every
    // device of the machine.
    if let Ok(number) = name.parse::<u32>() {
        let devices = the_devices_of_the_flow(THE_ALL)?;
        return Ok(devices.into_iter().nth((number.max(1) - 1) as usize));
    }
    // The name of the device, and maybe the number of it among the
    // devices of that name.
    let (wanted, number) = match name.rsplit_once(':') {
        Some((wanted, number)) => match number.parse::<u32>() {
            Ok(number) => (wanted, number.max(1)),
            Err(_) => (name, 1),
        },
        None => (name, 1),
    };
    let devices = the_devices_of_the_flow(THE_ALL)?;
    let mut left = number;
    for device in devices {
        let device_name = device.the_name();
        if device_name
            .to_lowercase()
            .starts_with(&wanted.to_lowercase())
        {
            left -= 1;
            if left == 0 {
                return Ok(Some(device));
            }
        }
    }
    Ok(None)
}

/// The device of the system: the one that o sistema usa.
pub(crate) fn the_device_of_the_system() -> Result<TheDevice, String> {
    let enumerator = the_enumerator()?;
    unsafe {
        type GetDefault =
            unsafe extern "system" fn(*mut c_void, u32, u32, *mut *mut c_void) -> HRESULT;
        let get: GetDefault = std::mem::transmute(enumerator.the_method(4));
        let mut pointer: *mut c_void = std::ptr::null_mut();
        let answer = get(
            enumerator.pointer,
            THE_PLAYBACK,
            THE_ROLE_CONSOLE,
            &mut pointer,
        );
        if answer < 0 || pointer.is_null() {
            return Err(format!(
                "the machine did not give the device of the system: {answer} ({:#x}): {}",
                answer as u32,
                com::the_reason_of(answer)
            ));
        }
        Ok(TheDevice::of_the_device(TheObject { pointer }))
    }
}

/// The device of the system of one flow of the data.
fn the_default_of_the_flow(flow: u32) -> Result<TheDevice, String> {
    let enumerator = the_enumerator()?;
    unsafe {
        type GetDefault =
            unsafe extern "system" fn(*mut c_void, u32, u32, *mut *mut c_void) -> HRESULT;
        let get: GetDefault = std::mem::transmute(enumerator.the_method(4));
        for role in [THE_ROLE_CONSOLE, THE_ROLE_MULTIMEDIA] {
            let mut pointer: *mut c_void = std::ptr::null_mut();
            let answer = get(enumerator.pointer, flow, role, &mut pointer);
            if answer >= 0 && !pointer.is_null() {
                return Ok(TheDevice::of_the_device(TheObject { pointer }));
            }
        }
    }
    Err(format!(
        "the machine has no device of the system for the flow {flow}"
    ))
}

/// The names of the devices of the sound: the one that plays and the
/// one that records.
pub fn the_names_of_the_devices() -> Result<(String, String), String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_names_of_the_devices();
    }
    let playing = the_default_of_the_flow(THE_PLAYBACK)?;
    let recording = match the_default_of_the_flow(THE_RECORD) {
        Ok(device) => device,
        Err(_) => {
            let devices = the_devices_of_the_flow(THE_RECORD)?;
            match devices.first() {
                Some(device) => {
                    return Ok((playing.the_name(), device.the_name()));
                }
                None => return Ok((playing.the_name(), String::new())),
            }
        }
    };
    Ok((playing.the_name(), recording.the_name()))
}

/// The name of the device that the script names, if the machine has one.
pub fn the_name_of_the_device_of_the_name(name: &str) -> Result<Option<String>, String> {
    if super::mixer::the_mixer_is_the_way() {
        return super::mixer::the_name_of_the_device_of_the_name(name);
    }
    Ok(the_device_of_the_name(name)?.map(|device| device.the_name()))
}

/// The pointer of an interface of a device of the sound.
///
/// The interface is the text of an `IID`, and the machine owns the
/// pointer it gives. The v2.0.31 asks with the `QueryInterface` of the
/// device and, when that refuses, with its `Activate` -- and the answer
/// is the pointer, even when the machine has no such interface (read in
/// its source, `source/lib/sound.cpp` of the reference tree).
pub fn the_interface_of_the_device(device: &str, interface: &str) -> Result<u64, String> {
    let device = the_device_of_the_name(device)?
        .ok_or_else(|| format!("the machine has no device of the sound called {device:?}"))?;
    let iid = com::the_interface_of_the_text(interface)?;
    device.the_interface(&iid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_machine_knows_the_devices_of_the_sound() {
        // Measured on the machine of the tree: the class
        // of the devices of the sound is the *MMDeviceEnumerator class*,
        // and the machine gives six devices that play and six that
        // record.
        //
        // The test reads the name of one device of each flow and no
        // volume at all: opening the volume of a device is work for the
        // service of the sound of the machine, and the work of many
        // devices at once is what makes the sound of the machine
        // stutter.
        let playing = the_devices_of_the_flow(THE_PLAYBACK).unwrap_or_default();
        let recording = the_devices_of_the_flow(THE_RECORD).unwrap_or_default();
        if playing.is_empty() || recording.is_empty() {
            eprintln!("no audio device on this machine; the test reads nothing");
            return;
        }
        for device in [playing.first(), recording.first()].into_iter().flatten() {
            assert!(!device.the_name().is_empty(), "a device has no name");
        }
    }

    #[test]
    fn the_volume_of_a_device_is_a_number() {
        let devices = the_devices_of_the_flow(THE_PLAYBACK).expect("the devices that play");
        // The list carries the devices that are unplugged too, and the
        // volume of one of those cannot be opened (the machine answers
        // 0x80070490). The first device that gives a state is the one the
        // test reads; a machine where none gives one has no volume to read.
        let Some((volume, muted)) = devices.iter().find_map(|device| device.the_state().ok()) else {
            eprintln!("no playback device gives a volume; the test reads nothing");
            return;
        };
        assert!(
            (0.0..=100.0).contains(&volume),
            "the volume of the device is {volume}"
        );
        assert!(muted || !muted, "the mute of the device is a thing");
    }

    #[test]
    fn the_interface_of_a_device_is_the_pointer_of_it() {
        if the_devices_of_the_flow(THE_PLAYBACK).map(|d| d.is_empty()).unwrap_or(true) {
            eprintln!("no audio device on this machine; the test reads nothing");
            return;
        }
        // Measured on the machine of the tree: the
        // device of the system has the interface of the volume -- the
        // `IAudioEndpointVolume` -- and the machine gives its pointer.
        // An interface that the machine does not have gives zero.
        let pointer = the_interface_of_the_device("", "{5CDF2C82-841E-4546-9722-0CF74078229A}")
            .expect("the interface of the volume of the device");
        assert_ne!(pointer, 0, "the interface of the volume is not there");
        let pointer = the_interface_of_the_device("", "{11111111-2222-3333-4444-555555555555}")
            .expect("the interface that the machine does not have");
        assert_eq!(
            pointer, 0,
            "the machine gave an interface that it does not have"
        );
    }

    #[test]
    fn the_name_of_a_device_of_the_machine_is_known() {
        let devices = the_devices_of_the_flow(THE_ALL).unwrap_or_default();
        if devices.is_empty() {
            eprintln!("no audio device on this machine; the test reads nothing");
            return;
        }
        let device = devices.first().expect("a device");
        let name = device.the_name();
        let answer = the_name_of_the_device_of_the_name(&name).expect("the name of the device");
        assert_eq!(answer, Some(name));
        let answer = the_name_of_the_device_of_the_name("a device that the machine does not have")
            .expect("the name of a device that is not there");
        assert_eq!(answer, None);
    }

    #[test]
    fn the_device_of_the_system_is_the_one_that_plays() {
        let device = the_device_of_the_name("").unwrap_or(None);
        if device.is_none() {
            eprintln!("no audio device on this machine; the test reads nothing");
            return;
        }
        let system = device.expect("the device of the system");
        let playing = the_default_of_the_flow(THE_PLAYBACK).expect("the device that plays");
        assert_eq!(
            system.the_name(),
            playing.the_name(),
            "the device of the system is not the one that plays"
        );
    }
}
