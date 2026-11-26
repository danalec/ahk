//! The `Empty` member of the `VarRef` class, of the KeySharp dialect.
//!
//! `VarRef.Empty` is the reference that points at no variable: the read
//! of it is nothing and the write into it goes nowhere. KeySharp makes
//! it with a getter of nothing and a setter that drops the write, and
//! hands it to the built-ins whose output parameter was left out. The
//! tree makes it a reference with the empty name, and the three places
//! that read, write and unset a reference know the empty name.

use alloc::string::String;

use crate::runtime::{Reference, Runtime, RuntimeFailure, Scope, Value};

/// A member of the `VarRef` class, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    if !_class.eq_ignore_ascii_case("varref") {
        return None;
    }
    match name.to_lowercase().as_str() {
        "empty" => Some(Ok(Value::Reference(Reference {
            name: String::new(),
            scope: Scope::Global,
            alias: true,
        }))),
        _ => None,
    }
}
