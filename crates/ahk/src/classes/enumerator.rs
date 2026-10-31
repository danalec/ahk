//! The `Enumerator` class.
//!
//! An enumerator hands out the pieces of a sequence, one turn at a time.
//! v2 makes one out of the `__Enum` of a value and out of the `OwnProps`
//! of an object, and the class of it is `Enumerator`: it is a function of
//! its own kind, and the `Call` of it is the turn of the enumeration.
//!
//! The enumerator gives the key and the value under it when the caller
//! gives it two variables, and the one of the two that it gives alone when
//! the caller gives it one.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};

/// The key the state of an enumerator lives under.
///
/// The name is not one a script can write, and it is not a property of the
/// enumerator either, so it stays out of the answers about the
/// properties.
const STATE: &str = "enumerator state";

/// The class of the object that carries the state of an enumerator.
const STATE_CLASS: &str = "enumerator-state";

/// The properties an enumerator owns: the ones of a function.
const PROPERTIES: [&str; 5] = ["name", "minparams", "maxparams", "isbuiltin", "isvariadic"];

/// The methods an enumerator has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 13] = [
    "call", "isoptional", "isbyref", "bind",
    "ownprops", "hasownprop", "defineprop", "deleteprop", "getownpropdesc",
    "hasprop", "hasmethod", "getmethod", "hasbase",
];

/// Which of the two of a pair is given when the caller wants one value.
///
/// Measured on the binary of v2.0.19 on 2026-10-31: the enumerator of an
/// array gives the value, and the enumerator of a map gives the key.
#[derive(Clone, Copy)]
pub(crate) enum Alone {
    /// The key of the pair: the place of an array, the key of a map, the
    /// name of a property.
    TheKey,
    /// The value under the key.
    TheValue,
}

/// The names an enumerator answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// Make an enumerator over the pairs.
///
/// The pairs are the key and the value under it. The enumerator gives the
/// key when the caller gives it two variables, and the one of the two that
/// `alone` names when the caller gives it one.
pub(crate) fn make(pairs: Vec<(Value, Value)>, alone: Alone) -> Value {
    let mut state = Object { class: STATE_CLASS.to_string(), properties: BTreeMap::new() };
    set_pairs(&mut state, pairs);
    put(
        &mut state.properties,
        "alone",
        Value::Int(match alone {
            Alone::TheKey => 0,
            Alone::TheValue => 1,
        }),
    );
    let mut table = BTreeMap::new();
    put(&mut table, STATE, Value::Object(Rc::new(RefCell::new(state))));
    Value::Object(Rc::new(RefCell::new(Object {
        class: "enumerator".to_string(),
        properties: table,
    })))
}

/// Whether a value is an enumerator.
pub(crate) fn is_one(value: &Value) -> bool {
    let Value::Object(object) = value else { return false };
    object.borrow().class == "enumerator"
}

/// How many turns the enumerator has left.
pub(crate) fn the_count(this: &Value) -> i64 {
    let Some(state) = state(this) else { return 0 };
    let state = state.borrow();
    pairs(&state).len() as i64
}

/// The turn of the enumeration, in the shape the caller wants.
///
/// The caller that wants two values gets the key and the value under it,
/// and the caller that wants one gets the one of the two that the
/// enumerator gives alone. `None` means that there is nothing left.
pub(crate) fn the_turn(this: &Value, wanted: usize) -> Option<Value> {
    let (key, value, alone) = the_pair(this)?;
    if wanted >= 2 {
        return Some(Value::List(vec![key, value]));
    }
    Some(match alone {
        Alone::TheKey => key,
        Alone::TheValue => value,
    })
}

/// A method of an `Enumerator`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match method {
        "call" => Some(the_call(this, arguments, rt, line)),
        // The enumerator has no properties of its own: the state of it is
        // not a property, and v2 says so.
        "ownprops" => Some(Ok(make(Vec::new(), Alone::TheKey))),
        "hasownprop" => {
            let name = arguments.first().map(|value| value.to_text()).unwrap_or_default();
            let key = name.to_lowercase();
            let answer = match this {
                Value::Object(object) => key != STATE && object.borrow().properties.contains_key(&key),
                _ => false,
            };
            Some(Ok(Value::Int(answer as i64)))
        }
        // The values of the class that the function carries.
        "isoptional" => Some(Ok(Value::Int(0))),
        "isbyref" => Some(Ok(Value::Int(0))),
        "bind" => Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "this runtime does not bind a function to an object.".into(),
            line,
        }))),
        _ => None,
    }
}

/// The call of the value of an enumerator.
///
/// The value is called, and not the `Call` method of it: the turn of the
/// enumeration is the call.
pub(crate) fn call_the_value(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    call("call", this, arguments, rt, line)
}

/// A property of an `Enumerator`.
///
/// The enumerator is a function, and it carries the values of a function:
/// the name of it is empty, it wants no parameters, and it is built in.
pub(crate) fn property(
    name: &str,
    _this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(Ok(match name {
        "name" => Value::Text(String::new()),
        "minparams" => Value::Int(0),
        "maxparams" => Value::Int(0),
        "isbuiltin" => Value::Int(1),
        "isvariadic" => Value::Int(0),
        _ => return None,
    }))
}

/// The turn of the enumeration, written into the variables of the caller.
///
/// The first variable gets the key and the second the value. A caller that
/// gives one variable gets the one of the two that the enumerator gives
/// alone, and a variable that is not a reference is left alone: v2 does
/// not fail there.
///
/// Measured on the binary of v2.0.19 on 2026-11-01: a turn that has no
/// value of its own leaves the variable of the caller without one: the
/// script that reads it is told that the variable has no value.
fn the_call(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let Some((key, value, alone)) = the_pair(this) else { return Ok(Value::Int(0)) };
    let alone_value = match alone {
        Alone::TheKey => key.clone(),
        Alone::TheValue => value.clone(),
    };
    if arguments.len() >= 2 {
        rt.write_reference(&arguments[0], key);
        if crate::runtime::is_a_value_without_one(&value) {
            rt.unset_reference(&arguments[1]);
        } else {
            rt.write_reference(&arguments[1], value);
        }
    } else if let Some(argument) = arguments.first() {
        if crate::runtime::is_a_value_without_one(&alone_value) {
            rt.unset_reference(argument);
        } else {
            rt.write_reference(argument, alone_value);
        }
    }
    Ok(Value::Int(1))
}

// -- the state ----------------------------------------------------------------

/// The state of an enumerator.
fn state(enumerator: &Value) -> Option<Rc<RefCell<Object>>> {
    let Value::Object(object) = enumerator else { return None };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// The pairs of an enumerator: the key and the value under it.
fn pairs(state: &Object) -> Vec<(Value, Value)> {
    let Some(Value::List(items)) = state.value_of("pairs") else { return Vec::new() };
    items
        .iter()
        .map(|item| match item {
            Value::List(pair) if pair.len() == 2 => (pair[0].clone(), pair[1].clone()),
            _ => (item.clone(), Value::Text(String::new())),
        })
        .collect()
}

/// The pairs of an enumerator, written back.
fn set_pairs(state: &mut Object, pairs: Vec<(Value, Value)>) {
    let items: Vec<Value> = pairs
        .into_iter()
        .map(|(key, value)| Value::List(vec![key, value]))
        .collect();
    put(&mut state.properties, "pairs", Value::List(items));
}

/// The pair of the turn, and which of the two is given alone.
fn the_pair(this: &Value) -> Option<(Value, Value, Alone)> {
    let state = state(this)?;
    let mut state = state.borrow_mut();
    let mut pairs = pairs(&state);
    if pairs.is_empty() {
        return None;
    }
    let (key, value) = pairs.remove(0);
    set_pairs(&mut state, pairs);
    let alone = match state.value_of("alone").and_then(|value| value.to_number()) {
        Some(number) if number >= 1.0 => Alone::TheValue,
        _ => Alone::TheKey,
    };
    Some((key, value, alone))
}
