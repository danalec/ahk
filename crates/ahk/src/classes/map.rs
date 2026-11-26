//! The `Map` class.
//!
//! A map is a table of keys and values. The key may be any value, and
//! the order of the entries is the order they were put in.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::classes::enumerator;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, SharedCell, Value, put};

#[cfg(feature = "std")]
type KeyMap<K> = std::collections::HashMap<K, usize, crate::fast_hash::FastHasherBuilder>;
#[cfg(not(feature = "std"))]
type KeyMap<K> = alloc::collections::BTreeMap<K, usize>;

/// The key the state of a map lives under.
///
/// The name is not one a script can write: the name of a property in a
/// script is always an identifier, and this one is not. It is not a
/// property of the map either, so it stays out of the answers about the
/// properties.
const STATE: &str = "map state";

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CaseSense {
    /// The letters are different from their lowercase: the default.
    On,
    /// The letters are the same, whatever their case.
    Off,
    /// The letters are the same, by the rules of the locale.
    Locale,
}

/// Backing O(1) indexed storage for a `Map`, preserving insertion order.
#[derive(Debug, Clone, PartialEq)]
pub struct MapData {
    entries: Vec<Option<(Value, Value)>>,
    live_count: usize,
    int_index: KeyMap<i64>,
    text_index: KeyMap<crate::runtime::Text>,
    obj_index: KeyMap<usize>,
    sense: CaseSense,
    capacity: i64,
}

impl MapData {
    fn new(sense: CaseSense, capacity: i64) -> Self {
        Self {
            entries: Vec::new(),
            live_count: 0,
            int_index: KeyMap::default(),
            text_index: KeyMap::default(),
            obj_index: KeyMap::default(),
            sense,
            capacity,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.live_count
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.live_count == 0
    }

    #[inline]
    pub(crate) fn get_int(&self, key: i64) -> Option<Value> {
        let slot = *self.int_index.get(&key)?;
        self.entries[slot].as_ref().map(|(_, v)| v.clone())
    }

    #[inline]
    pub(crate) fn get_value(&self, key: &Value) -> Option<Value> {
        self.get(key)
    }

    #[inline]
    pub(crate) fn set_int(&mut self, key: i64, value: Value) {
        #[cfg(feature = "std")]
        {
            use std::collections::hash_map::Entry;
            match self.int_index.entry(key) {
                Entry::Occupied(occ) => {
                    let slot = *occ.get();
                    if let Some(entry) = self.entries[slot].as_mut() {
                        entry.1 = value;
                        return;
                    }
                }
                Entry::Vacant(vac) => {
                    let slot = self.entries.len();
                    vac.insert(slot);
                    self.entries.push(Some((Value::Int(key), value)));
                    self.live_count += 1;
                    let count = self.live_count as i64;
                    if count > self.capacity {
                        self.capacity = count;
                    }
                    return;
                }
            }
        }
        self.set(&Value::Int(key), value);
    }

    #[inline]
    pub(crate) fn set_value(&mut self, key: &Value, value: Value) {
        self.set(key, value);
    }

    fn lookup_slot(&self, key: &Value) -> Option<usize> {
        match key {
            Value::Int(n) => self.int_index.get(n).copied(),
            Value::Object(obj) => self.obj_index.get(&obj.identity()).copied(),
            Value::Text(s) => match self.sense {
                CaseSense::On => self.text_index.get(s.as_str()).copied(),
                CaseSense::Off => {
                    let lower = crate::runtime::the_borrowed_key(s);
                    self.text_index.get(&*lower).copied()
                }
                CaseSense::Locale => self.text_index.get(s.to_lowercase().as_str()).copied(),
            },
            Value::Float(_) => {
                let s = key.to_text();
                match self.sense {
                    CaseSense::On => self.text_index.get(s.as_str()).copied(),
                    CaseSense::Off => {
                        let lower = crate::runtime::the_borrowed_key(&s);
                        self.text_index.get(&*lower).copied()
                    }
                    CaseSense::Locale => self.text_index.get(s.to_lowercase().as_str()).copied(),
                }
            }
            _ => None,
        }
    }

    fn get(&self, key: &Value) -> Option<Value> {
        let slot = self.lookup_slot(key)?;
        self.entries[slot].as_ref().map(|(_, v)| v.clone())
    }

    fn set(&mut self, key: &Value, value: Value) {
        if let Some(slot) = self.lookup_slot(key) {
            if let Some(entry) = self.entries[slot].as_mut() {
                entry.1 = value;
                return;
            }
        }
        let slot = self.entries.len();
        match key {
            Value::Int(n) => {
                self.int_index.insert(*n, slot);
            }
            Value::Object(obj) => {
                self.obj_index.insert(obj.identity(), slot);
            }
            Value::Text(s) => {
                let norm = match self.sense {
                    CaseSense::On => s.clone(),
                    CaseSense::Off => s.to_ascii_lowercase().into(),
                    CaseSense::Locale => s.to_lowercase().into(),
                };
                self.text_index.insert(norm, slot);
            }
            Value::Float(_) => {
                let s = key.to_text();
                let norm = match self.sense {
                    CaseSense::On => s.into(),
                    CaseSense::Off => s.to_ascii_lowercase().into(),
                    CaseSense::Locale => s.to_lowercase().into(),
                };
                self.text_index.insert(norm, slot);
            }
            _ => {}
        }
        self.entries.push(Some((key.clone(), value)));
        self.live_count += 1;
        let count = self.live_count as i64;
        if count > self.capacity {
            self.capacity = count;
        }
    }

    fn delete(&mut self, key: &Value) -> Option<Value> {
        let slot = match key {
            Value::Int(n) => self.int_index.remove(n),
            Value::Object(obj) => self.obj_index.remove(&obj.identity()),
            Value::Text(s) => match self.sense {
                CaseSense::On => self.text_index.remove(s.as_str()),
                CaseSense::Off => {
                    let lower = crate::runtime::the_borrowed_key(s);
                    self.text_index.remove(&*lower)
                }
                CaseSense::Locale => self.text_index.remove(s.to_lowercase().as_str()),
            },
            Value::Float(_) => {
                let s = key.to_text();
                match self.sense {
                    CaseSense::On => self.text_index.remove(s.as_str()),
                    CaseSense::Off => {
                        let lower = crate::runtime::the_borrowed_key(&s);
                        self.text_index.remove(&*lower)
                    }
                    CaseSense::Locale => self.text_index.remove(s.to_lowercase().as_str()),
                }
            }
            _ => None,
        }?;
        let (_, value) = self.entries[slot].take()?;
        self.live_count -= 1;
        if self.live_count == 0 {
            self.entries.clear();
        } else if self.entries.len() > 64 && self.live_count * 4 < self.entries.len() {
            self.compact();
        }
        Some(value)
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.live_count = 0;
        self.int_index.clear();
        self.text_index.clear();
        self.obj_index.clear();
    }

    fn compact(&mut self) {
        let mut compacted = Vec::with_capacity(self.live_count);
        self.int_index.clear();
        self.text_index.clear();
        self.obj_index.clear();
        for entry in self.entries.drain(..).flatten() {
            let slot = compacted.len();
            match &entry.0 {
                Value::Int(n) => {
                    self.int_index.insert(*n, slot);
                }
                Value::Object(obj) => {
                    self.obj_index.insert(obj.identity(), slot);
                }
                Value::Text(s) => {
                    let norm = match self.sense {
                        CaseSense::On => s.clone(),
                        CaseSense::Off => s.to_ascii_lowercase().into(),
                        CaseSense::Locale => s.to_lowercase().into(),
                    };
                    self.text_index.insert(norm, slot);
                }
                Value::Float(_) => {
                    let s = entry.0.to_text();
                    let norm = match self.sense {
                        CaseSense::On => s.into(),
                        CaseSense::Off => s.to_ascii_lowercase().into(),
                        CaseSense::Locale => s.to_lowercase().into(),
                    };
                    self.text_index.insert(norm, slot);
                }
                _ => {}
            }
            compacted.push(Some(entry));
        }
        self.entries = compacted;
    }

    fn pairs(&self) -> Vec<(Value, Value)> {
        let mut out = Vec::with_capacity(self.live_count);
        for entry in self.entries.iter().flatten() {
            out.push(entry.clone());
        }
        out
    }
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
        vec![(STATE, Value::MapData(SharedCell::new(MapData::new(CaseSense::On, 0))))],
    );
    if !arguments.is_empty() {
        if let Some(cell) = state(&map) {
            let mut data = cell.borrow_mut();
            for pair in arguments.chunks(2) {
                data.set(&pair[0], pair[1].clone());
            }
        }
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
            if let Some(cell) = state(this) {
                let mut data = cell.borrow_mut();
                for pair in arguments.chunks(2) {
                    data.set(&pair[0], pair[1].clone());
                }
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
            let Some(cell) = state(this) else {
                return failure("Item has no value.");
            };
            let mut data = cell.borrow_mut();
            let Some(value) = data.delete(key) else {
                return failure_with("Item has no value.", &key.to_text());
            };
            Some(Ok(value))
        }
        "clear" => {
            if let Some(cell) = state(this) {
                cell.borrow_mut().clear();
            }
            Some(Ok(Value::Text("".into())))
        }
        "clone" => Some(Ok(clone(this))),
        // The enumeration of a map is the keys, in the order the entries
        // were put in, and with two variables it is the key and the value
        // under it. Measured on the binary of v2.0.19: the
        // enumeration of v2 is an enumerator, and it gives the key when
        // the caller wants one value.
        "__enum" => {
            let Some(cell) = state(this) else {
                return Some(Ok(enumerator::make(Vec::new(), enumerator::Alone::TheKey)));
            };
            // The enumeration is live: the entries that the body of the
            // loop writes are seen by it, the way the v2 sees them.
            //
            // Measured on the binary of v2.0.32: the `for` over a map sees
            // the entry that the body of the loop writes to it.
            Some(Ok(enumerator::make_of_a_map(
                cell,
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
    let cell = state(this)?;
    let data = cell.borrow();
    match name {
        "count" => Some(Ok(Value::Int(data.live_count as i64))),
        "capacity" => Some(Ok(Value::Int(data.capacity))),
        "casesense" => Some(Ok(Value::Text(case_sense_text(data.sense).into()))),
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
            let cell = state(this)?;
            let mut data = cell.borrow_mut();
            // A capacity below the number of the entries is the number of
            // the entries: the map cannot hold less than it has.
            let count = data.live_count as i64;
            data.capacity = wanted.max(count);
            Some(Ok(()))
        }
        "casesense" => {
            let Some(sense) = case_sense_of(&value) else {
                return failure_with("Invalid value.", &value.to_text());
            };
            let cell = state(this)?;
            let mut data = cell.borrow_mut();
            // The keys are compared as the map says, and the map has keys
            // already: the setting cannot change under them.
            if data.live_count > 0 {
                return failure("Map must be empty");
            }
            data.sense = sense;
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

/// The pair of the place-th entry of a map that is there, and the place
/// of the next one.
///
/// The entries of a map are walked in the order of their places, and a
/// place whose entry is not there is not a turn of the walk.
pub(crate) fn the_pair_at(
    cell: &SharedCell<MapData>,
    place: usize,
) -> Option<(Value, Value, usize)> {
    let data = cell.borrow();
    let mut at = place;
    while at < data.entries.len() {
        if let Some(entry) = &data.entries[at] {
            return Some((entry.0.clone(), entry.1.clone(), at + 1));
        }
        at += 1;
    }
    None
}

/// The number of the entries of a map that are there from the place.
pub(crate) fn the_count_of_the_entries(cell: &SharedCell<MapData>, place: usize) -> usize {
    let data = cell.borrow();
    let mut count = 0;
    let mut at = place;
    while at < data.entries.len() {
        if data.entries[at].is_some() {
            count += 1;
        }
        at += 1;
    }
    count
}

/// The state of a map.
#[inline]
fn state(map: &Value) -> Option<SharedCell<MapData>> {
    let Value::Object(object) = map else {
        return None;
    };
    match object.borrow().value_ref(STATE)? {
        Value::MapData(data) => Some(data.clone()),
        _ => None,
    }
}

/// Backing `MapData` handle when `map` is a plain `Map` instance with no own
/// properties (`Default`, `__Item`, `__Enum`, `__Class`, or `DefineProp`
/// overrides).
#[inline]
pub(crate) fn plain_state(map: &Value) -> Option<SharedCell<MapData>> {
    let Value::Object(object) = map else {
        return None;
    };
    let borrowed = object.borrow();
    if borrowed.class != "map" || borrowed.properties.len() != 1 {
        return None;
    }
    match borrowed.value_ref(STATE)? {
        Value::MapData(data) => Some(data.clone()),
        _ => None,
    }
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
#[inline]
fn find(map: &Value, key: &Value) -> Option<Value> {
    let cell = state(map)?;
    cell.borrow().get(key)
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
#[inline]
fn set_pair(map: &Value, key: &Value, value: Value) {
    let Some(cell) = state(map) else { return };
    cell.borrow_mut().set(key, value);
}

/// A copy of a map: the same entries, in the same order, and the same
/// properties.
///
/// The values are copied the way a script copies them: a value that is
/// an object is the same object in the copy.
fn clone(map: &Value) -> Value {
    let cloned_data = match state(map) {
        Some(cell) => cell.borrow().clone(),
        None => MapData::new(CaseSense::On, 0),
    };
    let mut properties = BTreeMap::new();
    put(&mut properties, STATE, Value::MapData(SharedCell::new(cloned_data)));
    // The properties the script gave the map are copied as well.
    if let Value::Object(object) = map {
        for (name, property) in object.borrow().properties.iter() {
            if name != STATE {
                properties.insert(name.clone(), property.clone());
            }
        }
    }
    Value::Object(SharedCell::new(Object {
        class: "map".to_string(),
        properties,
    }))
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
                return failure("the state of a map is not a property of it.");
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

/// Whether the map has a property of that name.
///
/// The state of the map is not a property of it: it is the inside of the
/// map, and the script never wrote it.
fn own_property(object: &SharedCell<Object>, key: &str) -> bool {
    key != STATE && object.borrow().properties.contains_key(key)
}

/// The pairs of a map, in the order they were put in.
///
/// The built-ins that take a Map of options read it through here: the
/// entries are the data of the class, and only the module of the map
/// touches them. The answer is nothing when the value is not a map.
pub(crate) fn the_pairs(this: &Value) -> Option<Vec<(Value, Value)>> {
    let cell = state(this)?;
    Some(cell.borrow().pairs())
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
    let Some(cell) = state(this) else {
        return Ok(Value::Text("[]".into()));
    };
    let entries = cell.borrow().pairs();
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
    Ok(Value::Text(out.into()))
}
