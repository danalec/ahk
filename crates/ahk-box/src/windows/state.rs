//! The state that the capabilities of the desktop share.
//!
//! The desktop of a Windows machine is one thing that answers for the
//! machine: the windows of it are looked at through the state, and the
//! windows that the script made live in it.

use std::collections::BTreeMap;

use super::files::TheFiles;
use crate::pictures::Picture;

/// The state of the desktop of a Windows machine.
#[derive(Debug)]
pub struct TheState {
    /// The windows that the script made, and the number that the machine
    /// gave to each of them.
    ///
    /// The number is the handle of the window, and it is what the script
    /// holds.
    pub windows: BTreeMap<u64, u64>,
    /// The pictures that the script loaded, and the number that it
    /// holds for each of them.
    ///
    /// The number of a picture is the one of the runtime, and it is
    /// what the script writes in the file of a picture.
    pub pictures: Vec<Picture>,
    /// The files of the machine.
    ///
    /// The files that the script opened live in them, and the work of
    /// them is the work of the crate.
    pub files: TheFiles,
    /// The libraries that the script called.
    ///
    /// A library is loaded once and kept: a script that calls the same
    /// library twice does not load it twice.
    pub libraries: crate::native::Libraries,
    /// The window of the machine that owns the tray of it.
    ///
    /// The window is made when the tray is used for the first time, and
    /// it is a window that only receives messages: the user does not see
    /// it.
    pub tray_window: Option<windows_sys::Win32::Foundation::HWND>,
    /// The icon of the script in the tray of it.
    ///
    /// The icon is the file that the script gave: the `TraySetIcon` of the
    /// v2 and the `Menu, Tray, Icon` of the v1 are the same call.
    pub tray_icon: Option<String>,
    /// The window of the script itself.
    ///
    /// The window is the hidden one that owns the messages of the
    /// session, and the handle of it is the `A_ScriptHwnd` of v2. It is
    /// made when the name is asked for the first time.
    pub script_window: Option<windows_sys::Win32::Foundation::HWND>,
    /// The windows of the script, by the name that the script gave them.
    ///
    /// The v1 names its windows, and the name is what the `Gui, Name:` and
    /// the `GuiControl, Name:` use to find them. The window that has no
    /// name is the one of the number one, and it is the window that the
    /// commands of the tree that carry no name use.
    ///
    /// The number of a control is the order of its creation inside the
    /// window that carries it.
    pub guis: std::collections::BTreeMap<String, super::gui::TheWindow>,
    /// The window of the v1 that the `LV_*` and the `TV_*` write to.
    ///
    /// The v1 names the list view and the tree view with the `Gui, Name:
    /// ListView` and the `Gui, Name: TreeView`, and the commands that
    /// write to them work on the window that carries them.
    pub window_of_the_lists: String,
    /// The menus of the script.
    ///
    /// The number of a menu is its place in this list, and it is what
    /// the `Menu` object holds.
    pub menus: Vec<super::menu::TheMenu>,
    /// The number that the next item of a menu of the machine takes.
    ///
    /// The number of an item is the number of the command that the
    /// machine sends when the item is chosen, and the number of a command
    /// is the low word of it: the numbers of the items of the machine
    /// stay under 65536.
    ///
    /// The v1 numbers the items of the menus with one counter for the
    /// whole of the script: the first item of the first menu of a script
    /// of the v1.1.37.02 is the number 11003.
    pub next_menu_item: usize,
    /// The numbers of the items that are free.
    ///
    /// An item that is deleted gives its number back, and the next item
    /// takes it: the numbers of the machine do not grow for ever.
    pub free_menu_items: Vec<usize>,
    /// The menu bar of the window of the script, when it has one.
    pub menu_bar: Option<super::menu::TheMenuBar>,
    /// The window of the tip of the machine.
    ///
    /// The tip is a window of its own: it is made when the script shows
    /// the first tip, and it is destroyed when the program ends.
    pub tip: Option<windows_sys::Win32::Foundation::HWND>,
    /// The hotkeys of the script.
    ///
    /// The state is shared with the thread of the machine that pumps the
    /// messages of the session: the hotkeys that fire arrive in the
    /// window of that thread.
    pub hotkeys: super::hotkeys::TheShared,
    /// Whether the thread of the hotkeys started.
    ///
    /// The thread starts when the first hotkey of the script is
    /// registered, and it is the owner of the window that the hotkeys
    /// arrive in.
    pub the_thread_of_the_hotkeys_started: bool,
    /// The objects of the COM that the machine gave.
    ///
    /// The machine is the owner of the pointer of each of them, and the
    /// desktop gives it back when the program ends.
    pub com: Vec<*mut core::ffi::c_void>,
    /// The events of the COM that the machine gave.
    ///
    /// The queue is shared with the `sink` that the machine calls, and it
    /// is what the script reads between its statements.
    pub events: std::sync::Arc<std::sync::Mutex<Vec<super::events::TheEvent>>>,
}

impl Drop for TheState {
    fn drop(&mut self) {
        for pointer in &self.com {
            unsafe { super::com::the_object_is_released(*pointer) };
        }
        // The tip is a window of the machine, and it goes with the
        // program.
        if let Some(tip) = self.tip.take() {
            unsafe {
                windows_sys::Win32::UI::WindowsAndMessaging::DestroyWindow(tip);
            }
        }
    }
}

impl TheState {
    /// The object of the COM is kept for the machine.
    ///
    /// The machine is the owner of the pointer, and the desktop gives it
    /// back when the program ends.
    pub fn the_com_is_kept(&mut self, object: &super::com::TheCom) {
        self.com
            .push(object.the_pointer() as *mut core::ffi::c_void);
    }

    /// The number of the next item of a menu of the machine.
    ///
    /// The number of an item is the number of the command that the
    /// machine sends when the item is chosen, and the number of a command
    /// is the low word of it: the number stays under 65536.
    pub fn the_number_of_a_menu_item(&mut self) -> usize {
        if let Some(number) = self.free_menu_items.pop() {
            return number;
        }
        let number = self.next_menu_item;
        self.next_menu_item += 1;
        number
    }

    /// The numbers of the items that are deleted go back to the machine.
    pub fn the_menu_items_are_free(&mut self, numbers: Vec<usize>) {
        for number in numbers {
            if number != 0 {
                self.free_menu_items.push(number);
            }
        }
    }
}

impl Default for TheState {
    fn default() -> Self {
        Self {
            windows: BTreeMap::new(),
            pictures: Vec::new(),
            files: TheFiles::new(),
            libraries: crate::native::Libraries::new(),
            tray_window: None,
            tray_icon: None,
            script_window: None,
            hotkeys: std::sync::Arc::new(std::sync::Mutex::new(
                super::hotkeys::TheHotkeys::default(),
            )),
            the_thread_of_the_hotkeys_started: false,
            guis: std::collections::BTreeMap::new(),
            window_of_the_lists: String::new(),
            menus: Vec::new(),
            next_menu_item: 0x1_000,
            free_menu_items: Vec::new(),
            menu_bar: None,
            tip: None,
            com: Vec::new(),
            events: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }
}
