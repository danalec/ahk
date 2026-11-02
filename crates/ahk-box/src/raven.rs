//! The raven: the Windows of a box that has none.
//!
//! The `DllCall` of v2 calls a function of a library. On the box the
//! library is a `.so`, and the call is made by the trampoline of
//! `native.rs`. When the library is one of Windows, the box has no
//! Windows of its own -- and the **raven** is the one that answers: a
//! program that emulates a Windows process on Linux.
//!
//! The raven runs a guest program, the **helper**, and the helper does
//! the call inside the Windows of the raven: it loads the library and
//! calls the function. The tree talks to the helper over its standard
//! input, one line per call, and the lines are tab separated, with
//! every value in hexadecimal.
//!
//! The helper is the other half of the protocol: it is
//! `crates/ahk-box/helper/helper.c`, and the two halves were proved
//! together on the box on 2026-10-28.
//!
//! There are two helpers: one of the sixty-four bits and one of the
//! thirty-two. The raven runs a guest of the width of the program it is
//! given, and a library of thirty-two bits is only called by a guest of
//! thirty-two bits. The guest of the sixty-four bits is asked first,
//! and the guest of the thirty-two bits is asked when the first does
//! not have the library.
//!
//! Two things the box says out loud:
//!
//! * The stderr of the raven goes nowhere. The reason of a failure
//!   comes back in the protocol, and a pipe that nobody reads fills up
//!   and stops the raven.
//! * The raven is started with the first library of Windows that is
//!   asked for, and it lives until the desktop is dropped: a program of
//!   its own is not cheap to start.

use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::Duration;

use ahk::dll::{
    NativeArg, NativeCall, NativeFailure, NativeFailureKind, NativeResult, NativeType, NativeValue,
};

/// The life of the guest, in milliseconds.
///
/// The helper lives as long as the script needs it, and the raven ends
/// a guest that runs for longer than this.
const TIMEOUT_MS: &str = "86400000";

/// The time the process is given to end by itself, in tenths of a second.
const FAREWELL: u32 = 20;

// -- the paths ----------------------------------------------------------------

/// The paths of the raven and of the guests that it runs.
#[derive(Debug)]
pub struct Paths {
    /// The program of the raven.
    pub program: PathBuf,
    /// The guest of the sixty-four bits.
    pub helper: PathBuf,
    /// The guest of the thirty-two bits, when the box has one.
    pub helper32: Option<PathBuf>,
}

impl Paths {
    /// The paths of the box, read out of the environment.
    ///
    /// `AHK_RAVEN` is the program of the raven and `AHK_RAVEN_HELPER`
    /// is the guest of the sixty-four bits. `AHK_RAVEN_HELPER32` is the
    /// guest of the thirty-two bits, and the box may not have one: the
    /// libraries of thirty-two bits are then not called. A box that has
    /// not the raven and the guest of the sixty-four bits has no raven.
    pub fn from_environment() -> Option<Paths> {
        Some(Paths {
            program: read("AHK_RAVEN")?,
            helper: read("AHK_RAVEN_HELPER")?,
            helper32: read("AHK_RAVEN_HELPER32"),
        })
    }
}

/// The reason of a library that is not there.
///
/// It is the message of v2.0.19, and the raven says it in the two
/// guests: the library of thirty-two bits is not in the folder of the
/// guest of the sixty-four bits.
const NOT_THERE: &str = "Failed to load DLL.";

/// The reason of a function that is not in the library.
///
/// It is the message of v2.0.19, and the guest says it when the library is
/// there and the function is not.
const NO_FUNCTION: &str = "Call to nonexistent function.";

/// The path of a variable of the environment.
///
/// A variable that is empty is a variable that is not there: the box
/// sets it, and it sets it whole.
fn read(name: &str) -> Option<PathBuf> {
    let value = std::env::var_os(name)?;
    if value.is_empty() {
        return None;
    }
    Some(PathBuf::from(value))
}

// -- the transport ------------------------------------------------------------

/// Where the request of a call goes, and where the answer comes from.
///
/// The pipes of the guest are the real thing; the cases of the tree use
/// a transport of their own, so that the way the two guests are asked
/// can be proved without a raven.
trait Transport: std::fmt::Debug {
    /// The answer of the guest to a request of the protocol.
    fn ask(&mut self, request: &str) -> Result<String, String>;
    /// The guest is told that there is nothing more to come.
    fn close(&mut self) {}
}

/// The pipes of a guest: the input it reads and the output it writes.
#[derive(Debug)]
struct Pipes {
    input: Option<ChildStdin>,
    output: BufReader<ChildStdout>,
}

impl Transport for Pipes {
    fn ask(&mut self, request: &str) -> Result<String, String> {
        let input = self.input.as_mut().ok_or_else(|| "the raven is not running.".to_string())?;
        input
            .write_all(request.as_bytes())
            .map_err(|error| format!("the raven did not take the request: {error}"))?;
        input.flush().map_err(|error| format!("the raven did not take the request: {error}"))?;
        let mut reply = String::new();
        let read =
            self.output.read_line(&mut reply).map_err(|error| format!("the raven did not answer: {error}"))?;
        if read == 0 {
            return Err("the raven ended the answer.".to_string());
        }
        Ok(reply.trim_end_matches(['\n', '\r']).to_string())
    }

    fn close(&mut self) {
        // The end of the input is what ends the guest.
        drop(self.input.take());
    }
}

// -- the raven ----------------------------------------------------------------

/// The raven that runs, and the library of Windows it serves.
///
/// The raven is a process of its own: it runs the helper, and the
/// helper does the calls. It is started with the first library of
/// Windows that is asked for, and it lives until the desktop is
/// dropped.
#[derive(Debug)]
pub struct Raven {
    /// The pipes of the guest.
    transport: Box<dyn Transport>,
    /// The process of the raven, when the guest has one.
    process: Option<Child>,
}

impl Raven {
    /// The raven, started with the guest it runs.
    ///
    /// The guest is given the longest life the raven allows (a day),
    /// and the pipes of the host: the guest reads the requests from the
    /// input and writes the answers to the output.
    pub fn start(program: &Path, guest: &Path) -> Result<Raven, String> {
        let mut process = Command::new(program)
            .arg("--run-process")
            .arg("--execution=native")
            .arg(format!("--timeout-ms={TIMEOUT_MS}"))
            .arg("--stdio=inherit")
            .arg(guest)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            // The stderr of the raven goes nowhere: the reason of a
            // failure comes back in the protocol, and a pipe that
            // nobody reads fills up and stops the raven.
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| format!("the raven did not start: {error}"))?;
        let input = process.stdin.take().ok_or_else(|| "the raven has no input.".to_string())?;
        let output = process.stdout.take().ok_or_else(|| "the raven has no output.".to_string())?;
        Ok(Raven {
            transport: Box::new(Pipes { input: Some(input), output: BufReader::new(output) }),
            process: Some(process),
        })
    }

    /// The call of a function of a library of Windows.
    ///
    /// The request goes to the helper, and the answer comes back. The
    /// blocks that come with it go into the arguments that are the
    /// address of a value, and that is why the call is given by the
    /// address of the call: the argument is the way back to the
    /// variable of the script.
    pub fn call(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        let mut line = request(call);
        // The request is a line: the guest reads it until the end of
        // the line, and a request without it is a request it never
        // answers.
        line.push('\n');
        let reply = self.transport.ask(&line).map_err(the_failure)?;
        answer(call, &reply)
    }

    /// The raven stops: the guest is told by the end of its input,
    /// and the process is ended.
    pub fn stop(&mut self) {
        // The guest ends when its input ends.
        self.transport.close();
        // The process gets a moment to end by itself: the guest reads
        // the end of the input and leaves.
        let Some(mut process) = self.process.take() else {
            return;
        };
        for _ in 0..FAREWELL {
            if process.try_wait().ok().flatten().is_some() {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        // And it is ended when it is still there.
        let _ = process.kill();
        let _ = process.wait();
    }
}

impl Drop for Raven {
    fn drop(&mut self) {
        self.stop();
    }
}

// -- the guests ---------------------------------------------------------------

/// The two guests of the raven.
///
/// The guest of the sixty-four bits is asked first: it is the one that
/// has the libraries of most of the programs. The guest of the
/// thirty-two bits is asked when the first does not have the library:
/// the libraries of thirty-two bits live in the folder of the guest of
/// thirty-two bits, and the raven says the same in the two.
#[derive(Debug)]
pub struct Guests {
    paths: Paths,
    wide: Option<Raven>,
    narrow: Option<Raven>,
    notes: Vec<String>,
}

impl Guests {
    /// The guests of a box, with the paths of the programs.
    pub fn new(paths: Paths) -> Guests {
        Guests { paths, wide: None, narrow: None, notes: Vec::new() }
    }

    /// The call of a function of a library of Windows.
    ///
    /// The guest of the sixty-four bits is asked first, and the guest of
    /// the thirty-two bits is asked when the first does not have the
    /// library. A function that is not in the library is not looked for
    /// in the other guest: the library was there, and the function was
    /// not.
    pub fn call(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        let failure = {
            let wide = self.wide().map_err(the_failure)?;
            match wide.call(call) {
                Ok(answer) => return Ok(answer),
                Err(failure) => failure,
            }
        };
        if failure.kind != NativeFailureKind::NoLibrary {
            return Err(failure);
        }
        // The library is not in the folder of the guest of the sixty-four
        // bits: the guest of the thirty-two bits may have it.
        match self.narrow().map_err(the_failure)? {
            Some(narrow) => narrow.call(call),
            None => Err(failure),
        }
    }

    /// The notes of the guests: the places where the box was not what
    /// the script assumed.
    pub fn take_notes(&mut self) -> Vec<String> {
        std::mem::take(&mut self.notes)
    }

    /// The guest of the sixty-four bits, started when it is not there.
    fn wide(&mut self) -> Result<&mut Raven, String> {
        if self.wide.is_none() {
            let raven = Raven::start(&self.paths.program, &self.paths.helper)?;
            self.wide = Some(raven);
            self.notes.push("started a raven for the libraries of Windows".to_string());
        }
        Ok(self.wide.as_mut().expect("just started"))
    }

    /// The guest of the thirty-two bits, when the box has one.
    ///
    /// It starts when the library of the sixty-four bits is not there,
    /// and not before: a program of its own is not cheap to start.
    fn narrow(&mut self) -> Result<Option<&mut Raven>, String> {
        let guest = match &self.paths.helper32 {
            Some(guest) => guest.clone(),
            None => return Ok(None),
        };
        if self.narrow.is_none() {
            let raven = Raven::start(&self.paths.program, &guest)?;
            self.narrow = Some(raven);
            self.notes
                .push("started a raven of thirty-two bits for the libraries of thirty-two bits".to_string());
        }
        Ok(self.narrow.as_mut())
    }
}

// -- the protocol -------------------------------------------------------------

/// Whether the library is one of Windows, and not one of this machine.
///
/// A library of the machine is a path of this machine, or a name that
/// carries ".so" in it. Everything else is a library of Windows.
pub fn is_windows(library: &str) -> bool {
    // A library that is not named is the program itself, and the
    // program is the one of this machine.
    if library.is_empty() {
        return false;
    }
    // A path of this machine is a path of this machine: the slash is
    // the one of the machine. The backslash is the one of Windows, and
    // a path that carries it is a path of the Windows of the raven.
    if library.contains('/') {
        return false;
    }
    !library.contains(".so")
}

/// The request of a call, as the line the helper reads.
///
/// The line carries the library, the function, the type of the answer,
/// the number of the arguments, and then the type and the value of
/// every argument.
pub fn request(call: &NativeCall) -> String {
    let mut line = String::new();
    line.push_str(&call.library);
    line.push('\t');
    line.push_str(&call.function);
    line.push('\t');
    line.push_str(type_name(call.returns));
    line.push('\t');
    // The number of the arguments is in decimal: the helper reads it
    // as a number and not as digits.
    line.push_str(&call.arguments.len().to_string());
    for argument in &call.arguments {
        line.push('\t');
        line.push_str(type_name(argument.kind));
        // A star at the end of the type says that the argument is the
        // address of a value.
        if argument.address {
            line.push('*');
        }
        line.push('\t');
        line.push_str(&value_of(argument));
    }
    line
}

/// The answer of the helper, read into the result of the call.
///
/// The blocks that come back go into the arguments that are the address
/// of a value, as `NativeValue::Bytes`, the way the direct path of the
/// box leaves them.
pub fn answer(call: &mut NativeCall, line: &str) -> Result<NativeResult, NativeFailure> {
    let mut fields: Vec<&str> = line.split('\t').collect();
    // The helper ends the answer with a tab, and the last field of it
    // is empty.
    if fields.last() == Some(&"") {
        fields.pop();
    }
    let kind = fields.first().copied().unwrap_or("");
    if kind == "error" {
        // The reason is the message of v2.0.19: "Failed to load DLL.",
        // "Call to nonexistent function." and the rest.
        let reason = fields.get(1).copied().unwrap_or("the raven did not say why.");
        return Err(the_failure_of_the_reason(reason));
    }
    if kind != "ok" {
        return Err(NativeFailure::invalid(&format!(
            "the answer is not one the tree knows: {line}"
        )));
    }
    let digits = fields
        .get(1)
        .copied()
        .ok_or_else(|| NativeFailure::invalid("the answer has no value."))?;
    let returns = call.returns;
    // The blocks that come back go into the arguments that are the
    // address of a value, in the order of the arguments.
    let mut blocks = fields.get(2..).unwrap_or_default().iter();
    for argument in call.arguments.iter_mut() {
        if !argument.address {
            continue;
        }
        let block = blocks
            .next()
            .ok_or_else(|| NativeFailure::invalid("the answer has fewer blocks than the call asked for."))?;
        argument.value = NativeValue::Bytes(unhex(block).map_err(the_failure)?);
    }
    let value = if returns.is_text() {
        // A text answers with the bytes of the text, and the language
        // decodes them.
        NativeValue::Bytes(unhex(digits).map_err(the_failure)?)
    } else if returns.is_float() {
        // The answer of a float is the bits of the float.
        NativeValue::Float(ahk::dll::decode_number(
            &number(digits).map_err(the_failure)?.to_le_bytes(),
            returns,
        ))
    } else {
        // The answer of a whole number is the bits of the number, and
        // the type says how many of them are read.
        NativeValue::Integer(integer_answer(
            number(digits).map_err(the_failure)?,
            returns,
        ))
    };
    Ok(NativeResult { kind: returns, value })
}

/// The failure of the raven itself.
///
/// The raven is not the machine of the call: a failure of it is a failure of
/// the call that is not valid.
fn the_failure(reason: String) -> NativeFailure {
    NativeFailure::invalid(&reason)
}

/// The failure that the reason of the guest says.
///
/// The guest answers with the messages of v2.0.19: "Failed to load DLL."
/// and "Call to nonexistent function.". The kind is what the v1 turns into
/// the code that it writes in the `ErrorLevel`.
fn the_failure_of_the_reason(reason: &str) -> NativeFailure {
    match reason {
        NOT_THERE => NativeFailure::no_library(reason),
        NO_FUNCTION => NativeFailure::no_function(reason),
        _ => NativeFailure::invalid(reason),
    }
}

/// The name of a type, as the helper spells it.
fn type_name(kind: NativeType) -> &'static str {
    match kind {
        NativeType::Int => "int",
        NativeType::Int64 => "int64",
        NativeType::Short => "short",
        NativeType::Char => "char",
        NativeType::UChar => "uchar",
        NativeType::UShort => "ushort",
        NativeType::UInt => "uint",
        NativeType::UInt64 => "uint64",
        NativeType::Float => "float",
        NativeType::Double => "double",
        NativeType::Ptr => "ptr",
        NativeType::UPtr => "uptr",
        NativeType::Str => "str",
        NativeType::WStr => "wstr",
        NativeType::AStr => "astr",
    }
}

/// The value of an argument, as the hexadecimal digits of its bits.
///
/// A number is the bits of the whole number, and the type says how
/// many of them go on the wire: eight for a float of four bytes, and
/// sixteen for everything else. A text is the digits of its bytes, and
/// the bytes carry the nul at the end.
fn value_of(argument: &NativeArg) -> String {
    match &argument.value {
        NativeValue::Bytes(bytes) => hex(bytes),
        NativeValue::Integer(number) => match argument.kind {
            NativeType::Float => format!("{:08x}", (*number as f32).to_bits()),
            _ => format!("{:016x}", *number as u64),
        },
        NativeValue::Float(number) => match argument.kind {
            NativeType::Float => format!("{:08x}", (*number as f32).to_bits()),
            NativeType::Double => format!("{:016x}", number.to_bits()),
            _ => format!("{:016x}", *number as i64 as u64),
        },
    }
}

/// The bytes, as the hexadecimal digits of them.
fn hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(digit(byte >> 4));
        out.push(digit(byte & 15));
    }
    out
}

/// The bytes of the hexadecimal digits.
fn unhex(text: &str) -> Result<Vec<u8>, String> {
    if text.len() % 2 != 0 {
        return Err(format!("the answer has digits that are not a whole number of bytes: {text}"));
    }
    let mut bytes = Vec::with_capacity(text.len() / 2);
    for pair in text.as_bytes().chunks(2) {
        bytes.push(digit_of(pair[0])? << 4 | digit_of(pair[1])?);
    }
    Ok(bytes)
}

/// The hexadecimal digit of four bits.
fn digit(bits: u8) -> char {
    char::from_digit(bits as u32, 16).expect("four bits are a digit")
}

/// The value of a hexadecimal digit.
fn digit_of(byte: u8) -> Result<u8, String> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(format!("the answer has a digit that is not one: {}", byte as char)),
    }
}

/// The bits of a hexadecimal number.
fn number(text: &str) -> Result<u64, String> {
    u64::from_str_radix(text, 16)
        .map_err(|_| format!("the answer has a number that is not one: {text}"))
}

/// The answer of a function that answers a whole number.
///
/// The machine only defines the low bytes of the register for a narrow
/// answer: the rest of it is whatever the callee left there, and the
/// type says how to read it.
fn integer_answer(bits: u64, kind: NativeType) -> i64 {
    match kind {
        NativeType::Char => bits as u8 as i8 as i64,
        NativeType::UChar => bits as u8 as i64,
        NativeType::Short => bits as u16 as i16 as i64,
        NativeType::UShort => bits as u16 as i64,
        NativeType::Int => bits as u32 as i32 as i64,
        NativeType::UInt => bits as u32 as i64,
        _ => bits as i64,
    }
}

#[cfg(test)]
mod tests {
    //! The protocol, against the helper that reads it.
    //!
    //! The helper is the other half of the protocol: it is
    //! `crates/ahk-box/helper/helper.c`, and the two halves were proved
    //! together on the box on 2026-10-28. The cases here are the ones
    //! that say whether they agree: the line that goes out, and the
    //! line that comes back.

    use super::{Guests, Paths, Raven, Transport, answer, is_windows, request};
    use ahk::dll::{
        Convention, NativeArg, NativeCall, NativeFailureKind, NativeType, NativeValue,
    };
    use std::path::{Path, PathBuf};

    /// A call, with the library and the function of the cases.
    fn call(returns: NativeType, arguments: Vec<NativeArg>) -> NativeCall {
        NativeCall {
            library: "kernel32".to_string(),
            function: "GetTickCount".to_string(),
            convention: Convention::Standard,
            returns,
            arguments,
        }
    }

    /// A number, as an argument of a call.
    fn number(kind: NativeType, value: i64) -> NativeArg {
        NativeArg { kind, address: false, value: NativeValue::Integer(value) }
    }

    /// A text, as an argument of a call.
    fn text(kind: NativeType, value: &str) -> NativeArg {
        NativeArg { kind, address: false, value: NativeValue::Bytes(ahk::dll::encode(value, kind)) }
    }

    #[test]
    fn the_request_of_a_call_without_arguments_is_a_call() {
        let line = request(&call(NativeType::Int, Vec::new()));
        assert_eq!(line, "kernel32\tGetTickCount\tint\t0");
    }

    #[test]
    fn the_request_of_a_call_is_a_line() {
        // The guest reads the request until the end of the line: a
        // request that is not a line is a request it never answers.
        let mut guests = guests(&["ok\t000000000000002a\t"], &[]);
        let mut native = call(NativeType::Int, Vec::new());
        let answer = guests.call(&mut native).expect("the answer");
        assert_eq!(answer.value, NativeValue::Integer(42));
    }

    #[test]
    fn the_request_carries_the_bits_of_an_integer() {
        let line = request(&call(NativeType::Int64, vec![number(NativeType::Int64, 42)]));
        assert_eq!(line, "kernel32\tGetTickCount\tint64\t1\tint64\t000000000000002a");
    }

    #[test]
    fn the_request_carries_the_bytes_of_a_text() {
        let line = request(&call(NativeType::Int, vec![text(NativeType::AStr, "hi")]));
        assert_eq!(line, "kernel32\tGetTickCount\tint\t1\tastr\t686900");
    }

    #[test]
    fn the_request_of_the_address_of_a_value_carries_a_star() {
        let argument = NativeArg {
            kind: NativeType::Int64,
            address: true,
            value: NativeValue::Integer(42),
        };
        let line = request(&call(NativeType::Int, vec![argument]));
        assert_eq!(line, "kernel32\tGetTickCount\tint\t1\tint64*\t000000000000002a");
    }

    #[test]
    fn the_request_of_a_float_carries_the_bits_of_the_four_bytes() {
        let argument = NativeArg {
            kind: NativeType::Float,
            address: false,
            value: NativeValue::Float(1.5),
        };
        let line = request(&call(NativeType::Float, vec![argument]));
        assert_eq!(line, "kernel32\tGetTickCount\tfloat\t1\tfloat\t3fc00000");
    }

    #[test]
    fn the_answer_of_an_integer_is_a_number() {
        let mut native = call(NativeType::Int, Vec::new());
        let answer = answer(&mut native, "ok\t000000000000002a\t").expect("the answer");
        assert_eq!(answer.kind, NativeType::Int);
        assert_eq!(answer.value, NativeValue::Integer(42));
    }

    #[test]
    fn the_answer_of_a_float_is_a_number() {
        // The helper writes the bits of the float register, and the low
        // half of them is the float of four bytes.
        let mut native = call(NativeType::Float, Vec::new());
        let answer = answer(&mut native, "ok\t000000003fc00000\t").expect("the answer");
        assert_eq!(answer.kind, NativeType::Float);
        assert_eq!(answer.value, NativeValue::Float(1.5));
    }

    #[test]
    fn the_answer_of_a_text_is_its_bytes() {
        let mut native = call(NativeType::AStr, Vec::new());
        let answer = answer(&mut native, "ok\t6f6b\t").expect("the answer");
        assert_eq!(answer.kind, NativeType::AStr);
        let NativeValue::Bytes(bytes) = answer.value else {
            panic!("the answer is the bytes of the text");
        };
        assert_eq!(ahk::dll::decode(&bytes, NativeType::AStr), "ok");
    }

    #[test]
    fn the_blocks_come_back_into_the_arguments_that_are_the_address_of_a_value() {
        let number = NativeArg {
            kind: NativeType::Int64,
            address: true,
            value: NativeValue::Integer(0),
        };
        let text = NativeArg {
            kind: NativeType::AStr,
            address: true,
            value: NativeValue::Bytes(ahk::dll::encode("0000000000", NativeType::AStr)),
        };
        let mut native = call(NativeType::Int, vec![number, text]);
        let answer =
            answer(&mut native, "ok\t0000000000000001\t2a00000000000000\t3432\t").expect("the answer");
        assert_eq!(answer.value, NativeValue::Integer(1));
        let NativeValue::Bytes(bytes) = &native.arguments[0].value else {
            panic!("the bytes come back");
        };
        assert_eq!(ahk::dll::decode_number(bytes, NativeType::Int64), 42.0);
        let NativeValue::Bytes(bytes) = &native.arguments[1].value else {
            panic!("the bytes come back");
        };
        assert_eq!(ahk::dll::decode(bytes, NativeType::AStr), "42");
    }

    /// A transport of the cases: it answers with the lines it holds,
    /// and it fails when it has none left.
    #[derive(Debug)]
    struct Fake {
        answers: Vec<String>,
    }

    impl Fake {
        /// A transport that answers with these lines, one after the
        /// other.
        fn new(answers: &[&str]) -> Fake {
            Fake { answers: answers.iter().map(|line| line.to_string()).collect() }
        }
    }

    impl Transport for Fake {
        fn ask(&mut self, request: &str) -> Result<String, String> {
            if !request.ends_with('\n') {
                return Err("the request is not a line.".to_string());
            }
            if self.answers.is_empty() {
                return Err("the guest was asked and it had nothing to say.".to_string());
            }
            Ok(self.answers.remove(0))
        }
    }

    /// The two guests of a case, with the answers they give.
    fn guests(wide: &[&str], narrow: &[&str]) -> Guests {
        Guests {
            paths: Paths {
                program: PathBuf::from("raven"),
                helper: PathBuf::from("helper.exe"),
                helper32: Some(PathBuf::from("helper32.exe")),
            },
            wide: Some(Raven { transport: Box::new(Fake::new(wide)), process: None }),
            narrow: Some(Raven { transport: Box::new(Fake::new(narrow)), process: None }),
            notes: Vec::new(),
        }
    }

    #[test]
    fn the_guest_of_the_sixty_four_bits_answers_when_it_has_the_library() {
        let mut guests = guests(&["ok\t000000000000002a\t"], &[]);
        let mut native = call(NativeType::Int, Vec::new());
        let answer = guests.call(&mut native).expect("the answer");
        assert_eq!(answer.value, NativeValue::Integer(42));
    }

    #[test]
    fn the_guest_of_the_thirty_two_bits_answers_when_the_library_is_not_there() {
        let mut guests = guests(&["error\tFailed to load DLL."], &["ok\t0000000000000078\t"]);
        let mut native = call(NativeType::Int, Vec::new());
        let answer = guests.call(&mut native).expect("the answer");
        assert_eq!(answer.value, NativeValue::Integer(120));
    }

    #[test]
    fn a_function_that_is_not_in_the_library_is_not_looked_for_in_the_other_guest() {
        let mut guests = guests(&["error\tCall to nonexistent function."], &[]);
        let mut native = call(NativeType::Int, Vec::new());
        let error = guests.call(&mut native).expect_err("it fails");
        assert_eq!(error.reason, "Call to nonexistent function.");
        assert_eq!(error.kind, NativeFailureKind::NoFunction);
    }

    #[test]
    fn a_box_without_the_guest_of_the_thirty_two_bits_says_that_the_library_is_not_there() {
        let mut guests = guests(&["error\tFailed to load DLL."], &[]);
        guests.paths.helper32 = None;
        guests.narrow = None;
        let mut native = call(NativeType::Int, Vec::new());
        let error = guests.call(&mut native).expect_err("it fails");
        assert_eq!(error.reason, "Failed to load DLL.");
        assert_eq!(error.kind, NativeFailureKind::NoLibrary);
    }

    #[test]
    fn the_answer_of_a_failure_is_the_reason_of_v2() {
        let mut native = call(NativeType::Int, Vec::new());
        let error = answer(&mut native, "error\tFailed to load DLL.").expect_err("it fails");
        assert_eq!(error.reason, "Failed to load DLL.");
        assert_eq!(error.kind, NativeFailureKind::NoLibrary);
        let error = answer(&mut native, "error\tCall to nonexistent function.").expect_err("it fails");
        assert_eq!(error.reason, "Call to nonexistent function.");
        assert_eq!(error.kind, NativeFailureKind::NoFunction);
        // A reason that the raven does not know is a failure of the call
        // itself.
        let error = answer(&mut native, "error\tsomething else.").expect_err("it fails");
        assert_eq!(error.kind, NativeFailureKind::Invalid);
    }

    #[test]
    fn a_library_of_windows_is_not_a_library_of_the_machine() {
        assert!(is_windows("kernel32"));
        assert!(is_windows("user32.dll"));
        // A path of Windows has the backslash of Windows, and the
        // raven is the one that knows it.
        assert!(is_windows("C:\\windows\\system32\\user32.dll"));
        assert!(!is_windows("libc.so.6"));
        assert!(!is_windows("/usr/lib/libc.so.6"));
        assert!(!is_windows("/usr/lib/libahk.a"));
        assert!(!is_windows(""));
    }

    #[test]
    fn the_paths_of_the_box_are_read_out_of_the_environment() {
        // The environment belongs to the process and to every thread of
        // it, so the two cases are here in one test and not in two.
        let program = std::env::var_os("AHK_RAVEN");
        let helper = std::env::var_os("AHK_RAVEN_HELPER");
        let helper32 = std::env::var_os("AHK_RAVEN_HELPER32");
        // The environment of the runner is its own: nothing else is
        // reading these three variables while this happens.
        unsafe {
            std::env::set_var("AHK_RAVEN", "/usr/bin/raven");
            std::env::set_var("AHK_RAVEN_HELPER", "/usr/lib/ahk/helper.exe");
            std::env::set_var("AHK_RAVEN_HELPER32", "/usr/lib/ahk/helper32.exe");
        }
        let paths = Paths::from_environment().expect("the box has a raven");
        assert_eq!(paths.program, PathBuf::from("/usr/bin/raven"));
        assert_eq!(paths.helper, PathBuf::from("/usr/lib/ahk/helper.exe"));
        assert_eq!(paths.helper32, Some(PathBuf::from("/usr/lib/ahk/helper32.exe")));
        // A box that has not both has no raven.
        unsafe {
            std::env::remove_var("AHK_RAVEN_HELPER");
        }
        assert!(Paths::from_environment().is_none());
        // The guest of the thirty-two bits is not needed: a box that has
        // not it calls the libraries of the sixty-four bits and no more.
        unsafe {
            std::env::set_var("AHK_RAVEN_HELPER", "/usr/lib/ahk/helper.exe");
            std::env::remove_var("AHK_RAVEN_HELPER32");
        }
        let paths = Paths::from_environment().expect("the box has a raven");
        assert_eq!(paths.helper32, None);
        unsafe {
            match program {
                Some(value) => std::env::set_var("AHK_RAVEN", value),
                None => std::env::remove_var("AHK_RAVEN"),
            }
            match helper {
                Some(value) => std::env::set_var("AHK_RAVEN_HELPER", value),
                None => std::env::remove_var("AHK_RAVEN_HELPER"),
            }
            match helper32 {
                Some(value) => std::env::set_var("AHK_RAVEN_HELPER32", value),
                None => std::env::remove_var("AHK_RAVEN_HELPER32"),
            }
        }
    }

    #[test]
    fn a_raven_that_does_not_start_fails_with_the_reason() {
        let error = Raven::start(
            Path::new("/ahk/no-such-raven"),
            Path::new("/ahk/no-such-helper"),
        )
        .expect_err("it fails");
        assert!(error.contains("the raven did not start"), "{error}");
    }
}
