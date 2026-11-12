//! The `Timer` class of the KeySharp dialect.
//!
//! KeySharp makes the timers of the script objects: `A_Timers` is the array
//! of them, and each one says which function it runs, how many milliseconds
//! pass between two runs, and when it ran the last time.
//!
//! The object is made by the runtime, which holds the timers of the script;
//! a script does not make one.

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeFailure, Value};

/// The key the name of the function is under.
const THE_FUNCTION: &str = "\u{1}timer function";
/// The key the period is under.
const THE_PERIOD: &str = "\u{1}timer period";
/// The key of the last run is under.
const THE_LAST: &str = "\u{1}timer last";

/// The names of the members of a `Timer`.
pub(crate) fn names() -> Vec<&'static str> {
    vec!["function", "period", "last"]
}

/// The object of a timer: the function it runs, the period and the last
/// run.
pub(crate) fn the_timer(function: &str, period: u64, last: u64) -> Value {
    let Value::Object(object) = crate::classes::object("timer") else {
        return Value::Text(String::new());
    };
    object
        .borrow_mut()
        .put(THE_FUNCTION, Value::Text(function.to_string()));
    object
        .borrow_mut()
        .put(THE_PERIOD, Value::Int(period as i64));
    object.borrow_mut().put(THE_LAST, Value::Int(last as i64));
    Value::Object(object)
}

/// A member of a `Timer`, as a property read.
pub(crate) fn property(
    _class: &str,
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The chain of the extension classes asks every module, and a
    // module that answers the members of another class takes them
    // with the unset value: the guard keeps the class its own.
    if !_class.eq_ignore_ascii_case("timer") {
        return None;
    }
    let Value::Object(object) = this else {
        return None;
    };
    let answer = match name.to_lowercase().as_str() {
        // The function the timer runs.
        "function" => object
            .borrow()
            .value_of(THE_FUNCTION)
            .unwrap_or(Value::Unset),
        // The milliseconds between two runs.
        "period" => object.borrow().value_of(THE_PERIOD).unwrap_or(Value::Unset),
        // When it ran the last time.
        "last" => object.borrow().value_of(THE_LAST).unwrap_or(Value::Unset),
        _ => return None,
    };
    Some(Ok(answer))
}
