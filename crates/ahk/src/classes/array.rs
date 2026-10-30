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
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value};

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
                    let value = element_value(item).unwrap_or(Value::Text(String::new()));
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
    let Some(place) = place(index, length) else {
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
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let state = state(this)?;
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
    let Some(place) = place(index, length) else {
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
    let state = object.borrow().properties.get(STATE)?.clone();
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
    let Some(Value::List(items)) = state.properties.get("items") else { return Vec::new() };
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

/// The items of an array, written back.
fn set_items(state: &mut Object, items: Vec<Value>) {
    state.properties.insert("items".to_string(), Value::List(items));
}

/// How many items an array holds before it has to grow.
fn capacity(state: &Object) -> i64 {
    state
        .properties
        .get("capacity")
        .and_then(|value| value.to_number())
        .map(|number| number as i64)
        .unwrap_or(0)
}

/// How many items an array holds before it has to grow, written.
fn set_capacity(state: &mut Object, capacity: i64) {
    state.properties.insert("capacity".to_string(), Value::Int(capacity));
}

/// An element of an array: the value, and whether it has one.
///
/// The dialect has no value that means "nothing", so a place without a
/// value of its own is carried as a flag and an empty slot.
fn element(value: Option<Value>) -> Value {
    match value {
        Some(value) => Value::List(vec![Value::Int(1), value]),
        None => Value::List(vec![Value::Int(0)]),
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
    object.borrow().properties.get("default").cloned()
}

/// Put a value at the end of the array.
fn push_value(array: &Value, value: Value) {
    let Some(state) = state(array) else { return };
    let mut state = state.borrow_mut();
    let mut items = items(&state);
    items.push(element(Some(value)));
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
    properties.insert(STATE.to_string(), new_state(items, capacity));
    // The properties the script gave the array are copied as well.
    if let Value::Object(object) = array {
        for (name, value) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), value.clone());
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
                .map(|(name, value)| (Value::Text(name.clone()), value.clone()))
                .collect();
            Some(Ok(enumerator::make(pairs, enumerator::Alone::TheKey)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str())
                || own_property(object, &key)
                // Every value carries its class, and the base object has
                // the property of it.
                || key == "__class") as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(METHODS.contains(&key.as_str()) as i64))),
        // An array extends the base object, and nothing else.
        "hasbase" => Some(Ok(Value::Int((key == "object") as i64))),
        "getownpropdesc" => {
            let name = arguments.first().map(|value| value.to_text()).unwrap_or_default();
            let value = object.borrow().properties.get(&key).cloned();
            match value {
                Some(value) => {
                    // Measured on the binary of v2.0.19 on 2026-10-31: the
                    // descriptor that comes back is an object with the
                    // members of it.
                    let table = match rt.the_members_of_a_descriptor(&value) {
                        Some(members) => members,
                        None => {
                            let mut table = BTreeMap::new();
                            table.insert("value".to_string(), value);
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
            object.borrow_mut().properties.insert(key, descriptor);
            // The answer of v2 is the object itself.
            Some(Ok(this.clone()))
        }
        "deleteprop" => {
            if key == STATE {
                return Some(Ok(Value::Text(String::new())));
            }
            let gone = object.borrow_mut().properties.remove(&key);
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
        // A method of a class type of the runtime is not a value this
        // runtime carries.
        "getmethod" => failure("this runtime does not carry the method of a class type as a value."),
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
