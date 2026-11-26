//! The topology of a device of the sound.
//!
//! The v1 of Windows Vista and later does not ask the volume of the
//! device for the components that are not the master: it walks the
//! topology of the device and finds the part that is the component, and
//! the control of it.
//!
//! The walk is the one of the `SoundSetGet_FindComponent` of
//! v1.1.37.02:
//!
//! - a `SoundGet, out, Microphone, Volume` walks the topology of the
//!   device that plays, and not of the device that records: the
//!   `Microphone` of the v1 is the part of the microphone of the device
//!   that plays, and not the master of the device that records.
//!
//! The topology is a tree of parts: each part is a connector or a
//! subunit, and the subunits have the parts that come into them and the
//! ones that go out of them. The connector is the one that has the name
//! of the component -- the `KSNODETYPE_MICROPHONE` and the
//! `KSNODETYPE_DESKTOP_MICROPHONE` are the microphone -- and the
//! subunit is the one that has the control.
//!
//! A comparacao e a do v1: o `KSNODETYPE` da maquina e convertido no
//! tipo de componente do mixer (`MIXERLINE_COMPONENTTYPE_*`), e o
//! tipo do componente e o que a busca compara.

use std::ffi::c_void;
use std::ptr::null_mut;

use windows_sys::core::HRESULT;

use super::com::{self, TheObject};

/// The direction of the part: what comes into the part and what goes
/// out of it.
const THE_IN: u32 = 0;

/// The type of the part: zero is a connector, one is a subunit.
const THE_CONNECTOR: u32 = 0;
const THE_SUBUNIT: u32 = 1;

/// The types of the component of the mixer.
///
/// The numbers are the ones of the `mmeapi.h` of the machine: the ones
/// of the destination begin at zero and the ones of the source at
/// `0x1000`.
pub const DST_SPEAKERS: u32 = 0x0004;
pub const DST_HEADPHONES: u32 = 0x0005;
pub const SRC_UNDEFINED: u32 = 0x1000;
pub const SRC_DIGITAL: u32 = 0x1001;
pub const SRC_LINE: u32 = 0x1002;
pub const SRC_MICROPHONE: u32 = 0x1003;
pub const SRC_SYNTHESIZER: u32 = 0x1004;
pub const SRC_COMPACTDISC: u32 = 0x1005;
pub const SRC_TELEPHONE: u32 = 0x1006;
pub const SRC_PCSPEAKER: u32 = 0x1007;
pub const SRC_ANALOG: u32 = 0x1008;
pub const SRC_AUXILIARY: u32 = 0x1009;
pub const SRC_WAVEOUT: u32 = 0x100A;

/// A parte da topologia de um device.
struct ThePart {
    part: TheObject,
}

impl ThePart {
    /// O tipo da parte: o conector ou a subunidade.
    fn the_type(&self) -> Option<u32> {
        unsafe {
            type GetPartType = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
            let get: GetPartType = std::mem::transmute(self.part.the_method(6));
            let mut kind: u32 = 0;
            let answer = get(self.part.pointer, &mut kind);
            if answer < 0 {
                return None;
            }
            Some(kind)
        }
    }

    /// O nome do tipo da parte, e o `KSNODETYPE` da maquina.
    fn the_sub_type(&self) -> Option<windows_sys::core::GUID> {
        unsafe {
            type GetSubType = unsafe extern "system" fn(*mut c_void, *mut windows_sys::core::GUID)
                -> HRESULT;
            let get: GetSubType = std::mem::transmute(self.part.the_method(7));
            let mut number: windows_sys::core::GUID = std::mem::zeroed();
            let answer = get(self.part.pointer, &mut number);
            if answer < 0 {
                return None;
            }
            Some(number)
        }
    }

    /// The parts that come into this part, or the ones that go out of
    /// it.
    ///
    /// The two calls of the machine do not take the direction: the `In`
    /// and the `Out` are different methods of the `IPart`, the tenth and
    /// the eleventh.
    fn the_parts(&self, direction: u32) -> Vec<ThePart> {
        let mut found = Vec::new();
        unsafe {
            type EnumParts = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
            let index = if direction == THE_IN { 10 } else { 11 };
            let enumerate: EnumParts = std::mem::transmute(self.part.the_method(index));
            let mut list: *mut c_void = null_mut();
            let answer = enumerate(self.part.pointer, &mut list);
            if answer < 0 || list.is_null() {
                return found;
            }
            // A lista e dada de volta a maquina so depois que as partes
            // foram lidas: cada parte da lista e um objeto que a
            // maquina conta, e a lista e a dona deles.
            let table = *(list as *const *const *mut c_void);
            type GetCount = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
            let count_of: GetCount = std::mem::transmute(*table.add(3));
            let mut count: u32 = 0;
            if count_of(list, &mut count) < 0 {
                return found;
            }
            type GetPart = unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
            let part_of: GetPart = std::mem::transmute(*table.add(4));
            for index in 0..count {
                let mut pointer: *mut c_void = null_mut();
                if part_of(list, index, &mut pointer) < 0 || pointer.is_null() {
                    continue;
                }
                found.push(ThePart {
                    part: TheObject { pointer },
                });
            }
            // The list is given back: the parts that it had have a
            // count of use of each one, and the list loses its own.
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(*table.add(2));
            release(list);
        }
        found
    }

    /// O controle da parte, se a parte o tem.
    ///
    /// O controle e uma das formas do COM: o volume de um nivel
    /// (`IAudioVolumeLevel`) ou o mute (`IAudioMute`).
    fn the_control(&self, interface: &windows_sys::core::GUID) -> Option<TheObject> {
        the_control_of_the_part(self.part.pointer, interface)
    }
}

/// The control of a part, from its raw pointer.
///
/// A funcao toma o ponteiro cru e nao o solta: o conector que a busca
/// guarda e emprestado, e um objeto temporario que o soltasse no `Drop`
/// deixaria a contagem dele curta.
fn the_control_of_the_part(
    pointer: *mut c_void,
    interface: &windows_sys::core::GUID,
) -> Option<TheObject> {
    unsafe {
        type Activate = unsafe extern "system" fn(
            *mut c_void,
            u32,
            *const windows_sys::core::GUID,
            *mut *mut c_void,
        ) -> HRESULT;
        // O metodo 13 do `IPart` e o `Activate`.
        let table = *(pointer as *const *const *mut c_void);
        let activate: Activate = std::mem::transmute(*table.add(13));
        let mut control: *mut c_void = null_mut();
        let answer = activate(
            pointer,
            // O v1 abre o controle com o `CLSCTX_ALL`.
            0x17,
            interface,
            &mut control,
        );
        if answer < 0 || control.is_null() {
            return None;
        }
        Some(TheObject { pointer: control })
    }
}

/// A busca de um componente na topologia de um device.
///
/// A busca e a do `SoundComponentSearch` do v1.1.37.02: o tipo de
/// componente que e procurado, e o controle que e perguntado.
pub struct TheSearch {
    /// O tipo de componente do mixer que e procurado.
    pub wanted: u32,
    /// A forma do controle que e perguntado, e `None` quando a busca e
    /// so pelo componente.
    pub control: Option<windows_sys::core::GUID>,
    /// O controle que foi achado, e o que a busca devolve.
    pub found: Option<TheObject>,
    /// O conector do tipo que se procura, quando ele foi achado.
    ///
    /// O mute de um componente mora no conector, e o volume dele mora na
    /// subunidade: os dois sao procurados.
    connector: Option<*mut c_void>,
    /// Quantos componentes do tipo procurado foram achados.
    pub count: u32,
    /// A busca pelo numero do componente, e nao pelo tipo.
    ///
    /// O v2 nomeia um componente pelo lugar dele na lista: o
    /// `SoundGetVolume("1")` e o primeiro componente da topologia.
    /// Quando e `Some(N)`, a busca conta todos os componentes e para
    /// no N-esimo, seja qual for o tipo dele.
    pub by_index: Option<u32>,
    /// Whether the component was found, even without its control.
    ///
    /// O v2 distingue o componente que nao esta do componente que esta
    /// e nao tem o controle: o primeiro e "Component not found" e o
    /// segundo e "Component doesn't support this control type".
    pub component_found: bool,
}

/// O tipo de componente do mixer que o `KSNODETYPE` da maquina e.
///
/// O mapa e o do `SoundSetGet_ComponentType` do v1.1.37.02: o
/// microfone e o `KSNODETYPE_MICROPHONE` e o
/// `KSNODETYPE_DESKTOP_MICROPHONE`, e cada componente tem o seu.
pub fn the_component_type_of(sub_type: &windows_sys::core::GUID) -> u32 {
    use windows_sys::Win32::Media::KernelStreaming::{
        KSCATEGORY_AUDIO, KSNODETYPE_ANALOG_CONNECTOR, KSNODETYPE_CD_PLAYER,
        KSNODETYPE_DESKTOP_MICROPHONE, KSNODETYPE_LEGACY_AUDIO_CONNECTOR,
        KSNODETYPE_LINE_CONNECTOR, KSNODETYPE_MICROPHONE, KSNODETYPE_PHONE_LINE,
        KSNODETYPE_SPDIF_INTERFACE, KSNODETYPE_SPEAKER, KSNODETYPE_SYNTHESIZER,
        KSNODETYPE_TELEPHONE,
    };
    let same = |one: &windows_sys::core::GUID, other: &windows_sys::core::GUID| {
        one.data1 == other.data1
            && one.data2 == other.data2
            && one.data3 == other.data3
            && one.data4 == other.data4
    };
    if same(sub_type, &KSNODETYPE_MICROPHONE) || same(sub_type, &KSNODETYPE_DESKTOP_MICROPHONE) {
        SRC_MICROPHONE
    } else if same(sub_type, &KSNODETYPE_LEGACY_AUDIO_CONNECTOR)
        || same(sub_type, &KSCATEGORY_AUDIO)
        || same(sub_type, &KSNODETYPE_SPEAKER)
    {
        SRC_WAVEOUT
    } else if same(sub_type, &KSNODETYPE_CD_PLAYER) {
        SRC_COMPACTDISC
    } else if same(sub_type, &KSNODETYPE_SYNTHESIZER) {
        SRC_SYNTHESIZER
    } else if same(sub_type, &KSNODETYPE_LINE_CONNECTOR) {
        SRC_LINE
    } else if same(sub_type, &KSNODETYPE_TELEPHONE) || same(sub_type, &KSNODETYPE_PHONE_LINE) {
        SRC_TELEPHONE
    } else if same(sub_type, &KSNODETYPE_ANALOG_CONNECTOR) {
        SRC_ANALOG
    } else if same(sub_type, &KSNODETYPE_SPDIF_INTERFACE) {
        SRC_DIGITAL
    } else {
        SRC_UNDEFINED
    }
}

/// Anda pela topologia do device e acha o componente.
///
/// The topology begins at the connector zero of the device, and the
/// search follows the parts that come into it: the `Microphone` of the
/// v1 is the part of the microphone, and not the master of the device
/// that records.
pub fn the_component_of_the_device(
    device: &TheObject,
    wanted: u32,
    control: Option<windows_sys::core::GUID>,
) -> Option<TheObject> {
    let mut search = TheSearch {
        wanted,
        control,
        found: None,
        connector: None,
        count: 0,
        by_index: None,
        component_found: false,
    };
    if !the_search_of_the_device(device, &mut search) {
        return None;
    }
    search.found
}

/// The component of the device by its place in the list.
///
/// O v2 nomeia um componente pelo numero dele: o `SoundGetVolume("1")`
/// e o primeiro componente da topologia. A resposta distingue o
/// componente que nao esta do componente que esta e nao tem o controle.
///
/// Medido no binario do v2.0.31: o `SoundGetVolume("1")`
/// de um device cujo primeiro componente nao tem volume recusa com
/// "Component doesn't support this control type", e o
/// `SoundGetVolume("2")` recusa com "Component not found".
///
/// A resposta e `None` quando a topologia nao abre; `Some((false, _))`
/// quando o componente nao esta; `Some((true, None))` quando ele esta e
/// nao tem o controle; e `Some((true, Some(control)))` quando ele esta
/// com o controle.
pub fn the_component_of_the_device_by_index(
    device: &TheObject,
    index: u32,
    control: Option<windows_sys::core::GUID>,
) -> Option<(bool, Option<TheObject>)> {
    let mut search = TheSearch {
        wanted: SRC_UNDEFINED,
        control,
        found: None,
        connector: None,
        count: 0,
        by_index: Some(index),
        component_found: false,
    };
    if !the_search_of_the_device(device, &mut search) {
        return None;
    }
    Some((search.component_found, search.found))
}

/// Opens the topology of the device and walks it with the search.
///
/// The topology begins at the connector zero of the device, and the
/// search follows the parts that come into it: the `Microphone` of the
/// v1 is the part of the microphone, and not the master of the device
/// that records.
fn the_search_of_the_device(device: &TheObject, search: &mut TheSearch) -> bool {
    // The topology of the device: the machine gives its shape and the
    // connector zero.
    let topology = unsafe {
        // The `Activate` of the `IMMDevice` takes five arguments: the
        // object, the number of the shape, the context, the parameters
        // of the activation and the place of the answer.
        type Activate = unsafe extern "system" fn(
            *mut c_void,
            *const windows_sys::core::GUID,
            u32,
            *mut c_void,
            *mut *mut c_void,
        ) -> HRESULT;
        let activate: Activate = std::mem::transmute(device.the_method(3));
        let mut pointer: *mut c_void = null_mut();
        // The number of the shape is kept before the call: the
        // `IIDFromString` gives a new number every time, and the call
        // needs the same number during its whole life.
        let Some(number) = com::the_interface_of_the_topology().ok() else {
            return false;
        };
        let answer = activate(
            device.pointer,
            &number,
            // The v1 opens the topology with the `CLSCTX_ALL`: the
            // topology is an object that the machine gives, and not a
            // class that is instantiated.
            0x17, // CLSCTX_ALL
            null_mut(),
            &mut pointer,
        );
        if answer < 0 || pointer.is_null() {
            return false;
        }
        TheObject { pointer }
    };
    // O conector zero do device, e a parte de onde a busca comeca.
    let connector = unsafe {
        type GetConnector = unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
        // O metodo 4 do `IDeviceTopology` e o `GetConnector`.
        let get: GetConnector = std::mem::transmute(topology.the_method(4));
        let mut pointer: *mut c_void = null_mut();
        let answer = get(topology.pointer, 0, &mut pointer);
        if answer < 0 || pointer.is_null() {
            return false;
        }
        TheObject { pointer }
    };
    // O fluxo do conector: o que entra no device e o que sai dele.
    let flow = unsafe {
        type GetDataFlow = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
        // O metodo 4 do `IConnector` e o `GetDataFlow`.
        let get: GetDataFlow = std::mem::transmute(connector.the_method(4));
        let mut flow: u32 = 0;
        let answer = get(connector.pointer, &mut flow);
        if answer < 0 {
            return false;
        }
        flow
    };
    // The part on the other side of the connector: it is by it that the
    // search begins. The `GetConnectedTo` gives a connector, and not a
    // part: the v1 asks the connector for the shape of the part.
    let root = unsafe {
        type GetConnectedTo = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
        // O metodo 8 do `IConnector` e o `GetConnectedTo`.
        let get: GetConnectedTo = std::mem::transmute(connector.the_method(8));
        let mut pointer: *mut c_void = null_mut();
        let answer = get(connector.pointer, &mut pointer);
        if answer < 0 || pointer.is_null() {
            return false;
        }
        // The part is the other shape of the connector: the search uses
        // it and does not need its own shape.
        type QueryInterface = unsafe extern "system" fn(
            *mut c_void,
            *const windows_sys::core::GUID,
            *mut *mut c_void,
        ) -> HRESULT;
        // O `QueryInterface` e o metodo 0 do `IUnknown`.
        let table = *(pointer as *const *const *mut c_void);
        let ask: QueryInterface = std::mem::transmute(*table.add(0));
        let mut part: *mut c_void = null_mut();
        let Some(shape) = com::the_interface_of_the_part().ok() else {
            return false;
        };
        let answer = ask(pointer, &shape, &mut part);
        if answer < 0 || part.is_null() {
            return false;
        }
        TheObject { pointer: part }
    };
    // O sentido da busca: o v1 usa o `In` quando o fluxo do conector e
    // o `In`, e o `Out` quando o fluxo e o `Out`.
    the_part_is_searched(&ThePart { part: root }, search, flow);
    // O conector que a busca guardou e devolvido a maquina pelo `Drop`
    // do objeto: a contagem a mais foi feita quando ele foi guardado.
    if let Some(connector) = search.connector {
        drop(TheObject { pointer: connector });
    }
    true
}

/// Walks the parts and finds the component.
///
/// The search is the one of the `SoundSetGet_FindComponent` of
/// v1.1.37.02: the parts that come into the part are seen one by one,
/// and the connector whose type is the one that is wanted counts a
/// component. The subunit that has that connector is the one that has
/// the control, and this is why the search gives the control to it.
fn the_part_is_searched(part: &ThePart, search: &mut TheSearch, direction: u32) -> bool {
    let parts = part.the_parts(direction);
    let count = parts.len();
    for child in &parts {
        match child.the_type() {
            Some(THE_CONNECTOR) => {
                let Some(sub_type) = child.the_sub_type() else {
                    continue;
                };
                let the_type_matches = match search.by_index {
                    Some(_) => the_component_type_of(&sub_type) != SRC_UNDEFINED,
                    None => the_component_type_of(&sub_type) == search.wanted,
                };
                if the_type_matches {
                    search.count += 1;
                    let reached = match search.by_index {
                        Some(n) => search.count == n,
                        None => search.count == 1,
                    };
                    if reached {
                        search.component_found = true;
                        // O conector achado e o que da a subunidade que
                        // o controla: a busca volta para tras e pergunta
                        // o controle a subunidade que tem o conector.
                        //
                        // E o conector e o que tem o mute dele: medido
                        // no binario do v2.0.31, o
                        // `SoundGetMute("Microphone")` responde um, e o
                        // mute mora no conector.
                        // A contagem a mais: a parte que tem o
                        // conector e devolvida a maquina quando a
                        // volta da busca acaba, e o ponteiro fica.
                        child.part.the_object_is_counted();
                        search.connector = Some(child.part.pointer);
                        return true;
                    }
                }
            }
            Some(THE_SUBUNIT) => {
                // A subunidade que tem o conector e a que tem o
                // controle: o v1 pergunta o controle a ela, e so a
                // primeira vez que ele e achado.
                //
                // And the v1 asks the control only of the first
                // subunit that has a connector: when the subunit has
                // more than one part, the controls that are further back
                // are the ones of the master, and not the ones of the
                // component.
                if !the_part_is_searched(child, search, direction) {
                    continue;
                }
                if let Some(interface) = search.control {
                    if search.found.is_none() {
                        search.found = child.the_control(&interface);
                    }
                    // O mute mora no conector, e o volume mora na
                    // subunidade.
                    if search.found.is_none() {
                        if let Some(connector) = search.connector {
                            search.found = the_control_of_the_part(connector, &interface);
                        }
                    }
                    if count > 1 {
                        search.control = None;
                    }
                }
                return true;
            }
            _ => {}
        }
    }
    false
}
