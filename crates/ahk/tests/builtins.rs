//! The built-ins, against a fake desktop.
//!
//! The cases are the language ones: what a built-in answers, and what it
//! says when it is given the wrong thing. The wording of the failures is
//! the wording of AutoHotkey v2, because that is what a script sees.

use ahk::desktop::{
    Clipboard, Desktop, Dialogs, ControlAnswer, ControlEdit, ControlFlag, FileAnswer, FileOptions,
    Files, Hotkeys, Image, ImageOptions, ImageSearch, InputAnswer, InputOptions, Keyboard, Machine,
    Native, Network, PictureOptions, Pictures, Pointer, Processes, Screen, Sound, Timers, WaitFor,
    WindowGeometry, WindowRef, Windows, DriveSpace, Monitor,
};
use ahk::dll::{NativeCall, NativeFailure, NativeResult, NativeType, NativeValue, encode_number};
use ahk::parse;
use ahk::runtime::{Report, Runtime, Value};
use std::time::Duration;

/// A desktop that records what it was asked to do.
#[derive(Debug, Default)]
struct Fake {
    calls: Vec<(String, String)>,
    /// The native call the script made, if it made one.
    native: Option<NativeCall>,
    /// The answer the fake gives to a native call.
    answer: Option<NativeResult>,
    /// The numbers the fake writes into the arguments that are the
    /// address of a variable, in the order the arguments were given.
    writes: Vec<f64>,
    /// The text on the clipboard of the fake.
    clipboard: String,
    /// The items of the one list-shaped control the fake carries, and the
    /// 1-based index of the item that is chosen (0 = nothing).
    combo_items: Vec<String>,
    combo_choice: i64,
    combo_popup: bool,
    /// The text of the one status bar the fake carries.
    status_text: String,
    /// The rows of the one list view the fake carries.
    list_rows: Vec<Vec<String>>,
    list_selected: i64,
    /// The items of the one tree view the fake carries: (id, text, parent).
    tree_items: Vec<(i64, String, i64)>,
    tree_next: i64,
    tree_selected: i64,
    /// Paths the fake has put in the trash.
    recycled: Vec<String>,
    /// The shortcut the fake read back: target, dir, args, desc, icon.
    shortcut: Option<(String, String, String, String, String)>,
    /// The number of the one UpDown the fake carries, shared with its buddy.
    updown_value: i64,
    /// The pages of the one Tab the fake carries, and the chosen page.
    tab_pages: i64,
    tab_chosen: i64,
    /// Whether the fake's control is a date control, and its timestamp string.
    date_control: bool,
    date_value: String,
    date_month_only: bool,
    /// The file path of the one Pic the fake carries, if it made one.
    pic_path: Option<String>,
    /// The primary and second text of the one Link or Hotkey the fake carries.
    text_control: bool,
    text_value: String,
    second_text: String,
    /// Whether the fake has a window of the script at all.
    gui_open: bool,
}

impl Desktop for Fake {}
impl Pictures for Fake {
    fn load_picture(
        &mut self,
        filename: &str,
        _options: &PictureOptions,
    ) -> Result<Option<u32>, String> {
        self.calls
            .push(("load-picture".into(), filename.to_string()));
        Ok(Some(7))
    }
    fn image_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        image: &Image,
        _options: &ImageOptions,
    ) -> Result<ImageSearch, String> {
        self.calls
            .push(("image-search".into(), format!("{x1},{y1},{x2},{y2}")));
        match image {
            Image::File(name) if name.is_empty() => Ok(ImageSearch::NotLoadable),
            _ => Ok(ImageSearch::Found(10, 20)),
        }
    }
}
impl Network for Fake {
    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {
        self.calls
            .push(("download".into(), format!("{url}|{file}")));
        Ok(file.to_string())
    }
}

impl Sound for Fake {
    fn sound_volume(&mut self, device: &str) -> Result<f64, String> {
        self.calls.push(("volume".into(), device.into()));
        Ok(50.0)
    }
    fn sound_set_volume(&mut self, device: &str, volume: f64) -> Result<(), String> {
        self.calls
            .push(("set-volume".into(), format!("{device} {volume}")));
        Ok(())
    }
    fn sound_muted(&mut self, device: &str) -> Result<bool, String> {
        self.calls.push(("mute".into(), device.into()));
        Ok(false)
    }
    fn sound_set_muted(&mut self, device: &str, muted: bool) -> Result<(), String> {
        self.calls
            .push(("set-mute".into(), format!("{device} {muted}")));
        Ok(())
    }
    fn sound_play(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("play".into(), path.into()));
        Ok(format!("played {path}"))
    }
    fn sound_beep(&mut self, frequency: f64, duration: u64) -> Result<String, String> {
        self.calls
            .push(("beep".into(), format!("{frequency} {duration}")));
        Ok("beeped".to_string())
    }
}
impl Machine for Fake {
    fn drive_space(&mut self, path: &str) -> Result<Option<DriveSpace>, String> {
        self.calls.push(("drive-space".into(), path.to_string()));
        // One drive at `/`, and every other path is a drive that is not there.
        Ok(if path == "/" {
            Some(DriveSpace {
                free: 1000,
                capacity: 4000,
                filesystem: "ext4".to_string(),
                kind: "Fixed".to_string(),
            })
        } else {
            None
        })
    }
}
impl Dialogs for Fake {
    fn input_box(
        &mut self,
        prompt: &str,
        title: &str,
        options: &InputOptions,
        default: &str,
    ) -> Result<InputAnswer, String> {
        self.calls.push((
            "input-box".into(),
            format!("{prompt}|{title}|{default}|{:?}", options.timeout),
        ));
        Ok(InputAnswer {
            value: "the answer".into(),
            result: "OK".into(),
        })
    }

    fn file_select(
        &mut self,
        options: &FileOptions,
        root: &str,
        title: &str,
        filter: &str,
    ) -> Result<FileAnswer, String> {
        self.calls
            .push(("file-select".into(), format!("{root}|{title}|{filter}")));
        if options.multiple {
            Ok(FileAnswer::Many(vec![
                "/tmp/one.txt".into(),
                "/tmp/two.txt".into(),
            ]))
        } else {
            Ok(FileAnswer::One("/tmp/chosen.txt".into()))
        }
    }

    fn dir_select(&mut self, starting: &str, options: i64, prompt: &str) -> Result<String, String> {
        self.calls.push((
            "dir-select".into(),
            format!("{starting}|{options}|{prompt}"),
        ));
        Ok("/tmp/chosen".into())
    }

    fn notify(&mut self, title: &str, text: &str) -> Result<(), String> {
        self.calls
            .push(("notify".into(), format!("{title}|{text}")));
        Ok(())
    }
}

impl Fake {
    /// The answer about a write to a control: the one control takes it, and
    /// every other question is one of the two failures.
    fn the_edit(&self, control: &str, title: &str) -> Result<ControlEdit, String> {
        Ok(if control == "the label" && title == "the window" {
            ControlEdit::Done
        } else if title == "the window" {
            ControlEdit::NoSuchControl
        } else {
            ControlEdit::NoSuchWindow
        })
    }

    /// The answer about a flag of a control: the one control is visible and
    /// enabled, and every other question is one of the two failures.
    fn the_flag(&self, control: &str, title: &str) -> Result<ControlFlag, String> {
        Ok(if control == "the label" && title == "the window" {
            ControlFlag::Yes
        } else if title == "the window" {
            ControlFlag::NoSuchControl
        } else {
            ControlFlag::NoSuchWindow
        })
    }
}

impl Windows for Fake {
    fn control_text(&mut self, control: &str, title: &str) -> Result<ControlAnswer, String> {
        self.calls
            .push(("control-text".into(), format!("{control}|{title}")));
        // One control, in one window, and nothing else.
        Ok(if control == "the label" && title == "the window" {
            ControlAnswer::Found("the text of the label".to_string())
        } else if title == "the window" {
            ControlAnswer::NoSuchControl
        } else {
            ControlAnswer::NoSuchWindow
        })
    }

    fn control_visible(&mut self, control: &str, title: &str) -> Result<ControlFlag, String> {
        self.calls
            .push(("control-visible".into(), format!("{control}|{title}")));
        self.the_flag(control, title)
    }

    fn control_enabled(&mut self, control: &str, title: &str) -> Result<ControlFlag, String> {
        self.calls
            .push(("control-enabled".into(), format!("{control}|{title}")));
        self.the_flag(control, title)
    }

    fn control_set_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ControlEdit, String> {
        self.calls
            .push(("control-set-text".into(), format!("{control}|{title}|{text}")));
        Ok(if control == "the label" && title == "the window" {
            ControlEdit::Done
        } else if title == "the window" {
            ControlEdit::NoSuchControl
        } else {
            ControlEdit::NoSuchWindow
        })
    }

    fn control_click(&mut self, control: &str, title: &str) -> Result<ControlEdit, String> {
        self.calls
            .push(("control-click".into(), format!("{control}|{title}")));
        self.the_edit(control, title)
    }

    fn control_focus(&mut self, control: &str, title: &str) -> Result<ControlEdit, String> {
        self.calls
            .push(("control-focus".into(), format!("{control}|{title}")));
        self.the_edit(control, title)
    }

    fn control_send_text(
        &mut self,
        control: &str,
        title: &str,
        text: &str,
    ) -> Result<ControlEdit, String> {
        self.calls
            .push(("control-send-text".into(), format!("{control}|{title}|{text}")));
        self.the_edit(control, title)
    }

    fn matching_windows(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Vec<WindowRef>, String> {
        self.calls.push(("windows".into(), title.to_string()));
        // Two windows, and a title that names one of them. The empty title is
        // both of them, which is the answer v2 gives.
        let first = WindowRef {
            id: "0x1".to_string(),
            class: "first".to_string(),
        };
        let second = WindowRef {
            id: "0x2".to_string(),
            class: "second".to_string(),
        };
        Ok(match title {
            "" => vec![first, second],
            "first" => vec![first],
            "second" => vec![second],
            _ => Vec::new(),
        })
    }

    fn window_geometry(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<WindowGeometry>, String> {
        self.calls.push(("geometry".into(), title.to_string()));
        Ok(match title {
            "first" => Some(WindowGeometry {
                x: 10,
                y: 20,
                width: 100,
                height: 200,
            }),
            "second" => Some(WindowGeometry {
                x: 30,
                y: 40,
                width: 300,
                height: 400,
            }),
            _ => None,
        })
    }

    fn window_min_max(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
    ) -> Result<Option<i64>, String> {
        self.calls.push(("min-max".into(), title.to_string()));
        Ok(match title {
            "first" => Some(0),
            "second" => Some(1),
            "minimized" => Some(-1),
            _ => None,
        })
    }

    fn window_wait(
        &mut self,
        title: &str,
        _mode: ahk::desktop::TitleMatch,
        what: WaitFor,
        timeout_ms: u64,
    ) -> Result<Option<String>, String> {
        self.calls.push((
            "wait-window".into(),
            format!("{title}|{what:?}|{timeout_ms}"),
        ));
        let present = title == "first" || title == "second";
        Ok(match what {
            WaitFor::Present => present.then(|| "0x1".to_string()),
            WaitFor::Active => present.then(|| "0x1".to_string()),
            // `first` has the focus, `second` and a window that is not there
            // do not.
            WaitFor::NotActive => match title {
                "first" => None,
                _ => Some(String::new()),
            },
            WaitFor::Close => {
                if present {
                    None
                } else {
                    Some(String::new())
                }
            }
        })
    }

    // The one list-shaped control of the fake. These are the methods the
    // ComboBox handlers of round 262 and the pop-up and status bar of round
    // 263 lean on, so their cases can run without a GTK helper.
    fn gui_is_open(&mut self) -> bool {
        self.gui_open
    }
    fn gui_add(&mut self, _handle: u64, kind: &str, text: &str) -> Result<(u64, String), String> {
        self.calls.push(("add".into(), kind.to_string()));
        let classnn = match kind.to_lowercase().as_str() {
            "treeview" => "SysTreeView321",
            "listview" => "SysListView321",
            "statusbar" => "msctls_statusbar321",
            "updown" => "msctls_updown321",
            "slider" => "msctls_trackbar321",
            "progress" => "msctls_progress321",
            "datetime" => "SysDateTimePick321",
            "monthcal" => "SysMonthCal321",
            "link" => "SysLink1",
            "hotkey" => "msctls_hotkey321",
            "pic" => "Static1",
            "tab" => "SysTabControl321",
            _ => "ComboBox1",
        };
        if matches!(kind.to_lowercase().as_str(), "link" | "hotkey") {
            self.text_control = true;
            self.text_value = text.to_string();
        }
        if kind.eq_ignore_ascii_case("pic") {
            // Measured on the binary of v2.0.19 on 2026-11-10: the Value of a
            // Pic is the file it shows, and a write changes nothing.
            self.pic_path = Some(text.to_string());
        }
        if kind.eq_ignore_ascii_case("tab") {
            // One page per item; the first page is chosen from the start.
            self.tab_pages = text
                .split('\n')
                .filter(|name| !name.is_empty())
                .count() as i64;
            self.tab_chosen = 1;
        }
        if matches!(
            kind.to_lowercase().as_str(),
            "datetime" | "monthcal"
        ) {
            // The timestamp is a STRING: fourteen digits for a DateTime, eight
            // for a MonthCal, padded with zeros. Measured on the binary of
            // v2.0.19 on 2026-11-10.
            let wanted = if kind.eq_ignore_ascii_case("monthcal") { 8 } else { 14 };
            let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            self.date_control = true;
            self.date_month_only = wanted == 8;
            self.date_value = if digits.is_empty() {
                "0".repeat(wanted)
            } else {
                let mut held = digits;
                held.truncate(wanted);
                while held.len() < wanted {
                    held.push('0');
                }
                held
            };
        }
        // The numeric controls start at the lower bound of their `Range`, or
        // at the number in the text for a Slider or Progress. Measured on the
        // binary of v2.0.19 on 2026-11-10: `Range1-10` starts at 1, and
        // `Range0-100` with a text of 50 starts at 50.
        if matches!(
            kind.to_lowercase().as_str(),
            "updown" | "slider" | "progress"
        ) {
            let mut low = 0;
            let mut start = None;
            for word in text.split_whitespace() {
                if word.to_lowercase().starts_with("range") {
                    low = word[5..]
                        .split('-')
                        .next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                } else if let Ok(number) = word.parse::<i64>() {
                    start = Some(number);
                }
            }
            self.updown_value = start.unwrap_or(low);
        }
        Ok((1, classnn.to_string()))
    }
    fn gui_value(&mut self, _number: u64, setting: Option<&str>) -> Result<String, String> {
        if self.tab_pages > 0 {
            // A Tab's Value is the chosen page; a page that is not there is
            // refused. Measured on the binary of v2.0.19 on 2026-11-10.
            if let Some(text) = setting {
                let page = text.parse().unwrap_or(0);
                if page < 1 || page > self.tab_pages {
                    return Err("Invalid value.".to_string());
                }
                self.tab_chosen = page;
            }
            return Ok(self.tab_chosen.to_string());
        }
        if self.text_control {
            if let Some(text) = setting {
                self.text_value = text.to_string();
            }
            return Ok(self.text_value.clone());
        }
        if self.date_control {
            if let Some(text) = setting {
                let wanted = if self.date_month_only { 8 } else { 14 };
                let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                let mut held = digits;
                held.truncate(wanted);
                while held.len() < wanted {
                    held.push('0');
                }
                self.date_value = held;
            }
            return Ok(self.date_value.clone());
        }
        if let Some(path) = &self.pic_path {
            // Writing the Value of a Pic changes nothing. Measured on the
            // binary of v2.0.19 on 2026-11-10.
            let _ = setting;
            return Ok(path.clone());
        }
        if let Some(text) = setting {
            self.updown_value = text.parse().unwrap_or(0);
        }
        Ok(self.updown_value.to_string())
    }
    fn gui_second_value(&mut self, _number: u64, setting: Option<&str>) -> Result<String, String> {
        if let Some(text) = setting {
            self.second_text = text.to_string();
        }
        Ok(self.second_text.clone())
    }
    fn gui_control_pos(&mut self, _number: u64) -> Result<(i64, i64, i64, i64), String> {
        Ok((10, 20, 100, 30))
    }
    fn gui_number_named(&mut self, classnn: &str) -> Result<i64, String> {
        self.calls.push(("resolve".into(), classnn.to_string()));
        if classnn.eq_ignore_ascii_case("ComboBox1")
            || classnn.eq_ignore_ascii_case("msctls_statusbar321")
            || classnn.eq_ignore_ascii_case("SysListView321")
            || classnn.eq_ignore_ascii_case("SysTreeView321")
            || classnn.eq_ignore_ascii_case("msctls_updown321")
            || classnn.eq_ignore_ascii_case("msctls_trackbar321")
            || classnn.eq_ignore_ascii_case("msctls_progress321")
            || classnn.eq_ignore_ascii_case("SysDateTimePick321")
            || classnn.eq_ignore_ascii_case("SysMonthCal321")
            || classnn.eq_ignore_ascii_case("SysLink1")
            || classnn.eq_ignore_ascii_case("msctls_hotkey321")
            || classnn.eq_ignore_ascii_case("Edit1")
        {
            Ok(1)
        } else {
            Err(format!("there is no control named {classnn}"))
        }
    }
    fn gui_combo_items(&mut self, _number: u64) -> Result<String, String> {
        Ok(self.combo_items.join("\n"))
    }
    fn gui_combo_add(&mut self, _number: u64, text: &str) -> Result<(), String> {
        self.combo_items.push(text.to_string());
        Ok(())
    }
    fn gui_combo_select(&mut self, _number: u64, wanted: &str, by_text: bool) -> Result<i64, String> {
        if by_text {
            let found = self
                .combo_items
                .iter()
                .position(|item| item == wanted)
                .map(|index| index as i64 + 1);
            self.combo_choice = found.unwrap_or(0);
            Ok(self.combo_choice)
        } else {
            let index: i64 = wanted.parse().unwrap_or(0);
            self.combo_choice = if (1..=self.combo_items.len() as i64).contains(&index) {
                index
            } else {
                0
            };
            Ok(self.combo_choice)
        }
    }
    fn gui_combo_choice(&mut self, _number: u64) -> Result<(i64, String), String> {
        let index = self.combo_choice;
        let text = if index >= 1 {
            self.combo_items
                .get(index as usize - 1)
                .cloned()
                .unwrap_or_default()
        } else {
            String::new()
        };
        Ok((index, text))
    }
    fn gui_combo_delete(&mut self, _number: u64, index: &str) -> Result<(), String> {
        let at: usize = index.parse().unwrap_or(0);
        if at >= 1 && at <= self.combo_items.len() {
            self.combo_items.remove(at - 1);
            if self.combo_choice >= at as i64 {
                self.combo_choice = 0;
            }
            Ok(())
        } else {
            Err(format!("there is no item at {index}"))
        }
    }
    fn gui_combo_find(&mut self, _number: u64, text: &str) -> Result<i64, String> {
        Ok(self
            .combo_items
            .iter()
            .position(|item| item == text)
            .map(|index| index as i64 + 1)
            .unwrap_or(0))
    }
    fn gui_combo_dropdown(&mut self, _number: u64, show: bool) -> Result<(), String> {
        self.combo_popup = show;
        Ok(())
    }
    fn gui_status_text(&mut self) -> Result<String, String> {
        Ok(self.status_text.clone())
    }
    fn gui_title(&mut self, _handle: u64, _setting: Option<&str>) -> Result<String, String> {
        Ok("the window".to_string())
    }
    fn gui_list_add(&mut self, _number: u64, cells: &[String]) -> Result<i64, String> {
        self.list_rows.push(cells.to_vec());
        Ok(self.list_rows.len() as i64)
    }
    fn gui_list_count(&mut self, _number: u64) -> Result<i64, String> {
        Ok(self.list_rows.len() as i64)
    }
    fn gui_list_text(&mut self, _number: u64, row: i64, col: i64) -> Result<String, String> {
        Ok(self
            .list_rows
            .get(row as usize - 1)
            .and_then(|cells| cells.get(col as usize - 1))
            .cloned()
            .unwrap_or_default())
    }
    fn gui_list_content(&mut self, _number: u64, selected: bool) -> Result<String, String> {
        let rows: Vec<String> = self
            .list_rows
            .iter()
            .enumerate()
            .filter(|(index, _)| !selected || self.list_selected == *index as i64 + 1)
            .map(|(_, cells)| cells.join("\t"))
            .collect();
        Ok(rows.join("\n"))
    }
    fn gui_list_modify(&mut self, _number: u64, row: i64, cells: &[String]) -> Result<i64, String> {
        if let Some(existing) = self.list_rows.get_mut(row as usize - 1) {
            for (index, text) in cells.iter().enumerate() {
                if index < existing.len() {
                    existing[index] = text.clone();
                }
            }
        }
        self.list_selected = row;
        Ok(1)
    }
    fn gui_list_next(&mut self, _number: u64, start: i64) -> Result<i64, String> {
        if start == 0 {
            return Ok(self.list_selected);
        }
        Ok(if start < self.list_rows.len() as i64 {
            start + 1
        } else {
            0
        })
    }
    fn gui_tree_add(&mut self, _number: u64, text: &str, parent: i64, _options: &str) -> Result<i64, String> {
        let id = self.tree_next;
        self.tree_next += 1;
        self.tree_items.push((id, text.to_string(), parent));
        Ok(id)
    }
    fn gui_tree_count(&mut self, _number: u64) -> Result<i64, String> {
        Ok(self.tree_items.len() as i64)
    }
    fn gui_tree_text(&mut self, _number: u64, item: i64) -> Result<String, String> {
        Ok(self
            .tree_items
            .iter()
            .find(|(id, _, _)| *id == item)
            .map(|(_, text, _)| text.clone())
            .unwrap_or_default())
    }
    fn gui_tree_relation(&mut self, _number: u64, item: i64, relation: &str) -> Result<i64, String> {
        let entry = self.tree_items.iter().find(|(id, _, _)| *id == item).cloned();
        let Some((_id, _text, parent)) = entry else {
            return Ok(0);
        };
        Ok(match relation {
            "parent" => parent,
            "child" => self
                .tree_items
                .iter()
                .find(|(_, _, held)| *held == item)
                .map(|(id, _, _)| *id)
                .unwrap_or(0),
            "next" => self
                .tree_items
                .iter()
                .filter(|(_, _, held)| *held == parent)
                .map(|(id, _, _)| *id)
                .find(|id| *id > item)
                .unwrap_or(0),
            "prev" => self
                .tree_items
                .iter()
                .rev()
                .find(|(id, _, held)| *held == parent && *id < item)
                .map(|(id, _, _)| *id)
                .unwrap_or(0),
            _ => 0,
        })
    }
    fn gui_tree_selection(&mut self, _number: u64) -> Result<i64, String> {
        Ok(self.tree_selected)
    }
    fn gui_tree_modify(&mut self, _number: u64, item: i64, _options: &str) -> Result<i64, String> {
        self.tree_selected = item;
        Ok(item)
    }
    fn gui_tree_delete(&mut self, _number: u64, item: i64) -> Result<i64, String> {
        self.tree_items.retain(|(id, _, _)| *id != item);
        Ok(1)
    }
}
impl Files for Fake {
    fn recycle_a_file(&mut self, path: &str) -> Result<(), String> {
        self.calls.push(("recycle".into(), path.to_string()));
        self.recycled.push(path.to_string());
        Ok(())
    }
    fn read_shortcut(
        &mut self,
        shortcut: &str,
    ) -> Result<(String, String, String, String, String), String> {
        self.calls.push(("read-shortcut".into(), shortcut.to_string()));
        let (target, dir, args, desc, icon) = self.shortcut.clone().ok_or_else(|| {
            format!("the shortcut {shortcut} could not be read")
        })?;
        Ok((target, dir, args, desc, icon))
    }
    fn set_working_directory(&mut self, directory: &str) -> Result<String, String> {
        self.calls.push(("working-dir".into(), directory.into()));
        Ok(format!("the working directory is now {directory}"))
    }
    fn attributes_of_the_pattern(
        &mut self,
        pattern: &str,
        mode: &str,
    ) -> Result<Vec<String>, String> {
        self.calls.push((format!("pattern {mode}"), pattern.into()));
        if mode.contains('D') {
            return Ok(if pattern == "/tmp" {
                vec!["D".to_string()]
            } else {
                Vec::new()
            });
        }
        Ok(if pattern.ends_with(".txt") {
            vec!["A".to_string()]
        } else {
            Vec::new()
        })
    }
    fn file_attributes(&mut self, path: &str) -> Result<Vec<String>, String> {
        Ok(if path.ends_with(".txt") {
            vec!["A".to_string()]
        } else {
            vec!["D".to_string()]
        })
    }
    fn read_file(&mut self, _path: &str) -> Result<Vec<String>, String> {
        Ok(vec!["one".to_string(), "two".to_string()])
    }
    fn read_the_bytes_of_a_file(&mut self, path: &str) -> Result<Vec<u8>, String> {
        self.calls.push(("bytes-of-file".into(), path.into()));
        Ok(b"one\r\ntwo\r\n".to_vec())
    }
    fn delete_file(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("delete-file".into(), path.into()));
        Ok(String::new())
    }
    fn create_dir(&mut self, path: &str) -> Result<String, String> {
        self.calls.push(("create-dir".into(), path.into()));
        Ok(String::new())
    }
    fn copy_file(&mut self, from: &str, to: &str, overwrite: bool) -> Result<String, String> {
        self.calls
            .push(("copy-file".into(), format!("{from}|{to}|{overwrite}")));
        // The machine says "Failed" when the file that is copied is not
        // there, and the number of the files that failed goes in the
        // `Extra` of the failure.
        Err("Failed".to_string())
    }
    fn append_file(&mut self, path: &str, bytes: &[u8], mark: &[u8]) -> Result<String, String> {
        self.calls.push((
            "append-file".into(),
            format!(
                "{path} {} {}",
                String::from_utf8_lossy(bytes),
                String::from_utf8_lossy(mark)
            ),
        ));
        Ok(bytes.len().to_string())
    }
}

impl Clipboard for Fake {
    fn clipboard_read(&mut self) -> Result<String, String> {
        self.calls.push(("clipboard-read".into(), String::new()));
        Ok(self.clipboard.clone())
    }
    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        self.calls.push(("clipboard-write".into(), text.into()));
        self.clipboard = text.to_string();
        Ok(text.to_string())
    }
}

impl Timers for Fake {}

impl Hotkeys for Fake {}

impl Keyboard for Fake {
    fn type_text(&mut self, text: &str) -> Result<String, String> {
        self.calls.push(("type".into(), text.into()));
        Ok(format!("typed {text:?}"))
    }
    fn press_key(&mut self, name: &str, modifiers: &[String]) -> Result<String, String> {
        let mut spelled = modifiers.join("+");
        if !spelled.is_empty() {
            spelled.push('+');
        }
        spelled.push_str(name);
        Ok(format!("pressed {spelled}"))
    }
}

impl Pointer for Fake {
    fn click(&mut self, x: i64, y: i64, _button: &str, _count: u32) -> Result<String, String> {
        self.calls.push(("click".into(), format!("{x} {y}")));
        Ok(format!("clicked ({x}, {y})"))
    }
    fn mouse_down(
        &mut self,
        x: Option<i64>,
        y: Option<i64>,
        button: &str,
    ) -> Result<String, String> {
        self.calls
            .push(("down".into(), format!("{x:?} {y:?} {button}")));
        Ok("down".to_string())
    }
    fn mouse_up(&mut self, x: Option<i64>, y: Option<i64>, button: &str) -> Result<String, String> {
        self.calls
            .push(("up".into(), format!("{x:?} {y:?} {button}")));
        Ok("up".to_string())
    }
    fn move_pointer(&mut self, x: i64, y: i64) -> Result<String, String> {
        self.calls.push(("move".into(), format!("{x} {y}")));
        Ok("moved".to_string())
    }
}

impl Screen for Fake {
    fn monitors(&mut self) -> Result<Vec<Monitor>, String> {
        self.calls.push(("monitors".into(), String::new()));
        // One monitor, and it is the one the bounds of every case are.
        Ok(vec![Monitor {
            name: "MON1".to_string(),
            left: 0,
            top: 0,
            right: 1920,
            bottom: 1080,
            scale: 1,
        }])
    }
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        filename: &str,
    ) -> Result<String, String> {
        Ok(format!("/tmp/{filename}"))
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Processes for Fake {
    fn run(&mut self, command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, format!("ran {command}\n"), String::new()))
    }
    fn env_get(&mut self, name: &str) -> Result<Option<String>, String> {
        Ok(Some(format!("the value of {name}")))
    }
    fn env_set(&mut self, _name: &str, value: &str) -> Result<String, String> {
        Ok(value.to_string())
    }
    /// The process 42 is the one the fake knows; the name of it is `ahk`, and
    /// `ahk` names it too. Everything else is a process that is not there,
    /// which is what each of the six cases is written from.
    fn process_exist(&mut self, pid_or_name: &str) -> Result<u32, String> {
        self.calls
            .push(("process-exist".into(), pid_or_name.to_string()));
        if pid_or_name.is_empty() {
            // The process of the script itself.
            return Ok(7);
        }
        Ok(if pid_or_name == "42" || pid_or_name == "ahk" {
            42
        } else {
            0
        })
    }
    fn process_name(&mut self, pid_or_name: &str) -> Result<Option<String>, String> {
        self.calls
            .push(("process-name".into(), pid_or_name.to_string()));
        Ok(if pid_or_name == "42" || pid_or_name == "ahk" {
            Some("ahk".to_string())
        } else {
            None
        })
    }
    fn process_path(&mut self, pid_or_name: &str) -> Result<Option<String>, String> {
        self.calls
            .push(("process-path".into(), pid_or_name.to_string()));
        Ok(if pid_or_name == "42" {
            Some("/bin/ahk".to_string())
        } else {
            None
        })
    }
    fn process_parent(&mut self, pid_or_name: &str) -> Result<u32, String> {
        self.calls
            .push(("process-parent".into(), pid_or_name.to_string()));
        Ok(if pid_or_name == "42" { 1 } else { 0 })
    }
    fn process_close(&mut self, pid_or_name: &str) -> Result<u32, String> {
        self.calls
            .push(("process-close".into(), pid_or_name.to_string()));
        Ok(if pid_or_name == "42" || pid_or_name == "ahk" {
            42
        } else {
            0
        })
    }
    fn process_set_priority(
        &mut self,
        priority: &str,
        pid_or_name: &str,
    ) -> Result<u32, String> {
        self.calls
            .push(("process-priority".into(), format!("{priority}|{pid_or_name}")));
        // The process that is not there is the failure of v2; the word of the
        // priority is not checked, which is what the binary of v2.0.19 does.
        if pid_or_name == "42" {
            Ok(42)
        } else {
            Err("Target process not found.".to_string())
        }
    }
    fn process_wait(&mut self, pid_or_name: &str, timeout_ms: u64) -> Result<u32, String> {
        self.calls
            .push(("process-wait".into(), format!("{pid_or_name}|{timeout_ms}")));
        Ok(if pid_or_name == "42" || pid_or_name == "ahk" {
            42
        } else {
            0
        })
    }
    fn process_wait_close(&mut self, pid_or_name: &str, timeout_ms: u64) -> Result<u32, String> {
        self.calls.push((
            "process-wait-close".into(),
            format!("{pid_or_name}|{timeout_ms}"),
        ));
        // The measured table: the process that is not there and the one that
        // closed both answer the zero, and the one that outlived the timeout
        // answers its own id.
        Ok(if pid_or_name == "42" || pid_or_name == "ahk" {
            42
        } else {
            0
        })
    }
}

impl Native for Fake {
    fn call_native(&mut self, call: &mut NativeCall) -> Result<NativeResult, NativeFailure> {
        self.native = Some(call.clone());
        // The bytes the callee left in the memory of an address come
        // back into the argument: the argument is the way back.
        let mut writes = self.writes.iter();
        for argument in call.arguments.iter_mut() {
            if !argument.address {
                continue;
            }
            let Some(number) = writes.next() else {
                continue;
            };
            argument.value = NativeValue::Bytes(encode_number(*number, argument.kind));
        }
        Ok(self.answer.clone().unwrap_or(NativeResult {
            kind: call.returns,
            value: NativeValue::Integer(0),
        }))
    }
}

/// A desktop that cannot make a native call.
///
/// It writes the capability, but not the method: the default of the
/// trait says that this desktop cannot do it.
#[derive(Debug, Default)]
struct Blind;

impl Desktop for Blind {}
impl Pictures for Blind {}
impl Network for Blind {}
impl Sound for Blind {}
impl Machine for Blind {}
impl Dialogs for Blind {}

impl Windows for Blind {}

impl Files for Blind {}

impl Clipboard for Blind {}

impl Timers for Blind {}

impl Hotkeys for Blind {}

impl Native for Blind {}

impl Keyboard for Blind {
    fn type_text(&mut self, _text: &str) -> Result<String, String> {
        Ok(String::new())
    }
    fn press_key(&mut self, _name: &str, _modifiers: &[String]) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Pointer for Blind {
    fn click(&mut self, _x: i64, _y: i64, _button: &str, _count: u32) -> Result<String, String> {
        Ok(String::new())
    }
}

impl Screen for Blind {
    fn capture(
        &mut self,
        _x: i64,
        _y: i64,
        _width: i64,
        _height: i64,
        _filename: &str,
    ) -> Result<String, String> {
        Ok(String::new())
    }
    fn pixel(&mut self, _x: i64, _y: i64) -> Result<String, String> {
        Ok("#000000".into())
    }
}

impl Processes for Blind {
    fn run(&mut self, _command: &str, _window: &str) -> Result<(i32, String, String), String> {
        Ok((0, String::new(), String::new()))
    }
}

/// Run a script and answer with its report.
fn run(source: &str) -> Report {
    let mut desktop = Fake::default();
    run_on(source, &mut desktop)
}

/// Run a script against a desktop and answer with its report.
///
/// The desktop is the one of the caller, so that a case can look at
/// what the call carried.
fn run_on(source: &str, desktop: &mut Fake) -> Report {
    let statements = parse(source).expect("parses");
    Runtime::new(desktop, Duration::from_secs(10)).run(&statements)
}

/// The value of a variable, as the script left it.
fn value_of(report: &Report, name: &str) -> Value {
    report
        .variables
        .get(name)
        .cloned()
        .unwrap_or_else(|| panic!("{name} has no value"))
}

/// The value of a variable, as a whole number.
fn int_of(report: &Report, name: &str) -> i64 {
    match value_of(report, name) {
        Value::Int(number) => number,
        other => panic!("{name} is {other:?} and not a whole number"),
    }
}

/// The value of a variable, as a float.
fn float_of(report: &Report, name: &str) -> f64 {
    match value_of(report, name) {
        Value::Float(number) => number,
        other => panic!("{name} is {other:?} and not a float"),
    }
}

/// The message of the failure, when the script failed.
fn failure_of(report: &Report) -> String {
    assert!(!report.ok, "the script was expected to fail");
    report.error.clone().expect("the failure has a message")
}

#[test]
fn the_volume_of_a_device_is_a_number_between_zero_and_a_hundred() {
    let report = run("volume := SoundGetVolume()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(float_of(&report, "volume"), 50.0);
}

#[test]
fn the_volume_of_a_device_can_be_set() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume 30", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("set-volume".to_string(), " 30".to_string())]
    );
}

#[test]
fn a_sign_in_front_of_the_volume_adjusts_it() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume \"+10\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The volume of the device is asked first, and the sign adds ten to
    // it: 50 becomes 60.
    assert_eq!(
        desktop.calls,
        vec![
            ("volume".to_string(), String::new()),
            ("set-volume".to_string(), " 60".to_string())
        ]
    );
}

#[test]
fn a_volume_that_is_not_a_percentage_fails_the_script() {
    let report = run("SoundSetVolume \"loud\"");
    let failure = failure_of(&report);
    assert!(
        failure.contains("percentage between -100 and 100"),
        "{failure}"
    );
}

#[test]
fn the_mute_of_a_device_is_a_number() {
    let report = run("muted := SoundGetMute()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "muted"), 0);
}

#[test]
fn a_device_can_be_muted_and_unmuted() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetMute 1", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("set-mute".to_string(), " true".to_string())]
    );
}

#[test]
fn a_minus_sign_in_front_of_the_volume_adjusts_it_down() {
    let mut desktop = Fake::default();
    let report = run_on("SoundSetVolume \"-10\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The volume of the device is 50, and the sign takes ten from it.
    assert_eq!(
        desktop.calls,
        vec![
            ("volume".to_string(), String::new()),
            ("set-volume".to_string(), " 40".to_string())
        ]
    );
}

#[test]
fn a_component_that_the_box_does_not_have_fails_the_script() {
    let report = run("SoundGetVolume(\"Microphone\")");
    let failure = failure_of(&report);
    assert!(failure.contains("one volume per device"), "{failure}");
}

#[test]
fn a_tone_is_a_frequency_and_a_duration() {
    let mut desktop = Fake::default();
    let report = run_on("SoundBeep 750, 500", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("beep".to_string(), "750 500".to_string())]
    );
}

#[test]
fn a_tone_has_the_defaults_of_v2() {
    let mut desktop = Fake::default();
    let report = run_on("SoundBeep", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    // The frequency of v2 is 523 and its duration is 150 milliseconds.
    assert_eq!(
        desktop.calls,
        vec![("beep".to_string(), "523 150".to_string())]
    );
}

#[test]
fn a_frequency_that_is_not_a_tone_fails_the_script() {
    let report = run("SoundBeep 1");
    let failure = failure_of(&report);
    assert!(failure.contains("between 37 and 32767"), "{failure}");
}

#[test]
fn a_sound_file_is_played() {
    let mut desktop = Fake::default();
    let report = run_on("SoundPlay \"/tmp/a.wav\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("play".to_string(), "/tmp/a.wav".to_string())]
    );
}

#[test]
fn a_sound_that_the_desktop_cannot_answer_fails_the_script() {
    let statements = parse("volume := SoundGetVolume()").expect("parses");
    let mut desktop = Blind;
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    let failure = failure_of(&report);
    assert!(failure.contains("cannot"), "{failure}");
}

#[test]
fn the_names_of_the_sound_that_are_windows_fail_with_the_reason() {
    // `SoundGetName` answered the blanket reason until round 211 gave it one of its
    // own: the name of an audio device belongs to the system that runs it, and this
    // runtime answers the volume and the mute of the default device, which
    // `SoundGetMute` shows, and not the names the system gives them.
    let report = run("SoundGetName()");
    assert_eq!(
        failure_of(&report),
        "the name of an audio device belongs to the system that runs it, and this runtime answers the volume and the mute of the default device and not the names the system gives them"
    );
}

#[test]
fn a_string_in_title_case_has_the_first_letter_of_each_word_up() {
    let report = run("a := StrTitle(\"This is a test.\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("This Is A Test.".into())
    );
}

#[test]
fn the_title_case_lowercases_the_rest_of_the_word() {
    let report = run("a := StrTitle(\"GONE with the WIND\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("Gone With The Wind".into())
    );
}

#[test]
fn the_versions_are_compared_by_their_numbers() {
    let report = run("a := VerCompare(\"1.20.0\", \"1.3\")");
    assert!(report.ok, "{:?}", report.error);
    // The numbers are compared one by one, and twenty is bigger than three.
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_version_with_a_pre_release_is_older_than_the_same_version() {
    let report =
        run("a := VerCompare(\"2.0-a137\", \"2.0\")\nb := VerCompare(\"2.0-a137\", \"2.0-a136\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), -1);
    assert_eq!(int_of(&report, "b"), 1);
}

#[test]
fn the_second_version_may_carry_an_operator() {
    let report = run("a := VerCompare(\"2.0.1\", \">=2.0\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_label_is_not_there() {
    // The tree has no labels: there is nothing to jump to.
    let report = run("a := IsLabel(\"there\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 0);
}

#[test]
fn the_working_directory_of_the_script_can_be_changed() {
    let mut desktop = Fake::default();
    let report = run_on("SetWorkingDir \"there\"", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("working-dir".to_string(), "there".to_string())]
    );
}

#[test]
fn the_click_of_v2_takes_the_button_first() {
    let mut desktop = Fake::default();
    let report = run_on("MouseClick \"Right\", 10, 20", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("click".to_string(), "10 20".to_string())]
    );
}

#[test]
fn a_drag_goes_down_in_one_place_and_comes_up_in_another() {
    let mut desktop = Fake::default();
    let report = run_on("MouseClickDrag \"Left\", 10, 20, 30, 40", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![
            ("down".to_string(), "Some(10) Some(20) left".to_string()),
            ("move".to_string(), "30 40".to_string()),
            ("up".to_string(), "None None left".to_string()),
        ]
    );
}

#[test]
fn a_name_of_windows_fails_with_the_reason() {
    // `StrGet` used to be the example of the blanket reason. On 2026-11-10 it
    // was given a reason of its own: the reference hangs on it, so there is no
    // answer for this runtime to match. `RegRead` is a name of Windows in the
    // plain sense, and it still says so.
    let report = run("RegRead(\"HKLM\", \"Software\")");
    assert_eq!(
        failure_of(&report),
        "the registry is a thing of Windows, and this runtime does not carry it."
    );
}

#[test]
fn a_name_of_windows_without_a_counterpart_says_so() {
    // The group reason was corrected on 2026-11-10: a re-measurement found the
    // reference does NOT hang on these names -- an earlier note had mistaken
    // one call shape for the name. The true obstacle is that they ask for
    // Win32 resources this runtime has no counterpart for. `StrGet` was the
    // example until the same round answered it; `CallbackFree` is still in
    // the group.
    let report = run("CallbackFree(1)");
    assert_eq!(
        failure_of(&report),
        "these names ask for things of Windows that this runtime has no counterpart for: a caret's screen place, a Win32 shortcut, a COM audio interface, a string at a raw address, or a native callback thunk. Re-measured on the binary of v2.0.19 on 2026-11-10: it does NOT hang on them -- an earlier note mistook one call shape for the name."
    );
}

#[test]
fn a_hotstring_that_v2_would_refuse_fails_with_the_reason_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-28: the name of a
    // hotstring is written between two of the same character, and the
    // name is checked before the action.
    let report = run("Hotstring(\"a\", \"b\")");
    assert_eq!(failure_of(&report), "Parameter #1 of Hotstring is invalid.");
}

#[test]
fn the_date_is_answered_the_way_v2_answers_it() {
    // The date of v2 is a number of fourteen digits, and the unit comes
    // last: DateAdd("20260101", 1, "d") is the second of January.
    let report = run("a := DateAdd(\"20260101\", 1, \"d\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("20260102000000".into()));
}

#[test]
fn a_unit_that_v2_does_not_have_fails_with_the_reason_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the unit is read by its
    // first letter, so "days" works, and "years" and "weeks" are refused. The
    // old case used "days" and expected it to be refused: that was the defect.
    let report = run("DateAdd(\"20260101\", 1, \"years\")");
    assert_eq!(failure_of(&report), "Parameter #3 of DateAdd is invalid.");
}

#[test]
fn the_unit_of_a_date_is_read_by_its_first_letter() {
    // Measured on the binary of v2.0.19 on 2026-11-09: "Days" and "d" answer
    // the same thing, and "Months" is read as minutes because its first letter
    // is the one of minutes.
    let report = run(
        "a := DateAdd(\"20260101000000\", 11, \"Days\")\n\
         b := DateAdd(\"20260101000000\", 11, \"d\")\n\
         c := DateAdd(\"20260101000000\", 11, \"Months\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("20260112000000".into())
    );
    assert_eq!(
        value_of(&report, "b"),
        Value::Text("20260112000000".into())
    );
    assert_eq!(
        value_of(&report, "c"),
        Value::Text("20260101001100".into())
    );
}

#[test]
fn the_environment_is_asked_of_the_desktop() {
    let report = run("a := EnvGet(\"HOME\")\nb := EnvSet(\"HOME\", \"there\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("the value of HOME".into())
    );
    assert_eq!(value_of(&report, "b"), Value::Text("there".into()));
}

#[test]
fn the_files_are_asked_of_the_desktop() {
    let report = run(concat!(
        "a := FileExist(\"a.txt\")",
        "\nb := FileExist(\"a.md\")",
        "\nc := DirExist(\"/tmp\")",
        "\nd := DirExist(\"/nowhere\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    // v2 answers with the attributes and not with a number.
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("D".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("".into()));
}

#[test]
fn the_pattern_of_the_files_is_asked_of_the_desktop() {
    let mut desktop = Fake::default();
    let report = run_on(
        "a := FileExist(\"*.txt\")\nb := DirExist(\"/tmp\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    // v2 answers with the attributes of the first entry that matches, and
    // a folder is asked for with the mode "D".
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("D".into()));
    assert_eq!(
        desktop.calls,
        vec![
            ("pattern ".to_string(), "*.txt".to_string()),
            ("pattern D".to_string(), "/tmp".to_string()),
        ]
    );
}

#[test]
fn the_content_of_a_file_is_the_text_of_it() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer is the
    // text of the file and not an array of its lines.
    let report = run("text := FileRead(\"a.txt\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "text"),
        Value::Text("one\r\ntwo\r\n".into())
    );
}

#[test]
fn the_read_of_a_file_takes_the_options_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the second
    // parameter is the options of the read: the size of the part that is
    // read, the raw bytes and the linefeed.
    let report = run(
        "part := FileRead(\"a.txt\", \"m5\")\nsize := StrLen(part)\nraw := FileRead(\"a.txt\", \"RAW\")\nbytes := raw.Size\nlines := FileRead(\"a.txt\", \"`n\")\ncount := StrLen(lines)\nMsgBox \"%size% %bytes% %count%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["5 10 8"]);
}

#[test]
fn an_option_of_the_read_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the message is
    // "Invalid option.", the class of the failure is a ValueError, and the
    // option that was not known is in the Extra.
    let report = run(
        "try {\n    FileRead(\"a.txt\", \"bogus\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | FileRead | bogus"]
    );
}

#[test]
fn the_write_of_a_file_takes_the_options_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the third parameter
    // is the options of the write: the linefeed puts a carriage return
    // before every linefeed that has none.
    let mut desktop = Fake::default();
    let report = run_on("FileAppend(\"a`nb\", \"a.txt\", \"`n\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("append-file".to_string(), "a.txt a\r\nb ".to_string())]
    );
}

#[test]
fn an_option_of_the_write_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the message is
    // "Invalid option.", the class of the failure is a ValueError, and the
    // option that was not known is in the Extra.
    let report = run(
        "try {\n    FileAppend(\"a\", \"a.txt\", \"m5000\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | FileAppend | m5000"]
    );
}

#[test]
fn a_text_is_cut_at_the_delimiters() {
    let report = run("pieces := StrSplit(\"a,b,c\", \",\")\n\
         kind := Type(pieces)\nlength := pieces.Length\nfirst := pieces[1]");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-31: the answer of
    // `StrSplit` is an array of v2, and not a list of the tree.
    assert_eq!(value_of(&report, "kind"), Value::Text("Array".into()));
    assert_eq!(int_of(&report, "length"), 3);
    assert_eq!(value_of(&report, "first"), Value::Text("a".into()));
}

#[test]
fn the_delimiters_of_a_text_are_literals() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text is cut
    // where a delimiter is, and a delimiter is a literal and not a set of
    // characters.
    let report = run("one := StrSplit(\"aXXb\", \"XX\")\n\
         two := StrSplit(\"a.b.c\", \"..\")\n\
         first := one[1] . \",\" . one[2]\n\
         second := two[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b".into()));
    assert_eq!(value_of(&report, "second"), Value::Text("a.b.c".into()));
}

#[test]
fn the_delimiters_of_a_text_may_be_an_array() {
    let report = run("pieces := StrSplit(\"a.b-c\", [\".\", \"-\"])\n\
         first := pieces[1] . \",\" . pieces[2] . \",\" . pieces[3]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("a,b,c".into()));
}

#[test]
fn the_characters_to_omit_are_cut_from_the_pieces() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the characters to
    // omit are cut from the beginning and the end of every piece.
    let report = run("pieces := StrSplit(\"a,b,c\", \",\", \"ac\")\n\
         first := \"[\" pieces[1] \"]\" . \"[\" pieces[2] \"]\" . \"[\" pieces[3] \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("[][b][]".into()));
}

#[test]
fn the_number_of_the_pieces_says_how_many_times_the_text_is_cut() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the text is cut a
    // maximum of MaxParts-1 times, and the rest of the text is the last
    // piece.
    let report = run("two := StrSplit(\"a,b,c\", \",\", \"\", 2)\n\
         one := StrSplit(\"a,b,c\", \",\", \"\", 1)\n\
         none := StrSplit(\"a,b,c\", \",\", \"\", 0)\n\
         first := two[1] . \",\" . two[2] . \" \" . one[1] . \" \" . none.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "first"),
        Value::Text("a,b,c a,b,c 0".into())
    );
}

#[test]
fn a_blank_delimiter_makes_a_piece_of_every_character() {
    // Measured on the binary of v2.0.19 on 2026-10-31: when the
    // delimiters are blank every character of the text is a piece, and
    // the characters to omit are not pieces at all.
    let report = run("pieces := StrSplit(\"a b\", \"\", \" \")\n\
         first := pieces[1] . pieces[2]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "first"), Value::Text("ab".into()));
}

#[test]
fn a_text_that_is_empty_has_no_piece() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a text that is empty
    // has no piece at all, and the piece that is empty is not a piece.
    let report = run("pieces := StrSplit(\"\", \",\")\ncount := pieces.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "count"), 0);
}

#[test]
fn the_delimiters_that_are_not_a_text_are_refused() {
    let report = run("pieces := StrSplit(\"a,b\", Map())");
    assert_eq!(failure_of(&report), "Parameter #2 of StrSplit is invalid.");
}

#[test]
fn a_path_is_cut_into_its_pieces() {
    // v2 writes the pieces into the variables of the caller, and it
    // answers with nothing.
    let report = run(concat!(
        "SplitPath \"C:\\Users\\<user>\\file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("file.txt|C:\\Users\\<user>|txt|file|C:".into())
    );
}

#[test]
fn a_path_with_a_drive_and_no_backslash_keeps_the_drive_out_of_the_name() {
    // Measured on the binary of v2.0.19: the drive is cut off the path
    // before the name is, and the slash is not a separator of a file.
    let report = run(concat!(
        "SplitPath \"C:/Users/<user>/file.txt\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("/Users/<user>/file.txt|C:|txt|/Users/<user>/file|C:".into())
    );
}

#[test]
fn a_path_of_a_url_is_cut_at_the_slash() {
    let report = run(concat!(
        "SplitPath \"https://domain.com/images/pic.png\", &name, &dir, &ext, &noext, &drive\n",
        "pieces := name . \"|\" . dir . \"|\" . ext . \"|\" . noext . \"|\" . drive"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "pieces"),
        Value::Text("pic.png|https://domain.com/images|png|pic|https://domain.com".into())
    );
}

#[test]
fn a_path_that_wants_an_output_variable_says_so() {
    let report = run("SplitPath \"C:\\a.txt\", 5");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of SplitPath requires a variable reference, but received an Integer."
    );
}

#[test]
fn an_object_can_be_asked_about() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the name of the
    // class is a name of the script, and the variable may not have it.
    let report = run(concat!(
        "class Box {\n  x := 1\n}\n",
        "thing := Box()",
        "\na := HasProp(thing, \"x\")",
        "\nb := HasProp(thing, \"y\")",
        "\nc := HasBase(thing, Box.Prototype)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 0);
    assert_eq!(int_of(&report, "c"), 1);
}

#[test]
fn is_set_ref_says_whether_the_variable_has_a_value() {
    let report = run(concat!(
        "x := 1",
        "\nempty := \"\"",
        "\na := IsSetRef(&x)",
        "\nb := IsSetRef(&empty)",
        "\nc := IsSetRef(&nothere)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 1);
    // A variable that was never given a value is not set.
    assert_eq!(int_of(&report, "c"), 0);
}

#[test]
fn is_set_ref_wants_a_reference() {
    let report = run("a := IsSetRef(1)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of IsSetRef requires a variable reference, but received an Integer."
    );
}

#[test]
fn the_commands_of_the_settings_answer_with_the_value_that_was_there() {
    // Measured on the binary of v2.0.19 on 2026-10-31: every one of them
    // answers the value that was there before. The script sets the mode of
    // the titles first, so the answer does not depend on what another case
    // left behind, and it puts the mode back the way v2 starts it.
    let report = run(
        "SetTitleMatchMode(3)\na := SetTitleMatchMode(1)\nb := A_TitleMatchMode\nSetTitleMatchMode(\"RegEx\")\nc := SetTitleMatchMode(2)\nd := A_TitleMatchMode\ne := SetTitleMatchMode(\"Slow\")\nf := SetTitleMatchMode(\"Fast\")\ng := DetectHiddenWindows(true)\nh := DetectHiddenText(false)\ni := SetWinDelay(200)\nSetTitleMatchMode(2)\nMsgBox \"%a% %b% %c% %d% %e% %f% %g% %h% %i%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["3 1 RegEx 2 Fast Slow 0 1 100"]);
}

#[test]
fn the_commands_of_the_delay_of_the_controls_and_of_the_view_of_the_registry_answer_with_the_value_that_was_there()
 {
    // Measured on the binary of v2.0.19 on 2026-10-31: every one of them
    // answers the value that was there before, and the view of the
    // registry is always a text.
    let report = run("a := SetControlDelay(50)\nb := SetRegView(32)\nMsgBox \"%a% %b%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["20 Default"]);
}

#[test]
fn a_delay_of_the_controls_that_is_not_a_number_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay is a
    // number, and a text that holds no number is refused.
    let report = run(
        "try {\n    SetControlDelay(\"x\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "Parameter #1 of SetControlDelay requires a Number, but received a String. | TypeError | SetControlDelay | x"
        ]
    );
}

#[test]
fn a_delay_of_the_controls_that_is_out_of_the_range_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the delay goes from
    // -1 to the biggest number of thirty-two bits.
    let report = run(
        "try {\n    SetControlDelay(-2)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetControlDelay is invalid. | ValueError | SetControlDelay | -2"]
    );
}

#[test]
fn a_view_of_the_registry_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the view is 32, 64
    // or the word "Default".
    let report = run(
        "try {\n    SetRegView(33)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetRegView is invalid. | ValueError | SetRegView | 33"]
    );
}

#[test]
fn a_word_that_is_neither_the_mode_nor_the_speed_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode is a
    // number from 1 to 3, the word "RegEx", or the speed -- "Fast" or
    // "Slow".
    let report = run(
        "try {\n    SetTitleMatchMode(\"Bogus\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | Bogus"
        ]
    );
}

#[test]
fn a_mode_of_the_titles_that_is_not_one_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the mode is a number
    // from 1 to 3, and the others are refused.
    let report = run(
        "try {\n    SetTitleMatchMode(5)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of SetTitleMatchMode is invalid. | ValueError | SetTitleMatchMode | 5"]
    );
}

#[test]
fn the_mode_of_the_keys_is_the_one_of_the_box() {
    let report = run("SendMode \"Input\"");
    assert!(report.ok, "{:?}", report.error);
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("keyboard of the box")),
        "{:?}",
        report.notes
    );
    let report = run("SendMode \"Event\"");
    assert_eq!(
        failure_of(&report),
        "the keys of this runtime go through the keyboard of the box, and that is the only mode there is."
    );
}

#[test]
fn a_timer_runs_between_the_statements() {
    let report = run(concat!(
        "SetTimer(\"Tick\", 1)",
        "\nTick() {\n  Send \"tick\"\n}",
        "\nSleep 30",
    ));
    assert!(report.ok, "{:?}", report.error);
    let typed: Vec<&String> = report
        .steps
        .iter()
        .filter(|step| step.detail.contains("tick"))
        .map(|step| &step.detail)
        .collect();
    assert!(
        !typed.is_empty(),
        "the timer did not run: {:?}",
        report.steps
    );
}

#[test]
fn the_trigonometry_answers_a_float() {
    let report = run(concat!(
        "a := Sin(0)",
        "\nb := Cos(0)",
        "\nc := Tan(0)",
        "\nd := ASin(0)",
        "\ne := ACos(1)",
        "\nf := ATan(0)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Float(0.0));
    assert_eq!(value_of(&report, "b"), Value::Float(1.0));
    assert_eq!(value_of(&report, "c"), Value::Float(0.0));
    assert_eq!(value_of(&report, "d"), Value::Float(0.0));
    assert_eq!(value_of(&report, "e"), Value::Float(0.0));
    assert_eq!(value_of(&report, "f"), Value::Float(0.0));
}

#[test]
fn the_trigonometry_wants_a_number() {
    let report = run("a := Sin(\"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Sin requires a Number, but received a String."
    );
}

#[test]
fn the_inverse_trigonometry_fails_outside_the_unit_range() {
    let report = run("a := ACos(2)");
    assert_eq!(failure_of(&report), "Parameter #1 of ACos is invalid.");
}

#[test]
fn the_exponential_and_the_logarithms_answer_a_float() {
    let report = run("a := Exp(0)\nb := Ln(1)\nc := Log(10)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Float(1.0));
    assert_eq!(value_of(&report, "b"), Value::Float(0.0));
    let logarithm = float_of(&report, "c");
    assert!((logarithm - 1.0).abs() < 1e-12, "Log(10) is {logarithm}");
}

#[test]
fn the_logarithm_of_a_negative_number_fails() {
    let report = run("a := Ln(-1)");
    assert_eq!(failure_of(&report), "Parameter #1 of Ln is invalid.");
}

#[test]
fn a_number_that_is_not_a_number_fails_the_way_v2_does() {
    let report = run("a := Abs(\"abc\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received a String."
    );
}

#[test]
fn an_empty_text_is_named_in_the_message() {
    let report = run("a := Abs(\"\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received an empty string."
    );
}

#[test]
fn the_kind_of_a_value_is_named_in_the_message() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the kind of a value
    // is the name of its class.
    for (source, kind) in [
        ("a := Abs({})", "an Object"),
        ("a := Abs([1])", "an Array"),
        ("a := Abs(Map())", "a Map"),
        ("a := Abs(() => 1)", "a Func"),
    ] {
        let report = run(source);
        assert_eq!(
            failure_of(&report),
            format!("Parameter #1 of Abs requires a Number, but received {kind}."),
            "{source}"
        );
    }
}

#[test]
fn the_value_of_a_failure_is_empty_when_it_has_no_text() {
    let report =
        run("try {\n    x := Abs({})\n} catch Any as e {\n    MsgBox \"[\" e.Extra \"]\"\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn every_maths_built_in_names_the_type_it_wanted() {
    for (source, name) in [
        ("a := Sqrt(\"x\")", "Sqrt"),
        ("a := Floor(\"x\")", "Floor"),
        ("a := Ceil(\"x\")", "Ceil"),
        ("a := Round(\"x\")", "Round"),
        ("a := Min(\"x\", 2)", "Min"),
        ("a := Max(\"x\", 2)", "Max"),
        ("a := Sin(\"x\")", "Sin"),
        ("a := Cos(\"x\")", "Cos"),
        ("a := Tan(\"x\")", "Tan"),
        ("a := ASin(\"x\")", "ASin"),
        ("a := ACos(\"x\")", "ACos"),
        ("a := ATan(\"x\")", "ATan"),
        ("a := Exp(\"x\")", "Exp"),
        ("a := Ln(\"x\")", "Ln"),
        ("a := Log(\"x\")", "Log"),
    ] {
        let report = run(source);
        assert_eq!(
            failure_of(&report),
            format!("Parameter #1 of {name} requires a Number, but received a String."),
            "{source}"
        );
    }
}

#[test]
fn the_digits_of_a_round_are_a_number_too() {
    let report = run("a := Round(5, \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Round requires a Number, but received a String."
    );
}

#[test]
fn the_modulus_says_that_the_parameters_are_invalid() {
    let report = run("a := Mod(\"x\", 2)");
    assert_eq!(failure_of(&report), "Invalid parameter(s).");
}

#[test]
fn a_text_that_holds_a_number_is_a_number() {
    let report = run("a := Abs(\"3\")\nb := Abs(\"3.5\")\nc := Mod(\"5\", 2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Float(3.5));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
}

#[test]
fn a_reference_is_not_a_number() {
    let report = run("x := 1\na := Abs(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Abs requires a Number, but received a VarRef."
    );
}

#[test]
fn a_reference_is_not_a_text_either() {
    let report = run("x := 1\na := StrLen(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrLen requires a String, but received a VarRef."
    );
}

#[test]
fn a_reference_is_not_an_object_either() {
    let report = run("x := 1\na := ObjOwnProps(&x)");
    assert_eq!(failure_of(&report), "Expected an Object but got a VarRef.");
}

#[test]
fn the_base_of_an_object_is_the_prototype_of_its_class() {
    let report =
        run("class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\nx := ObjGetBase(b)");
    assert!(report.ok, "{:?}", report.error);
    match value_of(&report, "x") {
        Value::Object(object) => assert_eq!(object.borrow().class, "prototype"),
        other => panic!("the base is {other:?}"),
    }
}

#[test]
fn a_script_may_set_the_base_of_an_object() {
    let report = run(
        "class Alpha {\n}\nclass Beta extends Alpha {\n}\nb := Beta()\n\
         old := ObjSetBase(b, Alpha())\nnow := ObjGetBase(b)",
    );
    assert!(report.ok, "{:?}", report.error);
    // An object of a script has no base of its own until it is given one.
    assert_eq!(value_of(&report, "old"), Value::Text(String::new()));
    match value_of(&report, "now") {
        Value::Object(object) => assert_eq!(object.borrow().class, "alpha"),
        other => panic!("the base is {other:?}"),
    }
}

#[test]
fn the_own_properties_of_an_object_are_its_own() {
    let report = run("class Box {\n  x := 1\n}\nb := Box()\n\
         names := ObjOwnProps(b)\ncount := ObjOwnPropCount(b)\nhas := ObjHasOwnProp(b, \"x\")\n\
         kind := Type(names)\n\
         first := \"\"\nnames.Call(&first)\n\
         after := \"\"\nnames.Call(&after)");
    assert!(report.ok, "{:?}", report.error);
    // Measured on the binary of v2.0.19 on 2026-10-31: the own properties
    // of v2 are an enumerator, and it gives the name when the caller
    // wants one value.
    assert_eq!(value_of(&report, "kind"), Value::Text("Enumerator".into()));
    assert_eq!(value_of(&report, "first"), Value::Text("x".into()));
    assert_eq!(value_of(&report, "after"), Value::Text("".into()));
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(int_of(&report, "has"), 1);
}

#[test]
fn the_state_of_a_map_is_not_a_property_of_it() {
    let report = run(
        "m := Map()\nnames := ObjOwnProps(m)\nkind := Type(names)\ncount := ObjOwnPropCount(m)",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "kind"), Value::Text("Enumerator".into()));
    assert_eq!(int_of(&report, "count"), 0);
}

#[test]
fn the_name_that_counts_the_references_is_not_one_of_a_script() {
    let report = run("class Box {\n}\nb := Box()\na := ObjAddRef(b)");
    assert_eq!(failure_of(&report), "Parameter #1 of ObjAddRef is invalid.");
}

#[test]
fn a_plain_value_is_not_a_descriptor() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the second
    // parameter of `DefineProp` is a descriptor, and a value that is not
    // one is refused with the reason.
    let report = run("o := {}\no.DefineProp(\"x\", 1)");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Object.Prototype.DefineProp is invalid."
    );
}

#[test]
fn an_object_without_a_member_is_not_a_descriptor() {
    let report = run("o := {}\no.DefineProp(\"x\", {y: 1})");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of Object.Prototype.DefineProp is invalid."
    );
}

#[test]
fn a_descriptor_of_a_value_holds_the_value() {
    let report = run("o := {}\no.DefineProp(\"x\", {Value: 5})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(5));
}

#[test]
fn the_answer_of_a_defined_property_is_the_object() {
    let report = run("o := {}\nr := o.DefineProp(\"x\", {Value: 5})\na := r == o");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
}

#[test]
fn a_descriptor_of_a_getter_is_asked_for_the_value() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => 42})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(42));
}

#[test]
fn the_getter_of_a_descriptor_is_asked_with_the_object() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => Type(f)})\na := o.x");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Object".into()));
}

#[test]
fn a_descriptor_of_a_setter_is_asked_for_the_write() {
    let report = run("o := {}\no.DefineProp(\"x\", {Set: (f, v) => 0})\no.x := 3");
    assert!(report.ok, "{:?}", report.error);
}

#[test]
fn a_property_without_a_setter_is_read_only() {
    let report = run("o := {}\no.DefineProp(\"x\", {Get: (f) => 42})\no.x := 3");
    assert_eq!(failure_of(&report), "Property is read-only.");
}

#[test]
fn a_descriptor_of_a_call_is_called() {
    let report = run("o := {}\no.DefineProp(\"x\", {Call: (f, a) => a * 2})\na := o.x(5)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(10));
}

#[test]
fn the_descriptor_that_comes_back_is_an_object() {
    let report = run("o := {}\no.DefineProp(\"x\", {Value: 5})\n\
         a := Type(o.GetOwnPropDesc(\"x\")) . \" \" . o.GetOwnPropDesc(\"x\").Value");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Object 5".into()));
}

#[test]
fn the_descriptor_of_a_property_that_is_not_there_is_refused() {
    let report = run("o := {}\na := o.GetOwnPropDesc(\"nope\")");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Object\" has no property named \"nope\"."
    );
}

#[test]
fn the_take_away_of_a_property_answers_its_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: `DeleteProp`
    // answers the value of the property that is taken away.
    let report = run(
        "o := {}\no.DefineProp(\"x\", {Value: 7})\na := o.DeleteProp(\"x\")\nb := o.DeleteProp(\"x\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(7));
    assert_eq!(value_of(&report, "b"), Value::Text("".into()));
}

#[test]
fn the_names_of_com_fail_with_the_reason() {
    // `ObjFromPtr` was the example until round 203 answered it: a number that names an
    // object is a feature a runtime can carry on any platform, and it is an address in
    // v2 only because v2 is a C++ program. What is genuinely Windows is the COM
    // objects, and `ComObjActive` is one.
    let report = run("a := ComObjActive(\"Excel.Application\")");
    assert_eq!(
        failure_of(&report),
        "COM objects belong to the component model of Windows, and this runtime has no COM."
    );
}

#[test]
fn mod_takes_the_remainder() {
    let report = run("a := Mod(7, 3)\nb := Mod(7.5, 2)\nc := Mod(-7, 3)");
    assert!(report.ok, "{:?}", report.error);
    // The answer is a whole number when both of the arguments are.
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Float(1.5));
    // The sign of the remainder is the sign of the dividend.
    assert_eq!(value_of(&report, "c"), Value::Int(-1));
}

#[test]
fn mod_by_zero_says_so() {
    let report = run("a := Mod(7, 0)");
    assert_eq!(failure_of(&report), "Divide by zero.");
}

#[test]
fn random_answers_a_number_in_its_range() {
    let report = run("a := Random()\nb := Random(10)\nc := Random(5, 10)");
    assert!(report.ok, "{:?}", report.error);
    let unit = float_of(&report, "a");
    assert!((0.0..1.0).contains(&unit), "Random() is {unit}");
    let below = int_of(&report, "b");
    assert!((0..10).contains(&below), "Random(10) is {below}");
    let between = int_of(&report, "c");
    assert!((5..=10).contains(&between), "Random(5, 10) is {between}");
}

#[test]
fn sort_puts_the_lines_in_order() {
    let report = run("a := Sort(\"pear\" . Chr(10) . \"apple\" . Chr(10) . \"fig\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "a"),
        Value::Text("apple\nfig\npear".into())
    );
}

#[test]
fn sort_puts_a_list_in_order() {
    let mut desktop = Fake::default();
    let mut runtime = Runtime::new(&mut desktop, Duration::from_secs(10));
    let sort = ahk::builtins::find("Sort").expect("the tree carries Sort");
    let numbers = Value::List(vec![Value::Int(3), Value::Int(1), Value::Int(2)]);
    let sorted = (sort.handler)(&mut runtime, &[numbers], 1, 0).expect("the list is sorted");
    assert_eq!(
        sorted,
        Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
    );
    let words = Value::List(vec![
        Value::Text("pear".into()),
        Value::Text("apple".into()),
    ]);
    let sorted = (sort.handler)(&mut runtime, &[words], 1, 0).expect("the list is sorted");
    assert_eq!(
        sorted,
        Value::List(vec![
            Value::Text("apple".into()),
            Value::Text("pear".into())
        ])
    );
}

#[test]
fn sort_of_something_that_is_not_a_list_fails() {
    let report = run("a := Sort(5)");
    assert_eq!(failure_of(&report), "Parameter #1 of Sort is invalid.");
}

#[test]
fn chr_and_ord_know_the_characters() {
    let report = run("a := Chr(65)\nb := Ord(\"A\")\nc := Ord(\"\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("A".into()));
    assert_eq!(value_of(&report, "b"), Value::Int(65));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
}

#[test]
fn chr_of_a_code_that_is_not_a_character_fails() {
    let report = run("a := Chr(-1)");
    assert_eq!(failure_of(&report), "Parameter #1 of Chr is invalid.");
}

#[test]
fn str_compare_says_which_text_comes_first() {
    let report = run(concat!(
        "a := StrCompare(\"a\", \"b\")",
        "\nb := StrCompare(\"b\", \"a\")",
        "\nc := StrCompare(\"a\", \"a\")",
        "\nd := StrCompare(\"A\", \"a\")",
        "\ne := StrCompare(\"A\", \"a\", true)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(-1));
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    // The case is not looked at, unless the caller asks for it.
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(-1));
}

#[test]
fn the_case_sense_wants_a_boolean() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the sense of the case
    // is a boolean, and a text is not a value that the parameter can take.
    for (source, message) in [
        (
            "a := StrCompare(\"a\", \"A\", \"On\")",
            "Parameter #3 of StrCompare is invalid.",
        ),
        (
            "a := StrReplace(\"a\", \"a\", \"b\", \"On\")",
            "Parameter #4 of StrReplace is invalid.",
        ),
        (
            "a := InStr(\"a\", \"a\", \"On\")",
            "Parameter #3 of InStr is invalid.",
        ),
        (
            "a := StrCompare(\"a\", \"A\", 3)",
            "Parameter #3 of StrCompare is invalid.",
        ),
        (
            "a := StrReplace(\"a\", \"a\", \"b\", 3)",
            "Parameter #4 of StrReplace is invalid.",
        ),
        (
            "a := InStr(\"a\", \"a\", 1.0)",
            "Parameter #3 of InStr is invalid.",
        ),
    ] {
        let report = run(source);
        assert_eq!(failure_of(&report), message, "{source}");
    }
}

#[test]
fn format_puts_the_values_in_the_pattern() {
    let report = run(concat!(
        "a := Format(\"{1} of {2}\", 2, 3)",
        "\nb := Format(\"{{1}}\", 2)",
        "\nc := Format(\"{1:.2}\", 2.345)",
        "\nd := Format(\"{1}\", 2.5)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("2 of 3".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("{1}".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("2.35".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("2.5".into()));
}

#[test]
fn format_without_a_pattern_fails() {
    let report = run("a := Format()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: Format"
    );
}

#[test]
fn in_str_says_where_the_needle_is() {
    let report = run(concat!(
        "a := InStr(\"hello\", \"ll\")",
        "\nb := InStr(\"hello\", \"z\")",
        "\nc := InStr(\"hello\", \"l\", 1)",
        "\nd := InStr(\"hello\", \"l\", true, 4)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    // Measured on the binary of v2.0.19 on 2026-10-31: the third parameter is
    // the sense of the case and the fourth is the place where the search
    // begins.
    assert_eq!(value_of(&report, "c"), Value::Int(3));
    assert_eq!(value_of(&report, "d"), Value::Int(4));
}

#[test]
fn in_str_counts_the_occurrences() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the fifth parameter is
    // the occurrence that the caller wants, and zero is not one.
    let report = run(concat!(
        "a := InStr(\"abcabc\", \"b\", true, 1, 1)",
        "\nb := InStr(\"abcabc\", \"b\", true, 1, 2)",
        "\nc := InStr(\"abcabc\", \"b\", true, 1, 3)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(2));
    assert_eq!(value_of(&report, "b"), Value::Int(5));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
}

#[test]
fn the_occurrence_of_the_needle_wants_a_number() {
    // Measured on the binary of v2.0.19 on 2026-10-31.
    let report = run("a := InStr(\"abcabc\", \"b\", true, 1, 0)");
    assert_eq!(failure_of(&report), "Parameter #5 of InStr is invalid.");
    let report = run("a := InStr(\"abcabc\", \"b\", true, 1, \"x\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of InStr requires a Number, but received a String."
    );
}

#[test]
fn in_str_with_an_empty_needle_fails() {
    let report = run("a := InStr(\"hello\", \"\")");
    assert_eq!(failure_of(&report), "Parameter #2 of InStr is invalid.");
}

#[test]
fn str_replace_swaps_the_parts() {
    let report = run(concat!(
        "a := StrReplace(\"hello\", \"l\", \"L\")",
        "\nb := StrReplace(\"hello\", \"l\", \"L\", 1)",
        "\nc := StrReplace(\"hello\", \"L\", \"L\")",
        "\nd := StrReplace(\"Hello hello\", \"hello\", \"x\", 0)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("heLLo".into()));
    // The fourth parameter of v2 is the case sense, and it is on when the
    // caller says so.
    assert_eq!(value_of(&report, "b"), Value::Text("heLLo".into()));
    // The case is not looked at, unless the caller asks for it.
    assert_eq!(value_of(&report, "c"), Value::Text("heLLo".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("x x".into()));
}

#[test]
fn str_replace_writes_the_number_of_the_swaps() {
    let report = run(concat!(
        "a := StrReplace(\"aaa\", \"a\", \"b\", true, &count)",
        "\nb := StrReplace(\"aaa\", \"a\", \"b\", false, &other, 1)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("bbb".into()));
    assert_eq!(int_of(&report, "count"), 3);
    // The limit is the sixth parameter, and it says how many of the
    // swaps are made.
    assert_eq!(value_of(&report, "b"), Value::Text("baa".into()));
    assert_eq!(int_of(&report, "other"), 1);
}

#[test]
fn str_replace_wants_a_reference_where_it_writes_the_count() {
    let report = run("a := StrReplace(\"aaa\", \"a\", \"b\", true, 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of StrReplace requires a variable reference, but received an Integer."
    );
}

#[test]
fn var_set_str_capacity_answers_the_capacity() {
    let report = run(concat!(
        "x := \"abc\"",
        "\na := VarSetStrCapacity(&x, 100)",
        "\ny := \"abc\"",
        "\nb := VarSetStrCapacity(&y)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 100);
    // The string is emptied when a capacity is given.
    assert_eq!(value_of(&report, "x"), Value::Text(String::new()));
    // Without a capacity the length of the string is the answer, and the
    // string is left alone.
    assert_eq!(int_of(&report, "b"), 3);
    assert_eq!(value_of(&report, "y"), Value::Text("abc".into()));
}

#[test]
fn str_replace_with_an_empty_needle_changes_nothing() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a needle that is
    // empty changes nothing, and the text comes back as it is.
    let report = run("a := StrReplace(\"hello\", \"\", \"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("hello".into()));
}

#[test]
fn the_trims_take_the_characters_off_the_ends() {
    let report = run(concat!(
        "a := Trim(\"  hi  \")",
        "\nb := LTrim(\"  hi  \")",
        "\nc := RTrim(\"  hi  \")",
        "\nd := Trim(\"xxhixx\", \"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("hi".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("hi  ".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("  hi".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("hi".into()));
}

#[test]
fn sub_str_takes_the_part_between_the_places() {
    let report =
        run("a := SubStr(\"hello\", 2)\nb := SubStr(\"hello\", 2, 2)\nc := SubStr(\"hello\", -2)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("ello".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("el".into()));
    // A negative place counts from the end.
    assert_eq!(value_of(&report, "c"), Value::Text("lo".into()));
}

#[test]
fn sub_str_without_a_place_fails() {
    let report = run("a := SubStr(\"hello\")");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: SubStr"
    );
}

#[test]
fn the_predicates_say_what_the_value_is() {
    let report = run(concat!(
        "a := IsAlnum(\"abc123\")",
        "\nb := IsAlpha(\"abc\")",
        "\nc := IsDigit(\"123\")",
        "\nd := IsFloat(\"1.5\")",
        "\ne := IsInteger(\"42\")",
        "\nf := IsLower(\"abc\")",
        "\ng := IsNumber(\"4.2\")",
        "\nh := IsObject(1)",
        "\ni := IsSpace(\" \")",
        "\nj := IsTime(\"20240101\")",
        "\nk := IsUpper(\"ABC\")",
        "\nl := IsXDigit(\"1F\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(1));
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(1));
    assert_eq!(value_of(&report, "d"), Value::Int(1));
    assert_eq!(value_of(&report, "e"), Value::Int(1));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
    assert_eq!(value_of(&report, "g"), Value::Int(1));
    assert_eq!(value_of(&report, "h"), Value::Int(0));
    assert_eq!(value_of(&report, "i"), Value::Int(1));
    assert_eq!(value_of(&report, "j"), Value::Int(1));
    assert_eq!(value_of(&report, "k"), Value::Int(1));
    assert_eq!(value_of(&report, "l"), Value::Int(1));
}

#[test]
fn the_predicates_that_look_at_the_letters_want_a_text() {
    let report = run("a := IsAlpha(5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn the_predicates_say_no_when_they_should() {
    let report = run(concat!(
        "a := IsAlpha(\"ab1\")",
        "\nb := IsDigit(\"12a\")",
        "\nc := IsFloat(\"12\")",
        "\nd := IsInteger(\"1.5\")",
        "\ne := IsNumber(\"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(0));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(0));
}

#[test]
fn type_says_the_name_of_the_type() {
    let report = run("a := Type(1)\nb := Type(1.5)\nc := Type(\"x\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Integer".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("Float".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("String".into()));
}

#[test]
fn the_type_of_an_object_is_the_name_of_its_class() {
    let report = run("class Box {\n}\nb := Box()\n\
         a := Type(b)\nc := Type(Box)\nd := Type(Map())\ne := Type(Box.Prototype)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("Box".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Class".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("Map".into()));
    assert_eq!(value_of(&report, "e"), Value::Text("Prototype".into()));
}

#[test]
fn a_reference_is_a_value_of_its_own_kind() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `Type` of a
    // reference names its kind, and the predicates answer about it.
    let report = run(concat!(
        "y := 1\n",
        "a := Type(&y)\n",
        "b := IsObject(&y)\n",
        "c := IsInteger(&y)\n",
        "d := IsNumber(&y)\n",
        "e := IsFloat(&y)\n",
        "f := IsSetRef(&y)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("VarRef".into()));
    // A reference is an object of its own kind.
    assert_eq!(value_of(&report, "b"), Value::Int(1));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(0));
    assert_eq!(value_of(&report, "e"), Value::Int(0));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
}

#[test]
fn the_questions_about_a_value_answer_a_value() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a value that is
    // not an object has no method and no property, and the answer is
    // zero and not a refusal. The one property that every value has is
    // its class.
    let report = run(concat!(
        "y := 1\n",
        "a := HasMethod(&y)\n",
        "b := HasMethod(1)\n",
        "c := HasProp(&y, \"nothing\")\n",
        "d := HasProp(&y, \"__Class\")\n",
        "e := HasProp([1, 2], \"Length\")\n",
        "f := HasMethod([1, 2], \"Push\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(0));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(0));
    assert_eq!(value_of(&report, "d"), Value::Int(1));
    // The array answers about its own properties and methods.
    assert_eq!(value_of(&report, "e"), Value::Int(1));
    assert_eq!(value_of(&report, "f"), Value::Int(1));
}

#[test]
fn regex_match_says_where_the_pattern_is() {
    let report = run(concat!(
        "a := RegExMatch(\"hello\", \"l+\")",
        "\nb := RegExMatch(\"hello\", \"z\")",
        "\nc := RegExMatch(\"hello\", \"(l)(l)\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(3));
    assert_eq!(value_of(&report, "b"), Value::Int(0));
    assert_eq!(value_of(&report, "c"), Value::Int(3));
}

#[test]
fn regex_match_with_an_output_variable_fails() {
    let report = run("a := RegExMatch(\"hello\", \"l\", \"out\")");
    assert_eq!(
        failure_of(&report),
        "the matches of the pattern are not carried into a variable yet."
    );
}

#[test]
fn regex_replace_swaps_the_matches() {
    let report = run(concat!(
        "a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\")",
        "\nb := RegExReplace(\"a1b2\", \"([0-9])\", \"#$1#\")",
        "\nc := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &count, 1)",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("a#b#".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("a#1#b#2#".into()));
    // The limit is the fifth parameter of v2, and the number of the swaps
    // goes into the variable of the caller.
    assert_eq!(value_of(&report, "c"), Value::Text("a#b2".into()));
    assert_eq!(int_of(&report, "count"), 1);
}

#[test]
fn regex_replace_looks_at_the_case_when_the_pattern_says_so() {
    let report = run(concat!(
        "a := RegExReplace(\"Hello hello\", \"hello\", \"x\")",
        "\nb := RegExReplace(\"Hello hello\", \"(?i)hello\", \"x\")",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The pattern of v2 is exact, and the option `(?i)` is what takes the
    // case away.
    assert_eq!(value_of(&report, "a"), Value::Text("Hello x".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("x x".into()));
}

#[test]
fn regex_replace_starts_where_it_is_told_to() {
    let report = run(concat!(
        "a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &count, 2, 3)",
        "\nb := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", &other, 2, 2)",
    ));
    assert!(report.ok, "{:?}", report.error);
    // The sixth parameter of v2 is the place where the search starts, and
    // the text before it is left alone.
    assert_eq!(value_of(&report, "a"), Value::Text("a1b#".into()));
    assert_eq!(int_of(&report, "count"), 1);
    assert_eq!(value_of(&report, "b"), Value::Text("a#b#".into()));
    assert_eq!(int_of(&report, "other"), 2);
}

#[test]
fn regex_replace_wants_a_reference_where_it_writes_the_count() {
    let report = run("a := RegExReplace(\"a1b2\", \"[0-9]\", \"#\", 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of RegExReplace requires a variable reference, but received an Integer."
    );
}

// -- the clipboard ----------------------------------------------------------------

#[test]
fn a_wait_for_the_clipboard_that_finds_data_answers_one() {
    let mut desktop = Fake {
        clipboard: "the text".to_string(),
        ..Default::default()
    };
    let report = run_on("MsgBox ClipWait(0.25)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_wait_for_the_clipboard_that_times_out_answers_zero() {
    let mut desktop = Fake::default();
    let report = run_on("MsgBox ClipWait(0.25)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["0"]);
}

#[test]
fn a_wait_for_the_clipboard_with_no_timeout_waits_for_the_data() {
    let mut desktop = Fake {
        clipboard: "the text".to_string(),
        ..Default::default()
    };
    let report = run_on("MsgBox ClipWait()", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn a_wait_for_the_data_of_any_kind_fails_with_the_reason_of_the_text() {
    let mut desktop = Fake::default();
    let report = run_on("MsgBox ClipWait(0.25, 1)", &mut desktop);
    assert_eq!(
        failure_of(&report),
        "the clipboard of this runtime carries text only, and it cannot wait for data of any kind."
    );
}

#[test]
fn a_desktop_that_cannot_read_the_clipboard_fails_the_wait_with_the_reason() {
    let mut desktop = Blind;
    let statements = parse("ClipWait(0.25)").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert_eq!(
        failure_of(&report),
        "this desktop cannot read the clipboard"
    );
}

#[test]
fn each_name_of_the_shape_of_the_runtime_refuses_with_the_reason_of_its_own_thing() {
    // The fourteen names of the shape shared one reason, and that reason
    // hid fourteen different things.
    for (name, source, reason) in [
        (
            "Critical",
            "Critical",
            "this runtime runs one script in one thread, and it cannot make it critical.",
        ),
        (
            "Thread",
            // v2 refuses a `Thread` that carries no argument, and the
            // refusal comes before the reason of this runtime: measured on
            // the binary of v2.0.19 on 2026-10-31.
            "Thread(1)",
            "this runtime runs one script in one thread, and it cannot start another.",
        ),
        (
            "Pause",
            "Pause",
            "this runtime cannot wait for a key to continue: the script ends when it ends.",
        ),
        (
            "Reload",
            "Reload",
            "this runtime runs one script once, and it cannot run itself again.",
        ),
        (
            "Suspend",
            "Suspend",
            "this runtime cannot stop itself: it would never end.",
        ),
        (
            "ListLines",
            "ListLines",
            "the lines are listed in a window of v2, and this runtime has no window to show them in.",
        ),
    ] {
        let report = run(source);
        assert_eq!(failure_of(&report), reason, "{name}");
        // The reason of the name is the one that the surface says.
        assert_eq!(ahk::builtins::refusal_of(name), Some(reason), "{name}");
    }
}

#[test]
fn a_command_says_how_many_parameters_it_wants() {
    // Measured on the binary of v2.0.19 on 2026-10-31: a command whose
    // parameters are all needed is refused at the load, and the refusal
    // is the one of the count.
    let report = run("PixelGetColor()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: PixelGetColor"
    );
}

#[test]
fn a_command_says_when_there_are_too_many_parameters() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `MsgBox` takes
    // three parameters, and a fourth is refused.
    let report = run("MsgBox(\"a\", \"b\", \"c\", \"d\")");
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: MsgBox"
    );
}

#[test]
fn a_parameter_that_may_not_be_left_out_is_named() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the `FileGetAttrib`
    // declares that its parameter may be left out, and the command needs
    // it. The refusal names the parameter and not the count.
    let report = run("FileGetAttrib()");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of FileGetAttrib must not be omitted in this case."
    );
}

#[test]
fn the_drag_names_the_parameter_it_needs() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the drag needs the
    // place where it ends, and it names the first one that is missing.
    let report = run("MouseClickDrag()");
    assert_eq!(
        failure_of(&report),
        "Parameter #4 of MouseClickDrag must not be omitted in this case."
    );
    let report = run("MouseClickDrag(\"Left\", 1, 2, 3)");
    assert_eq!(
        failure_of(&report),
        "Parameter #5 of MouseClickDrag must not be omitted in this case."
    );
}

#[test]
fn the_failure_of_an_omitted_parameter_carries_the_name_of_the_built_in() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the failure of a
    // parameter that may not be left out carries the name of the built-in
    // in the `What`, and an empty `Extra`.
    //
    // The tree carried an empty `What` there, and the case of it pinned
    // the tree and not the binary: the name of the built-in goes in the
    // `What` of every failure of a built-in.
    let report = run(concat!(
        "try\n",
        "  FileGetAttrib()\n",
        "catch as e\n",
        "  MsgBox e.What \"|\" e.Extra \"|\" e.Message\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["FileGetAttrib||Parameter #1 of FileGetAttrib must not be omitted in this case."]
    );
}

#[test]
fn the_function_of_the_exit_takes_the_reason_and_the_code() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the callback of
    // the exit takes the reason of the exit and the code.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason \", \" ExitCode\n",
        "}\n",
        "MsgBox \"the script ends\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the script ends", "Exit, 0"]);
}

#[test]
fn the_function_of_the_exit_runs_when_the_script_stops_with_a_code() {
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason \", \" ExitCode\n",
        "}\n",
        "Exit 3\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Exit, 3"]);
    assert_eq!(report.exit_code, Some(3));
}

#[test]
fn the_function_of_the_exit_runs_when_the_script_fails() {
    // The end of the script is the end of the script, whatever the reason
    // it ended, and the reason is the one of v2.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode) {\n",
        "  MsgBox ExitReason\n",
        "}\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert_eq!(report.output, vec!["Error"]);
}

#[test]
fn a_function_of_the_exit_that_fails_fails_the_run() {
    let report = run("OnExit Bye\nBye(Reason, Code) {\n  MsgBox NoSuchVariable\n}\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This local variable has not been assigned a value.")
    );
    // The line is the line of the statement that failed, inside the
    // function.
    assert_eq!(report.error_line, Some(3));
}

#[test]
fn a_function_of_the_exit_that_fails_when_the_script_already_failed_is_a_note() {
    // The error of the script is the error of the run, and the failure of
    // the function is not lost.
    let report =
        run("OnExit Bye\nBye(Reason, Code) {\n  MsgBox NoSuchVariable\n}\nMsgBox NoSuchVariable\n");
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("This global variable has not been assigned a value.")
    );
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("the function of the exit failed")),
        "{:?}",
        report.notes
    );
}

#[test]
fn a_text_is_not_a_callback_and_refuses_with_the_reason_of_v2() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the callback is a
    // function object, and a text is refused.
    let report = run("OnExit(\"Bye\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of OnExit requires an Object, but received a String."
    );
}

#[test]
fn a_value_that_cannot_be_called_is_not_a_callback() {
    // v2 calls the value, and a value that cannot be called has no method
    // named "Call".
    let report = run("OnExit Map()");
    assert_eq!(
        failure_of(&report),
        "This value of type \"Object\" has no method named \"Call\"."
    );
}

#[test]
fn a_prototype_that_does_not_take_the_two_parameters_is_not_a_callback() {
    // Measured on the binary of v2.0.19 on 2026-10-29.
    let report = run("OnExit Bye\nBye() {\n}\n");
    assert_eq!(failure_of(&report), "Invalid callback function.");
}

#[test]
fn a_callback_that_takes_the_rest_of_the_arguments_is_a_callback() {
    // The asterisk takes what is left, and it fits.
    let report = run(concat!(
        "OnExit Bye\n",
        "Bye(ExitReason, ExitCode*) {\n",
        "  MsgBox ExitReason\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Exit"]);
}

#[test]
fn the_callback_of_the_exit_is_required() {
    let report = run("OnExit()");
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: OnExit"
    );
}

#[test]
fn the_order_of_the_callbacks_is_one_minus_one_or_zero() {
    let report = run("OnExit Bye, 5\nBye(Reason, Code) {\n}\n");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of OnExit is invalid.\n     Specifically: 5"
    );
}

#[test]
fn an_order_that_is_not_a_number_refuses_with_the_reason_of_v2() {
    let report = run("OnExit Bye, \"x\"\nBye(Reason, Code) {\n}\n");
    assert_eq!(
        failure_of(&report),
        "Parameter #2 of OnExit requires a Number, but received a String."
    );
}

#[test]
fn the_order_is_cut_to_a_whole_number() {
    // Measured on the binary of v2.0.19 on 2026-10-30: 1.5 is 1.
    let report = run("OnExit Bye, 1.5\nBye(Reason, Code) {\n  MsgBox \"bye\"\n}\n");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["bye"]);
}

#[test]
fn the_callback_that_is_asked_not_to_be_called_is_removed() {
    let report = run(concat!(
        "OnExit Bye\n",
        "OnExit Bye, 0\n",
        "Bye(Reason, Code) {\n",
        "  MsgBox \"bye\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

#[test]
fn the_callbacks_run_in_the_order_the_script_asked_for() {
    // One is after the ones that are registered, and minus one is before
    // them.
    let report = run(concat!(
        "First(Reason, Code) {\n  MsgBox \"first\"\n}\n",
        "Second(Reason, Code) {\n  MsgBox \"second\"\n}\n",
        "OnExit First\n",
        "OnExit Second\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["first", "second"]);
}

#[test]
fn a_callback_that_asks_to_run_before_the_others_runs_before_them() {
    let report = run(concat!(
        "First(Reason, Code) {\n  MsgBox \"first\"\n}\n",
        "Second(Reason, Code) {\n  MsgBox \"second\"\n}\n",
        "OnExit First\n",
        "OnExit Second, -1\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["second", "first"]);
}

#[test]
fn the_same_callback_registered_twice_runs_once() {
    // v2 keeps one entry for a callback.
    let report = run(concat!(
        "Bye(Reason, Code) {\n  MsgBox \"bye\"\n}\n",
        "OnExit Bye\n",
        "OnExit Bye\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["bye"]);
}

#[test]
fn a_function_of_the_exit_that_answers_a_number_keeps_the_script_alive() {
    // v2: the exit does not happen, and the callbacks that come after it
    // are not called. This runtime has no way to keep a script alive, so
    // the run ends and the note says so.
    let report = run(concat!(
        "Bye(Reason, Code) {\n  return 1\n}\n",
        "OnExit Bye\n",
        "Exit 3\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.exit_code, None);
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("keep the script alive")),
        "{:?}",
        report.notes
    );
}

#[test]
fn the_function_of_the_error_takes_the_value_and_the_mode() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the mode of a
    // failure of the runtime is "Return".
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox Mode\n}\n",
        "OnError Handler\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert_eq!(report.output, vec!["Return"]);
}

#[test]
fn the_function_of_the_error_takes_the_value_that_was_thrown() {
    // A value that the script threw is not a continuable failure: the
    // mode is "Exit".
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox Mode \", \" Thrown\n}\n",
        "OnError Handler\n",
        "throw \"the error of the script\"\n",
    ));
    assert!(!report.ok);
    assert_eq!(report.output, vec!["Exit, the error of the script"]);
}

#[test]
fn a_function_of_the_error_that_answers_one_hides_the_failure() {
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  return 1\n}\n",
        "OnError Handler\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(
        report
            .notes
            .iter()
            .any(|note| note.contains("hid the failure")),
        "{:?}",
        report.notes
    );
}

#[test]
fn a_function_of_the_error_that_answers_minus_one_lets_the_thread_continue() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the thread goes on
    // where the failure happened.
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  return -1\n}\n",
        "OnError Handler\n",
        "Fail()\n",
        "MsgBox \"after the call\"\n",
        "Fail() {\n",
        "  MsgBox NoSuchVariable\n",
        "  MsgBox \"inside the function\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["inside the function", "after the call"]);
}

#[test]
fn the_functions_of_the_error_are_not_called_while_a_try_is_active() {
    // v2: the catch of the script takes the failure first.
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox \"the handler\"\n}\n",
        "OnError Handler\n",
        "try {\n",
        "  MsgBox NoSuchVariable\n",
        "} catch {\n",
        "  MsgBox \"in the catch\"\n",
        "}\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["in the catch"]);
}

#[test]
fn the_function_of_the_error_is_removed_when_the_order_is_zero() {
    let report = run(concat!(
        "Handler(Thrown, Mode) {\n  MsgBox \"the handler\"\n}\n",
        "OnError Handler\n",
        "OnError Handler, 0\n",
        "MsgBox NoSuchVariable\n",
    ));
    assert!(!report.ok);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

#[test]
fn a_script_that_writes_the_clipboard_finds_what_it_wrote_when_it_reads_it_again() {
    let mut desktop = Fake::default();
    let report = run_on(
        "A_Clipboard := \"the new text\"\nMsgBox A_Clipboard",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["the new text"]);
    assert_eq!(desktop.clipboard, "the new text");
}

// -- the native call, against a desktop that records it ------------------------

#[test]
fn the_library_and_the_function_are_read_out_of_the_first_parameter() {
    let mut desktop = Fake::default();
    let report = run_on("DllCall(\"kernel32\\GetLastError\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.library, "kernel32");
    assert_eq!(call.function, "GetLastError");
}

#[test]
fn the_types_of_the_arguments_are_read_in_pairs() {
    let mut desktop = Fake::default();
    let report = run_on(
        "DllCall(\"f\", \"Int\", 5, \"Str\", \"hello\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.arguments.len(), 2);
    assert_eq!(call.arguments[0].kind, NativeType::Int);
    assert_eq!(call.arguments[0].value, NativeValue::Integer(5));
    assert_eq!(call.arguments[1].kind, NativeType::Str);
    // A text goes out as its bytes: two a character, and the nul at the end.
    assert_eq!(
        call.arguments[1].value,
        NativeValue::Bytes(vec![b'h', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0, 0, 0])
    );
}

#[test]
fn the_type_of_the_answer_is_the_last_parameter_when_the_rest_is_odd() {
    let mut desktop = Fake::default();
    let report = run_on("DllCall(\"f\", \"Int\", 5, \"Double\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    let call = desktop.native.expect("the desktop was asked for a call");
    assert_eq!(call.returns, NativeType::Double);
    assert_eq!(call.arguments.len(), 1);
}

#[test]
fn the_answer_of_the_desktop_comes_back_to_the_script() {
    let mut desktop = Fake {
        answer: Some(NativeResult {
            kind: NativeType::Int,
            value: NativeValue::Integer(42),
        }),
        ..Default::default()
    };
    let report = run_on("a := DllCall(\"f\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Int(42));
}

#[test]
fn a_star_at_the_end_of_a_type_is_the_address_of_a_variable() {
    // The fake writes into the memory of the address, the way the box does.
    let mut desktop = Fake {
        writes: vec![42.0],
        ..Default::default()
    };
    let report = run_on(
        "counter := 0\nDllCall(\"f\", \"Int64*\", &counter)",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "counter"), 42);
}

#[test]
fn a_type_that_is_not_a_type_fails() {
    let report = run("DllCall(\"f\", \"Nope\", 5)");
    assert_eq!(failure_of(&report), "Invalid arg type.");
}

#[test]
fn a_type_of_the_answer_that_is_not_a_type_fails() {
    let report = run("DllCall(\"f\", \"Int\", 5, \"Nope\")");
    assert_eq!(failure_of(&report), "Invalid return type.");
}

#[test]
fn a_number_where_a_text_is_wanted_fails() {
    let report = run("DllCall(\"f\", 5, 5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn a_text_where_a_number_is_wanted_fails() {
    let report = run("DllCall(\"f\", \"Int\", \"hello\")");
    assert_eq!(failure_of(&report), "Expected a Number but got a String.");
}

#[test]
fn a_first_parameter_that_is_not_a_text_fails() {
    let report = run("DllCall(5)");
    assert_eq!(failure_of(&report), "Expected a String but got an Integer.");
}

#[test]
fn the_base_of_an_object_of_a_class_type_of_the_runtime_cannot_be_written() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the prototype of the
    // class is there, and the base of the object cannot be replaced.
    for source in [
        "m := Map()\nObjSetBase(m, Object())",
        "a := [1, 2]\nObjSetBase(a, Object())",
        "b := Buffer(4)\nObjSetBase(b, Object())",
        "f := Abs\nObjSetBase(f, Object())",
        "m := Map()\nm.Base := Object()",
    ] {
        let report = run(&format!(
            "try {{\n    {source}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}\n"
        ));
        assert!(report.ok, "{:?}", report.error);
        assert_eq!(
            report.output,
            vec!["Invalid base. | ObjSetBase | "],
            "the base of {source}"
        );
    }
    // The base of a plain object, of an error and of an instance of a class
    // of the script can be written.
    for source in [
        "o := Object()\nObjSetBase(o, Object())",
        "e := Error(\"x\")\nObjSetBase(e, Object())",
        "class C {\n}\no := C()\nObjSetBase(o, Object())",
    ] {
        let report = run(source);
        assert!(report.ok, "{:?}", report.error);
    }
    // The value that is not an object goes in the `Extra` of the refusal.
    let report = run(
        "try {\n    ObjSetBase(Map(), 1)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Expected an Object but got an Integer. | ObjSetBase | 1"]
    );
}

#[test]
fn a_desktop_that_cannot_make_a_native_call_fails_with_the_reason() {
    let mut desktop = Blind;
    let statements = parse("DllCall(\"f\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert_eq!(
        failure_of(&report),
        "this desktop cannot make a native call."
    );
}

#[test]
fn the_failure_of_a_built_in_names_the_built_in() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the failure of a
    // built-in carries the name of it in the `What`, and the value of the
    // parameter that failed in the `Extra`.
    let report = run(
        "try {\n    x := Chr(-1)\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of Chr is invalid. | Chr | -1"]
    );
}

#[test]
fn the_failure_of_a_built_in_that_wants_a_number_carries_the_value() {
    let report = run(
        "try {\n    x := Round(\"abc\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of Round requires a Number, but received a String. | Round | abc"]
    );
}

#[test]
fn the_failure_of_a_built_in_is_of_the_class_of_v2() {
    let report = run("try {\n    x := Chr(-1)\n} catch Any as e {\n    MsgBox e is ValueError\n}");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1"]);
}

#[test]
fn the_file_append_writes_to_the_console_when_the_name_is_the_star() {
    // Measured on the binary of v2.0.19 on 2026-10-30: the star is the
    // standard output of the machine, and the answer of the command is
    // an empty text.
    let report = run("x := FileAppend(\"hello\", \"*\")\nMsgBox \"[%x%]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello", "[]"]);
}

#[test]
fn the_file_open_of_the_star_opens_the_console() {
    // The writes go to the standard output, the reads answer an empty
    // text, and the length of the file is the number of the characters
    // that were written to it.
    let report = run(
        "f := FileOpen(\"*\", \"w\")\nwritten := f.Write(\"hello\")\nMsgBox \"%written% %f.Length% %f.Pos% %f.AtEOF%\"",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["hello", "5 5 5 1"]);
}

#[test]
fn the_read_of_the_console_answers_an_empty_text() {
    let report = run("f := FileOpen(\"*\", \"r\")\ntext := f.Read()\nMsgBox \"[%text%]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

// -- the dialogs ----------------------------------------------------------------

#[test]
fn the_input_box_answers_an_object_with_the_text_and_the_result() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer of
    // the InputBox is an object, and it carries the text of the user in
    // its Value and the way the dialog ended in its Result.
    let report = run(concat!(
        "ib := InputBox(\"the prompt\", \"the title\", \"T10\", \"the default\")\n",
        "a := ib.Value\n",
        "b := ib.Result\n",
        "c := ib.__Class\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("the answer".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("OK".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("Object".into()));
}

#[test]
fn the_input_box_carries_what_it_was_asked() {
    let mut desktop = Fake::default();
    let report = run_on(
        "InputBox(\"the prompt\", \"the title\", \"T10\", \"the default\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![(
            "input-box".to_string(),
            "the prompt|the title|the default|Some(10.0)".to_string()
        )]
    );
}

#[test]
fn the_input_box_of_too_many_arguments_fails_the_way_v2_does() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the InputBox
    // takes four parameters, and the fifth is too many.
    let report = run("InputBox(\"a\", \"b\", \"c\", \"d\", \"e\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: InputBox"
    );
}

#[test]
fn an_option_of_the_input_box_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.29 on 2026-10-29: the message is
    // "Invalid option.", the class of the failure is a ValueError, and
    // the option that was not known is in the Extra.
    let report = run(
        "try {\n    InputBox(\"a\", \"b\", \"z9\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Invalid option. | ValueError | InputBox | z9"]
    );
}

#[test]
fn the_file_select_answers_the_path_of_what_was_chosen() {
    let report = run("a := FileSelect(\"D\", \"/tmp\", \"the title\")\nMsgBox \"%a%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/tmp/chosen.txt"]);
}

#[test]
fn the_file_select_of_many_files_answers_an_array() {
    let report = run("a := FileSelect(\"M\")\nMsgBox a.Length \" \" a[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["2 /tmp/one.txt"]);
}

#[test]
fn the_file_select_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("FileSelect(\"a\", \"b\", \"c\", \"d\", \"e\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: FileSelect"
    );
}

#[test]
fn an_option_of_the_file_select_that_v2_does_not_know_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the message is
    // "Parameter #1 of FileSelect is invalid." and the options are in
    // the Extra of the failure.
    let report = run(
        "try {\n    FileSelect(\"Q\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of FileSelect is invalid. | ValueError | FileSelect | Q"]
    );
}

#[test]
fn the_filter_of_the_file_select_is_not_there_when_a_folder_is_asked_for() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the filter of
    // the FileSelect is refused when the user chooses a folder.
    let report = run(
        "try {\n    FileSelect(\"D\", \"\", \"\", \"Text (*.txt)\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #4 of FileSelect is invalid. | Text (*.txt)"]
    );
}

#[test]
fn the_dir_select_answers_the_path_of_the_folder() {
    let report = run("a := DirSelect(\"/tmp\", 3, \"the prompt\")\nMsgBox \"%a%\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["/tmp/chosen"]);
}

#[test]
fn the_dir_select_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("DirSelect(\"a\", \"b\", \"c\", \"d\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: DirSelect"
    );
}

#[test]
fn the_options_of_the_dir_select_are_a_number() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the options of
    // the DirSelect are a number, and a text that is not one is
    // refused.
    let report = run(
        "try {\n    DirSelect(\"a\", \"b\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec![
            "Parameter #2 of DirSelect requires a Number, but received a String. | TypeError | DirSelect | b"
        ]
    );
}

#[test]
fn the_load_picture_answers_the_number_of_the_picture() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer is
    // the number that identifies the picture, and the type of the
    // handle is written in the third parameter.
    let mut desktop = Fake::default();
    let report = run_on(
        "t := 9\nx := LoadPicture(\"the.png\", \"w10\", &t)\nMsgBox x \" | \" t",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["7 | 0"]);
    assert_eq!(
        desktop.calls,
        vec![("load-picture".to_string(), "the.png".to_string())]
    );
}

#[test]
fn the_load_picture_without_arguments_fails_the_way_v2_does() {
    let report = run("x := LoadPicture()");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: LoadPicture"
    );
}

#[test]
fn the_load_picture_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("x := LoadPicture(\"a\", \"b\", &t, \"d\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: LoadPicture"
    );
}

#[test]
fn the_image_search_answers_one_and_writes_the_place() {
    let mut desktop = Fake::default();
    let report = run_on(
        "x := 0\ny := 0\nz := ImageSearch(&x, &y, 1, 2, 3, 4, \"the.png\")\nMsgBox z \" | \" x \" | \" y",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["1 | 10 | 20"]);
    assert_eq!(
        desktop.calls,
        vec![("image-search".to_string(), "1,2,3,4".to_string())]
    );
}

#[test]
fn the_image_search_wants_two_variables() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the first two
    // parameters want a variable.
    let report = run("z := ImageSearch(0, 0, 0, 0, 10, 10, \"the.png\")\n");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ImageSearch requires a VarRef, but received an Integer."
    );
}

#[test]
fn the_image_search_of_a_picture_that_cannot_be_loaded_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the message is
    // "Parameter #7 of ImageSearch is invalid." and the name of the
    // picture is in the Extra.
    let report = run(
        "try {\n    x := 0\n    y := 0\n    ImageSearch(&x, &y, 0, 0, 10, 10, \"\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #7 of ImageSearch is invalid. | ValueError | ImageSearch"]
    );
}

#[test]
fn the_image_search_of_too_few_arguments_fails_the_way_v2_does() {
    let report = run("x := 0\ny := 0\nz := ImageSearch(&x, &y, 0, 0, 10, 10)");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: ImageSearch"
    );
}

#[test]
fn the_image_search_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("x := 0\ny := 0\nz := ImageSearch(&x, &y, 0, 0, 10, 10, \"a\", \"b\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: ImageSearch"
    );
}

#[test]
fn a_desktop_that_does_not_read_the_images_says_so() {
    let mut desktop = Blind;
    let statements = parse("x := LoadPicture(\"the.png\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some(
            "the picture is read by the desktop, and this desktop does not read the images of the machine."
        )
    );
}

#[test]
fn the_download_writes_the_file_and_answers_nothing() {
    // Measured on the binary of v2.0.19 on 2026-10-29: the answer of the
    // command is an empty text, and the file is written where the script
    // said.
    let mut desktop = Fake::default();
    let report = run_on(
        "x := Download(\"https://example.com/file.txt\", \"/tmp/file.txt\")\nMsgBox \"[\" x \"]\"",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
    assert_eq!(
        desktop.calls,
        vec![(
            "download".to_string(),
            "https://example.com/file.txt|/tmp/file.txt".to_string()
        )]
    );
}

#[test]
fn the_download_of_too_many_arguments_fails_the_way_v2_does() {
    let report = run("Download(\"a\", \"b\", \"c\")");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too many parameters passed to function.\n     Specifically: Download"
    );
}

#[test]
fn the_download_without_arguments_fails_the_way_v2_does() {
    let report = run("Download()");
    assert!(!report.ok);
    assert_eq!(
        failure_of(&report),
        "Too few parameters passed to function.\n     Specifically: Download"
    );
}

#[test]
fn a_desktop_that_does_not_reach_the_network_says_so() {
    let mut desktop = Blind;
    let statements =
        parse("Download(\"https://example.com/file.txt\", \"/tmp/file.txt\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("the download is made by the desktop, and this desktop does not reach the network.")
    );
}

// -- the names that the tree does not carry ------------------------------------------------

#[test]
fn a_name_that_the_tree_does_not_carry_says_the_kind_of_the_parameter_first() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect says
    // the kind that the parameter wants before it says anything else.
    let report = run("x := 1\na := StrPut(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrPut requires a String, but received a VarRef."
    );
}

#[test]
fn a_name_that_the_tree_does_not_carry_wants_a_number_where_the_dialect_wants_one() {
    let report = run("x := 1\na := Shutdown(&x)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of Shutdown requires a Number, but received a VarRef."
    );
}

#[test]
fn a_value_of_the_wrong_kind_is_refused_before_the_reason_of_the_machine() {
    let report = run("a := StrPut([])");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of StrPut requires a String, but received an Array."
    );
}

#[test]
fn the_check_that_the_body_of_a_function_makes_says_its_own_words() {
    // The `StrPtr` checks the kind of its argument in its body, and the
    // words are not the ones of a parameter that declares its kind.
    let report = run("x := 1\na := StrPtr(&x)");
    assert_eq!(failure_of(&report), "Expected a String but got a VarRef.");
}

#[test]
fn a_parameter_that_wants_a_reference_says_it() {
    let report = run("a := CaretGetPos(5, 5)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of CaretGetPos requires a variable reference, but received an Integer."
    );
}

#[test]
fn an_empty_text_is_named_in_the_message_of_the_check() {
    let report = run("a := ObjOwnProps(\"\")");
    assert_eq!(
        failure_of(&report),
        "Expected an Object but got an empty string."
    );
}

#[test]
fn a_name_that_the_tree_does_not_carry_says_the_reason_of_the_machine_when_the_kind_fits() {
    // `StrPtr` used to answer the blanket reason. On 2026-11-10 it was given
    // one of its own: a pointer to a string is a value at a stable address, and
    // the values of this runtime have none.
    let report = run("a := StrPtr(\"x\")");
    assert_eq!(
        failure_of(&report),
        "a pointer to a string would be a value at a stable address, and the values of this runtime have none."
    );
}

#[test]
fn the_kind_of_a_value_comes_before_the_parameter_that_may_not_be_left_out() {
    // Measured on the binary of v2.0.19 on 2026-10-31: the dialect
    // converts the arguments before the body of the function runs.
    let report = run("a := ListViewGetContent([])");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of ListViewGetContent requires a String, but received an Array."
    );
}

#[test]
fn the_parameter_that_may_not_be_left_out_is_a_value_that_is_not_valid() {
    let report = run(
        "try {\n    a := ListViewGetContent(\"x\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.__Class\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of ListViewGetContent must not be omitted in this case. | ValueError"]
    );
}

#[test]
fn a_desktop_that_cannot_ask_the_user_says_so() {
    // The question is the machine's, and a machine that has no window
    // for it refuses with the reason.
    let mut desktop = Blind;
    let statements = parse("InputBox(\"the prompt\")").expect("parses");
    let report = Runtime::new(&mut desktop, Duration::from_secs(10)).run(&statements);
    assert!(!report.ok);
    assert_eq!(
        report.error.as_deref(),
        Some("this desktop cannot ask the user for a text")
    );
}

#[test]
fn the_name_of_a_file_that_is_empty_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the commands that
    // act on the name of a file refuse it when it is empty, and the value
    // that failed goes in the `Extra` as "empty string".
    let cases = [
        ("FileDelete \"\"", 1, "FileDelete"),
        ("FileRead \"\"", 1, "FileRead"),
        ("FileMove \"\", \"b.txt\"", 1, "FileMove"),
        ("FileMove \"a.txt\", \"\"", 2, "FileMove"),
        ("FileCopy \"\", \"b.txt\"", 1, "FileCopy"),
        ("FileCopy \"a.txt\", \"\"", 2, "FileCopy"),
        ("FileGetSize \"\"", 1, "FileGetSize"),
        ("FileGetTime \"\"", 1, "FileGetTime"),
        ("FileGetAttrib \"\"", 1, "FileGetAttrib"),
        ("DirCopy \"\", \"b\"", 1, "DirCopy"),
        ("DirCopy \"a\", \"\"", 2, "DirCopy"),
        ("DirMove \"\", \"b\"", 1, "DirMove"),
        ("DirMove \"a\", \"\"", 2, "DirMove"),
        ("FileSetTime \"a.txt\", \"\", \"b.txt\"", 2, "FileSetTime"),
        ("FileAppend \"a\", \"\"", 2, "FileAppend"),
    ];
    for (call, number, name) in cases {
        let report = run(&format!(
            "try {{\n    {call}\n}} catch Any as e {{\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}}"
        ));
        assert!(report.ok, "{call}: {:?}", report.error);
        assert_eq!(
            report.output,
            vec![format!(
                "Parameter #{number} of {name} is invalid. | {name} | empty string"
            )],
            "{call}"
        );
    }
}

#[test]
fn the_name_of_the_file_of_the_attributes_is_the_second_one_and_the_message_names_the_first() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of the file
    // is the second parameter of the `FileSetAttrib`, and the message of v2
    // names the first one, which is the attributes. The value of the first
    // one goes in the `Extra`.
    let report = run(
        "try {\n    FileSetAttrib(\"R\", \"\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #1 of FileSetAttrib is invalid. | FileSetAttrib | R"]
    );
}

#[test]
fn the_file_append_without_the_name_of_the_file_is_refused() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the name of the file
    // of a `FileAppend` may be left out only inside the body of a `Loop
    // Read` that has a file of its own.
    let report = run(
        "try {\n    FileAppend(\"a\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileAppend must not be omitted in this case. | FileAppend | "]
    );
}

#[test]
fn the_file_append_without_the_name_writes_to_the_file_of_the_loop() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the text of a
    // `FileAppend` that has no name of its own goes to the file of the
    // `Loop Read` that runs.
    let mut desktop = Fake::default();
    let report = run_on(
        "Loop Read \"in.txt\", \"out.txt\" {\n    FileAppend \"x\"\n}",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![
            ("append-file".to_string(), "out.txt x ".to_string()),
            ("append-file".to_string(), "out.txt x ".to_string()),
        ]
    );
}

#[test]
fn the_file_append_without_the_name_is_refused_in_a_loop_without_a_file() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the loop that reads
    // has no file of its own, and the name of the file may not be left out.
    let report = run(
        "try {\n    Loop Read \"in.txt\" {\n        FileAppend \"x\"\n    }\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileAppend must not be omitted in this case. | FileAppend"]
    );
}

#[test]
fn the_commands_of_the_files_answer_with_an_empty_text() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the answer of the
    // `DirCreate` and of the `FileDelete` is an empty text, and not the
    // name of what they did.
    let report = run("x := DirCreate(\"made\")\nMsgBox \"[\" x \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
    let report = run("x := FileDelete(\"a.txt\")\nMsgBox \"[\" x \"]\"");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["[]"]);
}

#[test]
fn the_name_of_the_built_in_is_in_the_failure_of_a_parameter_that_may_not_be_left_out() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the failure of a
    // parameter that may not be left out carries the name of the built-in
    // in the `What`.
    let report = run(
        "try {\n    x := FileSetTime(\"20200101000000\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["Parameter #2 of FileSetTime must not be omitted in this case. | FileSetTime"]
    );
}

#[test]
fn the_failure_of_a_copy_carries_the_number_of_the_files_that_failed() {
    // Measured on the binary of v2.0.19 on 2026-11-01: the copy of a file
    // that is not there fails with "Failed", and the `Extra` of the failure
    // is the number of the files that failed.
    let report = run(
        "try {\n    x := FileCopy(\"nope.txt\", \"b.txt\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What \" | \" e.Extra\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["Failed | FileCopy | 1"]);
}

#[test]
fn the_wildcards_of_the_delete_are_not_carried() {
    // The wildcards of v2 are not carried: the name of a file that is not
    // there is not a file, and the tree says so.
    let report = run(
        "try {\n    x := FileDelete(\"*.txt\")\n} catch Any as e {\n    MsgBox e.Message \" | \" e.What\n}",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.output,
        vec!["the wildcards of v2 are not carried yet. | FileDelete"]
    );
}
/// The class that the cases of the binding use: a method that takes one
/// argument and one that takes two.
const THE_THING: &str = "class Thing {\n    __New() {\n        this.v := 42\n    }\n    Method(x) {\n        return this.v + x\n    }\n    Two(x, y) {\n        return x + y\n    }\n}\n";

/// The `ObjBindMethod` binds the method of an object.
///
/// Measured on the binary of AutoHotkey v2.0.19 on 2026-11-05: the value is
/// of the class `BoundFunc`, and the call of it calls the method of the object
/// with the object as its `this`. The name of the method is not checked when
/// the value is made: the `ObjBindMethod(t, "Nope")` is made, and the call of
/// it fails.
#[test]
fn the_obj_bind_method_binds_the_method_of_the_object() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nx := f(8)"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 50);
}

/// The value of a binding is of the class `BoundFunc`, and its name is empty.
#[test]
fn the_value_of_a_binding_is_of_the_class_bound_func() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nthe_class := f.__Class\nthe_name := f.Name\nthe_minimum := f.MinParams"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "the_class"),
        Value::Text("BoundFunc".into())
    );
    assert_eq!(value_of(&report, "the_name"), Value::Text("".into()));
    assert_eq!(int_of(&report, "the_minimum"), 0);
}

/// The name of the method is not checked when the binding is made.
#[test]
fn the_obj_bind_method_does_not_check_the_name_of_the_method() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Nope\")"
    ));
    assert!(report.ok, "{:?}", report.error);

    // The failure comes when the value is called.
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Nope\")\nf()"
    ));
    assert_eq!(
        failure_of(&report),
        "This value of type \"Thing\" has no method named \"Nope\"."
    );
}

/// The call of a binding may go through the `Call` method of it.
#[test]
fn the_call_of_a_binding_may_go_through_the_call_method() {
    let report = run(&format!(
        "{THE_THING}t := Thing()\nf := ObjBindMethod(t, \"Method\")\nx := f.Call(8)"
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 50);
}

/// The binding of a method of a class type of the runtime.
#[test]
fn the_obj_bind_method_binds_the_method_of_a_class_type() {
    let report =
        run("m := Map()\nm[\"a\"] := 1\nf := ObjBindMethod(m, \"Clear\")\nf()\nn := m.Count");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "n"), 0);
}

// -- the processes -----------------------------------------------------------------
//
// The wording of v2 was measured on the binary of v2.0.19 on 2026-11-09: a
// process that is not there answers the zero to `ProcessExist`,
// `ProcessGetParent` and `ProcessClose`, and the failure "Target process not
// found." to `ProcessGetName`, `ProcessGetPath` and `ProcessSetPriority`.

#[test]
fn process_exist_answers_the_id_of_a_process() {
    let report = run("x := ProcessExist(42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
}

#[test]
fn process_exist_with_no_argument_asks_about_the_script_itself() {
    let mut desktop = Fake::default();
    let report = run_on("x := ProcessExist()", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 7);
    assert_eq!(
        desktop.calls,
        vec![("process-exist".to_string(), String::new())]
    );
}

#[test]
fn process_exist_answers_zero_when_the_process_is_not_there() {
    let report = run("x := ProcessExist(999)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn process_names_may_be_given_where_a_pid_is() {
    let report = run("x := ProcessExist(\"ahk\")\ny := ProcessGetName(\"ahk\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
    assert_eq!(report.variables.get("y"), Some(&Value::Text("ahk".into())));
}

#[test]
fn process_get_name_answers_the_name_of_the_image() {
    let report = run("x := ProcessGetName(42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("ahk".into())));
}

#[test]
fn process_get_name_of_a_process_that_is_not_there_fails() {
    let report = run("x := ProcessGetName(999)");
    assert_eq!(failure_of(&report), "Target process not found.");
}

#[test]
fn process_get_path_answers_the_path_of_the_image() {
    let report = run("x := ProcessGetPath(42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("x"),
        Some(&Value::Text("/bin/ahk".into()))
    );
}

#[test]
fn process_get_path_of_a_process_that_is_not_there_fails() {
    let report = run("x := ProcessGetPath(999)");
    assert_eq!(failure_of(&report), "Target process not found.");
}

#[test]
fn process_get_parent_answers_the_parent() {
    let report = run("x := ProcessGetParent(42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn process_get_parent_of_a_process_that_is_not_there_is_zero() {
    let report = run("x := ProcessGetParent(999)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn process_close_answers_the_id_of_the_process_it_stopped() {
    let report = run("x := ProcessClose(\"ahk\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
}

#[test]
fn process_close_of_a_process_that_is_not_there_is_zero() {
    let report = run("x := ProcessClose(999)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn process_set_priority_answers_the_id_of_the_process() {
    let mut desktop = Fake::default();
    let report = run_on("x := ProcessSetPriority(\"Low\", 42)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
    assert_eq!(
        desktop.calls,
        vec![("process-priority".to_string(), "Low|42".to_string())]
    );
}

#[test]
fn a_priority_that_v2_does_not_know_is_not_a_failure() {
    // Measured on the binary of v2.0.19 on 2026-11-09:
    // `ProcessSetPriority("Bogus", PID)` answered the id, so the word is not
    // checked.
    let report = run("x := ProcessSetPriority(\"Bogus\", 42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
}

#[test]
fn process_set_priority_of_a_process_that_is_not_there_fails() {
    let report = run("x := ProcessSetPriority(\"Low\", 999)");
    assert_eq!(failure_of(&report), "Target process not found.");
}

#[test]
fn process_wait_answers_the_id_when_the_process_is_there() {
    let report = run("x := ProcessWait(42)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
}

#[test]
fn process_wait_answers_zero_when_the_timeout_runs_out() {
    let report = run("x := ProcessWait(999, 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn process_wait_converts_the_seconds_of_v2_into_milliseconds() {
    let mut desktop = Fake::default();
    let report = run_on("x := ProcessWait(42, 0.5)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("process-wait".to_string(), "42|500".to_string())]
    );
}

#[test]
fn process_wait_without_a_timeout_waits_the_cap_and_not_forever() {
    // The binary of v2.0.19 waits forever here; this runtime does not,
    // because a script has to be stoppable. The cap is the one of `Sleep`.
    let mut desktop = Fake::default();
    let report = run_on("x := ProcessWait(42)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("process-wait".to_string(), "42|60000".to_string())]
    );
}

#[test]
fn process_wait_close_answers_zero_for_a_process_that_closed() {
    let report = run("x := ProcessWaitClose(999, 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn process_wait_close_answers_the_id_of_one_that_outlived_the_timeout() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the process that is
    // still there when the timeout runs out is the one the id answers.
    let report = run("x := ProcessWaitClose(42, 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 42);
}

// -- the notification --------------------------------------------------------------

#[test]
fn the_tray_tip_tells_the_user_something() {
    let mut desktop = Fake::default();
    let report = run_on("TrayTip(\"the text\", \"the title\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("notify".to_string(), "the title|the text".to_string())]
    );
}

#[test]
fn the_tray_tip_answers_nothing_the_script_reads() {
    let report = run("x := TrayTip(\"the text\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

// -- the listing of the windows ----------------------------------------------------
//
// The wording of v2 was measured on the binary of v2.0.19 on 2026-11-09:
// `WinGetID` and `WinGetClass` fail with "Target window not found." while
// `WinGetList` answers an empty array and `WinGetCount` answers the zero, and
// a call with no title at all asks about every window of the desktop.

#[test]
fn win_get_id_answers_the_identifier_of_the_window() {
    let report = run("x := WinGetID(\"first\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0x1".into())));
}

#[test]
fn win_get_id_of_a_window_that_is_not_there_fails() {
    let report = run("x := WinGetID(\"nothing\")");
    assert_eq!(failure_of(&report), "Target window not found.");
}

// -- the state that outlives a statement ------------------------------------------
//
// Two mechanisms of v2 are sets a script builds in one statement and reads in a
// later one. Measured on the binary of v2.0.19 on 2026-11-10: `GroupAdd`,
// `GroupDeactivate` and `GroupClose` answer the empty text, `GroupActivate`
// answers a whole number, and `WinGetIDLast` answers the window a `Win*` call
// last found.

#[test]
fn a_group_added_in_one_statement_is_read_in_a_later_one() {
    let report = run(concat!(
        "a := GroupAdd(\"mine\", \"first\")\n",
        "b := GroupActivate(\"mine\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Text(String::new())));
    assert_eq!(report.variables.get("b"), Some(&Value::Text("0x1".into())));
}

#[test]
fn a_group_that_was_not_added_answers_zero() {
    let report = run("x := GroupActivate(\"missing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn closing_a_group_takes_it_away() {
    let report = run(concat!(
        "GroupAdd(\"mine\", \"first\")\n",
        "GroupClose(\"mine\")\n",
        "x := GroupActivate(\"mine\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn the_last_window_is_the_one_a_wingetid_found() {
    let report = run(concat!(
        "a := WinGetID(\"first\")\n",
        "b := WinGetIDLast()\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("a"), Some(&Value::Text("0x1".into())));
    assert_eq!(report.variables.get("b"), Some(&Value::Text("0x1".into())));
}

#[test]
fn the_last_window_with_no_win_call_before_it_fails() {
    let report = run("x := WinGetIDLast()");
    assert_eq!(failure_of(&report), "Target window not found.");
}

// -- the clipboard watcher ---------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-10: `OnClipboardChange(Func, 1)`
// answers the empty text and the function is called with 1 when the script
// itself writes `A_Clipboard`; a second argument of 0 removes it.

#[test]
fn a_clipboard_change_calls_the_function_with_one() {
    let report = run(concat!(
        "Handler(Type) {\n    MsgBox \"called:\" Type\n}\n",
        "r := OnClipboardChange(Handler, 1)\n",
        "A_Clipboard := \"the new text\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("r"), Some(&Value::Text(String::new())));
    assert_eq!(report.output, vec!["called:1"]);
}

#[test]
fn a_clipboard_change_that_is_removed_is_not_called() {
    let report = run(concat!(
        "Handler(Type) {\n    MsgBox \"called:\" Type\n}\n",
        "OnClipboardChange(Handler, 1)\n",
        "OnClipboardChange(Handler, 0)\n",
        "A_Clipboard := \"the new text\"\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert!(report.output.is_empty(), "{:?}", report.output);
}

#[test]
fn a_clipboard_change_wants_a_function_not_a_text() {
    let report = run("OnClipboardChange(\"Handler\", 1)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of OnClipboardChange requires an Object, but received a String."
    );
}

// -- the list-shaped control -------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-10: a DropDownList is a
// `ComboBox1`, `ControlGetItems` answers an array, `ControlAddItem` makes it
// longer, `ControlChooseIndex` and `ControlChooseString` choose, `GetChoice`
// and `GetIndex` read, `FindItem` finds an index and `DeleteItem` removes one.
// The fake desktop carries the one list-shaped control, so these cases run
// without a GTK helper.

fn a_fake_with_a_combo() -> Fake {
    Fake {
        combo_items: vec!["alpha".into(), "beta".into(), "gamma".into()],
        gui_open: true,
        status_text: "the status".into(),
        ..Default::default()
    }
}

#[test]
fn control_get_items_answers_an_array_of_the_items() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on("x := ControlGetItems(\"ComboBox1\", \"the window\")\nn := x.Length\none := x[1]", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "n"), 3);
    assert_eq!(value_of(&report, "one"), Value::Text("alpha".into()));
}

#[test]
fn control_add_item_makes_the_array_one_longer() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "r := ControlAddItem(\"delta\", \"ComboBox1\", \"the window\")\nx := ControlGetItems(\"ComboBox1\", \"the window\")\nn := x.Length\nfour := x[4]",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "r"), Value::Text(String::new()));
    assert_eq!(int_of(&report, "n"), 4);
    assert_eq!(value_of(&report, "four"), Value::Text("delta".into()));
}

#[test]
fn control_choose_index_and_string_set_the_choice() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "ControlChooseIndex(2, \"ComboBox1\", \"the window\")\na := ControlGetChoice(\"ComboBox1\", \"the window\")\nb := ControlGetIndex(\"ComboBox1\", \"the window\")\nControlChooseString(\"gamma\", \"ComboBox1\", \"the window\")\nc := ControlGetIndex(\"ComboBox1\", \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text("beta".into()));
    assert_eq!(int_of(&report, "b"), 2);
    assert_eq!(int_of(&report, "c"), 3);
}

#[test]
fn control_find_item_and_delete_item() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "a := ControlFindItem(\"beta\", \"ComboBox1\", \"the window\")\nControlDeleteItem(1, \"ComboBox1\", \"the window\")\nx := ControlGetItems(\"ComboBox1\", \"the window\")\nn := x.Length\none := x[1]",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 2);
    assert_eq!(int_of(&report, "n"), 2);
    assert_eq!(value_of(&report, "one"), Value::Text("beta".into()));
}

#[test]
fn the_pop_up_and_the_status_bar_answer() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "a := ControlShowDropDown(\"ComboBox1\", \"the window\")\nb := ControlHideDropDown(\"ComboBox1\", \"the window\")\nc := StatusBarGetText(1, \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "a"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "b"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "c"), Value::Text("the status".into()));
}

#[test]
fn status_bar_wait_matches_a_case_sensitive_substring() {
    // Measured on the binary of v2.0.19 on 2026-11-10: "status" matches
    // "the status", a different case does not, and part 2 is refused.
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "a := StatusBarWait(\"status\", 0.2, 1, \"the window\")\nb := StatusBarWait(\"THE STATUS\", 0.2, 1, \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 0);
}

#[test]
fn status_bar_wait_of_a_part_that_is_not_one_is_refused() {
    let mut desktop = a_fake_with_a_combo();
    let report = run_on(
        "StatusBarWait(\"the status\", 0.2, 2, \"the window\")",
        &mut desktop,
    );
    assert_eq!(
        failure_of(&report),
        "Parameter #3 of StatusBarWait is invalid."
    );
}

// -- the list view -----------------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-10: `ListViewGetContent` answers
// the cells separated by tabs and the rows by line breaks, and the "Selected"
// option limits it to the row that is chosen.

fn a_fake_with_a_list_view() -> Fake {
    Fake {
        gui_open: true,
        list_rows: vec![
            vec!["alpha".into(), "10".into()],
            vec!["beta".into(), "20".into()],
        ],
        ..Default::default()
    }
}

#[test]
fn list_view_get_content_answers_the_rows_by_tabs_and_breaks() {
    let mut desktop = a_fake_with_a_list_view();
    let report = run_on(
        "c := ListViewGetContent(\"\", \"SysListView321\", \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "c"),
        Value::Text("alpha\t10\nbeta\t20".into())
    );
}

#[test]
fn list_view_get_content_of_the_selected_row() {
    let mut desktop = a_fake_with_a_list_view();
    desktop.list_selected = 1;
    let report = run_on(
        "c := ListViewGetContent(\"Selected\", \"SysListView321\", \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "c"), Value::Text("alpha\t10".into()));
}

// -- the tree view -----------------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-10: a TreeView is
// `SysTreeView321`, `tv.Add(text, parent)` answers an item id, and the Get*
// family walks the tree by those ids.

#[test]
fn an_up_down_shares_its_integer_value_with_the_buddy_edit() {
    // Measured on the binary of v2.0.19 on 2026-11-10: an UpDown is
    // `msctls_updown321`, its `Value` is an Integer, and setting it changes
    // the buddy Edit's text (and the other way round).
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\ne := g.Add(\"Edit\", \"\", \"5\")\nud := g.Add(\"UpDown\", \"Range1-10\")\ncls := ud.ClassNN\na := ud.Value\nud.Value := 7\nb := ud.Value\nc := e.Value\ne.Value := 3\nd := ud.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("msctls_updown321".into()));
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 7);
    assert_eq!(value_of(&report, "c"), Value::Text("7".into()));
    assert_eq!(int_of(&report, "d"), 3);
}

#[test]
fn file_recycle_answers_the_empty_text_and_remembers_the_path() {
    // Measured on the binary of v2.0.19 on 2026-11-10: FileRecycle answers
    // the empty text and the file is gone. The earlier claim that the
    // reference answers "Failed" was a path that was not there.
    let mut desktop = Fake::default();
    let report = run_on("r := FileRecycle(\"the-file.txt\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "r"), Value::Text(String::new()));
    assert_eq!(desktop.recycled, vec!["the-file.txt".to_string()]);
}

#[test]
fn file_get_shortcut_answers_the_fields_of_the_shortcut() {
    // Measured on the binary of v2.0.19 on 2026-11-10: all eight fields of a
    // shortcut FileCreateShortcut made come back. This tree reads the
    // .desktop file it wrote; the icon number and the run state have no
    // counterpart there, so those two answers are the empty text.
    let mut desktop = Fake {
        gui_open: true,
        shortcut: Some((
            "/usr/bin/zenity".into(),
            "/tmp".into(),
            "--info".into(),
            "a desc".into(),
            String::new(),
        )),
        ..Default::default()
    };
    let report = run_on(
        "FileGetShortcut \"the.desktop\", &t, &d, &a, &s\nFileGetShortcut \"the.desktop\", &t2, &d2, &a2, &s2, &i2, &n2, &r2",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "t"), Value::Text("/usr/bin/zenity".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("/tmp".into()));
    assert_eq!(value_of(&report, "a"), Value::Text("--info".into()));
    assert_eq!(value_of(&report, "s"), Value::Text("a desc".into()));
    assert_eq!(value_of(&report, "i2"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "n2"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "r2"), Value::Text(String::new()));
}

#[test]
fn a_control_get_pos_writes_the_four_places_of_a_control() {
    // Measured on the binary of v2.0.19 on 2026-11-10: ControlGetPos with
    // four VarRef outputs writes x, y, width and height relative to the
    // client area, and answers the empty text. This is the GTK allocation of
    // the control in the helper's box.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "x := y := w := h := 0\ng := Gui()\ng.Add(\"Edit\", \"\", \"x\")\nr := ControlGetPos(&x, &y, &w, &h, \"Edit1\", \"the window\")",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "r"), Value::Text(String::new()));
    assert_eq!(int_of(&report, "x"), 10);
    assert_eq!(int_of(&report, "y"), 20);
    assert_eq!(int_of(&report, "w"), 100);
    assert_eq!(int_of(&report, "h"), 30);
}

#[test]
fn a_link_has_a_read_only_empty_value_and_a_writable_text() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a Link is `SysLink1`,
    // its `Value` is always empty and writing it is refused with
    // "Invalid usage.", and `Text` is its caption, read and written.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\nlk := g.Add(\"Link\", \"\", \"click <a>here</a>\")\ncls := lk.ClassNN\nv := lk.Value\nt := lk.Text\nlk.Text := \"third <a>two</a>\"\nafter := lk.Text",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("SysLink1".into()));
    assert_eq!(value_of(&report, "v"), Value::Text(String::new()));
    assert_eq!(
        value_of(&report, "t"),
        Value::Text("click <a>here</a>".into())
    );
    assert_eq!(value_of(&report, "after"), Value::Text("third <a>two</a>".into()));

    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on("g := Gui()\nlk := g.Add(\"Link\", \"\", \"x\")\nlk.Value := \"y\"", &mut desktop);
    assert_eq!(failure_of(&report), "Invalid usage.");
}

#[test]
fn a_tab_holds_the_page_number_and_refuses_a_page_that_is_not_there() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a Tab is
    // `SysTabControl321`, `Value` is the 1-based page number, and a page that
    // is not there is refused with "Invalid value.", not clamped.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\ntab := g.Add(\"Tab\", \"w400 h300\", [\"First\", \"Second\"])\ncls := tab.ClassNN\na := tab.Value\ntab.Value := 2\nb := tab.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("SysTabControl321".into()));
    assert_eq!(int_of(&report, "a"), 1);
    assert_eq!(int_of(&report, "b"), 2);

    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\ntab := g.Add(\"Tab\", \"w400 h300\", [\"First\", \"Second\"])\ntab.Value := 9",
        &mut desktop,
    );
    assert!(!report.ok);
    assert_eq!(failure_of(&report), "Invalid value.");
}

#[test]
fn a_pic_is_a_static_whose_value_is_the_file_and_writes_are_ignored() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a Pic shares the
    // Static counter with Text, `Value` and `Text` answer the file it shows,
    // and writing `Value` answers ok and changes NOTHING.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\np := g.Add(\"Pic\", \"w100 h50\", \"the-image.png\")\ncls := p.ClassNN\na := p.Value\nb := p.Text\np.Value := \"other.png\"\nc := p.Value\nd := p.Text",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("Static1".into()));
    assert_eq!(value_of(&report, "a"), Value::Text("the-image.png".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("the-image.png".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("the-image.png".into()));
    assert_eq!(value_of(&report, "d"), Value::Text("the-image.png".into()));
}

#[test]
fn a_hotkey_keeps_its_value_and_text_apart() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a Hotkey is
    // `msctls_hotkey321`, `Value` is the hotkey string, `Text` is a SECOND
    // independent string, and writing either leaves the other alone.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\nhk := g.Add(\"Hotkey\", \"\", \"^a\")\ncls := hk.ClassNN\nv := hk.Value\nt := hk.Text\nhk.Value := \"!b\"\nhk.Text := \"xyz\"\nv2 := hk.Value\nt2 := hk.Text\nhk.Value := \"!c\"\nv3 := hk.Value\nt3 := hk.Text",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("msctls_hotkey321".into()));
    assert_eq!(value_of(&report, "v"), Value::Text("^a".into()));
    assert_eq!(value_of(&report, "t"), Value::Text(String::new()));
    assert_eq!(value_of(&report, "v2"), Value::Text("!b".into()));
    assert_eq!(value_of(&report, "t2"), Value::Text("xyz".into()));
    assert_eq!(value_of(&report, "v3"), Value::Text("!c".into()));
    assert_eq!(value_of(&report, "t3"), Value::Text("xyz".into()));
}

#[test]
fn a_date_time_and_a_month_cal_hold_timestamp_strings() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a DateTime is
    // `SysDateTimePick321` with a fourteen-digit Value, a MonthCal is
    // `SysMonthCal321` with an eight-digit one, both Strings, and a short
    // write is padded with zeros.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\ndt := g.Add(\"DateTime\", \"\", \"20261110120000\")\ncls := dt.ClassNN\na := dt.Value\nb := Type(dt.Value)\ndt.Value := \"20260101\"\nc := dt.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        value_of(&report, "cls"),
        Value::Text("SysDateTimePick321".into())
    );
    assert_eq!(value_of(&report, "a"), Value::Text("20261110120000".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("String".into()));
    assert_eq!(value_of(&report, "c"), Value::Text("20260101000000".into()));

    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\nmc := g.Add(\"MonthCal\", \"\", \"20261110\")\ncls := mc.ClassNN\na := mc.Value\nmc.Value := \"20261231\"\nb := mc.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("SysMonthCal321".into()));
    assert_eq!(value_of(&report, "a"), Value::Text("20261110".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("20261231".into()));
}

#[test]
fn a_slider_and_a_progress_hold_integer_values() {
    // Measured on the binary of v2.0.19 on 2026-11-10: a Slider is
    // `msctls_trackbar321`, a Progress is `msctls_progress321`, both have an
    // Integer `Value` that starts at the number in the text, and setting it
    // changes what is read back.
    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\nsl := g.Add(\"Slider\", \"Range0-100\", \"50\")\ncls := sl.ClassNN\na := sl.Value\nb := Type(sl.Value)\nsl.Value := 75\nc := sl.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("msctls_trackbar321".into()));
    assert_eq!(int_of(&report, "a"), 50);
    assert_eq!(value_of(&report, "b"), Value::Text("Integer".into()));
    assert_eq!(int_of(&report, "c"), 75);

    let mut desktop = Fake {
        gui_open: true,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\npr := g.Add(\"Progress\", \"Range0-100\", \"40\")\ncls := pr.ClassNN\na := pr.Value\npr.Value := 80\nb := pr.Value",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(value_of(&report, "cls"), Value::Text("msctls_progress321".into()));
    assert_eq!(int_of(&report, "a"), 40);
    assert_eq!(int_of(&report, "b"), 80);
}

#[test]
fn a_tree_view_is_built_and_walked_by_its_items() {
    let mut desktop = Fake {
        gui_open: true,
        tree_next: 1,
        ..Default::default()
    };
    let report = run_on(
        "g := Gui()\ntv := g.Add(\"TreeView\", \"w400\")\np1 := tv.Add(\"Parent one\")\np2 := tv.Add(\"Parent two\")\nc1 := tv.Add(\"Child one\", p1)\nn := tv.GetCount()\nt := tv.GetText(p1)\nparent := tv.GetParent(c1)\nchild := tv.GetChild(p1)\nnext := tv.GetNext(p1)\nprev := tv.GetPrev(p2)\ntv.Modify(p1, \"Select\")\nsel := tv.GetSelection()\ntv.Delete(c1)\nafter := tv.GetCount()",
        &mut desktop,
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "p1"), 1);
    assert_eq!(int_of(&report, "p2"), 2);
    assert_eq!(int_of(&report, "c1"), 3);
    assert_eq!(int_of(&report, "n"), 3);
    assert_eq!(value_of(&report, "t"), Value::Text("Parent one".into()));
    assert_eq!(int_of(&report, "parent"), 1);
    assert_eq!(int_of(&report, "child"), 3);
    assert_eq!(int_of(&report, "next"), 2);
    assert_eq!(int_of(&report, "prev"), 1);
    assert_eq!(int_of(&report, "sel"), 1);
    assert_eq!(int_of(&report, "after"), 2);
}

#[test]
fn win_get_class_answers_the_class_of_the_window() {
    let report = run("x := WinGetClass(\"second\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        report.variables.get("x"),
        Some(&Value::Text("second".into()))
    );
}

#[test]
fn win_get_class_of_a_window_that_is_not_there_fails() {
    let report = run("x := WinGetClass(\"nothing\")");
    assert_eq!(failure_of(&report), "Target window not found.");
}

#[test]
fn win_get_count_counts_the_windows_that_match() {
    let report =
        run("a := WinGetCount()\nb := WinGetCount(\"first\")\nc := WinGetCount(\"nothing\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 2);
    assert_eq!(int_of(&report, "b"), 1);
    assert_eq!(int_of(&report, "c"), 0);
}

#[test]
fn win_get_list_of_a_title_that_matches_nothing_is_empty() {
    // The answer is an array of v2 and not a bare list: the `Length` of it is
    // the number of the windows, which is what a script reads.
    let report = run("x := WinGetList(\"nothing\")\nn := x.Length");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "n"), 0);
}

#[test]
fn win_get_list_answers_every_window_with_an_empty_title() {
    let report = run("x := WinGetList()\nn := x.Length\nfirst := x[1]");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "n"), 2);
    assert_eq!(
        report.variables.get("first"),
        Some(&Value::Text("0x1".into()))
    );
}

// -- where a window is -------------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09: `WinGetPos` writes its four
// answers into four variables given by reference and answers the empty text, a
// window that is not there is the failure "Target window not found.", and a
// value where a variable is wanted is refused with the `VarRef` message.

#[test]
fn win_get_pos_writes_the_four_places_into_the_references() {
    let report = run("x := y := w := h := 0\nWinGetPos(&x, &y, &w, &h, \"first\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 10);
    assert_eq!(int_of(&report, "y"), 20);
    assert_eq!(int_of(&report, "w"), 100);
    assert_eq!(int_of(&report, "h"), 200);
}

#[test]
fn win_get_pos_answers_the_empty_text() {
    let report = run("x := y := w := h := 0\nr := WinGetPos(&x, &y, &w, &h, \"first\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("r"), Some(&Value::Text(String::new())));
}

#[test]
fn win_get_pos_of_a_window_that_is_not_there_fails() {
    let report = run("x := y := w := h := 0\nWinGetPos(&x, &y, &w, &h, \"nothing\")");
    assert_eq!(failure_of(&report), "Target window not found.");
}

#[test]
fn win_get_pos_with_no_window_fails_like_v2_with_no_last_found_window() {
    let report = run("x := y := w := h := 0\nWinGetPos(&x, &y, &w, &h)");
    assert_eq!(failure_of(&report), "Target window not found.");
}

#[test]
fn win_get_pos_needs_a_variable_and_not_a_value() {
    let report = run("WinGetPos(1, &y, &w, &h, \"first\")");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of WinGetPos requires a VarRef, but received an Integer."
    );
}

#[test]
fn win_get_client_pos_writes_the_four_places_too() {
    let report = run("x := y := w := h := 0\nWinGetClientPos(&x, &y, &w, &h, \"second\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 30);
    assert_eq!(int_of(&report, "w"), 300);
    assert_eq!(int_of(&report, "h"), 400);
}

// -- whether a window is maximized -------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09, on a window of its own:
// `0` before, `1` after `WinMaximize`, `0` after `WinRestore`, `-1` after
// `WinMinimize`.

#[test]
fn win_get_min_max_answers_zero_for_an_ordinary_window() {
    let report = run("x := WinGetMinMax(\"first\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn win_get_min_max_answers_one_for_a_maximized_window() {
    let report = run("x := WinGetMinMax(\"second\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn win_get_min_max_answers_minus_one_for_a_minimized_window() {
    let report = run("x := WinGetMinMax(\"minimized\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), -1);
}

#[test]
fn win_get_min_max_of_a_window_that_is_not_there_fails() {
    let report = run("x := WinGetMinMax(\"nothing\")");
    assert_eq!(failure_of(&report), "Target window not found.");
}

#[test]
fn win_get_min_max_with_no_window_fails() {
    let report = run("x := WinGetMinMax()");
    assert_eq!(failure_of(&report), "Target window not found.");
}

// -- waiting for a window ----------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09: `WinWait` and
// `WinWaitActive` answer the identifier or the zero; `WinWaitClose` and
// `WinWaitNotActive` answer `1` and `0`; and the conditions about the absence
// of a window hold at once when no window matches.

#[test]
fn win_wait_answers_the_identifier_of_the_window() {
    let report = run("x := WinWait(\"first\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("0x1".into())));
}

#[test]
fn win_wait_answers_zero_when_the_timeout_runs_out() {
    let report = run("x := WinWait(\"nothing\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn win_wait_active_answers_zero_when_no_window_has_the_focus() {
    let report = run("x := WinWaitActive(\"nothing\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn win_wait_not_active_answers_one_at_once_when_no_window_matches() {
    let report = run("x := WinWaitNotActive(\"nothing\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn win_wait_close_answers_one_when_the_window_is_gone() {
    let report = run("x := WinWaitClose(\"nothing\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn win_wait_close_answers_zero_while_the_window_is_there() {
    let report = run("x := WinWaitClose(\"first\", , 0.05)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 0);
}

#[test]
fn win_wait_converts_the_seconds_of_v2_into_milliseconds() {
    let mut desktop = Fake::default();
    let report = run_on("WinWait(\"first\", , 0.5)", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("wait-window".to_string(), "first|Present|500".to_string())]
    );
}

#[test]
fn win_wait_without_seconds_waits_the_cap_and_not_forever() {
    let mut desktop = Fake::default();
    let report = run_on("WinWait(\"first\")", &mut desktop);
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(
        desktop.calls,
        vec![("wait-window".to_string(), "first|Present|60000".to_string())]
    );
}

// -- the drives --------------------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetSpaceFree` and
// `DriveGetCapacity` answer whole megabytes, a drive that is there is "Ready",
// and a drive that is not there is a failure for the space and the empty text
// for the status.

#[test]
fn drive_get_space_free_answers_the_free_megabytes() {
    let report = run("x := DriveGetSpaceFree(\"/\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1000);
}

#[test]
fn drive_get_capacity_answers_the_capacity_in_megabytes() {
    let report = run("x := DriveGetCapacity(\"/\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 4000);
}

#[test]
fn drive_get_space_free_of_a_drive_that_is_not_there_fails() {
    let report = run("x := DriveGetSpaceFree(\"/nowhere\")");
    assert!(!report.ok, "the script was expected to fail");
    assert!(failure_of(&report).contains("is not there"));
}

#[test]
fn drive_get_status_answers_ready_for_a_drive_that_is_there() {
    let report = run("x := DriveGetStatus(\"/\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Ready".into())));
}

#[test]
fn drive_get_status_of_a_drive_that_is_not_there_is_empty() {
    let report = run("x := DriveGetStatus(\"/nowhere\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

#[test]
fn drive_get_filesystem_answers_the_filesystem_of_the_machine() {
    let report = run("x := DriveGetFilesystem(\"/\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("ext4".into())));
}

#[test]
fn drive_get_type_answers_the_kind_in_the_word_of_v2() {
    let report = run("x := DriveGetType(\"/\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("Fixed".into())));
}

#[test]
fn drive_get_type_of_a_drive_that_is_not_there_is_empty() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `DriveGetType` of a
    // drive that is not there is the empty text, and not a failure.
    let report = run("x := DriveGetType(\"/nowhere\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text(String::new())));
}

// -- the monitors ------------------------------------------------------------------
//
// Measured on the binary of v2.0.19 on 2026-11-09: the monitors are numbered
// from one, a number that names none is "Parameter #1 of MonitorGetName is
// invalid.", and `MonitorGet` answers 1 and writes the four bounds.

#[test]
fn monitor_get_count_answers_the_number_of_monitors() {
    let report = run("x := MonitorGetCount()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn monitor_get_name_answers_the_name_of_the_monitor() {
    let report = run("x := MonitorGetName(1)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.variables.get("x"), Some(&Value::Text("MON1".into())));
}

#[test]
fn monitor_get_name_of_a_monitor_that_is_not_there_fails() {
    let report = run("x := MonitorGetName(9)");
    assert_eq!(
        failure_of(&report),
        "Parameter #1 of MonitorGetName is invalid."
    );
}

#[test]
fn monitor_get_writes_the_four_bounds() {
    let report = run("l := t := r := b := 0\nMonitorGet(1, &l, &t, &r, &b)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "l"), 0);
    assert_eq!(int_of(&report, "t"), 0);
    assert_eq!(int_of(&report, "r"), 1920);
    assert_eq!(int_of(&report, "b"), 1080);
}

#[test]
fn monitor_get_answers_one() {
    let report = run("l := t := r := b := 0\nx := MonitorGet(1, &l, &t, &r, &b)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn monitor_get_primary_answers_the_first_monitor() {
    let report = run("x := MonitorGetPrimary()");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "x"), 1);
}

#[test]
fn the_work_area_is_the_monitor_when_the_panels_are_not_known() {
    let report = run("l := t := r := b := 0\nMonitorGetWorkArea(1, &l, &t, &r, &b)");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "r"), 1920);
}

// -- the bytes of a text ------------------------------------------------------------

#[test]
fn the_bytes_of_a_text_are_the_ones_v2_counts() {
    // Measured on the binary of v2.0.19 on 2026-11-09: the default encoding is
    // the UTF-16 of the machine, so "hello" is twelve bytes -- five of two and
    // the two of the terminator -- and the UTF-8 of it is six. The accent of
    // "héllo" is two bytes, which makes seven.
    let report = run(
        "a := StrPut(\"hello\")\n\
         b := StrPut(\"hello\", \"UTF-8\")\n\
         c := StrPut(\"hello\", \"UTF-16\")\n\
         d := StrPut(\"héllo\", \"UTF-8\")",
    );
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "a"), 12);
    assert_eq!(int_of(&report, "b"), 6);
    assert_eq!(int_of(&report, "c"), 12);
    assert_eq!(int_of(&report, "d"), 7);
}

#[test]
fn an_encoding_that_v2_does_not_know_is_refused() {
    let report = run("x := StrPut(\"hello\", \"Bogus\")");
    assert_eq!(failure_of(&report), "Invalid Encoding.");
}

#[test]
fn str_put_writes_a_buffer_and_str_get_reads_it_back() {
    // Measured on the binary of v2.0.19 on 2026-11-10: StrPut("hello", buf)
    // answers 12 (the UTF-16 bytes with the terminator), StrGet(buf.Ptr)
    // answers "hello", and a length of 3 answers "hel". A length of zero
    // answers the empty text.
    let report = run(concat!(
        "buf := Buffer(64)\n",
        "written := StrPut(\"hello\", buf)\n",
        "a := StrGet(buf.Ptr)\n",
        "b := StrGet(buf.Ptr, 3)\n",
        "c := StrGet(buf.Ptr, 0)\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "written"), 12);
    assert_eq!(value_of(&report, "a"), Value::Text("hello".into()));
    assert_eq!(value_of(&report, "b"), Value::Text("hel".into()));
    assert_eq!(value_of(&report, "c"), Value::Text(String::new()));
}

#[test]
fn str_put_writes_utf8_and_str_get_reads_it_with_the_encoding() {
    // Measured on the binary of v2.0.19 on 2026-11-10: StrGet with an
    // explicit length of zero answers the empty text -- a length is a count
    // of characters, and zero of them is nothing.
    let report = run(concat!(
        "buf := Buffer(64)\n",
        "written := StrPut(\"abc\", buf, , \"UTF-8\")\n",
        "a := StrGet(buf.Ptr, 3, \"UTF-8\")\n",
        "b := StrGet(buf.Ptr, 0, \"UTF-8\")\n",
    ));
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(int_of(&report, "written"), 4);
    assert_eq!(value_of(&report, "a"), Value::Text("abc".into()));
    assert_eq!(value_of(&report, "b"), Value::Text(String::new()));
}

#[test]
fn str_get_of_an_address_no_buffer_answered_fails() {
    let report = run("x := StrGet(99)");
    assert_eq!(
        failure_of(&report),
        "the address 99 does not name a buffer this script has asked about"
    );
}

#[test]
fn the_output_debug_goes_to_the_output_and_answers_nothing() {
    // Measured on the binary of v2.0.19 on 2026-11-09: `OutputDebug` answers
    // the empty text. This tree has no debugger, and one channel for what a
    // script says -- the output of the report.
    let report = run("r := OutputDebug(\"a log line\")");
    assert!(report.ok, "{:?}", report.error);
    assert_eq!(report.output, vec!["a log line"]);
    assert_eq!(report.variables.get("r"), Some(&Value::Text(String::new())));
}
