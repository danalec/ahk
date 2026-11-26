// SPDX-License-Identifier: Apache-2.0
//! The tools of the MCP that compare the tree with the binary of the v1.
//!
//! The tests that need the binary of the v1 are skipped when it is not
//! on the machine: the path comes from `AHK_V1`, and the place of the
//! install of the v1.1.37.02 is the default.
//!
//! The variable of the environment is the same for the whole process, so
//! the tests that read it and the test that writes it take the same lock.

use std::sync::{Mutex, MutexGuard, OnceLock};

use ahk_cli::mcp;

/// The lock of the variable of the environment of the path of the v1.
fn the_lock() -> MutexGuard<'static, ()> {
    static THE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    THE_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

/// Whether the binary of the v1 is on the machine.
fn the_v1_is_there() -> bool {
    std::path::Path::new(&ahk_cli::v1ref::the_path_of_the_v1()).exists()
}

/// The text of the tool of a `tools/call`.
fn the_text_of_the_tool(source: &str, name: &str) -> String {
    let request = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"name":"{name}","arguments":{{"source":{}}}}}}}"#,
        the_json_string(source)
    );
    let answer = mcp::handle_line(&request).expect("the request has an answer");
    // The answer of the tool is the text of the content of it, and the
    // text is a JSON string of its own.
    let Some(start) = answer.find(r#""text":""#) else {
        return answer;
    };
    let rest = &answer[start + 8..];
    let mut out = String::new();
    let mut escaped = false;
    for c in rest.chars() {
        if escaped {
            out.push(c);
            escaped = false;
            continue;
        }
        match c {
            '\\' => {
                out.push(c);
                escaped = true;
            }
            '"' => break,
            c => out.push(c),
        }
    }
    out.replace("\\\"", "\"").replace("\\\\", "\\")
}

/// A string of JSON, with the quotes and the bars of it.
fn the_json_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The list of the tools of the server.
fn the_tools() -> String {
    let request = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#;
    mcp::handle_line(request).expect("the request has an answer")
}

#[test]
fn the_list_of_the_tools_has_the_v1_and_the_diff() {
    let tools = the_tools();
    assert!(tools.contains(r#""name":"v1""#), "the tools are {}", tools);
    assert!(
        tools.contains(r#""name":"diff""#),
        "the tools are {}",
        tools
    );
}

#[test]
fn the_v1_tool_says_that_the_binary_is_not_there_when_it_is_not_there() {
    let _guard = the_lock();
    // The path of the binary of the v1 is a path that is not there.
    let path = std::env::temp_dir().join("ahk-mcp-test-no-v1").join("v1.exe");
    let before = std::env::var("AHK_V1").ok();
    // SAFETY: the lock of the tests keeps the other tests from reading
    // the variable while it is written.
    unsafe { std::env::set_var("AHK_V1", &path) };
    let text = the_text_of_the_tool("x := 1", "v1");
    match before {
        Some(old) => unsafe { std::env::set_var("AHK_V1", old) },
        None => unsafe { std::env::remove_var("AHK_V1") },
    }
    assert!(
        text.contains("the binary of the v1 is not there"),
        "the answer is {}",
        text
    );
}

#[test]
fn the_v1_tool_reports_the_error_of_a_script_that_does_not_load() {
    let _guard = the_lock();
    if !the_v1_is_there() {
        eprintln!("the binary of the v1 is not on the machine: the test is skipped");
        return;
    }
    let text = the_text_of_the_tool("BoxEx(_this, params*)", "v1");
    assert!(text.contains(r#""ok":true"#), "the answer is {}", text);
    // The v1 loads the line and refuses the call at the run: the function
    // does not exist.
    assert!(
        text.contains("nonexistent function"),
        "the answer is {}",
        text
    );
    assert!(
        text.contains(r#""errors":["#),
        "the answer is {}",
        text
    );
}

#[test]
fn the_diff_tool_gives_the_two_answers_side_by_side() {
    let _guard = the_lock();
    if !the_v1_is_there() {
        eprintln!("the binary of the v1 is not on the machine: the test is skipped");
        return;
    }
    // The script calls a function that does not exist: the v1 says that
    // the function does not exist, and the tree says the same in its own
    // words. The two answers are side by side, and a client reads the
    // difference without a shell.
    let text = the_text_of_the_tool("BoxEx(_this, params*)", "diff");
    assert!(text.contains(r#""tree""#), "the answer is {}", text);
    assert!(text.contains(r#""v1""#), "the answer is {}", text);
    assert!(
        text.contains("nonexistent function"),
        "the answer is {}",
        text
    );
    assert!(
        text.contains(r#""error":""#),
        "the answer is {}",
        text
    );
}
