//! A call into a library of the machine.
//!
//! The `DllCall` of v2 is a call into a library: the library is
//! loaded, the function is found in it, and the arguments go in the
//! registers that the convention of the machine asks for. The shape of
//! the call is only known at the time of the call -- how many arguments
//! there are, and which of them are floats -- so the call goes through
//! a trampoline: the assembly fills the registers, and the answer comes
//! back in one of them.
//!
//! The trampoline was proved before it came here. The prototype in
//! `ahk_init/native_proto` answered thirteen cases on Windows and on the
//! lab box on 2026-10-28, and it is ported here as it is.
//!
//! Four things the prototype paid for once, and that are worth keeping:
//!
//! * The `asm!` of Rust does not take a name in the operand of an
//!   explicit register: `inout("r9") x => _` is a syntax error, and
//!   `inout("r9") _ => x` is what it wants.
//! * The block has to give the stack back -- `mov rsp, rbx` and `pop
//!   rbx` at the end -- or the epilogue of the function walks into the
//!   wrong stack.
//! * The stack that is saved has to live in a register the callee does
//!   not touch (`rbx`): a volatile one (`r10`) does not survive the call.
//! * `rbx` may not be an operand of the block on Linux -- the LLVM
//!   reserves it -- so it may only be used inside the text of the
//!   assembly.

use ahk::dll::{NativeCall, NativeFailure, NativeResult, NativeType, NativeValue};

// -- the machine ---------------------------------------------------------------

/// One argument of a native call, in the shape the machine wants.
enum Slot {
    /// An integer or a pointer.
    Integer(u64),
    /// A float of eight bytes.
    Float(f64),
    /// A float of four bytes.
    Float32(f32),
    /// The address of a block of bytes.
    Address(u64),
}

/// The call, prepared for the machine.
struct Call {
    /// The address of the function.
    target: u64,
    /// The arguments, in the order they are given.
    arguments: Vec<Slot>,
}

/// The answer of a native call.
struct Answer {
    /// The bits the function left in the integer register.
    bits: u64,
    /// The bits the function left in the float register.
    float_bits: u64,
}

// -- the trampoline ------------------------------------------------------------

/// The call, on Windows.
///
/// The register of an argument is the one of its position: the first
/// four arguments go in RCX, RDX, R8 and R9, and the float of a
/// position goes in the XMM of the same position.
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
fn trampoline(call: Call) -> Answer {
    let mut gpr = [0u64; 4];
    let mut xmm = [0u64; 4];
    let mut stack: Vec<u64> = Vec::new();
    for (index, argument) in call.arguments.iter().enumerate() {
        match argument {
            Slot::Integer(value) | Slot::Address(value) => {
                if index < 4 {
                    gpr[index] = *value;
                } else {
                    stack.push(*value);
                }
            }
            Slot::Float(value) => {
                if index < 4 {
                    xmm[index] = value.to_bits();
                    // A function that takes a variable number of arguments
                    // reads the float of a position in the register of the
                    // integers as well.
                    gpr[index] = value.to_bits();
                } else {
                    stack.push(value.to_bits());
                }
            }
            Slot::Float32(value) => {
                let bits = value.to_bits() as u64;
                if index < 4 {
                    xmm[index] = bits;
                    gpr[index] = bits;
                } else {
                    stack.push(bits);
                }
            }
        }
    }
    let mut bits = 0u64;
    let mut float_bits = 0u64;
    unsafe {
        core::arch::asm!(
            // The floats, by position. They go first: the register of the
            // pointers is free after this.
            "movq xmm0, [rdx]",
            "movq xmm1, [rdx + 8]",
            "movq xmm2, [rdx + 16]",
            "movq xmm3, [rdx + 24]",
            // The stack: the shadow of the four registers, and the
            // arguments that do not fit in a register. The pointer of the
            // stack is kept in a register the callee does not touch.
            "push rbx",
            "mov rbx, rsp",
            "mov rdx, r9",
            "add rdx, 1",
            "shr rdx, 1",
            "shl rdx, 4",
            "add rdx, 32",
            "sub rsp, rdx",
            "and rsp, -16",
            "xor eax, eax",
            "2:",
            "cmp rax, r9",
            "jae 3f",
            "mov rdx, [rcx + rax*8]",
            "mov [rsp + 32 + rax*8], rdx",
            "inc rax",
            "jmp 2b",
            "3:",
            // The integers, by position.
            "mov rax, r8",
            "mov rcx, [rax]",
            "mov rdx, [rax + 8]",
            "mov r8, [rax + 16]",
            "mov r9, [rax + 24]",
            // The call.
            "call r11",
            // The float of the answer comes first: the registers are
            // volatile.
            "movq r10, xmm0",
            // The stack goes back where it was.
            "mov rsp, rbx",
            "pop rbx",
            inout("r9") stack.len() => _,
            inout("rcx") stack.as_ptr() => _,
            inout("rdx") xmm.as_ptr() => _,
            inout("r8") gpr.as_ptr() => _,
            in("r11") call.target,
            lateout("rax") bits,
            lateout("r10") float_bits,
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            out("xmm3") _,
        );
    }
    Answer { bits, float_bits }
}

/// The call, on Linux.
///
/// The register of an argument is the one of its class: the integers go
/// in RDI, RSI, RDX, RCX, R8 and R9, and the floats go in XMM0 to
/// XMM7.
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn trampoline(call: Call) -> Answer {
    let mut gpr = [0u64; 8];
    let mut xmm = [0u64; 8];
    let mut stack: Vec<u64> = Vec::new();
    let mut integers = 0usize;
    let mut floats = 0usize;
    for argument in &call.arguments {
        match argument {
            Slot::Integer(value) | Slot::Address(value) => {
                if integers < 6 {
                    gpr[integers] = *value;
                } else {
                    stack.push(*value);
                }
                integers += 1;
            }
            Slot::Float(value) => {
                if floats < 8 {
                    xmm[floats] = value.to_bits();
                } else {
                    stack.push(value.to_bits());
                }
                floats += 1;
            }
            Slot::Float32(value) => {
                let bits = value.to_bits() as u64;
                if floats < 8 {
                    xmm[floats] = bits;
                } else {
                    stack.push(bits);
                }
                floats += 1;
            }
        }
    }
    // The number of the float registers goes in AL: a function that
    // takes a variable number of arguments reads it to know how many of
    // them it has to save.
    gpr[6] = floats.min(8) as u64;
    let mut bits = 0u64;
    let mut float_bits = 0u64;
    unsafe {
        core::arch::asm!(
            // The floats, by class. They go first: the register of the
            // pointers is free after this.
            "movq xmm0, [rsi]",
            "movq xmm1, [rsi + 8]",
            "movq xmm2, [rsi + 16]",
            "movq xmm3, [rsi + 24]",
            "movq xmm4, [rsi + 32]",
            "movq xmm5, [rsi + 40]",
            "movq xmm6, [rsi + 48]",
            "movq xmm7, [rsi + 56]",
            // The stack: the arguments that do not fit in a register. The
            // pointer of the stack is kept in a register the callee does
            // not touch.
            "push rbx",
            "mov rbx, rsp",
            "mov rdx, r9",
            "add rdx, 1",
            "shr rdx, 1",
            "shl rdx, 4",
            "sub rsp, rdx",
            "and rsp, -16",
            "xor eax, eax",
            "2:",
            "cmp rax, r9",
            "jae 3f",
            "mov rdx, [rcx + rax*8]",
            "mov [rsp + rax*8], rdx",
            "inc rax",
            "jmp 2b",
            "3:",
            // The integers, by class.
            "mov rax, r8",
            "mov rdi, [rax]",
            "mov rsi, [rax + 8]",
            "mov rdx, [rax + 16]",
            "mov rcx, [rax + 24]",
            "mov r8, [rax + 32]",
            "mov r9, [rax + 40]",
            // The number of the float registers, for a function that
            // takes a variable number of arguments.
            "mov al, [rax + 48]",
            // The call.
            "call r11",
            // The float of the answer comes first: the registers are
            // volatile.
            "movq r10, xmm0",
            // The stack goes back where it was.
            "mov rsp, rbx",
            "pop rbx",
            inout("r9") stack.len() => _,
            inout("rcx") stack.as_ptr() => _,
            inout("rsi") xmm.as_ptr() => _,
            inout("r8") gpr.as_ptr() => _,
            in("r11") call.target,
            lateout("rax") bits,
            lateout("r10") float_bits,
            out("rdi") _,
            out("xmm0") _,
            out("xmm1") _,
            out("xmm2") _,
            out("xmm3") _,
            out("xmm4") _,
            out("xmm5") _,
            out("xmm6") _,
            out("xmm7") _,
        );
    }
    Answer { bits, float_bits }
}

/// The call, on a machine the trampoline does not know.
///
/// The shape of the call is the shape of x64, and the tree carries that
/// one: a machine of another shape is not answered, because a wrong
/// answer is worse than none.
#[cfg(not(any(
    all(target_arch = "x86_64", target_os = "windows"),
    all(target_arch = "x86_64", target_os = "linux")
)))]
fn trampoline(_call: Call) -> Answer {
    Answer {
        bits: 0,
        float_bits: 0,
    }
}

/// Whether the machine has a trampoline.
#[cfg(any(
    all(target_arch = "x86_64", target_os = "windows"),
    all(target_arch = "x86_64", target_os = "linux")
))]
const SUPPORTED: bool = true;

/// Whether the machine has a trampoline.
#[cfg(not(any(
    all(target_arch = "x86_64", target_os = "windows"),
    all(target_arch = "x86_64", target_os = "linux")
)))]
const SUPPORTED: bool = false;

// -- the loader ----------------------------------------------------------------

/// The library, on Windows.
#[cfg(target_os = "windows")]
mod loader {
    use std::ffi::{CString, c_void};
    use std::os::raw::c_char;

    unsafe extern "system" {
        fn LoadLibraryW(path: *const u16) -> *mut c_void;
        fn GetModuleHandleW(module: *const u16) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
    }

    /// The library, loaded.
    ///
    /// An empty name is the process itself, and the handle of it is
    /// what the loader answers for it.
    pub fn load(path: &str) -> Option<usize> {
        let handle = if path.is_empty() {
            unsafe { GetModuleHandleW(std::ptr::null()) }
        } else {
            let wide: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
            unsafe { LoadLibraryW(wide.as_ptr()) }
        };
        if handle.is_null() {
            None
        } else {
            Some(handle as usize)
        }
    }

    /// The function, in the library.
    pub fn symbol(module: usize, name: &str) -> Option<u64> {
        let Ok(name) = CString::new(name) else {
            return None;
        };
        let address = unsafe { GetProcAddress(module as *mut c_void, name.as_ptr()) };
        if address.is_null() {
            None
        } else {
            Some(address as u64)
        }
    }
}

/// The library, on Linux.
#[cfg(target_os = "linux")]
mod loader {
    use std::ffi::{CString, c_void};
    use std::os::raw::c_char;

    unsafe extern "C" {
        fn dlopen(path: *const c_char, flags: i32) -> *mut c_void;
        fn dlsym(module: *mut c_void, name: *const c_char) -> *mut c_void;
    }

    const RTLD_NOW: i32 = 2;

    /// The library, loaded.
    ///
    /// An empty name is the process itself, and the loader answers for
    /// it the handle of the program.
    pub fn load(path: &str) -> Option<usize> {
        let handle = if path.is_empty() {
            unsafe { dlopen(std::ptr::null(), RTLD_NOW) }
        } else {
            let Ok(path) = CString::new(path) else {
                return None;
            };
            unsafe { dlopen(path.as_ptr(), RTLD_NOW) }
        };
        if handle.is_null() {
            None
        } else {
            Some(handle as usize)
        }
    }

    /// The function, in the library.
    pub fn symbol(module: usize, name: &str) -> Option<u64> {
        let Ok(name) = CString::new(name) else {
            return None;
        };
        let address = unsafe { dlsym(module as *mut c_void, name.as_ptr()) };
        if address.is_null() {
            None
        } else {
            Some(address as u64)
        }
    }
}

/// The library, on a machine the loader does not know.
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
mod loader {
    /// The library, loaded.
    pub fn load(_path: &str) -> Option<usize> {
        None
    }

    /// The function, in the library.
    pub fn symbol(_module: usize, _name: &str) -> Option<u64> {
        None
    }
}

// -- the libraries -------------------------------------------------------------

/// The libraries that are loaded.
///
/// A library is loaded once and kept: a script that calls the same
/// library twice does not load it twice. The handle of a library is a
/// number and not a pointer: a pointer is not `Send`, and the desktop
/// may have to cross a thread.
pub struct Libraries {
    loaded: Vec<(String, usize)>,
}

impl Default for Libraries {
    fn default() -> Self {
        Self::new()
    }
}

impl Libraries {
    /// A desktop that has not loaded anything yet.
    pub fn new() -> Self {
        Libraries { loaded: Vec::new() }
    }

    /// How many libraries are loaded.
    pub fn loaded(&self) -> usize {
        self.loaded.len()
    }

    /// The address of the function, in the library.
    ///
    /// The two failures are the ones v2.0.19 gives, measured on the
    /// binary on 2026-10-28: the library that does not load, and the
    /// function that is not in it. The kind of the failure says which of the
    /// two it is, and the v1 turns it into the code that it writes in the
    /// `ErrorLevel`.
    pub fn symbol(&mut self, library: &str, function: &str) -> Result<u64, NativeFailure> {
        let handle = match self.loaded.iter().find(|(name, _)| name == library) {
            Some((_, handle)) => *handle,
            None => {
                let handle = loader::load(library)
                    .ok_or_else(|| NativeFailure::no_library("Failed to load DLL."))?;
                self.loaded.push((library.to_string(), handle));
                handle
            }
        };
        loader::symbol(handle, function)
            .ok_or_else(|| NativeFailure::no_function("Call to nonexistent function."))
    }
}

// -- the call ------------------------------------------------------------------

/// The call, on the machine.
///
/// The library is loaded, the function is found in it, the arguments go
/// in the registers the machine wants, and the answer comes back.
pub fn call(
    libraries: &mut Libraries,
    native: &mut NativeCall,
) -> Result<NativeResult, NativeFailure> {
    if !SUPPORTED {
        return Err(NativeFailure::invalid(
            "this desktop cannot make a native call on this machine.",
        ));
    }
    let target = libraries.symbol(&native.library, &native.function)?;
    // The blocks have to stay alive during the call: the callee reads
    // the text of an argument, and it writes into the block of an
    // argument that is the address of a variable.
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let mut slots: Vec<Slot> = Vec::new();
    // The arguments that come back: the argument and the block it was
    // given.
    let mut writes: Vec<(usize, usize)> = Vec::new();
    for (index, argument) in native.arguments.iter().enumerate() {
        match &argument.value {
            // A text goes out by the address of its bytes.
            NativeValue::Bytes(bytes) => {
                let mut block = bytes.clone();
                if block.is_empty() {
                    // An empty text is a text of one nul: the callee is
                    // given a place to read, and not a place that is
                    // nowhere.
                    block.push(0);
                }
                blocks.push(block);
                let address = blocks[blocks.len() - 1].as_ptr() as u64;
                slots.push(Slot::Address(address));
                if argument.address {
                    // The callee may write into the text, and the
                    // language reads the text back into the variable of
                    // the script.
                    writes.push((index, blocks.len() - 1));
                }
            }
            NativeValue::Float(number) if !argument.address => {
                if argument.kind == NativeType::Float {
                    slots.push(Slot::Float32(*number as f32));
                } else {
                    slots.push(Slot::Float(*number));
                }
            }
            NativeValue::Float(number) => {
                // The address of a float is a block of its own: the
                // callee writes into it, and the language reads it back.
                blocks.push(ahk::dll::encode_number(*number, argument.kind));
                writes.push((index, blocks.len() - 1));
                let address = blocks[blocks.len() - 1].as_ptr() as u64;
                slots.push(Slot::Address(address));
            }
            NativeValue::Integer(number) if !argument.address => {
                slots.push(Slot::Integer(integer_bits(*number, argument.kind)));
            }
            NativeValue::Integer(number) => {
                blocks.push(ahk::dll::encode_number(*number as f64, argument.kind));
                writes.push((index, blocks.len() - 1));
                let address = blocks[blocks.len() - 1].as_ptr() as u64;
                slots.push(Slot::Address(address));
            }
        }
    }
    let answer = trampoline(Call {
        target,
        arguments: slots,
    });
    // The blocks come back into the arguments they belong to: the
    // argument is the way back to the variable of the script.
    for (argument, block) in writes {
        native.arguments[argument].value = NativeValue::Bytes(blocks[block].clone());
    }
    let value = if native.returns.is_text() {
        // The callee answers with the address of a text, and the bytes
        // of that text are the answer: the language decodes them.
        NativeValue::Bytes(read_text(answer.bits, native.returns))
    } else if native.returns.is_float() {
        let number = if native.returns == NativeType::Float {
            f32::from_bits(answer.float_bits as u32) as f64
        } else {
            f64::from_bits(answer.float_bits)
        };
        NativeValue::Float(number)
    } else {
        NativeValue::Integer(integer_answer(answer.bits, native.returns))
    };
    Ok(NativeResult {
        kind: native.returns,
        value,
    })
}

/// The bits of a whole number, in the width its type asks for.
///
/// A number that is narrower than the register of the machine goes in
/// the low bytes of it, and the rest of the register is of no matter to
/// the callee.
fn integer_bits(number: i64, kind: NativeType) -> u64 {
    match kind.width() {
        1 => number as u8 as u64,
        2 => number as u16 as u64,
        4 => number as u32 as u64,
        _ => number as u64,
    }
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

/// The bytes of the text at an address.
///
/// The text ends at the nul. The bound is a guard: a pointer that is
/// not a text at all would otherwise walk the whole memory of the
/// machine.
fn read_text(address: u64, kind: NativeType) -> Vec<u8> {
    if address == 0 {
        return Vec::new();
    }
    let width = kind.character_width();
    let nul = vec![0u8; width];
    let mut bytes = Vec::new();
    let mut cursor = address;
    for _ in 0..65_536 {
        let unit = unsafe { std::slice::from_raw_parts(cursor as *const u8, width) };
        if unit == nul.as_slice() {
            bytes.extend_from_slice(unit);
            break;
        }
        bytes.extend_from_slice(unit);
        cursor += width as u64;
    }
    bytes
}

#[cfg(test)]
mod tests {
    //! The call, against the libraries of the machine.
    //!
    //! The trampoline is the part that fails silently: an argument in
    //! the wrong register is a wrong answer, and nothing complains
    //! about it. So the cases here make real calls, against the
    //! libraries of the machine, and they are the ones that say whether
    //! the shape of the call is right.

    use super::{Libraries, NativeType, NativeValue, call};
    use ahk::dll::{Convention, NativeArg, NativeCall, NativeFailure, NativeResult};

    /// A library that is on the machine, whatever the machine is.
    #[cfg(target_os = "windows")]
    const A_LIBRARY: &str = "kernel32";

    /// A library that is on the machine, whatever the machine is.
    #[cfg(target_os = "linux")]
    const A_LIBRARY: &str = "libc.so.6";

    /// A function that is in the library of the machine.
    #[cfg(target_os = "windows")]
    const A_FUNCTION: &str = "GetTickCount";

    /// A function that is in the library of the machine.
    #[cfg(target_os = "linux")]
    const A_FUNCTION: &str = "getpid";

    /// The call, with the libraries of one desktop.
    fn run(
        library: &str,
        function: &str,
        returns: NativeType,
        arguments: Vec<NativeArg>,
    ) -> Result<NativeResult, NativeFailure> {
        let mut libraries = Libraries::new();
        let mut native = NativeCall {
            library: library.to_string(),
            function: function.to_string(),
            convention: Convention::Standard,
            returns,
            arguments,
        };
        call(&mut libraries, &mut native)
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn the_tick_count_of_the_machine_is_not_zero() {
        let answer =
            run("kernel32", "GetTickCount", NativeType::Int, Vec::new()).expect("the call");
        let NativeValue::Integer(number) = answer.value else {
            panic!("the answer is a number");
        };
        assert_ne!(number, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_pid_of_the_process_is_not_zero() {
        let answer = run("libc.so.6", "getpid", NativeType::Int, Vec::new()).expect("the call");
        let NativeValue::Integer(number) = answer.value else {
            panic!("the answer is a number");
        };
        assert_ne!(number, 0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_length_of_a_text_is_the_length_of_the_text() {
        let text = NativeArg {
            kind: NativeType::AStr,
            address: false,
            value: NativeValue::Bytes(ahk::dll::encode("hello", NativeType::AStr)),
        };
        let answer = run("libc.so.6", "strlen", NativeType::Int64, vec![text]).expect("the call");
        let NativeValue::Integer(number) = answer.value else {
            panic!("the answer is a number");
        };
        assert_eq!(number, 5);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_power_of_two_numbers_is_the_number() {
        // `pow` is not in the C library: it is in the one of the
        // mathematics, which glibc keeps as a library of its own.
        let arguments = vec![
            NativeArg {
                kind: NativeType::Double,
                address: false,
                value: NativeValue::Float(2.0),
            },
            NativeArg {
                kind: NativeType::Double,
                address: false,
                value: NativeValue::Float(10.0),
            },
        ];
        let answer = run("libm.so.6", "pow", NativeType::Double, arguments).expect("the call");
        let NativeValue::Float(number) = answer.value else {
            panic!("the answer is a number");
        };
        assert_eq!(number, 1024.0);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn the_text_that_is_the_address_of_a_variable_comes_back() {
        // The callee writes into the text, and the bytes it left there
        // come back into the argument: the argument is the way back to
        // the variable of the script.
        // `sprintf` writes into its first argument, and reads the shape
        // of the text from its second one.
        let text = NativeArg {
            kind: NativeType::AStr,
            address: true,
            value: NativeValue::Bytes(ahk::dll::encode("0000000000", NativeType::AStr)),
        };
        let format = NativeArg {
            kind: NativeType::AStr,
            address: false,
            value: NativeValue::Bytes(ahk::dll::encode("%d", NativeType::AStr)),
        };
        let number = NativeArg {
            kind: NativeType::Int,
            address: false,
            value: NativeValue::Integer(42),
        };
        let mut libraries = Libraries::new();
        let mut native = NativeCall {
            library: "libc.so.6".to_string(),
            function: "sprintf".to_string(),
            convention: Convention::Standard,
            returns: NativeType::Int,
            arguments: vec![text, format, number],
        };
        call(&mut libraries, &mut native).expect("the call");
        let NativeValue::Bytes(bytes) = &native.arguments[0].value else {
            panic!("the bytes come back");
        };
        assert_eq!(ahk::dll::decode(bytes, NativeType::AStr), "42");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn a_call_to_a_function_that_takes_a_variable_number_of_arguments_carries_the_float() {
        // The number of the float registers travels in AL, and the callee
        // reads it to know how many of them it has to save.
        let format = ahk::dll::encode("%f", NativeType::AStr);
        let arguments = vec![
            NativeArg {
                kind: NativeType::AStr,
                address: false,
                value: NativeValue::Bytes(format),
            },
            NativeArg {
                kind: NativeType::Double,
                address: false,
                value: NativeValue::Float(1.5),
            },
        ];
        let answer = run("libc.so.6", "printf", NativeType::Int, arguments).expect("the call");
        let NativeValue::Integer(number) = answer.value else {
            panic!("the answer is a number");
        };
        assert_eq!(number, 8);
    }

    #[test]
    fn a_library_that_is_not_there_fails() {
        let error = run(
            "ahk-no-such-library",
            "anything",
            NativeType::Int,
            Vec::new(),
        )
        .expect_err("it fails");
        assert_eq!(error.reason, "Failed to load DLL.");
        assert_eq!(error.kind, ahk::dll::NativeFailureKind::NoLibrary);
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[test]
    fn a_function_that_is_not_in_the_library_fails() {
        let error = run(
            A_LIBRARY,
            "ahk_no_such_function",
            NativeType::Int,
            Vec::new(),
        )
        .expect_err("it fails");
        assert_eq!(error.reason, "Call to nonexistent function.");
        assert_eq!(error.kind, ahk::dll::NativeFailureKind::NoFunction);
    }

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    #[test]
    fn a_library_that_is_called_twice_is_loaded_once() {
        let mut libraries = Libraries::new();
        for _ in 0..2 {
            let mut native = NativeCall {
                library: A_LIBRARY.to_string(),
                function: A_FUNCTION.to_string(),
                convention: Convention::Standard,
                returns: NativeType::Int,
                arguments: Vec::new(),
            };
            call(&mut libraries, &mut native).expect("the call");
        }
        assert_eq!(libraries.loaded(), 1);
    }
}
