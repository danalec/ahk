//! The callbacks of the machine: `RegisterCallback`.
//!
//! A callback is an address the machine calls, and the call runs a
//! function of the script. The tree makes the address out of a pool of
//! trampolines: each one is a function of the machine that knows its
//! place in the pool, and the place names the function of the script.
//!
//! The call of the machine arrives while a `DllCall` runs: the runtime
//! is lent to the machine for the time of the call, and the trampoline
//! takes the loan back to run the function. The callback runs on the
//! thread of the script, the one that makes the `DllCall`; a callback
//! the machine calls on another thread finds no loan and answers zero.
//!
//! The re-entry is the one of a callback of a foreign interface: the
//! native call is blocked while the callback runs, and the runtime is
//! not used by the caller until the call comes back. The loan is a
//! borrow and not a pointer: the caller of the native call lends its
//! borrow of the runtime, the callback of the machine borrows it in its
//! turn, and there is one borrow of the runtime alive at any moment.
//! A pointer to the runtime would be a second borrow of the same place
//! at the same time, and that is undefined behaviour: the runtime of
//! the script would be in an unknown state when the callback comes
//! back.
//!
//! Measured on the binary of AutoHotkey v1.1.37.02: the hook of a
//! script of the v1 runs on the thread of the script -- the same thread
//! of the machine -- and it is nested in the call of the machine: the
//! function of the script calls the machine, and the machine calls the
//! hook again. The nest of the two is the same in the tree and in the
//! v1: it breaks at the depth of thirty and one, the limit of the
//! machine. The v1 does not run the hook of another thread, and the
//! tree does not either.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use std::cell::{Cell, RefCell};
use std::sync::Mutex;

use crate::runtime::{Runtime, Value};

/// The most callbacks the pool carries.
const THE_SLOTS: usize = 64;

/// One registered callback: the function of the script and the number of
/// parameters it takes.
struct Entry {
    name: String,
    params: usize,
    func: Option<Value>,
    event_info: Option<i64>,
}

/// The pool of the callbacks, by their place.
static REGISTRY: Mutex<Vec<Option<Entry>>> = Mutex::new(Vec::new());

thread_local! {
    static DEPTH: Cell<u32> = const { Cell::new(0) };
    /// The slots of the native calls that go on, the last one first.
    ///
    /// They are pointers to the slots and not pointers to the runtime: the
    /// runtime is lent, and the slot is the one that keeps the loan. The
    /// pointers keep their provenance -- a number would not.
    static SLOTS: RefCell<Vec<*mut TheSlot>> = const { RefCell::new(Vec::new()) };
}

/// The slot of a native call: the runtime is lent to the machine while
/// the call goes on.
///
/// The loan is a borrow of the runtime and not a pointer to it. The
/// callback of the machine takes the borrow out of the slot for the time
/// of its run and puts it back, so the borrow that is alive while the
/// callback runs is the one the caller of the native call made -- and
/// the caller does not use the runtime until the call comes back.
struct TheSlot {
    /// The runtime, lent to the machine.
    the_runtime: Cell<Option<&'static mut Runtime<'static>>>,
}

/// Keep the runtime of the thread for the time of a native call, and
/// answer the place that was there before.
///
/// A call inside a call keeps the outer place: the monitor of a message
/// makes a `DllCall` while a modal dialog pumps, and the runtime has to
/// stay kept when the inner call comes back -- the box goes on pumping.
///
/// The loan of the runtime is a borrow that travels as a value: the
/// callback of the machine borrows it in its turn, and the caller of the
/// native call does not touch the runtime while the call goes on. The
/// lifetime of the loan is the call and no more, and the `static` of it
/// is the lie that the machine makes: the runtime is alive for the time
/// of the call, and it is the same runtime that the caller lent.
pub fn set_current(rt: &mut Runtime<'_>) -> usize {
    let rt: &'static mut Runtime<'static> = unsafe {
        std::mem::transmute::<&mut Runtime<'_>, &'static mut Runtime<'static>>(rt)
    };
    let slot = Box::into_raw(Box::new(TheSlot {
        the_runtime: Cell::new(Some(rt)),
    }));
    SLOTS.with(|slots| {
        let mut slots = slots.borrow_mut();
        let depth = slots.len();
        slots.push(slot);
        depth
    })
}

/// The thread that runs the script: the number of the thread of the
/// machine.
#[cfg(feature = "std")]
fn the_thread_of_the_script() -> std::thread::ThreadId {
    std::thread::current().id()
}

/// Give the thread back the place it had before the call.
///
/// The slot goes away with the loan: the borrow of the runtime ends
/// there, and the caller of the native call may use the runtime again.
///
/// The calls of the machine are nested, and the slot of a call is the
/// last of the stack. A stack that is deeper than the call expects means
/// that a call-site forgot its `clear_current`: the loan of that call
/// stays lent, and the runtime stays borrowed while the caller uses it.
/// The stack is not cleaned here: the loan of the call that comes back
/// is the one on the top, and the loan of the call that was forgotten is
/// the one under it.
pub fn clear_current(previous: usize) {
    SLOTS.with(|slots| {
        let mut slots = slots.borrow_mut();
        if slots.len() <= previous {
            // The calls are not balanced: there is no slot of this call.
            #[cfg(feature = "std")]
            if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
                eprintln!(
                    "the stack of the loans of the machine is at {} and the call expects {}",
                    slots.len(),
                    previous
                );
            }
            return;
        }
        let raw = slots.pop().expect("the slot of the call is there");
        unsafe { drop(Box::from_raw(raw)) };
    });
}

/// The slot of the native call that goes on, if there is one.
///
/// The thread may be in the way of its own end: the windows of the thread
/// are destroyed when it ends, and the procedure of each of them asks for
/// the slot of the call that goes on. The thread-local of the slots may be
/// gone by then, and the ask is answered with the absence of a slot: the
/// message goes to the procedure of the machine, and the thread ends.
fn the_slot() -> Option<*mut TheSlot> {
    SLOTS
        .try_with(|slots| slots.borrow().last().copied())
        .ok()
        .flatten()
}

/// Run the answer with the runtime of the thread, if it is lent.
///
/// The loan is taken out of the slot for the time of the answer and it is
/// put back when the answer is done. The answer of a machine that called
/// on a thread that has no loan is `None`: the callback does not run, and
/// the machine gets zero.
fn with_the_runtime<T>(answer: impl FnOnce(&mut Runtime<'_>) -> T) -> Option<T> {
    let slot = the_slot()?;
    let runtime = unsafe { (*slot).the_runtime.take() }?;
    let answer = answer(&mut *runtime);
    unsafe { (*slot).the_runtime.set(Some(runtime)) };
    Some(answer)
}

/// A message of the machine arrived while the runtime of the thread is
/// kept (a modal dialog pumps the messages of the thread): the handler
/// `OnMessage` registered for it runs now, and the number it answers is
/// the answer of the window procedure. When nothing is kept, or no
/// handler is registered, the message goes to the procedure of the
/// machine.
pub fn the_message_runs(msg: i64, wparam: i64, lparam: i64, hwnd: i64) -> Option<i64> {
    with_the_runtime(|rt| rt.the_message_runs(msg, wparam, lparam, hwnd)).flatten()
}

/// Register a callback and answer the address the machine calls.
///
/// The address is the trampoline of a place of the pool; the place keeps
/// the function of the script. The answer is zero when the pool is full.
pub fn register(name: &str, params: usize) -> usize {
    register_with_event_info(name, params, None)
}

/// Register a callback with an explicit `A_EventInfo` value (the 4th parameter
/// of v1 `RegisterCallback(Function, Options, ParamCount, EventInfo)`).
pub fn register_with_event_info(name: &str, params: usize, event_info: Option<i64>) -> usize {
    let mut registry = match REGISTRY.lock() {
        Ok(registry) => registry,
        Err(_) => return 0,
    };
    let slot = match registry.iter().position(|entry| entry.is_none()) {
        Some(slot) => slot,
        None => {
            if registry.len() >= THE_SLOTS {
                return 0;
            }
            registry.push(None);
            registry.len() - 1
        }
    };
    let address = THE_TRAMPOLINES[slot] as usize;
    registry[slot] = Some(Entry {
        name: name.to_string(),
        params,
        func: None,
        event_info: Some(event_info.unwrap_or(address as i64)),
    });
    address
}

/// The trampoline of the place of the pool that an address names.
///
/// The address that the machine calls is a number, and a number has no
/// provenance: a test that calls the trampoline through the number
/// cannot be checked by Miri. This gives the trampoline itself, and the
/// test calls it the way the machine calls it.
pub fn the_trampoline_of(
    address: usize,
) -> Option<extern "C" fn(usize, usize, usize, usize) -> usize> {
    THE_TRAMPOLINES
        .iter()
        .position(|trampoline| *trampoline as usize == address)
        .map(|slot| THE_TRAMPOLINES[slot])
}

/// `CallbackCreate`: register a function value (not only a name) and answer
/// the callable address of a trampoline of the pool.
pub fn register_value(func: Value, params: usize) -> usize {    let mut registry = match REGISTRY.lock() {
        Ok(registry) => registry,
        Err(_) => return 0,
    };
    let slot = match registry.iter().position(|entry| entry.is_none()) {
        Some(slot) => slot,
        None => {
            if registry.len() >= THE_SLOTS {
                return 0;
            }
            registry.push(None);
            registry.len() - 1
        }
    };
    registry[slot] = Some(Entry {
        name: String::new(),
        params,
        func: Some(func),
        event_info: None,
    });
    THE_TRAMPOLINES[slot] as usize
}

/// `CallbackFree`: let a place of the pool go.
pub fn free(address: usize) {
    release_if_callback(address);
}

/// Release a callback slot when `address` is one of the static trampolines
/// returned by `RegisterCallback` / `CallbackCreate`.
///
/// Returns `true` if `address` matched a trampoline (even if already freed),
/// so `DllCall("GlobalFree", "UPtr", cb)` in v1 scripts does not pass a
/// static `.text` function address to Win32 `GlobalFree`.
pub fn release_if_callback(address: usize) -> bool {
    let Some(slot) = THE_TRAMPOLINES.iter().position(|&t| t as usize == address) else {
        return false;
    };
    if let Ok(mut registry) = REGISTRY.lock() {
        if let Some(entry) = registry.get_mut(slot) {
            *entry = None;
        }
    }
    true
}

/// Run the function of a place of the pool with the arguments of the
/// machine.
///
/// The arguments are the numbers the machine put in the registers of
/// the call; the function takes as many as it was registered with. The
/// answer is the number the function answers.
///
/// Measured on the binary of AutoHotkey v1.1.37.02: the
/// registers of the call are the parameters of the function of the script,
/// in the order of the call. The hook of the messages of a script of the v1
/// is a `WH_CALLWNDPROC`, and the machine calls it with
/// `(nCode, wParam, lParam)`: the third number is the pointer of the
/// `CWPSTRUCT`, and the `NumGet(lP+0, 2*A_PtrSize, "UInt")` of the
/// a script of the v1 reads the message out of it. The probes are in
/// the probe of the measurement (three parameters), of four and of one and two.
fn invoke(slot: usize, args: [usize; 4]) -> usize {
    let entry = {
        let registry = match REGISTRY.lock() {
            Ok(registry) => registry,
            Err(_) => return 0,
        };
        match registry.get(slot).and_then(|entry| entry.as_ref()) {
            Some(entry) => Entry {
                name: entry.name.clone(),
                params: entry.params,
                func: entry.func.clone(),
                event_info: entry.event_info,
            },
            None => return 0,
        }
    };
    let values: Vec<Value> = args
        .iter()
        .take(entry.params)
        .map(|&arg| Value::Int(arg as i64))
        .collect();
    DEPTH.with(|d| d.set(d.get() + 1));
    #[cfg(feature = "std")]
    if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
        eprintln!(
            "the callback {} runs at the depth {} on the thread {:?} with args [0x{:X}, 0x{:X}, 0x{:X}, 0x{:X}]",
            entry.name,
            DEPTH.with(|d| d.get()),
            the_thread_of_the_script(),
            args[0],
            args[1],
            args[2],
            args[3],
        );
    }
    // The runtime is lent to the machine: the callback borrows it in the
    // turn of the loan, and the borrow of the caller of the native call
    // is not used while the callback runs. The two of them are never
    // borrows of the runtime at the same time.
    let answer = with_the_runtime(|rt| {
        let prev_event_info = if let Some(info) = entry.event_info {
            let prev = rt.lookup("A_EventInfo");
            rt.set_global("A_EventInfo", Value::Int(info));
            Some(prev)
        } else {
            None
        };
        let result = if let Some(func) = &entry.func {
            rt.the_callback_runs(func, &values)
        } else {
            // The callback of a name is the callback of the `RegisterCallback`
            // of the v1, and it is the one the hook of the messages of the
            // a script of the v1 uses. The runtime counts it as a callback of
            // the machine, the way it counts the callback of a value: the
            // pump of the events of the machine looks at the counter, and the
            // run of the callback must not pump the messages again.
            rt.the_callback_of_a_name_runs(&entry.name, &values)
        };
        if let Some(Some(val)) = prev_event_info {
            rt.set_global("A_EventInfo", val);
        }
        result
    });
    DEPTH.with(|d| d.set(d.get() - 1));
    match answer {
        Some(Ok(value)) => value.to_number().unwrap_or(0.0) as usize,
        Some(Err(failure)) => {
            #[cfg(feature = "std")]
            if std::env::var_os("AHK_DEBUG_CALLBACK").is_some() {
                eprintln!("the callback {} failed: {}", entry.name, failure.message);
            }
            0
        }
        // The machine called the callback on a thread that has no loan:
        // the function of the script does not run.
        None => 0,
    }
}

/// The trampoline of a place of the pool: it knows its place, and the
/// place names the function of the script.
extern "C" fn trampoline<const SLOT: usize>(
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
) -> usize {
    invoke(SLOT, [a0, a1, a2, a3])
}

macro_rules! trampoline_array {
    ($($index:literal),* $(,)?) => {
        static THE_TRAMPOLINES: [extern "C" fn(usize, usize, usize, usize) -> usize; THE_SLOTS] = [
            $(trampoline::<$index>,)*
        ];
    };
}

trampoline_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
);
