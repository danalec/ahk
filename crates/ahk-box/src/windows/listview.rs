//! The list view of the machine: the `SysListView32` control, the rows
//! and the columns of it, and the messages of the `LVM` family.
//!
//! The rows and the columns of the v1 are counted from one; the machine
//! counts them from zero, and the translation is here.

use std::cell::RefCell;
use std::collections::BTreeMap;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::UI::Controls::{
    HDM_GETITEMCOUNT, LVCFMT_CENTER, LVCFMT_LEFT, LVCFMT_RIGHT, LVCF_FMT, LVCF_TEXT, LVCF_WIDTH,
    LVCOLUMNW, LVIF_IMAGE, LVIF_STATE, LVIF_TEXT, LVIS_FOCUSED, LVIS_SELECTED,
    LVIS_STATEIMAGEMASK, LVITEMW, LVM_DELETEALLITEMS, LVM_DELETEITEM, LVM_ENSUREVISIBLE,
    LVM_GETHEADER, LVM_GETITEMCOUNT, LVM_GETITEMTEXTW, LVM_GETNEXTITEM, LVM_GETSELECTEDCOUNT,
    LVM_GETTOPINDEX, LVM_INSERTCOLUMNW, LVM_INSERTITEMW, LVM_SETCOLUMNW, LVM_SETCOLUMNWIDTH,
    LVM_SETIMAGELIST, LVM_SETITEMSTATE, LVM_SETITEMTEXTW, LVM_SORTITEMS, LVNI_ALL, LVNI_FOCUSED,
    LVNI_SELECTED, LVSIL_NORMAL, LVSIL_SMALL,
};
use windows_sys::Win32::UI::WindowsAndMessaging::SendMessageW;

use super::gui::TheControl;
use super::sys::Wide;

/// The state image of a checked row: the index one of the state image
/// list, shifted into the mask the machine reads.
const CHECKED: u32 = 0x1000;
const UNCHECKED: u32 = 0x2000;

/// The text of one cell. The machine writes it into a buffer of ours.
pub fn text(control: &TheControl, row: i64, col: i64) -> String {
    let mut buffer = vec![0u16; 32768];
    let mut item: LVITEMW = unsafe { std::mem::zeroed() };
    item.iSubItem = col.saturating_sub(1) as i32;
    item.pszText = buffer.as_mut_ptr();
    item.cchTextMax = buffer.len() as i32;
    let read = unsafe {
        SendMessageW(
            control.handle,
            LVM_GETITEMTEXTW,
            row.saturating_sub(1) as WPARAM,
            &mut item as *mut LVITEMW as LPARAM,
        )
    };
    if read <= 0 {
        return String::new();
    }
    String::from_utf16_lossy(&buffer[..read as usize])
}

/// The number of rows.
pub fn count(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, LVM_GETITEMCOUNT, 0, 0) as i64 }
}

/// The number of columns, from the header of the control.
pub fn column_count(control: &TheControl) -> i64 {
    unsafe {
        let header = SendMessageW(control.handle, LVM_GETHEADER, 0, 0) as HWND;
        if header.is_null() {
            return 0;
        }
        SendMessageW(header, HDM_GETITEMCOUNT, 0, 0) as i64
    }
}

/// The number of rows that are chosen.
pub fn selected_count(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, LVM_GETSELECTEDCOUNT, 0, 0) as i64 }
}

/// The first row that is seen.
pub fn first_visible(control: &TheControl) -> i64 {
    unsafe { SendMessageW(control.handle, LVM_GETTOPINDEX, 0, 0) as i64 + 1 }
}

/// The options of a row: the words the v1 writes, and the state and the
/// image they ask for.
#[derive(Default)]
struct RowOptions {
    state: u32,
    state_mask: u32,
    image: Option<i32>,
    ensure_visible: bool,
    /// The column the texts that come after the options begin at.
    start_column: Option<i64>,
}

/// The words of the options of a row. A word that is not one of them is
/// not an option: the caller reads it as the text of a column.
fn the_row_options(options: &str) -> RowOptions {
    let mut out = RowOptions::default();
    for word in options.split_whitespace() {
        let lower = word.to_ascii_lowercase();
        let (name, on) = match lower.strip_prefix('-') {
            Some(rest) => (rest.to_string(), false),
            None => (lower.clone(), true),
        };
        let plus_name = name.strip_prefix('+').map(str::to_string);
        let name = plus_name.as_deref().unwrap_or(&name);
        match name {
            "check" => {
                out.state_mask |= LVIS_STATEIMAGEMASK;
                out.state |= if on { CHECKED } else { UNCHECKED };
            }
            "focus" => {
                out.state_mask |= LVIS_FOCUSED;
                if on {
                    out.state |= LVIS_FOCUSED;
                }
            }
            "select" => {
                out.state_mask |= LVIS_SELECTED;
                if on {
                    out.state |= LVIS_SELECTED;
                }
            }
            "vis" => out.ensure_visible = on,
            _ => {
                if let Some(number) = name.strip_prefix("icon") {
                    if let Ok(number) = number.parse::<i32>() {
                        out.image = Some(number.saturating_sub(1));
                    }
                } else if let Some(number) = name.strip_prefix("col") {
                    if let Ok(number) = number.parse::<i64>() {
                        out.start_column = Some(number);
                    }
                }
            }
        }
    }
    out
}

/// Whether the text is the options of a row and not the text of a
/// column: every word of it is a word the v1 knows.
pub fn the_text_is_row_options(text: &str) -> bool {
    if text.trim().is_empty() {
        return true;
    }
    text.split_whitespace().all(|word| {
        let lower = word.to_ascii_lowercase();
        let name = lower
            .trim_start_matches('-')
            .trim_start_matches('+');
        matches!(name, "check" | "focus" | "select" | "vis")
            || name.strip_prefix("icon").is_some_and(|n| n.parse::<i32>().is_ok())
            || name.strip_prefix("col").is_some_and(|n| n.parse::<i64>().is_ok())
    })
}

/// The state of a row is written.
fn the_state_is_written(control: &TheControl, row: i64, options: &RowOptions) {
    if options.state_mask == 0 {
        return;
    }
    let mut item: LVITEMW = unsafe { std::mem::zeroed() };
    item.mask = LVIF_STATE;
    item.state = options.state;
    item.stateMask = options.state_mask;
    unsafe {
        SendMessageW(
            control.handle,
            LVM_SETITEMSTATE,
            row.saturating_sub(1) as WPARAM,
            &mut item as *mut LVITEMW as LPARAM,
        );
    }
}

/// The text of one cell is written.
fn the_cell_is_written(control: &TheControl, row: i64, col: i64, text: &str) {
    let wide = Wide::of(text);
    let mut item: LVITEMW = unsafe { std::mem::zeroed() };
    item.mask = LVIF_TEXT;
    item.iSubItem = col.saturating_sub(1) as i32;
    item.pszText = wide.as_ptr() as *mut u16;
    unsafe {
        SendMessageW(
            control.handle,
            LVM_SETITEMTEXTW,
            row.saturating_sub(1) as WPARAM,
            &mut item as *mut LVITEMW as LPARAM,
        );
    }
}

/// A row is added at the end, with the texts of its columns; the answer
/// is the number of the row, counted from one.
pub fn add(control: &TheControl, options: &str, cells: &[String]) -> Result<i64, String> {
    let opts = the_row_options(options);
    let first = cells.first().map(String::as_str).unwrap_or("");
    let wide = Wide::of(first);
    let row = count(control);
    let mut item: LVITEMW = unsafe { std::mem::zeroed() };
    item.mask = LVIF_TEXT;
    item.iItem = row as i32;
    item.iSubItem = 0;
    item.pszText = wide.as_ptr() as *mut u16;
    if let Some(image) = opts.image {
        item.mask |= LVIF_IMAGE;
        item.iImage = image;
    }
    if opts.state_mask != 0 {
        item.mask |= LVIF_STATE;
        item.state = opts.state;
        item.stateMask = opts.state_mask;
    }
    let added = unsafe {
        SendMessageW(
            control.handle,
            LVM_INSERTITEMW,
            0,
            &mut item as *mut LVITEMW as LPARAM,
        )
    } as i64;
    if added < 0 {
        return Err("the machine did not add the row to the list view.".to_string());
    }
    let start = opts.start_column.unwrap_or(1);
    for (index, cell) in cells.iter().enumerate().skip(1) {
        the_cell_is_written(control, added + 1, start + index as i64, cell);
    }
    if opts.ensure_visible {
        unsafe {
            SendMessageW(control.handle, LVM_ENSUREVISIBLE, added as WPARAM, 0);
        }
    }
    Ok(added + 1)
}

/// A row is removed; the row zero removes them all.
pub fn delete(control: &TheControl, row: i64) -> bool {
    let answer = unsafe {
        if row <= 0 {
            SendMessageW(control.handle, LVM_DELETEALLITEMS, 0, 0)
        } else {
            SendMessageW(
                control.handle,
                LVM_DELETEITEM,
                row.saturating_sub(1) as WPARAM,
                0,
            )
        }
    };
    answer != 0
}

/// The next row that is chosen, checked or focused, after the row the
/// caller names; the answer is zero when there is none.
pub fn next(control: &TheControl, start: i64, mode: &str) -> i64 {
    let mode = mode.trim().to_ascii_lowercase();
    let checked = mode == "c" || mode == "checked";
    let flags = if mode == "f" || mode == "focused" {
        LVNI_FOCUSED
    } else if checked {
        LVNI_ALL
    } else {
        LVNI_SELECTED
    };
    let mut row = start.saturating_sub(1);
    loop {
        let found = unsafe {
            SendMessageW(
                control.handle,
                LVM_GETNEXTITEM,
                row as WPARAM,
                flags as LPARAM,
            )
        } as i64;
        if found < 0 {
            return 0;
        }
        if checked {
            let state = unsafe {
                SendMessageW(
                    control.handle,
                    windows_sys::Win32::UI::Controls::LVM_GETITEMSTATE,
                    found as WPARAM,
                    LVIS_STATEIMAGEMASK as LPARAM,
                ) as u32
            };
            if state & LVIS_STATEIMAGEMASK == CHECKED {
                return found as i64 + 1;
            }
            row = found;
            continue;
        }
        return found as i64 + 1;
    }
}

/// A row is changed: the options of it, and the texts of the columns.
/// The row zero changes them all.
pub fn modify(
    control: &TheControl,
    row: i64,
    options: &str,
    cells: &[String],
) -> bool {
    let opts = the_row_options(options);
    let rows: Vec<i64> = if row <= 0 {
        (1..=count(control)).collect()
    } else {
        vec![row]
    };
    let start = opts.start_column.unwrap_or(1);
    for row in rows {
        if opts.state_mask != 0 {
            the_state_is_written(control, row, &opts);
        }
        if let Some(image) = opts.image {
            let mut item: LVITEMW = unsafe { std::mem::zeroed() };
            item.mask = LVIF_IMAGE;
            item.iImage = image;
            unsafe {
                SendMessageW(
                    control.handle,
                    windows_sys::Win32::UI::Controls::LVM_SETITEMW,
                    row.saturating_sub(1) as WPARAM,
                    &mut item as *mut LVITEMW as LPARAM,
                );
            }
        }
        for (index, cell) in cells.iter().enumerate() {
            the_cell_is_written(control, row, start + index as i64, cell);
        }
        if opts.ensure_visible {
            unsafe {
                SendMessageW(
                    control.handle,
                    LVM_ENSUREVISIBLE,
                    row.saturating_sub(1) as WPARAM,
                    0,
                );
            }
        }
    }
    true
}

/// The sort of a column: the kind of the comparison the v1 names.
#[derive(Clone, Copy, PartialEq)]
enum SortKind {
    Text,
    Integer,
    Float,
}

thread_local! {
    /// The kind of the sort of each column, by the control and the column.
    static THE_SORT_KINDS: RefCell<BTreeMap<(isize, i64), SortKind>> = const { RefCell::new(BTreeMap::new()) };
}

/// The context of the comparison of the sort.
struct SortContext {
    handle: HWND,
    column: i64,
    descending: bool,
    kind: SortKind,
}

/// The comparison of two rows for the sort of the machine.
unsafe extern "system" fn the_sort_comparison(
    first: LPARAM,
    second: LPARAM,
    context: LPARAM,
) -> i32 {
    let context = unsafe { &*(context as *const SortContext) };
    let read = |row: LPARAM| -> String {
        let mut buffer = vec![0u16; 32768];
        let mut item: LVITEMW = unsafe { std::mem::zeroed() };
        item.iSubItem = context.column.saturating_sub(1) as i32;
        item.pszText = buffer.as_mut_ptr();
        item.cchTextMax = buffer.len() as i32;
        let read = unsafe {
            SendMessageW(
                context.handle,
                LVM_GETITEMTEXTW,
                row as WPARAM,
                &mut item as *mut LVITEMW as LPARAM,
            )
        };
        if read <= 0 {
            String::new()
        } else {
            String::from_utf16_lossy(&buffer[..read as usize])
        }
    };
    let one = read(first);
    let other = read(second);
    let order = match context.kind {
        SortKind::Integer => {
            let one = one.trim().parse::<i64>().unwrap_or(0);
            let other = other.trim().parse::<i64>().unwrap_or(0);
            one.cmp(&other)
        }
        SortKind::Float => {
            let one = one.trim().parse::<f64>().unwrap_or(0.0);
            let other = other.trim().parse::<f64>().unwrap_or(0.0);
            one.partial_cmp(&other).unwrap_or(std::cmp::Ordering::Equal)
        }
        SortKind::Text => one.to_lowercase().cmp(&other.to_lowercase()),
    };
    let mut answer = match order {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    };
    if context.descending {
        answer = -answer;
    }
    answer
}

/// A column is changed, or made when it is not there: the width, the
/// alignment, the title and the sort of it.
pub fn modify_col(control: &TheControl, col: i64, options: &str, title: Option<&str>) -> bool {
    let col = if col <= 0 { 1 } else { col };
    let existing = column_count(control);
    let mut width: Option<i32> = None;
    let mut fmt: Option<i32> = None;
    let mut auto = false;
    let mut auto_hdr = false;
    let mut sort: Option<bool> = None;
    let mut kind: Option<SortKind> = None;
    for word in options.split_whitespace() {
        let lower = word.to_ascii_lowercase();
        if let Ok(number) = lower.parse::<i32>() {
            width = Some(number);
            continue;
        }
        match lower.as_str() {
            "auto" => auto = true,
            "autohdr" => auto_hdr = true,
            "left" => fmt = Some(LVCFMT_LEFT),
            "center" => fmt = Some(LVCFMT_CENTER),
            "right" => fmt = Some(LVCFMT_RIGHT),
            "sort" => sort = Some(false),
            "sortdesc" => sort = Some(true),
            "nosort" => sort = None,
            "integer" => kind = Some(SortKind::Integer),
            "float" => kind = Some(SortKind::Float),
            "text" => kind = Some(SortKind::Text),
            "logical" => kind = Some(SortKind::Text),
            _ => {}
        }
    }
    if let Some(kind) = kind {
        THE_SORT_KINDS.with(|kinds| {
            kinds
                .borrow_mut()
                .insert((control.handle as isize, col), kind);
        });
    }
    let title_wide = title.map(Wide::of);
    if col > existing {
        // The columns that are not there yet are made, one by one.
        for number in existing + 1..=col {
            let mut column: LVCOLUMNW = unsafe { std::mem::zeroed() };
            column.mask = LVCF_WIDTH | LVCF_FMT;
            column.cx = 100;
            column.fmt = LVCFMT_LEFT;
            if number == col {
                if let Some(w) = width {
                    column.cx = w;
                }
                if let Some(f) = fmt {
                    column.fmt = f;
                }
                if let Some(wide) = &title_wide {
                    column.mask |= LVCF_TEXT;
                    column.pszText = wide.as_ptr() as *mut u16;
                }
            }
            unsafe {
                SendMessageW(
                    control.handle,
                    LVM_INSERTCOLUMNW,
                    (number - 1) as WPARAM,
                    &mut column as *mut LVCOLUMNW as LPARAM,
                );
            }
        }
    } else {
        let mut column: LVCOLUMNW = unsafe { std::mem::zeroed() };
        if let Some(w) = width {
            column.mask |= LVCF_WIDTH;
            column.cx = w;
        }
        if let Some(f) = fmt {
            column.mask |= LVCF_FMT;
            column.fmt = f;
        }
        if let Some(wide) = &title_wide {
            column.mask |= LVCF_TEXT;
            column.pszText = wide.as_ptr() as *mut u16;
        }
        if column.mask != 0 {
            unsafe {
                SendMessageW(
                    control.handle,
                    LVM_SETCOLUMNW,
                    (col - 1) as WPARAM,
                    &mut column as *mut LVCOLUMNW as LPARAM,
                );
            }
        }
    }
    if auto || auto_hdr {
        let flag: WPARAM = if auto_hdr { 0 } else { 1 };
        unsafe {
            SendMessageW(
                control.handle,
                LVM_SETCOLUMNWIDTH,
                (col - 1) as WPARAM,
                flag as LPARAM,
            );
        }
    }
    if let Some(descending) = sort {
        let kind = THE_SORT_KINDS.with(|kinds| {
            kinds
                .borrow()
                .get(&(control.handle as isize, col))
                .copied()
                .unwrap_or(SortKind::Text)
        });
        let mut context = SortContext {
            handle: control.handle,
            column: col,
            descending,
            kind,
        };
        unsafe {
            SendMessageW(
                control.handle,
                LVM_SORTITEMS,
                &mut context as *mut SortContext as WPARAM,
                the_sort_comparison as *const () as LPARAM,
            );
        }
    }
    true
}

/// The list of images is given to the control; the answer is the handle
/// of the one that was there before.
pub fn set_image_list(control: &TheControl, list: i64, large: bool) -> i64 {
    let which = if large { LVSIL_NORMAL } else { LVSIL_SMALL };
    unsafe {
        SendMessageW(
            control.handle,
            LVM_SETIMAGELIST,
            which as WPARAM,
            list as LPARAM,
        ) as i64
    }
}

/// Every row, the cells apart with tabs and the rows with line breaks.
pub fn content(control: &TheControl, selected: bool) -> String {
    let columns = column_count(control).max(1);
    let mut rows = Vec::new();
    let mut row = 0i64;
    loop {
        let found = if selected {
            next(control, row, "")
        } else {
            row += 1;
            if row > count(control) {
                0
            } else {
                row
            }
        };
        if found == 0 {
            break;
        }
        row = found;
        let cells: Vec<String> = (1..=columns).map(|col| text(control, row, col)).collect();
        rows.push(cells.join("\t"));
    }
    rows.join("\n")
}

/// The result of the machine, as the number the v1 reads.
pub fn the_answer_is_true(answer: LRESULT) -> bool {
    answer != 0
}