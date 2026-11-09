//! The report of a run, written as the JSON the notes of the runner promise.
//!
//! The fields are the ones of the report, in the order a reader expects
//! them, and the values of the script are written the way their kinds spell
//! them: a whole number as a number, a text as a text, a list as an array,
//! and an object as the class it is and the properties it holds.

use std::collections::BTreeMap;

use ahk::runtime::{Property, Report, Step, Value};
use ahk::warnings::Warning;

use crate::json::Object;

/// The report of a run that finished, or of one that failed.
pub fn the_report(report: &Report, notes: &[String]) -> String {
    let mut object = Object::new(1024);
    object.boolean("ok", report.ok);
    object.field("steps", |out| the_steps(out, &report.steps));
    object.texts("output", &report.output);
    object.field("warnings", |out| the_warnings(out, &report.warnings));
    object.texts("notes", notes);
    match &report.error {
        Some(error) => object.text("error", error),
        None => object.nothing("error"),
    };
    object.optional_number("error_line", report.error_line.map(i64::from));
    object.optional_number("exit_code", report.exit_code.map(i64::from));
    object.number("elapsed_ms", report.elapsed_ms as i64);
    object.field("variables", |out| the_variables(out, &report.variables));
    object.finish()
}

/// The report of a script that did not load.
///
/// The payload is reduced on purpose: a script that does not parse has no
/// run, so it has no `elapsed_ms`, no `exit_code` and no variable.
pub fn the_failure_of_the_load(error: &str, line: u32, column: u32) -> String {
    let mut object = Object::new(256);
    object.boolean("ok", false);
    object.text("error", error);
    object.number("error_line", i64::from(line));
    object.number("column", i64::from(column));
    // The three lists are empty, and they are written out: a report always
    // carries them, whatever the run did.
    object.field("steps", |out| out.push_str("[]"));
    object.field("output", |out| out.push_str("[]"));
    object.field("notes", |out| out.push_str("[]"));
    object.finish()
}

/// The steps of the run.
fn the_steps(out: &mut String, steps: &[Step]) {
    out.push('[');
    for (index, step) in steps.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let mut object = Object::new(128);
        object.number("line", i64::from(step.line));
        object.text("op", &step.op);
        object.text("detail", &step.detail);
        object.boolean("ok", step.ok);
        object.number("elapsed_ms", step.elapsed_ms as i64);
        out.push_str(&object.finish());
    }
    out.push(']');
}

/// The warnings of the script.
fn the_warnings(out: &mut String, warnings: &[Warning]) {
    out.push('[');
    for (index, warning) in warnings.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let mut object = Object::new(128);
        object.number("line", i64::from(warning.line));
        object.text("message", &warning.message);
        out.push_str(&object.finish());
    }
    out.push(']');
}

/// The variables the script left, in the order of their names.
fn the_variables(out: &mut String, variables: &BTreeMap<String, Value>) {
    out.push('{');
    for (index, (name, value)) in variables.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        crate::json::write_text(out, name);
        out.push(':');
        the_value(out, value);
    }
    out.push('}');
}

/// One value of the script.
fn the_value(out: &mut String, value: &Value) {
    match value {
        Value::Int(number) => out.push_str(&number.to_string()),
        Value::Float(number) => {
            if number.is_finite() {
                out.push_str(&number.to_string());
            } else {
                // The format has no spelling for an infinity or a NaN.
                out.push_str("null");
            }
        }
        Value::Text(text) => crate::json::write_text(out, text),
        Value::List(values) => {
            out.push('[');
            for (index, value) in values.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                the_value(out, value);
            }
            out.push(']');
        }
        Value::Object(object) => {
            let object = object.borrow();
            let mut written = Object::new(256);
            written.text("class", &object.class);
            written.field("properties", |out| {
                the_properties(out, &object.properties)
            });
            out.push_str(&written.finish());
        }
        Value::Reference(reference) => {
            let mut written = Object::new(128);
            written.text("name", &reference.name);
            written.text("scope", &format!("{:?}", reference.scope));
            written.boolean("alias", reference.alias);
            out.push_str(&written.finish());
        }
    }
}

/// The properties of an object.
fn the_properties(out: &mut String, properties: &BTreeMap<String, Property>) {
    out.push('{');
    for (index, (name, property)) in properties.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        crate::json::write_text(out, name);
        out.push(':');
        let mut written = Object::new(128);
        written.text("name", &property.name);
        written.field("value", |out| the_value(out, &property.value));
        out.push_str(&written.finish());
    }
    out.push('}');
}