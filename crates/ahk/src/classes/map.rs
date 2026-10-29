//! The `Map` class.
//!
//! A map is a table of keys and values. The key may be any value, and
//! the order of the entries is the order they were put in.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value};

/// The key the state of a map lives under.
///
/// The name is not one a script can write: the name of a property in a
/// script is always an identifier, and this one is not. It is not a
/// property of the map either, so it stays out of the answers about the
/// properties.
const STATE: &str = "map state";

/// The class of the object that carries the state of a map.
const STATE_CLASS: &str = "map-state";

/// The properties a map owns.
const PROPERTIES: [&str; 4] = ["count", "capacity", "casesense", "__item"];

/// The methods a map has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 17] = [
    "set", "get", "has", "delete", "clear", "clone", "__new", "__enum",
    "ownprops", "hasownprop", "defineprop", "deleteprop", "getownpropdesc",
    "hasprop", "hasmethod", "getmethod", "hasbase",
];

/// The names a map answers: its properties and its methods.
pub(crate) fn names() -> Vec<&'static str> {
    PROPERTIES.iter().chain(METHODS.iter()).copied().collect()
}

/// How the keys of a map are compared.
#[derive(Clone, Copy)]
enum CaseSense {
    /// The letters are different from their lowercase: the default.
    On,
    /// The letters are the same, whatever their case.
    Off,
    /// The letters are the same, by the rules of the locale.
    Locale,
}

/// The object a `Map` makes.
///
/// The pairs of the constructor go in, the way `Set` puts them in.
pub(crate) fn construct(
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    if arguments.len() % 2 != 0 {
        return Err(RuntimeFailure::Error(RuntimeError {
            message: "Invalid number of parameters.".into(),
            line,
        }));
    }
    let map = crate::classes::object_with("map", vec![(STATE, new_state(Vec::new(), CaseSense::On, 0))]);
    for pair in arguments.chunks(2) {
        set_pair(&map, &pair[0], pair[1].clone());
    }
    Ok(map)
}

/// A method of a `Map`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let method = method.to_lowercase();
    let failure = |message: &str| {
        Some(Err(RuntimeFailure::Error(RuntimeError { message: message.to_string(), line })))
    };
    match method.as_str() {
        // `Set` and `__New` are the same thing in v2: both put the pairs
        // in, and both answer the map, so that the calls can be chained.
        "set" | "__new" => {
            if arguments.len() % 2 != 0 {
                return failure("Invalid number of parameters.");
            }
            for pair in arguments.chunks(2) {
                set_pair(this, &pair[0], pair[1].clone());
            }
            Some(Ok(this.clone()))
        }
        "get" => {
            let Some(key) = arguments.first() else { return failure("Too few parameters passed to function.") };
            match find(this, key) {
                Some(value) => Some(Ok(value)),
                // The default of the call comes first, and then the default
                // the script gave to the map.
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
            let Some(key) = arguments.first() else { return failure("Too few parameters passed to function.") };
            Some(Ok(Value::Int(find(this, key).is_some() as i64)))
        }
        "delete" => {
            let Some(key) = arguments.first() else { return failure("Too few parameters passed to function.") };
            let Some(state) = state(this) else { return failure("Item has no value.") };
            let mut state = state.borrow_mut();
            let sense = case_sense(&state);
            let mut pairs = entries(&state);
            let Some(index) = pairs.iter().position(|(existing, _)| same_key(existing, key, sense)) else {
                return failure("Item has no value.");
            };
            let (_, value) = pairs.remove(index);
            set_entries(&mut state, pairs);
            Some(Ok(value))
        }
        "clear" => {
            if let Some(state) = state(this) {
                let mut state = state.borrow_mut();
                set_entries(&mut state, Vec::new());
            }
            Some(Ok(Value::Text(String::new())))
        }
        "clone" => Some(Ok(clone(this))),
        // The enumeration of a map is the keys, in the order the entries
        // were put in, and with two variables it is the key and the value
        // under it.
        "__enum" => {
            // The number of the variables the loop wants is the argument:
            // with one it wants the keys, with two the keys and the
            // values.
            let wanted = arguments.first().and_then(|value| value.to_number()).unwrap_or(0.0) as i64;
            let Some(state) = state(this) else { return Some(Ok(Value::List(Vec::new()))) };
            let state = state.borrow();
            let items: Vec<Value> = entries(&state)
                .into_iter()
                .map(|(key, value)| {
                    // The enumerator of a map gives the key, and the
                    // value under it when the loop wants two.
                    if wanted == 1 {
                        key
                    } else {
                        Value::List(vec![key, value])
                    }
                })
                .collect();
            Some(Ok(Value::List(items)))
        }
        _ => base_method(&method, this, arguments, line),
    }
}

/// Write an item of a map: `map[key] := value`.
///
/// The item of a map is its entry, and the map knows how to write one.
pub(crate) fn set_item(
    key: &Value,
    this: &Value,
    value: Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<(), RuntimeFailure>> {
    set_pair(this, key, value);
    Some(Ok(()))
}

/// A property of a `Map`.
pub(crate) fn property(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    _line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let state = state(this)?;
    let state = state.borrow();
    match name {
        "count" => Some(Ok(Value::Int(entries(&state).len() as i64))),
        "capacity" => Some(Ok(Value::Int(capacity(&state)))),
        "casesense" => Some(Ok(Value::Text(case_sense_text(case_sense(&state)).to_string()))),
        _ => None,
    }
}

/// A property of a `Map` that is written.
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
        "capacity" => {
            let wanted = match rt.integer(&value, line) {
                Ok(number) => number,
                Err(failure) => return Some(Err(failure)),
            };
            // The capacity of a map is a number of v2 that does not go
            // below zero: a value that is out of the range wraps around,
            // the way a number of thirty-two bits does.
            let wanted = wanted as u32 as i64;
            let state = state(this)?;
            let mut state = state.borrow_mut();
            // A capacity below the number of the entries is the number of
            // the entries: the map cannot hold less than it has.
            let count = entries(&state).len() as i64;
            set_capacity(&mut state, wanted.max(count));
            Some(Ok(()))
        }
        "casesense" => {
            let Some(sense) = case_sense_of(&value) else { return failure("Invalid value.") };
            let state = state(this)?;
            let mut state = state.borrow_mut();
            // The keys are compared as the map says, and the map has keys
            // already: the setting cannot change under them.
            if !entries(&state).is_empty() {
                return failure("Map must be empty");
            }
            set_case_sense(&mut state, sense);
            Some(Ok(()))
        }
        _ => None,
    }
}

/// The item of a `Map`: the `__Item` of the class.
///
/// The item of a key that is not there is an error, unless the script
/// gave the map a default.
pub(crate) fn item(
    key: &Value,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match find(this, key) {
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

/// The state of a map.
fn state(map: &Value) -> Option<Rc<RefCell<Object>>> {
    let Value::Object(object) = map else { return None };
    let state = object.borrow().properties.get(STATE)?.clone();
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// A new state: the entries, the way the keys are compared, and how many
/// of them fit.
fn new_state(pairs: Vec<(Value, Value)>, sense: CaseSense, capacity: i64) -> Value {
    let mut state = Object { class: STATE_CLASS.to_string(), properties: BTreeMap::new() };
    set_entries(&mut state, pairs);
    set_case_sense(&mut state, sense);
    set_capacity(&mut state, capacity);
    Value::Object(Rc::new(RefCell::new(state)))
}

/// The entries of a map: the pairs, in the order they were put in.
fn entries(state: &Object) -> Vec<(Value, Value)> {
    let Some(Value::List(items)) = state.properties.get("entries") else { return Vec::new() };
    items
        .iter()
        .map(|entry| match entry {
            Value::List(pair) if pair.len() == 2 => (pair[0].clone(), pair[1].clone()),
            _ => (Value::Text(String::new()), Value::Text(String::new())),
        })
        .collect()
}

/// The entries of a map, written back.
fn set_entries(state: &mut Object, pairs: Vec<(Value, Value)>) {
    let items: Vec<Value> = pairs
        .into_iter()
        .map(|(key, value)| Value::List(vec![key, value]))
        .collect();
    state.properties.insert("entries".to_string(), Value::List(items));
}

/// The way the keys of a map are compared.
fn case_sense(state: &Object) -> CaseSense {
    let setting = state.properties.get("casesense").map(|value| value.to_text()).unwrap_or_default();
    match setting.to_lowercase().as_str() {
        "off" => CaseSense::Off,
        "locale" => CaseSense::Locale,
        _ => CaseSense::On,
    }
}

/// The way the keys of a map are compared, written.
fn set_case_sense(state: &mut Object, sense: CaseSense) {
    state.properties.insert("casesense".to_string(), Value::Text(case_sense_text(sense).to_string()));
}

/// How many entries a map holds before it has to grow.
fn capacity(state: &Object) -> i64 {
    state
        .properties
        .get("capacity")
        .and_then(|value| value.to_number())
        .map(|number| number as i64)
        .unwrap_or(0)
}

/// How many entries a map holds before it has to grow, written.
fn set_capacity(state: &mut Object, capacity: i64) {
    state.properties.insert("capacity".to_string(), Value::Int(capacity));
}

/// The way the keys are compared, as the script reads it.
fn case_sense_text(sense: CaseSense) -> &'static str {
    match sense {
        CaseSense::On => "On",
        CaseSense::Off => "Off",
        CaseSense::Locale => "Locale",
    }
}

/// The way the keys are compared, from the value the script wrote.
fn case_sense_of(value: &Value) -> Option<CaseSense> {
    match value {
        Value::Int(1) => Some(CaseSense::On),
        Value::Int(0) => Some(CaseSense::Off),
        other => {
            let setting = other.to_text().to_lowercase();
            match setting.as_str() {
                "on" => Some(CaseSense::On),
                "off" => Some(CaseSense::Off),
                "locale" => Some(CaseSense::Locale),
                _ => None,
            }
        }
    }
}

/// The value of a key, when the map has it.
fn find(map: &Value, key: &Value) -> Option<Value> {
    let state = state(map)?;
    let state = state.borrow();
    let sense = case_sense(&state);
    entries(&state)
        .into_iter()
        .find(|(existing, _)| same_key(existing, key, sense))
        .map(|(_, value)| value)
}

/// The default of a map: the value the script gave to the `Default`
/// property, when it gave one.
fn default_property(map: &Value) -> Option<Value> {
    let Value::Object(object) = map else { return None };
    object.borrow().properties.get("default").cloned()
}

/// Put a value in, under a key.
///
/// A key that is already there keeps its place: the order of the entries
/// is the order they were put in.
fn set_pair(map: &Value, key: &Value, value: Value) {
    let Some(state) = state(map) else { return };
    let mut state = state.borrow_mut();
    let sense = case_sense(&state);
    let mut pairs = entries(&state);
    match pairs.iter().position(|(existing, _)| same_key(existing, key, sense)) {
        Some(index) => pairs[index].1 = value,
        None => pairs.push((key.clone(), value)),
    }
    let count = pairs.len() as i64;
    set_entries(&mut state, pairs);
    // The map grows to hold what is put in it.
    if count > capacity(&state) {
        set_capacity(&mut state, count);
    }
}

/// A copy of a map: the same entries, in the same order, and the same
/// properties.
///
/// The values are copied the way a script copies them: a value that is
/// an object is the same object in the copy.
fn clone(map: &Value) -> Value {
    let (pairs, sense, capacity) = match state(map) {
        Some(state) => {
            let state = state.borrow();
            (entries(&state), case_sense(&state), capacity(&state))
        }
        None => (Vec::new(), CaseSense::On, 0),
    };
    let mut properties = BTreeMap::new();
    properties.insert(STATE.to_string(), new_state(pairs, sense, capacity));
    // The properties the script gave the map are copied as well.
    if let Value::Object(object) = map {
        for (name, value) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), value.clone());
            }
        }
    }
    Value::Object(Rc::new(RefCell::new(Object { class: "map".to_string(), properties })))
}

// -- the keys -----------------------------------------------------------------

/// Whether two keys are the same key.
///
/// The rules are the ones of v2: a string key is compared with its case
/// as the map says, an integer key with its number, an object key with
/// its address, and a float key is a string key before it is compared.
fn same_key(left: &Value, right: &Value, sense: CaseSense) -> bool {
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => Rc::ptr_eq(left, right),
        (Value::Object(_), _) | (_, Value::Object(_)) => false,
        (Value::Float(_), _) | (_, Value::Float(_)) => same_text(&left.to_text(), &right.to_text(), sense),
        (Value::Int(left), Value::Int(right)) => left == right,
        (Value::Text(left), Value::Text(right)) => same_text(left, right, sense),
        // A string key is never an integer key.
        _ => false,
    }
}

/// Two keys as text, with the case as the map says.
fn same_text(left: &str, right: &str, sense: CaseSense) -> bool {
    match sense {
        CaseSense::On => left == right,
        CaseSense::Off => left.eq_ignore_ascii_case(right),
        CaseSense::Locale => left.to_lowercase() == right.to_lowercase(),
    }
}

// -- the base object ----------------------------------------------------------

/// The methods of the base object, answered for a map.
///
/// The runtime answers them for the classes of the script, and it knows
/// nothing of the class types of the runtime: a map has no method of the
/// script, and the entries of a map are not properties of it.
fn base_method(
    method: &str,
    this: &Value,
    arguments: &[Value],
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
            let names: Vec<Value> = object
                .borrow()
                .properties
                .iter()
                .filter(|(name, _)| name.as_str() != STATE)
                .map(|(name, _)| Value::Text(name.clone()))
                .collect();
            Some(Ok(Value::List(names)))
        }
        "hasownprop" => Some(Ok(Value::Int(own_property(object, &key) as i64))),
        "hasprop" => Some(Ok(Value::Int(
            (PROPERTIES.contains(&key.as_str()) || own_property(object, &key)) as i64,
        ))),
        "hasmethod" => Some(Ok(Value::Int(METHODS.contains(&key.as_str()) as i64))),
        // A map extends the base object, and nothing else.
        "hasbase" => Some(Ok(Value::Int((key == "object") as i64))),
        "getownpropdesc" => {
            let value = object.borrow().properties.get(&key).cloned();
            Some(Ok(match value {
                Some(value) => {
                    let mut table = BTreeMap::new();
                    table.insert("value".to_string(), value);
                    Value::Object(Rc::new(RefCell::new(Object {
                        class: "property".to_string(),
                        properties: table,
                    })))
                }
                None => Value::Object(Rc::new(RefCell::new(Object {
                    class: "property".to_string(),
                    properties: BTreeMap::new(),
                }))),
            }))
        }
        "defineprop" => {
            if key == STATE {
                return failure("the state of a map is not a property of it.");
            }
            let value = arguments.get(1).cloned().unwrap_or(Value::Text(String::new()));
            object.borrow_mut().properties.insert(key, value);
            Some(Ok(Value::Int(1)))
        }
        "deleteprop" => {
            if key == STATE {
                return Some(Ok(Value::Int(0)));
            }
            let gone = object.borrow_mut().properties.remove(&key);
            Some(Ok(Value::Int(gone.is_some() as i64)))
        }
        // A method of a class type of the runtime is not a value this
        // runtime carries.
        "getmethod" => failure("this runtime does not carry the method of a class type as a value."),
        _ => None,
    }
}

/// Whether the map has a property of that name.
///
/// The state of the map is not a property of it: it is the inside of the
/// map, and the script never wrote it.
fn own_property(object: &Rc<RefCell<Object>>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}
