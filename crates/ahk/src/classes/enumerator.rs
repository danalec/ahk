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
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};
use crate::runtime::SharedCell;

/// The key the state of an enumerator lives under.
///
/// The name is not one a script can write, and it is not a property of the
/// enumerator either, so it stays out of the answers about the
/// properties.
const STATE: &str = "enumerator state";

/// The name of the list of the pairs of an enumerator.
const PAIRS: &str = "pairs";

/// The name of the place of the pair that the enumeration gives next.
///
/// The pairs of the enumerator stay where they are, and the place of the
/// next one is what moves: the enumeration of many pairs is a walk of the
/// list of them, and not a copy of the list at every turn.
const PLACE: &str = "at";

/// The class of the object that carries the state of an enumerator.
const STATE_CLASS: &str = "enumerator-state";

/// The name of the collection that an enumerator walks.
///
/// The collection is the source of the pairs, and not a copy of them: the
/// enumeration of it is live, and the entries that the loop adds are seen
/// by it.
const SOURCE: &str = "source";

/// The name of the kind of the collection that an enumerator walks.
const SOURCE_KIND: &str = "source kind";

/// The kinds of the collection that an enumerator walks.
const AN_ARRAY: i64 = 0;
const A_MAP: i64 = 1;

/// The properties an enumerator owns: the ones of a function.
const PROPERTIES: [&str; 5] = ["name", "minparams", "maxparams", "isbuiltin", "isvariadic"];

/// The methods an enumerator has: the ones of the class, and the ones every
/// object has.
const METHODS: [&str; 13] = [
    "call",
    "isoptional",
    "isbyref",
    "bind",
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

/// Which of the two of a pair is given when the caller wants one value.
///
/// Measured on the binary of v2.0.19: the enumerator of an
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
    let mut state = Object {
        class: STATE_CLASS.to_string(),
        properties: BTreeMap::new(),
    };
    set_pairs(&mut state, pairs);
    put(
        &mut state.properties,
        "alone",
        Value::Int(match alone {
            Alone::TheKey => 0,
            Alone::TheValue => 1,
        }),
    );
    put(&mut state.properties, PLACE, Value::Int(0));
    let mut table = BTreeMap::new();
    put(
        &mut table,
        STATE,
        Value::Object(SharedCell::new(state)),
    );
    Value::Object(SharedCell::new(Object {
        class: "enumerator".to_string(),
        properties: table,
    }))
}

/// Make an enumerator that walks the entries of a map.
///
/// The enumerator walks the map itself, and not a copy of it: the entries
/// that the body of the loop adds to it are seen by the enumeration, the
/// way the v2 sees them.
pub(crate) fn make_of_a_map(
    cell: SharedCell<crate::classes::map::MapData>,
    alone: Alone,
) -> Value {
    make_of_a_collection(Value::MapData(cell), A_MAP, alone)
}

/// Make an enumerator that walks the items of an array.
///
/// The enumerator walks the array itself, and not a copy of it: the items
/// that the body of the loop adds to it are seen by the enumeration, the
/// way the v2 sees them.
pub(crate) fn make_of_an_array(cell: SharedCell<Object>, alone: Alone) -> Value {
    make_of_a_collection(Value::Object(cell), AN_ARRAY, alone)
}

/// Make an enumerator over a collection that it walks.
fn make_of_a_collection(source: Value, kind: i64, alone: Alone) -> Value {
    let mut state = Object {
        class: STATE_CLASS.to_string(),
        properties: BTreeMap::new(),
    };
    put(&mut state.properties, SOURCE, source);
    put(&mut state.properties, SOURCE_KIND, Value::Int(kind));
    put(
        &mut state.properties,
        "alone",
        Value::Int(match alone {
            Alone::TheKey => 0,
            Alone::TheValue => 1,
        }),
    );
    put(&mut state.properties, PLACE, Value::Int(0));
    let mut table = BTreeMap::new();
    put(&mut table, STATE, Value::Object(SharedCell::new(state)));
    Value::Object(SharedCell::new(Object {
        class: "enumerator".to_string(),
        properties: table,
    }))
}

/// Whether a value is an enumerator.
pub(crate) fn is_one(value: &Value) -> bool {
    let Value::Object(object) = value else {
        return false;
    };
    object.borrow().class == "enumerator"
}

/// How many turns the enumerator has left.
pub(crate) fn the_count(this: &Value) -> i64 {
    let Some(state) = state(this) else { return 0 };
    let state = state.borrow();
    let place = the_place(&state);
    // The turns that are left are the entries of the collection that are
    // there from the place of the turn.
    if let Some(count) = the_count_of_the_collection(&state, place) {
        return count as i64;
    }
    let total = the_number_of_pairs(&state);
    total.saturating_sub(place) as i64
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
            let name = arguments
                .first()
                .map(|value| value.to_text())
                .unwrap_or_default();
            let key = name.to_lowercase();
            let answer = match this {
                Value::Object(object) => {
                    key != STATE && object.borrow().properties.contains_key(&key)
                }
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
        "name" => Value::Text("".into()),
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
/// Measured on the binary of v2.0.19: a turn that has no
/// value of its own leaves the variable of the caller without one: the
/// script that reads it is told that the variable has no value.
fn the_call(
    this: &Value,
    arguments: &[Value],
    rt: &mut Runtime,
    _line: u32,
) -> Result<Value, RuntimeFailure> {
    let Some((key, value, alone)) = the_pair(this) else {
        return Ok(Value::Int(0));
    };
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
fn state(enumerator: &Value) -> Option<SharedCell<Object>> {
    let Value::Object(object) = enumerator else {
        return None;
    };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// The pairs of an enumerator, written back.
fn set_pairs(state: &mut Object, pairs: Vec<(Value, Value)>) {
    let items: Vec<Value> = pairs
        .into_iter()
        .map(|(key, value)| Value::List(vec![key, value]))
        .collect();
    put(&mut state.properties, PAIRS, Value::List(items));
}

/// The number of the pairs of an enumerator, without a copy of them.
fn the_number_of_pairs(state: &Object) -> usize {
    match state.value_ref(PAIRS) {
        Some(Value::List(items)) => items.len(),
        _ => 0,
    }
}

/// The one of the two of the pair that the enumerator gives alone.
fn the_alone_of(state: &Object) -> Alone {
    match state
        .value_ref("alone")
        .and_then(|value| value.to_number())
        .map(|number| number as i64)
    {
        Some(1) => Alone::TheValue,
        _ => Alone::TheKey,
    }
}

/// The place of the pair that the enumeration gives next.
fn the_place(state: &Object) -> usize {
    state
        .value_of(PLACE)
        .and_then(|value| value.to_number())
        .map(|number| number as usize)
        .unwrap_or(0)
}

/// The pair of a place of the list of the pairs.
///
/// The pair is read without a copy of the list: the list of many pairs is
/// not copied at every turn of the enumeration.
fn the_pair_at(state: &Object, place: usize) -> Option<(Value, Value)> {
    let Some(Value::List(items)) = state.value_ref(PAIRS) else {
        return None;
    };
    let item = items.get(place)?;
    Some(match item {
        Value::List(pair) if pair.len() == 2 => (pair[0].clone(), pair[1].clone()),
        _ => (item.clone(), Value::Text("".into())),
    })
}

/// The pairs of an enumerator that it has not given yet.
///
/// The pairs are read without a copy of the list of them: the owner of the
/// list asks for it when it needs it, and not at every turn.
pub(crate) fn the_pairs_of(enumerator: &Value) -> Vec<(Value, Value)> {
    let Some(state) = state(enumerator) else {
        return Vec::new();
    };
    let state = state.borrow();
    let place = the_place(&state);
    // The pairs of a collection are the entries of it that are there from
    // the place of the turn.
    if the_source_of(&state).is_some() {
        let mut pairs = Vec::new();
        let mut at = place;
        while let Some((key, value, next)) = the_pair_of_the_collection(&state, at) {
            pairs.push((key, value));
            at = next;
        }
        return pairs;
    }
    let Some(Value::List(items)) = state.value_ref(PAIRS) else {
        return Vec::new();
    };
    items
        .iter()
        .skip(place)
        .map(|item| match item {
            Value::List(pair) if pair.len() == 2 => (pair[0].clone(), pair[1].clone()),
            _ => (item.clone(), Value::Text("".into())),
        })
        .collect()
}

/// The collection that an enumerator walks, when it walks one.
fn the_source_of(state: &Object) -> Option<&Value> {
    state.value_ref(SOURCE)
}

/// The pair of the turn out of the collection that the enumerator walks, and
/// the place of the next one.
///
/// The collection is the source of the pairs: the enumeration of it is live,
/// and the entries that the body of the loop adds are seen by it.
fn the_pair_of_the_collection(
    state: &Object,
    place: usize,
) -> Option<(Value, Value, usize)> {
    let kind = state.value_ref(SOURCE_KIND)?.to_number()? as i64;
    match kind {
        A_MAP => {
            let Some(Value::MapData(cell)) = the_source_of(state) else {
                return None;
            };
            crate::classes::map::the_pair_at(cell, place)
        }
        AN_ARRAY => {
            let Some(Value::Object(cell)) = the_source_of(state) else {
                return None;
            };
            crate::classes::array::the_item_at(cell, place)
        }
        _ => None,
    }
}

/// The number of the entries of the collection that are there from the place
/// of the turn.
fn the_count_of_the_collection(state: &Object, place: usize) -> Option<usize> {
    let kind = state.value_ref(SOURCE_KIND)?.to_number()? as i64;
    match kind {
        A_MAP => {
            let Some(Value::MapData(cell)) = the_source_of(state) else {
                return None;
            };
            Some(crate::classes::map::the_count_of_the_entries(cell, place))
        }
        AN_ARRAY => {
            let Some(Value::Object(cell)) = the_source_of(state) else {
                return None;
            };
            Some(crate::classes::array::the_count_of_the_items(cell, place))
        }
        _ => None,
    }
}

/// The pair of the turn, and which of the two is given alone.
fn the_pair(this: &Value) -> Option<(Value, Value, Alone)> {
    let state = state(this)?;
    let mut state = state.borrow_mut();
    let place = the_place(&state);
    // The pairs of a collection come out of the collection itself.
    if let Some((key, value, next)) = the_pair_of_the_collection(&state, place) {
        put(&mut state.properties, PLACE, Value::Int(next as i64));
        let alone = the_alone_of(&state);
        return Some((key, value, alone));
    }
    // The pair of the turn is read out of the list of the pairs, and the
    // list is not copied: the enumeration of many pairs is a walk of the
    // list, and not a copy of it at every turn.
    let (key, value) = the_pair_at(&state, place)?;
    // The place of the next pair is what moves.
    put(&mut state.properties, PLACE, Value::Int(place as i64 + 1));
    let alone = match state.value_of("alone").and_then(|value| value.to_number()) {
        Some(number) if number >= 1.0 => Alone::TheValue,
        _ => Alone::TheKey,
    };
    Some((key, value, alone))
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;
    use alloc::format;

    /// The turns of an enumerator come in the order of the pairs, and the
    /// count of them goes down at every turn.
    ///
    /// The list of the pairs stays where it is, and the place of the next
    /// pair is what moves: the enumeration of many pairs is a walk of the
    /// list, and not a copy of it at every turn.
    #[test]
    fn the_turns_of_an_enumerator_come_in_the_order_of_the_pairs() {
        let pairs: Vec<(Value, Value)> = (1..=5)
            .map(|number| (Value::Int(number), Value::Text((format!("v{number}")).into())))
            .collect();
        let enumerator = make(pairs, Alone::TheValue);
        assert_eq!(the_count(&enumerator), 5);
        for number in 1..=5 {
            let turn = the_turn(&enumerator, 2).expect("the enumerator has a turn");
            let Value::List(pair) = turn else {
                panic!("the turn of the enumerator is not a pair: {turn:?}");
            };
            assert_eq!(pair[0], Value::Int(number));
            assert_eq!(pair[1], Value::Text((format!("v{number}")).into()));
            assert_eq!(the_count(&enumerator), 5 - number);
        }
        assert!(the_turn(&enumerator, 2).is_none());
        assert_eq!(the_count(&enumerator), 0);
    }

    /// The pairs of an enumerator that it has not given yet are the ones
    /// that are left, and the ones that it gave are not there any more.
    #[test]
    fn the_pairs_of_an_enumerator_are_the_ones_that_are_left() {
        let pairs: Vec<(Value, Value)> = (1..=4)
            .map(|number| (Value::Int(number), Value::Int(number * 10)))
            .collect();
        let enumerator = make(pairs, Alone::TheKey);
        assert_eq!(the_pairs_of(&enumerator).len(), 4);
        let _ = the_turn(&enumerator, 2);
        let left = the_pairs_of(&enumerator);
        assert_eq!(left.len(), 3);
        assert_eq!(left[0].0, Value::Int(2));
        assert_eq!(left[2].0, Value::Int(4));
    }

    /// The enumerator that gives the key alone gives the key of the turn.
    #[test]
    fn the_enumerator_that_gives_the_key_alone_gives_the_key_of_the_turn() {
        let enumerator = make(
            vec![(Value::Int(1), Value::Text("a".into())), (Value::Int(2), Value::Text("b".into()))],
            Alone::TheKey,
        );
        assert_eq!(the_turn(&enumerator, 1), Some(Value::Int(1)));
        assert_eq!(the_turn(&enumerator, 1), Some(Value::Int(2)));
        assert_eq!(the_turn(&enumerator, 1), None);
    }
}
