//! The `Array` class.
//!
//! An array is a row of values. The places count from one, the way
//! AutoHotkey counts them, and a place can be left without a value of
//! its own.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::enumerator;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};
use crate::runtime::SharedCell;

/// The key the state of an array lives under.
///
/// The name is not one a script can write: the name of a property in a
/// script is always an identifier, and this one is not. It is not a
/// property of the array either, so it stays out of the answers about
/// the properties.
const STATE: &str = "array state";

/// The class of the object that carries the state of an array.
const STATE_CLASS: &str = "array-state";

/// The prefix of the name of a keyed item of the state.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the array of the v1 is
/// a table of keys, and the item of a key that is not there is not a hole:
/// the `a[12345] := "x"` is the item of the key 12345, and the array does
/// not grow to hold the places that are not there. The tree kept the row of
/// the places and grew it with holes, and the write of a key that is the
/// handle of a window cost the time of the handle.
///
/// The name of a keyed item is the key in a fixed width: the table of the
/// names is sorted as texts, and the texts of the keys would put "10" before
/// "5" otherwise.
const KEY: &str = "key:";

/// The properties an array owns.
const PROPERTIES: [&str; 3] = ["length", "capacity", "__item"];

/// The methods an array has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 19] = [
    "push",
    "pop",
    "insertat",
    "removeat",
    "delete",
    "get",
    "has",
    "clone",
    "__new",
    "__enum",
    "ownprops",
    "hasownprop",
    "defineprop",
    "deleteprop",
    "getownpropdesc",
    "hasprop",
    "hasmethod",
    "getmethod",
    "hasbase",
];

/// The names an array answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// The object an `Array` makes.
///
/// The values of the constructor go in, the way `Push` puts them in.
pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let array = crate::classes::object_with("array", vec![(STATE, new_state(Vec::new(), 0))]);
    for value in arguments {
        push_value(&array, value.clone());
    }
    Ok(array)
}

/// The object the literal of an array makes.
///
/// Measured on the binary of v2.0.19: the literal may have a
/// hole, and the place of the hole is a place of the array that has no
/// value of its own.
pub(crate) fn construct_with_holes(
    values: &[Option<Value>],
    _rt: &mut Runtime,
    _line: u32,
) -> Value {
    let array = crate::classes::object_with("array", vec![(STATE, new_state(Vec::new(), 0))]);
    for value in values {
        push_element(&array, value.clone());
    }
    array
}

/// The array of the items: for the values that cross a thread and come
/// back as an array of the script.
pub(crate) fn of_items(items: Vec<Value>) -> Value {
    let array = crate::classes::object_with("array", vec![(STATE, new_state(Vec::new(), 0))]);
    for value in items {
        push_value(&array, value);
    }
    array
}

/// A method of an `Array`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let method = method.to_lowercase();
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError {
            message: message.to_string(),
            line,
        })))
    };
    // The failure of a member that complains about a value carries the
    // value in the `Extra` of the error.
    let failure_with = |message: &str, extra: &str| {
        Some(Err(crate::classes::the_failure_with(message, extra, line)))
    };
    // The place of an array is a parameter of the methods that take one,
    // and a place that is not there is an invalid parameter.
    let invalid_place = |method: &str, index: i64| {
        failure_with(
            &format!(
                "Parameter #1 of {} is invalid.",
                crate::classes::the_name_of("array", method)
            ),
            &index.to_string(),
        )
    };
    // The members that the KeySharp extension adds: none of their names is
    // a method of the v2, so they are asked before the methods of the v2
    // and the dialect of the v2.0.19 never reaches them.
    if rt
        .dialect()
        .carries_the_member_of_the_extension("array", &method)
    {
        return the_extension_answer(&method, this, arguments, rt, line);
    }

    match method.as_str() {
        // `Push` and `__New` are the same thing in v2: both put the values
        // at the end, and both answer the array, so that the calls can be
        // chained.
        "push" | "__new" => Some(Ok(push_fast(this, arguments))),
        "pop" => {
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Array is empty.");
            };
            let Some(last) = items.pop() else {
                return failure("Array is empty.");
            };
            Some(Ok(
                element_value(&last).unwrap_or(Value::Text("".into()))
            ))
        }
        "insert" if rt.dialect().the_array_has_the_methods_of_the_v1() => {
            // Measured on the binary of AutoHotkey v1.1.37.02:
            // the `Insert` of the v1 takes the index and the value: the
            // `a.Insert(1, 9)` puts the nine at the first place. A value on
            // its own goes to the end: the `a.Insert(9)` puts the nine at
            // the end of the array.
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid index.");
            };
            let length = items.len() as i64;
            if arguments.len() < 2 {
                if let Some(value) = arguments.first() {
                    items.push(element(Some(value.clone())));
                }
                return Some(Ok(Value::Text("".into())));
            }
            let index = match rt.integer(&arguments[0], line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(place) = insert_place(index, length) else {
                return invalid_place("Insert", index);
            };
            for (offset, value) in arguments.iter().skip(1).enumerate() {
                items.insert(place + offset, element(Some(value.clone())));
            }
            Some(Ok(Value::Text("".into())))
        }
        "remove" if rt.dialect().the_array_has_the_methods_of_the_v1() => {
            // Measured on the binary of AutoHotkey v1.1.37.02:
            // the `Remove` of the v1 takes the index: the `a.Remove(1)`
            // takes the first one away. Without an index the last one goes
            // away: the `a.Remove()` is the `a.Pop()`.
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid index.");
            };
            let length = items.len() as i64;
            let Some(index) = arguments.first() else {
                let Some(last) = items.pop() else {
                    return failure("Array is empty.");
                };
                return Some(Ok(
                    element_value(&last).unwrap_or(Value::Text("".into()))
                ));
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(place) = place(index, length) else {
                return invalid_place("Remove", index);
            };
            let removed = items.remove(place);
            Some(Ok(
                element_value(&removed).unwrap_or(Value::Text("".into()))
            ))
        }
        "insertat" => {
            let Some(index) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            // There is something to insert: an index on its own inserts
            // nothing.
            if arguments.len() < 2 {
                return failure("Too few parameters passed to function.");
            }
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid index.");
            };
            let length = items.len() as i64;
            // A zero is the end of the array, and a negative index counts
            // from the end.
            let Some(place) = insert_place(index, length) else {
                return invalid_place("InsertAt", index);
            };
            for (offset, value) in arguments.iter().skip(1).enumerate() {
                items.insert(place + offset, element(Some(value.clone())));
            }
            Some(Ok(Value::Text("".into())))
        }
        "removeat" => {
            let Some(index) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid index.");
            };
            let length = items.len() as i64;
            let Some(place) = place(index, length) else {
                return invalid_place("RemoveAt", index);
            };
            match arguments.get(1) {
                // One place: the value that was there is the answer.
                None => {
                    let removed = items.remove(place);
                    Some(Ok(
                        element_value(&removed).unwrap_or(Value::Text("".into()))
                    ))
                }
                // A range: the answer is the number of the values that were
                // in it.
                Some(count) => {
                    let count = match rt.integer(count, line) {
                        Ok(number) => number,
                        Err(failure) => return Some(Err(failure)),
                    };
                    let mut removed = 0;
                    for _ in 0..count.max(1) {
                        if place >= items.len() {
                            break;
                        }
                        if element_value(&items[place]).is_some() {
                            removed += 1;
                        }
                        items.remove(place);
                    }
                    Some(Ok(Value::Int(removed)))
                }
            }
        }
        "delete" => {
            let Some(index) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid index.");
            };
            let length = items.len() as i64;
            let Some(place) = place(index, length) else {
                return invalid_place("Delete", index);
            };
            // The place stays, and it has no value of its own any more.
            let removed = element_value(&items[place]).unwrap_or(Value::Text("".into()));
            items[place] = element(None);
            Some(Ok(removed))
        }
        "get" => {
            let Some(index) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else {
                return failure("Invalid index.");
            };
            let state = state.borrow();
            let Some(items) = the_items_of(&state) else {
                return failure_with("Invalid index.", &index.to_string());
            };
            let length = items.len() as i64;
            let Some(place) = place(index, length) else {
                return failure_with("Invalid index.", &index.to_string());
            };
            match element_value(&items[place]) {
                Some(value) => Some(Ok(value)),
                // The default of the call comes first, and then the default
                // the script gave to the array.
                None => match arguments.get(1) {
                    Some(default) => Some(Ok(default.clone())),
                    None => match default_property(this) {
                        Some(default) => Some(Ok(default)),
                        None => failure("Item has no value."),
                    },
                },
            }
        }
        "has" => {
            let Some(index) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else {
                return Some(Ok(Value::Int(0)));
            };
            let state = state.borrow();
            // The item under a key is there when the key is there.
            if rt.dialect().the_array_grows_with_the_write_of_an_item()
                && the_keyed_item_of(&state, index).is_some()
            {
                return Some(Ok(Value::Int(1)));
            }
            let Some(items) = the_items_of(&state) else {
                return Some(Ok(Value::Int(0)));
            };
            let length = items.len() as i64;
            // A place that is not there has no value, and that is not an
            // error: the question is answered.
            let found = match place(index, length) {
                Some(place) => element_value(&items[place]).is_some(),
                None => false,
            };
            Some(Ok(Value::Int(found as i64)))
        }
        "clone" => Some(Ok(clone(this))),
        // The enumeration of an array is the values, in the order of their
        // places. Measured on the binary of v2.0.19: the
        // enumeration of v2 is an enumerator, and it gives the value when
        // the caller wants one value and the place and the value when the
        // caller wants two. The number of the variables is not carried by
        // the enumerator: it is the call that says how many it wants.
        "__enum" => {
            let Some(state) = state(this) else {
                return Some(Ok(enumerator::make(
                    Vec::new(),
                    enumerator::Alone::TheValue,
                )));
            };
            if rt.dialect().the_array_grows_with_the_write_of_an_item() {
                let state = state.borrow();
                // Measured on the binary of AutoHotkey v1.1.37.02: the
                // enumeration of the v1 walks the items that are there, and
                // the keys come in the order of their indexes: the place
                // that has no value of its own is not a turn of the
                // enumeration.
                let pairs: Vec<(Value, Value)> = the_items_with_a_value(&state)
                    .into_iter()
                    .map(|(key, value)| (Value::Int(key), value))
                    .collect();
                return Some(Ok(enumerator::make(
                    pairs,
                    enumerator::Alone::TheValue,
                )));
            }
            // The enumeration is live: the items that the body of the loop
            // pushes are seen by it, the way the v2 sees them.
            //
            // Measured on the binary of v2.0.32: the `for` over an array
            // sees the item that the body of the loop pushes to it.
            Some(Ok(enumerator::make_of_an_array(
                state,
                enumerator::Alone::TheValue,
            )))
        }
        _ => base_method(&method, this, arguments, rt, line),
    }
}

/// Write an item of an array: `array[index] := value`.
///
/// The index must be an item that is there: an array grows with `Push`
/// and with `InsertAt`, and not with an assignment.
pub(crate) fn set_item(
    key: &Value,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    // The place of an item is a number: a value that is not one is not a
    // place at all, and v2 says that the index is invalid.
    let Some(index) = rt.the_place_of_an_item(key) else {
        // Measured on the binary of AutoHotkey v1.1.37.02: the
        // v1 takes the item of an array with a key that is not a number for
        // the property of that name: the `a["b"] := 9` writes the property
        // "b" of the array.
        if rt
            .dialect()
            .the_item_of_an_array_with_a_key_that_is_not_a_number_is_its_property()
        {
            let name = key.to_text();
            return Some(
                rt.write_property_of(this, &name, value, line)
                    .map_err(RuntimeFailure::Error),
            );
        }
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &crate::runtime::the_extra_of_the_value(key),
            line,
        )));
    };
    let Some(state) = state(this) else {
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    let mut state = state.borrow_mut();
    // Measured on the binary of AutoHotkey v1.1.37.02: the array of the v1
    // is a table of keys, and the write of a key that is not there does not
    // leave a hole: the item goes under its key, and the array grows to
    // the highest key it has. The tree grew the row of the places with
    // holes, and the write of a key that is the handle of a window cost
    // the time of the handle.
    if rt.dialect().the_array_grows_with_the_write_of_an_item() {
        let length = the_items_of(&state).map_or(0, |items| items.len() as i64);
        if index > length {
            set_the_keyed_item(&mut state, index, value);
            return Some(Ok(()));
        }
    }
    let Some(items) = the_items_mut(&mut state) else {
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    let length = items.len() as i64;
    let place = if rt.dialect().the_array_grows_with_the_write_of_an_item() {
        the_place_of_the_v1(index, length)
    } else {
        place(index, length)
    };
    let Some(place) = place else {
        // Measured on the binary of AutoHotkey v1.1.37.02: the
        // v1 makes the array grow: the `a[5] := 7` of an array of two is the
        // seven, and the places that are not there are holes.
        if rt.dialect().the_array_grows_with_the_write_of_an_item() {
            if index > length {
                while (items.len() as i64) < index - 1 {
                    items.push(element(None));
                }
                items.push(element(Some(value)));
                return Some(Ok(()));
            }
            // The index that is not positive is not a place of the array: the
            // `a[0] := 7` writes the property "0" of the array, and the
            // `a[-1] := 7` writes the property "-1".
            let name = key.to_text();
            return Some(
                rt.write_property_of(this, &name, value, line)
                    .map_err(RuntimeFailure::Error),
            );
        }
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    items[place] = element(Some(value));
    Some(Ok(()))
}

/// A property of an `Array`.
pub(crate) fn property(
    name: &str,
    this: &Value,
    rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let state = state(this)?;
    // Measured on the binary of AutoHotkey v1.1.37.02: the
    // array of the v1 has neither the length nor the capacity: the
    // `a.Length` is the empty text. The v1 counts the array with the
    // `a.Length()` and the `a.Count()`.
    if !rt.dialect().the_array_has_the_length_and_the_capacity() {
        return None;
    }
    let state = state.borrow();
    match name {
        "length" => Some(Ok(Value::Int(
            the_items_of(&state).map_or(0, |items| items.len() as i64),
        ))),
        "capacity" => Some(Ok(Value::Int(capacity(&state)))),
        _ => None,
    }
}

/// A property of an `Array` that is written.
pub(crate) fn set_property(
    name: &str,
    this: &Value,
    value: Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError {
            message: message.to_string(),
            line,
        })))
    };
    match name {
        "length" => {
            let wanted = match rt.integer(&value, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            if wanted < 0 {
                return failure("Invalid value.");
            }
            let state = state(this)?;
            let mut state = state.borrow_mut();
            let Some(items) = the_items_mut(&mut state) else {
                return failure("Invalid value.");
            };
            // A longer array has places without a value of their own, and
            // a shorter one loses the places at the end.
            items.truncate(wanted as usize);
            while (items.len() as i64) < wanted {
                items.push(element(None));
            }
            Some(Ok(()))
        }
        "capacity" => {
            let wanted = match rt.integer(&value, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            if wanted < 0 {
                return failure("Invalid value.");
            }
            let state = state(this)?;
            let mut state = state.borrow_mut();
            let count = {
                let Some(items) = the_items_mut(&mut state) else {
                    return failure("Invalid value.");
                };
                // A capacity below the length of the array takes the places at
                // the end away.
                if (wanted as usize) < items.len() {
                    items.truncate(wanted as usize);
                }
                items.len() as i64
            };
            set_capacity(&mut state, wanted.max(count));
            Some(Ok(()))
        }
        _ => None,
    }
}

/// The item of an `Array`: the `__Item` of the class.
///
/// A place without a value of its own is an error, unless the script
/// gave the array a default.
pub(crate) fn item(
    key: &Value,
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    // The place of an item is a number: a value that is not one is not a
    // place at all, and v2 says that the index is invalid.
    let Some(index) = rt.the_place_of_an_item(key) else {
        // Measured on the binary of AutoHotkey v1.1.37.02: the
        // v1 takes the item of an array with a key that is not a number for
        // the property of that name: the `a["b"]` reads the property "b".
        if rt
            .dialect()
            .the_item_of_an_array_with_a_key_that_is_not_a_number_is_its_property()
        {
            let name = key.to_text();
            return Some(rt.read_property(this, &name, line));
        }
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &crate::runtime::the_extra_of_the_value(key),
            line,
        )));
    };
    let Some(state) = state(this) else {
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    let state = state.borrow();
    // The item under a key comes first: the write of a key of the v1 goes
    // under the key, and the row of the places is only the row of the
    // places that are there.
    if rt.dialect().the_array_grows_with_the_write_of_an_item() {
        if let Some(value) = the_keyed_item_of(&state, index) {
            return Some(Ok(value));
        }
    }
    let Some(items) = the_items_of(&state) else {
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    let length = items.len() as i64;
    let place = if rt
        .dialect()
        .the_item_of_an_array_that_is_not_there_is_the_empty_text()
    {
        the_place_of_the_v1(index, length)
    } else {
        place(index, length)
    };
    let Some(place) = place else {
        // Measured on the binary of AutoHotkey v1.1.37.02: the
        // v1 does not refuse the item of an array that is not there: the
        // `a[5]` of an array of two is the empty text, and the `a[0]` is the
        // property "0" of the array.
        if rt
            .dialect()
            .the_item_of_an_array_that_is_not_there_is_the_empty_text()
        {
            if index < 1 {
                let name = key.to_text();
                return Some(rt.read_property(this, &name, line));
            }
            return Some(Ok(Value::Text("".into())));
        }
        return Some(Err(crate::classes::the_failure_with(
            "Invalid index.",
            &index.to_string(),
            line,
        )));
    };
    match element_value(&items[place]) {
        Some(value) => Some(Ok(value)),
        None => match default_property(this) {
            Some(default) => Some(Ok(default)),
            None => {
                // Measured on the binary of v1.1.37.02: the
                // v1 has no place without a value: the place that the
                // literal leaves empty is the empty text, and the write
                // of it fills the place.
                if rt
                    .dialect()
                    .the_place_without_a_value_of_an_array_is_the_empty_text()
                {
                    return Some(Ok(Value::Text("".into())));
                }
                Some(Err(RuntimeFailure::Error(RuntimeError {
                    message: "Item has no value.".into(),
                    line,
                })))
            }
        },
    }
}

// -- the state ----------------------------------------------------------------

/// The state of an array.
fn state(array: &Value) -> Option<SharedCell<Object>> {
    let Value::Object(object) = array else {
        return None;
    };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// A new state: the items in the order of their places, and how many of
/// them fit.
fn new_state(items: Vec<Value>, capacity: i64) -> Value {
    let mut state = Object {
        class: STATE_CLASS.to_string(),
        properties: BTreeMap::new(),
    };
    set_items(&mut state, items);
    set_capacity(&mut state, capacity);
    Value::Object(SharedCell::new(state))
}

/// The items of an array: the elements in the order of their places.
///
/// The copy of the row of the places is a copy of every place of it, and
/// the owner of the row asks for it without a copy when it writes one place
/// of it: the write of a place of an array of many places is a write of
/// that place and not a copy of all of them.
fn items(state: &Object) -> Vec<Value> {
    the_items_of(state).cloned().unwrap_or_default()
}

/// The item of the place of an array, the key of it, and the place of the
/// next one.
///
/// The items of an array are walked in the order of their places, and a
/// place that has no item of its own is a turn of the walk all the same:
/// the value of it is the mark of a value without one.
pub(crate) fn the_item_at(
    cell: &SharedCell<Object>,
    place: usize,
) -> Option<(Value, Value, usize)> {
    let state = cell.borrow();
    let items = the_items_of(&state)?;
    let item = items.get(place)?;
    let value =
        element_value(item).unwrap_or_else(crate::runtime::the_mark_of_a_value_without_one);
    Some((Value::Int(place as i64 + 1), value, place + 1))
}

/// The number of the items of an array that are there from the place.
pub(crate) fn the_count_of_the_items(cell: &SharedCell<Object>, place: usize) -> usize {
    let state = cell.borrow();
    let Some(items) = the_items_of(&state) else {
        return 0;
    };
    items.len().saturating_sub(place)
}

/// The items of an array, without a copy of them.
fn the_items_of(state: &Object) -> Option<&Vec<Value>> {
    match state.value_ref("items") {
        Some(Value::List(items)) => Some(items),
        _ => None,
    }
}

/// The items of an array, for the writing.
fn the_items_mut(state: &mut Object) -> Option<&mut Vec<Value>> {
    match state.value_mut("items") {
        Some(Value::List(items)) => Some(items),
        _ => None,
    }
}

/// The name of a keyed item: the key in a fixed width.
fn the_name_of_a_key(key: i64) -> String {
    format!("{}{:020}", KEY, key)
}

/// The key of a keyed item, from the name of it.
fn the_key_of_a_name(name: &str) -> Option<i64> {
    name.strip_prefix(KEY)?.parse::<i64>().ok()
}

/// The item under a key, when the array has it.
///
/// The item is a copy of the value: the owner of the item writes it
/// without a copy when it writes the place of it.
fn the_keyed_item_of(state: &Object, key: i64) -> Option<Value> {
    state
        .properties
        .get(&the_name_of_a_key(key))
        .map(|property| property.value.clone())
}

/// Put an item under a key.
fn set_the_keyed_item(state: &mut Object, key: i64, value: Value) {
    put(&mut state.properties, &the_name_of_a_key(key), value);
}

/// Whether the state holds any keyed item.
fn the_state_has_a_keyed_item(state: &Object) -> bool {
    state
        .properties
        .keys()
        .any(|name| the_key_of_a_name(name).is_some())
}

/// The keyed items of the state, with their keys.
fn the_keyed_items_of(state: &Object) -> Vec<(i64, Value)> {
    state
        .properties
        .iter()
        .filter_map(|(name, property)| {
            the_key_of_a_name(name).map(|key| (key, property.value.clone()))
        })
        .collect()
}

/// The highest key of the state, when it has one.
fn the_highest_key_of(state: &Object) -> Option<i64> {
    state
        .properties
        .keys()
        .filter_map(|name| the_key_of_a_name(name))
        .max()
}

/// The items of the array that have a value, with their keys.
///
/// The keyed items come after the row of the places, and the key of an
/// item that is in both is the one of the keyed table: the write of a key
/// is the last one that was made.
fn the_items_with_a_value(state: &Object) -> Vec<(i64, Value)> {
    let mut items: BTreeMap<i64, Value> = BTreeMap::new();
    if let Some(row) = the_items_of(state) {
        for (place, item) in row.iter().enumerate() {
            if let Some(value) = element_value(item) {
                items.insert(place as i64 + 1, value);
            }
        }
    }
    for (key, value) in the_keyed_items_of(state) {
        items.insert(key, value);
    }
    items.into_iter().collect()
}

/// The items of an array, for the built-ins that take one.
///
/// The `StrSplit` takes the delimiters in an array, and the `Sort` takes
/// the values of one. Neither of them may reach into the state of the
/// array: the module that owns the class is the one that answers.
pub(crate) fn the_items(array: &Value) -> Vec<Value> {
    let Some(state) = state(array) else {
        return Vec::new();
    };
    let state = state.borrow();
    // The array of the v1 is a table of keys, and its items are the ones
    // that are there, in the order of their keys.
    if the_state_has_a_keyed_item(&state) {
        return the_items_with_a_value(&state)
            .into_iter()
            .map(|(_, value)| value)
            .collect();
    }
    the_items_of(&state)
        .map(|items| {
            items
                .iter()
                .map(|element| element_value(element).unwrap_or(Value::Text("".into())))
                .collect()
        })
        .unwrap_or_default()
}

/// Whether the array has the key.
///
/// The key of the v1 is a number, and the answer does not walk the array:
/// the item under the key is asked for.
pub(crate) fn has_the_key(array: &Value, key: i64) -> bool {
    let Some(state) = state(array) else {
        return false;
    };
    let state = state.borrow();
    if the_keyed_item_of(&state, key).is_some() {
        return true;
    }
    let Some(items) = the_items_of(&state) else {
        return false;
    };
    if key < 1 {
        return false;
    }
    let place = key as usize - 1;
    match items.get(place) {
        Some(item) => element_value(item).is_some(),
        None => false,
    }
}

/// The number of the items of an array that are there.
///
/// The places that are not there are not items: the array of two with the
/// third one written has three items, and the holes are not counted.
pub(crate) fn the_number_of_the_items(array: &Value) -> usize {
    let Some(state) = state(array) else { return 0 };
    let state = state.borrow();
    let keyed = the_keyed_items_of(&state);
    if keyed.is_empty() {
        return the_items_of(&state)
            .map(|items| {
                items
                    .iter()
                    .filter(|item| element_value(item).is_some())
                    .count()
            })
            .unwrap_or(0);
    }
    // The item of a key that is in the row of the places is counted once:
    // the keyed table holds the write that came last.
    let in_the_table = keyed
        .iter()
        .map(|(key, _)| (*key, ()))
        .collect::<BTreeMap<i64, ()>>();
    let in_the_row = the_items_of(&state)
        .map(|items| {
            items
                .iter()
                .enumerate()
                .filter(|(place, item)| {
                    element_value(item).is_some()
                        && !in_the_table.contains_key(&(*place as i64 + 1))
                })
                .count()
        })
        .unwrap_or(0);
    in_the_row + keyed.len()
}

/// The number of the places of an array.
///
/// The places that are not there are places: the array of two with the
/// fifth one written has five places.
pub(crate) fn the_number_of_the_places(array: &Value) -> usize {
    let Some(state) = state(array) else { return 0 };
    let state = state.borrow();
    let places = the_items_of(&state).map_or(0, |items| items.len());
    // Measured on the binary of AutoHotkey v1.1.37.02: the length of the
    // array of the v1 is the highest key it has: the `a[5] := 1` of an empty
    // array is a length of five, and the places that are not there are not
    // items.
    match the_highest_key_of(&state) {
        Some(key) => places.max(key as usize),
        None => places,
    }
}

/// The indexes of an array that have a value.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// `MinIndex()` of an array is the lowest index that has a value, and the
/// `MaxIndex()` is the highest one: the `[1, 2, 3]` with the fifth one
/// written is the one and the five, and the array that has only its third
/// one is the three and the three.
pub(crate) fn the_indexes_with_a_value(array: &Value) -> Vec<i64> {
    let Some(state) = state(array) else {
        return Vec::new();
    };
    let state = state.borrow();
    // The keys of the v1 come in the order of their indexes, and the item
    // of a key that is in the row of the places is the one of the keyed
    // table: the write of a key is the last one that was made.
    the_items_with_a_value(&state)
        .into_iter()
        .map(|(key, _)| key)
        .collect()
}

/// The items of an array, written back.
fn set_items(state: &mut Object, items: Vec<Value>) {
    put(&mut state.properties, "items", Value::List(items));
}

/// How many items an array holds before it has to grow.
fn capacity(state: &Object) -> i64 {
    state
        .value_of("capacity")
        .and_then(|value| value.to_number())
        .map(|number| number as i64)
        .unwrap_or(0)
}

/// How many items an array holds before it has to grow, written.
fn set_capacity(state: &mut Object, capacity: i64) {
    put(&mut state.properties, "capacity", Value::Int(capacity));
}

/// An element of an array: the value, and whether it has one.
///
/// The dialect has no value that means "nothing", so a place without a
/// value of its own is carried as a flag and an empty slot.
/// An element of the array, as the state keeps it: stored directly, with
/// `Value::Unset` marking a hole (no per-element heap allocation).
#[inline]
fn element(value: Option<Value>) -> Value {
    match value {
        Some(value) => value,
        None => Value::Unset,
    }
}

/// The value of an element, when it has one.
#[inline]
fn element_value(element: &Value) -> Option<Value> {
    match element {
        Value::Unset => None,
        other => Some(other.clone()),
    }
}

/// Fast path for `arr.Push(values...)`: single state borrow and batch capacity update.
#[inline]
pub(crate) fn push_fast(array: &Value, values: &[Value]) -> Value {
    if !values.is_empty() {
        if let Some(state) = state(array) {
            let mut state = state.borrow_mut();
            let count = if let Some(items) = the_items_mut(&mut state) {
                items.extend(values.iter().cloned());
                items.len() as i64
            } else {
                0
            };
            if let Some(prop) = state.properties.get_mut("capacity") {
                if let Value::Int(cap) = &mut prop.value {
                    if count > *cap {
                        *cap = count;
                    }
                    return array.clone();
                }
            }
            if count > capacity(&state) {
                set_capacity(&mut state, count);
            }
        }
    }
    array.clone()
}

/// Backing state cell when `array` is a plain v2 `Array` instance with no own
/// properties (`Default`, `__Item`, `Push`, `__Enum`, `__Class`, etc.) and no
/// v1 sparse keyed items.
#[inline]
pub(crate) fn plain_state(array: &Value) -> Option<SharedCell<Object>> {
    let Value::Object(object) = array else {
        return None;
    };
    let borrowed = object.borrow();
    if borrowed.class != "array" || borrowed.properties.len() != 1 {
        return None;
    }
    let Some(Value::Object(state_cell)) = borrowed.value_ref(STATE) else {
        return None;
    };
    if state_cell.borrow().properties.len() != 2 {
        return None;
    }
    Some(state_cell.clone())
}

/// Append a single value directly to a cached plain `Array` state cell.
#[inline]
pub(crate) fn fast_push_one(state_cell: &SharedCell<Object>, val: Value) {
    let mut state = state_cell.borrow_mut();
    let count = match the_items_mut(&mut state) {
        Some(items) => {
            items.push(val);
            items.len() as i64
        }
        None => return,
    };
    if let Some(prop) = state.properties.get_mut("capacity") {
        if let Value::Int(cap) = &mut prop.value {
            if count > *cap {
                *cap = count;
            }
        }
    }
}

/// Read a 1-based (or negative) integer index from a cached plain `Array` state cell.
#[inline]
pub(crate) fn fast_get_int(state_cell: &SharedCell<Object>, index: i64) -> Option<Value> {
    let state = state_cell.borrow();
    let items = the_items_of(&state)?;
    let pos = place(index, items.len() as i64)?;
    element_value(&items[pos])
}

/// Write a 1-based (or negative) integer index in a cached plain `Array` state cell.
#[inline]
pub(crate) fn fast_set_int(state_cell: &SharedCell<Object>, index: i64, val: Value) -> bool {
    let mut state = state_cell.borrow_mut();
    let Some(items) = the_items_mut(&mut state) else {
        return false;
    };
    let Some(pos) = place(index, items.len() as i64) else {
        return false;
    };
    items[pos] = val;
    true
}

/// Read the raw element at 0-based `place` for a compiled `for` loop (`Value::Unset` for holes).
#[inline]
pub(crate) fn fast_for_at(state_cell: &SharedCell<Object>, place: usize) -> Option<Value> {
    let state = state_cell.borrow();
    let items = the_items_of(&state)?;
    items.get(place).cloned()
}

/// The default of an array: the value the script gave to the `Default`
/// property, when it gave one.
fn default_property(array: &Value) -> Option<Value> {
    let Value::Object(object) = array else {
        return None;
    };
    object.borrow().value_of("default")
}

/// Put a value at the end of the array.
fn push_value(array: &Value, value: Value) {
    push_element(array, Some(value));
}

/// Put a value at the end of the array, or a hole.
///
/// The hole is a place of the array that has no value of its own, and it
/// is what the literal of an array with a hole makes.
fn push_element(array: &Value, value: Option<Value>) {
    let Some(state) = state(array) else { return };
    let mut state = state.borrow_mut();
    let count = {
        let Some(items) = the_items_mut(&mut state) else { return };
        items.push(element(value));
        items.len() as i64
    };
    // The array grows to hold what is put in it.
    if count > capacity(&state) {
        set_capacity(&mut state, count);
    }
}

/// A copy of an array: the same values, in the same places, and the same
/// properties.
///
/// The values are copied the way a script copies them: a value that is
/// an object is the same object in the copy.
fn clone(array: &Value) -> Value {
    let (items, capacity, keyed) = match state(array) {
        Some(state) => {
            let state = state.borrow();
            (items(&state), capacity(&state), the_keyed_items_of(&state))
        }
        None => (Vec::new(), 0, Vec::new()),
    };
    let mut properties = BTreeMap::new();
    let mut copy = new_state(items, capacity);
    // The items under a key are copied as well: the copy of an array of the
    // v1 has the keys of the original.
    if let Value::Object(cell) = &mut copy {
        let mut state = cell.borrow_mut();
        for (key, value) in keyed {
            set_the_keyed_item(&mut state, key, value);
        }
    }
    put(&mut properties, STATE, copy);
    // The properties the script gave the array are copied as well.
    if let Value::Object(object) = array {
        for (name, property) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), property.clone());
            }
        }
    }
    Value::Object(SharedCell::new(Object {
        class: "array".to_string(),
        properties,
    }))
}

// -- the places ---------------------------------------------------------------

/// The place of an index: the places count from one, and a negative
/// index counts from the end.
fn place(index: i64, length: i64) -> Option<usize> {
    let place = if index < 0 { length + index + 1 } else { index };
    if place < 1 || place > length {
        None
    } else {
        Some(place as usize - 1)
    }
}

/// The place of an index in the dialect of the v1: the places count from
/// one, and an index that is not positive is not a place at all.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the v1
/// does not count from the end: the `a[-1]` of an array of two is the
/// empty text, and the `a[0]` is the empty text too.
fn the_place_of_the_v1(index: i64, length: i64) -> Option<usize> {
    if index < 1 || index > length {
        None
    } else {
        Some(index as usize - 1)
    }
}

/// The place of an insert: the places count from one, a zero is the end
/// of the array, and a negative index counts from the end.
fn insert_place(index: i64, length: i64) -> Option<usize> {
    let place = if index < 0 {
        length + index + 1
    } else if index == 0 {
        length + 1
    } else {
        index
    };
    if place < 1 || place > length + 1 {
        None
    } else {
        Some(place as usize - 1)
    }
}

// -- the base object ----------------------------------------------------------

/// The methods of the base object, answered for an array.
///
/// The runtime answers them for the classes of the script, and it knows
/// nothing of the class types of the runtime: an array has no method of
/// the script, and the places of an array are not properties of it.
fn base_method(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else {
        return None;
    };
    // Measured on the binary of AutoHotkey v1.1.37.02: the
    // array of the v1 has no method of the base object of the v2: the
    // `a.HasOwnProp(1)` is ignored and the answer is the empty text.
    if !rt.dialect().the_object_has_the_methods_of_the_base_object() {
        return None;
    }
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError {
            message: message.to_string(),
            line,
        })))
    };
    let name = arguments
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let key = name.to_lowercase();
    match method {
        "ownprops" => {
            // Measured on the binary of v2.0.19: the own
            // properties of v2 are an enumerator, and it gives the name
            // when the caller wants one value.
            let pairs: Vec<(Value, Value)> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(_, property)| (Value::Text(property.name.clone().into()), property.value.clone()))
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheKey)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str())
                // The methods of the class are the properties of the value.
                //
                // Measured on the binary of v2.0.19:
                // `m.HasProp("Get")` answers 1.
                || METHODS.contains(&key.as_str())
                || own_property(object, &key)
                // Every value carries its class, and the base object has
                // the property of it.
                || key == "__class"
                // Every class has an initialiser, and every object has a
                // base.
                //
                // Measured on the binary of v2.0.19:
                // `m.HasProp("__Init")` and `m.HasProp("Base")` answer 1.
                || key == "__init"
                || key == "base") as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(
            (METHODS.contains(&key.as_str())
                // Every class has an initialiser.
                //
                // Measured on the binary of v2.0.19:
                // `m.HasMethod("__Init")` answers 1.
                || key == "__init") as i64,
        ))),
        "getownpropdesc" => {
            let name = arguments
                .first()
                .map(|value| value.to_text())
                .unwrap_or_default();
            let value = object.borrow().value_of(&key);
            match value {
                Some(value) => {
                    // Measured on the binary of v2.0.19: the
                    // descriptor that comes back is an object with the
                    // members of it.
                    let table = match rt.the_members_of_a_descriptor(&value) {
                        Some(members) => members,
                        None => {
                            let mut table = BTreeMap::new();
                            put(&mut table, "value", value);
                            table
                        }
                    };
                    Some(Ok(Value::Object(SharedCell::new(Object {
                        class: "object".to_string(),
                        properties: table,
                    }))))
                }
                // The property is not there: v2 says so.
                None => Some(Err(rt.no_property(this, &name, line))),
            }
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of an array is not a property of it.");
            }
            // Measured on the binary of v2.0.19: the second
            // parameter is a descriptor, and a value that is not one is
            // refused with the reason.
            let descriptor = match rt.the_descriptor(arguments.get(1), line) {
                Ok(descriptor) => descriptor,
                Err(failure) => return Some(Err(failure)),
            };
            put(&mut object.borrow_mut().properties, &name, descriptor);
            // The answer of v2 is the object itself.
            Some(Ok(this.clone()))
        }
        "deleteprop" => {
            if key == STATE {
                return Some(Ok(Value::Text("".into())));
            }
            let gone = object
                .borrow_mut()
                .properties
                .remove(&key)
                .map(|property| property.value);
            // Measured on the binary of v2.0.19: the answer
            // is the value of the property that is taken away, and an
            // empty text when there was none.
            Some(Ok(match gone {
                None => Value::Text("".into()),
                Some(value) => match rt.the_value_of_the_descriptor(&value, this, &name, line) {
                    Ok(Some(value)) => value,
                    _ => value,
                },
            }))
        }
        _ => None,
    }
}

/// Whether the array has a property of that name.
///
/// The state of the array is not a property of it: it is the inside of
/// the array, and the script never wrote it.
fn own_property(object: &SharedCell<Object>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}

/// The places a search visits, in the order it visits them.
///
/// A start of one walks from the first place toward the end, and a start of
/// minus one walks from the last place toward the beginning: the
/// documentation says a negative start searches backwards.
fn the_places_of_the_walk(start: usize, forward: bool, len: usize) -> Vec<usize> {
    if forward {
        (start..len).collect()
    } else {
        (0..=start).rev().collect()
    }
}

/// The place a search starts at: the 0-based place, and whether it walks
/// forward.
///
/// A start of zero is refused, and so is one whose absolute value is past
/// the end: the documentation says both throw. A negative start walks from
/// the end toward the beginning, and -1 is the last place.
fn the_start_of_the_search(start: i64, len: i64) -> Option<(usize, bool)> {
    if start == 0 || start.unsigned_abs() as i64 > len {
        return None;
    }
    if start > 0 {
        Some(((start - 1) as usize, true))
    } else {
        Some(((len + start) as usize, false))
    }
}

/// The start a member is asked with: the second parameter when it is there,
/// and one when it is not.
///
/// The start is a number of the whole kind: a text that does not hold one
/// is an invalid parameter, the way the invalid places of the v2 are.
fn the_start_of_the_arguments(
    arguments: &[Value],
    who: &str,
    line: u32,
) -> Result<i64, RuntimeFailure> {
    let Some(value) = arguments.get(1) else {
        return Ok(1);
    };
    let text = value.to_text();
    let Ok(start) = text.trim().parse::<i64>() else {
        return Err(crate::classes::the_failure_with(
            &alloc::format!("Parameter #2 of Array.Prototype.{who} is invalid."),
            &text,
            line,
        ));
    };
    Ok(start)
}

/// Whether two values are the same, the way the searches of KeySharp
/// compare them.
///
/// The documentation says the comparisons compare the actual underlying
/// values, so "1" is not 1 -- a search of an array is not the `=` of an
/// `if`, which reads a text that holds a number as the number. Two numbers
/// are the same number, the way the numbers of the tree are: 1 and 1.0 are
/// one.
fn the_search_matches(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Object(one), Value::Object(other)) => SharedCell::ptr_eq(one, other),
        (Value::Object(_), _) | (_, Value::Object(_)) => false,
        (Value::Text(one), Value::Text(other)) => one == other,
        (Value::Int(one), Value::Int(other)) => one == other,
        (Value::Float(one), Value::Float(other)) => one == other,
        (Value::Int(one), Value::Float(other)) | (Value::Float(other), Value::Int(one)) => {
            (*one as f64) == *other
        }
        (Value::Unset, Value::Unset) => true,
        _ => false,
    }
}

/// The place of the first item that matches, as the 1-based answer, or 0.
///
/// An empty array answers zero whatever the start is: the documentation
/// says the empty array is not validated, so the start is not read there.
/// A place that has no value does not match, because a value of the caller
/// is never the absence of one.
fn the_place_of_the_match(
    items: &[Value],
    wanted: &Value,
    arguments: &[Value],
    who: &str,
    _rt: &mut Runtime,
    line: u32,
) -> Result<i64, RuntimeFailure> {
    let len = items.len() as i64;
    if len == 0 {
        return Ok(0);
    }
    let start = the_start_of_the_arguments(arguments, who, line)?;
    let Some((from, forward)) = the_start_of_the_search(start, len) else {
        let text = arguments.get(1).map(|v| v.to_text()).unwrap_or_default();
        return Err(crate::classes::the_failure_with(
            &alloc::format!("Parameter #2 of Array.Prototype.{who} is invalid."),
            &text,
            line,
        ));
    };
    for place in the_places_of_the_walk(from, forward, items.len()) {
        if let Some(item) = element_value(&items[place]) {
            if the_search_matches(&item, wanted) {
                return Ok(place as i64 + 1);
            }
        }
    }
    Ok(0)
}

/// The call of the callback of a member: the callback takes the item, and
/// when it takes two parameters, the place of the item as well.
///
/// The count of the parameters is read out of the function: a callback
/// written with one is called with the item alone, because a call that
/// gives a parameter the function does not have is refused. A value that is
/// not a function is an invalid first parameter.
fn the_call_of_the_callback(
    callback: &Value,
    item: &Value,
    place: i64,
    who: &str,
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let invalid = || {
        crate::classes::the_failure_with(
            &alloc::format!("Parameter #1 of Array.Prototype.{who} is invalid."),
            "a function",
            line,
        )
    };
    let Value::Object(_) = callback else {
        return Err(invalid());
    };
    let count = rt.the_count_of_the_parameters_of_a_function(callback) as i64;
    let arguments = if count >= 2 {
        vec![item.clone(), Value::Int(place)]
    } else {
        vec![item.clone()]
    };
    match crate::classes::func::call_the_value(callback, &arguments, rt, line) {
        Some(result) => result,
        None => Err(invalid()),
    }
}

/// The sign of the answer of the callback of a `Sort`: only the sign
/// counts, the way the documentation says, and an answer that is not a
/// number counts as equal.
fn the_sign_of(value: &Value) -> f64 {
    match value {
        Value::Int(number) => number.signum() as f64,
        Value::Float(number) => number.signum(),
        Value::Text(text) => text
            .trim()
            .parse::<f64>()
            .map(|n| n.signum())
            .unwrap_or(0.0),
        _ => 0.0,
    }
}

/// The text an element of a `ToString` answer is written with.
///
/// A place without a value is written `unset`; a text is written between
/// quotes; an object is written with the answer of its own `ToString`, or
/// with the name of its class when it has none; and an object whose
/// `ToString` raises is written `<ERROR>`, the way the documentation says.
pub(crate) fn the_text_of_an_element(item: &Value, rt: &mut Runtime, line: u32) -> String {
    let Some(value) = element_value(item) else {
        return "unset".to_string();
    };
    the_text_of_a_value(&value, rt, line)
}

/// The text a value is written with in a `ToString` answer, the way the
/// documentation of KeySharp formats the elements of an `Array`.
pub(crate) fn the_text_of_a_value(value: &Value, rt: &mut Runtime, line: u32) -> String {
    match value {
        Value::Text(text) => {
            let mut out = String::from("\"");
            for character in text.chars() {
                if character == '"' || character == '\\' {
                    out.push('\\');
                }
                out.push(character);
            }
            out.push('"');
            out
        }
        Value::Object(object) => {
            let class = object.borrow().class.clone();
            match crate::classes::call(&class, "tostring", value, &[], rt, line) {
                Some(Ok(text)) => text.to_text(),
                Some(Err(_)) => "<ERROR>".to_string(),
                None => crate::classes::the_spelling_of(&class),
            }
        }
        other => other.to_text(),
    }
}

/// The answer of a member that the KeySharp extension adds.
///
/// The rules of each one are the ones the documentation of KeySharp writes;
/// where the documentation does not say what a place without a value does,
/// the choice is written where it is made.
fn the_extension_answer(
    method: &str,
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let invalid_first = |who: &str, extra: &str| {
        crate::classes::the_failure_with(
            &alloc::format!("Parameter #1 of Array.Prototype.{who} is invalid."),
            extra,
            line,
        )
    };
    let Some(state) = state(this) else {
        return Some(Err(RuntimeFailure::Error(RuntimeError {
            message: "Invalid index.".to_string(),
            line,
        })));
    };
    let items = items(&state.borrow());
    match method {
        "contains" => {
            let wanted = arguments.first().cloned().unwrap_or(Value::Unset);
            let found =
                match the_place_of_the_match(&items, &wanted, arguments, "Contains", rt, line) {
                    Ok(found) => found,
                    Err(failure) => return Some(Err(failure)),
                };
            Some(Ok(Value::Int((found != 0) as i64)))
        }
        "indexof" => {
            let wanted = arguments.first().cloned().unwrap_or(Value::Unset);
            let found =
                match the_place_of_the_match(&items, &wanted, arguments, "IndexOf", rt, line) {
                    Ok(found) => found,
                    Err(failure) => return Some(Err(failure)),
                };
            Some(Ok(Value::Int(found)))
        }
        "findindex" => {
            let Some(callback) = arguments.first().cloned() else {
                return Some(Err(invalid_first("FindIndex", "a function")));
            };
            let len = items.len() as i64;
            if len == 0 {
                return Some(Ok(Value::Int(0)));
            }
            let start = match the_start_of_the_arguments(arguments, "FindIndex", line) {
                Ok(start) => start,
                Err(failure) => return Some(Err(failure)),
            };
            let Some((from, forward)) = the_start_of_the_search(start, len) else {
                let text = arguments.get(1).map(|v| v.to_text()).unwrap_or_default();
                return Some(Err(invalid_first("FindIndex", &text)));
            };
            for place in the_places_of_the_walk(from, forward, items.len()) {
                // A place that has no value is handed to the callback as the
                // value of unset: the callback decides what an absence is.
                let item = element_value(&items[place]).unwrap_or(Value::Unset);
                let answer = match the_call_of_the_callback(
                    &callback,
                    &item,
                    place as i64 + 1,
                    "FindIndex",
                    rt,
                    line,
                ) {
                    Ok(answer) => answer,
                    Err(failure) => return Some(Err(failure)),
                };
                if answer.truthy() {
                    return Some(Ok(Value::Int(place as i64 + 1)));
                }
            }
            Some(Ok(Value::Int(0)))
        }
        "filter" | "map" => {
            let who = if method == "filter" { "Filter" } else { "Map" };
            let Some(callback) = arguments.first().cloned() else {
                return Some(Err(invalid_first(who, "a function")));
            };
            if items.is_empty() {
                // An empty array answers an empty one whatever the start is.
                return Some(Ok(crate::classes::object_with(
                    "array",
                    vec![(STATE, new_state(Vec::new(), 0))],
                )));
            }
            let start = match the_start_of_the_arguments(arguments, who, line) {
                Ok(start) => start,
                Err(failure) => return Some(Err(failure)),
            };
            let Some((from, forward)) = the_start_of_the_search(start, items.len() as i64) else {
                let text = arguments.get(1).map(|v| v.to_text()).unwrap_or_default();
                return Some(Err(invalid_first(who, &text)));
            };
            let mut out = Vec::new();
            for place in the_places_of_the_walk(from, forward, items.len()) {
                let item = element_value(&items[place]).unwrap_or(Value::Unset);
                let answer = match the_call_of_the_callback(
                    &callback,
                    &item,
                    place as i64 + 1,
                    who,
                    rt,
                    line,
                ) {
                    Ok(answer) => answer,
                    Err(failure) => return Some(Err(failure)),
                };
                if method == "filter" {
                    if answer.truthy() {
                        out.push(items[place].clone());
                    }
                } else {
                    // The answer of the callback is a value, and the places
                    // of an array hold the wrapped form of one: the value is
                    // wrapped the way a push wraps it, so that the array the
                    // member answers reads like any other.
                    out.push(element(Some(answer)));
                }
            }
            Some(Ok(crate::classes::object_with(
                "array",
                vec![(STATE, new_state(out, 0))],
            )))
        }
        "join" => {
            let separator = arguments
                .first()
                .map(|value| value.to_text())
                .unwrap_or_else(|| ",".to_string());
            // A place without a value joins as the empty text: the
            // documentation does not say, and the empty text is the one the
            // rest of the tree reads a place without a value as.
            let parts: Vec<String> = items
                .iter()
                .map(|item| {
                    element_value(item)
                        .map(|value| value.to_text())
                        .unwrap_or_default()
                })
                .collect();
            Some(Ok(Value::Text(parts.join(&separator).into())))
        }
        "remove" => {
            if items.is_empty() {
                return Some(Ok(Value::Int(0)));
            }
            let found = match arguments.first() {
                // Without a value, the first place that has no value goes.
                None => items
                    .iter()
                    .position(|item| element_value(item).is_none())
                    .map(|place| place as i64 + 1)
                    .unwrap_or(0),
                Some(wanted) => {
                    match the_place_of_the_match(&items, wanted, arguments, "Remove", rt, line) {
                        Ok(found) => found,
                        Err(failure) => return Some(Err(failure)),
                    }
                }
            };
            if found == 0 {
                return Some(Ok(Value::Int(0)));
            }
            let mut owned = items;
            owned.remove((found - 1) as usize);
            set_items(&mut state.borrow_mut(), owned);
            Some(Ok(Value::Int(1)))
        }
        "sort" => {
            let Some(callback) = arguments.first().cloned() else {
                return Some(Err(invalid_first("Sort", "a function")));
            };
            if items.len() < 2 {
                return Some(Ok(this.clone()));
            }
            let mut sorted = items;
            // The callback may raise, and the sort cannot stop in the middle:
            // the first failure it makes is kept and raised when the sort is
            // over, and the array is left as the partial sort left it.
            let failure: RefCell<Option<RuntimeFailure>> = RefCell::new(None);
            sorted.sort_by(|one, other| {
                if failure.borrow().is_some() {
                    return core::cmp::Ordering::Equal;
                }
                // The items are the places of the array, and the place of
                // one that has no value is handed to the callback as the
                // value of unset: the callback decides what an absence is.
                let first = element_value(one).unwrap_or(Value::Unset);
                let second = element_value(other).unwrap_or(Value::Unset);
                let answer = match crate::classes::func::call_the_value(
                    &callback,
                    &[first, second],
                    rt,
                    line,
                ) {
                    Some(Ok(answer)) => answer,
                    Some(Err(raise)) => {
                        *failure.borrow_mut() = Some(raise);
                        return core::cmp::Ordering::Equal;
                    }
                    None => {
                        *failure.borrow_mut() = Some(invalid_first("Sort", "a function"));
                        return core::cmp::Ordering::Equal;
                    }
                };
                let sign = the_sign_of(&answer);
                sign.partial_cmp(&0.0).unwrap_or(core::cmp::Ordering::Equal)
            });
            if let Some(failure) = failure.into_inner() {
                return Some(Err(failure));
            }
            set_items(&mut state.borrow_mut(), sorted);
            Some(Ok(this.clone()))
        }
        "tostring" => {
            let mut out = String::new();
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push_str(", ");
                }
                out.push_str(&the_text_of_an_element(item, rt, line));
            }
            out.push(']');
            Some(Ok(Value::Text(out.into())))
        }
        // The dialect said the member is carried, and the table of the
        // extensions says which ones are: a name that is not here is a
        // member the table and the dialect disagree about, and the honest
        // answer is that the tree does not have it.
        _ => None,
    }
}
