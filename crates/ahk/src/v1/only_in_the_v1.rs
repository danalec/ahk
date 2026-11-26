//! The names that the v1 has and the v2 does not: the `ComObjMissing`, the
//! `ComObjParameter`, the `ObjRawGet`, the `OnClipboardChange`, the
//! `SB_SetText` and the `SB_SetParts`.
//!
//! Measured on the binary of v1.1.37.02: the six are functions of the v1,
//! and the table of the surface of the v1 did not carry them. Their arity
//! was measured with the probe of the surface: the `ComObjMissing` and the
//! `ComObjParameter` take up to three parameters, the `ObjRawGet` takes
//! two, the `OnClipboardChange` takes one or two, the `SB_SetText` takes
//! one to three and the `SB_SetParts` takes any number of them.
//!
//! The tree does not carry the variant of the COM that marks an argument
//! that is missing, and the value of the `ComObjMissing` and of the
//! `ComObjParameter` is the empty text -- which is what the v1 answers
//! when the value is printed.
//!
//! The tree does not watch the clipboard, and the `OnClipboardChange` says
//! so in a note. The tree does not carry the status bar either, and the
//! `SB_SetText` and the `SB_SetParts` say so in a note too.

use alloc::format;
use alloc::string::String;

use crate::runtime::{Runtime, RuntimeError, Value};

/// The text of a parameter, the way the functions of the v1 read it.
fn text_of(values: &[Value], at: usize) -> String {
    values
        .get(at)
        .map(|value| value.to_text())
        .unwrap_or_default()
}

/// `ObjRawGet(Object, Key)`: the value of a property of an object, without
/// the getter of it.
///
/// Measured on the binary of v1.1.37.02: the value of the property is the
/// answer, and the property that is not there answers the empty text.
pub(crate) fn obj_raw_get(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let object = values
        .first()
        .cloned()
        .unwrap_or(Value::Text("".into()));
    let Value::Object(obj) = &object else {
        return Err(RuntimeError {
            message: crate::runtime::requires(
                1,
                "ObjRawGet",
                crate::runtime::TypeName::Object,
                &object,
            ),
            line,
        });
    };
    let key = text_of(values, 1);
    Ok(obj.borrow().value_of(&key).unwrap_or(Value::Text("".into())))
}

/// `ComObj(CLSID, IID)`: the COM object of the call of the v1.
///
/// Measured on the binary of v1.1.37.02: the call of the v1 takes up to
/// three parameters. The tree does not carry the call of the COM of the v1,
/// and the value of it is the empty text.
pub(crate) fn com_obj(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    rt.note(format!(
        "the call of the COM of the v1 is a thing of Windows, and this runtime does not carry it (line {line})"
    ));
    Ok(Value::Text("".into()))
}

/// `ComObjMissing()`: the value that marks an argument that is not there.
///
/// Measured on the binary of v1.1.37.02: the value of it is the empty text
/// when it is printed.
pub(crate) fn com_obj_missing(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text("".into()))
}

/// `ComObjParameter(Type, Value)`: the parameter of a call of a COM.
///
/// Measured on the binary of v1.1.37.02: the value of it is the empty text
/// when it is printed. The tree does not carry the parameter that holds the
/// reference of a variable, and the value of it is the empty text.
pub(crate) fn com_obj_parameter(
    _rt: &mut Runtime<'_>,
    _values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(Value::Text("".into()))
}

/// `SB_SetText(Text, PartNumber, uType)`: the text of a part of a status bar.
///
/// Measured on the binary of v1.1.37.02: the text of the status bar is the
/// answer, and the tree does not carry the status bar: the call says so
/// in a note.
pub(crate) fn sb_set_text(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    rt.note(format!(
        "the status bar is a thing of Windows, and this runtime does not carry it (line {line})"
    ));
    Ok(Value::Text("".into()))
}

/// `SB_SetParts(PartWidth1, ...)`: the widths of the parts of a status bar.
///
/// Measured on the binary of v1.1.37.02: the widths of the parts are the
/// answer, and the tree does not carry the status bar: the call says so in
/// a note.
pub(crate) fn sb_set_parts(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    rt.note(format!(
        "the status bar is a thing of Windows, and this runtime does not carry it (line {line})"
    ));
    Ok(Value::Text("".into()))
}
