//! The runtime: it executes a parsed script against a desktop.
//!
//! A port of the Python runtime in the `yoko-ahk-mcp` package. The script
//! is compiled in memory and executed statement by statement. Nothing is
//! written to disk: the source is gone when the turn is over.
//!
//! Every statement produces a record -- what it did, whether it worked,
//! how long it took -- because a script that drives a desktop has to be
//! able to say what it did, not just that it finished.
//!
//! The desktop is whatever implements [`Desktop`]. The real one lives on
//! the lab box (the `ahk-box` crate); the tests use a fake that records
//! the calls.
//!
//! ## The focus check
//!
//! A script that types into the wrong window is worse than a script that
//! fails, so the language carries the primitives that answer "is the right
//! thing in front of us?":
//!
//! * `MouseGetPos()` -- where the pointer is.
//! * `PixelGet(x, y)` -- the colour under it.
//! * `PixelSearch(x1, y1, x2, y2, colour)` -- where that colour is.
//!
//! These three work on any desktop, because they are read from the screen
//! itself. `WinActive(title)` and `WinExist(title)` are the window-level
//! questions, and they are answered by whatever the desktop can actually
//! ask: on a Wayland session with no window API, the answer is
//! *unknown*, and the runtime says so out loud rather than guessing
//! `false`. A script that cannot answer the question fails; it does not
//! quietly take the wrong branch.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::ast::{AssignOp, BinaryOp, LiteralValue, Node, UnaryOp};

/// A value a script can hold.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// A whole number.
    Int(i64),
    /// A number with a point.
    Float(f64),
    /// Text.
    Text(String),
    /// A boolean.
    Bool(bool),
    /// A list of values: what `MouseGetPos` and `PixelSearch` return.
    List(Vec<Value>),
}

impl Value {
    /// Whether the value counts as true.
    pub fn truthy(&self) -> bool {
        match self {
            Value::Text(text) => !text.is_empty(),
            Value::Int(number) => *number != 0,
            Value::Float(number) => *number != 0.0,
            Value::Bool(flag) => *flag,
            Value::List(items) => !items.is_empty(),
        }
    }

    /// The value as text.
    pub fn to_text(&self) -> String {
        match self {
            Value::Int(number) => number.to_string(),
            Value::Float(number) => number.to_string(),
            Value::Text(text) => text.clone(),
            Value::Bool(flag) => flag.to_string(),
            Value::List(items) => items
                .iter()
                .map(|item| item.to_text())
                .collect::<Vec<String>>()
                .join(","),
        }
    }

    /// The value as a number.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Int(number) => Some(*number as f64),
            Value::Float(number) => Some(*number),
            Value::Bool(flag) => Some(if *flag { 1.0 } else { 0.0 }),
            Value::Text(text) => text.trim().parse::<f64>().ok(),
            Value::List(_) => None,
        }
    }

    /// A number as a whole number when it is one.
    pub fn from_number(number: f64) -> Value {
        if number.fract() == 0.0 && number.abs() < 9.0e15 {
            Value::Int(number as i64)
        } else {
            Value::Float(number)
        }
    }
}

/// A statement failed.
#[derive(Debug, thiserror::Error)]
#[error("{message} (line {line})")]
pub struct RuntimeError {
    pub message: String,
    pub line: u32,
}

/// The script ran for longer than the caller allowed.
#[derive(Debug, thiserror::Error)]
#[error("the script ran for more than {seconds}s")]
pub struct Timeout {
    pub seconds: f64,
}

/// What the desktop could tell us about a window.
///
/// The third case is the point: on a session with no window API, the
/// honest answer is "I do not know", and it has to stay distinct from
/// "no".
#[derive(Debug, Clone, PartialEq)]
pub enum WindowAnswer {
    /// The window is the active one.
    Active,
    /// The window exists and is not the active one.
    NotActive,
    /// The desktop cannot tell; the reason says why.
    Unknown(String),
}

/// What the runtime needs from the machine it drives.
pub trait Desktop {
    /// Type text exactly, and say how it was typed.
    fn type_text(&mut self, text: &str) -> Result<String, String>;

    /// Press one key, and say how it was pressed.
    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String>;

    /// Click at a screen position.
    fn click(&mut self, x: i64, y: i64, button: &str) -> Result<String, String>;

    /// Run a command; return its exit code, stdout and stderr.
    fn run(&mut self, command: &str, window: &str) -> Result<(i32, String, String), String>;

    /// Capture a region of the screen into a file; return the path.
    fn capture(&mut self, x: i64, y: i64, width: i64, height: i64, filename: &str) -> Result<String, String>;

    /// The colour of one pixel, as `#rrggbb`.
    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String>;

    /// Move the pointer to a screen position.
    ///
    /// A desktop that cannot move the pointer says so; the default is an
    /// error, because a script that believes it moved the pointer and did
    /// not is a script that types into the wrong window.
    fn move_pointer(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Err("this desktop cannot move the pointer".to_string())
    }

    /// Where the pointer is, in screen pixels.
    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        Err("this desktop cannot tell where the pointer is".to_string())
    }

    /// The first pixel of a colour inside a region, if there is one.
    fn pixel_search(
        &mut self,
        _x1: i64,
        _y1: i64,
        _x2: i64,
        _y2: i64,
        _colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        Err("this desktop cannot search the screen for a colour".to_string())
    }

    /// Whether the window with this title is the active one.
    fn window_active(&mut self, _title: &str) -> Result<WindowAnswer, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }

    /// Whether a window with this title exists.
    fn window_exists(&mut self, _title: &str) -> Result<WindowAnswer, String> {
        Err("this desktop cannot list the windows".to_string())
    }

    /// The title of the active window.
    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        Err("this desktop cannot tell which window is active".to_string())
    }
}

/// What one statement did.
#[derive(Debug, Clone, Serialize)]
pub struct Step {
    pub line: u32,
    pub op: String,
    pub detail: String,
    pub ok: bool,
    pub elapsed_ms: u64,
}

/// The whole run: the steps, the output, and whether it finished.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Report {
    pub ok: bool,
    pub steps: Vec<Step>,
    pub output: Vec<String>,
    pub notes: Vec<String>,
    pub error: Option<String>,
    pub error_line: Option<u32>,
    pub exit_code: Option<i32>,
    pub elapsed_ms: u64,
    pub variables: BTreeMap<String, Value>,
}

/// The way a script left a block.
enum Flow {
    Break,
    Continue,
    Return(Option<Value>),
    Exit(i32),
}

/// A function the script defined.
#[derive(Clone)]
struct Function {
    parameters: Vec<String>,
    body: Vec<Node>,
}

/// Executes a script.
///
/// The limits are the guard rails: a script that loops forever must not
/// take the box with it, and one that types a million characters must not
/// either. Both limits are reported when they are hit.
pub struct Runtime<'a> {
    desktop: &'a mut dyn Desktop,
    timeout: Duration,
    variables: BTreeMap<String, Value>,
    scopes: Vec<BTreeMap<String, Value>>,
    functions: BTreeMap<String, Function>,
    steps: Vec<Step>,
    output: Vec<String>,
    notes: Vec<String>,
    started: Instant,
    steps_taken: usize,
}

impl<'a> Runtime<'a> {
    /// The most statements one script may execute.
    pub const MAX_STEPS: usize = 2000;
    /// The most times a loop body may run.
    pub const MAX_ITERATIONS: usize = 1000;

    /// A runtime that drives `desktop` for at most `timeout`.
    pub fn new(desktop: &'a mut dyn Desktop, timeout: Duration) -> Self {
        Self {
            desktop,
            timeout,
            variables: BTreeMap::new(),
            scopes: Vec::new(),
            functions: BTreeMap::new(),
            steps: Vec::new(),
            output: Vec::new(),
            notes: Vec::new(),
            started: Instant::now(),
            steps_taken: 0,
        }
    }

    /// Run the script and report what happened.
    pub fn run(mut self, statements: &[Node]) -> Report {
        let mut error = None;
        let mut error_line = None;
        let mut exit_code = None;
        // The functions are collected before the first statement runs, so a
        // script may call a function it defines further down.
        self.collect_functions(statements);
        match self.run_block(statements) {
            Ok(()) => {}
            Err(RuntimeFailure::Flow(Flow::Break)) | Err(RuntimeFailure::Flow(Flow::Continue)) => {}
            Err(RuntimeFailure::Flow(Flow::Return(_))) => {}
            Err(RuntimeFailure::Flow(Flow::Exit(code))) => exit_code = Some(code),
            Err(RuntimeFailure::Error(failure)) => {
                error = Some(failure.message);
                error_line = Some(failure.line);
            }
            Err(RuntimeFailure::Timeout(timeout)) => {
                error = Some(timeout.to_string());
            }
        }
        Report {
            ok: error.is_none(),
            steps: self.steps,
            output: self.output,
            notes: self.notes,
            error,
            error_line,
            exit_code,
            elapsed_ms: self.started.elapsed().as_millis() as u64,
            variables: self.variables.clone(),
        }
    }

    /// A note for the report: the place where the box was not what the
    /// script assumed.
    pub fn note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }

    /// Find the functions the script defines, wherever they are.
    fn collect_functions(&mut self, statements: &[Node]) {
        for statement in statements {
            match statement {
                Node::FunctionDef { name, parameters, body, .. } => {
                    self.functions.insert(
                        name.to_lowercase(),
                        Function { parameters: parameters.clone(), body: body.clone() },
                    );
                }
                Node::If { branches, otherwise, .. } => {
                    for branch in branches {
                        self.collect_functions(&branch.body);
                    }
                    self.collect_functions(otherwise);
                }
                Node::Switch { cases, .. } => {
                    for case in cases {
                        self.collect_functions(&case.body);
                    }
                }
                Node::While { body, .. } | Node::Loop { body, .. } => self.collect_functions(body),
                _ => {}
            }
        }
    }

    fn run_block(&mut self, statements: &[Node]) -> Result<(), RuntimeFailure> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, statement: &Node) -> Result<(), RuntimeFailure> {
        self.steps_taken += 1;
        if self.steps_taken > Self::MAX_STEPS {
            return Err(RuntimeFailure::Error(RuntimeError {
                message: format!("the script ran more than {} statements", Self::MAX_STEPS),
                line: statement.line(),
            }));
        }
        if self.started.elapsed() > self.timeout {
            return Err(RuntimeFailure::Timeout(Timeout { seconds: self.timeout.as_secs_f64() }));
        }

        let started = Instant::now();
        let mut detail = String::new();

        match statement {
            Node::Assign { name, op, value, line } => {
                let value = self.evaluate(value)?;
                let current = self.lookup(name);
                let stored = match op {
                    AssignOp::Set => value,
                    AssignOp::Add => match current {
                        Some(Value::Text(text)) => Value::Text(format!("{text}{}", value.to_text())),
                        Some(other) => Value::Float(
                            other.to_number().unwrap_or(0.0) + value.to_number().unwrap_or(0.0),
                        ),
                        None => value,
                    },
                    AssignOp::Subtract => Value::Float(
                        current.and_then(|v| v.to_number()).unwrap_or(0.0)
                            - value.to_number().unwrap_or(0.0),
                    ),
                    AssignOp::Concat => Value::Text(format!(
                        "{}{}",
                        current.map(|v| v.to_text()).unwrap_or_default(),
                        value.to_text()
                    )),
                };
                self.assign(name, stored);
                detail = format!("{name} = {}", self.lookup(name).map(|v| v.to_text()).unwrap_or_default());
                let _ = line;
            }
            Node::Send { items, .. } => {
                let mut parts = Vec::new();
                for item in items {
                    match item {
                        Node::KeyName { name, .. } => {
                            parts.push(self.desktop.press_key(name, &[])?);
                        }
                        other => {
                            let value = self.evaluate(other)?;
                            parts.push(self.desktop.type_text(&self.interpolate(&value.to_text()))?);
                        }
                    }
                }
                detail = parts.concat().chars().take(200).collect();
            }
            Node::Click { x, y, button, .. } => {
                let x = self.evaluate(x)?.to_number().unwrap_or(0.0) as i64;
                let y = self.evaluate(y)?.to_number().unwrap_or(0.0) as i64;
                detail = self.desktop.click(x, y, button)?;
            }
            Node::MouseMove { x, y, .. } => {
                let x = self.evaluate(x)?.to_number().unwrap_or(0.0) as i64;
                let y = self.evaluate(y)?.to_number().unwrap_or(0.0) as i64;
                detail = self.desktop.move_pointer(x, y)?;
            }
            Node::Sleep { millis, .. } => {
                let millis = self.evaluate(millis)?.to_number().unwrap_or(0.0) as u64;
                let millis = millis.min(60_000);
                std::thread::sleep(Duration::from_millis(millis));
                detail = format!("{millis} ms");
            }
            Node::Run { command, window, .. } => {
                let command = self.evaluate(command)?.to_text();
                let (code, stdout, stderr) = self.desktop.run(&command, window)?;
                detail = format!("{command} -> exit {code}");
                if !stdout.trim().is_empty() {
                    self.output.push(stdout.trim_end().to_string());
                }
                if !stderr.trim().is_empty() {
                    self.output.push(stderr.trim_end().to_string());
                }
            }
            Node::PixelGet { x, y, .. } => {
                let x = self.evaluate(x)?.to_number().unwrap_or(0.0) as i64;
                let y = self.evaluate(y)?.to_number().unwrap_or(0.0) as i64;
                let colour = self.desktop.pixel(x, y)?;
                self.variables.insert("pixel".to_string(), Value::Text(colour.clone()));
                detail = format!("({x}, {y}) = {colour}");
            }
            Node::Screenshot { x, y, width, height, filename, .. } => {
                let x = self.evaluate(x)?.to_number().unwrap_or(0.0) as i64;
                let y = self.evaluate(y)?.to_number().unwrap_or(0.0) as i64;
                let width = self.evaluate(width)?.to_number().unwrap_or(0.0) as i64;
                let height = self.evaluate(height)?.to_number().unwrap_or(0.0) as i64;
                let name = self.evaluate(filename)?.to_text();
                let path = self.desktop.capture(x, y, width, height, &name)?;
                self.variables.insert("screenshot".to_string(), Value::Text(path.clone()));
                detail = path;
            }
            Node::Msg { text, .. } => {
                let value = self.evaluate(text)?.to_text();
                let text = self.interpolate(&value);
                self.output.push(text.clone());
                detail = text.chars().take(200).collect();
            }
            Node::Call { name, arguments, line } => {
                let value = self.call(name, arguments, *line)?;
                detail = value.to_text();
            }
            Node::If { branches, otherwise, .. } => {
                let mut taken = false;
                for branch in branches {
                    if self.evaluate(&branch.condition)?.truthy() {
                        self.run_block(&branch.body)?;
                        taken = true;
                        break;
                    }
                }
                if !taken {
                    self.run_block(otherwise)?;
                }
            }
            Node::Switch { subject, cases, line } => {
                let value = self.evaluate(subject)?;
                let mut matched = false;
                for case in cases {
                    match &case.value {
                        Some(expected) => {
                            let expected = self.evaluate(expected)?;
                            if values_equal(&value, &expected) {
                                self.run_block(&case.body)?;
                                matched = true;
                                break;
                            }
                        }
                        None => {
                            self.run_block(&case.body)?;
                            matched = true;
                            break;
                        }
                    }
                }
                let _ = (matched, line);
            }
            Node::While { condition, body, .. } => {
                let mut iterations = 0usize;
                while self.evaluate(condition)?.truthy() {
                    iterations += 1;
                    if iterations > Self::MAX_ITERATIONS {
                        return Err(RuntimeFailure::Error(RuntimeError {
                            message: format!("the loop ran more than {} times", Self::MAX_ITERATIONS),
                            line: statement.line(),
                        }));
                    }
                    match self.run_block(body) {
                        Ok(()) => {}
                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                        Err(other) => return Err(other),
                    }
                }
            }
            Node::Loop { count, body, .. } => {
                let count = match count {
                    Some(node) => self.evaluate(node)?.to_number().unwrap_or(0.0) as i64,
                    None => 1,
                };
                for _ in 0..count.max(0) {
                    match self.run_block(body) {
                        Ok(()) => {}
                        Err(RuntimeFailure::Flow(Flow::Break)) => break,
                        Err(RuntimeFailure::Flow(Flow::Continue)) => continue,
                        Err(other) => return Err(other),
                    }
                }
            }
            Node::Break { .. } => return Err(RuntimeFailure::Flow(Flow::Break)),
            Node::Continue { .. } => return Err(RuntimeFailure::Flow(Flow::Continue)),
            Node::Return { value, .. } => {
                let value = match value {
                    Some(node) => Some(self.evaluate(node)?),
                    None => None,
                };
                return Err(RuntimeFailure::Flow(Flow::Return(value)));
            }
            Node::ExitApp { code, .. } => {
                let code = match code {
                    Some(node) => self.evaluate(node)?.to_number().unwrap_or(0.0) as i32,
                    None => 0,
                };
                return Err(RuntimeFailure::Flow(Flow::Exit(code)));
            }
            Node::FunctionDef { name, .. } => {
                detail = format!("{name} is defined");
            }
            Node::Literal { .. } | Node::Var { .. } | Node::KeyName { .. } => {
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "this is an expression, not a statement".into(),
                    line: statement.line(),
                }));
            }
            Node::Unary { .. } | Node::Binary { .. } | Node::Index { .. } => {
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "this is an expression, not a statement".into(),
                    line: statement.line(),
                }));
            }
        }

        self.steps.push(Step {
            line: statement.line(),
            op: op_name(statement).to_string(),
            detail,
            ok: true,
            elapsed_ms: started.elapsed().as_millis() as u64,
        });
        Ok(())
    }

    // -- scopes ---------------------------------------------------------------------

    /// The value of a variable, from the innermost scope outwards.
    fn lookup(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        self.variables.get(name).cloned()
    }

    /// Set a variable: in the innermost scope when there is one, in the
    /// global one otherwise. A function that assigns a name creates a local
    /// one, the way AutoHotkey v2 does.
    fn assign(&mut self, name: &str, value: Value) {
        match self.scopes.last_mut() {
            Some(scope) => {
                scope.insert(name.to_string(), value);
            }
            None => {
                self.variables.insert(name.to_string(), value);
            }
        }
    }

    // -- calls ----------------------------------------------------------------------

    /// Call a function: the one the script defined, or one of the builtins.
    fn call(&mut self, name: &str, arguments: &[Node], line: u32) -> Result<Value, RuntimeFailure> {
        if let Some(function) = self.functions.get(name).cloned() {
            let mut values = Vec::with_capacity(arguments.len());
            for argument in arguments {
                values.push(self.evaluate(argument)?);
            }
            let mut scope: BTreeMap<String, Value> = BTreeMap::new();
            for (index, parameter) in function.parameters.iter().enumerate() {
                scope.insert(parameter.clone(), values.get(index).cloned().unwrap_or(Value::Text(String::new())));
            }
            self.scopes.push(scope);
            let outcome = self.run_block(&function.body);
            self.scopes.pop();
            return match outcome {
                Ok(()) => Ok(Value::Text(String::new())),
                Err(RuntimeFailure::Flow(Flow::Return(value))) => {
                    Ok(value.unwrap_or(Value::Text(String::new())))
                }
                Err(other) => Err(other),
            };
        }
        let mut values = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        self.call_builtin(name, &values, line)
    }

    /// The functions the language carries.
    fn call_builtin(
        &mut self,
        name: &str,
        values: &[Value],
        line: u32,
    ) -> Result<Value, RuntimeFailure> {
        let text_of = |index: usize| values.get(index).map(|v| v.to_text()).unwrap_or_default();
        let number_of = |index: usize| values.get(index).and_then(|v| v.to_number()).unwrap_or(0.0);
        let wrong = |what: &str| {
            RuntimeFailure::Error(RuntimeError { message: format!("{name} {what}"), line })
        };
        match name {
            // -- the screen ------------------------------------------------------------
            "pixelget" => {
                if values.len() != 2 {
                    return Err(wrong("takes x and y"));
                }
                let x = number_of(0) as i64;
                let y = number_of(1) as i64;
                Ok(Value::Text(self.desktop.pixel(x, y)?))
            }
            "pixelsearch" => {
                if values.len() != 5 {
                    return Err(wrong("takes x1, y1, x2, y2 and a colour"));
                }
                let x1 = number_of(0) as i64;
                let y1 = number_of(1) as i64;
                let x2 = number_of(2) as i64;
                let y2 = number_of(3) as i64;
                let colour = text_of(4);
                match self.desktop.pixel_search(x1, y1, x2, y2, &colour)? {
                    Some((x, y)) => Ok(Value::List(vec![Value::Int(x), Value::Int(y)])),
                    None => Ok(Value::List(Vec::new())),
                }
            }
            "mousegetpos" => {
                let (x, y) = self.desktop.mouse_position()?;
                self.variables.insert("mouse_x".to_string(), Value::Int(x));
                self.variables.insert("mouse_y".to_string(), Value::Int(y));
                Ok(Value::List(vec![Value::Int(x), Value::Int(y)]))
            }
            "wingettitle" => match self.desktop.active_window_title()? {
                Some(title) => Ok(Value::Text(title)),
                None => Ok(Value::Text(String::new())),
            },
            "winactive" => {
                if values.len() != 1 {
                    return Err(wrong("takes a window title"));
                }
                match self.desktop.window_active(&text_of(0))? {
                    WindowAnswer::Active => Ok(Value::Bool(true)),
                    WindowAnswer::NotActive => Ok(Value::Bool(false)),
                    WindowAnswer::Unknown(reason) => Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("WinActive cannot answer: {reason}"),
                        line,
                    })),
                }
            }
            "winexist" => {
                if values.len() != 1 {
                    return Err(wrong("takes a window title"));
                }
                match self.desktop.window_exists(&text_of(0))? {
                    WindowAnswer::Active | WindowAnswer::NotActive => Ok(Value::Bool(true)),
                    WindowAnswer::Unknown(reason) => Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("WinExist cannot answer: {reason}"),
                        line,
                    })),
                }
            }
            // -- the numbers ----------------------------------------------------------
            "abs" => Ok(Value::from_number(number_of(0).abs())),
            "sqrt" => {
                let number = number_of(0);
                if number < 0.0 {
                    return Err(wrong("of a negative number"));
                }
                Ok(Value::from_number(number.sqrt()))
            }
            "mod" => {
                let divisor = number_of(1);
                if divisor == 0.0 {
                    return Err(wrong("by zero"));
                }
                Ok(Value::from_number(number_of(0) % divisor))
            }
            "floor" => Ok(Value::from_number(number_of(0).floor())),
            "ceil" => Ok(Value::from_number(number_of(0).ceil())),
            "round" => {
                let digits = values.get(1).and_then(|v| v.to_number()).unwrap_or(0.0) as i32;
                let factor = 10f64.powi(digits);
                Ok(Value::from_number((number_of(0) * factor).round() / factor))
            }
            "min" => Ok(Value::from_number(
                values.iter().filter_map(|v| v.to_number()).fold(f64::INFINITY, f64::min),
            )),
            "max" => Ok(Value::from_number(
                values.iter().filter_map(|v| v.to_number()).fold(f64::NEG_INFINITY, f64::max),
            )),
            // -- the text -------------------------------------------------------------
            "strlen" | "strlength" => Ok(Value::Int(text_of(0).chars().count() as i64)),
            "substr" => {
                // AutoHotkey counts positions from one; this does too.
                let text: Vec<char> = text_of(0).chars().collect();
                let start = number_of(1).max(1.0) as usize - 1;
                let sliced: String = match values.get(2).and_then(|v| v.to_number()) {
                    Some(length) => text
                        .iter()
                        .skip(start)
                        .take(length.max(0.0) as usize)
                        .collect(),
                    None => text.iter().skip(start).collect(),
                };
                Ok(Value::Text(sliced))
            }
            "strreplace" => {
                if values.len() < 3 {
                    return Err(wrong("takes a text, what to look for and what to put there"));
                }
                let haystack = text_of(0);
                let needle = text_of(1);
                let replacement = text_of(2);
                if needle.is_empty() {
                    return Ok(Value::Text(haystack));
                }
                Ok(Value::Text(haystack.replace(&needle, &replacement)))
            }
            "instr" => {
                if values.len() < 2 {
                    return Err(wrong("takes a text and what to look for"));
                }
                let haystack = text_of(0);
                let needle = text_of(1);
                // AutoHotkey does not distinguish the case here.
                let position = haystack.to_lowercase().find(&needle.to_lowercase());
                Ok(match position {
                    Some(byte) => {
                        let characters = haystack[..byte].chars().count();
                        Value::Int(characters as i64 + 1)
                    }
                    None => Value::Int(0),
                })
            }
            "trim" => {
                let text = text_of(0);
                let characters: Vec<char> = text.chars().collect();
                let set: Vec<char> = values
                    .get(1)
                    .map(|v| v.to_text())
                    .unwrap_or_else(|| " \t\r\n".to_string())
                    .chars()
                    .collect();
                let mut start = 0usize;
                let mut end = characters.len();
                while start < end && set.contains(&characters[start]) {
                    start += 1;
                }
                while end > start && set.contains(&characters[end - 1]) {
                    end -= 1;
                }
                Ok(Value::Text(characters[start..end].iter().collect()))
            }
            "upper" => Ok(Value::Text(text_of(0).to_uppercase())),
            "lower" => Ok(Value::Text(text_of(0).to_lowercase())),
            "number" => Ok(Value::from_number(number_of(0))),
            "format" => {
                if values.is_empty() {
                    return Err(wrong("takes a pattern"));
                }
                Ok(Value::Text(format_pattern(&text_of(0), &values[1..])))
            }
            // -- the clock ------------------------------------------------------------
            "sleep" => {
                let millis = number_of(0).clamp(0.0, 60_000.0) as u64;
                std::thread::sleep(Duration::from_millis(millis));
                Ok(Value::Int(millis as i64))
            }
            _ => Err(RuntimeFailure::Error(RuntimeError {
                message: format!("unknown function {name:?}"),
                line,
            })),
        }
    }

    // -- expressions ---------------------------------------------------------------

    fn evaluate(&mut self, node: &Node) -> Result<Value, RuntimeFailure> {
        match node {
            Node::Literal { value, .. } => Ok(match value {
                LiteralValue::Number(number) => Value::from_number(*number),
                LiteralValue::Text(text) => Value::Text(text.clone()),
                LiteralValue::Bool(flag) => Value::Bool(*flag),
            }),
            Node::Var { name, line } => self.lookup(name).ok_or_else(|| {
                RuntimeFailure::Error(RuntimeError {
                    message: format!("the variable {name:?} is not set"),
                    line: *line,
                })
            }),
            Node::KeyName { name, .. } => Ok(Value::Text(format!("{{{name}}}"))),
            Node::Unary { op, operand, .. } => {
                let value = self.evaluate(operand)?;
                Ok(match op {
                    UnaryOp::Not => Value::Bool(!value.truthy()),
                    UnaryOp::Negate => Value::Float(-value.to_number().unwrap_or(0.0)),
                })
            }
            Node::Binary { op, left, right, .. } => {
                let left_value = self.evaluate(left)?;
                match op {
                    BinaryOp::And => {
                        if !left_value.truthy() {
                            return Ok(Value::Bool(false));
                        }
                        return Ok(Value::Bool(self.evaluate(right)?.truthy()));
                    }
                    BinaryOp::Or => {
                        if left_value.truthy() {
                            return Ok(Value::Bool(true));
                        }
                        return Ok(Value::Bool(self.evaluate(right)?.truthy()));
                    }
                    _ => {}
                }
                let right_value = self.evaluate(right)?;
                let left_number = left_value.to_number().unwrap_or(0.0);
                let right_number = right_value.to_number().unwrap_or(0.0);
                let both_numbers = matches!(left_value, Value::Int(_) | Value::Float(_))
                    && matches!(right_value, Value::Int(_) | Value::Float(_));
                Ok(match op {
                    BinaryOp::Equal => Value::Bool(values_equal(&left_value, &right_value)),
                    BinaryOp::NotEqual => Value::Bool(!values_equal(&left_value, &right_value)),
                    BinaryOp::Less => Value::Bool(left_number < right_number),
                    BinaryOp::Greater => Value::Bool(left_number > right_number),
                    BinaryOp::LessEqual => Value::Bool(left_number <= right_number),
                    BinaryOp::GreaterEqual => Value::Bool(left_number >= right_number),
                    BinaryOp::Add => {
                        if both_numbers {
                            // Two whole numbers add up to a whole number, the
                            // way they do in the Python the port came from.
                            match (&left_value, &right_value) {
                                (Value::Int(a), Value::Int(b)) => Value::Int(a + b),
                                _ => Value::Float(left_number + right_number),
                            }
                        } else {
                            Value::Text(format!("{}{}", left_value.to_text(), right_value.to_text()))
                        }
                    }
                    BinaryOp::Subtract => Value::Float(left_number - right_number),
                    BinaryOp::Multiply => Value::Float(left_number * right_number),
                    BinaryOp::Divide => {
                        if right_number == 0.0 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "division by zero".into(),
                                line: node.line(),
                            }));
                        }
                        Value::Float(left_number / right_number)
                    }
                    BinaryOp::Concat => {
                        Value::Text(format!("{}{}", left_value.to_text(), right_value.to_text()))
                    }
                    BinaryOp::And | BinaryOp::Or => unreachable!("handled above"),
                })
            }
            Node::Call { name, arguments, line } => self.call(name, arguments, *line),
            Node::Index { target, index, line } => {
                let target = self.evaluate(target)?;
                let position = self.evaluate(index)?.to_number().unwrap_or(0.0) as i64;
                // The positions count from one, the way AutoHotkey counts
                // them.
                let position = position.max(1) as usize;
                match target {
                    Value::List(items) => items.get(position - 1).cloned().ok_or_else(|| {
                        RuntimeFailure::Error(RuntimeError {
                            message: format!("the list has no item {position}"),
                            line: *line,
                        })
                    }),
                    Value::Text(text) => {
                        let characters: Vec<char> = text.chars().collect();
                        characters.get(position - 1).map(|c| Value::Text(c.to_string())).ok_or_else(|| {
                            RuntimeFailure::Error(RuntimeError {
                                message: format!("the text has no character {position}"),
                                line: *line,
                            })
                        })
                    }
                    other => Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("{other:?} has no items to pick from"),
                        line: *line,
                    })),
                }
            }
            Node::Send { .. }
            | Node::Click { .. }
            | Node::MouseMove { .. }
            | Node::Sleep { .. }
            | Node::Run { .. }
            | Node::PixelGet { .. }
            | Node::Screenshot { .. }
            | Node::Msg { .. }
            | Node::If { .. }
            | Node::Switch { .. }
            | Node::While { .. }
            | Node::Loop { .. }
            | Node::Break { .. }
            | Node::Continue { .. }
            | Node::Return { .. }
            | Node::ExitApp { .. }
            | Node::FunctionDef { .. }
            | Node::Assign { .. } => Err(RuntimeFailure::Error(RuntimeError {
                message: "this is a statement, not an expression".into(),
                line: node.line(),
            })),
        }
    }

    /// Replace `%name%` with the variable's value.
    ///
    /// AutoHotkey's way of putting a variable in a string. A `%` that is
    /// not a complete `%name%` is left alone, so a path with a percent
    /// sign in it survives.
    fn interpolate(&self, text: &str) -> String {
        if !text.contains('%') {
            return text.to_string();
        }
        let mut out = String::with_capacity(text.len());
        let chars: Vec<char> = text.chars().collect();
        let mut index = 0usize;
        while index < chars.len() {
            if chars[index] == '%' {
                if let Some(offset) = chars[index + 1..].iter().position(|c| *c == '%') {
                    let close = index + 1 + offset;
                    let name: String = chars[index + 1..close].iter().collect();
                    if !name.is_empty() {
                        if let Some(value) = self.lookup(&name) {
                            out.push_str(&value.to_text());
                            index = close + 1;
                            continue;
                        }
                    }
                    out.push_str(&chars[index..=close].iter().collect::<String>());
                    index = close + 1;
                    continue;
                }
                out.extend(&chars[index..]);
                break;
            }
            out.push(chars[index]);
            index += 1;
        }
        out
    }
}

/// Whether two values are equal, the way a script expects.
///
/// A number that came out of an addition is compared with a number that
/// was written down: `2` and `2.0` are the same number, even though one
/// of them is a whole number and the other is not.
fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        _ => left == right,
    }
}

/// Put the arguments into a pattern.
///
/// The pattern is AutoHotkey's: `{0}` is the first argument, `{1:.2}` is
/// the second with two decimals.
fn format_pattern(pattern: &str, arguments: &[Value]) -> String {
    let mut out = String::with_capacity(pattern.len());
    let chars: Vec<char> = pattern.chars().collect();
    let mut index = 0usize;
    while index < chars.len() {
        if chars[index] != '{' {
            out.push(chars[index]);
            index += 1;
            continue;
        }
        let Some(offset) = chars[index + 1..].iter().position(|c| *c == '}') else {
            out.push(chars[index]);
            index += 1;
            continue;
        };
        let close = index + 1 + offset;
        let inside: String = chars[index + 1..close].iter().collect();
        let (position, decimals) = match inside.split_once(':') {
            Some((position, spec)) => {
                let decimals = spec.trim_start_matches('.').parse::<usize>().unwrap_or(0);
                (position.trim(), Some(decimals))
            }
            None => (inside.trim(), None),
        };
        match position.parse::<usize>().ok().and_then(|index| arguments.get(index)) {
            Some(value) => {
                match decimals {
                    Some(decimals) => {
                        let number = value.to_number().unwrap_or(0.0);
                        out.push_str(&format!("{number:.decimals$}"));
                    }
                    None => out.push_str(&value.to_text()),
                }
            }
            None => {
                out.push('{');
                out.push_str(&inside);
                out.push('}');
            }
        }
        index = close + 1;
    }
    out
}

/// The name a statement is reported under.
pub fn op_name(statement: &Node) -> &'static str {
    match statement {
        Node::Assign { .. } => "Assign",
        Node::Send { .. } => "Send",
        Node::Click { .. } => "Click",
        Node::MouseMove { .. } => "MouseMove",
        Node::Sleep { .. } => "Sleep",
        Node::Run { .. } => "Run",
        Node::PixelGet { .. } => "PixelGet",
        Node::Screenshot { .. } => "Screenshot",
        Node::Msg { .. } => "Msg",
        Node::If { .. } => "If",
        Node::Switch { .. } => "Switch",
        Node::While { .. } => "While",
        Node::Loop { .. } => "Loop",
        Node::Break { .. } => "Break",
        Node::Continue { .. } => "Continue",
        Node::Return { .. } => "Return",
        Node::ExitApp { .. } => "ExitApp",
        Node::FunctionDef { .. } => "FunctionDef",
        Node::Literal { .. } => "Literal",
        Node::Var { .. } => "Var",
        Node::KeyName { .. } => "KeyName",
        Node::Unary { .. } => "Unary",
        Node::Binary { .. } => "Binary",
        Node::Index { .. } => "Index",
        Node::Call { .. } => "Call",
    }
}

enum RuntimeFailure {
    Error(RuntimeError),
    Timeout(Timeout),
    Flow(Flow),
}

impl From<RuntimeError> for RuntimeFailure {
    fn from(error: RuntimeError) -> Self {
        RuntimeFailure::Error(error)
    }
}

impl From<Timeout> for RuntimeFailure {
    fn from(timeout: Timeout) -> Self {
        RuntimeFailure::Timeout(timeout)
    }
}

impl From<Flow> for RuntimeFailure {
    fn from(flow: Flow) -> Self {
        RuntimeFailure::Flow(flow)
    }
}

impl From<String> for RuntimeFailure {
    fn from(message: String) -> Self {
        RuntimeFailure::Error(RuntimeError { message, line: 0 })
    }
}
