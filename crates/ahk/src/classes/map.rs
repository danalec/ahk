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

use crate::classes::enumerator;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};

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
    "set",
    "get",
    "has",
    "delete",
    "clear",
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
    let map = crate::classes::object_with(
        "map",
        vec![(STATE, new_state(Vec::new(), CaseSense::On, 0))],
    );
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
    // The member that the KeySharp extension adds: none of its names is
    // a method of the v2.
    if rt
        .dialect()
        .carries_the_member_of_the_extension("map", &method)
    {
        if method == "tostring" {
            return Some(the_extension_tostring(this, rt, line));
        }
        return None;
    }
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
            let Some(key) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            match find(this, key) {
                Some(value) => Some(Ok(value)),
                // The default of the call comes first, and then the default
                // the script gave to the map.
                None => match arguments.get(1) {
                    Some(default) => Some(Ok(default.clone())),
                    None => match default_property(this) {
                        Some(default) => Some(Ok(default)),
                        None => failure_with("Item has no value.", &key.to_text()),
                    },
                },
            }
        }
        "has" => {
            let Some(key) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            Some(Ok(Value::Int(find(this, key).is_some() as i64)))
        }
        "delete" => {
            let Some(key) = arguments.first() else {
                return failure("Too few parameters passed to function.");
            };
            let Some(state) = state(this) else {
                return failure("Item has no value.");
            };
            let mut state = state.borrow_mut();
            let sense = case_sense(&state);
            let mut pairs = entries(&state);
            let Some(index) = pairs
                .iter()
                .position(|(existing, _)| same_key(existing, key, sense))
            else {
                return failure_with("Item has no value.", &key.to_text());
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
        // under it. Measured on the binary of v2.0.19 on 2026-10-31: the
        // enumeration of v2 is an enumerator, and it gives the key when
        // the caller wants one value.
        "__enum" => {
            let Some(state) = state(this) else {
                return Some(Ok(enumerator::make(Vec::new(), enumerator::Alone::TheKey)));
            };
            let state = state.borrow();
            Some(Ok(enumerator::make(
                entries(&state),
                enumerator::Alone::TheKey,
            )))
        }
        _ => base_method(&method, this, arguments, rt, line),
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
        "casesense" => Some(Ok(Value::Text(
            case_sense_text(case_sense(&state)).to_string(),
        ))),
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
            let Some(sense) = case_sense_of(&value) else {
                return failure_with("Invalid value.", &value.to_text());
            };
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
            None => Some(Err(crate::classes::the_failure_with(
                "Item has no value.",
                &key.to_text(),
                line,
            ))),
        },
    }
}

// -- the state ----------------------------------------------------------------

/// The state of a map.
fn state(map: &Value) -> Option<Rc<RefCell<Object>>> {
    let Value::Object(object) = map else {
        return None;
    };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// A new state: the entries, the way the keys are compared, and how many
/// of them fit.
fn new_state(pairs: Vec<(Value, Value)>, sense: CaseSense, capacity: i64) -> Value {
    let mut state = Object {
        class: STATE_CLASS.to_string(),
        properties: BTreeMap::new(),
    };
    set_entries(&mut state, pairs);
    set_case_sense(&mut state, sense);
    set_capacity(&mut state, capacity);
    Value::Object(Rc::new(RefCell::new(state)))
}

/// The entries of a map: the pairs, in the order they were put in.
fn entries(state: &Object) -> Vec<(Value, Value)> {
    let Some(Value::List(items)) = state.value_of("entries") else {
        return Vec::new();
    };
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
    put(&mut state.properties, "entries", Value::List(items));
}

/// The way the keys of a map are compared.
fn case_sense(state: &Object) -> CaseSense {
    let setting = state
        .value_of("casesense")
        .map(|value| value.to_text())
        .unwrap_or_default();
    match setting.to_lowercase().as_str() {
        "off" => CaseSense::Off,
        "locale" => CaseSense::Locale,
        _ => CaseSense::On,
    }
}

/// The way the keys of a map are compared, written.
fn set_case_sense(state: &mut Object, sense: CaseSense) {
    put(
        &mut state.properties,
        "casesense",
        Value::Text(case_sense_text(sense).to_string()),
    );
}

/// How many entries a map holds before it has to grow.
fn capacity(state: &Object) -> i64 {
    state
        .value_of("capacity")
        .and_then(|value| value.to_number())
        .map(|number| number as i64)
        .unwrap_or(0)
}

/// How many entries a map holds before it has to grow, written.
fn set_capacity(state: &mut Object, capacity: i64) {
    put(&mut state.properties, "capacity", Value::Int(capacity));
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
    let Value::Object(object) = map else {
        return None;
    };
    object.borrow().value_of("default")
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
    match pairs
        .iter()
        .position(|(existing, _)| same_key(existing, key, sense))
    {
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
    put(&mut properties, STATE, new_state(pairs, sense, capacity));
    // The properties the script gave the map are copied as well.
    if let Value::Object(object) = map {
        for (name, property) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), property.clone());
            }
        }
    }
    Value::Object(Rc::new(RefCell::new(Object {
        class: "map".to_string(),
        properties,
    })))
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
        (Value::Float(_), _) | (_, Value::Float(_)) => {
            same_text(&left.to_text(), &right.to_text(), sense)
        }
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
    rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let Value::Object(object) = this else {
        return None;
    };
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
            let name = arguments
                .first()
                .map(|value| value.to_text())
                .unwrap_or_default();
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
                return failure("the state of a map is not a property of it.");
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
            let gone = object
                .borrow_mut()
                .properties
                .remove(&key)
                .map(|property| property.value);
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

/// Whether the map has a property of that name.
///
/// The state of the map is not a property of it: it is the inside of the
/// map, and the script never wrote it.
fn own_property(object: &Rc<RefCell<Object>>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}

/// The pairs of a map, in the order they were put in.
///
/// The built-ins that take a Map of options read it through here: the
/// entries are the data of the class, and only the module of the map
/// touches them. The answer is nothing when the value is not a map.
pub(crate) fn the_pairs(this: &Value) -> Option<Vec<(Value, Value)>> {
    let state = state(this)?;
    Some(entries(&state.borrow()))
}

/// The text the `ToString` of a `Map` is written with.
///
/// The documentation of KeySharp writes it as `[key: value, ...]`, and says
/// the keys and the values are formatted the way `Array.ToString()` formats
/// its elements: a text between quotes, a place without a value as `unset`.
fn the_extension_tostring(
    this: &Value,
    rt: &mut Runtime,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let Some(state) = state(this) else {
        return Ok(Value::Text("[]".to_string()));
    };
    let entries = entries(&state.borrow());
    let mut out = String::new();
    out.push('[');
    for (index, (key, value)) in entries.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&super::array::the_text_of_a_value(key, rt, line));
        out.push_str(": ");
        out.push_str(&super::array::the_text_of_a_value(value, rt, line));
    }
    out.push(']');
    Ok(Value::Text(out))
}
