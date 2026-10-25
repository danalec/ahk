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
}

impl Value {
    /// Whether the value counts as true.
    pub fn truthy(&self) -> bool {
        match self {
            Value::Text(text) => !text.is_empty(),
            Value::Int(number) => *number != 0,
            Value::Float(number) => *number != 0.0,
            Value::Bool(flag) => *flag,
        }
    }

    /// The value as text.
    pub fn to_text(&self) -> String {
        match self {
            Value::Int(number) => number.to_string(),
            Value::Float(number) => number.to_string(),
            Value::Text(text) => text.clone(),
            Value::Bool(flag) => flag.to_string(),
        }
    }

    /// The value as a number.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Int(number) => Some(*number as f64),
            Value::Float(number) => Some(*number),
            Value::Bool(flag) => Some(if *flag { 1.0 } else { 0.0 }),
            Value::Text(text) => text.trim().parse::<f64>().ok(),
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
    pub elapsed_ms: u64,
    pub variables: BTreeMap<String, Value>,
}

enum Flow {
    Break,
    Continue,
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
        match self.run_block(statements) {
            Ok(()) => {}
            Err(RuntimeFailure::Flow(Flow::Break)) | Err(RuntimeFailure::Flow(Flow::Continue)) => {}
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
            elapsed_ms: self.started.elapsed().as_millis() as u64,
            variables: self
                .variables
                .iter()
                .filter(|(_, value)| matches!(value, Value::Int(_) | Value::Float(_) | Value::Text(_) | Value::Bool(_)))
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        }
    }

    /// A note for the report: the place where the box was not what the
    /// script assumed.
    pub fn note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
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
                let current = self.variables.get(name).cloned();
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
                self.variables.insert(name.clone(), stored);
                detail = format!("{name} = {}", self.variables[name].to_text());
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
            Node::If { condition, then_body, otherwise, .. } => {
                if self.evaluate(condition)?.truthy() {
                    self.run_block(then_body)?;
                } else {
                    self.run_block(otherwise)?;
                }
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
            Node::Call { .. } | Node::Literal { .. } | Node::Var { .. } | Node::KeyName { .. } => {
                return Err(RuntimeFailure::Error(RuntimeError {
                    message: "this is an expression, not a statement".into(),
                    line: statement.line(),
                }));
            }
            Node::Unary { .. } | Node::Binary { .. } => {
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

    // -- expressions ---------------------------------------------------------------

    fn evaluate(&mut self, node: &Node) -> Result<Value, RuntimeFailure> {
        match node {
            Node::Literal { value, .. } => Ok(match value {
                LiteralValue::Number(number) => {
                    if number.fract() == 0.0 && number.abs() < 9.0e15 {
                        Value::Int(*number as i64)
                    } else {
                        Value::Float(*number)
                    }
                }
                LiteralValue::Text(text) => Value::Text(text.clone()),
                LiteralValue::Bool(flag) => Value::Bool(*flag),
            }),
            Node::Var { name, line } => self.variables.get(name).cloned().ok_or_else(|| {
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
            Node::Call { name, arguments, line } => {
                let mut values = Vec::with_capacity(arguments.len());
                for argument in arguments {
                    values.push(self.evaluate(argument)?);
                }
                match name.as_str() {
                    "pixelget" => {
                        if values.len() != 2 {
                            return Err(RuntimeFailure::Error(RuntimeError {
                                message: "PixelGet takes x and y".into(),
                                line: *line,
                            }));
                        }
                        let x = values[0].to_number().unwrap_or(0.0) as i64;
                        let y = values[1].to_number().unwrap_or(0.0) as i64;
                        Ok(Value::Text(self.desktop.pixel(x, y)?))
                    }
                    "strlen" => {
                        let text = values.first().map(|v| v.to_text()).unwrap_or_default();
                        Ok(Value::Int(text.chars().count() as i64))
                    }
                    "substr" => {
                        let text = values.first().map(|v| v.to_text()).unwrap_or_default();
                        let start = values.get(1).and_then(|v| v.to_number()).unwrap_or(0.0) as usize;
                        let chars: Vec<char> = text.chars().collect();
                        let sliced: String = match values.get(2).and_then(|v| v.to_number()) {
                            Some(length) => chars
                                .iter()
                                .skip(start)
                                .take(length.max(0.0) as usize)
                                .collect(),
                            None => chars.iter().skip(start).collect(),
                        };
                        Ok(Value::Text(sliced))
                    }
                    "upper" => Ok(Value::Text(
                        values.first().map(|v| v.to_text()).unwrap_or_default().to_uppercase(),
                    )),
                    "lower" => Ok(Value::Text(
                        values.first().map(|v| v.to_text()).unwrap_or_default().to_lowercase(),
                    )),
                    "trim" => Ok(Value::Text(
                        values.first().map(|v| v.to_text()).unwrap_or_default().trim().to_string(),
                    )),
                    "number" => Ok(Value::Float(
                        values.first().and_then(|v| v.to_number()).unwrap_or(0.0),
                    )),
                    _ => Err(RuntimeFailure::Error(RuntimeError {
                        message: format!("unknown function {name:?}"),
                        line: *line,
                    })),
                }
            }
            Node::Send { .. }
            | Node::Click { .. }
            | Node::Sleep { .. }
            | Node::Run { .. }
            | Node::PixelGet { .. }
            | Node::Screenshot { .. }
            | Node::Msg { .. }
            | Node::If { .. }
            | Node::While { .. }
            | Node::Loop { .. }
            | Node::Break { .. }
            | Node::Continue { .. }
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
                        if let Some(value) = self.variables.get(&name) {
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

/// The name a statement is reported under.
pub fn op_name(statement: &Node) -> &'static str {
    match statement {
        Node::Assign { .. } => "Assign",
        Node::Send { .. } => "Send",
        Node::Click { .. } => "Click",
        Node::Sleep { .. } => "Sleep",
        Node::Run { .. } => "Run",
        Node::PixelGet { .. } => "PixelGet",
        Node::Screenshot { .. } => "Screenshot",
        Node::Msg { .. } => "Msg",
        Node::If { .. } => "If",
        Node::While { .. } => "While",
        Node::Loop { .. } => "Loop",
        Node::Break { .. } => "Break",
        Node::Continue { .. } => "Continue",
        Node::Literal { .. } => "Literal",
        Node::Var { .. } => "Var",
        Node::KeyName { .. } => "KeyName",
        Node::Unary { .. } => "Unary",
        Node::Binary { .. } => "Binary",
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
