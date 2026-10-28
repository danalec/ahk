#![cfg_attr(not(feature = "std"), no_std)]
//! The ahk language: a lexer, an AST, a parser and an interpreter
//! for AutoHotkey-style scripts.
//!
//! The crate is portable and does no I/O of its own: the script is parsed
//! here and executed against a [`runtime::Desktop`], which is whatever
//! implements it. The real one lives on the lab box (the `ahk-box`
//! crate); the tests use a fake that records the calls.
//!
//! ## Without the standard library
//!
//! The crate runs on a target that has no `std` at all. Turn the `std`
//! feature off and the language is the same; what the host has to supply
//! is the clock -- the time the runtime measures itself with, and the
//! pause the `Sleep` statement asks for -- through the [`runtime::Clock`]
//! trait.
//!
//! ```
//! use ahk::runtime::{Clock, Runtime};
//! # use ahk::desktop::{Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers, Windows};
//! # use core::time::Duration;
//! # #[derive(Default)]
//! # struct Fake;
//! # impl Desktop for Fake {}
//! # impl Keyboard for Fake {
//! #     fn type_text(&mut self, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn press_key(&mut self, _: &str, _: &[String]) -> Result<String, String> { Ok(String::new()) }
//! # }
//! # impl Pointer for Fake {
//! #     fn click(&mut self, _: i64, _: i64, _: &str, _: u32) -> Result<String, String> { Ok(String::new()) }
//! # }
//! # impl Screen for Fake {
//! #     fn capture(&mut self, _: i64, _: i64, _: i64, _: i64, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn pixel(&mut self, _: i64, _: i64) -> Result<String, String> { Ok("#000000".into()) }
//! # }
//! # impl Processes for Fake {
//! #     fn run(&mut self, _: &str, _: &str) -> Result<(i32, String, String), String> { Ok((0, String::new(), String::new())) }
//! # }
//! # impl Windows for Fake {}
//! # impl Files for Fake {}
//! # impl Clipboard for Fake {}
//! # impl Timers for Fake {}
//! # impl Hotkeys for Fake {}
//! # impl Native for Fake {}
//! # struct TheClock;
//! # impl Clock for TheClock {
//! #     fn now_ms(&self) -> u64 { 0 }
//! #     fn sleep_ms(&mut self, _: u64) {}
//! # }
//! # let script = "x := 40 + 2\nMsgBox \"the answer is %x%\"";
//! # let statements = ahk::parse(script).expect("parses");
//! # let mut desktop = Fake::default();
//! # let mut clock = TheClock;
//! # let report = Runtime::with_clock(&mut desktop, &mut clock, Duration::from_secs(10)).run(&statements);
//! # assert!(report.ok);
//! ```
//!
//! ```
//! use ahk::{parse, runtime::{Runtime, Value}};
//!
//! let script = "x := 40 + 2\nMsgBox \"the answer is %x%\"";
//! let statements = parse(script).expect("parses");
//! let mut desktop = Fake::default();
//! let report = Runtime::new(&mut desktop, core::time::Duration::from_secs(10)).run(&statements);
//! assert!(report.ok);
//! assert_eq!(report.output, vec!["the answer is 42"]);
//! # use ahk::desktop::{Clipboard, Desktop, Files, Hotkeys, Keyboard, Native, Pointer, Processes, Screen, Timers, Windows};
//! # #[derive(Default)]
//! # struct Fake;
//! # impl Desktop for Fake {}
//! # impl Keyboard for Fake {
//! #     fn type_text(&mut self, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn press_key(&mut self, _: &str, _: &[String]) -> Result<String, String> { Ok(String::new()) }
//! # }
//! # impl Pointer for Fake {
//! #     fn click(&mut self, _: i64, _: i64, _: &str, _: u32) -> Result<String, String> { Ok(String::new()) }
//! # }
//! # impl Screen for Fake {
//! #     fn capture(&mut self, _: i64, _: i64, _: i64, _: i64, _: &str) -> Result<String, String> { Ok(String::new()) }
//! #     fn pixel(&mut self, _: i64, _: i64) -> Result<String, String> { Ok("#000000".into()) }
//! # }
//! # impl Processes for Fake {
//! #     fn run(&mut self, _: &str, _: &str) -> Result<(i32, String, String), String> { Ok((0, String::new(), String::new())) }
//! # }
//! # impl Windows for Fake {}
//! # impl Files for Fake {}
//! # impl Clipboard for Fake {}
//! # impl Timers for Fake {}
//! # impl Hotkeys for Fake {}
//! # impl Native for Fake {}
//! ```

extern crate alloc;

pub mod ast;
pub mod builtins;
pub mod classes;
pub mod dll;
pub mod desktop;
pub mod keys;
pub mod lexer;
pub mod parser;
pub mod regex;
pub mod runtime;
pub mod variables;

pub use ast::{AssignOp, BinaryOp, LiteralValue, Node, Parameter, UnaryOp};
pub use keys::{KeyStroke, UnknownKey, parse_spec, resolve_name};
pub use lexer::{SyntaxError, Token, TokenKind, tokenize};
pub use parser::{Parser, parse, precedence};
pub use desktop::{Desktop, WindowAnswer};
pub use runtime::{Clock, Report, Runtime, RuntimeError, Step, Timeout, Value};
#[cfg(feature = "std")]
pub use runtime::SystemClock;
