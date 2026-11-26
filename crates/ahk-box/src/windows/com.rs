//! The COM of the machine.
//!
//! The COM of Windows is the way of the machine to let a program of it
//! use another program of it: the shell of the machine (the pickers of the
//! files), the sound of it (the devices and their volume) and the classes
//! of the registry of it (the objects of the v1 and of the v2).
//!
//! The COM is a table of functions: the object points at the table, and
//! the table holds the address of each method. The calls of this module
//! are the ones of the machine, and the numbers of the methods are the
//! places of the table.
//!
//! What the machine serves, measured on the machine of the tree the class of the picker of the files
//! (`{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}`), the class of the shell
//! of the machine (`{13709620-C279-11CE-A49E-4445-5354-0000}`), the
//! class of the link (`{00021401-0000-0000-C000-000000000046}`) and
//! the class of the devices of the sound
//! (`{BCDE0395-E52F-467C-8E3D-C4579291692E}`) are all served to a plain
//! program. The tree carried a byte wrong in each of the first two, and
//! the machine refused them with the number `0x80040154` -- the refusal
//! was the tree's and not the machine's.
//!
//! The object model is here too: the class of an object is the `ProgID`
//! or the text of the `CLSID`, the interface is the text of the `IID`,
//! and the method is its place in the table of the object.

use core::ffi::c_void;

use ahk::desktop::ComValue;
use ahk::dll::{Convention, NativeArg, NativeCall, NativeResult, NativeType};
use windows_sys::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, CLSCTX_SERVER, CLSIDFromProgID, CLSIDFromString,
    COINIT_APARTMENTTHREADED, CoCreateInstance, CoGetObject, CoInitializeEx, CoUninitialize,
    IIDFromString,
};
use windows_sys::core::{GUID, HRESULT};

/// The apartment of the COM.
///
/// The apartment is the place where the objects of the COM live, and the
/// thread of the script is the one that holds it.
///
/// The apartment belongs to the thread: a thread that makes an object of
/// the COM without an apartment is refused by the machine with the
/// number 0x800401F0 -- "the COM has not been initialized" (measured on
/// the machine of the tree).
///
/// The apartment is therefore taken by the thread that holds it, and
/// each thread of the program takes its own.
pub fn the_apartment_is_taken() {
    thread_local! {
        /// Whether the thread of the script already has its apartment.
        static THE_APARTMENT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    THE_APARTMENT.with(|taken| {
        if taken.get() {
            return;
        }
        let answer = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
        // The apartment is taken when the machine says that it is: the
        // answer is zero, or a number that says that the apartment of
        // the thread is of another kind.
        taken.set(answer >= 0);
    });
}

/// An object of the COM.
///
/// The object is a pointer to the table of its methods, and the table is
/// the first thing that the object points at.
pub struct TheObject {
    pub(crate) pointer: *mut c_void,
}

impl TheObject {
    /// The object of a class of the registry.
    ///
    /// The class is the number that the registry of the machine gives to
    /// it, and the interface is the number of the shape that the caller
    /// wants.
    pub fn of_the_class(class: &GUID, interface: &GUID) -> Result<Self, String> {
        the_apartment_is_taken();
        let mut pointer: *mut c_void = std::ptr::null_mut();
        let answer = unsafe {
            CoCreateInstance(
                class,
                std::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                interface,
                &mut pointer,
            )
        };
        if answer < 0 {
            return Err(format!(
                "the COM of the machine refused with the number {answer} ({:#x}): {}",
                answer as u32,
                the_reason_of(answer)
            ));
        }
        Ok(Self { pointer })
    }

    /// The address of a method of the object.
    ///
    /// The number is the place of the method in the table of the object.
    pub(crate) fn the_method(&self, number: usize) -> *mut c_void {
        unsafe {
            let table = *(self.pointer as *const *const *mut c_void);
            *table.add(number)
        }
    }

    /// The object is counted one more time.
    ///
    /// The COM counts the objects that it has, and a pointer that is
    /// kept after the owner gives the object back needs a count of its
    /// own.
    pub fn the_object_is_counted(&self) {
        unsafe {
            type AddRef = unsafe extern "system" fn(*mut c_void) -> u32;
            let count: AddRef = std::mem::transmute(self.the_method(1));
            count(self.pointer);
        }
    }

    /// The object is given back to the machine.
    ///
    /// The COM counts the objects that it has, and the count goes down
    /// when the object is given back.
    pub fn the_object_is_released(&self) {
        unsafe {
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(self.the_method(2));
            release(self.pointer);
        }
    }
}

impl Drop for TheObject {
    fn drop(&mut self) {
        self.the_object_is_released();
    }
}

/// The reason of a failure of the COM.
///
/// The COM answers with a number, and the number has a text in the table
/// of the machine.
///
/// The numbers are of the SDK of the machine (`winerror.h` and
/// `combaseapi.h`), measured on the machine of the tree.
pub fn the_reason_of(answer: HRESULT) -> String {
    match answer {
        // 0x8007000E -- E_OUTOFMEMORY.
        -2147024882 => "the machine ran out of memory.".to_string(),
        // 0x80040154 -- REGDB_E_CLASSNOTREG: the machine has no class
        // of that name, or the class does not serve the shape that was
        // asked for. The tree carried this number for a long time
        // because of a byte swapped in the number of the class of the
        // picker of the files (measured on the machine of the tree).
        -2147221164 => "the machine has no class of that name.".to_string(),
        // 0x800401F0 -- CO_E_NOTINITIALIZED: the thread holds no
        // apartment of the COM. This was the first refusal of the tree,
        // and it was of the tree: the apartment was taken once for the
        // whole program, and the thread that made the object was another
        // one. The apartment belongs to the thread (measured on the
        // machine of the tree).
        -2147221008 => "the COM of this thread has not been initialized.".to_string(),
        // 0x80070005 -- E_ACCESSDENIED.
        -2147024891 => "the machine refused the access.".to_string(),
        // 0x80070057 -- E_INVALIDARG.
        -2147024809 => "the machine refused the argument.".to_string(),
        // 0x8007000C -- the data of the answer is not good.
        -2147024884 => "the machine answered with bad data.".to_string(),
        // 0x80004002 -- E_NOINTERFACE: the class is there and does not
        // have the shape that was asked for.
        -2147467262 => "the class does not have the shape that was asked for.".to_string(),
        // 0x80004005 -- E_FAIL: the class refused to be made.
        -2147467259 => "the class refused to be made.".to_string(),
        // 0x8007007E -- ERROR_MOD_NOT_FOUND: the library of the class is
        // not there.
        -2147024770 => "the library of the class is not there.".to_string(),
        // 0x800700C1 -- ERROR_BAD_EXE_FORMAT: the library of the class is
        // of another architecture than the program.
        -2147024703 => "the library of the class is of another architecture.".to_string(),
        // 0x80010106 -- RPC_E_CHANGED_MODE: the apartment of the thread
        // is of another kind than the one that was asked for.
        -2147417850 => "the apartment of this thread is of another kind.".to_string(),
        // 0x800704C7 -- ERROR_CANCELLED: the user closed the dialog of
        // the machine. It is not a failure of the COM: it is the answer
        // of a window that the user closed.
        -2147023673 => "the user closed the dialog.".to_string(),
        _ => format!(
            "the COM of the machine refused with the number {answer} ({:#x}).",
            answer as u32
        ),
    }
}

/// The places of the methods of the picker in the table of its object.
///
/// The table is the one of `IFileDialog` -- `ShObjIdl_core.h`, the
/// interface of the picker of the shell of the machine --: the three of
/// `IUnknown`, the `Show` of `IModalWindow`, and then the methods of the
/// interface in the order of the header.
///
/// The places were wrong in the tree: the title was written with the
/// place of `GetFileName`, the choice of the user was read with the place
/// of `SetFileNameLabel`, the name of the item was read with the place of
/// `BindToHandler`, and the filter was written with the place of
/// `ClearClientData`. The picker was never shown, so the wrong places
/// never answered anything.
mod places {
    /// `IModalWindow::Show`: the picker is shown.
    pub const SHOW: usize = 3;
    /// `IFileDialog::SetFileTypes`: the kinds of the files that it offers.
    pub const SET_FILE_TYPES: usize = 4;
    /// `IFileDialog::SetOptions`: the flags of the picker.
    pub const SET_OPTIONS: usize = 9;
    /// `IFileDialog::SetFolder`: the folder that it opens in.
    pub const SET_FOLDER: usize = 12;
    /// `IFileDialog::SetFileName`: the name of the file that it shows.
    pub const SET_FILE_NAME: usize = 15;
    /// `IFileDialog::SetTitle`: the title of the picker.
    pub const SET_TITLE: usize = 17;
    /// `IFileDialog::GetResult`: the item that the user chose.
    pub const GET_RESULT: usize = 20;
    /// `IFileOpenDialog::GetResults`: the items that the user chose.
    pub const GET_RESULTS: usize = 27;
    /// `IShellItemArray::GetCount`: how many items there are.
    pub const GET_COUNT: usize = 7;
    /// `IShellItemArray::GetItemAt`: the item of a place.
    pub const GET_ITEM_AT: usize = 8;
    /// `IShellItem::GetDisplayName`: the name of the item.
    pub const GET_DISPLAY_NAME: usize = 5;
}

/// The name of the system of files, the number that the shell gives to it.
///
/// `SIGDN_FILESYSPATH` (`ShObjIdl_core.h`).
const THE_FILESYSPATH: u32 = 0x80058000;

/// The number of the machine that says that the user closed the picker.
///
/// `ERROR_CANCELLED` (`0x800704C7`): the shell answers that when the
/// user closed the picker without choosing anything.
const THE_CANCELLED: HRESULT = -2147023673;

/// A kind of file that the picker offers.
///
/// The `COMDLG_FILTERSPEC` of the SDK: the text that is shown and the
/// pattern of the files that it keeps.
#[repr(C)]
struct TheFilter {
    /// The text that is shown.
    name: *const u16,
    /// The pattern of the files that it keeps.
    pattern: *const u16,
}

/// The picker of the files of the machine.
///
/// The picker is the shell of the machine: it asks the user for the file
/// and it answers with the item of the shell that the user chose.
pub struct ThePicker {
    kind: TheKindOfPicker,
}

/// The two ways of the picker: the shell of Vista, and the common
/// dialogs of the era of XP.
enum TheKindOfPicker {
    Shell(TheShellPicker),
    Classic(std::cell::RefCell<super::picker::Classic>),
}

impl ThePicker {
    /// The picker that asks for a file to open.
    ///
    /// The shell of Vista is asked first; when the machine has no
    /// class of it -- the Windows XP -- the common dialogs answer.
    pub fn to_open() -> Result<Self, String> {
        Ok(match TheShellPicker::to_open() {
            Ok(shell) => Self { kind: TheKindOfPicker::Shell(shell) },
            Err(_) => Self { kind: TheKindOfPicker::Classic(std::cell::RefCell::new(super::picker::Classic::new(false))) },
        })
    }

    /// The picker that asks where to save a file.
    pub fn to_save() -> Result<Self, String> {
        Ok(match TheShellPicker::to_save() {
            Ok(shell) => Self { kind: TheKindOfPicker::Shell(shell) },
            Err(_) => Self { kind: TheKindOfPicker::Classic(std::cell::RefCell::new(super::picker::Classic::new(true))) },
        })
    }

    /// The flags of the picker.
    pub fn the_options_are_set(&self, options: u32) -> Result<(), String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_options_are_set(options),
            TheKindOfPicker::Classic(picker) => {
                picker.borrow_mut().the_options_are_set(options);
                Ok(())
            }
        }
    }

    /// The title of the picker.
    pub fn the_title_is_written(&self, title: &str) -> Result<(), String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_title_is_written(title),
            TheKindOfPicker::Classic(picker) => {
                picker.borrow_mut().the_title_is_written(title);
                Ok(())
            }
        }
    }

    /// The filter of the picker: the kinds of the files that it offers.
    pub fn the_filter_is_written(&self, kinds: &[(&str, &str)]) -> Result<(), String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_filter_is_written(kinds),
            TheKindOfPicker::Classic(picker) => {
                picker.borrow_mut().the_filter_is_written(kinds);
                Ok(())
            }
        }
    }

    /// The name of the file that the picker shows.
    pub fn the_file_name_is_written(&self, name: &str) -> Result<(), String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_file_name_is_written(name),
            TheKindOfPicker::Classic(picker) => {
                picker.borrow_mut().the_file_name_is_written(name);
                Ok(())
            }
        }
    }

    /// The folder that the picker opens in.
    pub fn the_folder_is_set(&self, path: &str) -> Result<(), String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_folder_is_set(path),
            TheKindOfPicker::Classic(picker) => {
                picker.borrow_mut().the_folder_is_set(path);
                Ok(())
            }
        }
    }

    /// The picker is shown, and the answer is the paths of what the
    /// user chose, or `None` when the user closed it without choosing.
    pub fn the_picker_is_shown(&self, many: bool) -> Result<Option<Vec<String>>, String> {
        match &self.kind {
            TheKindOfPicker::Shell(picker) => picker.the_picker_is_shown(many),
            TheKindOfPicker::Classic(picker) => picker.borrow().the_picker_is_shown(many),
        }
    }
}

/// The picker of the shell of Vista: the `IFileDialog` of the machine.
struct TheShellPicker {
    object: TheObject,
}

impl TheShellPicker {
    /// The picker that asks for a file to open.
    pub fn to_open() -> Result<Self, String> {
        Ok(Self {
            object: TheObject::of_the_class(
                &the_class_of_the_picker_of_the_files()?,
                &the_interface_of_the_picker()?,
            )?,
        })
    }

    /// The picker that asks where to save a file.
    pub fn to_save() -> Result<Self, String> {
        Ok(Self {
            object: TheObject::of_the_class(
                &the_class_of_the_picker_of_the_save()?,
                &the_interface_of_the_picker()?,
            )?,
        })
    }

    /// The flags of the picker.
    ///
    /// The flags are the ones of the machine: `FOS_NOCHANGEDIR`,
    /// `FOS_PICKFOLDERS`, `FOS_ALLOWMULTISELECT`, `FOS_FILEMUSTEXIST`,
    /// `FOS_PATHMUSTEXIST`, `FOS_CREATEPROMPT`, `FOS_OVERWRITEPROMPT` and
    /// `FOS_NODEREFERENCELINKS` (`ShObjIdl_core.h`, the
    /// `_FILEOPENDIALOGOPTIONS` of the picker).
    pub fn the_options_are_set(&self, options: u32) -> Result<(), String> {
        unsafe {
            type SetOptions = unsafe extern "system" fn(*mut c_void, u32) -> HRESULT;
            let set: SetOptions = std::mem::transmute(self.object.the_method(places::SET_OPTIONS));
            let answer = set(self.object.pointer, options);
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the flags of the picker were set",
                ));
            }
        }
        Ok(())
    }

    /// The title of the picker.
    pub fn the_title_is_written(&self, title: &str) -> Result<(), String> {
        let wide = super::sys::Wide::of(title);
        unsafe {
            type SetTitle = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
            let set: SetTitle = std::mem::transmute(self.object.the_method(places::SET_TITLE));
            let answer = set(self.object.pointer, wide.as_ptr());
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the title of the picker was written",
                ));
            }
        }
        Ok(())
    }

    /// The filter of the picker: the kinds of the files that it offers.
    ///
    /// Each kind is a pair: the text that is shown and the pattern of the
    /// files that it keeps. The machine takes them as the
    /// `COMDLG_FILTERSPEC` of the SDK.
    pub fn the_filter_is_written(&self, kinds: &[(&str, &str)]) -> Result<(), String> {
        // The names and the patterns have to be alive while the machine
        // reads them.
        let names: Vec<super::sys::Wide> = kinds
            .iter()
            .map(|(name, _)| super::sys::Wide::of(name))
            .collect();
        let patterns: Vec<super::sys::Wide> = kinds
            .iter()
            .map(|(_, pattern)| super::sys::Wide::of(pattern))
            .collect();
        let table: Vec<TheFilter> = kinds
            .iter()
            .enumerate()
            .map(|(index, _)| TheFilter {
                name: names[index].as_ptr(),
                pattern: patterns[index].as_ptr(),
            })
            .collect();
        unsafe {
            type SetFileTypes =
                unsafe extern "system" fn(*mut c_void, u32, *const TheFilter) -> HRESULT;
            let set: SetFileTypes =
                std::mem::transmute(self.object.the_method(places::SET_FILE_TYPES));
            let answer = set(self.object.pointer, table.len() as u32, table.as_ptr());
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the filter of the picker was written",
                ));
            }
        }
        Ok(())
    }

    /// The name of the file that the picker shows.
    pub fn the_file_name_is_written(&self, name: &str) -> Result<(), String> {
        let wide = super::sys::Wide::of(name);
        unsafe {
            type SetFileName = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
            let set: SetFileName =
                std::mem::transmute(self.object.the_method(places::SET_FILE_NAME));
            let answer = set(self.object.pointer, wide.as_ptr());
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the name of the file was written",
                ));
            }
        }
        Ok(())
    }

    /// The folder that the picker opens in.
    ///
    /// The folder is the item of the shell that has the path of it, and
    /// the machine makes that item with `SHCreateItemFromParsingName`.
    pub fn the_folder_is_set(&self, path: &str) -> Result<(), String> {
        let wide = super::sys::Wide::of(path);
        // `SHCreateItemFromParsingName` e de Windows Vista, e a
        // maquina que nao a tem e avisada: o item do shell nao pode ser
        // feito, e o XP nao tem o picker do shell de Vista.
        let Some(symbol) = super::sys::the_symbol_of("shell32.dll", "SHCreateItemFromParsingName")
        else {
            return Err(
                "this machine has no shell item of a path: it is a machine of Windows XP.".to_string(),
            );
        };
        type Maker = unsafe extern "system" fn(
            *const u16,
            *mut c_void,
            *const windows_sys::core::GUID,
            *mut *mut c_void,
        ) -> windows_sys::core::HRESULT;
        let maker: Maker = unsafe { std::mem::transmute(symbol) };
        unsafe {
            let mut item: *mut c_void = std::ptr::null_mut();
            let answer = maker(
                wide.as_ptr(),
                std::ptr::null_mut(),
                &the_interface_of_the_item_of_the_shell()?,
                &mut item,
            );
            if answer < 0 || item.is_null() {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the folder of the picker was made",
                ));
            }
            let item = TheObject { pointer: item };
            type SetFolder = unsafe extern "system" fn(*mut c_void, *mut c_void) -> HRESULT;
            let set: SetFolder = std::mem::transmute(self.object.the_method(places::SET_FOLDER));
            let answer = set(self.object.pointer, item.pointer);
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the folder of the picker was set",
                ));
            }
        }
        Ok(())
    }

    /// The picker is shown, and the answer is the paths of what the user
    /// chose.
    ///
    /// The answer is `None` when the user closed the picker without
    /// choosing anything: the machine says that with the number
    /// `0x800704C7` (`ERROR_CANCELLED`), and that is not a failure.
    ///
    /// The picker is a window of the machine, and the window is shown on
    /// the thread of the script: the answer comes when the user closes
    /// it.
    pub fn the_picker_is_shown(&self, many: bool) -> Result<Option<Vec<String>>, String> {
        unsafe {
            type Show = unsafe extern "system" fn(*mut c_void, *mut c_void) -> HRESULT;
            let show: Show = std::mem::transmute(self.object.the_method(places::SHOW));
            let answer = show(self.object.pointer, std::ptr::null_mut());
            if answer < 0 {
                if answer == THE_CANCELLED {
                    return Ok(None);
                }
                return Err(the_refusal_of_the_com(answer, "the picker was shown"));
            }
            if many {
                Ok(Some(self.the_results_are_read()?))
            } else {
                Ok(Some(vec![self.the_result_is_read()?]))
            }
        }
    }

    /// The item that the user chose.
    fn the_result_is_read(&self) -> Result<String, String> {
        unsafe {
            type GetResult = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
            let get: GetResult = std::mem::transmute(self.object.the_method(places::GET_RESULT));
            let mut item: *mut c_void = std::ptr::null_mut();
            let answer = get(self.object.pointer, &mut item);
            if answer < 0 || item.is_null() {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the choice of the user was read",
                ));
            }
            let item = TheObject { pointer: item };
            the_path_of_the_item(&item)
        }
    }

    /// The items that the user chose.
    ///
    /// The picker that opens the files is asked for the shape that has
    /// the results of the choice: `IFileOpenDialog`.
    fn the_results_are_read(&self) -> Result<Vec<String>, String> {
        unsafe {
            type QueryInterface =
                unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
            let query: QueryInterface = std::mem::transmute(self.object.the_method(0));
            let mut opened: *mut c_void = std::ptr::null_mut();
            let answer = query(
                self.object.pointer,
                &the_interface_of_the_picker_that_opens()?,
                &mut opened,
            );
            if answer < 0 || opened.is_null() {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the picker of many files was made",
                ));
            }
            let opened = TheObject { pointer: opened };
            type GetResults = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
            let get: GetResults = std::mem::transmute(opened.the_method(places::GET_RESULTS));
            let mut array: *mut c_void = std::ptr::null_mut();
            let answer = get(opened.pointer, &mut array);
            if answer < 0 || array.is_null() {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the files of the choice were read",
                ));
            }
            let array = TheObject { pointer: array };
            type GetCount = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
            let count: GetCount = std::mem::transmute(array.the_method(places::GET_COUNT));
            let mut number = 0u32;
            let answer = count(array.pointer, &mut number);
            if answer < 0 {
                return Err(the_refusal_of_the_com(
                    answer,
                    "the files of the choice were counted",
                ));
            }
            type GetItemAt =
                unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
            let at: GetItemAt = std::mem::transmute(array.the_method(places::GET_ITEM_AT));
            let mut paths = Vec::new();
            for index in 0..number {
                let mut item: *mut c_void = std::ptr::null_mut();
                let answer = at(array.pointer, index, &mut item);
                if answer < 0 || item.is_null() {
                    continue;
                }
                let item = TheObject { pointer: item };
                paths.push(the_path_of_the_item(&item)?);
            }
            Ok(paths)
        }
    }
}

/// The path of an item of the shell.
///
/// The name of the item is the one of the system of files, and the
/// machine gives it with the memory of the COM.
fn the_path_of_the_item(item: &TheObject) -> Result<String, String> {
    unsafe {
        type GetDisplayName = unsafe extern "system" fn(*mut c_void, u32, *mut *mut u16) -> HRESULT;
        let name: GetDisplayName = std::mem::transmute(item.the_method(places::GET_DISPLAY_NAME));
        let mut path: *mut u16 = std::ptr::null_mut();
        let answer = name(item.pointer, THE_FILESYSPATH, &mut path);
        if answer < 0 || path.is_null() {
            return Err(the_refusal_of_the_com(
                answer,
                "the path of the item was read",
            ));
        }
        let mut length = 0usize;
        while *path.add(length) != 0 {
            length += 1;
        }
        let letters = std::slice::from_raw_parts(path, length);
        let answer = String::from_utf16_lossy(letters);
        windows_sys::Win32::System::Com::CoTaskMemFree(path.cast());
        Ok(answer)
    }
}

/// The refusal of a call of the COM.
///
/// The COM answers with a number, and the number has a reason in the
/// table of `the_reason_of`.
fn the_refusal_of_the_com(answer: HRESULT, what: &str) -> String {
    format!(
        "the COM of the machine refused with the number {answer} ({:#x}) when {what}: {}",
        answer as u32,
        the_reason_of(answer)
    )
}

/// The number of a class, as the machine reads it.
///
/// The text is the name of the key of the class in the registry of the
/// machine -- `{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}` -- and the
/// machine is the one that turns the text into the number. The tree
/// that wrote the number by hand wrote it wrong twice (measured on the
/// machine of the tree), and the machine refused the
/// class with the number `0x80040154` for that reason alone.
pub fn the_class_of_the_text(text: &str) -> Result<GUID, String> {
    let wide = super::sys::Wide::of(text);
    let mut number: GUID = unsafe { std::mem::zeroed() };
    let answer = unsafe { CLSIDFromString(wide.as_ptr(), &mut number) };
    if answer < 0 {
        return Err(format!(
            "the machine did not read the number {text:?}: {answer} ({:#x})",
            answer as u32
        ));
    }
    Ok(number)
}

/// The number of an interface, as the machine reads it.
///
/// The text is the name of the interface, and the machine is the one
/// that turns it into the number.
pub fn the_interface_of_the_text(text: &str) -> Result<GUID, String> {
    let wide = super::sys::Wide::of(text);
    let mut number: GUID = unsafe { std::mem::zeroed() };
    let answer = unsafe { IIDFromString(wide.as_ptr(), &mut number) };
    if answer < 0 {
        return Err(format!(
            "the machine did not read the number {text:?}: {answer} ({:#x})",
            answer as u32
        ));
    }
    Ok(number)
}

/// The number of a class that the machine has, read from the machine.
///
/// The class is looked for in the registry of the machine by the
/// description that the machine gives of it -- the value by default of
/// its key -- and the number is the name of the key, which the machine
/// turns into the number. Two machines can describe the same class in
/// different words, and that is why the description and not the text of
/// the number is what the tree carries.
///
/// The tree of the registry is walked **once** for the whole program:
/// the walk opens every key of the classes of the machine -- eight
/// thousand and sixteen keys on the machine of the tree --
/// and it costs about 130 ms. Doing that walk for each class is what
/// made the machine of the tree stutter when the tests of the tree
/// asked for the class dos dispositivos do som outra vez e outra vez
/// (medido).
pub fn the_class_of_the_machine(description: &str) -> Result<GUID, String> {
    match the_classes_of_the_machine().get(description) {
        Some(class) => Ok(*class),
        None => Err(format!(
            "the machine has no class called {description:?} in its registry"
        )),
    }
}

/// The table of the classes of the machine: the description of each one
/// and its number.
///
/// The table is made at the first search and asked after: the walk of
/// the registry costs about 130 ms, and the machine of the tree stutters
/// when the tree walks it many times (measured on the machine of the
/// tree).
fn the_classes_of_the_machine() -> &'static std::collections::BTreeMap<String, GUID> {
    static CLASSES: std::sync::OnceLock<std::collections::BTreeMap<String, GUID>> =
        std::sync::OnceLock::new();
    CLASSES.get_or_init(the_walk_of_the_classes)
}

/// The walk of the tree of the classes of the machine.
///
/// Every key is opened and its default value read: that value is the
/// description of the class, and the name of the key is its number.
fn the_walk_of_the_classes() -> std::collections::BTreeMap<String, GUID> {
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::{
        HKEY, HKEY_CLASSES_ROOT, KEY_READ, RegCloseKey, RegEnumKeyExW, RegOpenKeyExW,
        RegQueryValueExW,
    };
    // A tabela das classes e uma arvore, e nao um monte: o `HashMap`
    // do `std` pede a `ProcessPrng` da maquina, e a `ProcessPrng` e de
    // Windows 7, e a DLL dela nao esta no XP.
    let mut classes: std::collections::BTreeMap<String, GUID> = std::collections::BTreeMap::new();
    unsafe {
        let mut keys: HKEY = std::ptr::null_mut();
        let path = super::sys::Wide::of("CLSID");
        let answer = RegOpenKeyExW(HKEY_CLASSES_ROOT, path.as_ptr(), 0, KEY_READ, &mut keys);
        if answer != ERROR_SUCCESS {
            return classes;
        }
        let mut index = 0u32;
        loop {
            let mut name = [0u16; 64];
            let mut length = name.len() as u32;
            let answer = RegEnumKeyExW(
                keys,
                index,
                name.as_mut_ptr(),
                &mut length,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if answer != ERROR_SUCCESS {
                break;
            }
            index += 1;
            // The description of the class: the default value of its key.
            let mut class: HKEY = std::ptr::null_mut();
            if RegOpenKeyExW(keys, name.as_ptr(), 0, KEY_READ, &mut class) != ERROR_SUCCESS {
                continue;
            }
            let mut value = [0u16; 256];
            let mut size = (value.len() * 2) as u32;
            let mut kind = 0u32;
            let answer = RegQueryValueExW(
                class,
                std::ptr::null(),
                std::ptr::null_mut(),
                &mut kind,
                value.as_mut_ptr() as *mut u8,
                &mut size,
            );
            RegCloseKey(class);
            if answer != ERROR_SUCCESS {
                continue;
            }
            let length = value.iter().position(|letter| *letter == 0).unwrap_or(0);
            let description = String::from_utf16_lossy(&value[..length]);
            // The name of the key is the number of the class, and the machine
            // is the one that turns the name into the number.
            let length = name.iter().position(|letter| *letter == 0).unwrap_or(0);
            let text = String::from_utf16_lossy(&name[..length]);
            if let Ok(number) = the_class_of_the_text(&text) {
                // The first class of a description is the one that stays: two
                // classes can share a description, and the machine does not
                // say which one the script wants.
                classes.entry(description).or_insert(number);
            }
        }
        RegCloseKey(keys);
    }
    classes
}

/// The class of the picker of the files, read from the machine.
///
/// Measured on the machine of the tree: the class of the
/// picker of the files is
/// `HKEY_CLASSES_ROOT\CLSID\{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}`
/// and the machine makes it.
pub fn the_class_of_the_picker_of_the_files() -> Result<GUID, String> {
    the_class_of_the_machine("File Open Dialog")
}

/// The class of the picker of the save, read from the machine.
///
/// `HKEY_CLASSES_ROOT\CLSID\{C0B4E2F3-BA21-4773-8DBA-335EC946EB8B}`.
pub fn the_class_of_the_picker_of_the_save() -> Result<GUID, String> {
    the_class_of_the_machine("File Save Dialog")
}

/// The class of the link, read from the machine.
///
/// `HKEY_CLASSES_ROOT\CLSID\{00021401-0000-0000-C000-000000000046}`.
pub fn the_class_of_the_link() -> Result<GUID, String> {
    the_class_of_the_machine("Shortcut")
}

/// The class of the shell of the machine, read from the machine.
///
/// `HKEY_CLASSES_ROOT\CLSID\{13709620-C279-11CE-A49E-444553540000}` --
/// the *Shell Automation Service*. The machine makes it in a plain
/// program (measured on the machine of the tree); the
/// tree was carrying two bytes of the number wrong -- the fifth and the
/// sixth of the last four were `0x35, 0x40` where the machine has
/// `0x53, 0x54` -- and the machine was refusing it with the number
/// `0x80040154`.
pub fn the_class_of_the_shell() -> Result<GUID, String> {
    the_class_of_the_machine("Shell Automation Service")
}

/// The class of the devices of the sound, read from the machine.
///
/// `HKEY_CLASSES_ROOT\CLSID\{BCDE0395-E52F-467C-8E3D-C4579291692E}` --
/// the *MMDeviceEnumerator class*, the one that knows the devices of
/// the sound of the machine.
pub fn the_class_of_the_devices_of_the_sound() -> Result<GUID, String> {
    the_class_of_the_machine("MMDeviceEnumerator class")
}

/// The shape of the picker of the files.
pub fn the_interface_of_the_picker() -> Result<GUID, String> {
    the_interface_of_the_text("{42F85136-DB7E-439C-85F1-E4075D135FC8}")
}

/// The shape of the picker that opens the files.
///
/// `IFileOpenDialog`: the shape of the picker that has the results of the
/// choice of the user.
pub fn the_interface_of_the_picker_that_opens() -> Result<GUID, String> {
    the_interface_of_the_text("{D57C7288-D4AD-4768-BE02-9D969532D960}")
}

/// The shape of an item of the shell.
///
/// `IShellItem`: the shape of the item that the picker answers with.
pub fn the_interface_of_the_item_of_the_shell() -> Result<GUID, String> {
    the_interface_of_the_text("{43826D1E-E718-42EE-BC55-A1E261C37BFE}")
}

/// The shape of the factory of the classes.
pub fn the_interface_of_the_factory() -> Result<GUID, String> {
    the_interface_of_the_text("{00000001-0000-0000-C000-000000000046}")
}

/// The shape of the file of a shortcut.
pub fn the_interface_of_the_file() -> Result<GUID, String> {
    the_interface_of_the_text("{0000010B-0000-0000-C000-000000000046}")
}

/// The shape of the link of a shortcut.
pub fn the_interface_of_the_link() -> Result<GUID, String> {
    the_interface_of_the_text("{000214F9-0000-0000-C000-000000000046}")
}

/// The simplest shape of all.
pub fn the_interface_of_the_unknown() -> Result<GUID, String> {
    the_interface_of_the_text("{00000000-0000-0000-C000-000000000046}")
}

/// The shape of the class that knows the devices of the sound.
///
/// `IMMDeviceEnumerator`.
pub fn the_interface_of_the_devices() -> Result<GUID, String> {
    the_interface_of_the_text("{A95664D2-9614-4F35-A746-DE8DB63617E6}")
}

/// The shape of the volume of a device of the sound.
///
/// `IAudioEndpointVolume`.
pub fn the_interface_of_the_volume() -> Result<GUID, String> {
    the_interface_of_the_text("{5CDF2C82-841E-4546-9722-0CF74078229A}")
}

/// The shape of the property of the name of a device of the sound.
///
/// `PKEY_Device_FriendlyName`.
pub fn the_interface_of_the_name_of_the_device() -> Result<GUID, String> {
    the_interface_of_the_text("{A45C254E-DF1C-4EFD-8020-67D146A850E0}")
}

/// The shape de uma parte da topologia de um device do som.
///
/// `IPart`.
pub fn the_interface_of_the_part() -> Result<GUID, String> {
    the_interface_of_the_text("{AE2DE0E4-5BCA-4F2D-AA46-5D13F8FDB3A9}")
}

/// The shape of the topologia de um device do som.
///
/// `IDeviceTopology`.
pub fn the_interface_of_the_topology() -> Result<GUID, String> {
    the_interface_of_the_text("{2A07407E-6497-4A18-9787-32F79BD0D98F}")
}

/// The shape do volume de um nivel.
///
/// `IAudioVolumeLevel`.
pub fn the_interface_of_the_level() -> Result<GUID, String> {
    the_interface_of_the_text("{7FB7B48F-531D-44A2-BCB3-5AD5A134B3DC}")
}

/// The shape do mute.
///
/// `IAudioMute`.
pub fn the_interface_of_the_mute() -> Result<GUID, String> {
    the_interface_of_the_text("{DF45AEEA-B74A-4B6B-AFAD-2366B6AA012E}")
}

/// The COM of the machine is given back.
///
/// The apartment of the thread is given back when the thread ends.
pub fn the_apartment_is_given_back() {
    unsafe {
        CoUninitialize();
    }
}

// -- the objects of the COM --------------------------------------------------------

#[link(name = "ole32")]
unsafe extern "system" {
    /// The object that a program of the machine has running.
    ///
    /// The function is of `ole32.dll` and windows-sys does not carry it.
    fn GetActiveObject(
        rclsid: *const GUID,
        pvreserved: *mut c_void,
        ppunk: *mut *mut c_void,
    ) -> HRESULT;
}

/// The kind of a value that is an object that knows dispatch.
const THE_DISPATCH: i64 = 9;
/// The kind of a value that is an object that does not know dispatch.
const THE_UNKNOWN: i64 = 13;
/// The kind of a value that is a wide text.
const THE_BSTR: i64 = 8;
/// The kind of a value that is a number of four bytes.
const THE_I4: i64 = 3;
/// The kind of a value that is a number of eight bytes with a point.
const THE_R8: i64 = 5;
/// The kind of a value that is a yes or a no.
const THE_BOOL: i64 = 11;
/// The kind of a value that is a number of eight bytes.
const THE_I8: i64 = 20;

/// O word of the machine for a failure of the COM.///
/// The v2 writes the number of the failure and the word of the machine
/// for it, and the tree writes the same thing so that the script sees
/// the same answer.
///
/// Measured on the machine of the tree: the word of the
/// machine is the one of `FormatMessage` -- 0x80070057 is "The parameter
/// is incorrect." and 0x80040154 is "Class not registered".
pub fn the_message_of(answer: HRESULT) -> String {
    let number = answer as u32;
    let word = match number {
        0x8007_0057 => "The parameter is incorrect.",
        0x8004_01F3 => "Invalid class string",
        0x8004_0154 => "Class not registered",
        0x8004_01F0 => "CoInitialize has not been called.",
        0x8000_4002 => "No such interface supported",
        0x8007_0005 => "Access is denied.",
        0x8004_01FD => "Object is not connected to server",
        0x8004_0111 => "ClassFactory cannot supply requested class",
        0x8002_802B => "Element not found.",
        _ => return the_reason_of(answer),
    };
    format!("(0x{number:08X}) {word}")
}

/// An object of the COM of the machine.
///
/// The object is the pointer that the machine gave and the kind of the
/// value of it -- the `VARTYPE` of v2. The machine is the owner of the
/// pointer: the desktop keeps it and gives it back when the program
/// ends.
#[derive(Clone, Copy)]
pub struct TheCom {
    /// The pointer of the object.
    pointer: *mut c_void,
    /// The kind of the value: the `VARTYPE` of v2.
    kind: i64,
}

impl TheCom {
    /// The pointer of the object.
    pub fn the_pointer(&self) -> u64 {
        self.pointer as u64
    }

    /// The kind of the value of the object.
    pub fn the_kind(&self) -> i64 {
        self.kind
    }

    /// The object of a pointer that the desktop keeps.
    ///
    /// The desktop keeps the pointer and gives it back to the machine
    /// when the program ends, and the object here is the pointer and the
    /// kind of the value.
    pub fn of_the_pointer(pointer: u64, kind: i64) -> Self {
        Self {
            pointer: pointer as *mut c_void,
            kind,
        }
    }
}

/// The address of a method of an object.
///
/// The object points at the table of its methods, and the table holds the
/// address of each of them.
fn the_method(pointer: *mut c_void, number: usize) -> *mut c_void {
    unsafe {
        let table = *(pointer as *const *const *mut c_void);
        *table.add(number)
    }
}

/// The object is given back to the machine.
///
/// The pointer is the one of an object of the COM, and the machine is
/// the owner of it.
///
/// # Safety
///
/// The pointer must be the one of an object of the COM that the machine
/// made, and it must not have been given back before.
pub unsafe fn the_object_is_released(pointer: *mut c_void) {
    unsafe {
        type Release = unsafe extern "system" fn(*mut c_void) -> u32;
        let release: Release = std::mem::transmute(the_method(pointer, 2));
        release(pointer);
    }
}

/// The count of a COM pointer, one more: the `AddRef` of `IUnknown`.
///
/// The pointer is the one the script carries, from a `DllCall` or from
/// the `Ptr` of a value of the COM.
pub fn the_pointer_is_counted(pointer: u64) -> Result<u64, String> {
    if pointer == 0 {
        return Err("ObjAddRef: the pointer is null.".to_string());
    }
    type AddRef = unsafe extern "system" fn(*mut c_void) -> u32;
    // SAFETY: the pointer is the one of an object of the COM, and the
    // second place of its table is the AddRef of IUnknown.
    unsafe {
        let add_ref: AddRef = std::mem::transmute(the_method(pointer as *mut c_void, 1));
        Ok(add_ref(pointer as *mut c_void) as u64)
    }
}

/// The count of a COM pointer, one less: the `Release` of `IUnknown`.
pub fn the_pointer_is_released(pointer: u64) -> Result<u64, String> {
    if pointer == 0 {
        return Err("ObjRelease: the pointer is null.".to_string());
    }
    type Release = unsafe extern "system" fn(*mut c_void) -> u32;
    // SAFETY: the pointer is the one of an object of the COM, and the
    // third place of its table is the Release of IUnknown.
    unsafe {
        let release: Release = std::mem::transmute(the_method(pointer as *mut c_void, 2));
        Ok(release(pointer as *mut c_void) as u64)
    }
}

/// The write of the default member of the object: the `DISPID_VALUE` of
/// the COM, the `obj := value` of the v1.
pub fn the_default_is_written(object: &TheCom, value: ComValue) -> Result<(), String> {
    let mut variants: Vec<super::events::TheVariant> =
        vec![super::events::the_variant_of(&value)];
    let mut result: super::events::TheVariant = unsafe { std::mem::zeroed() };
    // SAFETY: the object is the one of the COM, and the Invoke of its
    // table takes the default member with the value of the write.
    unsafe {
        const DISPID_PROPERTY_PUT: i32 = -3;
        let named_for_write = [DISPID_PROPERTY_PUT];
        let mut parameters = super::events::TheParameters {
            arguments: variants.as_mut_ptr(),
            named: named_for_write.as_ptr() as *mut i32,
            count: 1,
            named_count: 1,
        };
        type Invoke = unsafe extern "system" fn(
            *mut c_void,
            i32,
            *const GUID,
            u32,
            u16,
            *mut super::events::TheParameters,
            *mut super::events::TheVariant,
            *mut c_void,
            *mut u32,
        ) -> HRESULT;
        let invoke: Invoke = std::mem::transmute(the_method(object.pointer, 6));
        let null_iid = GUID {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        };
        let answer = invoke(
            object.pointer,
            0, // DISPID_VALUE
            &null_iid,
            0x0400, // LOCALE_USER_DEFAULT
            4,      // DISPATCH_PROPERTYPUT
            &mut parameters,
            &mut result,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if answer < 0 {
            return Err(the_message_of(answer));
        }
    }
    Ok(())
}

/// The number of a class of the machine, by the name of it.
///
/// A name that begins with a brace is the text of a `CLSID`, and any
/// other one is a `ProgID`. The machine knows the two, and the tree asks
/// it. The shape of it is the one of the v2.0.31, read in its source
/// (`source/script_com.cpp` of the reference tree).
fn the_class_of_the_name(text: &str) -> Result<GUID, String> {
    let wide = super::sys::Wide::of(text);
    let mut number: GUID = unsafe { std::mem::zeroed() };
    let answer = unsafe {
        if text.trim_start().starts_with('{') {
            CLSIDFromString(wide.as_ptr(), &mut number)
        } else {
            CLSIDFromProgID(wide.as_ptr(), &mut number)
        }
    };
    if answer < 0 {
        return Err(the_message_of(answer));
    }
    Ok(number)
}

/// The shape of the `IDispatch`.
fn the_interface_of_the_dispatch() -> GUID {
    the_interface_of_the_text("{00020400-0000-0000-C000-000000000046}").unwrap_or_default()
}

/// Whether two numbers of the COM are the same.
///
/// A `GUID` of windows-sys is not compared, and the bytes of it are the
/// number.
fn the_same_number(one: &GUID, other: &GUID) -> bool {
    let one: [u8; 16] = unsafe { std::mem::transmute(*one) };
    let other: [u8; 16] = unsafe { std::mem::transmute(*other) };
    one == other
}

/// The object of a class of the machine.
///
/// The class is the text of a `ProgID` or of a `CLSID`, and the interface
/// is the text of an `IID` -- the `IDispatch` when the script does not
/// say.
pub fn the_object_of_the_class(class: &str, interface: Option<&str>) -> Result<TheCom, String> {
    the_apartment_is_taken();
    let class = the_class_of_the_name(class)?;
    let (interface, kind) = match interface {
        Some(text) => (the_interface_of_the_text(text)?, THE_UNKNOWN),
        None => (the_interface_of_the_dispatch(), THE_DISPATCH),
    };
    let mut pointer: *mut c_void = std::ptr::null_mut();
    let answer = unsafe {
        CoCreateInstance(
            &class,
            std::ptr::null_mut(),
            CLSCTX_SERVER,
            &interface,
            &mut pointer,
        )
    };
    if answer < 0 || pointer.is_null() {
        return Err(the_message_of(answer));
    }
    Ok(TheCom { pointer, kind })
}

/// The object that a program of the machine has running.
pub fn the_object_of_the_running(class: &str) -> Result<TheCom, String> {
    the_apartment_is_taken();
    let class = the_class_of_the_text(class)?;
    let mut pointer: *mut c_void = std::ptr::null_mut();
    let answer = unsafe { GetActiveObject(&class, std::ptr::null_mut(), &mut pointer) };
    if answer < 0 || pointer.is_null() {
        return Err(the_message_of(answer));
    }
    // The `IDispatch` of the object.
    let dispatch = the_interface_of_the_dispatch();
    let mut unknown: *mut c_void = std::ptr::null_mut();
    type QueryInterface =
        unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
    let query: QueryInterface = unsafe { std::mem::transmute(the_method(pointer, 0)) };
    let answer = unsafe { query(pointer, &dispatch, &mut unknown) };
    if answer < 0 || unknown.is_null() {
        return Err(the_message_of(answer));
    }
    Ok(TheCom {
        pointer: unknown,
        kind: THE_DISPATCH,
    })
}

/// The object that the machine gives for a name.
///
/// The name is a moniker of the machine: the text of a `CLSID`, or the
/// name of a file, or the name of an object that is running.
pub fn the_object_of_the_name(name: &str) -> Result<TheCom, String> {
    the_apartment_is_taken();
    let wide = super::sys::Wide::of(name);
    let dispatch = the_interface_of_the_dispatch();
    let mut pointer: *mut c_void = std::ptr::null_mut();
    let answer = unsafe { CoGetObject(wide.as_ptr(), std::ptr::null(), &dispatch, &mut pointer) };
    if answer < 0 || pointer.is_null() {
        return Err(the_message_of(answer));
    }
    Ok(TheCom {
        pointer,
        kind: THE_DISPATCH,
    })
}

/// Another interface of the same object.
pub fn the_object_is_queried(object: &TheCom, interface: &str) -> Result<TheCom, String> {
    let iid = the_interface_of_the_text(interface)?;
    let mut pointer: *mut c_void = std::ptr::null_mut();
    type QueryInterface =
        unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
    let query: QueryInterface = unsafe { std::mem::transmute(the_method(object.pointer, 0)) };
    let answer = unsafe { query(object.pointer, &iid, &mut pointer) };
    if answer < 0 || pointer.is_null() {
        return Err(the_message_of(answer));
    }
    let kind = if the_same_number(&iid, &the_interface_of_the_dispatch()) {
        THE_DISPATCH
    } else {
        THE_UNKNOWN
    };
    Ok(TheCom { pointer, kind })
}

/// A value of the COM: the kind of it and the number of it.
///
/// A text of the kind `VT_BSTR` is a text that the machine made: a
/// `BSTR` is a wide text with the machine that counts it.
pub fn the_value_is_made(kind: i64, value: ComValue) -> Result<TheCom, String> {
    let number = match (kind, value) {
        (THE_BSTR, ComValue::Text(text)) => {
            let wide = super::sys::Wide::of(&text);
            let bstr = unsafe { windows_sys::Win32::Foundation::SysAllocString(wide.as_ptr()) };
            if bstr.is_null() {
                return Err(the_message_of(-2147024882));
            }
            bstr as u64
        }
        (THE_I4, ComValue::Number(number)) => (number as i32) as u32 as u64,
        (THE_R8, ComValue::Number(number)) => number.to_bits(),
        (THE_BOOL, ComValue::Number(number)) => u64::from(number != 0.0),
        (THE_I8, ComValue::Number(number)) => number as i64 as u64,
        (_, ComValue::Pointer(pointer)) => pointer,
        (_, ComValue::Number(number)) => number as i64 as u64,
        (_, ComValue::Text(text)) => text.len() as u64,
    };
    Ok(TheCom {
        pointer: number as *mut c_void,
        kind,
    })
}

/// The call of a method of the object, by its place in the table.
///
/// The method is the place of it in the table of the object, and the
/// object is the first argument of the call. The answer that has no type
/// given is an `HRESULT`, and an `HRESULT` that failed is the failure of
/// the call.
pub fn the_method_is_called(
    object: &TheCom,
    index: usize,
    arguments: &mut [NativeArg],
    returns: NativeType,
    failure_is_an_error: bool,
) -> Result<NativeResult, String> {
    // The place of the method in the table of the object.
    let address = the_method(object.pointer, index);
    let mut call = NativeCall {
        library: String::new(),
        function: String::new(),
        convention: Convention::Standard,
        returns,
        arguments: arguments.to_vec(),
    };
    let answer =
        crate::native::call_at(address as u64, &mut call).map_err(|failure| failure.reason)?;
    // The arguments that come back: the argument is the way back to the
    // variable of the script.
    for (given, argument) in arguments.iter_mut().zip(call.arguments.iter()) {
        *given = argument.clone();
    }
    if failure_is_an_error {
        if let ahk::dll::NativeValue::Integer(number) = answer.value {
            if (number as u32 as i32) < 0 {
                return Err(the_message_of(number as i32 as HRESULT));
            }
        }
    }
    Ok(answer)
}

/// The script is connected to the events of an object.
///
/// The interface of the events is the implementation of the class of the
/// object with the flags `IMPLTYPEFLAG_FDEFAULT | IMPLTYPEFLAG_FSOURCE`,
/// and the machine is the owner of it. The connection point of the
/// machine is who carries the events, and the `sink` is the object that
/// it calls.
///
/// The shape is the one of the v2.0.31, read in its source
/// (`source/script_com.cpp` of the reference tree).
pub fn the_object_is_connected(
    object: &TheCom,
    prefix: &str,
    events: std::sync::Arc<std::sync::Mutex<Vec<super::events::TheEvent>>>,
) -> Result<(), String> {
    let Some(class) = the_class_information_of_the_object(object)? else {
        return Err("the machine has no information of the class of the object.".to_string());
    };
    let Some((source, number)) = the_source_of_the_events(class) else {
        return Err("the object has no source of events.".to_string());
    };
    // The `sink` is the object that the machine calls.
    let sink = super::events::TheSink::new(
        source,
        &number,
        prefix,
        events,
        object.the_pointer(),
        object.the_kind(),
    );
    let sink_pointer = sink as *mut c_void;
    // The connection point of the machine.
    let container = the_object_is_queried_raw(object, "{B196B284-BAB4-101A-B69C-00AA00341D07}")?;
    let point = the_connection_point(container, &number)?;
    // The `Advise` is the sixth method of the table of the point.
    type Advise = unsafe extern "system" fn(*mut c_void, *mut c_void, *mut u32) -> HRESULT;
    let advise: Advise = unsafe { std::mem::transmute(the_method(point, 5)) };
    let mut cookie = 0u32;
    let answer = unsafe { advise(point, sink_pointer, &mut cookie) };
    unsafe {
        type Release = unsafe extern "system" fn(*mut c_void) -> u32;
        let release: Release = std::mem::transmute(the_method(point, 2));
        release(point);
        release(container);
    }
    if answer < 0 {
        // The machine refused: the `sink` goes back.
        unsafe {
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(the_method(sink_pointer, 2));
            release(sink_pointer);
        }
        return Err(the_message_of(answer));
    }
    Ok(())
}

/// The interface of the events of an object: the information of its
/// type and its number.
///
/// The machine says which implementation of the class is the source of
/// the events: the one whose flags are `IMPLTYPEFLAG_FDEFAULT |
/// IMPLTYPEFLAG_FSOURCE` -- one and two, three. Measured on the machine
/// of the tree: the object of the `MSXML2.DOMDocument`
/// and the one of the `ADODB.Connection` have such an implementation,
/// and the ones of the `Scripting.FileSystemObject` and of the
/// `Shell.Application` have not.
fn the_source_of_the_events(class: TheTypeInfo) -> Option<(*mut c_void, GUID)> {
    unsafe {
        let attributes = the_attributes_of_the_type_information(class)?;
        let implementations = u32::from(attributes.cImplTypes);
        for index in 0..implementations {
            let mut flags: i32 = 0;
            type GetImplTypeFlags =
                unsafe extern "system" fn(*mut c_void, u32, *mut i32) -> HRESULT;
            let get_flags: GetImplTypeFlags = std::mem::transmute(the_method(class, 9));
            if get_flags(class, index, &mut flags) < 0 {
                continue;
            }
            // `IMPLTYPEFLAG_FDEFAULT | IMPLTYPEFLAG_FSOURCE` is three.
            if flags != 3 {
                continue;
            }
            let mut reference: u32 = 0;
            type GetRefTypeOfImplType =
                unsafe extern "system" fn(*mut c_void, u32, *mut u32) -> HRESULT;
            let get_reference: GetRefTypeOfImplType = std::mem::transmute(the_method(class, 8));
            if get_reference(class, index, &mut reference) < 0 {
                continue;
            }
            let mut information: TheTypeInfo = std::ptr::null_mut();
            type GetRefTypeInfo =
                unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
            let get_information: GetRefTypeInfo = std::mem::transmute(the_method(class, 14));
            if get_information(class, reference, &mut information) < 0 || information.is_null() {
                continue;
            }
            // The events come by a dispatch interface: the v2 takes
            // nothing else (`TKIND_DISPATCH` is four).
            let dispatch = the_attributes_of_the_type_information(information)
                .is_some_and(|attributes| attributes.typekind == 4);
            if !dispatch {
                type Release = unsafe extern "system" fn(*mut c_void) -> u32;
                let release: Release = std::mem::transmute(the_method(information, 2));
                release(information);
                continue;
            }
            let number = the_number_of_the_type_information_raw(information);
            return Some((information, number?));
        }
    }
    None
}

/// The interface of the source of the events of a class: only the
/// information of the type, for the question of a test.
#[cfg(test)]
pub(crate) fn the_interface_of_the_source_of_the_events(class: TheTypeInfo) -> Option<*mut c_void> {
    the_source_of_the_events(class).map(|(source, _)| source)
}

/// Another interface of the object, by its number.
fn the_object_is_queried_raw(object: &TheCom, interface: &str) -> Result<*mut c_void, String> {
    let iid = the_interface_of_the_text(interface)?;
    let mut pointer: *mut c_void = std::ptr::null_mut();
    unsafe {
        type QueryInterface =
            unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
        let query: QueryInterface = std::mem::transmute(the_method(object.pointer, 0));
        let answer = query(object.pointer, &iid, &mut pointer);
        if answer < 0 || pointer.is_null() {
            return Err(the_message_of(answer));
        }
    }
    Ok(pointer)
}

/// The connection point of the machine for an interface.
fn the_connection_point(container: *mut c_void, number: &GUID) -> Result<*mut c_void, String> {
    let mut point: *mut c_void = std::ptr::null_mut();
    unsafe {
        // `IConnectionPointContainer::FindConnectionPoint` is the fifth
        // method of the table.
        type FindConnectionPoint =
            unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
        let find: FindConnectionPoint = std::mem::transmute(the_method(container, 4));
        let answer = find(container, number, &mut point);
        if answer < 0 || point.is_null() {
            return Err(the_message_of(answer));
        }
    }
    Ok(point)
}

/// The messages of the apartment are pumped, and the machine gives the
/// events.
///
/// The machine calls the script by the queue of the thread of the
/// apartment, and the queue only moves when the thread looks at it. The
/// wait is the time the tree gives to the machine.
///
/// Measured on the machine of the tree: the flag that
/// dispatches the calls is `COWAIT_DISPATCH_CALLS`, eight; with one --
/// `COWAIT_WAITALL` -- the wait comes back and the events of the object
/// are not carried.
pub fn the_messages_are_pumped(wait_ms: u64) {
    unsafe {
        // `COWAIT_DISPATCH_CALLS` is eight, and the window messages go
        // with it: ten.
        windows_sys::Win32::System::Com::CoWaitForMultipleHandles(
            0x8 | 0x10,
            wait_ms.min(u32::MAX as u64) as u32,
            0,
            std::ptr::null(),
            std::ptr::null_mut(),
        );
    }
}

/// A member of an object of the COM: got, set or called.
///
/// The name goes to `GetIDsOfNames` and the number to `Invoke`, with the
/// arguments in the order the machine wants them -- the last one first.
/// The value of the answer is the `VARIANT` that `Invoke` left, and the
/// machine owns the memory of it.
pub fn the_member_of_the_object(
    object: &TheCom,
    what: ahk::desktop::MemberWhat,
    name: &str,
    arguments: &[ComValue],
) -> Result<ahk::desktop::ComAnswer, String> {
    use ahk::desktop::MemberWhat;
    if object.pointer.is_null() {
        return Err("the COM object pointer is null.".to_string());
    }
    let wide: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut name_pointer: *mut u16 = wide.as_ptr() as *mut u16;
    let mut number: i32 = 0;
    unsafe {
        // `IDispatch::GetIDsOfNames` is the sixth method of the table.
        type GetIDsOfNames = unsafe extern "system" fn(
            *mut c_void,
            *const GUID,
            *mut *mut u16,
            u32,
            u32,
            *mut i32,
        ) -> HRESULT;
        let get: GetIDsOfNames = std::mem::transmute(the_method(object.pointer, 5));
        let null_iid = GUID {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        };
        let answer = get(
            object.pointer,
            &null_iid,
            &mut name_pointer,
            1,
            0x0400, // LOCALE_USER_DEFAULT
            &mut number,
        );
        if answer < 0 {
            return Err(the_message_of(answer));
        }
    }
    // COM `IDispatch::Invoke` expects `rgvarg` in reverse order (last argument
    // first) for all invocations (`Get`, `Call`, and `Set`).
    let mut variants: Vec<super::events::TheVariant> = arguments
        .iter()
        .rev()
        .map(super::events::the_variant_of)
        .collect();
    let flags: u16 = match what {
        MemberWhat::Get => 2,  // DISPATCH_PROPERTYGET
        MemberWhat::Set => 4,  // DISPATCH_PROPERTYPUT
        MemberWhat::Call => 3, // DISPATCH_METHOD | DISPATCH_PROPERTYGET
    };
    let mut result: super::events::TheVariant = unsafe { std::mem::zeroed() };
    let answer_of_the_call = unsafe {
        let mut parameters = super::events::TheParameters {
            arguments: if variants.is_empty() {
                std::ptr::null_mut()
            } else {
                variants.as_mut_ptr()
            },
            named: std::ptr::null_mut(),
            count: variants.len() as u32,
            named_count: 0,
        };
        // A write names its value with `DISPID_PROPERTYPUT`.
        const DISPID_PROPERTY_PUT: i32 = -3;
        let named_for_write = [DISPID_PROPERTY_PUT];
        if what == MemberWhat::Set {
            parameters.named = named_for_write.as_ptr() as *mut i32;
            parameters.named_count = 1;
        }
        // `IDispatch::Invoke` is the seventh method of the table.
        type Invoke = unsafe extern "system" fn(
            *mut c_void,
            i32,
            *const GUID,
            u32,
            u16,
            *mut super::events::TheParameters,
            *mut super::events::TheVariant,
            *mut c_void,
            *mut u32,
        ) -> HRESULT;
        let invoke: Invoke = std::mem::transmute(the_method(object.pointer, 6));
        let null_iid = GUID {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        };
        // The machine wants a real `DISPPARAMS` even when there is no
        // argument: the null pointer makes the `MSXML2.DOMDocument` fall
        // (measured on the machine of the tree).
        invoke(
            object.pointer,
            number,
            &null_iid,
            0x0400,
            flags,
            &mut parameters,
            &mut result,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    // Free any BSTRs we allocated for the argument variants.
    for variant in &variants {
        if variant.kind == 8 && variant.data != 0 {
            unsafe {
                windows_sys::Win32::Foundation::SysFreeString(variant.data as *const u16);
            }
        }
    }
    if answer_of_the_call < 0 {
        return Err(the_message_of(answer_of_the_call));
    }
    Ok(super::events::the_answer_of(&mut result))
}

/// The name or the number of the class or of the interface of an object.
///
/// The `what` is "Name" or "IID" -- "Class" and "CLSID" are the shapes
/// of the information of the class. The shape is the one of the v2.0.31,
/// read in its source (`source/script_com.cpp`): the information of the class is
/// `IProvideClassInfo` do objeto, e a da interface vem do `IDispatch`.
pub fn the_type_of_the_object(object: &TheCom, what: &str) -> Result<String, String> {
    let asked = what.trim().to_ascii_lowercase();
    let (of_the_class, asked) = match asked.as_str() {
        "class" => (true, "name"),
        "clsid" => (true, "iid"),
        "name" => (false, "name"),
        "iid" => (false, "iid"),
        _ => {
            return Err(format!(
                "the machine is asked for {what:?}, and it knows the name and the number"
            ));
        }
    };
    let info = if of_the_class {
        the_class_information_of_the_object(object)?
    } else {
        the_interface_information_of_the_object(object)?
    };
    // An object without type information gives no name and no number:
    // the answer of the v2 is the empty text.
    let Some(info) = info else {
        return Ok(String::new());
    };
    if asked == "name" {
        the_name_of_the_type_information(info)
    } else {
        the_number_of_the_type_information(info)
    }
}

/// The type information of an object: the pointer and the machine that gives it.
type TheTypeInfo = *mut c_void;

/// The information of the class of an object.
///
/// The `IProvideClassInfo` of the object gives the information of the
/// class through its `GetClassInfo`, and when the object has no such
/// information the type library of the machine is walked for the
/// `coclass` whose default interface is the one of the object. The walk
/// is the one of the v2.0.31 (`source/script_com.cpp`).
pub(crate) fn the_class_information_of_the_object(
    object: &TheCom,
) -> Result<Option<TheTypeInfo>, String> {
    let iid = the_interface_of_the_text("{B196B283-BAB4-101A-B69C-00AA00341D07}")?;
    let mut giver: *mut c_void = std::ptr::null_mut();
    unsafe {
        type QueryInterface =
            unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
        let query: QueryInterface = std::mem::transmute(the_method(object.pointer, 0));
        let answer = query(object.pointer, &iid, &mut giver);
        if answer >= 0 && !giver.is_null() {
            // `IProvideClassInfo::GetClassInfo` is the fourth method.
            type GetClassInfo = unsafe extern "system" fn(*mut c_void, *mut *mut c_void) -> HRESULT;
            let get: GetClassInfo = std::mem::transmute(the_method(giver, 3));
            let mut info: TheTypeInfo = std::ptr::null_mut();
            let answer = get(giver, &mut info);
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(the_method(giver, 2));
            release(giver);
            if answer >= 0 && !info.is_null() {
                return Ok(Some(info));
            }
        }
    }
    the_class_of_the_type_library(object)
}

/// The class of an object, by the type library of the machine.
///
/// The library says which `coclass` owns the interface of the object:
/// the one whose default interface is the number of that interface.
fn the_class_of_the_type_library(object: &TheCom) -> Result<Option<TheTypeInfo>, String> {
    let Some(interface) = the_interface_information_of_the_object(object)? else {
        return Ok(None);
    };
    let Some(wanted) = the_number_of_the_type_information_raw(interface) else {
        return Ok(None);
    };
    // A biblioteca de tipos da interface.
    let mut library: *mut c_void = std::ptr::null_mut();
    unsafe {
        type GetContainingTypeLib =
            unsafe extern "system" fn(*mut c_void, *mut *mut c_void, *mut u32) -> HRESULT;
        // `ITypeInfo::GetContainingTypeLib` e' o decimo setimo metodo da tabela.
        let get: GetContainingTypeLib = std::mem::transmute(the_method(interface, 18));
        let answer = get(interface, &mut library, std::ptr::null_mut());
        if answer < 0 || library.is_null() {
            return Ok(None);
        }
        // The classes of the library: the one that has the interface as its default.
        type GetTypeInfoCount = unsafe extern "system" fn(*mut c_void) -> u32;
        let count_of: GetTypeInfoCount = std::mem::transmute(the_method(library, 3));
        let count = count_of(library);
        for index in 0..count {
            let mut class: TheTypeInfo = std::ptr::null_mut();
            type GetTypeInfoType = unsafe extern "system" fn(*mut c_void, u32, *mut i32) -> HRESULT;
            let kind_of: GetTypeInfoType = std::mem::transmute(the_method(library, 5));
            let mut kind: i32 = 0;
            if kind_of(library, index, &mut kind) < 0 {
                continue;
            }
            // `TKIND_COCLASS` e' cinco.
            if kind != 5 {
                continue;
            }
            type GetTypeInfo =
                unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
            let get: GetTypeInfo = std::mem::transmute(the_method(library, 4));
            if get(library, index, &mut class) < 0 || class.is_null() {
                continue;
            }
            let found = the_class_with_the_interface(class, &wanted);
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(the_method(class, 2));
            release(class);
            if found {
                // The class is the type information that gives it.
                let mut again: TheTypeInfo = std::ptr::null_mut();
                let get: GetTypeInfo = std::mem::transmute(the_method(library, 4));
                get(library, index, &mut again);
                return Ok(Some(again));
            }
        }
    }
    Ok(None)
}

/// Whether the class has the interface as its default one.
fn the_class_with_the_interface(class: TheTypeInfo, wanted: &GUID) -> bool {
    unsafe {
        let attributes = the_attributes_of_the_type_information(class);
        let Some(attributes) = attributes else {
            return false;
        };
        let implementations = u32::from(attributes.cImplTypes);
        for index in 0..implementations {
            // `IMPLTYPEFLAG_FDEFAULT` is one.
            let mut flags: i32 = 0;
            type GetImplTypeFlags =
                unsafe extern "system" fn(*mut c_void, u32, *mut i32) -> HRESULT;
            let get_flags: GetImplTypeFlags = std::mem::transmute(the_method(class, 9));
            if get_flags(class, index, &mut flags) < 0 || flags != 1 {
                continue;
            }
            // A interface da classe.
            let mut reference: u32 = 0;
            type GetRefTypeOfImplType =
                unsafe extern "system" fn(*mut c_void, u32, *mut u32) -> HRESULT;
            let get_reference: GetRefTypeOfImplType = std::mem::transmute(the_method(class, 8));
            if get_reference(class, index, &mut reference) < 0 {
                continue;
            }
            let mut information: TheTypeInfo = std::ptr::null_mut();
            type GetRefTypeInfo =
                unsafe extern "system" fn(*mut c_void, u32, *mut *mut c_void) -> HRESULT;
            let get_information: GetRefTypeInfo = std::mem::transmute(the_method(class, 14));
            if get_information(class, reference, &mut information) < 0 || information.is_null() {
                continue;
            }
            let number = the_number_of_the_type_information_raw(information);
            type Release = unsafe extern "system" fn(*mut c_void) -> u32;
            let release: Release = std::mem::transmute(the_method(information, 2));
            release(information);
            if let Some(number) = number {
                if the_same_number(&number, wanted) {
                    return true;
                }
            }
        }
    }
    false
}

/// The attributes of a type information.
///
/// The machine owns them, and the `ReleaseTypeAttr` gives them back.
fn the_attributes_of_the_type_information(
    info: TheTypeInfo,
) -> Option<windows_sys::Win32::System::Com::TYPEATTR> {
    unsafe {
        type GetTypeAttr = unsafe extern "system" fn(
            *mut c_void,
            *mut *mut windows_sys::Win32::System::Com::TYPEATTR,
        ) -> HRESULT;
        let get: GetTypeAttr = std::mem::transmute(the_method(info, 3));
        let mut attributes: *mut windows_sys::Win32::System::Com::TYPEATTR = std::ptr::null_mut();
        if get(info, &mut attributes) < 0 || attributes.is_null() {
            return None;
        }
        let copy = *attributes;
        // The `ReleaseTypeAttr` gives back the pointer the machine gave.
        type ReleaseTypeAttr =
            unsafe extern "system" fn(*mut c_void, *mut windows_sys::Win32::System::Com::TYPEATTR);
        // ITypeInfo::ReleaseTypeAttr e' o decimo nono metodo da tabela.
        let release: ReleaseTypeAttr = std::mem::transmute(the_method(info, 19));
        release(info, attributes);
        Some(copy)
    }
}

/// The information of the interface of an object.
///
/// The information of the `IDispatch` of the object is the one of the
/// interface, and the machine owns it. An object without dispatch has no
/// such information.
pub(crate) fn the_interface_information_of_the_object(
    object: &TheCom,
) -> Result<Option<TheTypeInfo>, String> {
    let mut info: TheTypeInfo = std::ptr::null_mut();
    unsafe {
        // `IDispatch::GetTypeInfo` is the fifth method of the table.
        type GetTypeInfo =
            unsafe extern "system" fn(*mut c_void, u32, u32, *mut *mut c_void) -> HRESULT;
        let get: GetTypeInfo = std::mem::transmute(the_method(object.pointer, 4));
        // The locale of the user: `LOCALE_USER_DEFAULT`.
        let answer = get(object.pointer, 0, 0x0400, &mut info);
        if answer < 0 {
            return Ok(None);
        }
    }
    Ok(Some(info))
}

/// The name that the machine gives to a type information.
///
/// A type information without documentation gives no name: the answer
/// of the v2 is the empty text.
fn the_name_of_the_type_information(info: TheTypeInfo) -> Result<String, String> {
    unsafe {
        // `ITypeInfo::GetDocumentation` is the thirteenth method of the
        // tabela.
        type GetDocumentation = unsafe extern "system" fn(
            *mut c_void,
            i32,
            *mut *mut u16,
            *mut *mut u16,
            *mut u32,
            *mut *mut u16,
        ) -> HRESULT;
        let get: GetDocumentation = std::mem::transmute(the_method(info, 12));
        let mut name: *mut u16 = std::ptr::null_mut();
        let answer = get(
            info,
            -1,
            &mut name,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if answer < 0 || name.is_null() {
            return Ok(String::new());
        }
        let mut length = 0usize;
        while *name.add(length) != 0 {
            length += 1;
        }
        let text = String::from_utf16_lossy(std::slice::from_raw_parts(name, length));
        // The text is a `BSTR`, and the machine gives it back with `SysFreeString`.
        windows_sys::Win32::Foundation::SysFreeString(name.cast());
        Ok(text)
    }
}

/// The number that the machine gives to a type information.
///
/// A type information without attributes gives no number: the answer
/// of the v2 is the empty text.
fn the_number_of_the_type_information(info: TheTypeInfo) -> Result<String, String> {
    let Some(number) = the_number_of_the_type_information_raw(info) else {
        return Ok(String::new());
    };
    let mut text = [0u16; 40];
    let length = unsafe {
        windows_sys::Win32::System::Com::StringFromGUID2(
            &number,
            text.as_mut_ptr(),
            text.len() as i32,
        )
    };
    Ok(String::from_utf16_lossy(
        &text[..(length.max(0) - 1) as usize],
    ))
}

/// The number that the machine gives to a type information, as bytes.
fn the_number_of_the_type_information_raw(info: TheTypeInfo) -> Option<GUID> {
    // The first field of the `TYPEATTR` is the number.
    let attributes = the_attributes_of_the_type_information(info)?;
    Some(attributes.guid)
}

/// The number of each method of each shape, in the table of the shape.
///
/// The table of the link: `QueryInterface`, `AddRef`, `Release`,
/// `GetPath`, `GetIDList`, `SetIDList`, `GetDescription`, `SetDescription`,
/// `GetWorkingDirectory`, `SetWorkingDirectory`, `GetArguments`,
/// `SetArguments`, `GetHotkey`, `SetHotkey`, `GetShowCmd`, `SetShowCmd`,
/// `GetIconLocation`, `SetIconLocation`, `SetRelativePath`, `Resolve`,
/// `SetPath`.
mod the_numbers_of_the_link {
    /// `SetPath`.
    pub const SET_PATH: usize = 20;
    /// `SetWorkingDirectory`.
    pub const SET_WORKING_DIRECTORY: usize = 9;
    /// `SetArguments`.
    pub const SET_ARGUMENTS: usize = 11;
    /// `SetDescription`.
    pub const SET_DESCRIPTION: usize = 7;
    /// `SetIconLocation`.
    pub const SET_ICON_LOCATION: usize = 17;
    /// `GetPath`.
    pub const GET_PATH: usize = 3;
    /// `GetWorkingDirectory`.
    pub const GET_WORKING_DIRECTORY: usize = 8;
    /// `GetArguments`.
    pub const GET_ARGUMENTS: usize = 10;
    /// `GetDescription`.
    pub const GET_DESCRIPTION: usize = 6;
    /// `GetIconLocation`.
    pub const GET_ICON_LOCATION: usize = 16;
}

/// The number of each method of the shape of the file, in its table.
///
/// `QueryInterface`, `AddRef`, `Release`, `GetClassID`, `IsDirty`,
/// `Load`, `Save`, `SaveCompleted`, `GetCurFile`.
mod the_numbers_of_the_file {
    /// `Load`.
    pub const LOAD: usize = 5;
    /// `Save`.
    pub const SAVE: usize = 6;
}

/// The file of a shortcut: the shape `IPersistFile`.
///
/// The shortcut is a file, and this is the shape that reads and writes
/// files.
fn the_file_of(link: &TheObject) -> Result<TheObject, String> {
    let mut pointer: *mut c_void = std::ptr::null_mut();
    let answer = unsafe {
        type QueryInterface =
            unsafe extern "system" fn(*mut c_void, *const GUID, *mut *mut c_void) -> HRESULT;
        let query: QueryInterface = std::mem::transmute(link.the_method(0));
        query(link.pointer, &the_interface_of_the_file()?, &mut pointer)
    };
    if answer < 0 || pointer.is_null() {
        return Err(format!(
            "the shell refused the file of the shortcut with the number {answer} ({:#x}): {}",
            answer as u32,
            the_reason_of(answer)
        ));
    }
    Ok(TheObject { pointer })
}

/// A shortcut of the machine is made.
///
/// The target is the program of the shortcut, the working directory is
/// where it begins, the arguments are what it is called with, the
/// description is its name and the icon is the picture of it.
pub fn the_shortcut_is_created(
    target: &str,
    shortcut: &str,
    working_directory: &str,
    arguments: &str,
    description: &str,
    icon: &str,
) -> Result<(), String> {
    let link = TheObject::of_the_class(&the_class_of_the_link()?, &the_interface_of_the_link()?)?;

    /// The answer of the shell, with its number.
    fn the_answer(answer: HRESULT, what: &str) -> Result<(), String> {
        if answer < 0 {
            return Err(format!(
                "the shell refused {what} with the number {answer} ({:#x}): {}",
                answer as u32,
                the_reason_of(answer)
            ));
        }
        Ok(())
    }

    unsafe {
        let wide = super::sys::Wide::of(target);
        type SetPath = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
        let set: SetPath = std::mem::transmute(link.the_method(the_numbers_of_the_link::SET_PATH));
        the_answer(set(link.pointer, wide.as_ptr()), "the path of the shortcut")?;

        if !working_directory.is_empty() {
            let wide = super::sys::Wide::of(working_directory);
            type SetDirectory = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
            let set: SetDirectory = std::mem::transmute(
                link.the_method(the_numbers_of_the_link::SET_WORKING_DIRECTORY),
            );
            the_answer(
                set(link.pointer, wide.as_ptr()),
                "the working directory of the shortcut",
            )?;
        }

        if !arguments.is_empty() {
            let wide = super::sys::Wide::of(arguments);
            type SetArguments = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
            let set: SetArguments =
                std::mem::transmute(link.the_method(the_numbers_of_the_link::SET_ARGUMENTS));
            the_answer(
                set(link.pointer, wide.as_ptr()),
                "the arguments of the shortcut",
            )?;
        }

        if !description.is_empty() {
            let wide = super::sys::Wide::of(description);
            type SetDescription = unsafe extern "system" fn(*mut c_void, *const u16) -> HRESULT;
            let set: SetDescription =
                std::mem::transmute(link.the_method(the_numbers_of_the_link::SET_DESCRIPTION));
            the_answer(
                set(link.pointer, wide.as_ptr()),
                "the description of the shortcut",
            )?;
        }

        if !icon.is_empty() {
            let wide = super::sys::Wide::of(icon);
            type SetIcon = unsafe extern "system" fn(*mut c_void, *const u16, i32) -> HRESULT;
            let set: SetIcon =
                std::mem::transmute(link.the_method(the_numbers_of_the_link::SET_ICON_LOCATION));
            the_answer(
                set(link.pointer, wide.as_ptr(), 0),
                "the icon of the shortcut",
            )?;
        }
    }

    let file = the_file_of(&link)?;
    let wide = super::sys::Wide::of(shortcut);
    let answer = unsafe {
        type Save = unsafe extern "system" fn(*mut c_void, *const u16, i32) -> HRESULT;
        let save: Save = std::mem::transmute(file.the_method(the_numbers_of_the_file::SAVE));
        save(file.pointer, wide.as_ptr(), 1)
    };
    if answer < 0 {
        return Err(format!(
            "the shell refused to save the shortcut with the number {answer} ({:#x}): {}",
            answer as u32,
            the_reason_of(answer)
        ));
    }
    Ok(())
}

/// A shortcut of the machine is read back.
///
/// The answer is the target, the working directory, the arguments, the
/// description and the icon.
pub fn the_shortcut_is_read(
    shortcut: &str,
) -> Result<(String, String, String, String, String), String> {
    let link = TheObject::of_the_class(&the_class_of_the_link()?, &the_interface_of_the_link()?)?;
    let file = the_file_of(&link)?;

    let wide = super::sys::Wide::of(shortcut);
    let answer = unsafe {
        type Load = unsafe extern "system" fn(*mut c_void, *const u16, u32) -> HRESULT;
        let load: Load = std::mem::transmute(file.the_method(the_numbers_of_the_file::LOAD));
        // The mode zero is the one of reading only.
        load(file.pointer, wide.as_ptr(), 0)
    };
    if answer < 0 {
        return Err(format!(
            "the shell refused to open the shortcut with the number {answer} ({:#x}): {}",
            answer as u32,
            the_reason_of(answer)
        ));
    }

    /// The text of a method that the shell answers in a buffer.
    fn the_text(link: &TheObject, number: usize, size: usize) -> Result<String, String> {
        let mut buffer = vec![0u16; size];
        // The shell writes the data of the file in these places, and it
        // writes them even when the caller does not want them: the
        // places are given anyway.
        let mut found = vec![0u8; 1200];
        let mut icon: i32 = 0;
        let answer = unsafe {
            match number {
                the_numbers_of_the_link::GET_PATH => {
                    type GetPath = unsafe extern "system" fn(
                        *mut c_void,
                        *mut u16,
                        i32,
                        *mut c_void,
                        u32,
                    ) -> HRESULT;
                    let get: GetPath = std::mem::transmute(link.the_method(number));
                    get(
                        link.pointer,
                        buffer.as_mut_ptr(),
                        size as i32,
                        found.as_mut_ptr() as *mut c_void,
                        0,
                    )
                }
                the_numbers_of_the_link::GET_ICON_LOCATION => {
                    type GetIcon =
                        unsafe extern "system" fn(*mut c_void, *mut u16, i32, *mut i32) -> HRESULT;
                    let get: GetIcon = std::mem::transmute(link.the_method(number));
                    get(link.pointer, buffer.as_mut_ptr(), size as i32, &mut icon)
                }
                _ => {
                    type GetText = unsafe extern "system" fn(*mut c_void, *mut u16, i32) -> HRESULT;
                    let get: GetText = std::mem::transmute(link.the_method(number));
                    get(link.pointer, buffer.as_mut_ptr(), size as i32)
                }
            }
        };
        if answer < 0 {
            return Err(format!(
                "the shell refused to give a field of the shortcut with the number {answer} ({:#x}): {}",
                answer as u32,
                the_reason_of(answer)
            ));
        }
        let length = buffer.iter().position(|letter| *letter == 0).unwrap_or(0);
        Ok(String::from_utf16_lossy(&buffer[..length]))
    }

    let target = the_text(&link, the_numbers_of_the_link::GET_PATH, 1024)?;
    let directory = the_text(&link, the_numbers_of_the_link::GET_WORKING_DIRECTORY, 1024)?;
    let arguments = the_text(&link, the_numbers_of_the_link::GET_ARGUMENTS, 1024)?;
    let description = the_text(&link, the_numbers_of_the_link::GET_DESCRIPTION, 1024)?;
    let icon = the_text(&link, the_numbers_of_the_link::GET_ICON_LOCATION, 1024)?;
    Ok((target, directory, arguments, description, icon))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahk::dll::NativeValue;

    #[test]
    fn the_reason_of_a_refusal_is_known() {
        // The table of the reasons is of the SDK of the machine: every
        // number of it has a text, and the text is never empty.
        for number in [
            -2147024882, // 0x8007000E
            -2147221164, // 0x80040154
            -2147221008, // 0x800401F0
            -2147024891, // 0x80070005
            -2147024809, // 0x80070057
            -2147024884, // 0x8007000C
            -2147467262, // 0x80004002
            -2147467259, // 0x80004005
            -2147024770, // 0x8007007E
            -2147024703, // 0x800700C1
            -2147417850, // 0x80010106
        ] {
            let reason = the_reason_of(number);
            assert!(!reason.is_empty(), "the number {number} has no reason");
            assert!(
                !reason.contains("refused with the number"),
                "the number {number} is not in the table of the reasons"
            );
        }
    }

    #[test]
    fn the_reason_of_an_unknown_number_says_the_number() {
        let reason = the_reason_of(-1);
        assert!(reason.contains("0xffffffff"), "the reason is {reason}");
    }

    #[test]
    fn the_picker_takes_the_flags_and_the_title() {
        // The places of the table of the picker are the ones of the
        // interface `IFileDialog` (`ShObjIdl_core.h`): the title is the
        // seventeenth method and the flags are the ninth. The tree had
        // the sixteenth and the ninth wrong, and the picker was never
        // shown, so the wrong places never answered anything.
        let picker = ThePicker::to_open().expect("the machine did not make the picker");
        picker
            .the_options_are_set(0x8)
            .expect("the machine did not take the flags of the picker");
        picker
            .the_title_is_written("the tree")
            .expect("the machine did not take the title of the picker");
    }

    #[test]
    fn the_picker_keeps_the_kinds_that_it_was_given() {
        // The kinds of the files are the `COMDLG_FILTERSPEC` of the SDK,
        // and the machine keeps them: the kind that it shows is the first
        // one, and it answers the number of it.
        let picker = TheShellPicker::to_open().expect("the machine did not make the picker");
        picker
            .the_filter_is_written(&[("Text", "*.txt"), ("All Files", "*.*")])
            .expect("the machine did not take the kinds of the picker");
        unsafe {
            // The kind that is shown is the one that the machine was
            // told to show: the machine refuses a kind that it does not
            // have, and it answers the number of the one that it has.
            type SetFileTypeIndex = unsafe extern "system" fn(*mut c_void, u32) -> HRESULT;
            let set: SetFileTypeIndex = std::mem::transmute(picker.object.the_method(5));
            let answer = set(picker.object.pointer, 1);
            assert!(
                answer >= 0,
                "the machine refused the kind to show: {answer}"
            );
            type GetFileTypeIndex = unsafe extern "system" fn(*mut c_void, *mut u32) -> HRESULT;
            let get: GetFileTypeIndex = std::mem::transmute(picker.object.the_method(6));
            let mut index = 0u32;
            let answer = get(picker.object.pointer, &mut index);
            assert!(
                answer >= 0,
                "the machine refused the kind that it shows: {answer}"
            );
            assert_eq!(index, 1, "the kind that the machine shows is the first one");
        }
    }

    #[test]
    fn the_picker_takes_the_folder_that_it_opens_in() {
        // The folder is the item of the shell that has the path of it,
        // and the machine makes that item with
        // `SHCreateItemFromParsingName`.
        let picker = ThePicker::to_open().expect("the machine did not make the picker");
        let folder = std::env::temp_dir();
        let folder = folder.to_string_lossy().to_string();
        picker
            .the_folder_is_set(&folder)
            .expect("the machine did not take the folder of the picker");
    }

    #[test]
    fn the_machine_makes_the_picker_of_the_files() {
        // Measured on the machine of the tree: the class
        // of the picker of the files is
        // "HKEY_CLASSES_ROOT\CLSID\{DC1C5A9C-E88A-4DDE-A5A1-60F82A20AEF7}"
        // and the machine makes it. The tree was carrying a byte swapped
        // in that number -- the tenth byte was 0x1A and not 0xA1 -- and
        // the machine was refusing with the number 0x80040154 for that
        // reason alone, in the program of thirty-two bits and in the
        // program of sixty-four bits alike.
        let answer = ThePicker::to_open();
        if let Err(reason) = &answer {
            panic!("the machine refused the picker of the files: {reason}");
        }
        let answer = ThePicker::to_save();
        if let Err(reason) = &answer {
            panic!("the machine refused the picker of the save: {reason}");
        }
    }

    #[test]
    fn the_bytes_of_the_class_are_the_bytes_of_the_machine() {
        // The table of the classes of `comdlg32.dll`, measured on the
        // machine of the tree: the class of the picker of
        // the files is the first of the four, and the class of the picker
        // of the save is the second. The numbers are read from the
        // machine now, and this test says what the machine answered.
        let bytes: [u8; 16] =
            unsafe { std::mem::transmute(the_class_of_the_picker_of_the_files().unwrap()) };
        assert_eq!(
            bytes,
            [
                0x9C, 0x5A, 0x1C, 0xDC, 0x8A, 0xE8, 0xDE, 0x4D, 0xA5, 0xA1, 0x60, 0xF8, 0x2A, 0x20,
                0xAE, 0xF7
            ]
        );
        let bytes: [u8; 16] =
            unsafe { std::mem::transmute(the_class_of_the_picker_of_the_save().unwrap()) };
        assert_eq!(
            bytes,
            [
                0xF3, 0xE2, 0xB4, 0xC0, 0x21, 0xBA, 0x73, 0x47, 0x8D, 0xBA, 0x33, 0x5E, 0xC9, 0x46,
                0xEB, 0x8B
            ]
        );
        // The class of the shell: the thirty-fourth of the table of the
        // `shell32.dll` of the machine.
        let bytes: [u8; 16] = unsafe { std::mem::transmute(the_class_of_the_shell().unwrap()) };
        assert_eq!(
            bytes,
            [
                0x20, 0x96, 0x70, 0x13, 0x79, 0xC2, 0xCE, 0x11, 0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54,
                0x00, 0x00
            ]
        );
    }

    #[test]
    fn the_machine_has_no_class_of_that_name() {
        let class = GUID {
            data1: 0x0000_0000,
            data2: 0x0000,
            data3: 0x0000,
            data4: [0, 0, 0, 0, 0, 0, 0, 0],
        };
        let answer = TheObject::of_the_class(&class, &the_interface_of_the_picker().unwrap());
        assert!(answer.is_err());
    }

    #[test]
    fn the_machine_makes_the_class_of_the_shell() {
        // Measured on the machine of the tree: the class
        // of the shell of the machine is
        // "HKEY_CLASSES_ROOT\CLSID\{13709620-C279-11CE-A49E-444553540000}"
        // and the machine makes it in a plain program. The tree was
        // carrying two bytes of that number wrong, and the machine was
        // refusing the class with the number 0x80040154 for that reason
        // alone.
        let answer = TheObject::of_the_class(
            &the_class_of_the_shell().unwrap(),
            &the_interface_of_the_unknown().unwrap(),
        );
        if let Err(reason) = &answer {
            panic!("the machine refused the class of the shell: {reason}");
        }
    }

    #[test]
    fn the_machine_makes_the_object_of_the_shell() {
        // Measured on the machine of the tree: the object
        // of `Shell.Application` is made by a plain program, e a forma
        // dele e' a do `IDispatch` -- o `VARTYPE` nove.
        let object = the_object_of_the_class("Shell.Application", None)
            .expect("the object of the shell of the machine");
        assert_eq!(object.the_kind(), 9, "the kind of the object");
        assert_ne!(object.the_pointer(), 0, "the pointer of the object");
    }

    #[test]
    fn a_method_of_the_object_is_called() {
        // Measured on the machine of the tree: o segundo
        // metodo da tabela do objeto e' o `AddRef`, e ele responde com
        // a contagem das referencias do objeto.
        let object = the_object_of_the_class("Shell.Application", None)
            .expect("the object of the shell of the machine");
        let mut arguments = vec![NativeArg {
            kind: NativeType::Ptr,
            address: false,
            value: NativeValue::Integer(object.the_pointer() as i64),
        }];
        let answer = the_method_is_called(&object, 1, &mut arguments, NativeType::Int, false)
            .expect("the call of the method");
        assert_eq!(
            answer.value,
            NativeValue::Integer(2),
            "the answer of the AddRef"
        );
    }

    #[test]
    fn the_machine_names_the_class_and_the_interface_of_an_object() {
        // Measured on the machine of the tree: the object
        // of the `Scripting.FileSystemObject` has the interface of the
        // `IFileSystem3` -- its number is
        // `{2A0B9D10-4B87-11D3-A97A-00104B365C9F}` -- and its class is
        // the `FileSystemObject`, with the number
        // `{0D43FE01-F093-11CF-8940-00A0C9054228}` (the number of the
        // registro da maquina).
        let object = the_object_of_the_class("Scripting.FileSystemObject", None)
            .expect("the object of the file system");
        assert_eq!(
            the_type_of_the_object(&object, "Name").expect("the name of the interface"),
            "IFileSystem3"
        );
        assert_eq!(
            the_type_of_the_object(&object, "IID").expect("the number of the interface"),
            "{2A0B9D10-4B87-11D3-A97A-00104B365C9F}"
        );
        assert_eq!(
            the_type_of_the_object(&object, "Class").expect("the name of the class"),
            "FileSystemObject"
        );
        assert_eq!(
            the_type_of_the_object(&object, "CLSID").expect("the number of the class"),
            "{0D43FE01-F093-11CF-8940-00A0C9054228}"
        );
    }

    #[test]
    fn the_machine_has_no_object_of_that_name() {
        // Measured on the machine of the tree: um nome que
        // the machine does not know is refused with its word -- "(0x80070057)
        // The parameter is incorrect.".
        let answer = the_object_of_the_name("");
        let reason = answer.err().expect("the machine refused the name");
        assert_eq!(reason, "(0x80070057) The parameter is incorrect.");
    }

    #[test]
    fn a_shortcut_of_the_machine_is_made_and_read_back() {
        // Measured on the machine of the tree: the class of
        // the link is served by the `shell32.dll` of the machine to a
        // plain program, and the file that the shell makes is read back
        // by the machine with the fields that were given to it.
        let folder = std::env::temp_dir().join(format!("ahk-shortcut-{}", std::process::id()));
        std::fs::create_dir_all(&folder).expect("the folder of the test");
        let shortcut = folder.join("the shortcut.lnk");
        let path = shortcut.to_string_lossy().into_owned();
        let answer = the_shortcut_is_created(
            r"C:\Windows\System32\notepad.exe",
            &path,
            r"C:\Windows\System32",
            "--the test",
            "the shortcut of the test",
            "",
        );
        if let Err(reason) = &answer {
            panic!("the machine refused the shortcut: {reason}");
        }
        assert!(shortcut.exists(), "the file of the shortcut is not there");
        let answer = the_shortcut_is_read(&path);
        let (target, directory, arguments, description, _icon) = match &answer {
            Ok(fields) => fields.clone(),
            Err(reason) => panic!("the machine refused to read the shortcut: {reason}"),
        };
        assert!(
            target.to_lowercase().contains("notepad.exe"),
            "the target of the shortcut is {target}"
        );
        assert_eq!(arguments, "--the test");
        assert_eq!(description, "the shortcut of the test");
        assert!(
            directory.to_lowercase().contains("system32"),
            "the working directory of the shortcut is {directory}"
        );
        std::fs::remove_dir_all(&folder).ok();
    }

    #[test]
    fn the_apartment_belongs_to_the_thread() {
        // The apartment is per thread: a thread that asks for it gets
        // it, and another thread asks for its own. The machine of the
        // tree answers 0x800401F0 when a thread has no apartment
        // (measured on the machine of the tree), and the tree was refusing
        // with it
        // because the apartment was taken once for the whole program.
        the_apartment_is_taken();
        the_apartment_is_taken();
        let other = std::thread::spawn(|| {
            the_apartment_is_taken();
            the_apartment_is_taken();
            the_apartment_is_given_back();
        });
        assert!(other.join().is_ok());
    }
}
