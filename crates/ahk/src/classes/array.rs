//! The `Array` class.
//!
//! An array is a row of values. The places count from one, the way
//! AutoHotkey counts them, and a place can be left without a value of
//! its own.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::enumerator;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};

/// The key the state of an array lives under.
///
/// The name is not one a script can write: the name of a property in a
/// script is always an identifier, and this one is not. It is not a
/// property of the array either, so it stays out of the answers about
/// the properties.
const STATE: &str = "array state";

/// The class of the object that carries the state of an array.
const STATE_CLASS: &str = "array-state";

/// The properties an array owns.
const PROPERTIES: [&str; 3] = ["length", "capacity", "__item"];

/// The methods an array has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 19] = [
    "push", "pop", "insertat", "removeat", "delete", "get", "has", "clone",
    "__new", "__enum", "ownprops", "hasownprop", "defineprop", "deleteprop",
    "getownpropdesc", "hasprop", "hasmethod", "getmethod", "hasbase",
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
/// Measured on the binary of v2.0.19 on 2026-11-01: the literal may have a
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
        Some(Err(RuntimeFailure::Error(RuntimeError { message: message.to_string(), line })))
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
            &format!("Parameter #1 of {} is invalid.", crate::classes::the_name_of("array", method)),
            &index.to_string(),
        )
    };
    match method.as_str() {
        // `Push` and `__New` are the same thing in v2: both put the values
        // at the end, and both answer the array, so that the calls can be
        // chained.
        "push" | "__new" => {
            for value in arguments {
                push_value(this, value.clone());
            }
            Some(Ok(this.clone()))
        }
        "pop" => {
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let Some(last) = items.pop() else { return failure("Array is empty.") };
            set_items(&mut state, items);
            Some(Ok(element_value(&last).unwrap_or(Value::Text(String::new()))))
        }
        "insert" if rt.dialect().the_array_has_the_methods_of_the_v1() => {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `Insert` of the v1 takes the index and the value: the
            // `a.Insert(1, 9)` puts the nine at the first place. A value on
            // its own goes to the end: the `a.Insert(9)` puts the nine at
            // the end of the array.
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let length = items.len() as i64;
            if arguments.len() < 2 {
                if let Some(value) = arguments.first() {
                    items.push(element(Some(value.clone())));
                }
                set_items(&mut state, items);
                return Some(Ok(Value::Text(String::new())));
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
            set_items(&mut state, items);
            Some(Ok(Value::Text(String::new())))
        }
        "remove" if rt.dialect().the_array_has_the_methods_of_the_v1() => {
            // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04:
            // the `Remove` of the v1 takes the index: the `a.Remove(1)`
            // takes the first one away. Without an index the last one goes
            // away: the `a.Remove()` is the `a.Pop()`.
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let length = items.len() as i64;
            let Some(index) = arguments.first() else {
                let Some(last) = items.pop() else { return failure("Array is empty.") };
                set_items(&mut state, items);
                return Some(Ok(element_value(&last).unwrap_or(Value::Text(String::new()))));
            };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(place) = place(index, length) else {
                return invalid_place("Remove", index);
            };
            let removed = items.remove(place);
            set_items(&mut state, items);
            Some(Ok(element_value(&removed).unwrap_or(Value::Text(String::new()))))
        }
        "insertat" => {
            let Some(index) = arguments.first() else { return failure("Too few parameters passed to function.") };
            // There is something to insert: an index on its own inserts
            // nothing.
            if arguments.len() < 2 {
                return failure("Too few parameters passed to function.");
            }
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let length = items.len() as i64;
            // A zero is the end of the array, and a negative index counts
            // from the end.
            let Some(place) = insert_place(index, length) else {
                return invalid_place("InsertAt", index);
            };
            for (offset, value) in arguments.iter().skip(1).enumerate() {
                items.insert(place + offset, element(Some(value.clone())));
            }
            set_items(&mut state, items);
            Some(Ok(Value::Text(String::new())))
        }
        "removeat" => {
            let Some(index) = arguments.first() else { return failure("Too few parameters passed to function.") };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let length = items.len() as i64;
            let Some(place) = place(index, length) else {
                return invalid_place("RemoveAt", index);
            };
            match arguments.get(1) {
                // One place: the value that was there is the answer.
                None => {
                    let removed = items.remove(place);
                    set_items(&mut state, items);
                    Some(Ok(element_value(&removed).unwrap_or(Value::Text(String::new()))))
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
                    set_items(&mut state, items);
                    Some(Ok(Value::Int(removed)))
                }
            }
        }
        "delete" => {
            let Some(index) = arguments.first() else { return failure("Too few parameters passed to function.") };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let mut state = state.borrow_mut();
            let mut items = items(&state);
            let length = items.len() as i64;
            let Some(place) = place(index, length) else {
                return invalid_place("Delete", index);
            };
            // The place stays, and it has no value of its own any more.
            let removed = element_value(&items[place]).unwrap_or(Value::Text(String::new()));
            items[place] = element(None);
            set_items(&mut state, items);
            Some(Ok(removed))
        }
        "get" => {
            let Some(index) = arguments.first() else { return failure("Too few parameters passed to function.") };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else { return failure("Invalid index.") };
            let state = state.borrow();
            let items = items(&state);
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
            let Some(index) = arguments.first() else { return failure("Too few parameters passed to function.") };
            let index = match rt.integer(index, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            let Some(state) = state(this) else { return Some(Ok(Value::Int(0))) };
            let state = state.borrow();
            let items = items(&state);
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
        // places. Measured on the binary of v2.0.19 on 2026-10-31: the
        // enumeration of v2 is an enumerator, and it gives the value when
        // the caller wants one value and the place and the value when the
        // caller wants two. The number of the variables is not carried by
        // the enumerator: it is the call that says how many it wants.
        "__enum" => {
            let Some(state) = state(this) else {
                return Some(Ok(enumerator::make(Vec::new(), enumerator::Alone::TheValue)));
            };
            let state = state.borrow();
            let pairs: Vec<(Value, Value)> = items(&state)
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    // Measured on the binary of v2.0.19 on 2026-11-01: the
                    // place that has no value of its own is a turn of the
                    // enumeration too, and the turn has no value of its own:
                    // the variable of the loop is left without one.
                    let value = element_value(item).unwrap_or_else(|| element(None));
                    (Value::Int(index as i64 + 1), value)
                })
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheValue)))
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
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
    let mut items = items(&state);
    let length = items.len() as i64;
    let place = if rt.dialect().the_array_grows_with_the_write_of_an_item() {
        the_place_of_the_v1(index, length)
    } else {
        place(index, length)
    };
    let Some(place) = place else {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 makes the array grow: the `a[5] := 7` of an array of two is the
        // seven, and the places that are not there are holes.
        if rt.dialect().the_array_grows_with_the_write_of_an_item() {
            if index > length {
                while (items.len() as i64) < index - 1 {
                    items.push(element(None));
                }
                items.push(element(Some(value)));
                set_items(&mut state, items);
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
    set_items(&mut state, items);
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
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    // array of the v1 has neither the length nor the capacity: the
    // `a.Length` is the empty text. The v1 counts the array with the
    // `a.Length()` and the `a.Count()`.
    if !rt.dialect().the_array_has_the_length_and_the_capacity() {
        return None;
    }
    let state = state.borrow();
    match name {
        "length" => Some(Ok(Value::Int(items(&state).len() as i64))),
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
        Some(Err(RuntimeFailure::Error(RuntimeError { message: message.to_string(), line })))
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
            let mut items = items(&state);
            // A longer array has places without a value of their own, and
            // a shorter one loses the places at the end.
            items.truncate(wanted as usize);
            while (items.len() as i64) < wanted {
                items.push(element(None));
            }
            set_items(&mut state, items);
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
            let mut items = items(&state);
            // A capacity below the length of the array takes the places at
            // the end away.
            if (wanted as usize) < items.len() {
                items.truncate(wanted as usize);
            }
            let count = items.len() as i64;
            set_items(&mut state, items);
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
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
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
    let items = items(&state);
    let length = items.len() as i64;
    let place = if rt.dialect().the_item_of_an_array_that_is_not_there_is_the_empty_text() {
        the_place_of_the_v1(index, length)
    } else {
        place(index, length)
    };
    let Some(place) = place else {
        // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
        // v1 does not refuse the item of an array that is not there: the
        // `a[5]` of an array of two is the empty text, and the `a[0]` is the
        // property "0" of the array.
        if rt.dialect().the_item_of_an_array_that_is_not_there_is_the_empty_text() {
            if index < 1 {
                let name = key.to_text();
                return Some(rt.read_property(this, &name, line));
            }
            return Some(Ok(Value::Text(String::new())));
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
            None => Some(Err(RuntimeFailure::Error(RuntimeError {
                message: "Item has no value.".into(),
                line,
            }))),
        },
    }
}

// -- the state ----------------------------------------------------------------

/// The state of an array.
fn state(array: &Value) -> Option<Rc<RefCell<Object>>> {
    let Value::Object(object) = array else { return None };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// A new state: the items in the order of their places, and how many of
/// them fit.
fn new_state(items: Vec<Value>, capacity: i64) -> Value {
    let mut state = Object { class: STATE_CLASS.to_string(), properties: BTreeMap::new() };
    set_items(&mut state, items);
    set_capacity(&mut state, capacity);
    Value::Object(Rc::new(RefCell::new(state)))
}

/// The items of an array: the elements in the order of their places.
fn items(state: &Object) -> Vec<Value> {
    let Some(Value::List(items)) = state.value_of("items") else { return Vec::new() };
    items.clone()
}

/// The items of an array, for the built-ins that take one.
///
/// The `StrSplit` takes the delimiters in an array, and the `Sort` takes
/// the values of one. Neither of them may reach into the state of the
/// array: the module that owns the class is the one that answers.
pub(crate) fn the_items(array: &Value) -> Vec<Value> {
    let Some(state) = state(array) else { return Vec::new() };
    let state = state.borrow();
    items(&state)
        .iter()
        .map(|element| element_value(element).unwrap_or(Value::Text(String::new())))
        .collect()
}

/// The number of the items of an array that are there.
///
/// The places that are not there are not items: the array of two with the
/// third one written has three items, and the holes are not counted.
pub(crate) fn the_number_of_the_items(array: &Value) -> usize {
    let Some(state) = state(array) else { return 0 };
    let state = state.borrow();
    items(&state).iter().filter(|item| element_value(item).is_some()).count()
}

/// The number of the places of an array.
///
/// The places that are not there are places: the array of two with the
/// fifth one written has five places.
pub(crate) fn the_number_of_the_places(array: &Value) -> usize {
    let Some(state) = state(array) else { return 0 };
    let state = state.borrow();
    items(&state).len()
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
fn element(value: Option<Value>) -> Value {
    match value {
        Some(value) => Value::List(vec![Value::Int(1), value]),
        None => crate::runtime::the_mark_of_a_value_without_one(),
    }
}

/// The value of an element, when it has one.
fn element_value(element: &Value) -> Option<Value> {
    match element {
        Value::List(items) if items.len() == 2 && items[0] == Value::Int(1) => Some(items[1].clone()),
        _ => None,
    }
}

/// The default of an array: the value the script gave to the `Default`
/// property, when it gave one.
fn default_property(array: &Value) -> Option<Value> {
    let Value::Object(object) = array else { return None };
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
    let mut items = items(&state);
    items.push(element(value));
    let count = items.len() as i64;
    set_items(&mut state, items);
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
    let (items, capacity) = match state(array) {
        Some(state) => {
            let state = state.borrow();
            (items(&state), capacity(&state))
        }
        None => (Vec::new(), 0),
    };
    let mut properties = BTreeMap::new();
    put(&mut properties, STATE, new_state(items, capacity));
    // The properties the script gave the array are copied as well.
    if let Value::Object(object) = array {
        for (name, property) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), property.clone());
            }
        }
    }
    Value::Object(Rc::new(RefCell::new(Object { class: "array".to_string(), properties })))
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
/// Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the v1
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
    let Value::Object(object) = this else { return None };
    // Measured on the binary of AutoHotkey v1.1.37.02 on 2026-11-04: the
    // array of the v1 has no method of the base object of the v2: the
    // `a.HasOwnProp(1)` is ignored and the answer is the empty text.
    if !rt.dialect().the_object_has_the_methods_of_the_base_object() {
        return None;
    }
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError { message: message.to_string(), line })))
    };
    let name = arguments.first().map(|value| value.to_text()).unwrap_or_default();
    let key = name.to_lowercase();
    match method {
        "ownprops" => {
            // Measured on the binary of v2.0.19 on 2026-10-31: the own
            // properties of v2 are an enumerator, and it gives the name
            // when the caller wants one value.
            let pairs: Vec<(Value, Value)> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(_, property)| (Value::Text(property.name.clone()), property.value.clone()))
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheKey)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str())
                // The methods of the class are the properties of the value.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasProp("Get")` answers 1.
                || METHODS.contains(&key.as_str())
                || own_property(object, &key)
                // Every value carries its class, and the base object has
                // the property of it.
                || key == "__class"
                // Every class has an initialiser, and every object has a
                // base.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasProp("__Init")` and `m.HasProp("Base")` answer 1.
                || key == "__init"
                || key == "base") as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(
            (METHODS.contains(&key.as_str())
                // Every class has an initialiser.
                //
                // Measured on the binary of v2.0.19 on 2026-10-31:
                // `m.HasMethod("__Init")` answers 1.
                || key == "__init") as i64,
        ))),
        "getownpropdesc" => {
            let name = arguments.first().map(|value| value.to_text()).unwrap_or_default();
            let value = object.borrow().value_of(&key);
            match value {
                Some(value) => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
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
                    Some(Ok(Value::Object(Rc::new(RefCell::new(Object {
                        class: "object".to_string(),
                        properties: table,
                    })))))
                }
                // The property is not there: v2 says so.
                None => Some(Err(rt.no_property(this, &name, line))),
            }
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of an array is not a property of it.");
            }
            // Measured on the binary of v2.0.19 on 2026-10-31: the second
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
                return Some(Ok(Value::Text(String::new())));
            }
            let gone = object.borrow_mut().properties.remove(&key).map(|property| property.value);
            // Measured on the binary of v2.0.19 on 2026-10-31: the answer
            // is the value of the property that is taken away, and an
            // empty text when there was none.
            Some(Ok(match gone {
                None => Value::Text(String::new()),
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
fn own_property(object: &Rc<RefCell<Object>>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}
