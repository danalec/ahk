//! The yoko-ahk language: a lexer, an AST, a parser and an interpreter
//! for AutoHotkey-style scripts.
//!
//! The crate is portable and does no I/O of its own: the script is parsed
//! here and executed against a [`runtime::Desktop`], which is whatever
//! implements it. The real one lives on the lab box (the `ahk-box`
//! crate); the tests use a fake that records the calls.
//!
//! ```
//! use ahk::{parse, runtime::{Runtime, Value}};
//!
//! let script = "x := 40 + 2\nMsg \"the answer is %x%\"";
//! let statements = parse(script).expect("parses");
//! let mut desktop = Fake::default();
//! let report = Runtime::new(&mut desktop, std::time::Duration::from_secs(10)).run(&statements);
//! assert!(report.ok);
//! assert_eq!(report.output, vec!["the answer is 42"]);
//! # use ahk::runtime::Desktop;
//! # #[derive(Default)]
//! # struct Fake;
//! # impl Desktop for Fake {
//! #     fn type_text(&mut self, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn press_key(&mut self, _: &str, _: &[String]) -> Result<String, String> { Ok(String::new()) }
//! #     fn click(&mut self, _: i64, _: i64, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn run(&mut self, _: &str, _: &str) -> Result<(i32, String, String), String> { Ok((0, String::new(), String::new())) }
//! #     fn capture(&mut self, _: i64, _: i64, _: i64, _: i64, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn pixel(&mut self, _: i64, _: i64) -> Result<String, String> { Ok("#000000".into()) }
//! # }
//! ```

pub mod ast;
pub mod keys;
pub mod lexer;
pub mod parser;
pub mod runtime;

pub use ast::{AssignOp, BinaryOp, LiteralValue, Node, UnaryOp};
pub use keys::{KeyStroke, UnknownKey, parse_spec, resolve_name};
pub use lexer::{SyntaxError, Token, TokenKind, tokenize};
pub use parser::{Parser, parse};
pub use runtime::{Desktop, Report, Runtime, RuntimeError, Step, Timeout, Value};
