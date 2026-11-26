// SPDX-License-Identifier: Apache-2.0
//! The machine memory of the buffers of a script.
//!
//! The bytes of a `Buffer` live in a cell that the script and the machine
//! share: the address of the cell is the `Ptr` of the buffer, and the
//! machine writes into it while a native call goes on. `NumGet` reads the
//! same bytes, so what the machine left there is seen by the script
//! without a copy in between.
//!
//! The table below holds the buffers whose address was asked for: a
//! script that reads the memory of an address asks for the buffer that
//! holds it, and the address of a buffer is the address of its own bytes.
//!
//! The table belongs to one run of one thread: the script runs on its
//! own thread (the callbacks re-enter it there), and when the run is
//! over the table is let go, so the addresses of a run are not the
//! addresses of the next one.

use alloc::vec::Vec;
use std::cell::{Cell, RefCell};

use crate::runtime::{SharedCell, Value};

/// One buffer the machine has the address of: the bytes of it and the
/// object of the script it belongs to.
struct Mirror {
    bytes: SharedCell<Vec<u8>>,
    owner: Value,
}

/// One thread's table of buffer mirrors and its most-recently-hit index.
struct MirrorTable {
    entries: RefCell<Vec<Mirror>>,
    last: Cell<usize>,
}

thread_local! {
    static MIRRORS: MirrorTable = const {
        MirrorTable {
            entries: RefCell::new(Vec::new()),
            last: Cell::new(0),
        }
    };
}

/// Whether two values are the same object of the script.
fn the_same_object(first: &Value, second: &Value) -> bool {
    match (first, second) {
        (Value::Object(first), Value::Object(second)) => SharedCell::ptr_eq(first, second),
        _ => false,
    }
}

/// The address of the machine memory of a buffer.
///
/// The address is the address of the bytes of the buffer: the machine
/// writes into them, and the script reads them, and there is no copy of
/// the bytes in between. The mirror is made when the address is asked for
/// the first time; the later asks answer the same address while the size
/// is the same.
pub fn the_address_of_a_buffer(owner: &Value, bytes: &SharedCell<Vec<u8>>) -> i64 {
    MIRRORS.with(|table| {
        let address = bytes.borrow().as_ptr() as i64;
        let last = table.last.get();
        let mut mirrors = table.entries.borrow_mut();
        if let Some(mirror) = mirrors.get_mut(last) {
            if the_same_object(&mirror.owner, owner) {
                if !SharedCell::ptr_eq(&mirror.bytes, bytes) {
                    mirror.bytes = bytes.clone();
                }
                return address;
            }
        }
        for (index, mirror) in mirrors.iter_mut().enumerate() {
            if index == last {
                continue;
            }
            if the_same_object(&mirror.owner, owner) {
                // The bytes of the buffer may be a new block: the buffer was
                // made of another size, and the address of it is another one.
                if !SharedCell::ptr_eq(&mirror.bytes, bytes) {
                    mirror.bytes = bytes.clone();
                }
                table.last.set(index);
                return address;
            }
        }
        let index = mirrors.len();
        mirrors.push(Mirror {
            bytes: bytes.clone(),
            owner: owner.clone(),
        });
        table.last.set(index);
        address
    })
}

/// The buffer whose bytes hold the address, and the distance from the
/// start of them.
///
/// The address of a mirror is the address of its bytes, and the address
/// of a script is the address of its buffer plus the distance from the
/// start of it.
pub fn the_mirror_at(address: i64) -> Option<(Value, usize)> {
    MIRRORS.with(|table| {
        let mirrors = table.entries.borrow();
        let last = table.last.get();
        if let Some(mirror) = mirrors.get(last) {
            let block = mirror.bytes.borrow();
            let base = block.as_ptr() as i64;
            let end = base + block.len() as i64;
            if address >= base && address < end {
                return Some((mirror.owner.clone(), (address - base) as usize));
            }
        }
        for (index, mirror) in mirrors.iter().enumerate() {
            if index == last {
                continue;
            }
            let block = mirror.bytes.borrow();
            let base = block.as_ptr() as i64;
            let end = base + block.len() as i64;
            if address >= base && address < end {
                table.last.set(index);
                return Some((mirror.owner.clone(), (address - base) as usize));
            }
        }
        None
    })
}

/// Read `width` bytes directly from the mirror holding `address`, at
/// `address + offset`, without cloning the owner `Value` or borrowing the
/// owner's property table.
///
/// Returns `None` if no mirror holds `address`, `Some(Err(reason))` if the
/// range is out of bounds, or `Some(Ok(answer))` with the decoded value.
pub fn read_bytes_at<R>(
    address: i64,
    offset: i64,
    width: usize,
    read: impl Fn(&[u8]) -> R,
) -> Option<Result<R, &'static str>> {
    MIRRORS.with(|table| {
        let mirrors = table.entries.borrow();
        let last = table.last.get();
        let check = |mirror: &Mirror| -> Option<Result<R, &'static str>> {
            let block = mirror.bytes.borrow();
            let base = block.as_ptr() as i64;
            let end_addr = base + block.len() as i64;
            if address < base || address >= end_addr {
                return None;
            }
            let target = (address - base).checked_add(offset);
            let Some(start) = target.filter(|&n| n >= 0).map(|n| n as usize) else {
                return Some(Err("the offset of a buffer may not be negative."));
            };
            let end = start.saturating_add(width);
            if end > block.len() {
                return Some(Err("the bytes do not fit in the buffer."));
            }
            Some(Ok(read(&block[start..end])))
        };
        if let Some(mirror) = mirrors.get(last) {
            if let Some(result) = check(mirror) {
                return Some(result);
            }
        }
        for (index, mirror) in mirrors.iter().enumerate() {
            if index == last {
                continue;
            }
            if let Some(result) = check(mirror) {
                table.last.set(index);
                return Some(result);
            }
        }
        None
    })
}

/// Find the shared byte cell of the mirror holding `address`, if any.
///
/// Used by the fast loop executor to cache a buffer's backing cell across
/// iterations without re-entering thread-local storage on every `NumGet`.
pub fn find_mirror_cell(address: i64) -> Option<SharedCell<Vec<u8>>> {
    MIRRORS.with(|table| {
        let mirrors = table.entries.borrow();
        let last = table.last.get();
        if let Some(mirror) = mirrors.get(last) {
            let block = mirror.bytes.borrow();
            let base = block.as_ptr() as i64;
            let end = base + block.len() as i64;
            if address >= base && address < end {
                return Some(mirror.bytes.clone());
            }
        }
        for (index, mirror) in mirrors.iter().enumerate() {
            if index == last {
                continue;
            }
            let block = mirror.bytes.borrow();
            let base = block.as_ptr() as i64;
            let end = base + block.len() as i64;
            if address >= base && address < end {
                table.last.set(index);
                return Some(mirror.bytes.clone());
            }
        }
        None
    })
}

/// Let the mirrors of a run go: the addresses of a run do not outlive it.
pub fn forget_all() {
    MIRRORS.with(|table| {
        table.entries.borrow_mut().clear();
        table.last.set(0);
    });
}
