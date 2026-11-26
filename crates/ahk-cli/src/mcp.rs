// SPDX-License-Identifier: Apache-2.0
//! A Model Context Protocol server for debugging AHK development, over
//! JSON-RPC 2.0 on stdio (one JSON object per line), the same framing as the
//! rgx MCP. It exposes the tree's own answers so a client can read the errors
//! of a script without a shell:
//!
//! * `check`  — parse a script and report the load error (line/column/message).
//! * `run`    — run a script and report the whole report (output/steps/error).
//! * `surface`— the summary of the names of a dialect.
//!
//! The request is read with the djson on-demand parser (rgx-core); the answer
//! is written with the tree's own JSON writer, so a client reads the same
//! report the runner writes.

use std::io::{BufRead, Write};
use std::path::PathBuf;

use ahk::dialect::{self, Dialect};
use ahk::json::Object;
use ahk::parse_with;
use ahk::runtime::Runtime;
use ahk::surface;
use crate::includes;
use crate::report;
use rgx_core::djson::ondemand::Ondemand;

const PROTOCOL_VERSION: &str = "2025-03-26";

/// Serve the MCP on stdio until the input ends.
pub fn serve() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        if let Some(answer) = handle_line(&line) {
            let _ = writeln!(out, "{answer}");
            let _ = out.flush();
        }
    }
}

/// One request, one answer (notifications get no answer).
pub fn handle_line(line: &str) -> Option<String> {
    let mut od = Ondemand::new(line).ok()?;
    let root = od.root().ok()?;
    let mut method = String::new();
    let mut id = String::from("null");
    let mut has_id = false;
    let mut name = String::new();
    let mut source = String::new();
    let mut dialect = String::new(); // no dialect was named: the script may name one
    let _ = root.object_entries(|key, v| {
        match key.as_ref() {
            "method" => {
                if let Ok(Some(s)) = v.as_str() {
                    method = s.into_owned();
                }
            }
            "id" => {
                if let Ok(raw) = v.raw() {
                    id = raw.to_string();
                    has_id = true;
                }
            }
            "params" => {
                let _ = v.object_entries(|k2, v2| {
                    match k2.as_ref() {
                        "name" => {
                            if let Ok(Some(s)) = v2.as_str() {
                                name = s.into_owned();
                            }
                        }
                        "arguments" => {
                            let _ = v2.object_entries(|k3, v3| {
                                match k3.as_ref() {
                                    "source" => {
                                        if let Ok(Some(s)) = v3.as_str() {
                                            source = s.into_owned();
                                        }
                                    }
                                    "dialect" => {
                                        if let Ok(Some(s)) = v3.as_str() {
                                            dialect = s.into_owned();
                                        }
                                    }
                                    _ => {}
                                }
                                Ok(true)
                            });
                        }
                        _ => {}
                    }
                    Ok(true)
                });
            }
            _ => {}
        }
        Ok(true)
    });
    if !has_id {
        return None; // a notification
    }
    let body = match method.as_str() {
        "initialize" => initialize_result(),
        "ping" => Object::new(4).finish(),
        "tools/list" => tools_json(),
        "tools/call" => tool_call(&name, &source, &dialect),
        _ => return Some(error(&id, -32601, "method not found")),
    };
    Some(result(&id, &body))
}

fn result(id: &str, body: &str) -> String {
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{body}}}")
}

fn error(id: &str, code: i32, message: &str) -> String {
    let mut e = Object::new(4);
    e.number("code", code as i64);
    e.text("message", message);
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"error\":{}}}", e.finish())
}

fn initialize_result() -> String {
    let mut o = Object::new(8);
    o.text("protocolVersion", PROTOCOL_VERSION);
    o.field("capabilities", |s| s.push_str("{\"tools\":{}}"));
    o.field("serverInfo", |s| {
        let mut i = Object::new(4);
        i.text("name", "ahk");
        i.text("version", env!("CARGO_PKG_VERSION"));
        s.push_str(&i.finish());
    });
    o.finish()
}

fn tool_schema(name: &str, desc: &str) -> String {
    let mut t = Object::new(8);
    t.text("name", name);
    t.text("description", desc);
    t.field("inputSchema", |s| {
        let mut sc = Object::new(8);
        sc.text("type", "object");
        sc.field("properties", |p| {
            let mut props = Object::new(8);
            props.field("source", |x| {
                let mut d = Object::new(4);
                d.text("type", "string");
                d.text("description", "the AHK source");
                x.push_str(&d.finish());
            });
            props.field("dialect", |x| {
                let mut d = Object::new(4);
                d.text("type", "string");
                d.text("description", "v1 | v2 | v2.1 | ks | ks20 (default v2)");
                x.push_str(&d.finish());
            });
            p.push_str(&props.finish());
        });
        sc.texts("required", &["source".to_string()]);
        s.push_str(&sc.finish());
    });
    t.finish()
}

fn tools_json() -> String {
    let mut o = Object::new(4);
    o.field("tools", |s| {
        s.push('[');
        s.push_str(&tool_schema("check", "Parse an AHK script and report the load error (line, column, message), or ok."));
        s.push(',');
        s.push_str(&tool_schema("run", "Run an AHK script and report the whole report: output, steps, error."));
        s.push(',');
        s.push_str(&tool_schema("surface", "The summary of the names of a dialect (dialect argument only)."));
        s.push(',');
        s.push_str(&tool_schema(
            "v1",
            "Run the script under the binary of the AutoHotkey v1 (the reference of the tree) and report what it said: the output, the errors and the code of the end.",
        ));
        s.push(',');
        s.push_str(&tool_schema(
            "diff",
            "Run the script under the tree and under the binary of the AutoHotkey v1 and report the two answers side by side: the error of each one.",
        ));
        s.push(']');
    });
    o.finish()
}

fn the_dialect(name: &str) -> Option<&'static dyn Dialect> {
    match name.to_lowercase().as_str() {
        "v1" => Some(dialect::v1()),
        "v2.1" | "v2_1" => Some(dialect::v2_1()),
        "ks" => Some(dialect::ks()),
        "ks20" | "ks2.0" => Some(dialect::ks_20()),
        "v2" => Some(dialect::v2()),
        // No dialect was named: the script may name one in its `#Requires`.
        "" => None,
        _ => Some(dialect::v2()),
    }
}

fn tool_call(name: &str, source: &str, dialect_name: &str) -> String {
    // The dialect of the script: the one the client named, or the one the
    // script names in its `#Requires`, or the v2 of the tree.
    let dialect = the_dialect(dialect_name)
        .or_else(|| dialect::the_dialect_of_the_source(source))
        .unwrap_or_else(dialect::v2);
    let content = match name {
        "check" => match the_source_with_the_includes(source, dialect) {
            Ok(source) => match parse_with(&source, dialect) {
                Ok(_) => {
                    let mut o = Object::new(4);
                    o.boolean("ok", true);
                    o.finish()
                }
                Err(e) => {
                    let mut o = Object::new(8);
                    o.boolean("ok", false);
                    o.text("error", &e.message);
                    o.number("line", e.line as i64);
                    o.number("column", e.column as i64);
                    o.finish()
                }
            },
            // The include that could not be resolved is the refusal of the
            // load, and it carries the line of the directive.
            Err(refusal) => {
                let mut o = Object::new(8);
                o.boolean("ok", false);
                o.text("error", &refusal.message);
                o.number("line", i64::from(refusal.line));
                o.number("column", 0);
                o.finish()
            }
        },
        "run" => match the_source_with_the_includes(source, dialect) {
            Ok(source) => match parse_with(&source, dialect) {
                Ok(statements) => {
                    let mut desktop = the_desktop();
                    let runtime = Runtime::new(&mut *desktop, std::time::Duration::from_secs(10))
                        .with_source(&source)
                        .with_dialect(dialect);
                    let rep = runtime.run(&statements);
                    report::the_report(&rep, &[])
                }
                Err(e) => report::the_failure_of_the_load(&e.message, e.line, e.column),
            },
            Err(refusal) => report::the_failure_of_the_load(&refusal.message, refusal.line, 0),
        },
        "surface" => {
            let entries = surface::entries_of(dialect);
            let summary = surface::summary_of(&entries, dialect);
            let mut o = Object::new(4);
            o.number("total", entries.len() as i64);
            o.text("summary", &summary.text());
            o.finish()
        }
        "v1" => the_answer_of_the_v1(source),
        "diff" => the_answer_of_the_diff(source, dialect),
        _ => {
            let mut o = Object::new(4);
            o.boolean("isError", true);
            o.text("error", &format!("unknown tool {name}"));
            o.finish()
        }
    };
    let mut o = Object::new(4);
    o.field("content", |s| {
        s.push('[');
        let mut c = Object::new(4);
        c.text("type", "text");
        c.text("text", &content);
        s.push_str(&c.finish());
        s.push(']');
    });
    o.finish()
}

#[cfg(windows)]
fn the_desktop() -> Box<dyn ahk::desktop::Desktop> {
    Box::new(ahk_box::windows::WindowsDesktop::new())
}
#[cfg(not(windows))]
fn the_desktop() -> Box<dyn ahk::desktop::Desktop> {
    Box::new(ahk_box::LabDesktop::new())
}

/// The answer of the v1 about a script.
fn the_answer_of_the_v1(source: &str) -> String {
    let answer = crate::v1ref::the_run_of_the_v1(source);
    let mut o = Object::new(8);
    match answer.failure {
        Some(failure) => {
            o.boolean("ok", false);
            o.text("error", &failure);
        }
        None => {
            o.boolean("ok", true);
            o.text("output", &answer.output);
            o.number("code", answer.code.unwrap_or(-1) as i64);
            o.boolean("hung", answer.hung);
            let errors = answer.the_errors();
            o.texts("errors", &errors);
        }
    }
    o.finish()
}

/// The source of the script with the includes of it in place of the lines
/// that ask for them.
///
/// The includes are resolved before the parse, the way the runner resolves
/// them: a script that asks for a library that is not there does not load,
/// and the MCP says so with the words of the official.
///
/// The MCP has no path of the script: the folder of the script is the
/// working directory of the server.
fn the_source_with_the_includes(
    source: &str,
    dialect: &'static dyn Dialect,
) -> Result<String, includes::Refusal> {
    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    includes::resolve_includes(source, &base, None, &mut Vec::new(), dialect)
}

/// The answer of the tree and the answer of the v1, side by side.
fn the_answer_of_the_diff(source: &str, dialect: &'static dyn Dialect) -> String {
    let mut o = Object::new(8);
    // the answer of the tree
    match the_source_with_the_includes(source, dialect) {
        Ok(source) => match parse_with(&source, dialect) {
            Ok(statements) => {
                let mut desktop = the_desktop();
                let runtime = Runtime::new(&mut *desktop, std::time::Duration::from_secs(10))
                    .with_source(&source)
                    .with_dialect(dialect);
                let rep = runtime.run(&statements);
                let mut t = Object::new(8);
                t.text("error", &rep.error.clone().unwrap_or_default());
                t.number("line", rep.error_line.unwrap_or(0) as i64);
                o.field("tree", |s| s.push_str(&t.finish()));
            }
            Err(e) => {
                let mut t = Object::new(8);
                t.text("error", &e.message);
                t.number("line", e.line as i64);
                o.field("tree", |s| s.push_str(&t.finish()));
            }
        },
        Err(refusal) => {
            let mut t = Object::new(8);
            t.text("error", &refusal.message);
            t.number("line", i64::from(refusal.line));
            o.field("tree", |s| s.push_str(&t.finish()));
        }
    }
    // the answer of the v1
    let answer = crate::v1ref::the_run_of_the_v1(source);
    let mut v = Object::new(8);
    match answer.failure {
        Some(failure) => {
            v.text("error", &failure);
        }
        None => {
            v.text("error", &answer.the_errors().join(" | "));
            v.number("code", answer.code.unwrap_or(-1) as i64);
            v.boolean("hung", answer.hung);
        }
    }
    o.field("v1", |s| s.push_str(&v.finish()));
    o.finish()
}
