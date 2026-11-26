// SPDX-License-Identifier: Apache-2.0
//! The structs of the v2.1.
//!
//! A struct is a collection of typed fields stored adjacently in memory.
//! The script defines one with `struct Name { field : Type }`, and the
//! fields are laid out with the alignment of their types, the way the C
//! compiler lays out the struct of the same fields.
//!
//! Measured on the binary of v2.1-alpha.39 and on the notes
//! of v2.1-alpha.22: the numeric types (`Int32`, `UInt32`, ...) are struct
//! classes of one field, `Struct` is the empty base of them all, and a
//! struct class defaults to extending `Struct`.
//!
//! The bytes of a struct instance live in the state of a buffer, so the
//! reads, the writes and the address of them are the ones the buffer
//! module already owns.

use alloc::string::String;
use alloc::vec::Vec;

use crate::dll::NativeType;

/// The native type of a numeric struct class of the v2.1.
///
/// `Int8`..`Int64` are signed, `UInt8`..`UInt32` unsigned, `IntPtr` is the
/// width of a pointer, and `Float32`/`Float64` are the floats. There is no
/// `UInt64`: AutoHotkey does not carry an unsigned integer of eight bytes.
pub(crate) fn numeric(name: &str) -> Option<NativeType> {
    match name.to_ascii_lowercase().as_str() {
        "int8" => Some(NativeType::Char),
        "uint8" => Some(NativeType::UChar),
        "int16" => Some(NativeType::Short),
        "uint16" => Some(NativeType::UShort),
        "int32" => Some(NativeType::Int),
        "uint32" => Some(NativeType::UInt),
        "int64" => Some(NativeType::Int64),
        "intptr" => Some(NativeType::Ptr),
        "float32" => Some(NativeType::Float),
        "float64" => Some(NativeType::Double),
        _ => None,
    }
}

/// Whether the name is a struct class the dialect carries on its own:
/// `Struct` the base, or one of the numeric types.
pub(crate) fn is_a_struct_class_name(name: &str) -> bool {
    name.eq_ignore_ascii_case("struct") || numeric(name).is_some()
}

/// The type of a struct field: a numeric type, or another struct.
#[derive(Clone, Debug)]
pub(crate) enum FieldType {
    Numeric(NativeType),
    Struct(String),
}

/// A field laid out in the memory of a struct: its name, its offset from
/// the start of the struct, and the type of its value.
#[derive(Clone, Debug)]
pub(crate) struct FieldLayout {
    pub name: String,
    pub offset: usize,
    pub field_type: FieldType,
}

/// The layout of a struct: the fields in order, the whole size, and the
/// alignment (the largest alignment of its fields).
#[derive(Clone, Debug)]
pub(crate) struct Layout {
    pub fields: Vec<FieldLayout>,
    pub size: usize,
    pub align: usize,
}

/// The alignment of a field of the given size: its own width, capped at
/// eight bytes (the default pack).
pub(crate) fn alignment_of(size: usize) -> usize {
    size.clamp(1, 8)
}

/// Round an offset up to a multiple of the alignment.
pub(crate) fn align_up(offset: usize, align: usize) -> usize {
    offset.div_ceil(align) * align
}
