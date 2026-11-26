//! The desktop of a Windows machine.
//!
//! The struct is the sum of the capabilities of the machine: the keyboard
//! of `user32`, the pointer, the screen, the windows, the processes, the
//! files, the clipboard, the hotkeys, the sound, the machine itself and
//! the dialogs. A capability that this wave does not carry keeps the
//! refusal of the trait, which is the reason of the tree.

use ahk::desktop::{
    Clipboard, Com, ComObject, ComValue, ControlAnswer, ControlEdit, ControlFlag, Dialogs, Drive,
    DriveSpace, Files, Folders, Hotkeys, Idle, Keyboard, Machine, Monitor, Native, Network,
    PictureOptions, Pictures, Pointer, ProcessState, Processes, Screen, Sound, ThePlaceOfTheV1,
    TheStyleOfTheV1, Timers, TitleMatch, WaitFor, WindowAnswer, WindowGeometry, WindowRef,
    Windows as WindowsCapability,
};

use super::machine;
use super::state::TheState;
use windows_sys::Win32::UI::WindowsAndMessaging::{IsWindow, SW_HIDE, SW_SHOWNA, ShowWindow};

/// The desktop of a Windows machine.
#[derive(Debug, Default)]
pub struct WindowsDesktop {
    /// The state that the capabilities share.
    ///
    /// The windows of the machine are looked at through it, and the
    /// windows that the script made live in it.
    state: TheState,
}

impl WindowsDesktop {
    /// The menus of the machine that are dead are made again.
    ///
    /// The machine takes the menu of an item away when the item is
    /// deleted, and the menus that the items of a menu opened go with it.
    /// The menus of the script live as long as the script does, and the
    /// v1 makes them again when they are needed.
    ///
    /// The menus are made again the deepest first: the menu of a menu
    /// that is made again takes the menus that its items opened away
    /// with it.
    fn the_dead_menus_are_made_again(&mut self) {
        let mut order: Vec<usize> = Vec::new();
        for place in 0..self.state.menus.len() {
            self.the_dead_menus_are_collected(place, &mut order, 0);
        }
        for place in order {
            self.the_menu_is_made_again(place);
        }
    }

    /// The place of the menu, and the places of the menus that it opens,
    /// the deepest first.
    fn the_dead_menus_are_collected(
        &self,
        place: usize,
        order: &mut Vec<usize>,
        depth: usize,
    ) {
        // The menus of the script may open one another, and the walk does
        // not go on for ever.
        if depth > 16 {
            return;
        }
        let Some(menu) = self.state.menus.get(place) else {
            return;
        };
        let children: Vec<usize> = menu
            .items
            .iter()
            .filter_map(|item| item.submenu)
            .collect();
        for child in children {
            self.the_dead_menus_are_collected(child, order, depth + 1);
        }
        if order.contains(&place) {
            return;
        }
        if menu.handle.is_null() || super::menu::TheMenu::the_menu_is_dead(menu) {
            order.push(place);
        }
    }

    /// The menu of the machine is made again, with the items it had.
    fn the_menu_is_made_again(&mut self, place: usize) {
        if std::env::var_os("AHK_DEBUG_CALLBACK").is_some()
            && self
                .state
                .menus
                .get(place)
                .map(|menu| menu.the_machine_makes_it_again())
                .unwrap_or(false)
        {
            eprintln!("the menu {place} is made again");
        }
        // The menus that the items open, and the handle they have now:
        // the popup of each of them, because the items of the menu open
        // the popup of the menu that they open.
        let children: Vec<(usize, windows_sys::Win32::UI::WindowsAndMessaging::HMENU)> = self
            .state
            .menus
            .iter()
            .enumerate()
            .map(|(place, menu)| (place, menu.handle))
            .collect();
        let Some(menu) = self.state.menus.get_mut(place) else {
            return;
        };
        let _ = menu.rebuild(&children);
        let handle = menu.handle;
        // The items that opened this menu have to open it again: the handle
        // of the machine is a new one, and the old one is gone.
        //
        // The v1 keeps the *menu* in the model of the script and reapplies
        // the link when the menu is made again; the tree keeps the handle,
        // and the link has to be put back by hand.
        for other in 0..self.state.menus.len() {
            if other == place {
                continue;
            }
            let Some(other_menu) = self.state.menus.get_mut(other) else {
                continue;
            };
            other_menu.the_submenu_is_attached_again(place, handle);
        }
    }

    /// The desktop of the machine that runs the script.
    pub fn new() -> Self {
        Self::default()
    }

    /// The thread of the machine that pumps the messages of the session.
    ///
    /// The thread starts when the first hotkey of the script is
    /// registered, and it is the owner of the window that the hotkeys
    /// arrive in.
    fn the_thread_of_the_hotkeys_starts(&mut self) -> Result<(), String> {
        if self.state.the_thread_of_the_hotkeys_started {
            return Ok(());
        }
        super::hotkeys::the_thread_starts(&self.state.hotkeys)?;
        self.state.the_thread_of_the_hotkeys_started = true;
        Ok(())
    }

    /// The window of the machine that owns the tray of it.
    ///
    /// The window is made the first time that the tray is used, and it
    /// is a window that only receives messages.
    fn the_window_of_the_tray(&mut self) -> Result<windows_sys::Win32::Foundation::HWND, String> {
        if let Some(window) = self.state.tray_window {
            return Ok(window);
        }
        let window = super::toplevel::a_message_window("ahk-rs-tray")?;
        self.state.tray_window = Some(window);
        Ok(window)
    }

    /// The window that the `+Owner` or the `+Parent` of a window of the
    /// v1 names.
    ///
    /// The owner is the handle of a window of the machine, or the name of
    /// another window of the script.
    ///
    /// Measured on the binary of v1.1.37.02: the handle
    /// may be written in decimal or in hexadecimal, and the two give the
    /// owner to the window; a number from 1 to 99 that is written with
    /// two characters or less is the number of a window of the script and
    /// not the handle of a window of the machine; the text that is
    /// nothing is the main window of the script, and the `+Parent` with
    /// no value is an error.
    fn the_owner_of_the_option(
        &mut self,
        owner: &str,
        parent: bool,
    ) -> Result<windows_sys::Win32::Foundation::HWND, String> {
        let text = owner.trim();
        if text.is_empty() {
            if parent {
                // The v1: the `+Parent` with no value is an error.
                return Err("Invalid or nonexistent owner or parent window.".to_string());
            }
            // The v1: the `+Owner` with no value owns the window with the
            // main window of the script, so that the window has no button
            // in the task bar.
            return self.the_window_of_the_script();
        }
        // The v1: the number is the handle of a window of the machine.
        if let Some(number) = the_number_of_the_owner(text) {
            // The v1: a number from 1 to 99 that is written with two
            // characters or less is the number of a window of the script.
            if !(1..=99).contains(&number) || text.chars().count() > 2 {
                let handle = number as windows_sys::Win32::Foundation::HWND;
                if unsafe { IsWindow(handle) } != 0 {
                    return Ok(handle);
                }
            }
        }
        // The v1: the name of another window of the script. The name is
        // matched without the case, as the v1 does.
        if let Some(other) = self
            .state
            .guis
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(text))
        {
            return Ok(other.1.handle);
        }
        // The v1: the window that the script made without a name is the
        // number one.
        if text == "1" {
            if let Some(other) = self.state.guis.get("") {
                return Ok(other.handle);
            }
        }
        Err("Invalid or nonexistent owner or parent window.".to_string())
    }

    /// The window of a title, for the controls of it.
    ///
    /// The mode of the match is the one of the v2: the title of a window
    /// that a control is looked for in starts with the text.
    fn the_window_of(
        &self,
        _control: &str,
        title: &str,
    ) -> Result<Option<windows_sys::Win32::Foundation::HWND>, String> {
        super::toplevel::the_handle_of_the_title(&self.state, title, TitleMatch::Starts)
    }

    /// The window of the script itself.
    ///
    /// The window of the script is the hidden one that owns the messages
    /// of the session: the thread of the script makes it, and the
    /// handle of it is the `A_ScriptHwnd` of v2.
    ///
    /// Measured on the binary of v2.0.31: the style of the
    /// window of the script is `80674816` and its extended style is `256`.
    fn the_window_of_the_script(&mut self) -> Result<windows_sys::Win32::Foundation::HWND, String> {
        if let Some(window) = self.state.script_window {
            return Ok(window);
        }
        // The thread of the machine has to run: the procedures of the
        // windows of the session find the state of it there, and the
        // hotkeys arrive in the window of that thread.
        self.the_thread_of_the_hotkeys_starts()?;
        // The window of the script is made by the thread of the script:
        // this is the thread that runs the script, and a window belongs
        // to the thread that makes it. The v1 makes the window of the
        // script in the thread of the script too, and the `A_ScriptHwnd`
        // of the v1 is the handle of a window of that thread -- measured
        // on the binary of v1.1.37.02, the `GetCurrentThreadId` of the
        // script and the `GetWindowThreadProcessId` of the
        // `A_ScriptHwnd` are the same number.
        let window = super::hotkeys::the_script_window_is_made(&self.state.hotkeys)?;
        self.state.script_window = Some(window);
        Ok(window)
    }

    /// The window of a title, for the questions of style.
    ///
    /// The mode of the match is the one that the script adjusted
    /// (`TitleMatchMode`), as in the v2: the `WinGetStyle` matches the
    /// title the way the script asked.
    fn the_window_of_the_title(
        &self,
        title: &str,
    ) -> Result<windows_sys::Win32::Foundation::HWND, String> {
        if title.trim().is_empty() {
            return Err(
                "the title is empty: the runtime asks the desktop by the title of the window \
                 that the script found last, and that window has no title."
                    .to_string(),
            );
        }
        let mode = ahk::settings::title_match_mode();
        let Some(window) = super::toplevel::the_handle_of_the_title(&self.state, title, mode)?
        else {
            return Err(format!(
                "no window of the machine has a title that matches {title:?}."
            ));
        };
        Ok(window)
    }
}

/// The desktop is the sum of the capabilities.
///
/// The trait of the sum has no method of its own: it says that the
/// desktop answers every question of the runtime.
impl ahk::desktop::Desktop for WindowsDesktop {}

impl Machine for WindowsDesktop {
    fn user_name(&mut self) -> Result<String, String> {
        machine::user_name()
    }

    fn computer_name(&mut self) -> Result<String, String> {
        machine::computer_name()
    }

    fn shell(&mut self) -> Result<String, String> {
        machine::shell()
    }

    fn folders(&mut self) -> Result<Folders, String> {
        machine::folders()
    }

    fn os_version(&mut self) -> Result<String, String> {
        machine::os_version()
    }

    fn language(&mut self) -> Result<String, String> {
        machine::language()
    }

    fn is_admin(&mut self) -> Result<bool, String> {
        machine::is_admin()
    }

    fn last_error(&mut self) -> Result<i64, String> {
        machine::last_error()
    }

    fn set_last_error(&mut self, code: i64) -> Result<(), String> {
        machine::set_last_error(code)
    }

    fn the_text_of_an_error(&mut self, code: i64) -> Result<String, String> {
        machine::the_text_of_an_error(code)
    }

    fn executable(&mut self) -> Result<String, String> {
        machine::executable()
    }

    fn tray_set_icon(&mut self, name: &str) -> Result<(), String> {
        // The icon of the tray belongs to the window of the script: the
        // window is in the thread of the script, so the mouse of the
        // icon arrives in it, and the `NIM_DELETE` of the end of the run
        // points at the same window and at the same number.
        let window = self.the_window_of_the_script()?;
        self.state.tray_icon = Some(name.to_string());
        super::trash::set_the_icon(window, name, &the_title_of_the_script())
    }

    fn icon_of_a_file(&mut self, path: &str, number: i64) -> Result<(), String> {
        super::trash::the_icon_of_the_file_is_read(path, number)
    }

    fn tray_of_the_script_is_shown(&mut self) -> Result<(), String> {
        // The window of the script is the one of the thread of the
        // script, so the mouse of the tray arrives in it.
        let window = self.the_window_of_the_script()?;
        // The icon of the tray is the one that the script gave, and the
        // one of the program of the script when it gave none.
        let icon = self.state.tray_icon.clone();
        super::trash::show_the_tray(window, &the_title_of_the_script(), icon.as_deref())
    }

    fn tray_exit_was_asked(&mut self) -> bool {
        self.state
            .hotkeys
            .lock()
            .map(|hotkeys| hotkeys.exit_asked)
            .unwrap_or(false)
    }

    fn tray_of_the_script_is_removed(&mut self) -> Result<(), String> {
        if let Some(window) = self.state.script_window {
            super::trash::remove_the_tray(window);
        }
        Ok(())
    }

    fn drive_eject(&mut self, _path: &str) -> Result<(), String> {
        Err("the drives of the machine are driven by the machine alone, and the tree does not drive them."
            .to_string())
    }

    fn drive_lock(&mut self, _path: &str) -> Result<(), String> {
        Err("the drives of the machine are driven by the machine alone, and the tree does not drive them."
            .to_string())
    }

    fn drive_unlock(&mut self, _path: &str) -> Result<(), String> {
        Err("the drives of the machine are driven by the machine alone, and the tree does not drive them."
            .to_string())
    }

    fn drive_retract(&mut self, _path: &str) -> Result<(), String> {
        Err("the drives of the machine are driven by the machine alone, and the tree does not drive them."
            .to_string())
    }

    fn drive_set_label(&mut self, path: &str, label: &str) -> Result<(), String> {
        machine::drive_set_label(path, label)
    }

    fn drive_space(&mut self, path: &str) -> Result<Option<DriveSpace>, String> {
        machine::drive_space(path)
    }

    fn registry_value(
        &mut self,
        key: &str,
        name: &str,
    ) -> Result<Option<(String, String)>, String> {
        machine::registry_value(key, name)
    }

    fn registry_write(
        &mut self,
        key: &str,
        name: &str,
        kind: &str,
        value: &str,
    ) -> Result<(), String> {
        machine::registry_write(key, name, kind, value)
    }

    fn registry_delete(&mut self, key: &str, name: &str) -> Result<(), String> {
        machine::registry_delete(key, name)
    }

    fn registry_delete_key(&mut self, key: &str) -> Result<(), String> {
        machine::registry_delete_key(key)
    }

    fn registry_create_key(&mut self, key: &str) -> Result<(), String> {
        machine::registry_create_key(key)
    }

    fn registry_enumerate(
        &mut self,
        key: &str,
        mode: &str,
        quiet: bool,
    ) -> Result<Vec<ahk::desktop::TheRegEntry>, String> {
        machine::registry_enumerate(key, mode, quiet)
    }

    fn windows_directory(&mut self) -> Result<String, String> {
        machine::windows_directory()
    }

    fn program_files(&mut self) -> Result<String, String> {
        machine::program_files()
    }

    fn drive_label(&mut self, path: &str) -> Result<Option<String>, String> {
        machine::drive_label(path)
    }

    fn drive_serial(&mut self, path: &str) -> Result<Option<String>, String> {
        machine::drive_serial(path)
    }

    fn drive_list(&mut self) -> Result<Vec<String>, String> {
        machine::drive_list()
    }

    fn drive_status_cd(&mut self, path: &str) -> Result<Option<String>, String> {
        machine::drive_status_cd(path)
    }

    fn idle_times(&mut self) -> Result<Idle, String> {
        machine::idle_times()
    }

    fn drive_status(&mut self, path: &str) -> Result<Option<String>, String> {
        machine::drive_status(path)
    }

    fn drive_type(&mut self, path: &str) -> Result<Option<String>, String> {
        machine::drive_type(path)
    }

    fn drive(&mut self, path: &str) -> Result<Option<Drive>, String> {
        machine::drive(path)
    }

    fn output_debug(&mut self, text: &str) -> Result<(), String> {
        machine::output_debug(text)
    }

    fn the_current_thread_id(&mut self) -> Result<i64, String> {
        machine::the_current_thread_id()
    }

    fn create_worker_thread(
        &mut self,
        source: &str,
        dialect: &str,
        function: &str,
        args: Vec<ahk::desktop::ThreadValue>,
        globals: Vec<(String, ahk::desktop::ThreadValue)>,
        shared: Option<ahk::desktop::SharedGlobals>,
    ) -> Result<u64, String> {
        crate::workers::create(
            super::workers::a_windows_desktop,
            source,
            dialect,
            function,
            args,
            globals,
            shared,
        )
    }

    fn worker_thread_state(&mut self, id: u64) -> Result<ahk::desktop::WorkerState, String> {
        crate::workers::state(id)
    }

    fn worker_thread_list(&mut self) -> Result<Vec<(u64, String)>, String> {
        crate::workers::list()
    }
}

/// The windows of the machine.
impl WindowsCapability for WindowsDesktop {
    fn window_active(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        super::toplevel::window_active(&self.state, title, mode)
    }

    fn window_exists(&mut self, title: &str, mode: TitleMatch) -> Result<WindowAnswer, String> {
        super::toplevel::window_exists(&self.state, title, mode)
    }

    fn active_window_title(&mut self) -> Result<Option<String>, String> {
        super::toplevel::active_window_title(&self.state)
    }

    fn matching_windows(
        &mut self,
        title: &str,
        mode: TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        super::toplevel::matching_windows(&self.state, title, mode)
    }

    fn window_controls(&mut self, id: &str, handles: bool) -> Result<Option<String>, String> {
        // The identifier of the window is the one that the desktop names it
        // with, and the number of the handle is in it.
        let Ok(number) = id.parse::<usize>() else {
            return Ok(None);
        };
        let handle = number as windows_sys::Win32::Foundation::HWND;
        if handle.is_null() || unsafe { IsWindow(handle) } == 0 {
            return Ok(None);
        }
        Ok(unsafe { super::toplevel::the_controls_of_the_window(handle, handles) })
    }

    fn window_text(&mut self, id: &str) -> Result<Option<String>, String> {
        // The identifier of the window is the one that the desktop names it
        // with, and the number of the handle is in it.
        let Ok(number) = id.parse::<usize>() else {
            return Ok(None);
        };
        let handle = number as windows_sys::Win32::Foundation::HWND;
        if handle.is_null() || unsafe { IsWindow(handle) } == 0 {
            return Ok(None);
        }
        // The texts of the child windows of it are the text of the window:
        // the v2 enumerates them (source `lib/win.cpp:2075` of the v2.0.32).
        Ok(Some(unsafe { super::toplevel::the_texts_of_the_window(handle) }))
    }

    fn window_exe(&mut self, id: i64) -> String {
        super::toplevel::the_exe_of_the_window(id)
    }

    fn window_geometry(
        &mut self,
        title: &str,
        mode: TitleMatch,
    ) -> Result<Option<WindowGeometry>, String> {
        super::toplevel::window_geometry(&self.state, title, mode)
    }

    fn window_min_max(&mut self, title: &str, mode: TitleMatch) -> Result<Option<i64>, String> {
        super::toplevel::window_min_max(&self.state, title, mode)
    }

    fn window_wait(
        &mut self,
        title: &str,
        mode: TitleMatch,
        what: WaitFor,
        timeout_ms: u64,
        excludes: &[String],
    ) -> Result<Option<String>, String> {
        super::toplevel::window_wait(&self.state, title, mode, what, timeout_ms, excludes)
    }

    fn window_pid(&mut self, title: &str) -> Result<Option<i64>, String> {
        super::toplevel::window_pid(&self.state, title)
    }

    /// The style of a window, or its extended style.
    ///
    /// The number is the one of bits of Windows: the `WS_*` of the style
    /// and the `WS_EX_*` of the extended style. A window that is not
    /// there is a failure, and not a zero.
    fn window_style(&mut self, title: &str, extended: bool) -> Result<i64, String> {
        let window = self.the_window_of_the_title(title)?;
        unsafe { super::styles::the_style_of(window, extended) }
    }

    /// The style of a window is written, or its extended style.
    ///
    /// The `style` is the new **absolute** style: the one that computes
    /// the `+`, the `-` and the `^` over the style that was there is the
    /// runtime.
    ///
    /// What the machine does is what the v2 does: the writing is
    /// checked by the reading that comes after it, the frame of the
    /// window is drawn again with the `SetWindowPos` and the window is
    /// invalidated with the `InvalidateRect`.
    fn window_set_style(&mut self, title: &str, style: i64, extended: bool) -> Result<(), String> {
        let window = self.the_window_of_the_title(title)?;
        unsafe { super::styles::the_style_is_written(window, style, extended, true) }
    }

    /// The colour that a window lets through.
    ///
    /// The colour is the colour key of a layered window, and the text is
    /// the `0x%06X` of the RGB that the v2 writes. A window that is not
    /// layered answers the empty text.
    fn window_trans_color(&mut self, title: &str) -> Result<String, String> {
        let window = self.the_window_of_the_title(title)?;
        unsafe { super::styles::the_transparent_colour_of(window) }
    }

    /// The colour that a window lets through is written.
    ///
    /// The `color` comes normalised: `0xRRGGBB` (the RGB that the
    /// script wrote) or the empty text (the colour goes away). The
    /// window becomes layered when the colour comes, and it loses the
    /// `WS_EX_LAYERED` when the colour goes away.
    fn window_set_trans_color(&mut self, title: &str, color: &str) -> Result<(), String> {
        let window = self.the_window_of_the_title(title)?;
        unsafe { super::styles::the_transparent_colour_is_written(window, color) }
    }

    /// The style of a control of a window, or its extended style.
    ///
    /// A control is a window of its own, and its style is read the same
    /// way as the one of a window.
    fn control_style(&mut self, control: &str, title: &str, extended: bool) -> Result<i64, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Err(format!(
                "no window of the machine has a title that matches {title:?}."
            ));
        };
        super::controls::control_style(window, control, extended)
    }

    /// The style of a control of a window is written, or its extended
    /// style.
    ///
    /// The `style` is the new **absolute** style: the one that computes
    /// the `+`, the `-` and the `^` over the style that was there is the
    /// runtime.
    fn control_set_style(
        &mut self,
        control: &str,
        title: &str,
        style: i64,
        extended: bool,
    ) -> Result<(), String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Err(format!(
                "no window of the machine has a title that matches {title:?}."
            ));
        };
        super::controls::control_set_style(window, control, style, extended)
    }

    fn control_hwnds_of(&mut self, window: i64) -> Result<Vec<i64>, String> {
        Ok(super::controls::control_hwnds(
            window as isize as windows_sys::Win32::Foundation::HWND,
        ))
    }

    fn control_classnn_of(&mut self, control: i64) -> Result<String, String> {
        Ok(super::controls::control_classnn(
            control as isize as windows_sys::Win32::Foundation::HWND,
        ))
    }

    fn control_style_of_hwnd(&mut self, control: i64, extended: bool) -> Result<i64, String> {
        super::controls::control_style_by_hwnd(
            control as isize as windows_sys::Win32::Foundation::HWND,
            extended,
        )
    }

    fn control_set_style_of_hwnd(
        &mut self,
        control: i64,
        style: i64,
        extended: bool,
    ) -> Result<(), String> {
        super::controls::control_set_style_by_hwnd(
            control as isize as windows_sys::Win32::Foundation::HWND,
            style,
            extended,
        )
    }

    /// The number of the window of the script.
    ///
    /// The window of the script is the hidden window that carries the
    /// messages of the session: the thread of the session makes it, and
    /// its number is the `A_ScriptHwnd` of the v2.
    ///
    /// Measured on the binary of v2.0.31: the style of
    /// the window of the script is `80674816` and the extended style is
    /// `256`.
    fn script_window(&mut self) -> Result<i64, String> {
        // The thread of the session is the one that pumps the messages,
        // and the window of the script is made by it: the thread begins
        // here, if it has not begun yet.
        let window = self.the_window_of_the_script()?;
        Ok(window as i64)
    }

    fn carries_the_gui(&mut self) -> bool {
        true
    }

    fn gui_is_open(&mut self) -> bool {
        self.state.guis.contains_key("")
    }

    fn gui_open(&mut self, title: &str) -> Result<u64, String> {
        // The tree makes one window at a time, and the window that is
        // there is not replaced by another one without a word about it.
        if self.state.guis.contains_key("") {
            return Err(
                "this desktop makes one window at a time, and a window of the script is \
                 already there."
                    .to_string(),
            );
        }
        let window = super::gui::open(title)?;
        self.state.guis.insert(String::new(), window);
        Ok(1)
    }

    fn gui_add(&mut self, _handle: u64, kind: &str, text: &str) -> Result<(u64, String), String> {
        let Some(window) = self.state.guis.get_mut("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::add(window, kind, text)
    }

    fn gui_show(&mut self, _handle: u64) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_window_is_shown(window, true);
        Ok(())
    }

    fn gui_hide(&mut self) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_window_is_shown(window, false);
        Ok(())
    }

    fn gui_destroy(&mut self, _handle: u64) -> Result<(), String> {
        let Some(window) = self.state.guis.remove("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_window_is_closed(&window);
        Ok(())
    }

    fn gui_title(&mut self, _handle: u64, setting: Option<&str>) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        match setting {
            Some(title) => {
                super::gui::the_title_is_written(window, title);
                Ok(title.to_string())
            }
            None => Ok(super::gui::the_title_of(window)),
        }
    }

    fn gui_place(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
    ) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_place_is_given(window, x, y, width, height)
    }

    fn gui_value(&mut self, number: u64, setting: Option<&str>) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match setting {
            // The text of the control is written when the setting is there,
            // and read when it is not.
            Some(text) => {
                super::gui::the_text_is_written(control, text);
                Ok(text.to_string())
            }
            None => Ok(super::gui::the_text_of(control)),
        }
    }

    fn gui_number_named(&mut self, classnn: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        for (index, _control) in window.controls.iter().enumerate() {
            let number = super::gui::the_classnn_of(window, index as u64 + 1);
            if number.eq_ignore_ascii_case(classnn) {
                return Ok(index as i64 + 1);
            }
        }
        // The v1 names a control by its ClassNN or by its text: measured
        // in the CHM of the v1.1.37.02, the `ControlID` can be the
        // ClassNN of the control or its text, and the text matches with
        // the mode of the title.
        if !classnn.is_empty() {
            let mode = ahk::settings::title_match_mode();
            for (index, control) in window.controls.iter().enumerate() {
                if mode.matches(&super::gui::the_text_of(control), classnn).unwrap_or(false) {
                    return Ok(index as i64 + 1);
                }
            }
        }
        Err(format!("there is no control named {classnn}"))
    }

    fn gui_classnn(&mut self, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        if super::gui::the_control(window, number).is_none() {
            return Err(format!("there is no control with the number {number}"));
        }
        Ok(super::gui::the_classnn_of(window, number))
    }

    fn gui_handle(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(super::gui::the_handle_of(control))
    }

    fn gui_focused(&mut self) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(focused) = super::gui::the_focused() else {
            return Ok(String::new());
        };
        for (index, control) in window.controls.iter().enumerate() {
            if control.handle == focused {
                return Ok(super::gui::the_classnn_of(window, index as u64 + 1));
            }
        }
        Ok(String::new())
    }

    fn gui_focus(&mut self, number: u64) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_focus_is_given(control);
        Ok(())
    }

    fn gui_control_pos(&mut self, number: u64) -> Result<(i64, i64, i64, i64), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        // The place of a control is the place of it in the client area
        // of the window that holds it.
        //
        // The CHM of the v1.1.37.02 says of the sub-command "Pos" of the
        // `GuiControlGet`: "The position is relative to the GUI window's
        // client area, which is the area not including title bar, menu bar,
        // and borders."
        //
        // Measured on the binary of v1.1.37.02: the
        // `GuiControlGet, Edit, Pos` of an edit that the script put in the
        // window of the script answers 10,35, and the machine has the
        // control at 10,35 of the client area of the window.
        Ok(super::gui::the_place_of_the_control_in_the_window(window, control))
    }

    fn gui_size(&mut self, number: u64, width: i64, height: i64) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_size_is_changed(control, width, height);
        Ok(())
    }

    fn gui_line_count(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(super::gui::the_number_of_lines(control))
    }

    fn gui_caret(&mut self) -> Result<Option<(i64, i64)>, String> {
        Ok(super::gui::the_place_of_the_caret())
    }

    fn gui_combo_items(&mut self, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_items_of_a_combo(control)
    }

    fn gui_combo_add(&mut self, number: u64, text: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_combo_is_added(control, text)
    }

    fn gui_combo_select(
        &mut self,
        number: u64,
        wanted: &str,
        by_text: bool,
    ) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_combo_is_chosen(control, wanted, by_text)
    }

    fn gui_combo_choice(&mut self, number: u64) -> Result<(i64, String), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_choice_of_a_combo(control)
    }

    fn gui_combo_delete(&mut self, number: u64, index: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_combo_is_removed(control, index)
    }

    fn gui_combo_find(&mut self, number: u64, text: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_combo_is_found(control, text)
    }

    fn gui_combo_dropdown(&mut self, number: u64, show: bool) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_list_of_a_combo_is_shown(control, show)
    }

    fn gui_list_add(&mut self, number: u64, cells: &[String]) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_list_is_added(control, cells)
    }

    fn gui_list_count(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_number_of_the_items_of_a_list(control)
    }

    fn gui_list_text(&mut self, number: u64, row: i64, col: i64) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        let text = super::gui::the_text_of_an_item_of_a_list(control, row.saturating_sub(1))?;
        // The columns of an item are apart with a tab.
        Ok(text
            .split('\t')
            .nth(col.max(0) as usize)
            .unwrap_or("")
            .to_string())
    }

    fn gui_list_content(&mut self, number: u64, selected: bool) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        if selected {
            super::gui::the_chosen_items_of_a_list(control)
        } else {
            super::gui::the_items_of_a_list(control)
        }
    }

    fn gui_list_modify(&mut self, number: u64, row: i64, cells: &[String]) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_list_is_written(control, row, cells)
    }

    fn gui_list_next(&mut self, number: u64, start: i64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_next_chosen_item_of_a_list(control, start)
    }

    fn gui_tree_add(
        &mut self,
        number: u64,
        text: &str,
        parent: i64,
        options: &str,
    ) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_tree_is_added(control, text, parent, options)
    }

    fn gui_tree_count(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_number_of_the_items_of_a_tree(control)
    }

    fn gui_tree_text(&mut self, number: u64, item: i64) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_text_of_an_item_of_a_tree(control, item)
    }

    fn gui_tree_relation(&mut self, number: u64, item: i64, relation: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match relation.trim().to_lowercase().as_str() {
            "parent" => super::gui::the_parent_of_an_item_of_a_tree(control, item),
            other => Err(format!(
                "the machine knows the relation Parent and not {other}."
            )),
        }
    }

    fn gui_tree_selection(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_chosen_item_of_a_tree(control)
    }

    fn gui_tree_modify(&mut self, number: u64, item: i64, options: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        // The options that the machine carries are the ones of the v2:
        // "Select" chooses the item, and the name of it is the one that
        // the machine writes.
        if options.trim().eq_ignore_ascii_case("select") {
            let chosen = super::gui::the_item_of_a_tree_is_chosen(control, item)?;
            return Ok(chosen);
        }
        Err("the machine does not change an item of a tree: the options are Select.".to_string())
    }

    fn gui_tree_delete(&mut self, number: u64, item: i64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_tree_is_removed(control, item)?;
        Ok(item)
    }

    // -- the windows of the v1 ----------------------------------------------------
    //
    // The v1 names its windows: each name is a window of its own, and the
    // controls of a window are numbered from one inside it.

    fn v1_gui_windows(&mut self) -> Result<Vec<String>, String> {
        Ok(self.state.guis.keys().cloned().collect())
    }

    fn v1_gui_open(&mut self, name: &str, title: &str) -> Result<String, String> {
        if self.state.guis.contains_key(name) {
            return Ok(name.to_string());
        }
        // The window of the v1 is a window of its own with the styles
        // that the v1 gives to it: the `WS_POPUP` makes it a window of
        // its own and not a control, and a script of the v1 owns
        // the windows that it paints its picture into.
        let mut window = super::gui::open_of_the_v1(title)?;
        window.name = name.to_string();
        super::gui::the_window_name_is_set(window.handle as isize, &window.name, &window.label);
        self.state.guis.insert(name.to_string(), window);
        Ok(name.to_string())
    }

    fn v1_gui_is_open(&mut self, name: &str) -> Result<bool, String> {
        Ok(self.state.guis.contains_key(name))
    }

    fn v1_gui_number_of(&mut self, name: &str) -> Result<i64, String> {
        if !self.state.guis.contains_key(name) {
            return Err(format!("there is no window named {name}"));
        }
        // The window that has no name is the one of the number one, and the
        // other ones are numbered after it, the way the v1 numbers them.
        let mut names: Vec<String> = self.state.guis.keys().cloned().collect();
        names.sort();
        let place = names.iter().position(|other| other == name).unwrap_or(0);
        Ok(place as i64 + 1)
    }

    fn v1_gui_add(
        &mut self,
        window: &str,
        kind: &str,
        variable: &str,
        text: &str,
        style: TheStyleOfTheV1,
    ) -> Result<(u64, String), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::add_named(window, kind, text, variable, None, style)
    }

    fn v1_gui_place(
        &mut self,
        window: &str,
        number: u64,
        place: ThePlaceOfTheV1,
    ) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        let (_, _, old_width, old_height) = super::gui::the_place_of(control);
        // The place of the control is the one that the options of the v1
        // ask for, and the machine computes it from the window and from
        // the control that was added before it.
        let (x, y, req_width, mut req_height) =
            super::gui::the_place_of_a_control_is_computed(window, number, place);
        if req_height.is_none() {
            if let Some(rows) = place.rows {
                req_height = Some(super::gui::the_height_of_rows(window, &control.kind, rows));
            }
        }
        let style = super::gui::the_style_of_the_control(control);
        let (width, height) = super::gui::the_size_of_a_control(
            window,
            &control.kind,
            &control.text,
            style,
            req_width,
            req_height,
        );
        let the_place = super::gui::ThePlace {
            x,
            y,
            width: if width > 0 { width } else { old_width },
            height: if height > 0 { height } else { old_height },
        };
        super::gui::the_place_of_a_control_is_written(control, the_place);
        // The image of a picture control goes into it when the control
        // has its place: pass the requested dimensions (preserving -1 for
        // aspect-ratio scaling) so high-res icons and bitmaps scale properly.
        if control.kind == "picture" {
            super::gui::the_picture_is_loaded(control, req_width, req_height);
        }
        let is_tab = control.class == "SysTabControl32";
        // The v1: the first control of a window always begins a section,
        // and the options of a control may ask for a new one.
        //
        // Read in the source of the v1 (`script_gui.cpp:4099-4107`).
        if place.section || number <= 1 {
            window.section = Some((x, y));
            window.section_at = Some(number.max(1) as usize - 1);
        }
        if !is_tab {
            window.tab_just_switched = false;
            window.layout_anchor_control = None;
        }
        Ok(())
    }

    fn v1_gui_use_window(&mut self, window: &str) -> Result<(), String> {
        self.state.window_of_the_lists = window.to_string();
        Ok(())
    }

    fn v1_gui_window_place(
        &mut self,
        window: &str,
        x: Option<i64>,
        y: Option<i64>,
        width: Option<i64>,
        height: Option<i64>,
        centered_x: bool,
        centered_y: bool,
    ) -> Result<(), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        // The v1: the first show of the window gives it the size of the
        // controls that it holds, and puts it in the middle of the screen.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02
        // (the probe of the measurement): the `Gui, B: Show` of a window that
        // holds a button of 30x20 at 0,0 and a button of 30x20 at 40,30
        // gives a client area of 70x50, and the window is in the middle of
        // the screen. The source of the v1 (`script_gui.cpp:6763-6786` and
        // `script_gui.cpp:6815-6819`) asks `mGuiShowHasNeverBeenDone`
        // before it reads the size of the client area, and it centers the
        // window that is shown for the first time.
        let first = !window.shown;
        let (mut width, mut height) = (width, height);
        if first {
            let (control_width, control_height) = super::gui::the_size_of_the_controls(window);
            width = width.or(Some(control_width));
            height = height.or(Some(control_height));
        }
        // The v1: the window that has no size is not moved, and the one
        // that has a size is centered in the work area of the screen.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02
        // (the probe of the measurement): the `Gui, A: Show` of a window that holds no
        // control leaves the window at (-32768,-32768) with a client area
        // of 0x0 -- the window is neither moved nor resized. The tree put
        // it in the middle of the screen, and a script of the v1
        // paints its picture in a window like this one: the canvas of it
        // was in the middle of the screen instead of the corner of the
        // window of the program, and the background of the program was
        // not seen.
        let empty = width.unwrap_or(0) <= 0 && height.unwrap_or(0) <= 0;
        let mut x = x;
        let mut y = y;
        let mut centered_x = centered_x && !empty;
        let mut centered_y = centered_y && !empty;
        if !empty {
            centered_x = centered_x || (first && x.is_none());
            centered_y = centered_y || (first && y.is_none());
        }
        if centered_x || centered_y {
            // The v1 centers the window in the work area of the screen --
            // the source of the v1 (`script_gui.cpp:6863-6865`) asks for it
            // with the `SPI_GETWORKAREA` -- and it centers the window
            // itself, with the border and the caption around it.
            let (left, top, right, bottom) = super::screen::monitor_work_area(
                super::screen::primary_monitor()?,
            )?
            .unwrap_or_else(|| {
                let (width, height) = super::screen::screen_size().unwrap_or((0, 0));
                (0, 0, width, height)
            });
            if centered_x && x.is_none() {
                let (width, _) = super::gui::the_size_of_the_window(window, width, height);
                let width = width.unwrap_or_else(|| super::gui::the_width_of(window));
                x = Some(left + (right - left - width) / 2);
            }
            if centered_y && y.is_none() {
                let (_, height) = super::gui::the_size_of_the_window(window, width, height);
                let height = height.unwrap_or_else(|| super::gui::the_height_of(window));
                y = Some(top + (bottom - top - height) / 2);
            }
        }
        super::gui::the_place_is_given(window, x, y, width, height)
    }

    fn v1_gui_number_named(&mut self, window: &str, name: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control_named(window, name) else {
            // The text of a control is the way the v1 names it when it has
            // no variable and no ClassNN that matches. The mode of the
            // title of the machine says how the texts are compared.
            let mode = ahk::settings::title_match_mode();
            let Some(control) = super::gui::the_control_with_the_text(window, name, |text, name| {
                mode.matches(text, name).unwrap_or(false)
            }) else {
                return Err(format!("there is no control named {name}"));
            };
            return Ok(window
                .controls
                .iter()
                .position(|other| other.handle == control.handle)
                .map(|index| index as i64 + 1)
                .unwrap_or(0));
        };
        Ok(window
            .controls
            .iter()
            .position(|other| other.handle == control.handle)
            .map(|index| index as i64 + 1)
            .unwrap_or(0))
    }

    fn v1_gui_classnn(&mut self, window: &str, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        if super::gui::the_control(window, number).is_none() {
            return Err(format!("there is no control with the number {number}"));
        }
        Ok(super::gui::the_classnn_of(window, number))
    }

    fn v1_gui_control_name(&mut self, window: &str, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(control.name.clone())
    }

    fn v1_gui_choose(
        &mut self,
        window: &str,
        number: u64,
        value: &str,
        by_text: bool,
    ) -> Result<i64, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_item_of_a_control_is_chosen(control, value, by_text)
    }

    fn v1_gui_tab(&mut self, window: &str, tab: i64) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_tab_is_chosen(window, tab)
    }

    fn v1_gui_name_of_handle(&mut self, handle: i64) -> Result<String, String> {
        for (name, window) in &self.state.guis {
            if window.handle as i64 == handle {
                return Ok(name.clone());
            }
        }
        Err(format!("there is no window with the handle {handle}"))
    }

    fn v1_gui_handle(&mut self, window: &str, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(super::gui::the_handle_of(control))
    }

    fn v1_gui_window_handle(&mut self, window: &str) -> Result<i64, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        Ok(window.handle as i64)
    }

    fn v1_gui_value(
        &mut self,
        window: &str,
        number: u64,
        setting: Option<&str>,
    ) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match setting {
            Some(text) => {
                super::gui::the_text_is_written(control, text);
                Ok(text.to_string())
            }
            // The v1: the contents of the control are the state of a check
            // box and of a radio button, the position of a bar of the
            // progress and of a field of numbers, and the text of it for
            // the others.
            //
            // Measured on the binary of v1.1.37.02: the
            // `GuiControlGet, out, , CHK` of a check box that the script
            // made checked answers "1".
            None => Ok(super::gui::the_contents_of_the_v1(control)),
        }
    }

    fn v1_gui_caption(&mut self, window: &str, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        // The v1: the caption of a control is the text that the machine
        // shows in it, and it is what the `GuiControlGet` with the "Text"
        // in the `Value` gives.
        Ok(super::gui::the_text_of(control))
    }

    fn v1_gui_flag(
        &mut self,
        window: &str,
        number: u64,
        what: &str,
        setting: Option<bool>,
    ) -> Result<i64, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match what {
            "enabled" => {
                if let Some(on) = setting {
                    super::gui::the_enabling_is_changed(control, on);
                }
                Ok(super::gui::the_control_is_enabled(control) as i64)
            }
            "visible" => {
                if let Some(on) = setting {
                    super::gui::the_visibility_is_changed(control, on);
                }
                Ok(super::gui::the_control_is_visible(control) as i64)
            }
            other => Err(format!("the tree does not carry the {other} of a control.")),
        }
    }

    fn v1_gui_visible(&mut self, window: &str, number: u64, on: bool) -> Result<(), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_visibility_is_changed(control, on);
        Ok(())
    }

    fn v1_gui_focus(&mut self, window: &str, number: u64) -> Result<(), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_focus_is_given(control);
        Ok(())
    }

    fn v1_gui_focused(&mut self, window: &str) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(focused) = super::gui::the_focused() else {
            return Ok(String::new());
        };
        for (index, control) in window.controls.iter().enumerate() {
            if control.handle == focused {
                return Ok(super::gui::the_classnn_of(window, index as u64 + 1));
            }
        }
        Ok(String::new())
    }

    fn v1_gui_control_pos(
        &mut self,
        window: &str,
        number: u64,
    ) -> Result<(i64, i64, i64, i64), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        // The place of a control is the place of it in the client area
        // of the window that holds it.
        //
        // The CHM of the v1.1.37.02 says of the sub-command "Pos" of the
        // `GuiControlGet`: "The position is relative to the GUI window's
        // client area, which is the area not including title bar, menu bar,
        // and borders."
        //
        // Measured on the binary of v1.1.37.02: the
        // `GuiControlGet, Edit, Pos` of an edit that the script put in the
        // window of the script answers 10,35, and the machine has the
        // control at 10,35 of the client area of the window.
        Ok(super::gui::the_place_of_the_control_in_the_window(window, control))
    }

    fn v1_gui_show(&mut self, window: &str, title: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        if !title.trim().is_empty() {
            super::gui::the_title_is_written(window, title);
        }
        super::gui::the_window_is_shown(window, true);
        Ok(())
    }

    fn v1_gui_show_without_activating(&mut self, window: &str, title: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        if !title.trim().is_empty() {
            super::gui::the_title_is_written(window, title);
        }
        super::gui::the_window_is_shown_without_activating(window);
        Ok(())
    }

    fn v1_gui_auto_size(&mut self, window: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_window_is_sized_to_its_controls(window);
        Ok(())
    }

    fn v1_gui_hide(&mut self, window: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_window_is_shown(window, false);
        Ok(())
    }

    fn v1_gui_destroy(&mut self, window: &str) -> Result<(), String> {
        let Some(mut window) = self.state.guis.remove(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_window_is_closed(&window);
        window.controls.clear();
        Ok(())
    }

    fn v1_gui_title(&mut self, window: &str, setting: Option<&str>) -> Result<String, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        match setting {
            Some(title) => {
                super::gui::the_title_is_written(window, title);
                Ok(title.to_string())
            }
            None => Ok(super::gui::the_title_of(window)),
        }
    }

    fn v1_gui_option(
        &mut self,
        window: &str,
        name: &str,
        value: Option<&str>,
        on: bool,
    ) -> Result<(), String> {
        match value {
            Some(value) => {
                let Some(window) = self.state.guis.get_mut(window) else {
                    return Err(format!("there is no window named {window}"));
                };
                super::gui::the_option_with_a_value_is_applied(window, name, value, on)
            }
            None => {
                let Some(window) = self.state.guis.get(window) else {
                    return Err(format!("there is no window named {window}"));
                };
                super::gui::the_option_is_applied(window, name, on)
            }
        }
    }

    fn v1_gui_style(
        &mut self,
        window: &str,
        extended: bool,
        style: u32,
        on: bool,
    ) -> Result<(), String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_style_is_applied(window, extended, style, on)
    }

    fn v1_gui_owner(
        &mut self,
        window: &str,
        owner: Option<&str>,
        parent: bool,
    ) -> Result<(), String> {
        // The owner is the name of another window of the script, or the
        // handle of a window of the machine.
        //
        // Measured on the binary of v1.1.37.02: the
        // `+Owner` with the handle of a window of the machine gives the
        // owner to the window, and the handle may be written in decimal
        // or in hexadecimal; the `+Owner` with no value gives the owner
        // that the main window of the script has; the `+Owner` with a
        // number that is not a window of the machine and the `+Owner` with
        // a name that is not a window of the script are the error
        // "Invalid or nonexistent owner or parent window."
        let handle = match owner {
            // The v1: the `-Owner` takes the owner away from the window.
            None => None,
            Some(text) => Some(self.the_owner_of_the_option(text, parent)?),
        };
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let Some(handle) = handle else {
            super::gui::the_owner_is_removed(window, parent);
            return Ok(());
        };
        if handle == window.handle {
            // The v1: a window cannot own itself.
            return Err("Invalid or nonexistent owner or parent window.".to_string());
        }
        super::gui::the_owner_is_written(window, handle, parent);
        Ok(())
    }

    fn v1_gui_color(&mut self, window: &str, color: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let colour = super::gui::the_colour_of_the_v1(color)?;
        super::gui::the_background_is_written(window, colour);
        Ok(())
    }

    fn v1_gui_margin(&mut self, window: &str, x: i64, y: i64) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_margin_is_written(window, x, y);
        Ok(())
    }

    fn v1_gui_font(&mut self, window: &str, options: &str, name: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        match super::gui::the_font_of_the_v1_from(window.font.as_ref(), options, name) {
            Some(font) => super::gui::the_font_is_written(window, font),
            None => super::gui::the_font_is_reset(window),
        }
        Ok(())
    }

    fn v1_gui_menu_bar(
        &mut self,
        window: &str,
        name: &str,
        json: &str,
        menu: u64,
    ) -> Result<(), String> {
        let bar = self.the_menu_bar_is_made(json, menu)?;
        let Some(window) = self.state.guis.get_mut(window) else {
            return Err(format!("there is no window named {window}"));
        };
        super::gui::the_menu_bar_is_written(window, name, json, bar)
    }

    fn v1_gui_menu_bar_is_updated(
        &mut self,
        name: &str,
        json: &str,
        menu: u64,
    ) -> Result<(), String> {
        // The v1: the bar of the menu of a window is the menu of the
        // script, and the items that the script adds to the menu after
        // the bar is attached are in the bar of the window.
        //
        // The source of the v1 (`script_gui.cpp:1913-1919` of
        // v1.1.37.02) writes the bar of the windows that use the menu
        // again, when the menu changes.
        let names: Vec<String> = self
            .state
            .guis
            .iter()
            .filter(|(_, window)| window.menu_name.eq_ignore_ascii_case(name))
            .map(|(name, _)| name.clone())
            .collect();
        for window in names {
            let bar = self.the_menu_bar_is_made(json, menu)?;
            let Some(window) = self.state.guis.get_mut(&window) else {
                continue;
            };
            super::gui::the_menu_bar_is_written(window, name, json, bar)?;
        }
        Ok(())
    }


    fn v1_gui_submit(&mut self, window: &str) -> Result<Vec<(String, String)>, String> {
        let Some(window) = self.state.guis.get(window) else {
            return Err(format!("there is no window named {window}"));
        };
        let mut answer = Vec::new();
        for control in &window.controls {
            if control.name.is_empty() {
                continue;
            }
            answer.push((control.name.clone(), super::gui::the_text_of_the_v1(control)));
        }
        Ok(answer)
    }

    fn v1_lv_add(&mut self, number: u64, options: &str, cells: &[String]) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        super::listview::add(control, options, cells)
    }

    fn v1_lv_delete(&mut self, number: u64, row: i64) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::delete(control, row))
    }

    fn v1_lv_count(&mut self, number: u64, mode: &str) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        let mode = mode.trim().to_ascii_lowercase();
        Ok(match mode.as_str() {
            "s" | "selected" => super::listview::selected_count(control),
            "c" | "column" => super::listview::column_count(control),
            _ => super::listview::count(control),
        })
    }

    fn v1_lv_next(&mut self, number: u64, start: i64, mode: &str) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::next(control, start, mode))
    }

    fn v1_lv_text(&mut self, number: u64, row: i64, col: i64) -> Result<String, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::text(control, row, col))
    }

    fn v1_lv_modify(
        &mut self,
        number: u64,
        row: i64,
        options: &str,
        cells: &[String],
    ) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::modify(control, row, options, cells))
    }

    fn v1_lv_modifycol(
        &mut self,
        number: u64,
        col: i64,
        options: &str,
        title: Option<&str>,
    ) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::modify_col(control, col, options, title))
    }

    fn v1_lv_setimagelist(&mut self, number: u64, list: i64, large: bool) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::set_image_list(control, list, large))
    }

    fn v1_lv_firstvisible(&mut self, number: u64) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::listview::first_visible(control))
    }

    fn v1_il_create(&mut self, initial: i32, grow: i32, large: bool) -> Result<i64, String> {
        Ok(super::imagelist::create(initial, grow, large))
    }

    fn v1_il_add(&mut self, list: i64, source: &str, icon_number: i64) -> Result<i64, String> {
        Ok(super::imagelist::add(list, source, icon_number))
    }

    fn v1_il_destroy(&mut self, list: i64) -> Result<bool, String> {
        Ok(super::imagelist::destroy(list))
    }

    fn v1_tv_add(
        &mut self,
        number: u64,
        text: &str,
        parent: i64,
        options: &str,
    ) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        super::treeview::add(control, text, parent, options)
    }

    fn v1_tv_delete(&mut self, number: u64, item: i64) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::delete(control, item))
    }

    fn v1_tv_count(&mut self, number: u64) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::count(control))
    }

    fn v1_tv_text(&mut self, number: u64, item: i64) -> Result<String, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::text(control, item))
    }

    fn v1_tv_get(&mut self, number: u64, item: i64, attribute: &str) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::get(control, item, attribute))
    }

    fn v1_tv_child(&mut self, number: u64, parent: i64) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::relation(control, parent, "child"))
    }

    fn v1_tv_parent(&mut self, number: u64, item: i64) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::relation(control, item, "parent"))
    }

    fn v1_tv_next(&mut self, number: u64, item: i64, mode: &str) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::next(control, item, mode))
    }

    fn v1_tv_selection(&mut self, number: u64) -> Result<i64, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::selection(control))
    }

    fn v1_tv_modify(
        &mut self,
        number: u64,
        item: i64,
        options: &str,
        text: Option<&str>,
    ) -> Result<bool, String> {
        let control = self.the_v1_control(number)?;
        Ok(super::treeview::modify(control, item, options, text))
    }

    fn gui_flag(&mut self, number: u64, name: &str, setting: Option<bool>) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_flag_of_a_control(control, name, setting)
    }

    fn gui_cursor(&mut self, number: u64) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_place_of_the_caret_of_a_control(control)
    }

    fn gui_second_value(&mut self, number: u64, _setting: Option<&str>) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        Ok(super::gui::the_second_value_of_a_control(control)?.to_string())
    }

    fn gui_status_text(&mut self) -> Result<String, String> {
        Err(super::gui::THE_REASON_OF_THE_COMMON_CONTROLS.to_string())
    }

    fn gui_menu_bar(&mut self, json: &str) -> Result<(), String> {
        // The bar is made out of the items of the v1 and attached to the
        // window of the script. The callbacks of the items go to the
        // table the procedure of the window reads.
        let bar = super::menu::TheMenuBar::of_the_items(json)?;
        let Some(gui) = self.state.guis.get("") else {
            return Err("there is no window of the script to attach the menu bar to".to_string());
        };
        let answer = unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::SetMenu(gui.handle, bar.handle)
        };
        if answer == 0 {
            return Err("the machine did not attach the menu bar to the window.".to_string());
        }
        super::gui::the_bar_callbacks_are_set(gui.handle as isize, bar.items.clone());
        self.state.menu_bar = Some(bar);
        Ok(())
    }

    fn menu_create(&mut self) -> Result<u64, String> {
        // The numbers of the items of the menus are unique in the whole of
        // the script, and the machine keeps them under 65536: the number
        // of an item is the number of the command of the click on it.
        let menu = super::menu::TheMenu::new()?;
        self.state.menus.push(menu);
        Ok(self.state.menus.len() as u64)
    }

    fn menu_add(&mut self, menu: u64, name: &str, callback: &str) -> Result<(), String> {
        let id = self.state.the_number_of_a_menu_item();
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.add(id, name, callback)
    }

    fn menu_update(&mut self, menu: u64, name: &str, callback: &str) -> Result<(), String> {
        let place = (menu.max(1) - 1) as usize;
        let Some(m) = self.state.menus.get(place) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        // The item that is there keeps its number, and the item that is
        // not takes a new one. A separator has an empty name and is always
        // appended with a fresh non-zero ID.
        if !name.is_empty() && m.has_item(name) {
            let Some(m) = self.state.menus.get_mut(place) else {
                return Err(format!("there is no menu with the number {menu}"));
            };
            return m.update(0, name, callback);
        }
        let id = self.state.the_number_of_a_menu_item();
        let Some(m) = self.state.menus.get_mut(place) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.add(id, name, callback)
    }

    fn menu_delete_all(&mut self, menu: u64) -> Result<(), String> {
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        let free = m.delete_all()?;
        self.state.the_menu_items_are_free(free);
        // The menus that the items opened are dead: the machine takes
        // them away with the items. They are made again.
        self.the_dead_menus_are_made_again();
        Ok(())
    }

    fn menu_insert(&mut self, menu: u64, name: &str, callback: &str) -> Result<(), String> {
        let id = self.state.the_number_of_a_menu_item();
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.insert(id, name, callback)
    }

    fn menu_set_submenu(
        &mut self,
        menu: u64,
        item: &str,
        submenu: u64,
    ) -> Result<(), String> {
        let Some(child) = self.state.menus.get((submenu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {submenu}"));
        };
        // The item opens the popup of the menu: the machine draws a
        // popup menu as a dropdown, and a window menu as a thin bar.
        let handle = child.handle;
        let place = (submenu.max(1) - 1) as usize;
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.set_submenu(item, place, handle)
    }

    fn menu_set_icon(
        &mut self,
        menu: u64,
        item: &str,
        icon: &str,
        number: i64,
    ) -> Result<(), String> {
        // The v1: "If FileName is omitted, any custom icon already
        // associated with the menu item will be removed" (the CHM of the
        // v1.1.37.02, `Menu.htm`). The name of the file is empty: the
        // image goes away from the item.
        //
        // Measured on the binary of the v1.1.37.02: the
        // `Menu, MyMenu, Icon, Item` of a menu whose item carries an icon
        // takes the icon away from it.
        let bitmap = if icon.trim().is_empty() {
            std::ptr::null_mut()
        } else {
            super::pictures::the_bitmap_of_the_icon_of_a_file(icon, number)
                .ok_or_else(|| format!("the file has no icon: {icon}"))?
        };
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.set_icon(item, bitmap)
    }

    fn menu_handle(&mut self, menu: u64) -> Result<i64, String> {
        let Some(m) = self.state.menus.get((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        Ok(m.handle as i64)
    }

    fn menu_delete(&mut self, menu: u64, name: &str) -> Result<(), String> {
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        let free = m.delete(name)?;
        self.state.the_menu_items_are_free(free);
        // The menu that the item opened is dead: the machine takes it
        // away with the item. It is made again.
        self.the_dead_menus_are_made_again();
        Ok(())
    }

    fn menu_delete_the_menu(&mut self, menu: u64) -> Result<(), String> {
        // The v1: "As a precaution, don't allow a menu to be destroyed if
        // a window is using it as its menu bar. That might have bad
        // side-effects on some OSes, especially older ones" (the source
        // of the v1, `script_menu.cpp:1340-1345` of v1.1.37.02).
        //
        // Measured on the binary of the v1.1.37.02: the
        // `Menu, MyBar, Delete` of a bar that a window of the script
        // uses leaves the bar alone, and the menu that it opens alive.
        let place = (menu.max(1) - 1) as usize;
        let Some(held) = self.state.menus.get(place) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        let handle = held.handle;
        let used = self
            .state
            .guis
            .values()
            .any(|window| window.menu.as_ref().map(|bar| bar.handle) == Some(handle));
        if used {
            return Err("a window of the script is using the menu as its menu bar.".to_string());
        }
        let Some(m) = self.state.menus.get_mut(place) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        // The menu of the machine goes away, and the numbers of its items
        // go back to the machine: the numbers of the items are unique and
        // they stay under 65536.
        //
        // The menus that the items opened stay: they belong to the
        // script, and the v1 leaves them alone.
        //
        // Measured on the binary of the v1.1.37.02: the
        // `Menu, MyBar, Delete` of a bar that opens the menu `MyMenu`
        // leaves `MyMenu` alive, with its items.
        let free = m.destroy();
        self.state.the_menu_items_are_free(free);
        // The menus that the items opened are dead: the machine takes
        // them away with the menu. They are made again.
        self.the_dead_menus_are_made_again();
        Ok(())
    }

    fn menu_rename(&mut self, menu: u64, name: &str, new: &str) -> Result<(), String> {
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.rename(name, new)
    }

    fn menu_enable(&mut self, menu: u64, name: &str, on: bool) -> Result<(), String> {
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.enable(name, on)
    }

    fn menu_check(&mut self, menu: u64, name: &str, on: bool) -> Result<(), String> {
        let Some(m) = self.state.menus.get_mut((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.check(name, on)
    }

    fn menu_count(&mut self, menu: u64) -> Result<i64, String> {
        let Some(m) = self.state.menus.get((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        Ok(m.count())
    }

    fn menu_is_enabled(&mut self, menu: u64, name: &str) -> Result<bool, String> {
        let Some(m) = self.state.menus.get((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.is_enabled(name)
    }

    fn menu_is_checked(&mut self, menu: u64, name: &str) -> Result<bool, String> {
        let Some(m) = self.state.menus.get((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        m.is_checked(name)
    }

    fn menu_show(&mut self, menu: u64) -> Result<Option<String>, String> {
        // The popup is shown at the pointer: the window of the script is
        // the owner of it, and the thread of the session is the one that
        // shows it.
        let (x, y) = super::pointer::the_place_of_the_pointer()?;
        self.the_menu_is_shown(menu, x as i32, y as i32)
    }

    fn menu_show_at(&mut self, menu: u64, x: i32, y: i32) -> Result<Option<String>, String> {
        // The v1: the menu of the script is shown over the window of the
        // script, and the message that the machine sends when the menu
        // opens carries the handle of the menu that was shown.
        self.the_menu_is_shown(menu, x, y)
    }


    fn menu_attach(&mut self, menu: u64) -> Result<(), String> {
        let Some(m) = self.state.menus.get((menu.max(1) - 1) as usize) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        let Some(gui) = self.state.guis.get("") else {
            return Err("there is no window of the script to attach the menu to".to_string());
        };
        unsafe { m.attach(gui.handle) }
    }

    fn gui_option(&mut self, name: &str, on: bool) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_option_is_applied(window, name, on)
    }

    fn gui_control_state(&mut self, classnn: &str, what: &str, on: bool) -> Result<(), String> {
        // The control is the one of the window of the script itself: the
        // ClassNN gives its number, and the flag (enabled/visible) is
        // written.
        let number = self.gui_number_named(classnn)?;
        self.gui_flag(number as u64, what, Some(on))?;
        Ok(())
    }

    fn gui_events(&mut self) -> Result<Vec<String>, String> {
        let mut events = super::hotkeys::the_events(&self.state.hotkeys)?;
        let mut of_the_windows = super::gui::the_events_of_the_gui();
        // The v1: the window that is resized runs the label
        // `<name>GuiSize` of the script, with the handle of the
        // window, the kind of the change, and the size of the client
        // area of it.
        for event in of_the_windows.iter_mut() {
            let Some(rest) = event.strip_prefix("v1size:") else {
                continue;
            };
            let mut parts = rest.split(':');
            let Some(handle) = parts.next().and_then(|it| it.parse::<isize>().ok()) else {
                continue;
            };
            let kind = parts.next().unwrap_or("0");
            let Some((name, window)) = self
                .state
                .guis
                .iter()
                .find(|(_, window)| window.handle as isize == handle)
            else {
                continue;
            };
            let label = if name.is_empty() {
                "GuiSize".to_string()
            } else {
                format!("{name}GuiSize")
            };
            let (width, height) = super::gui::the_client_size_of(window);
            *event = format!(
                "v1size:{label}:{kind}:{width}:{height}:{handle}"
            );
        }
        events.extend(of_the_windows);
        Ok(events)
    }

    fn watch_a_message(&mut self, number: i64) -> Result<(), String> {
        // The window of the script has to exist to receive the message:
        // it is made in the thread of the script, and the thread of the
        // machine has to run for the procedures of the windows of the
        // session to find the state of it.
        self.the_thread_of_the_hotkeys_starts()?;
        self.the_window_of_the_script()?;
        super::hotkeys::watch_a_message(&self.state.hotkeys, number)
    }

    fn post_a_message(
        &mut self,
        window: i64,
        message: i64,
        wparam: i64,
        lparam: i64,
    ) -> Result<(), String> {
        let answer = unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::PostMessageW(
                window as windows_sys::Win32::Foundation::HWND,
                message as u32,
                wparam as usize,
                lparam as isize,
            )
        };
        if answer == 0 {
            return Err("Target window not found.".to_string());
        }
        Ok(())
    }

    fn send_a_message(
        &mut self,
        window: i64,
        message: i64,
        wparam: i64,
        lparam: i64,
    ) -> Result<i64, String> {
        let answer = unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW(
                window as windows_sys::Win32::Foundation::HWND,
                message as u32,
                wparam as usize,
                lparam as isize,
            )
        };
        Ok(answer as i64)
    }

    fn window_by_id(&mut self, id: i64) -> Option<ahk::desktop::WindowInfo> {
        super::toplevel::the_window_of_the_handle(id)
    }

    fn windows(&mut self) -> Result<Vec<ahk::desktop::WindowInfo>, String> {
        // The window that the compositor has on the screen is the
        // window that the script sees: the listing and the enumeration
        // use the same criterion of the rule of the visibility.
        let mut all = Vec::new();
        for mut window in super::toplevel::the_windows_of_the_machine()? {
            let handle = window.id as windows_sys::Win32::Foundation::HWND;
            if !unsafe { super::toplevel::the_window_is_seen_of_the_machine(handle) } {
                continue;
            }
            window.active = super::toplevel::the_window_is_active(handle);
            all.push(window);
        }
        Ok(all)
    }

    fn window_enabled(
        &mut self,
        window: &ahk::desktop::WindowInfo,
    ) -> Result<Option<bool>, String> {
        super::toplevel::the_window_is_enabled(window.id)
    }

    fn set_always_on_top(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        on: bool,
    ) -> Result<(), String> {
        super::toplevel::the_window_is_made_always_on_top(window.id, on)
    }

    fn set_window_enabled(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        on: bool,
    ) -> Result<(), String> {
        super::toplevel::the_window_is_made_enabled(window.id, on)
    }

    fn move_window(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
    ) -> Result<(), String> {
        super::toplevel::the_window_is_moved(window.id, x, y, width, height)
    }

    fn activate_window(
        &mut self,
        window: &ahk::desktop::WindowInfo,
    ) -> Result<(), String> {
        super::toplevel::the_window_is_activated(window.id)
    }

    fn set_window_title(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        title: &str,
    ) -> Result<(), String> {
        super::toplevel::the_title_of_the_window_is_written(window.id, title)
    }

    fn set_window_alpha(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        alpha: i64,
    ) -> Result<(), String> {
        let handle = window.id as windows_sys::Win32::Foundation::HWND;
        unsafe { super::styles::the_alpha_is_written(handle, alpha) }
    }

    fn set_window_region(
        &mut self,
        window: &ahk::desktop::WindowInfo,
        region: &ahk::desktop::WindowRegion,
    ) -> Result<(), String> {
        super::toplevel::the_region_is_given(window.id, region)
    }

    fn always_on_top(&mut self, window: &ahk::desktop::WindowInfo) -> Result<Option<bool>, String> {
        super::toplevel::the_window_is_always_on_top(window.id)
    }

    fn gui_selection(&mut self, number: u64) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_chosen_items_of_a_list(control)
    }

    fn the_window_at(&mut self, x: i64, y: i64) -> Result<Option<i64>, String> {
        super::toplevel::the_window_at(x, y)
    }

    fn the_window_under_the_pointer(&mut self, mode: i64) -> Result<(i64, i64), String> {
        let (x, y) = self.mouse_position()?;
        Ok(super::toplevel::the_window_and_the_control_under_the_pointer(
            x,
            y,
            mode & 0x01 != 0,
        ))
    }

    fn window_capabilities(&mut self) -> Vec<String> {
        vec![
            "the titles of the windows".to_string(),
            "the classes of the windows".to_string(),
            "the place of the windows".to_string(),
            "the process of a window".to_string(),
            "the window of the focus".to_string(),
            "the window at a place".to_string(),
            "the enabled of a window".to_string(),
            "the always-on-top of a window".to_string(),
        ]
    }

    fn gui_visibility(&mut self, classnn: &str, on: bool) -> Result<(), String> {
        let number = self.gui_number_named(classnn)?;
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number as u64) else {
            return Err(format!("there is no control named {classnn}"));
        };
        unsafe {
            ShowWindow(control.handle, if on { SW_SHOWNA } else { SW_HIDE });
        }
        Ok(())
    }

    fn control_text(&mut self, control: &str, title: &str) -> Result<ControlAnswer, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlAnswer::NoSuchWindow);
        };
        Ok(super::controls::control_text(window, control))
    }

    fn control_visible(&mut self, control: &str, title: &str) -> Result<ControlFlag, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlFlag::NoSuchWindow);
        };
        Ok(super::controls::control_visible(window, control))
    }

    fn control_enabled(&mut self, control: &str, title: &str) -> Result<ControlFlag, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlFlag::NoSuchWindow);
        };
        Ok(super::controls::control_enabled(window, control))
    }

    fn control_set_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ControlEdit, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlEdit::NoSuchWindow);
        };
        Ok(super::controls::control_set_text(window, control, text))
    }

    fn control_click(&mut self, control: &str, title: &str) -> Result<ControlEdit, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlEdit::NoSuchWindow);
        };
        Ok(super::controls::control_click(window, control))
    }

    fn control_focus(&mut self, control: &str, title: &str) -> Result<ControlEdit, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlEdit::NoSuchWindow);
        };
        Ok(super::controls::control_focus(window, control))
    }

    fn control_send_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ControlEdit, String> {
        let Some(window) = self.the_window_of(control, title)? else {
            return Ok(ControlEdit::NoSuchWindow);
        };
        Ok(super::controls::control_send_text(window, control, text))
    }
}

/// The files of the machine.
impl Files for WindowsDesktop {
    super::files::the_files_of_the_desktop!();
}
/// The clipboard of the machine.
impl Clipboard for WindowsDesktop {
    fn clipboard_read(&mut self) -> Result<String, String> {
        super::clipboard::clipboard_read()
    }

    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        super::clipboard::clipboard_write(text)
    }

    fn clipboard_read_all(&mut self) -> Result<Vec<u8>, String> {
        super::clipboard::clipboard_read_all()
    }

    fn clipboard_write_all(&mut self, data: &[u8]) -> Result<(), String> {
        super::clipboard::clipboard_write_all(data)
    }

    fn read_memory(&mut self, address: i64, size: i64) -> Result<Vec<u8>, String> {
        super::clipboard::read_the_memory(address, size)
    }
}
impl Timers for WindowsDesktop {
    fn the_menu_is_visible(&mut self) -> bool {
        super::gui::the_menu_is_visible()
    }
}
/// The hotkeys of the script.
///
/// The window that owns them is the one of a thread of the machine: the
/// thread pumps the messages of the session, and the hotkeys that fire
/// arrive in the window of it.
impl Hotkeys for WindowsDesktop {
    fn register_hotkey(&mut self, name: &str) -> Result<String, String> {
        self.the_thread_of_the_hotkeys_starts()?;
        super::hotkeys::register(&self.state.hotkeys, name)
    }

    fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {
        super::hotkeys::unregister(&self.state.hotkeys, name)
    }

    fn hotkeys(&mut self) -> Result<Vec<String>, String> {
        super::hotkeys::the_hotkeys(&self.state.hotkeys)
    }

    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {
        super::hotkeys::the_fired(&self.state.hotkeys)
    }

    /// The run waits for an event of the machine.
    ///
    /// The wait is the one of the keyboard: the thread that watches the
    /// keys tells when one was pressed, and the run does not ask at
    /// every instant.
    fn wait_for_event(&mut self, timeout: u64) -> Result<bool, String> {
        super::hotkeys::an_event_is_waited(&self.state.hotkeys, timeout)
    }
}
/// The native calls of the machine.
///
/// The loader of the libraries and the trampoline of the call are the
/// ones of the crate: they are the work of the machine, and not of the
/// desktop of it.
impl Native for WindowsDesktop {
    fn call_native(
        &mut self,
        call: &mut ahk::dll::NativeCall,
    ) -> Result<ahk::dll::NativeResult, ahk::dll::NativeFailure> {
        crate::native::call(&mut self.state.libraries, call)
    }

    fn call_native_at(
        &mut self,
        address: u64,
        call: &mut ahk::dll::NativeCall,
    ) -> Result<ahk::dll::NativeResult, ahk::dll::NativeFailure> {
        crate::native::call_at(address, call)
    }

    fn load_library(&mut self, name: &str) -> Result<(), String> {
        self.state
            .libraries
            .preload(name)
            .map_err(|failure| failure.reason)
    }

    fn create_callback(
        &mut self,
        func: &ahk::runtime::Value,
        param_count: usize,
    ) -> Result<u64, String> {
        let address = ahk::callback::register_value(func.clone(), param_count);
        if address == 0 {
            return Err("the callback pool is full.".to_string());
        }
        Ok(address as u64)
    }

    fn free_callback(&mut self, address: u64) {
        ahk::callback::free(address as usize);
    }

    /// The bytes of the memory of the machine at an address: the `StrGet`
    /// of the v1 reads the memory that a call of the machine filled.
    fn the_bytes_of_the_memory(&mut self, address: u64, length: usize) -> Option<Vec<u8>> {
        crate::native::memory::read(address, length)
    }

    /// Write bytes into the memory of the machine at an address: the
    /// `StrPut` of the v1 writes in the memory that a call of the machine
    /// allocated.
    fn the_memory_is_written(&mut self, address: u64, bytes: &[u8]) -> bool {
        crate::native::memory::write(address, bytes)
    }
}

/// The COM of the machine.
///
/// The COM is a thing of Windows, and this desktop is the Windows of the
/// machine: the classes are the ones of the registry of the machine, and
/// the objects are the ones that the machine makes.
impl Com for WindowsDesktop {
    fn com_object_of_the_class(
        &mut self,
        class: &str,
        interface: Option<&str>,
    ) -> Result<ComObject, String> {
        let object = super::com::the_object_of_the_class(class, interface)?;
        Ok(self.the_com_is_kept(object))
    }

    fn com_object_of_the_running(&mut self, class: &str) -> Result<ComObject, String> {
        let object = super::com::the_object_of_the_running(class)?;
        Ok(self.the_com_is_kept(object))
    }

    fn com_object_of_the_name(&mut self, name: &str) -> Result<ComObject, String> {
        let object = super::com::the_object_of_the_name(name)?;
        Ok(self.the_com_is_kept(object))
    }

    fn com_object_is_queried(
        &mut self,
        object: &ComObject,
        interface: &str,
    ) -> Result<ComObject, String> {
        let found = super::com::the_object_is_queried(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            interface,
        )?;
        Ok(self.the_com_is_kept(found))
    }

    fn com_type_of_the_object(&mut self, object: &ComObject, what: &str) -> Result<String, String> {
        super::com::the_type_of_the_object(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            what,
        )
    }

    fn com_value_is_made(&mut self, kind: i64, value: ComValue) -> Result<ComObject, String> {
        let made = super::com::the_value_is_made(kind, value)?;
        Ok(ComObject {
            pointer: made.the_pointer(),
            kind: made.the_kind(),
        })
    }

    fn com_pointer_is_counted(&mut self, pointer: u64) -> Result<u64, String> {
        super::com::the_pointer_is_counted(pointer)
    }

    fn com_pointer_is_released(&mut self, pointer: u64) -> Result<u64, String> {
        super::com::the_pointer_is_released(pointer)
    }

    fn com_default_is_written(
        &mut self,
        object: &ComObject,
        value: ComValue,
    ) -> Result<(), String> {
        super::com::the_default_is_written(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            value,
        )
    }

    fn com_method_is_called(
        &mut self,
        object: &ComObject,
        index: usize,
        arguments: &mut [ahk::dll::NativeArg],
        returns: ahk::dll::NativeType,
        failure_is_an_error: bool,
    ) -> Result<ahk::dll::NativeResult, String> {
        super::com::the_method_is_called(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            index,
            arguments,
            returns,
            failure_is_an_error,
        )
    }

    fn com_object_is_connected(&mut self, object: &ComObject, prefix: &str) -> Result<(), String> {
        let events = self.state.events.clone();
        super::com::the_object_is_connected(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            prefix,
            events,
        )
    }

    fn com_events_are_taken(&mut self, wait_ms: u64) -> Vec<ahk::desktop::ComEvent> {
        // The machine calls the script by the queue of the thread of the
        // apartment, and the queue only moves when the thread looks at
        // it.
        super::com::the_messages_are_pumped(wait_ms);
        let mut taken = Vec::new();
        if let Ok(mut events) = self.state.events.lock() {
            taken = events.drain(..).collect();
        }
        taken
            .into_iter()
            .map(|event| ahk::desktop::ComEvent {
                name: event.name,
                object: event.object,
                kind: event.kind,
                arguments: event.arguments,
            })
            .collect()
    }

    fn com_member(
        &mut self,
        object: &ComObject,
        what: ahk::desktop::MemberWhat,
        name: &str,
        arguments: &[ComValue],
    ) -> Result<ahk::desktop::ComAnswer, String> {
        let answer = super::com::the_member_of_the_object(
            &super::com::TheCom::of_the_pointer(object.pointer, object.kind),
            what,
            name,
            arguments,
        )?;
        // An object that the machine gave stays in the desktop, and the
        // desktop gives it back when the program ends.
        if let ahk::desktop::ComAnswer::Object { pointer, .. } = answer {
            if pointer != 0 {
                self.state.com.push(pointer as *mut core::ffi::c_void);
            }
        }
        Ok(answer)
    }
}

impl WindowsDesktop {
    /// The control of the script by its number, for the `LV_*` and the
    /// `TV_*` of the v1.
    ///
    /// The v1 numbers the controls of each of its windows from one, and
    /// the window that the list view and the tree view are in is the one
    /// that the last `Gui, Name: ListView` named.
    fn the_v1_control(&self, number: u64) -> Result<&super::gui::TheControl, String> {
        let Some(window) = self.state.guis.get(&self.state.window_of_the_lists) else {
            return Err("there is no window of the script".to_string());
        };
        super::gui::the_control(window, number)
            .ok_or_else(|| format!("there is no control with the number {number}"))
    }

    /// The bar of the menu of a window of the v1 is made.
    ///
    /// The bar is the menu of the machine when the machine has one: the
    /// items of the bar are the items of that menu, and the dropdowns of
    /// the bar are the menus that the script made.
    ///
    /// The bar that is made of the JSON of the script is the bar of a
    /// machine that cannot make a menu: its items open menus that it
    /// made itself, and the script does not know them.
    fn the_menu_bar_is_made(
        &mut self,
        json: &str,
        menu: u64,
    ) -> Result<super::menu::TheMenuBar, String> {
        if menu == 0 {
            return super::menu::TheMenuBar::of_the_items(json);
        }
        let place = (menu.max(1) - 1) as usize;
        if place >= self.state.menus.len() {
            return super::menu::TheMenuBar::of_the_items(json);
        }
        // The menu of the script is the bar of the window: the bar of a
        // window has to be a window menu, and the machine refuses a
        // popup menu as the bar of a window.
        //
        // The v1: "menu->Create(MENU_TYPE_BAR); SetMenu(gui.mHwnd, menu
        // ? menu->mMenu : NULL)" (`script_gui.cpp:458-461` of
        // v1.1.37.02). The menu of the script is turned into a window
        // menu in place, and the window keeps the handle of it: the
        // window that takes another bar does not destroy the menus of
        // the script.
        self.the_menu_becomes_a_bar(place);
        // The callbacks of every item of the bar and of the menus that it
        // opens: the click on an item of a dropdown is a command with
        // the number of the item, and the procedure of the window looks
        // the number up in the table of the callbacks.
        let mut items = Vec::new();
        self.the_callbacks_are_collected(place, &mut items, 0);
        let held = &self.state.menus[place];
        Ok(super::menu::TheMenuBar::of_the_menu(held, items))
    }

    /// The menu of the script becomes the bar of a window.
    ///
    /// The bar of a window has to be a window menu, and the machine
    /// refuses a popup menu as the bar of a window. The v1 turns the menu
    /// of the script into a window menu when the script attaches it to a
    /// window, and it keeps the handle of the menu in the window.
    ///
    /// The machine takes the menus that the popup opens with it: "This
    /// seems to indicate that submenus are destroyed whenever a parent
    /// menu is destroyed" (the source of the v1, `script_menu.cpp:1329`
    /// of v1.1.37.02). The menus that the popup opened are made again,
    /// and the items of the bar open them.
    fn the_menu_becomes_a_bar(&mut self, place: usize) {
        // The menu is a window menu from now on.
        self.state.menus[place].becomes_a_bar();
        // The menus that are dead are made again FIRST: they are the menus
        // that the machine took away with the items of the menu that is
        // gone, and the bar is written with the handles that its items
        // have NOW -- a bar that opens a menu that is dead opens nothing.
        self.the_dead_menus_are_made_again();
        // The menu of the machine is made again as a window menu: the
        // items go with `RemoveMenu`, and the menus that they open keep
        // their handle.
        //
        // Measured on the binary of AutoHotkey v1.1.37.02: the handle of
        // the menu that the bar opens does not change when the bar is
        // attached to the window, and the script holds that handle: the
        // map of the dropdowns of the script is keyed by it,
        // and the dropdown is built when it opens.
        self.the_menu_is_made_again(place);
    }

    /// The callbacks of the items of a menu and of the menus that it opens.
    ///
    /// The v1: the item of a dropdown of the bar runs the label of it, the
    /// way the item of the bar itself does.
    ///
    /// Measured on the binary of the v1.1.37.02: the click
    /// on an item of a dropdown of the bar of a script of the v1 runs
    /// its label.
    fn the_callbacks_are_collected(
        &self,
        menu: usize,
        out: &mut Vec<(usize, String)>,
        depth: usize,
    ) {
        // The menus of the script may open one another, and the search
        // does not go on for ever.
        if depth > 16 {
            return;
        }
        let Some(held) = self.state.menus.get(menu) else {
            return;
        };
        for item in held.items.iter() {
            if !item.callback.is_empty() {
                out.push((item.id, item.callback.clone()));
            }
            if let Some(child) = item.submenu {
                self.the_callbacks_are_collected(child, out, depth + 1);
            }
        }
    }

    /// The menu of the machine is shown at a place, and the callback of
    /// the item that was chosen is the answer.
    ///
    /// The item that was chosen may be an item of a menu that the menu
    /// opens: the number of the item is the number of the item of the
    /// menu of the machine, and the menus of the machine share the
    /// numbers of their items.
    ///
    /// The tracking of the menu happens in the thread of the script: the
    /// window of the script belongs to that thread, and the
    /// `TrackPopupMenu` of the machine belongs to the thread that owns
    /// the window of the owner. A call from another thread fails at once,
    /// and the menu never appears.
    fn the_menu_is_shown(
        &mut self,
        menu: u64,
        x: i32,
        y: i32,
    ) -> Result<Option<String>, String> {
        let place = (menu.max(1) - 1) as usize;
        let Some(m) = self.state.menus.get(place) else {
            return Err(format!("there is no menu with the number {menu}"));
        };
        // The popup of the menu: the machine shows a menu as a popup only
        // when the menu is a popup menu, and the menus of the script are
        // window menus, because the bar of a window has to be one.
        let popup = m.handle as usize;
        // The window of the script has to exist: it is the window of the
        // owner of the menu, and the owner has to belong to the thread
        // that shows the menu.
        self.the_window_of_the_script()?;
        let chosen = super::hotkeys::the_menu_is_shown(&self.state.hotkeys, popup, x, y)?;
        let Some(chosen) = chosen else {
            return Ok(None);
        };
        Ok(self.the_callback_of_the_item(place, chosen, 0))
    }

    /// The callback of the item of a number, in the menu of the machine
    /// and in the menus that it opens.
    fn the_callback_of_the_item(
        &self,
        menu: usize,
        id: usize,
        depth: usize,
    ) -> Option<String> {
        // The menus of the script may open one another, and the search
        // does not go on for ever.
        if depth > 16 {
            return None;
        }
        let held = self.state.menus.get(menu)?;
        if let Some(item) = held.items.iter().find(|item| item.id == id) {
            if item.callback.is_empty() {
                return None;
            }
            return Some(item.callback.clone());
        }
        for item in held.items.iter() {
            let Some(child) = item.submenu else { continue };
            if let Some(found) = self.the_callback_of_the_item(child, id, depth + 1) {
                return Some(found);
            }
        }
        None
    }

    /// The object of the COM is kept for the machine, and the machine
    /// gets it back when the program ends.
    fn the_com_is_kept(&mut self, object: super::com::TheCom) -> ComObject {
        self.state.the_com_is_kept(&object);
        ComObject {
            pointer: object.the_pointer(),
            kind: object.the_kind(),
        }
    }
}

/// The sound of the machine.
impl Sound for WindowsDesktop {
    fn sound_play(&mut self, path: &str) -> Result<String, String> {
        super::sound::a_file_is_played(path)
    }

    fn sound_device_names(&mut self) -> Result<(String, String), String> {
        super::audio::the_names_of_the_devices()
    }

    fn sound_device_named(&mut self, device: &str) -> Result<Option<String>, String> {
        super::audio::the_name_of_the_device_of_the_name(device)
    }

    fn sound_volume(&mut self, device: &str, component: &str) -> Result<f64, String> {
        super::audio::the_volume_of_the_component(device, component)
    }

    fn sound_set_volume(
        &mut self,
        device: &str,
        component: &str,
        volume: f64,
    ) -> Result<(), String> {
        super::audio::the_volume_of_the_component_is_set(device, component, volume)
    }

    fn sound_muted(&mut self, device: &str, component: &str) -> Result<bool, String> {
        super::audio::the_mute_of_the_component(device, component)
    }

    fn sound_set_muted(
        &mut self,
        device: &str,
        component: &str,
        muted: bool,
    ) -> Result<(), String> {
        super::audio::the_mute_of_the_component_is_set(device, component, muted)
    }

    fn sound_interface_of_the_device(
        &mut self,
        device: &str,
        interface: &str,
    ) -> Result<u64, String> {
        super::audio::the_interface_of_the_device(device, interface)
    }

    fn sound_control(
        &mut self,
        component: &str,
        control: &str,
        device: &str,
    ) -> Result<ahk::desktop::SoundControl, ahk::desktop::SoundFailure> {
        super::audio::the_control_of_a_component(component, control, device)
    }

    fn sound_control_is_set(
        &mut self,
        component: &str,
        control: &str,
        value: f64,
        device: &str,
    ) -> Result<(), ahk::desktop::SoundFailure> {
        super::audio::the_control_of_a_component_is_set(component, control, value, device)
    }

    fn sound_beep(&mut self, frequency: f64, duration_ms: u64) -> Result<String, String> {
        super::sound::sound_beep(frequency, duration_ms)
    }
}
/// The dialogs of the machine.
///
/// The box of the text is a window of the machine; the pickers of the
/// files are the shell of the machine, and they are the COM of it.
impl Dialogs for WindowsDesktop {
    fn input_box(
        &mut self,
        prompt: &str,
        title: &str,
        options: &ahk::desktop::InputOptions,
        default: &str,
    ) -> Result<ahk::desktop::InputAnswer, String> {
        super::dialogs::input_box(prompt, title, options, default)
    }

    fn file_select(
        &mut self,
        options: &ahk::desktop::FileOptions,
        root: &str,
        title: &str,
        filter: &str,
    ) -> Result<ahk::desktop::FileAnswer, String> {
        super::dialogs::file_select(options, root, title, filter)
    }

    fn dir_select(&mut self, starting: &str, options: i64, prompt: &str) -> Result<String, String> {
        super::dialogs::dir_select(starting, options, prompt)
    }

    fn notify(&mut self, title: &str, text: &str) -> Result<(), String> {
        let window = self.the_window_of_the_tray()?;
        super::trash::notify(window, title, text)
    }

    fn msg_box(&mut self, text: &str, title: &str, options: i64) -> Result<String, String> {
        super::dialogs::msg_box(text, title, options)
    }
}
/// The network of the machine.
impl Network for WindowsDesktop {
    fn ip_addresses(&mut self) -> Result<Vec<String>, String> {
        super::network::ip_addresses()
    }

    fn send_mail(&mut self, _message: &ahk::desktop::MailMessage) -> Result<(), String> {
        Err(
            "the mail is a thing of the network of the machine, and the tree does not carry it."
                .to_string(),
        )
    }

    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        super::network::download(url, file)
    }
}

/// The keyboard of the machine.
impl Keyboard for WindowsDesktop {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        super::keyboard::type_text(text)
    }

    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        super::keyboard::press_key(name, modifiers)
    }

    fn key_down(&mut self, name: &str) -> Result<String, String> {
        super::keyboard::key_down(name)
    }

    fn key_up(&mut self, name: &str) -> Result<String, String> {
        super::keyboard::key_up(name)
    }

    fn keyboard_layout_name(&mut self) -> Result<String, String> {
        super::keyboard::the_layout_of_the_keyboard()
    }

    fn key_in_layout(&mut self, vk: u32, name: &str, layout: &str) -> Result<(u32, u32), String> {
        super::keyboard::the_key_in_the_layout(vk, name, layout)
    }

    fn lock_light(&mut self, name: &str) -> Result<Option<bool>, String> {
        super::keyboard::the_light_of_a_lock(name)
    }

    fn set_lock(&mut self, lock: &str, on: bool) -> Result<(), String> {
        super::keyboard::set_lock_light(lock, on)?;
        Ok(())
    }

    fn key_is_down(&mut self, name: &str) -> Result<bool, String> {
        super::keyboard::key_is_down(name)
    }

    fn keys_down(&mut self) -> Result<Vec<String>, String> {
        super::keyboard::keys_down()
    }

    fn lock_is_on(&mut self, lock: &str) -> Result<bool, String> {
        super::keyboard::lock_is_on(lock)
    }

    fn set_lock_light(&mut self, name: &str, on: bool) -> Result<bool, String> {
        super::keyboard::set_lock_light(name, on)
    }
}

/// The pointer of the machine.
impl Pointer for WindowsDesktop {
    fn click(&mut self, x: i64, y: i64, button: &str, count: u32) -> Result<String, String> {
        super::pointer::click(x, y, button, count)
    }

    fn mouse_down(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        super::pointer::mouse_down(x, y, button)
    }

    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        super::pointer::mouse_up(x, y, button)
    }

    fn mouse_wheel(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        direction: &str,
        amount: u32,
    ) -> Result<String, String> {
        super::pointer::mouse_wheel(x, y, direction, amount)
    }

    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        super::pointer::move_pointer(x, y)
    }

    fn mouse_position(&mut self) -> Result<(i64, i64), String> {
        super::pointer::the_place_of_the_pointer()
    }

    fn set_mouse_speed(&mut self, speed: i64) -> Result<String, String> {
        // The setting is kept, the way the v2.0.31 keeps
        // `g->DefaultMouseSpeed`, and the machine answers: the `SendInput`
        // that the tree sends with ignores it, the way the v2's own
        // `MouseMove` says -- "Mouse speed (aSpeed) is ignored for
        // SendInput/Play mode: the mouse always moves instantaneously"
        // (`source/keyboard_mouse.cpp`).
        crate::pointer::set_speed(speed);
        Ok(format!("the speed of the pointer is {speed}"))
    }

    fn set_mouse_delay(&mut self, delay: i64) -> Result<String, String> {
        // The setting is kept, the way the v2.0.31 keeps `g->MouseDelay`,
        // and `DoMouseDelay()` of the same source does nothing in the
        // `SendInput` mode: the tree sends with `SendInput`.
        crate::pointer::set_delay(delay);
        Ok(format!("the delay of the pointer is {delay}ms"))
    }

    fn clip_cursor(&mut self, rectangle: Option<(i64, i64, i64, i64)>) -> Result<(), String> {
        super::pointer::clip_cursor(rectangle)
    }

    fn cursor_shape(&mut self) -> Result<String, String> {
        super::pointer::the_shape_of_the_cursor()
    }
}

/// The screen of the machine.
impl Screen for WindowsDesktop {
    fn capture(
        &mut self,
        x: i64,
        y: i64,
        width: i64,
        height: i64,
        filename: &str,
    ) -> Result<String, String> {
        super::screen::capture(x, y, width, height, filename)
    }

    fn pixel(&mut self, x: i64, y: i64) -> Result<String, String> {
        super::screen::pixel(x, y)
    }

    fn pixel_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        colour: &str,
    ) -> Result<Option<(i64, i64)>, String> {
        let Some(target) = super::pictures::the_colour_of(colour) else {
            return Err(format!("the machine has no colour named {colour}."));
        };
        let (left, top) = (x1.min(x2), y1.min(y2));
        let (right, bottom) = (x1.max(x2), y1.max(y2));
        let width = right - left + 1;
        let height = bottom - top + 1;
        if width <= 0 || height <= 0 {
            return Ok(None);
        }
        let picture = super::screen::the_picture_of(left, top, width, height)?;
        Ok(super::pictures::search_the_colour(&picture, target, 0)
            .map(|(x, y)| (x + left, y + top)))
    }

    fn gui_opacity(&mut self, setting: Option<i64>) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        match setting {
            // The transparency of the window of the script is the alpha of
            // a layered window: `SetLayeredWindowAttributes`.
            Some(alpha) => {
                unsafe { super::styles::the_alpha_is_written(window.handle, alpha)? };
                Ok(alpha)
            }
            None => unsafe { super::styles::the_alpha_of(window.handle) },
        }
    }

    fn window_move(&mut self, x: i64, y: i64, width: i64, height: i64) -> Result<(), String> {
        super::toplevel::the_window_of_the_focus_is_moved(x, y, width, height)
    }

    fn tooltip_show(&mut self, text: &str, x: i64, y: i64) -> Result<i64, String> {
        if text.is_empty() {
            // The tip goes away: the window of it is destroyed.
            if let Some(tip) = self.state.tip.take() {
                unsafe {
                    windows_sys::Win32::UI::WindowsAndMessaging::DestroyWindow(tip);
                }
            }
            return Ok(0);
        }
        // The tip is a layered window of the machine, and it lives while
        // the script shows it.
        let handle = match self.state.tip {
            Some(handle) => handle,
            None => {
                let handle = super::tooltip::the_tip_is_made()?;
                self.state.tip = Some(handle);
                handle
            }
        };
        unsafe { super::tooltip::the_tip_is_written(handle, text, x, y)? };
        // The number that the tip answers is the identity of it: the
        // handle of the window, as v2 answers.
        Ok(handle as i64)
    }

    fn screen_size(&mut self) -> Result<(i64, i64), String> {
        super::screen::screen_size()
    }
    fn gui_checked(&mut self, number: u64, setting: Option<bool>) -> Result<i64, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        match setting {
            Some(checked) => {
                super::gui::the_check_is_written(control, checked);
                Ok(if checked { 1 } else { 0 })
            }
            None => Ok(super::gui::the_check_of(control)),
        }
    }

    fn gui_paste(&mut self, number: u64, _text: &str) -> Result<(), String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let Some(control) = super::gui::the_control(window, number) else {
            return Err(format!("there is no control with the number {number}"));
        };
        super::gui::the_text_is_pasted(control);
        Ok(())
    }

    fn gui_controls(&mut self) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let mut names = Vec::new();
        for index in 0..window.controls.len() {
            names.push(super::gui::the_classnn_of(window, index as u64 + 1));
        }
        Ok(names.join(","))
    }

    fn gui_all_text(&mut self) -> Result<String, String> {
        let Some(window) = self.state.guis.get("") else {
            return Err("there is no window of the script".to_string());
        };
        let mut all = Vec::new();
        for control in &window.controls {
            all.push(super::gui::the_text_of(control));
        }
        Ok(all.join("\n"))
    }

    fn screen_dpi(&mut self) -> Result<i64, String> {
        super::screen::screen_dpi()
    }

    fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        super::screen::monitors()
    }

    fn monitor_work_area(
        &mut self,
        monitor: usize,
    ) -> Result<Option<(i64, i64, i64, i64)>, String> {
        super::screen::monitor_work_area(monitor)
    }

    fn primary_monitor(&mut self) -> Result<usize, String> {
        super::screen::primary_monitor()
    }

    fn system_metric(&mut self, metric: i32) -> Option<i64> {
        Some(super::screen::system_metric(metric))
    }
}

/// The pictures of the machine.
impl Pictures for WindowsDesktop {
    fn load_picture(
        &mut self,
        filename: &str,
        options: &PictureOptions,
    ) -> Result<Option<u32>, String> {
        // Measured on the binary of v2.0.31: a file that is not there gives
        // the handle zero, and not an error: the answer of LoadPicture is 0.
        if !std::path::Path::new(filename).exists() {
            return Ok(None);
        }
        let handle = super::pictures::load_picture(&mut self.state.pictures, filename, options)?;
        Ok(Some(handle))
    }

    fn image_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        image: &ahk::desktop::Image,
        options: &ahk::desktop::ImageOptions,
    ) -> Result<ahk::desktop::ImageSearch, String> {
        super::pictures::image_search(&self.state.pictures, x1, y1, x2, y2, image, options)
    }
}

/// The processes of the machine.
impl Processes for WindowsDesktop {
    fn run(&mut self, command: &str, window: &str) -> Result<(i32, String, String), String> {
        super::processes::run(command, window)
    }

    fn run_with_pid(
        &mut self,
        command: &str,
        window: &str,
    ) -> Result<(i32, String, String, u32), String> {
        super::processes::run_with_pid(command, window)
    }

    fn launch(&mut self, command: &str, working_dir: &str) -> Result<u32, String> {
        super::processes::launch(command, working_dir)
    }

    fn run_and_wait(
        &mut self,
        command: &str,
        working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        super::processes::run_and_wait(command, working_dir)
    }

    fn env_get(&mut self, name: &str) -> Result<Option<String>, String> {
        super::processes::env_get(name)
    }

    fn env_set(&mut self, name: &str, value: &str) -> Result<String, String> {
        super::processes::env_set(name, value)
    }

    fn process_exist(&mut self, which: Option<&str>) -> Result<u32, String> {
        super::processes::process_exist(which)
    }

    fn process_name(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        super::processes::process_name(which)
    }

    fn process_path(&mut self, which: Option<&str>) -> Result<Option<String>, String> {
        super::processes::process_path(which)
    }

    fn process_parent(&mut self, which: Option<&str>) -> Result<u32, String> {
        super::processes::process_parent(which)
    }

    fn process_close(&mut self, which: &str) -> Result<u32, String> {
        super::processes::process_close(which)
    }

    fn process_wait(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        super::processes::process_wait(which, timeout)
    }

    fn process_wait_close(&mut self, which: &str, timeout: Option<f64>) -> Result<u32, String> {
        super::processes::process_wait_close(which, timeout)
    }

    fn process_set_priority(
        &mut self,
        priority: &str,
        which: Option<&str>,
    ) -> Result<Option<u32>, String> {
        super::processes::process_set_priority(priority, which)
    }

    fn kill_a_process(&mut self, pid: u32) -> Result<(), String> {
        super::processes::kill_a_process(pid)
    }

    fn the_state_of_a_process(&mut self, pid: u32) -> Result<Option<ProcessState>, String> {
        super::processes::the_state_of_a_process(pid)
    }

    fn run_with_stdin(
        &mut self,
        program: &str,
        arguments: &[String],
        standard_input: &str,
        working_dir: &str,
    ) -> Result<(u32, i32, String, String), String> {
        super::processes::run_with_stdin(program, arguments, standard_input, working_dir)
    }

    fn launch_with_stdin(
        &mut self,
        program: &str,
        arguments: &[String],
        standard_input: &str,
        working_dir: &str,
    ) -> Result<u32, String> {
        super::processes::launch_with_stdin(program, arguments, standard_input, working_dir)
    }
}

/// The number that the text of the owner of a window of the v1 names.
///
/// The v1 writes the handle of a window of the machine with the "0x" in
/// front of it, and the script may write it as a decimal number too.
///
/// Measured on the binary of v1.1.37.02: the `+Owner0x...`
/// and the `+Owner<decimal>` of a window that exists both give the owner
/// to the window.
fn the_number_of_the_owner(text: &str) -> Option<i64> {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        return i64::from_str_radix(hex, 16).ok();
    }
    if let Some(digits) = text.strip_prefix('-') {
        return digits.parse::<i64>().ok().map(|number| -number);
    }
    if !text.is_empty() && text.chars().all(|character| character.is_ascii_digit()) {
        return text.parse::<i64>().ok();
    }
    None
}

/// The title of the script, the way the tray of the v1 shows it.
///
/// The v1 shows the name of the file of the script in the tip of the
/// icon of the tray.
///
/// Measured on the binary of the v1.1.37.02: the tip of the icon of a
/// script called `agentB-e9.ahk` is `agentB-e9.ahk`, and it is the name
/// of the file with the tip of the script after it when the script gave
/// one (`agentB-e8.ahk QQQ`). The title of the window of the v1 is
/// another thing (`agentB-e8.ahk - AutoHotkey v1.1.37.02`), and the icon
/// of the tray does not follow it.
///
/// The name of the script is the first argument of the runner that is
/// not a flag: the runner reads the file of the script the same way, and
/// the desktop of the machine has no other place where the name of the
/// script lives. A script that came on the standard input has no file,
/// and the tip is the name of the program of the tree.
fn the_title_of_the_script() -> String {
    use std::sync::OnceLock;
    static THE_TITLE: OnceLock<String> = OnceLock::new();
    THE_TITLE
        .get_or_init(|| {
            let file = std::env::args_os()
                .skip(1)
                .find(|argument| !argument.to_string_lossy().starts_with('-'))
                .map(std::path::PathBuf::from);
            match file {
                Some(path) => path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "ahk".to_string()),
                None => "ahk".to_string(),
            }
        })
        .clone()
}

/// The icon of the tray of the script goes with the run.
///
/// The v1 takes the icon of the tray away when the script ends, and the
/// program may go on after it: the server of the MCP runs one script
/// after another in the same process, and the icon of the script of the
/// last run must not be the icon of the next one.
impl Drop for WindowsDesktop {
    fn drop(&mut self) {
        // The window of the script is the one that owns the icon: the
        // `NIM_DELETE` points at it, and the shell takes the icon away.
        if let Some(window) = self.state.script_window {
            super::trash::remove_the_tray(window);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use windows_sys::Win32::Foundation::{HINSTANCE, HWND};
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, DestroyWindow, RegisterClassExW, SW_SHOW, WNDCLASSEXW,
        WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    };

    use crate::windows::sys::Wide;

    /// The window of the test, with a button inside it.
    ///
    /// The window has the title that the test looks for, and the button
    /// is a real control: the test writes its style and reads it again.
    /// Each test has its own title, because the machine finds the window
    /// by the title and the tests run at the same time.
    fn a_window_with_a_button(title: &str) -> (HWND, HWND) {
        unsafe {
            let module: HINSTANCE = GetModuleHandleW(std::ptr::null());
            let class = Wide::of("ahk-rs-test-desktop");
            let window_class = WNDCLASSEXW {
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: module,
                lpszClassName: class.as_ptr(),
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                ..std::mem::zeroed()
            };
            // The class is registered once: the second registration of
            // the same class is refused with the number `1410`, and the
            // window is made with the class that is already there.
            RegisterClassExW(&window_class);
            let title = Wide::of(title);
            let class = Wide::of("ahk-rs-test-desktop");
            let window = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                0,
                0,
                320,
                240,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                module,
                std::ptr::null(),
            );
            assert!(!window.is_null(), "the window of the test is there");
            // The window of the test is **visible**. Measured on the
            // binary of v2.0.31: with `DetectHiddenWindows`
            // off -- the state of the beginning -- a window that exists
            // and is not visible is not found by the title, and the
            // machine answers `Target window not found.`. The test of
            // the colour and the one of the style look for the window
            // by the title, and this is why the window of the test has to
            // be visible.
            ShowWindow(window, SW_SHOW);
            let button_class = Wide::of("BUTTON");
            let button_text = Wide::of("the button of the test");
            let button = CreateWindowExW(
                0,
                button_class.as_ptr(),
                button_text.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                10,
                10,
                120,
                30,
                window,
                std::ptr::null_mut(),
                module,
                std::ptr::null(),
            );
            assert!(!button.is_null(), "the button of the test is there");
            (window, button)
        }
    }

    #[test]
    fn o_estilo_de_uma_janela_e_lido_e_escrito() {
        let mut desktop = WindowsDesktop::new();
        let title = "the window of the test of the style";
        let (window, _button) = a_window_with_a_button(title);
        // Measured on the binary of v2.0.31: the style of
        // a **visible** top-level window is 349110272, and the one of the
        // same hidden window is 80674816. The difference is the
        // `WS_VISIBLE` bit, and nothing else: the machine puts the
        // `WS_CLIPSIBLINGS` on its own.
        let style = desktop.window_style(title, false).expect("the style is read");
        assert_eq!(style, 349110272);
        // The new style is the absolute style: the window stays without
        // the frame that the mouse resizes.
        let wanted = style & !(0x0004_0000);
        desktop
            .window_set_style(title, wanted, false)
            .expect("the style is written");
        assert_eq!(
            desktop.window_style(title, false).expect("the style is read"),
            wanted
        );
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn a_cor_de_uma_janela_e_escrita_e_lida_de_novo() {
        let mut desktop = WindowsDesktop::new();
        let title = "the window of the test of the colour";
        let (window, _button) = a_window_with_a_button(title);
        // A window that is not layered has no colour at all.
        assert_eq!(
            desktop.window_trans_color(title).expect("the colour is read"),
            String::new()
        );
        desktop
            .window_set_trans_color(title, "0xFF0000")
            .expect("a cor é escrita");
        assert_eq!(
            desktop.window_trans_color(title).expect("the colour is read"),
            "0xFF0000"
        );
        // A cor que some tira a cor-chave.
        desktop
            .window_set_trans_color(title, "")
            .expect("a cor é escrita");
        assert_eq!(
            desktop.window_trans_color(title).expect("the colour is read"),
            String::new()
        );
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn o_estilo_de_um_controle_e_lido_e_escrito() {
        let mut desktop = WindowsDesktop::new();
        let title = "the window of the test of the control";
        let (window, button) = a_window_with_a_button(title);
        let style = desktop
            .control_style("Button1", title, false)
            .expect("the style of the control is read");
        assert!(style != 0, "the button has a style");
        // The new style is the absolute style, and the control is a
        // control: the `SetWindowPos` of the top-level windows is not
        // called for it.
        let wanted = style | 0x0000_0080; // O BS_AUTOCHECKBOX.
        desktop
            .control_set_style("Button1", title, wanted, false)
            .expect("the style of the control is written");
        assert_eq!(
            desktop
                .control_style("Button1", title, false)
                .expect("the style of the control is read"),
            wanted
        );
        let _ = button;
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn um_controle_que_nao_existe_e_recusado() {
        let mut desktop = WindowsDesktop::new();
        let title = "the window of the test of the missing control";
        let (window, _button) = a_window_with_a_button(title);
        assert!(desktop.control_style("Edit9", title, false).is_err());
        assert!(desktop.control_set_style("Edit9", title, 0, false).is_err());
        unsafe { DestroyWindow(window) };
    }

    #[test]
    fn a_janela_do_script_tem_o_estilo_do_script() {
        let mut desktop = WindowsDesktop::new();
        let window = desktop.script_window().expect("a janela do script é feita");
        assert!(window != 0, "the window of the script has a number");
        let handle = window as HWND;
        // Measured on the binary of v2.0.31: the style of
        // the window of the script is 80674816 and the extended style is
        // 256 (the WS_EX_TOOLWINDOW).
        assert_eq!(
            unsafe {
                crate::windows::styles::the_style_of(handle, false).expect("the style is read")
            },
            80674816
        );
        assert_eq!(
            unsafe {
                crate::windows::styles::the_style_of(handle, true).expect("the style is read")
            },
            256
        );
        // The window of the script has no colour at all.
        assert_eq!(
            unsafe {
                crate::windows::styles::the_transparent_colour_of(handle).expect("the colour is read")
            },
            String::new()
        );
    }
}
