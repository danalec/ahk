//! The events of the COM of the machine.
//!
//! The machine calls a script at every event of an object it has been
//! connected to. The call arrives at the `sink` -- an `IDispatch` that
//! the tree makes and the machine owns -- and the sink puts it in the
//! queue that the desktop reads between the statements of the script.
//!
//! The shape is the one of the v2.0.31, read in its source
//! (`source/script_com.cpp` of the reference tree): the name of the
//! member comes from the type information of the events interface, the
//! name of the event is the prefix and the name of the member, and the
//! object that fired the event goes as the last argument.

use core::cell::Cell;
use core::ffi::c_void;

use std::sync::{Arc, Mutex};

use ahk::desktop::ComValue;
use windows_sys::core::{GUID, HRESULT};

/// An event that the machine gave.
#[derive(Clone, Debug)]
pub struct TheEvent {
    /// The name of the event: the prefix and the name of the member.
    pub name: String,
    /// The pointer of the object that fired the event.
    pub object: u64,
    /// The kind of the value of the object: the `VARTYPE` of v2.
    pub kind: i64,
    /// The arguments of the event.
    pub arguments: Vec<ComValue>,
}

/// The `VARIANT` of the machine.
#[repr(C)]
pub(crate) struct TheVariant {
    pub(crate) kind: u16,
    _reserved1: u16,
    _reserved2: u16,
    _reserved3: u16,
    pub(crate) data: u64,
}

/// The `DISPPARAMS` of the machine.
#[repr(C)]
pub(crate) struct TheParameters {
    /// The arguments, reversed: the last one comes first.
    pub(crate) arguments: *mut TheVariant,
    /// The named arguments.
    pub(crate) named: *mut i32,
    /// How many arguments there are.
    pub(crate) count: u32,
    /// How many named arguments there are.
    pub(crate) named_count: u32,
}

/// The value of a variant of the machine, in the shape of the desktop.
fn the_value_of(variant: &TheVariant) -> ComValue {
    match variant.kind {
        // `VT_BSTR`.
        8 => {
            let pointer = variant.data as *const u16;
            if pointer.is_null() {
                return ComValue::Text(String::new());
            }
            let mut length = 0usize;
            unsafe {
                while *pointer.add(length) != 0 {
                    length += 1;
                }
                ComValue::Text(String::from_utf16_lossy(std::slice::from_raw_parts(
                    pointer, length,
                )))
            }
        }
        // `VT_I2` and `VT_BOOL`.
        2 | 11 => ComValue::Number((variant.data as u16) as i16 as f64),
        // `VT_I4`.
        3 => ComValue::Number((variant.data as u32) as i32 as f64),
        // `VT_R4`.
        4 => ComValue::Number(f32::from_bits(variant.data as u32) as f64),
        // `VT_R8`.
        5 => ComValue::Number(f64::from_bits(variant.data)),
        // `VT_I8`.
        20 => ComValue::Number(variant.data as i64 as f64),
        // `VT_DISPATCH` and `VT_UNKNOWN`.
        9 | 13 => ComValue::Pointer(variant.data),
        _ => ComValue::Number(0.0),
    }
}

/// The value of the desktop, in the shape of the machine.
pub(crate) fn the_variant_of(value: &ComValue) -> TheVariant {
    let mut variant = TheVariant {
        kind: 0,
        _reserved1: 0,
        _reserved2: 0,
        _reserved3: 0,
        data: 0,
    };
    match value {
        ComValue::Number(number) => {
            if number.fract() == 0.0
                && *number >= i32::MIN as f64
                && *number <= i32::MAX as f64
            {
                variant.kind = 3; // VT_I4
                variant.data = (*number as i32 as u32) as u64;
            } else {
                variant.kind = 5; // VT_R8
                variant.data = number.to_bits();
            }
        }
        ComValue::Text(text) => {
            variant.kind = 8; // VT_BSTR
            let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
            let bstr = unsafe { windows_sys::Win32::Foundation::SysAllocString(wide.as_ptr()) };
            variant.data = bstr as u64;
        }
        ComValue::Pointer(pointer) => {
            variant.kind = 9; // VT_DISPATCH
            variant.data = *pointer;
        }
    }
    variant
}

/// The answer of the machine, in the shape of the desktop.
///
/// The `VARIANT` is the machine's: what is inside it goes back to the
/// machine, and an object that the script keeps is held one more time
/// before the machine gives its own hold away.
pub(crate) fn the_answer_of(variant: &mut TheVariant) -> ahk::desktop::ComAnswer {
    use ahk::desktop::ComAnswer;
    let kind = variant.kind & 0x0FFF;
    let answer = match kind {
        0 | 1 => ComAnswer::Empty,
        8 => {
            let value = the_value_of(&*variant);
            match value {
                ComValue::Text(text) => ComAnswer::Text(text),
                _ => ComAnswer::Empty,
            }
        }
        9 | 13 => {
            let pointer = variant.data as *mut c_void;
            if pointer.is_null() {
                ComAnswer::Empty
            } else {
                unsafe {
                    type AddRef = unsafe extern "system" fn(*mut c_void) -> u32;
                    let add_ref: AddRef =
                        std::mem::transmute(*(*(pointer as *const *const usize)).add(1));
                    add_ref(pointer);
                }
                ComAnswer::Object {
                    pointer: variant.data,
                    kind: i64::from(kind),
                }
            }
        }
        _ => match the_value_of(&*variant) {
            ComValue::Number(number) => ComAnswer::Number(number),
            _ => ComAnswer::Empty,
        },
    };
    unsafe {
        // The layout of `TheVariant` is the one of the `VARIANT` of the
        // machine: the kind, three reserved words, and the data.
        windows_sys::Win32::System::Variant::VariantClear(
            variant as *mut TheVariant as *mut windows_sys::Win32::System::Variant::VARIANT,
        );
    }
    answer
}

/// The `IDispatch` that the machine calls when an event fires.
///
/// The first field is the pointer to the table of the methods, and the
/// table is in the object.
///
/// The `QueryInterface` answers the `IDispatch` and the interface of
/// the events: the machine asks for the events interface before it
/// calls the `sink`. This is the shape of the v2.0.31, read in its
/// source (`source/script_com.cpp`).
#[repr(C)]
pub struct TheSink {
    /// The pointer to the table of the methods.
    table: *const usize,
    /// The table: the address of each method, in its place.
    methods: [usize; 7],
    /// The count of the references that the machine holds.
    references: Cell<u32>,
    /// The type information of the events interface.
    type_info: *mut c_void,
    /// The number of the events interface.
    events_iid: GUID,
    /// The queue of the events that the machine gave.
    events: Arc<Mutex<Vec<TheEvent>>>,
    /// The prefix of the names of the events.
    prefix: String,
    /// The pointer of the object of the events.
    object: u64,
    /// The kind of the value of the object.
    kind: i64,
}

impl TheSink {
    /// The `sink` of the events of an object, made on the heap: the
    /// machine holds the pointer and gives it back with `Release`.
    pub fn new(
        type_info: *mut c_void,
        events_iid: &GUID,
        prefix: &str,
        events: Arc<Mutex<Vec<TheEvent>>>,
        object: u64,
        kind: i64,
    ) -> *mut TheSink {
        let sink = Box::new(Self {
            table: std::ptr::null(),
            methods: [
                the_query_interface as *const () as usize,
                the_add_ref as *const () as usize,
                the_release as *const () as usize,
                the_count_of_the_types as *const () as usize,
                the_type_information as *const () as usize,
                the_ids_of_the_names as *const () as usize,
                the_invoke as *const () as usize,
            ],
            references: Cell::new(1),
            type_info,
            events_iid: *events_iid,
            events,
            prefix: prefix.to_string(),
            object,
            kind,
        });
        let sink = Box::into_raw(sink);
        // The table is inside the object: the pointer points at it.
        unsafe { (*sink).table = (*sink).methods.as_ptr() };
        sink
    }

    /// The name of the member that the number of the call names.
    fn the_name_of_the_member(&self, number: i32) -> Option<String> {
        unsafe {
            // `ITypeInfo::GetNames` is the eighth method of the table.
            type GetNames = unsafe extern "system" fn(
                *mut c_void,
                i32,
                *mut *mut u16,
                u32,
                *mut u32,
            ) -> HRESULT;
            let get: GetNames = std::mem::transmute(the_slot_of(self.type_info, 7));
            let mut name: *mut u16 = std::ptr::null_mut();
            let mut count = 0u32;
            if get(self.type_info, number, &mut name, 1, &mut count) < 0 || name.is_null() {
                return None;
            }
            let mut length = 0usize;
            while *name.add(length) != 0 {
                length += 1;
            }
            let text = String::from_utf16_lossy(std::slice::from_raw_parts(name, length));
            windows_sys::Win32::Foundation::SysFreeString(name.cast());
            Some(text)
        }
    }

    /// The arguments of the call, in the order that the script takes them.
    fn the_arguments(&self, parameters: *mut TheParameters) -> Vec<ComValue> {
        let mut arguments = Vec::new();
        if parameters.is_null() {
            return arguments;
        }
        let parameters = unsafe { &*parameters };
        if parameters.arguments.is_null() {
            return arguments;
        }
        // The arguments come reversed: the last one comes first.
        for index in (0..parameters.count).rev() {
            let variant = unsafe { &*parameters.arguments.add(index as usize) };
            arguments.push(the_value_of(variant));
        }
        arguments
    }
}

/// `S_OK`.
const THE_OK: HRESULT = 0;
/// `E_NOINTERFACE`.
const NO_INTERFACE: HRESULT = -2147467262;
/// `E_FAIL`.
const THE_FAILURE: HRESULT = -2147467259;
/// `DISP_E_MEMBERNOTFOUND`.
const MEMBER_NOT_FOUND: HRESULT = -2147352572;

/// The `QueryInterface` of the `sink`.
///
/// The `sink` answers the `IUnknown`, the `IDispatch` and the interface
/// of the events. Measured on the machine of the tree:
/// the object fires the `Invoke` of the `sink` with the number of the
/// member when the `sink` answers the interface of the events and the
/// count of the types; with the interface refused the object calls
/// nothing.
extern "system" fn the_query_interface(
    this: *mut c_void,
    iid: *const GUID,
    out: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        if out.is_null() {
            return THE_FAILURE;
        }
        *out = std::ptr::null_mut();
        // The `IDispatch`: `{00020400-…-000000000046}`.
        let dispatch: [u8; 16] = [
            0x00, 0x04, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x46,
        ];
        // The `IUnknown`: `{00000000-0000-0000-C000-000000000046}`.
        let unknown: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x46,
        ];
        let sink = &*(this as *const TheSink);
        let wanted: [u8; 16] = std::mem::transmute(sink.events_iid);
        let bytes: [u8; 16] = std::mem::transmute(*iid);
        if bytes != dispatch && bytes != unknown && bytes != wanted {
            return NO_INTERFACE;
        }
        *out = this;
        the_add_ref(this);
    }
    THE_OK
}

/// The `AddRef` of the `sink`.
extern "system" fn the_add_ref(this: *mut c_void) -> u32 {
    unsafe {
        let sink = &*(this as *const TheSink);
        let count = sink.references.get() + 1;
        sink.references.set(count);
        count
    }
}

/// The `Release` of the `sink`.
extern "system" fn the_release(this: *mut c_void) -> u32 {
    unsafe {
        let sink = &*(this as *const TheSink);
        let count = sink.references.get();
        sink.references.set(count - 1);
        if count == 1 {
            // The type information is the machine's, and the machine
            // takes it back.
            if !sink.type_info.is_null() {
                type Release = unsafe extern "system" fn(*mut c_void) -> u32;
                let release: Release = std::mem::transmute(the_slot_of(sink.type_info, 2));
                release(sink.type_info);
            }
            drop(Box::from_raw(this as *mut TheSink));
        }
        count - 1
    }
}

/// The `GetTypeInfoCount` of the `sink`.
///
/// Measured on the machine of the tree: the object of the
/// `MSXML2.DOMDocument` calls the `Invoke` of the `sink` with the number
/// of the member when the `sink` answers the interface of the events and
/// the count of the types, one. With the count zero, or with the
/// interface of the events refused, the object calls nothing: the raw
/// slots of the interface of the events are what it knows, and it wants
/// the information of the type to read them. The v2.0.31 answers the
/// three, and the machine is the one the v2 was written against.
extern "system" fn the_count_of_the_types(_this: *mut c_void, out: *mut u32) -> HRESULT {
    unsafe {
        if !out.is_null() {
            *out = 1;
        }
    }
    THE_OK
}

/// The `GetTypeInfo` of the `sink`: the information of the events
/// interface, held one more time for the machine.
extern "system" fn the_type_information(
    this: *mut c_void,
    _index: u32,
    _lcid: u32,
    out: *mut *mut c_void,
) -> HRESULT {
    unsafe {
        if out.is_null() {
            return THE_FAILURE;
        }
        let sink = &*(this as *const TheSink);
        if sink.type_info.is_null() {
            *out = std::ptr::null_mut();
            return NO_INTERFACE;
        }
        type AddRef = unsafe extern "system" fn(*mut c_void) -> u32;
        let table = *(sink.type_info as *const *const *mut c_void);
        let add_ref: AddRef = std::mem::transmute(*table.add(1));
        add_ref(sink.type_info);
        *out = sink.type_info;
    }
    THE_OK
}

/// The `GetIDsOfNames` of the `sink`.
extern "system" fn the_ids_of_the_names(
    this: *mut c_void,
    _iid: *const GUID,
    names: *mut *mut u16,
    count: u32,
    _lcid: u32,
    out: *mut i32,
) -> HRESULT {
    unsafe {
        let sink = &*(this as *const TheSink);
        for index in 0..count {
            let name = *names.add(index as usize);
            let mut length = 0usize;
            while *name.add(length) != 0 {
                length += 1;
            }
            let text = String::from_utf16_lossy(std::slice::from_raw_parts(name, length));
            let Some(number) = the_number_of_the_member_of(sink.type_info, &text) else {
                return MEMBER_NOT_FOUND;
            };
            *out.add(index as usize) = number;
        }
    }
    THE_OK
}

/// The `Invoke` of the `sink`: the machine gave an event.
extern "system" fn the_invoke(
    this: *mut c_void,
    number: i32,
    _iid: *const GUID,
    _lcid: u32,
    _flags: u16,
    parameters: *mut TheParameters,
    _result: *mut TheVariant,
    _exception: *mut u8,
    _argument_error: *mut u32,
) -> HRESULT {
    unsafe {
        let sink = &*(this as *const TheSink);
        let Some(member) = sink.the_name_of_the_member(number) else {
            return MEMBER_NOT_FOUND;
        };
        let arguments = sink.the_arguments(parameters);
        let event = TheEvent {
            name: format!("{}{member}", sink.prefix),
            object: sink.object,
            kind: sink.kind,
            arguments,
        };
        if let Ok(mut events) = sink.events.lock() {
            events.push(event);
        }
    }
    THE_OK
}

/// The address of a method in the table of an object of the machine.
///
/// The first field of the object is the pointer to the table, and the
/// table holds one address in each place.
fn the_slot_of(object: *mut c_void, number: usize) -> usize {
    unsafe {
        let table = *(object as *const *const usize);
        *table.add(number)
    }
}

/// The number of the member of this name in the type information.
fn the_number_of_the_member_of(type_info: *mut c_void, name: &str) -> Option<i32> {
    unsafe {
        // `ITypeInfo::GetIDsOfNames` is the eleventh method of the
        // table, and it takes no locale.
        type GetIDsOfNames =
            unsafe extern "system" fn(*mut c_void, *mut *mut u16, u32, *mut i32) -> HRESULT;
        let get: GetIDsOfNames = std::mem::transmute(the_slot_of(type_info, 10));
        let mut wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
        let mut names: *mut u16 = wide.as_mut_ptr();
        let mut number: i32 = 0;
        if get(type_info, &mut names, 1, &mut number) < 0 {
            None
        } else {
            Some(number)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_value_of_the_machine_becomes_the_value_of_the_desktop() {
        // The number of the `VT_R8` is the bits of the float, the text
        // of the `VT_BSTR` is the wide text, and the object of the
        // `VT_UNKNOWN` is the pointer.
        let number = TheVariant {
            kind: 5,
            _reserved1: 0,
            _reserved2: 0,
            _reserved3: 0,
            data: 4.0f64.to_bits(),
        };
        assert_eq!(the_value_of(&number), ComValue::Number(4.0));
        let pointer = TheVariant {
            kind: 13,
            _reserved1: 0,
            _reserved2: 0,
            _reserved3: 0,
            data: 0x1_0000,
        };
        assert_eq!(the_value_of(&pointer), ComValue::Pointer(0x1_0000));
        let wide: Vec<u16> = "Speakers"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();
        let text = TheVariant {
            kind: 8,
            _reserved1: 0,
            _reserved2: 0,
            _reserved3: 0,
            data: wide.as_ptr() as u64,
        };
        assert_eq!(the_value_of(&text), ComValue::Text("Speakers".to_string()));
    }

    #[test]
    fn the_sink_names_the_member_of_the_machine() {
        // One object of the machine: the source of the events of the
        // `MSXML2.DOMDocument` gives the names `ondataavailable` and
        // `onreadystatechange` for the numbers 198 and -609 -- the same
        // numbers that the object carries to the `Invoke` of the `sink`
        // (measured on the machine of the tree). The tree
        // had the read of the method of the table one dereference short:
        // it called the seventh field of the object as the method, and
        // the name came back of nowhere -- the event was lost in quiet.
        let object = crate::windows::com::the_object_of_the_class("MSXML2.DOMDocument.6.0", None)
            .expect("the object of the machine");
        let class = crate::windows::com::the_class_information_of_the_object(&object)
            .expect("the information of the class")
            .expect("the class of the object of the machine");
        let source = crate::windows::com::the_interface_of_the_source_of_the_events(class)
            .expect("the source of the events of the class");
        let sink = TheSink {
            table: std::ptr::null(),
            methods: [0; 7],
            references: Cell::new(1),
            type_info: source,
            events_iid: GUID {
                data1: 0,
                data2: 0,
                data3: 0,
                data4: [0; 8],
            },
            events: Arc::new(Mutex::new(Vec::new())),
            prefix: String::new(),
            object: 0,
            kind: 0,
        };
        assert_eq!(
            sink.the_name_of_the_member(-609).as_deref(),
            Some("onreadystatechange")
        );
        assert_eq!(
            sink.the_name_of_the_member(198).as_deref(),
            Some("ondataavailable")
        );
        unsafe {
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let table = *(source as *const *const *mut c_void);
            let release: Release = std::mem::transmute(*table.add(2));
            release(source);
        }
    }

    #[test]
    fn the_sink_goes_back_to_the_machine_when_the_last_reference_goes() {
        // The table of the `sink` points at the methods of the `sink`
        // itself, and the count of the references falls to the machine.
        let events = Arc::new(Mutex::new(Vec::new()));
        let iid = GUID {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        };
        let sink = TheSink::new(std::ptr::null_mut(), &iid, "On", events, 7, 9);
        let pointer = sink as *mut c_void;
        assert_eq!(the_add_ref(pointer), 2);
        assert_eq!(the_release(pointer), 1);
        assert_eq!(the_release(pointer), 0);
    }
}
