//! The `LV_*`, the `TV_*` and the `IL_*` of the v1: the list view, the
//! tree view and the lists of images of the window of the script.
//!
//! The functions write to the default control: the last list view or
//! tree view that was added, or the one the `Gui, ListView` names.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::runtime::{Runtime, RuntimeError, Value};

/// The control the `LV_*` write to.
fn the_listview(rt: &mut Runtime<'_>, line: u32) -> Result<Option<u64>, RuntimeError> {
    // Measured on the binary of v1.1.37.02: the `LV_*` of a script that
    // has no list view answers zero and does nothing -- it does not fail.
    let Some((window, number)) = rt.the_default_listview_of_the_v1() else {
        return Ok(None);
    };
    // The v1 numbers the controls of each of its windows from one, and
    // the list view is the one of the window that named it.
    rt.desktop()
        .v1_gui_use_window(&window)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Some(number))
}

/// The control the `TV_*` write to.
fn the_treeview(rt: &mut Runtime<'_>, line: u32) -> Result<Option<u64>, RuntimeError> {
    // Measured on the binary of v1.1.37.02: the `TV_*` of a script that
    // has no tree view answers zero and does nothing -- it does not fail.
    let Some((window, number)) = rt.the_default_treeview_of_the_v1() else {
        return Ok(None);
    };
    rt.desktop()
        .v1_gui_use_window(&window)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Some(number))
}

fn text_of(values: &[Value], at: usize) -> String {
    values
        .get(at)
        .map(|value| value.to_text())
        .unwrap_or_default()
}

fn number_of(values: &[Value], at: usize) -> i64 {
    values
        .get(at)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0) as i64
}

fn the_failure(reason: String, line: u32) -> RuntimeError {
    RuntimeError {
        message: reason,
        line,
    }
}

/// Whether the text is the options of a row and not the text of a
/// column: every word of it is a word the v1 knows.
fn the_text_is_row_options(text: &str) -> bool {
    if text.trim().is_empty() {
        return true;
    }
    text.split_whitespace().all(|word| {
        let lower = word.to_ascii_lowercase();
        let name = lower.trim_start_matches('-').trim_start_matches('+');
        matches!(name, "check" | "focus" | "select" | "vis")
            || name
                .strip_prefix("icon")
                .is_some_and(|n| n.parse::<i32>().is_ok())
            || name
                .strip_prefix("col")
                .is_some_and(|n| n.parse::<i64>().is_ok())
    })
}

/// The cells of a row, as the texts the machine takes.
fn the_cells(values: &[Value], from: usize) -> Vec<String> {
    values
        .iter()
        .skip(from)
        .map(|value| value.to_text())
        .collect()
}

/// `LV_Add([Options, Col1, Col2, ...])`: a row at the end of the list.
pub(crate) fn lv_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let first_is_options = match values.first() {
        Some(Value::Int(_) | Value::Float(_)) => false,
        Some(value) => the_text_is_row_options(&value.to_text()),
        None => false,
    };
    let (options, from) = if first_is_options {
        (text_of(values, 0), 1)
    } else {
        (String::new(), 0)
    };
    let cells = the_cells(values, from);
    let row = rt
        .desktop()
        .v1_lv_add(number, &options, &cells)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(row))
}

/// `LV_Delete([RowNumber])`: a row, or them all.
pub(crate) fn lv_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let row = number_of(values, 0);
    let answer = rt
        .desktop()
        .v1_lv_delete(number, row)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `LV_GetCount(["Selected" | "Column"])`: the number of rows.
pub(crate) fn lv_getcount(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let mode = text_of(values, 0);
    let count = rt
        .desktop()
        .v1_lv_count(number, &mode)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(count))
}

/// `LV_GetNext([StartingRow, "Checked" | "Focused"])`: the next row that
/// is chosen.
pub(crate) fn lv_getnext(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let start = number_of(values, 0);
    let mode = text_of(values, 1);
    let row = rt
        .desktop()
        .v1_lv_next(number, start, &mode)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(row))
}

/// `LV_GetText(OutputVar, RowNumber [, ColumnNumber])`: the text of one
/// cell; the answer is one when the row is there.
pub(crate) fn lv_gettext(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let row = number_of(values, 1);
    let col = if values.get(2).is_some() {
        number_of(values, 2)
    } else {
        1
    };
    let count = rt
        .desktop()
        .v1_lv_count(number, "")
        .map_err(|reason| the_failure(reason, line))?;
    if row < 1 || row > count {
        return Ok(Value::Int(0));
    }
    let text = rt
        .desktop()
        .v1_lv_text(number, row, col.max(1))
        .map_err(|reason| the_failure(reason, line))?;
    if let Some(output) = values.first() {
        rt.write_reference(output, Value::Text(text.into()));
    }
    Ok(Value::Int(1))
}

/// `LV_Modify(RowNumber [, Options, Col1, Col2, ...])`: a row changed.
pub(crate) fn lv_modify(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let row = number_of(values, 0);
    let second_is_options = match values.get(1) {
        Some(Value::Int(_) | Value::Float(_)) => false,
        Some(value) => the_text_is_row_options(&value.to_text()),
        None => false,
    };
    let (options, from) = if second_is_options {
        (text_of(values, 1), 2)
    } else {
        (String::new(), 1)
    };
    let cells = the_cells(values, from);
    let answer = rt
        .desktop()
        .v1_lv_modify(number, row, &options, &cells)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `LV_ModifyCol([ColumnNumber, Options, ColumnTitle])`: a column
/// changed, or made when it is not there.
pub(crate) fn lv_modifycol(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let col = number_of(values, 0);
    let options = text_of(values, 1);
    let title = values.get(2).map(|value| value.to_text());
    let answer = rt
        .desktop()
        .v1_lv_modifycol(number, col, &options, title.as_deref())
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `LV_SetImageList(ImageListID [, 0|1])`: the list of images of the
/// control.
pub(crate) fn lv_setimagelist(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_listview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let list = number_of(values, 0);
    let large = number_of(values, 1) != 0;
    rt.desktop()
        .v1_lv_setimagelist(number, list, large)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(1))
}

/// `IL_Create([InitialCount, GrowCount, LargeIcons])`: a list of images.
pub(crate) fn il_create(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let initial = number_of(values, 0) as i32;
    let grow = number_of(values, 1) as i32;
    let large = number_of(values, 2) != 0;
    let handle = rt
        .desktop()
        .v1_il_create(initial, grow, large)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(handle))
}

/// `IL_Add(ImageListID, Filename [, IconNumber])`: an image in a list;
/// the answer is its place, and zero when it could not be loaded.
pub(crate) fn il_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let list = number_of(values, 0);
    let source = text_of(values, 1);
    let icon = if values.get(2).is_some() {
        number_of(values, 2)
    } else {
        1
    };
    let index = rt
        .desktop()
        .v1_il_add(list, &source, icon)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(index))
}

/// `IL_Destroy(ImageListID)`: a list of images given back.
pub(crate) fn il_destroy(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let list = number_of(values, 0);
    let answer = rt
        .desktop()
        .v1_il_destroy(list)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `TV_Add(Text [, ParentItemID, Options])`: an item of the tree.
pub(crate) fn tv_add(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let text = text_of(values, 0);
    let parent = number_of(values, 1);
    let options = text_of(values, 2);
    let item = rt
        .desktop()
        .v1_tv_add(number, &text, parent, &options)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(item))
}

/// `TV_Delete([ItemID])`: an item, or them all.
pub(crate) fn tv_delete(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 0);
    let answer = rt
        .desktop()
        .v1_tv_delete(number, item)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `TV_GetCount()`: the number of items.
pub(crate) fn tv_getcount(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let count = rt
        .desktop()
        .v1_tv_count(number)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(count))
}

/// `TV_GetText(OutputVar, ItemID)`: the text of an item.
pub(crate) fn tv_gettext(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 1);
    if item == 0 {
        return Ok(Value::Int(0));
    }
    let text = rt
        .desktop()
        .v1_tv_text(number, item)
        .map_err(|reason| the_failure(reason, line))?;
    if let Some(output) = values.first() {
        rt.write_reference(output, Value::Text(text.into()));
    }
    Ok(Value::Int(1))
}

/// `TV_Get(ItemID, "Expand" | "Check" | "Bold")`: the state of an item.
pub(crate) fn tv_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 0);
    let attribute = text_of(values, 1);
    let answer = rt
        .desktop()
        .v1_tv_get(number, item, &attribute)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `TV_GetChild(ParentItemID)`: the first child of a parent.
pub(crate) fn tv_getchild(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let parent = number_of(values, 0);
    let item = rt
        .desktop()
        .v1_tv_child(number, parent)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(item))
}

/// `TV_GetParent(ItemID)`: the parent of an item.
pub(crate) fn tv_getparent(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 0);
    let parent = rt
        .desktop()
        .v1_tv_parent(number, item)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(parent))
}

/// `TV_GetNext([ItemID, "Checked" | "Full"])`: the next item.
pub(crate) fn tv_getnext(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 0);
    let mode = text_of(values, 1);
    let next = rt
        .desktop()
        .v1_tv_next(number, item, &mode)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(next))
}

/// `TV_GetSelection()`: the item that is chosen.
pub(crate) fn tv_getselection(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = rt
        .desktop()
        .v1_tv_selection(number)
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(item))
}

/// `TV_Modify(ItemID [, Options, NewText])`: an item changed. The
/// options that are not there select the item and make it seen.
pub(crate) fn tv_modify(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let Some(number) = the_treeview(rt, line)? else {
        return Ok(Value::Int(0));
    };
    let item = number_of(values, 0);
    let options = if values.get(1).is_some() {
        text_of(values, 1)
    } else {
        "Select Vis".to_string()
    };
    let text = values.get(2).map(|value| value.to_text());
    let answer = rt
        .desktop()
        .v1_tv_modify(number, item, &options, text.as_deref())
        .map_err(|reason| the_failure(reason, line))?;
    Ok(Value::Int(answer as i64))
}

/// `ObjRawSet(Object, Key, Value)`: the property is written directly, and
/// the meta-functions of the object are not asked.
pub(crate) fn obj_raw_set(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let object = values
        .first()
        .cloned()
        .unwrap_or(Value::Text("".into()));
    let Value::Object(obj) = &object else {
        return Err(RuntimeError {
            message: crate::runtime::requires(1, "ObjRawSet", crate::runtime::TypeName::Object, &object),
            line,
        });
    };
    let key = text_of(values, 1);
    let value = values
        .get(2)
        .cloned()
        .unwrap_or(Value::Text("".into()));
    obj.borrow_mut().put(&key, value.clone());
    Ok(value)
}

/// `Exception(Message, What, Extra)`: the object the `throw` of the v1
/// carries, and the `Catch, err` reads the `Message` of.
pub(crate) fn exception(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    Ok(crate::classes::object_with(
        "object",
        alloc::vec![
            ("Message", Value::Text(text_of(values, 0).into())),
            ("What", Value::Text(text_of(values, 1).into())),
            ("Extra", Value::Text(text_of(values, 2).into())),
        ],
    ))
}