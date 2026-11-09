//! The `RegExMatchInfo` class.
//!
//! The object that a `RegExMatch` writes into the variable of the caller:
//! the text the match was found in, the span and the name of every group,
//! and the mark of the pattern. It is made by the built-in and not by a
//! script -- v2 refuses the call of the class with "Invalid base." -- and
//! the script reads it by the place of a group, by the name of one, and by
//! the members `Count`, `Mark`, `Len`, `Pos` and `Name`.
//!
//! Measured on the binary of v2.0.19 on 2026-11-09, with the source of it
//! read beside the measurements:
//!
//! - `Count` is the number of the groups, the whole match not counted, and
//!   `Mark` is the name of the `(*MARK:...)` of the pattern -- the engine
//!   of the tree does not carry the mark, and the answer is empty.
//! - `m[0]` is the whole match and `m[n]` the group n; a group that took no
//!   part in the match is the empty text, its `Len` is 0 and its `Pos` is
//!   0. `m["name"]` and `m.name` are the group of that name, and the name
//!   is matched without regard to the case.
//! - `Len`, `Pos` and `Name` are read with a place or a name between the
//!   brackets -- `m.Len[2]` -- and read without one they answer the whole
//!   match; `Name` needs the place, and without it the read is refused.
//! - A place or a name that is not one is refused with "Parameter #1 of
//!   <member> is invalid." and the value in the `Extra`; a property that
//!   is not there is asked of `__Get`, which refuses it in the same words.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;

use crate::classes::enumerator;
use crate::runtime::{Object, Runtime, RuntimeError, RuntimeFailure, Value, put};

/// The key the state of a match lives under.
///
/// The key has a space in it, so that a script cannot reach the inside of
/// the object by the name of a property.
const STATE: &str = "regexmatchinfo state";

/// The class of the object that carries the state of a match.
const STATE_CLASS: &str = "regexmatchinfo-state";

/// The names a match answers: its properties and its methods.
const NAMES: [&str; 8] = [
    "__enum", "__get", "__item", "count", "len", "mark", "name", "pos",
];

/// The names of the members that are answered by the object itself.
pub(crate) fn names() -> Vec<&'static str> {
    NAMES.to_vec()
}

/// The object a match makes.
///
/// The spans are the characters of the text, counting from zero, and the
/// state carries them so that the script can read a group without the text
/// of the match being searched again.
pub(crate) fn of(
    haystack: &str,
    regex: &crate::regex::Regex,
    found: &crate::regex::Match,
) -> Value {
    let groups = regex.group_count();
    let mut spans: Vec<Value> = Vec::with_capacity(groups);
    let mut names: Vec<Value> = Vec::with_capacity(groups);
    for group in 0..groups {
        match found.span_of(group) {
            Some((start, end)) => spans.push(Value::List(vec![
                Value::Int(start as i64),
                Value::Int((end - start) as i64),
            ])),
            None => spans.push(Value::List(Vec::new())),
        }
        names.push(Value::Text(
            regex.name_of(group).unwrap_or("").to_string(),
        ));
    }
    let mut state = Object {
        class: STATE_CLASS.to_string(),
        properties: BTreeMap::new(),
    };
    put(&mut state.properties, "text", Value::Text(haystack.to_string()));
    put(&mut state.properties, "spans", Value::List(spans));
    put(&mut state.properties, "names", Value::List(names));
    put(
        &mut state.properties,
        "mark",
        Value::Text(String::new()),
    );
    let mut table = BTreeMap::new();
    put(
        &mut table,
        STATE,
        Value::Object(Rc::new(RefCell::new(state))),
    );
    Value::Object(Rc::new(RefCell::new(Object {
        class: "regexmatchinfo".to_string(),
        properties: table,
    })))
}

/// A property of a `RegExMatchInfo`.
///
/// `None` means the name is not one the object answers itself: the runtime
/// then looks for the method of the name, the base of the object, and the
/// class of the value, in the order of v2.
pub(crate) fn property(
    name: &str,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    match name.to_lowercase().as_str() {
        "count" => Some(count(this, line)),
        "mark" => Some(mark(this, line)),
        // The getter of the whole match: the members with a place between
        // the brackets answer in [`property_with_params`].
        "len" => Some(whole_span(this, "Len.Get", line).map(|(_, length)| Value::Int(length))),
        "pos" => Some(whole_span(this, "Pos.Get", line).map(|(place, _)| Value::Int(place))),
        // `Name` needs the place of a group, and the read without one is
        // refused the way v2 refuses it: the name of the getter in the
        // `Extra`, and nothing in the `What`.
        "name" => Some(Err(crate::classes::the_arity_failure(
            "regexmatchinfo",
            "Name.Get",
            "Too few parameters passed to function.",
            line,
        ))),
        // The names of the methods, the base of the object and the class of
        // the value are the business of the runtime, and it answers them
        // after this.
        "__enum" | "__get" | "__item" | "base" | "__class" => None,
        _ if crate::classes::the_base_object_has_the_method(&name.to_lowercase()) => None,
        // A property that is not one of the members is asked of the
        // `__Get` of v2, which answers the name of a group and refuses a
        // name that is not one.
        _ => Some(
            answer(this, "__Get", &Value::Text(name.to_string()), line)
                .map_err(|failure| named_for(failure, "__Get")),
        ),
    }
}

/// The value of a member of the class that is read with a parameter
/// between the brackets: `m.Len[2]`, `m.Pos["year"]` and `m.Name[1]`.
pub(crate) fn property_with_params(
    name: &str,
    params: &[Value],
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let member = match name.to_lowercase().as_str() {
        "len" => "Len.Get",
        "pos" => "Pos.Get",
        "name" => "Name.Get",
        _ => return None,
    };
    let key = params.first()?;
    Some(answer(this, member, key, line).map_err(|failure| named_for(failure, member)))
}

/// A method of a `RegExMatchInfo`.
pub(crate) fn call(
    method: &str,
    this: &Value,
    arguments: &[Value],
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    let method = method.to_lowercase();
    match method.as_str() {
        "__enum" => Some(enumeration(this, arguments, line)),
        // `__Get(Name, Params)`: the places between the brackets are the
        // parameters of the access, and a parameter is a use of v2 that the
        // match has not.
        "__get" => {
            let name = arguments
                .first()
                .cloned()
                .unwrap_or(Value::Text(String::new()));
            let params = arguments.get(1);
            if let Some(Value::List(items)) = params {
                if !items.is_empty() {
                    return Some(Err(crate::classes::the_failure_with(
                        "Invalid usage.",
                        "",
                        line,
                    )));
                }
            }
            Some(answer(this, "__Get", &name, line))
        }
        "len" => Some(answer(
            this,
            "Len.Get",
            arguments.first().unwrap_or(&Value::Int(0)),
            line,
        )),
        "pos" => Some(answer(
            this,
            "Pos.Get",
            arguments.first().unwrap_or(&Value::Int(0)),
            line,
        )),
        "name" => Some(answer(
            this,
            "Name.Get",
            arguments.first().unwrap_or(&Value::Int(0)),
            line,
        )),
        _ => None,
    }
}

/// The item of a `RegExMatchInfo`: `m[0]` and `m["name"]`.
pub(crate) fn item(
    key: &Value,
    this: &Value,
    _rt: &mut Runtime,
    line: u32,
) -> Option<Result<Value, RuntimeFailure>> {
    Some(answer(this, "__Item.Get", key, line))
}

/// The answer of a member for a place or a name.
fn answer(
    this: &Value,
    member: &str,
    key: &Value,
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let group = resolve(this, key, member, line)?;
    match member {
        "Len.Get" => Ok(Value::Int(group.1)),
        "Pos.Get" => Ok(Value::Int(group.0)),
        "Name.Get" => Ok(Value::Text(name_of(this, group.2))),
        _ => Ok(Value::Text(text_of(this, group.2))),
    }
}

/// The place, the length and the number of the group a member names.
fn resolve(
    this: &Value,
    key: &Value,
    member: &str,
    line: u32,
) -> Result<(i64, i64, usize), RuntimeFailure> {
    let count = count_of(this, line)?;
    // A place that is a number is the number of the group: a text that
    // holds a number is one too, and the part after the point is cut, the
    // way v2 cuts it.
    let group = if crate::builtins::is_numeric_value(key) {
        crate::builtins::token_to_int64(key) as i32 as i64
    } else {
        let written = key.to_text();
        let mut found: Option<i64> = None;
        for group in 1..=count {
            if name_of(this, group as usize).eq_ignore_ascii_case(&written) {
                if found.is_none() {
                    found = Some(group);
                }
                // A pattern may name two groups the same, and the one that
                // took part in the match is the answer.
                if span_of(this, group as usize)
                    .map(|span| span.0 != span.1)
                    .unwrap_or(false)
                {
                    found = Some(group);
                    break;
                }
            }
        }
        match found {
            Some(group) => group,
            None => return Err(the_parameter_failed(member, key, line)),
        }
    };
    if group < 0 || group > count {
        // The answer of a group that is not there is the failure of the
        // parameter, and not the empty text.
        return Err(the_parameter_failed(member, key, line));
    }
    let span = span_of(this, group as usize);
    Ok((
        span.map(|span| span.0 + 1).unwrap_or(0),
        span.map(|span| span.1).unwrap_or(0),
        group as usize,
    ))
}

/// The place and the length of the whole match.
fn whole_span(
    this: &Value,
    member: &str,
    line: u32,
) -> Result<(i64, i64), RuntimeFailure> {
    let Some(state) = state(this) else {
        return Err(not_a_match(line));
    };
    let state = state.borrow();
    let _ = member;
    match spans(&state).first() {
        Some(Value::List(items)) if items.len() == 2 => Ok((
            items[0].to_number().unwrap_or(0.0) as i64 + 1,
            items[1].to_number().unwrap_or(0.0) as i64,
        )),
        _ => Ok((1, 0)),
    }
}

/// `Count`: the number of the groups, the whole match not counted.
fn count(this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    Ok(Value::Int(count_of(this, line)?))
}

/// `Mark`: the mark of the pattern, which the engine of the tree does not
/// carry.
fn mark(this: &Value, line: u32) -> Result<Value, RuntimeFailure> {
    let Some(state) = state(this) else {
        return Err(not_a_match(line));
    };
    let state = state.borrow();
    Ok(state
        .value_of("mark")
        .unwrap_or_else(|| Value::Text(String::new())))
}

/// `__Enum`: the groups of the match.
///
/// Measured on the binary of v2.0.19 on 2026-11-09, with the source of it
/// read beside the measurement: the keys are the places of the groups, the
/// named ones under their names, and the values are the texts; a group
/// that took no part is the empty text. The parameter of the enumeration
/// is the number of the variables of the loop, and the loop reads that
/// itself: the enumerator of the tree is the same for either number.
fn enumeration(
    this: &Value,
    _arguments: &[Value],
    line: u32,
) -> Result<Value, RuntimeFailure> {
    let count = count_of(this, line)?;
    let mut pairs: Vec<(Value, Value)> = Vec::new();
    let mut group = 0i64;
    while group <= count {
        let key = match name_of(this, group as usize) {
            name if name.is_empty() => Value::Int(group),
            name => Value::Text(name),
        };
        pairs.push((key, Value::Text(text_of(this, group as usize))));
        group += 1;
    }
    Ok(enumerator::make(pairs, enumerator::Alone::TheValue))
}

// -- the state ------------------------------------------------------------------

/// The state of a match.
fn state(this: &Value) -> Option<Rc<RefCell<Object>>> {
    let Value::Object(object) = this else {
        return None;
    };
    let state = object.borrow().value_of(STATE)?;
    match state {
        Value::Object(state) => Some(state),
        _ => None,
    }
}

/// The spans of the groups of a match.
fn spans(state: &Object) -> Vec<Value> {
    match state.value_of("spans") {
        Some(Value::List(spans)) => spans.clone(),
        _ => Vec::new(),
    }
}

/// The number of the groups of a match, the whole match not counted.
fn count_of(this: &Value, line: u32) -> Result<i64, RuntimeFailure> {
    let Some(state) = state(this) else {
        return Err(not_a_match(line));
    };
    let state = state.borrow();
    Ok(spans(&state).len().saturating_sub(1) as i64)
}

/// The span of one group: the place and the length of it, or nothing when
/// the group took no part in the match.
fn span_of(this: &Value, group: usize) -> Option<(i64, i64)> {
    let state = state(this)?;
    let state = state.borrow();
    let spans = spans(&state);
    match spans.get(group) {
        Some(Value::List(items)) if items.len() == 2 => Some((
            items[0].to_number().unwrap_or(0.0) as i64,
            items[1].to_number().unwrap_or(0.0) as i64,
        )),
        _ => None,
    }
}

/// The text of the match the state carries.
fn haystack(this: &Value) -> Option<String> {
    let state = state(this)?;
    let state = state.borrow();
    state.value_of("text").map(|value| value.to_text())
}

/// The text of a group.
fn text_of(this: &Value, group: usize) -> String {
    let Some(haystack) = haystack(this) else {
        return String::new();
    };
    let Some((start, length)) = span_of(this, group) else {
        return String::new();
    };
    let chars: Vec<char> = haystack.chars().collect();
    let start = (start.max(0) as usize).min(chars.len());
    let end = (start + length.max(0) as usize).min(chars.len());
    chars[start..end].iter().collect()
}

/// The name of a group, or the empty text.
fn name_of(this: &Value, group: usize) -> String {
    let Some(state) = state(this) else {
        return String::new();
    };
    let state = state.borrow();
    match state.value_of("names") {
        Some(Value::List(names)) => names
            .get(group)
            .map(|value| value.to_text())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

// -- the failures ----------------------------------------------------------------

/// The failure of a value that is not a match of the class.
fn not_a_match(line: u32) -> RuntimeFailure {
    RuntimeFailure::Error(RuntimeError {
        message: "this value is not a RegExMatchInfo.".to_string(),
        line,
    })
}

/// The failure of a member that is read without the runtime around it: the
/// name of the member goes in the `What`.
fn named_for(failure: RuntimeFailure, member: &str) -> RuntimeFailure {
    crate::classes::named(
        failure,
        &crate::classes::the_name_of("regexmatchinfo", member),
    )
}

/// The failure of a place or a name that is not one of the groups: the
/// first parameter is invalid, and the value goes in the `Extra`.
fn the_parameter_failed(member: &str, key: &Value, line: u32) -> RuntimeFailure {
    crate::classes::the_failure_with(
        &format!(
            "Parameter #1 of {} is invalid.",
            crate::classes::the_name_of("regexmatchinfo", member)
        ),
        &key.to_text(),
        line,
    )
}