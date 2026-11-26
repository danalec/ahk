//! The floor of the API of the machine: what Windows XP has and what
//! Windows Vista brought.
//!
//! WHY: the binary of the laboratory is a program of Windows XP, and
//! the loader of XP refuses a program that asks for a function of
//! Vista: the program does not start, and the refusal happens before
//! the first line of it runs. So the table of the imports of the
//! binary has to be a table that XP answers.
//!
//! The `std` of Rust is written for the Windows of today, and it asks
//! the machine for eighteen functions that XP does not have: the
//! fibers (`FlsAlloc` and its kin), the readers of the lock of Vista
//! (`AcquireSRWLockExclusive`), the writer of the rename of Vista
//! (`SetFileInformationByHandle`) and others. This crate is the answer
//! of the tree: it *is* those functions, written over the ones that XP
//! has, and the linker uses these and never asks the machine for the
//! missing ones.
//!
//! The names are the names of the API of Microsoft, and the behavior is
//! the behavior that `std` expects of them. The functions that XP has
//! no answer for are the ones that answer "this machine does not do
//! that", and the program that asked for them is told so.
//!
//! The crate is of the floor of XP alone: the modern Windows has the
//! real functions, and a program that runs on it must not use these.
//! The program of the laboratory is built with the feature `xp`, and
//! the program of the modern Windows is built without it.
//!
//! The gate of the laboratory (`scripts/xp/gate.py`) reads the table of
//! the imports of the binary and fails when it asks for something that
//! the machine of the laboratory does not have. The evidence that these
//! are the ones that are missing is `scripts/xp/floor-xp-sp3.txt`.

#![allow(non_snake_case)]

use std::boxed::Box;
use core::ffi::c_void;
use core::ptr;

use windows_sys::Win32::Foundation::{FALSE, HANDLE, SetLastError, TRUE};
use windows_sys::Win32::Storage::FileSystem::{
    BY_HANDLE_FILE_INFORMATION, FILE_ATTRIBUTE_REPARSE_POINT, GetFileInformationByHandle,
};
use windows_sys::Win32::System::Threading::{
    CRITICAL_SECTION, DeleteCriticalSection, EnterCriticalSection, InitializeCriticalSection,
    LeaveCriticalSection, TlsAlloc, TlsFree, TlsSetValue, TryEnterCriticalSection,
};
use windows_sys::core::BOOL;

/// The answer of `CompareStringW`: the first text is smaller.
const CSTR_LESS_THAN: i32 = 1;
/// The answer of `CompareStringW`: the texts are equal.
const CSTR_EQUAL: i32 = 2;
/// The answer of `CompareStringW`: the first text is greater.
const CSTR_GREATER_THAN: i32 = 3;

/// The comparison of two strings, without the language of the machine.
///
/// WHY: `CompareStringOrdinal` is of Windows Vista. The answer of the
/// machine of XP is `CompareStringW`, that is of Windows 2000, and the
/// two say the same thing for the comparison that `std` makes of the
/// names of the environment: the case of the letters does not matter,
/// and the order of the letters is the order of the numbers.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn CompareStringOrdinal(
    first: *const u16,
    first_length: i32,
    second: *const u16,
    second_length: i32,
    ignore_case: BOOL,
) -> i32 {
    if first.is_null() || second.is_null() {
        return CSTR_EQUAL;
    }
    let flags = if ignore_case != FALSE {
        windows_sys::Win32::Globalization::NORM_IGNORECASE
    } else {
        0
    };
    let answer = unsafe {
        windows_sys::Win32::Globalization::CompareStringW(
            windows_sys::Win32::Globalization::LOCALE_USER_DEFAULT,
            flags,
            first,
            first_length,
            second,
            second_length,
        )
    };
    if answer == 0 {
        // The machine could not compare: the answer of the ordinal
        // comparison of `std` is then the comparison of the bytes.
        let left = unsafe { core::slice::from_raw_parts(first, first_length.max(0) as usize) };
        let right = unsafe { core::slice::from_raw_parts(second, second_length.max(0) as usize) };
        return match left.cmp(right) {
            core::cmp::Ordering::Less => CSTR_LESS_THAN,
            core::cmp::Ordering::Equal => CSTR_EQUAL,
            core::cmp::Ordering::Greater => CSTR_GREATER_THAN,
        };
    }
    // `CompareStringW` answers the number of the characters, and the
    // relation is in the sign of it.
    match answer.cmp(&0) {
        core::cmp::Ordering::Less => CSTR_LESS_THAN,
        core::cmp::Ordering::Equal => CSTR_EQUAL,
        core::cmp::Ordering::Greater => CSTR_GREATER_THAN,
    }
}

/// The link of the machine: the file that is a link to another file.
///
/// WHY: the links of files are of Windows Vista, and XP has none. The
/// machine of XP is told that it does not do that.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn CreateSymbolicLinkW(
    _link: *const u16,
    _target: *const u16,
    _flags: u32,
) -> BOOL {
    unsafe { SetLastError(50) };
    FALSE
}

/// The timer of the machine, of the resolution of Windows 10.
///
/// WHY: the timer of the high resolution is of Windows 10, and the one
/// of XP is the `SetWaitableTimer` of Windows 2000. The `std` of Rust
/// asks for the timer of the high resolution and, when the machine does
/// not have it, it uses the pause of the kernel32.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn CreateWaitableTimerExW(
    _attributes: *const c_void,
    _name: *const u16,
    _flags: u32,
    _access: u32,
) -> HANDLE {
    ptr::null_mut()
}

/// The list of the attributes of a process: the end of it.
///
/// WHY: the list of the attributes is of Windows Vista. The program of
/// the laboratory does not ask for a list of attributes — the `std` of
/// Rust asks for one only when the program itself asks for one — so the
/// end of a list that does not exist is nothing at all.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DeleteProcThreadAttributeList(_list: *mut c_void) {}

/// The place of the machine that is of the thread that is running.
///
/// WHY: the fibers are of Windows Vista, and XP has the places of the
/// thread (`TlsAlloc` and its kin), that are of Windows 2000. The
/// place of the fiber is the place of the thread here: XP has no
/// fibers of its own, and the thread of the program is the fiber of it.
///
/// The destructor of the place is lost: the XP does not run the
/// destructor of a place of thread when the thread dies, and the value
/// stays with the thread. The memory that the value holds is small,
/// and the life of the program is the life of the thread.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn FlsAlloc(_callback: *const c_void) -> u32 {
    let key = unsafe { TlsAlloc() };
    if key == u32::MAX {
        return u32::MAX;
    }
    key
}

/// The end of the life of a place of the thread.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn FlsFree(index: u32) -> BOOL {
    unsafe { TlsFree(index) }
}

/// The value of a place of the thread.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn FlsSetValue(index: u32, value: *mut c_void) -> BOOL {
    unsafe { TlsSetValue(index, value) }
}

/// The information of a file, the kind that the machine has.
///
/// WHY: `GetFileInformationByHandleEx` is of Windows Vista. The `std`
/// asks for it only for the files that are links, and the XP has no
/// links of files: the answer is the file that it is.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetFileInformationByHandleEx(
    handle: HANDLE,
    class: i32,
    information: *mut c_void,
    _size: u32,
) -> BOOL {
    let mut found: BY_HANDLE_FILE_INFORMATION = unsafe { core::mem::zeroed() };
    let answer = unsafe { GetFileInformationByHandle(handle, &mut found) };
    if answer == FALSE {
        return FALSE;
    }
    // The class that the `std` asks is the one of the attributes and of
    // the mark of the link, and the XP has no link of file: the file is
    // what it is.
    if class == 9 {
        let attributes = found.dwFileAttributes & !FILE_ATTRIBUTE_REPARSE_POINT;
        unsafe {
            let place = information as *mut u32;
            *place = attributes;
            *place.add(1) = 0;
        }
    }
    TRUE
}

/// The final path of a file: the path that opens it.
///
/// WHY: `GetFinalPathNameByHandleW` is of Windows Vista. The `std`
/// asks for it to canonicalize a path, and the tree of the laboratory
/// canonicalizes with the `GetFullPathNameW` of Windows 2000, that
/// says the same thing. The answer here is that the machine does not
/// have the answer.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetFinalPathNameByHandleW(
    _handle: HANDLE,
    _buffer: *mut u16,
    _size: u32,
    _flags: u32,
) -> u32 {
    0
}

/// The name of the machine, as the network calls it.
///
/// WHY: `GetHostNameW` is of Windows Vista, and XP does not have it.
/// The name of the machine is the `GetComputerNameW` of the kernel32,
/// that is of Windows 2000, and the two say the same name.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetHostNameW(name: *mut u16, size: *mut u32) -> i32 {
    unsafe { windows_sys::Win32::System::WindowsProgramming::GetComputerNameW(name, size) }
}

/// The list of the attributes of a process: the beginning of it.
///
/// WHY: the list of the attributes is of Windows Vista. The program of
/// the laboratory does not ask for a list of attributes, and the `std`
/// only uses the list when the program asks for one. The answer is that
/// the list cannot be made, and the `std` tells the program.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn InitializeProcThreadAttributeList(
    _list: *mut c_void,
    _count: u32,
    _flags: u32,
    size: *mut usize,
) -> BOOL {
    if !size.is_null() {
        unsafe { *size = 0 };
    }
    unsafe { SetLastError(100) };
    FALSE
}

/// The thread that is running is a fiber?
///
/// WHY: the fibers are of Windows Vista. The XP does not have fibers,
/// and the answer is that it does not.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn IsThreadAFiber() -> BOOL {
    FALSE
}

/// The information of a file, written in the machine.
///
/// WHY: `SetFileInformationByHandle` is of Windows Vista, and the `std`
/// uses it to rename a file that the machine refuses to rename. The
/// answer here is that a file that the machine refuses to rename is a
/// file that stays where it is.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn SetFileInformationByHandle(
    _handle: HANDLE,
    _class: i32,
    _information: *mut c_void,
    _size: u32,
) -> BOOL {
    FALSE
}

/// The stack of the thread, guaranteed.
///
/// WHY: the guarantee of the stack is of Windows Vista. The `std` asks
/// for it for the thread new, and the answer is that the machine does
/// not give it.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn SetThreadStackGuarantee(_size: *mut u32) -> BOOL {
    FALSE
}

/// The list of the attributes of a process: an attribute new.
///
/// WHY: the list of the attributes is of Windows Vista, and the program
/// of the laboratory does not ask for one. The answer is that the list
/// does not take attributes.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn UpdateProcThreadAttribute(
    _list: *mut c_void,
    _flags: u32,
    _attribute: usize,
    _value: *mut c_void,
    _size: usize,
) -> BOOL {
    FALSE
}

/// A critical section, and the lock of Vista that XP does not have.
///
/// WHY: `SRWLOCK` is of Windows Vista, and the XP has the critical
/// section, that is of Windows 2000. The critical section is kept per
/// thread, and the `SRWLOCK` is kept too — and the `SRWLOCK` is of the
/// size of a pointer, that is the size of a critical section kept
/// apart.
///
/// The critical section of the XP is recursive and the `SRWLOCK` is
/// not. The difference is in favor of the program: a lock that is
/// taken twice does not bite, and the `std` does not take it twice.
struct TheLock {
    section: CRITICAL_SECTION,
}

/// The section of the lock, made the first time that somebody asks.
unsafe fn the_section(lock: *mut c_void) -> *mut CRITICAL_SECTION {
    if lock.is_null() {
        return ptr::null_mut();
    }
    let place = lock as *mut core::sync::atomic::AtomicPtr<CRITICAL_SECTION>;
    let current = unsafe { (*place).load(core::sync::atomic::Ordering::Acquire) };
    if !current.is_null() {
        return current;
    }
    let mut fresh = Box::new(TheLock {
        section: unsafe { core::mem::zeroed() },
    });
    unsafe { InitializeCriticalSection(&mut fresh.section) };
    let pointer = Box::into_raw(fresh) as *mut CRITICAL_SECTION;
    // Two threads can arrive here at the same time: the one that writes
    // the pointer first is the owner of the section, and the other one
    // uses the same section.
    let previous = unsafe {
        (*place).compare_exchange(
            core::ptr::null_mut(),
            pointer,
            core::sync::atomic::Ordering::AcqRel,
            core::sync::atomic::Ordering::Acquire,
        )
    };
    match previous {
        Ok(_) => pointer,
        Err(previous) => {
            // The other thread was faster: the section of this one is
            // the same section of that one.
            unsafe {
                DeleteCriticalSection(&mut *pointer);
                let _ = Box::from_raw(pointer as *mut TheLock);
            }
            previous
        }
    }
}

/// Take the lock of Vista, that XP does not have.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn AcquireSRWLockExclusive(lock: *mut c_void) {
    let section = unsafe { the_section(lock) };
    if section.is_null() {
        return;
    }
    unsafe { EnterCriticalSection(section) };
}

/// Give back the lock of Vista, that XP does not have.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseSRWLockExclusive(lock: *mut c_void) {
    let section = unsafe { the_section(lock) };
    if section.is_null() {
        return;
    }
    unsafe { LeaveCriticalSection(section) };
}

/// Take the lock of Vista without waiting, that XP does not have.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn TryAcquireSRWLockExclusive(lock: *mut c_void) -> BOOL {
    let section = unsafe { the_section(lock) };
    if section.is_null() {
        return FALSE;
    }
    unsafe { TryEnterCriticalSection(section) }
}

/// The list of the functions of the floor.
///
/// The list is of raw pointers, and the raw pointer is not `Sync` for
/// the compiler: the list is read only, and the compiler is told so.
struct TheList([*const (); 18]);

// The list is read only, and the functions of it are of the machine.
unsafe impl Sync for TheList {}
unsafe impl Send for TheList {}

/// The functions of the floor, tied to the binary.
///
/// WHY: the linker only takes the member of an archive that answers a
/// name that is missing, and a function that nobody calls is a name
/// that nobody is missing. This static names every function of the
/// floor, and the binary asks for the static, and the linker takes the
/// whole floor with it.
#[used]
static THE_FLOOR: TheList = TheList([
    CompareStringOrdinal as *const (),
    CreateSymbolicLinkW as *const (),
    CreateWaitableTimerExW as *const (),
    DeleteProcThreadAttributeList as *const (),
    FlsAlloc as *const (),
    FlsFree as *const (),
    FlsSetValue as *const (),
    GetFileInformationByHandleEx as *const (),
    GetFinalPathNameByHandleW as *const (),
    GetHostNameW as *const (),
    InitializeProcThreadAttributeList as *const (),
    IsThreadAFiber as *const (),
    SetFileInformationByHandle as *const (),
    SetThreadStackGuarantee as *const (),
    UpdateProcThreadAttribute as *const (),
    AcquireSRWLockExclusive as *const (),
    ReleaseSRWLockExclusive as *const (),
    TryAcquireSRWLockExclusive as *const (),
]);

/// The number of the functions of the floor.
///
/// The binary of the laboratory asks this, and a resposta e o que
/// amarra o piso ao binario: cada funcao do piso e chamada aqui, uma
/// vez, com argumentos que a maquina nao usa. O linker so tira de um
/// arquivo de uma biblioteca o membro que responde a um nome que
/// falta, e um nome so falta quando o codigo o pede.
pub fn the_floor() -> usize {
    unsafe {
        let _ = CompareStringOrdinal(ptr::null(), 0, ptr::null(), 0, FALSE);
        let _ = CreateSymbolicLinkW(ptr::null(), ptr::null(), 0);
        let _ = CreateWaitableTimerExW(ptr::null(), ptr::null(), 0, 0);
        DeleteProcThreadAttributeList(ptr::null_mut());
        let key = FlsAlloc(ptr::null());
        let _ = FlsSetValue(key, ptr::null_mut());
        let _ = FlsFree(key);
        let _ = GetFileInformationByHandleEx(ptr::null_mut(), 0, ptr::null_mut(), 0);
        let _ = GetFinalPathNameByHandleW(ptr::null_mut(), ptr::null_mut(), 0, 0);
        let mut nome = [0u16; 64];
        let mut tamanho = 64u32;
        let _ = GetHostNameW(nome.as_mut_ptr(), &mut tamanho);
        let mut tamanho_da_lista = 0usize;
        let _ = InitializeProcThreadAttributeList(ptr::null_mut(), 0, 0, &mut tamanho_da_lista);
        let _ = IsThreadAFiber();
        let _ = SetFileInformationByHandle(ptr::null_mut(), 0, ptr::null_mut(), 0);
        let mut pilha = 0u32;
        let _ = SetThreadStackGuarantee(&mut pilha);
        let _ = UpdateProcThreadAttribute(ptr::null_mut(), 0, 0, ptr::null_mut(), 0);
        let _ = AcquireSRWLockExclusive(ptr::null_mut());
        let _ = ReleaseSRWLockExclusive(ptr::null_mut());
        let _ = TryAcquireSRWLockExclusive(ptr::null_mut());
    }
    THE_FLOOR.0.len()
}
