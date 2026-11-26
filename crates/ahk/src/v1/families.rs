//! The families of the v1 that the v2 does not have.
//!
//! The v1 has some commands and functions that the v2 does not carry:
//! the `Gui` of the v1 is a command and not a class, the `Menu` of the v1
//! is a command and not a class, and the `ComObjCreate` and the
//! `ComObjArray` of the v1 are functions that the v2 does not have.
//!
//! Every one of them is answered here in the shape of the v1: the command
//! takes the parameters that the v1 takes, in the order that the v1 takes
//! them, and it writes the answer of the v1 into the variable of the
//! caller.
//!
//! The answers come from the same desktop that the v2 uses, so that the
//! two dialects agree about the machine.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::desktop::{TheCoordinate, ThePlaceOfTheV1, TheSize, TheStyleOfTheV1};
use crate::runtime::{Runtime, RuntimeError, Value};

/// The answer of a command of the v1 that writes nothing.
///
/// The `Gui`, the `GuiControl`, the `Menu`, the `Progress`, the `Control`
/// and the `Drive` of the v1 write nothing: they have no variable of the
/// caller to write into, and the failure of them is a note and not a stop.
fn the_answer_of_an_act(
    rt: &mut Runtime<'_>,
    name: &str,
    answer: Result<Value, RuntimeError>,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => Ok(value),
        Err(failure) => {
            rt.note(format!(
                "the command {name} did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

/// The answer of a command of the v1 that goes into the variable of the
/// caller.
///
/// The `ErrorLevel` is the one that the v1 gives: the failure of a command
/// of the v1 is a note and not a stop, and the variable of the caller is
/// empty.
fn the_answer_that_goes_on(
    rt: &mut Runtime<'_>,
    name: &str,
    values: &[Value],
    line: u32,
    answer: Result<Value, RuntimeError>,
    error_level: bool,
) -> Result<Value, RuntimeError> {
    match answer {
        Ok(value) => {
            crate::v1::commands::write_the_answer(rt, name, values, 0, value.clone(), line)?;
            if error_level {
                rt.set_the_error_level(Value::Int(0));
            }
            Ok(value)
        }
        Err(failure) => {
            crate::v1::commands::write_the_answer(
                rt,
                name,
                values,
                0,
                Value::Text("".into()),
                line,
            )?;
            if error_level {
                rt.set_the_error_level(Value::Int(1));
            }
            rt.note(format!(
                "the command {name} did not do the thing (line {}): {}",
                failure.line, failure.message
            ));
            Ok(Value::Text("".into()))
        }
    }
}

// -- the COM of the machine ---------------------------------------------------------

/// `ComObjCreate(CLSID, IID)`: an object of the COM of the machine.
///
/// The v2 has no such function: it has the `ComObjGet` and the
/// `ComObjQuery`, and the object of a class of the machine is what this
/// asks for. Measured on the binary of v1.1.37.02: the
/// `ComObjCreate` of a class that is not there throws.
pub(crate) fn com_obj_create(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let class = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let interface = values.get(1).map(|value| value.to_text());
    let interface = interface.filter(|text| !text.is_empty());
    let object = rt
        .desktop()
        .com_object_of_the_class(&class, interface.as_deref())
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    // The v1: the answer is the number of the object when the script asked
    // for an interface, and the wrapper of it when it did not.
    //
    // Measured on the binary of v1.1.37.02
    // (the probe of the measurement): the `ComObjCreate` with the interface
    // `{42f85136-db7e-439c-85f1-e4075d135fc8}` answers a number -- 10322152
    // -- and the one without an interface answers an object. The number is
    // the pointer of the object, and the script reads the table of its
    // methods with it: `NumGet(pDlg+0, "UPtr")`.
    if interface.is_some() {
        return Ok(Value::Int(object.pointer as i64));
    }
    Ok(crate::classes::comvalue::the_value(
        object.pointer,
        object.kind,
        0,
    ))
}

/// `ComObject(VarType, Value)`: a value of the COM, the wrapper of it.
///
/// The v1 names it `ComObject`; the making of the value is the one of
/// the `ComValue` of the v2.
pub(crate) fn com_object_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // Measured on the binary of v1.1.37.02: the `ComObject` without a
    // class answers the empty text, and it does not throw.
    match crate::classes::comvalue::construct(values, rt, line) {
        Ok(value) => Ok(value),
        Err(_) => Ok(Value::Text("".into())),
    }
}

/// `ObjAddRef(Pointer)`: the count of the COM pointer, one more.
///
/// Measured on the binary of v1.1.37.02: the answer is the new count of
/// the references, and the pointer may be a number or a value of the COM.
pub(crate) fn obj_add_ref_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let pointer = the_pointer_of_the_com(values);
    // Measured on the binary of v1.1.37.02: the `ObjAddRef` of a value
    // that is not an object answers the empty text, and it does not throw.
    if pointer == 0 {
        return Ok(Value::Text("".into()));
    }
    let count = rt
        .desktop()
        .com_pointer_is_counted(pointer)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Int(count as i64))
}

/// `ObjRelease(Pointer)`: the count of the COM pointer, one less.
pub(crate) fn obj_release_of_the_v1(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let pointer = the_pointer_of_the_com(values);
    // Measured on the binary of v1.1.37.02: the `ObjRelease` of a value
    // that is not an object answers the empty text, and it does not throw.
    if pointer == 0 {
        return Ok(Value::Text("".into()));
    }
    let count = rt
        .desktop()
        .com_pointer_is_released(pointer)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    Ok(Value::Int(count as i64))
}

/// The pointer of the argument of the COM: the number the wrapper of a
/// value carries, or the number the script wrote.
fn the_pointer_of_the_com(values: &[Value]) -> u64 {
    match values.first() {
        Some(value) => crate::classes::comvalue::the_number_of(value)
            .map(|number| number as u64)
            .unwrap_or_else(|| value.to_number().unwrap_or(0.0) as u64),
        None => 0,
    }
}

/// `ComObjArray(VarType, Count1, ..., Count8)`: a `SafeArray` for the COM.
///
/// The v2 has no such function either. The desktop of the machine makes
/// the array of the COM, and the answer is the wrapper of it.
pub(crate) fn com_obj_array(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let kind = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default()
        .trim()
        .parse::<i64>()
        .unwrap_or(0);
    let count = values
        .get(1)
        .map(|value| value.to_text())
        .unwrap_or_default()
        .trim()
        .parse::<f64>()
        .unwrap_or(0.0);
    let object = rt
        .desktop()
        .com_value_is_made(kind, crate::desktop::ComValue::Number(count))
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    // The value that the machine made is the value that the script carries.
    Ok(crate::classes::comvalue::the_value(
        object.pointer,
        object.kind,
        0,
    ))
}

/// `ComObjError(Enable)`: whether the errors of the COM are raised.
///
/// The v1 has the setting, and the v2 does not: the v2 always raises the
/// errors of the COM.
pub(crate) fn com_obj_error(
    _rt: &mut Runtime<'_>,
    values: &[Value],
    _line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The setting of the runtime is the one that the v1 answers, and the
    // value that is given is the one that goes into it.
    let before = crate::settings::com_obj_error();
    if let Some(setting) = values.first() {
        let on = match setting {
            Value::Int(number) => *number != 0,
            Value::Float(number) => *number != 0.0,
            other => !other.to_text().is_empty(),
        };
        crate::settings::set_com_obj_error(on);
    }
    Ok(Value::Int(if before { 1 } else { 0 }))
}

// -- the control of a window --------------------------------------------------------

/// `Control, SubCommand, Value, Control, WinTitle, ...`: a change in a
/// control of a window.
///
/// The v2 has a built-in for each one of the changes: the `ControlSetText`
/// and the `ControlSetEnabled` and the `ControlHide` and the
/// `ControlShow`. The v1 puts them all under one command, and the
/// sub-command says which one is meant.
pub(crate) fn control(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Control, SubCommand, Value, Control, WinTitle, ...`.
    let sub_command = value_of(values, 0);
    let value = value_of(values, 1);
    let control = value_of(values, 2);
    let title = value_of(values, 3);
    // The v2 wants the control first and the window after it.
    let mut rest: Vec<Value> = vec![Value::Text(control.clone().into()), Value::Text(title.clone().into())];
    rest.extend(values.iter().skip(4).cloned());
    // The `Style` and the `ExStyle` of the v1 change the style of the
    // control, and the value is the change of it: the v2 wants the style
    // first, and the control and the window after it.
    //
    // Measured on the binary of v1.1.37.02: the
    // `Control, Style, +0x80, , ahk_id %hButton%` of a button of a GUI of
    // the script puts the `BS_BITMAP` on the button.
    let mut with_the_style: Vec<Value> = vec![Value::Text(value.clone().into())];
    with_the_style.extend(rest.iter().cloned());
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        "enable" => crate::builtins::control_set_enabled(rt, &rest, line, 0),
        "disable" => crate::builtins::control_set_enabled(rt, &rest, line, 0),
        "hide" => crate::builtins::control_visibility(rt, &rest, line, 0),
        "show" => crate::builtins::control_show(rt, &rest, line, 0),
        "focus" => crate::builtins::control_act(rt, &rest, line, 1),
        "move" => crate::builtins::control_move(rt, &rest, line, 0),
        "click" => crate::builtins::control_act(rt, &rest, line, 0),
        "send" => crate::builtins::control_send(rt, &rest, line, 0),
        "sendtext" => crate::builtins::control_send_text(rt, &rest, line, 0),
        "style" => crate::builtins::control_set_style(rt, &with_the_style, line, 0),
        "exstyle" => crate::builtins::control_set_style(rt, &with_the_style, line, 1),
        "showdropdown" => crate::builtins::control_show_drop_down(rt, &rest, line, 0),
        "hidedropdown" => crate::builtins::control_hide_drop_down(rt, &rest, line, 0),
        "check" => crate::builtins::control_set_checked(rt, &rest, line, 0),
        "uncheck" => crate::builtins::control_set_checked(rt, &rest, line, 0),
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the Control is not known."),
            line,
        }),
    };
    // The v1 puts the value of the change in the second parameter, and the
    // v2 wants it after the control and the window.
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        "enable" | "disable" | "check" | "uncheck" => {
            let mut with_value = rest.clone();
            with_value.insert(0, Value::Text(value.clone().into()));
            match sub_command.to_ascii_lowercase().as_str() {
                "enable" => crate::builtins::control_set_enabled(rt, &with_value, line, 0),
                "disable" => crate::builtins::control_set_enabled(rt, &with_value, line, 0),
                "check" => crate::builtins::control_set_checked(rt, &with_value, line, 0),
                _ => crate::builtins::control_set_checked(rt, &with_value, line, 0),
            }
        }
        _ => answer,
    };
    the_answer_of_an_act(rt, "Control", answer)
}

// -- the drive of the machine -------------------------------------------------------

/// `Drive, SubCommand, Value1, Value2`: the drive of the machine.
///
/// The v2 has a built-in for each one of the changes: the `DriveSetLabel`
/// and the `DriveLock` and the `DriveUnlock` and the `DriveEject` and the
/// `DriveRetract`.
pub(crate) fn drive(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Drive, SubCommand, Value1, Value2`.
    let sub_command = value_of(values, 0);
    let value = value_of(values, 1);
    let answer = match sub_command.to_ascii_lowercase().as_str() {
        "label" => crate::builtins::drive_set_label(rt, &[Value::Text(value.into())], line, 0),
        "lock" => crate::builtins::drive_lock(rt, &[Value::Text(value.into())], line, 0),
        "unlock" => crate::builtins::drive_unlock(rt, &[Value::Text(value.into())], line, 0),
        "eject" => crate::builtins::drive_eject(rt, &[Value::Text(value.into())], line, 0),
        "retract" => crate::builtins::drive_retract(rt, &[Value::Text(value.into())], line, 0),
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the Drive is not known."),
            line,
        }),
    };
    the_answer_of_an_act(rt, "Drive", answer)
}

// -- the window of the script -------------------------------------------------------

/// `Gui, SubCommand, Value1, Value2, Value3`: the window of the script.
///
/// The v1 has the GUI as a command and the v2 has it as a class.
///
/// The window of the v1 is one at a time: the `Gui, Add` puts a control in
/// the window that is there, and makes the window when there is none. The
/// desktop of the tree has one window of the script as well, and it
/// refuses a second one.
///
/// The sub-commands that the tree does not carry -- the "Submit", the
/// "Cancel", the "Font", the "Color", the "Margin", the "Flash", the
/// "Default" and the "Tab" -- are refused with the reason, and the reason
/// is a note and not a stop: the script goes on.
///
/// Measured on the binary of v1.1.37.02: the sub-commands
/// that the v1 knows are the "New", the "Add", the "Show", the "Submit",
/// the "Cancel", the "Hide", the "Destroy", the "Font", the "Color", the
/// "Margin", the "Menu", the "Minimize", the "Maximize", the "Restore",
/// the "Flash", the "Default" and the "Tab"; the "List", the "ListVars"
/// and the "Back" are refused with "Parameter #1 invalid.".
pub(crate) fn gui(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Gui, SubCommand, Value1, Value2, Value3`. The name of the
    // window may come before the sub-command: the `Gui, Name: Show`.
    let sub_command = value_of(values, 0);
    let trimmed = sub_command.trim();
    let (name, sub) = match trimmed.split_once(':') {
        Some((name, rest)) => (name.trim().to_string(), rest.trim().to_string()),
        None => (String::new(), trimmed.to_string()),
    };
    // The v1: "By design, scripts that use the GUI cmd anywhere are
    // persistent" (source `script.cpp:6192-6199` of v1.1.37.02). A script
    // that uses the `Gui` stays alive after its declarations end, and the
    // a script of the v1 depends on this.
    rt.the_script_asked_to_stay(true);
    // The window of the command: the name that the script gives it, and
    // the window of the thread when the name is left out.
    let window = the_window_of_the_gui(rt, &name, line)?;
    // The v1 makes the window when it is not there and the sub-command
    // needs one. Measured on the binary of v1.1.37.02: the
    // `Gui, PVx: Color` of a window that does not exist is not an error,
    // and the `Gui, PVx: +hwndPVhwnd` answers with the handle of the
    // window that it made.
    let key = sub.to_ascii_lowercase();
    if the_sub_command_makes_the_window(&key) {
        rt.desktop()
            .v1_gui_open(&window, "")
            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
        // The v1: the first window that the script makes is the window of
        // the thread, and the commands that carry no name work on it.
        //
        // Measured on the binary of v1.1.37.02: the
        // `GuiControlGet, MyEdit` of a script that made only the window
        // `PVwin` answers the text of the control of `PVwin`, and the
        // CHM of the v1.1.37.02 says of the windows of a script: "Non-GUI
        // threads use 1 as their default", the one being the first window
        // that the script made.
        if !window.is_empty()
            && rt.the_default_window_of_the_v1().is_empty()
            && !rt.desktop().v1_gui_is_open("").unwrap_or(false)
        {
            rt.remember_the_default_window_of_the_v1(&window);
        }
    }
    let answer = if sub.starts_with('+') || sub.starts_with('-') {
        // The v1: the sub-command of the window may be a list of options:
        // the `Gui, Name: +Option1 -Option2`. The CHM of the v1.1.37.02
        // says: "One or more options and styles may be specified
        // immediately after the GUI command, each separated from the next
        // with one or more spaces or tabs."
        the_options_of_the_window_are_applied(rt, &window, &sub, line)
    } else {
        match sub.to_ascii_lowercase().as_str() {
            "new" => {
                // The v1: the new window takes the place of the one that
                // is there, and it becomes the window of the thread.
                let title = value_of(values, 1);
                let _ = rt.desktop().v1_gui_destroy(&window);
                rt.desktop()
                    .v1_gui_open(&window, title.trim())
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                rt.remember_the_default_window_of_the_v1(&window);
                Ok(Value::Text("".into()))
            }
            "add" => {
                // The v1: `Gui, Add, Type, Options, Text`. The type of the
                // control is the second parameter, the options are the
                // third and the text is the fourth. Measured in the
                // CHM of the v1.1.37.02: "Gui , Add, ControlType , Options,
                // Text", and the example of the CHM is the `Gui, Add,
                // Edit, vName`, which has no text at all.
                let kind = the_kind_of_the_machine(&value_of(values, 1));
                let options = value_of(values, 2);
                let text = value_of(values, 3);
                // The variable that the control is bound to: the `v` of the
                // options of the v1.
                let variable = the_variable_of_the_options(&options);
                // The style of the control: the numbers of the options are
                // bits of it, and the `BS_MULTILINE` of a button depends on
                // the size that the options ask for.
                let style = the_style_of_the_add_options(&options, &kind, text.trim());
                // The window of the v1 is made when it is not there, and
                // the command that made it is the one before this one.
                let added = rt                    .desktop()
                    .v1_gui_add(&window, &kind, &variable, text.trim(), style)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                // The list view and the tree view that were added last
                // are the ones the `LV_*` and the `TV_*` write to.
                match kind.as_str() {
                    "listview" => rt.remember_the_listview_of_the_v1(&window, added.0),
                    "treeview" => rt.remember_the_treeview_of_the_v1(&window, added.0),
                    _ => {}
                }
                // The place of the control: the `x` and the `y` and the `w`
                // and the `h` of the options of the v1.
                let place = the_place_of_the_add_options(&options, &kind);
                if place.x.is_some()
                    || place.y.is_some()
                    || place.width.is_some()
                    || place.height.is_some()
                    || place.section
                    // A picture is always given its place: the image of it
                    // is loaded when the control has its size.
                    || kind == "picture"
                {
                    let _ = rt.desktop().v1_gui_place(&window, added.0, place);
                }
                // The `hwnd` of the options of the v1 puts the handle of the
                // control into a variable of the script.
                //
                // Measured on the binary of v1.1.37.02: the
                // `Gui, PVwin: Add, Text, x0 y0 w1 h1 BackgroundTrans
                // vPicOnGui1 hwndhPic0, Previous image` writes the handle
                // into `hPic0`, and the name of the variable keeps the case
                // that the script wrote. The plus sign in front of the
                // name of the option is accepted too: the `+hwndhPic0`
                // writes the handle into `hPic0` as well.
                let handle_variable = the_handle_variable_of_the_options(&options);
                if !handle_variable.is_empty() {
                    if let Ok(handle) = rt.desktop().v1_gui_handle(&window, added.0) {
                        // The variable of the option is the global of the
                        // script when the script has a global of that name,
                        // and the scope of the caller when it has not.
                        //
                        // Measured on the binary of AutoHotkey v1.1.37.02
                        //: the `Gui, PVa: Add, Edit,
                        // HwndgvFoo3` of a function leaves the handle in the
                        // global `gvFoo3` of the script, and the top level
                        // of the script wrote that global without a
                        // `Global` declaration; the handle is the local of
                        // the function when the script has no global of
                        // that name, and when the function names it in its
                        // body.
                        the_handle_variable_is_written(rt, &handle_variable, handle);
                    }
                }
                Ok(Value::Text(added.0.to_string().into()))
            }
            "listview" => {
                // The v1: `Gui, Name: ListView, ControlID` names the list
                // view that the `LV_*` write to. The name of the control
                // is the name of the variable of it, or its ClassNN.
                let name = value_of(values, 1);
                let number = rt
                    .desktop()
                    .v1_gui_number_named(&window, name.trim())
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                rt.remember_the_listview_of_the_v1(&window, number as u64);
                Ok(Value::Text("".into()))
            }
            "treeview" => {
                // The v1: `Gui, Name: TreeView, ControlID` names the tree
                // view that the `TV_*` write to.
                let name = value_of(values, 1);
                let number = rt
                    .desktop()
                    .v1_gui_number_named(&window, name.trim())
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                rt.remember_the_treeview_of_the_v1(&window, number as u64);
                Ok(Value::Text("".into()))
            }
            "show" => {
                // The v1: the window is made when there is none, and the
                // options of it and the title come with the `Show`. Read in
                // the CHM of the v1.1.37.02 (`Gui.htm`): "Gui , Show [,
                // Options, Title]", and the options are the `w`, the `h`,
                // the `x` and the `y` among others.
                let options = value_of(values, 1);
                let title = value_of(values, 2);
                let (x, y, width, height) = the_place_of_the_options(&options);
                let (centered_x, centered_y) = the_center_of_the_options(&options);
                // The mode of the window is ONE, and the options are read
                // from the left to the right: the last one of them wins.
                //
                // Measured on the binary of v1.1.37.02
                // (the probe of the measurement): the `Gui, Show, Maximize Hide
                // Center w2258 h1440` of a script of the v1 gives a
                // window of 2274x1479 that is hidden -- the "Hide" is the
                // last mode -- and the size is the one that the script
                // wrote. The source of the v1 (`script_gui.cpp`) reads the
                // options in the same order and keeps one mode.
                let mode = the_mode_of_the_options(&options);
                rt.desktop()
                    .v1_gui_window_place(&window, x, y, width, height, centered_x, centered_y)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                // The v1: the "AutoSize" makes the window as big as the
                // controls that it holds.
                if the_options_say_auto_size(&options) {
                    rt.desktop()
                        .v1_gui_auto_size(&window)
                        .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                }
                // The title of the window comes with the `Show`, and the
                // mode of the window is the one that the options say.
                if !title.trim().is_empty() && mode != TheMode::Show {
                    rt.desktop()
                        .v1_gui_title(&window, Some(title.trim()))
                        .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                }
                match mode {
                    TheMode::Show => {
                        rt.desktop()
                            .v1_gui_show(&window, title.trim())
                            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                    }
                    // The v1: the "NoActivate" and the "NA" show the
                    // window without making it the one of the thread.
                    TheMode::ShowWithoutActivating => {
                        rt.desktop()
                            .v1_gui_show_without_activating(&window, title.trim())
                            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                    }
                    TheMode::Hide => {
                        rt.desktop()
                            .v1_gui_hide(&window)
                            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                    }
                    TheMode::Maximize | TheMode::Minimize | TheMode::Restore => {
                        let name = match mode {
                            TheMode::Maximize => "maximize",
                            TheMode::Minimize => "minimize",
                            _ => "restore",
                        };
                        let _ = rt.desktop().v1_gui_option(&window, name, None, true);
                    }
                }
                Ok(Value::Text("".into()))
            }
            "hide" => rt
                .desktop()
                .v1_gui_hide(&window)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line)),
            "destroy" => {
                // The v1: the `Gui, Name: Destroy` of a window that is not
                // there does nothing, and it is not an error.
                //
                // Measured on the binary of v1.1.37.02: the
                // `Gui, NoSuchWin: Destroy` is not an error.
                let _ = rt.desktop().v1_gui_destroy(&window);
                Ok(Value::Text("".into()))
            }
            "menu" => {
                // The v1: `Gui, Menu, MenuName` -- the bar of the window is
                // the menu of the script that the name names.
                let name = value_of(values, 1);
                let items = crate::v1::menus::the_items_of_the_menu(rt, &name);
                let machine = crate::v1::menus::the_machine_menu_of(rt, &name);

                rt.desktop()
                    .v1_gui_menu_bar(&window, &name, &items, machine.unwrap_or(0))
                    .map(|_| Value::Text("".into()))
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
            "color" => {
                // The v1: `Gui, Color, Color1, Color2`. The colour of the
                // window is the first one.
                let color = value_of(values, 1);
                rt.desktop()
                    .v1_gui_color(&window, color.trim())
                    .map(|_| Value::Text("".into()))
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
            "margin" => {
                // The v1: `Gui, Margin, X, Y`.
                let x = value_of(values, 1).trim().parse::<i64>().unwrap_or(0);
                let y = value_of(values, 2).trim().parse::<i64>().unwrap_or(0);
                rt.desktop()
                    .v1_gui_margin(&window, x, y)
                    .map(|_| Value::Text("".into()))
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
            "font" => {
                // The v1: `Gui, Font, Options, FontName`.
                let options = value_of(values, 1);
                let name = value_of(values, 2);
                rt.desktop()
                    .v1_gui_font(&window, options.trim(), name.trim())
                    .map(|_| Value::Text("".into()))
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
            "submit" => {
                // The v1: `Gui, Submit` puts the contents of the controls
                // that have a variable of their own into those variables,
                // and hides the window unless the "NoHide" is there.
                //
                // The variables are the ones of the scope of the caller:
                // the CHM of the v1.1.37.02 says of the sub-command:
                // "Saves the contents of each control to its associated
                // variable (if any) and hides the window unless the
                // NoHide option is present."
                let answer = rt
                    .desktop()
                    .v1_gui_submit(&window)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                for (name, text) in answer {
                    rt.assign_the_variable_of_the_pseudo_array(&name, Value::Text(text.into()));
                }
                let options = value_of(values, 1);
                if !options
                    .split_whitespace()
                    .any(|word| word.eq_ignore_ascii_case("nohide"))
                {
                    let _ = rt.desktop().v1_gui_hide(&window);
                }
                Ok(Value::Text("".into()))
            }
            "cancel" => rt
                .desktop()
                .v1_gui_hide(&window)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line)),
            "default" => {
                // The v1: `Gui, Name: Default` makes the window that the
                // name names the one of the thread.
                rt.remember_the_default_window_of_the_v1(&window);
                Ok(Value::Text("".into()))
            }
            "minimize" => {
                let _ = rt.desktop().v1_gui_option(&window, "minimize", None, true);
                Ok(Value::Text("".into()))
            }
            "maximize" => {
                let _ = rt.desktop().v1_gui_option(&window, "maximize", None, true);
                Ok(Value::Text("".into()))
            }
            "restore" => {
                let _ = rt.desktop().v1_gui_option(&window, "restore", None, true);
                Ok(Value::Text("".into()))
            }
            "flash" => {
                // The v1: the `Flash` blinks the button of the window in
                // the task bar. The machine of the tree has no task bar of
                // its own.
                Ok(Value::Text("".into()))
            }
            "tab" => {
                // The v1: `Gui, Name: Tab, N` switches to the tab `N` of
                // the tab control of the window. The CHM of the v1 does
                // not list the sub-command, and the binary of the
                // v1.1.37.02 accepts it.
                let tab = value_of(values, 1).trim().parse::<i64>().unwrap_or(0);
                rt.desktop()
                    .v1_gui_tab(&window, tab)
                    .map(|_| Value::Text("".into()))
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
            _ => Err(RuntimeError {
                message: format!("the sub-command {sub_command} of the Gui is not known."),
                line,
            }),
        }
    };
    the_answer_of_an_act(rt, "Gui", answer)
}

/// Whether the sub-command of the `Gui` makes the window when it is not
/// there.
///
/// The CHM of the v1.1.37.02 says of the "Add": "Adds a control to a GUI
/// window (first creating the GUI window itself, if necessary)." The
/// other sub-commands that work on the window itself make it too, and the
/// ones that the CHM says have no effect when the window does not exist
/// do not: the "Submit", the "Cancel", the "Hide", the "Destroy", the
/// "Minimize", the "Maximize", the "Restore" and the "Flash".
fn the_sub_command_makes_the_window(sub: &str) -> bool {
    !matches!(
        sub,
        "new" | "destroy" | "submit" | "cancel" | "hide" | "minimize" | "maximize" | "restore"
            | "flash" | "default"
    )
}

/// The window of the v1 that a command of the `Gui` works on.
///
/// The v1 names its windows, and the name may be the name of the window
/// or the handle of it. The name that is left out is the window of the
/// thread, the one that the `Gui, Name: Default` named.
fn the_window_of_the_gui(
    rt: &mut Runtime<'_>,
    name: &str,
    _line: u32,
) -> Result<String, RuntimeError> {
    if name.is_empty() {
        return Ok(rt.the_default_window_of_the_v1().to_string());
    }
    // The window of the name comes first.
    if rt.desktop().v1_gui_is_open(name).unwrap_or(false) {
        return Ok(name.to_string());
    }
    // The handle of a window names it too: the `Gui, %MyGuiHwnd%:Destroy`.
    if let Some(handle) = the_handle_of_the_v1(name) {
        if let Ok(found) = rt.desktop().v1_gui_name_of_handle(handle) {
            return Ok(found);
        }
    }
    Ok(name.to_string())
}

/// The handle of a window of the v1, out of the text that the script
/// wrote.
///
/// The v1 writes the handle of a window as a number with the "0x" in
/// front of it, and the script may write it as a decimal number too.
fn the_handle_of_the_v1(text: &str) -> Option<i64> {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        return i64::from_str_radix(hex, 16).ok();
    }
    if !text.is_empty() && text.chars().all(|character| character.is_ascii_digit()) {
        return text.parse::<i64>().ok();
    }
    None
}

/// The options of a window of the v1 are applied.
///
/// The sub-command of the `Gui` may be a list of options, and each of
/// them carries its own sign. The options that take a value carry it
/// right after the name of them, with no space between the two.
fn the_options_of_the_window_are_applied(
    rt: &mut Runtime<'_>,
    window: &str,
    text: &str,
    line: u32,
) -> Result<Value, RuntimeError> {
    for word in text.split_whitespace() {
        let Some(sign) = word.chars().next() else {
            continue;
        };
        let on = match sign {
            '+' => true,
            '-' => false,
            // A word with no sign is not an option, and the v1 walks over
            // it.
            _ => continue,
        };
        let rest = &word[sign.len_utf8()..];
        // The styles without a name: the `+0x...` and the `+E0x...`.
        if let Some((extended, style)) = the_style_without_a_name(rest) {
            rt.desktop()
                .v1_gui_style(window, extended, style, on)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            continue;
        }
        // The options with a name. The name of the option is in lower
        // case, and the value of it keeps the case that the script wrote:
        // the `+hwndPVhwnd` puts the handle into the variable `PVhwnd`.
        let lower = rest.to_ascii_lowercase();
        let Some((name, length)) = the_option_of_the_window(&lower) else {
            return Err(RuntimeError {
                message: format!("the option {rest} of the window is not one that the v1 knows."),
                line,
            });
        };
        let value = &rest[length..];
        the_option_of_the_window_is_applied(rt, window, name, value, on, line)?;
    }
    Ok(Value::Text("".into()))
}

/// The name of the option of a window of the v1 and the value of it.
///
/// The options are the ones that the CHM of the v1.1.37.02 lists for a
/// window, and the ones that take a value carry it right after the name
/// of them. The list is in the order of the length of the names, so that
/// the longer name is found first: the "lastfoundexist" before the
/// "lastfound".
const THE_OPTIONS_OF_A_WINDOW: &[(&str, bool)] = &[
    ("lastfoundexist", false),
    ("alwaysontop", false),
    ("maximizebox", false),
    ("minimizebox", false),
    ("delimiter", true),
    ("owndialogs", false),
    ("toolwindow", false),
    ("lastfound", false),
    ("disabled", false),
    ("dpiscale", false),
    ("minsize", true),
    ("maxsize", true),
    ("sysmenu", false),
    ("caption", false),
    ("resize", false),
    ("border", false),
    ("parent", true),
    ("owner", true),
    ("label", true),
    ("theme", false),
    ("hwnd", true),
];

/// The name of the option of a window of the v1 and the value of it.
///
/// The answer is `None` when the v1 does not know the option.
fn the_option_of_the_window(text: &str) -> Option<(&'static str, usize)> {
    for (name, takes_value) in THE_OPTIONS_OF_A_WINDOW {
        if let Some(value) = text.strip_prefix(name) {
            if !*takes_value && !value.is_empty() {
                continue;
            }
            return Some((name, name.len()));
        }
    }
    None
}

/// The style without a name that the script wrote, and whether it is the
/// extended one.
///
/// The v1 writes the ordinary style with a sign and a number, and the
/// extended one with the letter `E` before the number.
fn the_style_without_a_name(text: &str) -> Option<(bool, u32)> {
    let (extended, digits) = if let Some(digits) = text
        .strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
    {
        (false, digits)
    } else {
        let digits = text
            .strip_prefix("E0x")
            .or_else(|| text.strip_prefix("E0X"))
            .or_else(|| text.strip_prefix("e0x"))
            .or_else(|| text.strip_prefix("e0X"))?;
        (true, digits)
    };
    let style = u32::from_str_radix(digits, 16).ok()?;
    Some((extended, style))
}

/// The option of a window of the v1 is applied.
fn the_option_of_the_window_is_applied(
    rt: &mut Runtime<'_>,
    window: &str,
    name: &str,
    value: &str,
    on: bool,
    line: u32,
) -> Result<(), RuntimeError> {
    match name {
        "hwnd" => {
            // The v1: the handle of the window goes into the variable of
            // the script.
            //
            // Measured on the binary of v1.1.37.02: the
            // name of the variable keeps the case that the script wrote.
            if value.is_empty() {
                return Ok(());
            }
            let handle = rt
                .desktop()
                .v1_gui_window_handle(window)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            // The variable of the option is the global of the script when
            // the script has a global of that name, and the scope of the
            // caller when it has not.
            //
            // Measured on the binary of AutoHotkey v1.1.37.02 the `Gui, PVe: +hwndgvE` of a function leaves the
            // handle in the global `gvE` of the script, and the top level
            // of the script wrote that global without a `Global`
            // declaration.
            the_handle_variable_is_written(rt, value, handle);
            Ok(())
        }
        "owner" | "parent" => {
            // The v1: the `-Owner` takes the owner away from the window,
            // and the `-Parent` takes the parent away from it.
            //
            // Measured on the binary of v1.1.37.02: the
            // `-Owner` takes the `GWLP_HWNDPARENT` of the window back to
            // zero, and the `-Parent` makes the window go back to being a
            // window of the top level.
            let owner = if on { Some(value) } else { None };
            rt.desktop()
                .v1_gui_owner(window, owner, name == "parent")
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "lastfound" => {
            // The v1: the window becomes the last found window, so that
            // the `WinSet` and the `WinSetTitle` work on it.
            let handle = rt
                .desktop()
                .v1_gui_window_handle(window)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            rt.remember_the_last_window_of_the_v1(&handle.to_string());
            Ok(())
        }
        "lastfoundexist" => {
            if rt.desktop().v1_gui_is_open(window).unwrap_or(false) {
                let handle = rt
                    .desktop()
                    .v1_gui_window_handle(window)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                rt.remember_the_last_window_of_the_v1(&handle.to_string());
            }
            Ok(())
        }
        // The v1: the dialogs of the thread belong to the window. The
        // tree has no dialog of its own that belongs to a window.
        "owndialogs" => Ok(()),
        // The v1 scales the place of the window by the screen when the
        // "DPIScale" is on. The tree works in the pixels of the machine,
        // and the scaling of the screen is left to it.
        "dpiscale" => Ok(()),
        _ => rt
            .desktop()
            .v1_gui_option(
                window,
                name,
                if value.is_empty() { None } else { Some(value) },
                on,
            )
            .map_err(|reason| crate::builtins::desktop_failure(reason, line)),
    }
}

/// The centering of the window, out of the options of the `Show`.
///
/// The v1 centers the window with the "Center", with the "xCenter" it
/// centers it from side to side, and with the "yCenter" from top to
/// bottom.
fn the_center_of_the_options(options: &str) -> (bool, bool) {
    let mut horizontal = false;
    let mut vertical = false;
    for word in options.split_whitespace() {
        match word.to_ascii_lowercase().as_str() {
            "center" => {
                horizontal = true;
                vertical = true;
            }
            "xcenter" => horizontal = true,
            "ycenter" => vertical = true,
            _ => {}
        }
    }
    (horizontal, vertical)
}

/// The mode of the window of the v1, the one that the `Show` gives to it.
///
/// The v1: the options are read from the left to the right and the mode
/// of the window is the last one of them, so that the "Hide" after the
/// "Maximize" hides the window instead of maximizing it.
///
/// Measured on the binary of v1.1.37.02 (the probe of the measurement):
/// the `Gui, Show, Maximize Hide Center w2258 h1440` of a script of the v1
/// Viewer gives a window of 2274x1479 that is hidden.
#[derive(Clone, Copy, PartialEq, Eq)]
enum TheMode {
    /// The window is shown and it is the one of the thread.
    Show,
    /// The window is shown and it is not the one of the thread.
    ShowWithoutActivating,
    /// The window is hidden.
    Hide,
    /// The window is minimized.
    Minimize,
    /// The window is maximized.
    Maximize,
    /// The window goes back to the size that it had.
    Restore,
}

/// The mode of the window, out of the options of the `Show`.
fn the_mode_of_the_options(options: &str) -> TheMode {
    let mut mode = TheMode::Show;
    for word in options.split_whitespace() {
        match word.to_ascii_lowercase().as_str() {
            "maximize" => mode = TheMode::Maximize,
            "minimize" => mode = TheMode::Minimize,
            "restore" => mode = TheMode::Restore,
            "hide" => mode = TheMode::Hide,
            "noactivate" | "na" => mode = TheMode::ShowWithoutActivating,
            _ => {}
        }
    }
    mode
}

/// Whether the options of the `Show` say that the window is as big as the
/// controls that it holds.
fn the_options_say_auto_size(options: &str) -> bool {
    options
        .split_whitespace()
        .any(|word| word.eq_ignore_ascii_case("autosize"))
}

/// The variable that a control of the v1 is bound to.
///
/// The `v` of the options of the `Gui, Add` binds a control to a variable
/// of the script, and the `GuiControl` and the `GuiControlGet` name the
/// control by that name.
fn the_variable_of_the_options(options: &str) -> String {
    for word in options.split_whitespace() {
        let Some(name) = word.strip_prefix('v').or_else(|| word.strip_prefix('V')) else {
            continue;
        };
        if !name.is_empty() && !name.starts_with('+') {
            // The name of the variable keeps the case that the script
            // wrote.
            return name.to_string();
        }
    }
    String::new()
}

/// The variable that the handle of a control of the v1 goes into.
///
/// The `hwnd` of the options of the `Gui, Add` puts the handle of the
/// control into a variable of the script.
///
/// Measured on the binary of v1.1.37.02: the sign in
/// front of the name of the option is accepted, and the name of the
/// variable keeps the case that the script wrote: the `+hwndhPic0`
/// writes the handle into `hPic0`, the `HwndMyEditHwnd` writes it into
/// `MyEditHwnd`, and the `-hwndMinusHwnd` writes it into `MinusHwnd`.
fn the_handle_variable_of_the_options(options: &str) -> String {
    for word in options.split_whitespace() {
        // The sign of the option is not a part of the name of the
        // variable: the CHM of the v1.1.37.02 says of the options of a
        // control: "Note: In the absence of a preceding sign, a plus
        // sign is assumed".
        let word = word
            .strip_prefix('+')
            .or_else(|| word.strip_prefix('-'))
            .unwrap_or(word);
        if word.len() <= 4
            || !word
                .get(..4)
                .is_some_and(|head| head.eq_ignore_ascii_case("hwnd"))
        {
            continue;
        }
        // The name of the variable keeps the case that the script wrote.
        return word[4..].to_string();
    }
    String::new()
}

/// The handle of a control or of a window goes into the variable of the
/// script that the option names.
///
/// The v1 writes the variable as a dynamic variable of the scope of the
/// caller: the CHM of the v1.1.37.02 says of the option of the `Gui,
/// Add`: "When within a function, MyEditHwnd is treated as a function
/// dynamic variable."
///
/// Measured on the binary of v1.1.37.02:
/// - the `Gui, PVa: Add, Edit, HwndFoo1` of a function that does not name
///   `Foo1` leaves the handle in the global of the script;
/// - the `Gui, PVb: Add, Edit, HwndFoo2` of a function that names `Foo2`
///   in its body leaves it in the local of the function;
/// - the `Gui, PVa3: Add, Edit, HwndgvHwndA3` of a function leaves it in
///   the global `gvHwndA3` of the script, when there is one of that name.
///
/// The tree has no pass that reads the body of the function, so the
/// global of the script is the one that is written when there is one of
/// that name, and the scope of the caller is the one that is written when
/// there is none.
fn the_handle_variable_is_written(rt: &mut Runtime<'_>, name: &str, handle: i64) {
    let global_of_that_name = rt
        .variable_names()
        .iter()
        .any(|(other, _)| other.eq_ignore_ascii_case(name));
    // The v1: the handle is a hexadecimal text with the "0x" in front of
    // it.
    let handle = crate::builtins::the_handle_of_the_machine(rt, handle);
    if global_of_that_name {
        rt.set_global(name, handle);
        return;
    }
    rt.assign_the_variable_of_the_pseudo_array(name, handle);
}

/// The window of the script, made when there is none.
///
/// The v1 has one window of the script at a time: the `Gui, Add` puts a
/// control in the window that is there, and makes it when there is none.
/// The desktop of the tree refuses a second window, so the question comes
/// first.
/// The place and the size of the window, out of the options of the `Show`.
///
/// Read in the CHM of the v1.1.37.02 (`Gui.htm`): `w` and `h` are the
/// width and the height, `x` and `y` are the place, and the ones that are
/// not there are not there.
fn the_place_of_the_options(options: &str) -> (Option<i64>, Option<i64>, Option<i64>, Option<i64>) {
    let mut x = None;
    let mut y = None;
    let mut width = None;
    let mut height = None;
    for word in options.split_whitespace() {
        let Some(letter) = word.chars().next() else {
            continue;
        };
        let rest = &word[letter.len_utf8()..];
        let Some(value) = the_number_of_an_option(rest) else {
            continue;
        };
        match letter.to_ascii_lowercase() {
            'w' => width = Some(value),
            'h' => height = Some(value),
            'x' => x = Some(value),
            'y' => y = Some(value),
            _ => {}
        }
    }
    (x, y, width, height)
}

/// The number of an option, when it is a number.
///
/// The v1 reads the options of the `Show` with the number that they start
/// with, and the sign is part of it.
fn the_number_of_an_option(text: &str) -> Option<i64> {
    let first = text.chars().next()?;
    if !(first.is_ascii_digit() || first == '-' || first == '+') {
        return None;
    }
    Some(the_number_of(text))
}

/// The place and the size of the options of the `Gui, Add` of the v1.
///
/// Read in the CHM of the v1.1.37.02 (`Gui.htm`) and in the source of the
/// v1 (`script_gui.cpp:5717-5835`): the `x` and the `y` of the options have
/// many forms, and each of them is a place that the machine computes from
/// the window and from the control that was added before it.
///
/// - `x10` is the place of the client area of the window;
/// - `x+10` is the right edge of the control before it, ten pixels more;
/// - `x+m` is the right edge of the control before it, the margin more;
/// - `xm+10` is the margin of the window, ten pixels more;
/// - `xp+10` is the left edge of the control before it, ten pixels more;
/// - `xs+10` is the section of the window, ten pixels more.
///
/// The `y` is the same, with the bottom edge and the top edge. The `wp` and
/// the `hp` are the size of the control before it.
fn the_place_of_the_add_options(options: &str, kind: &str) -> ThePlaceOfTheV1 {
    let mut place = ThePlaceOfTheV1::default();
    for word in options.split_whitespace() {
        // The sign of the option comes before the word of it (source
        // `script_gui.cpp:4569-4587` of v1.1.37.02).
        let word = word
            .strip_prefix('+')
            .or_else(|| word.strip_prefix('-'))
            .unwrap_or(word);
        let Some(letter) = word.chars().next() else {
            continue;
        };
        // The v1 reads the options that have a name before the ones that
        // have only a letter, and the "hwnd" of a control is not the
        // height of it (source `script_gui.cpp:4602-4613` of v1.1.37.02).
        if the_option_has_the_name_of_another_option(word, kind) {
            continue;
        }
        let rest = &word[letter.len_utf8()..];
        match letter.to_ascii_lowercase() {
            'x' => {
                if let Some(coordinate) = the_coordinate_of_the_options(rest) {
                    place.x = Some(coordinate);
                }
            }
            'y' => {
                if let Some(coordinate) = the_coordinate_of_the_options(rest) {
                    place.y = Some(coordinate);
                }
            }
            'w' => {
                if let Some(size) = the_size_of_the_options(rest) {
                    place.width = Some(size);
                }
            }
            'h' => {
                if let Some(size) = the_size_of_the_options(rest) {
                    place.height = Some(size);
                }
            }
            'r' => {
                if let Ok(rows) = rest.parse::<i64>() {
                    if rows > 0 {
                        place.rows = Some(rows);
                    }
                }
            }
            's' if rest.eq_ignore_ascii_case("ection") => {
                place.section = true;
            }
            _ => {}
        }
    }
    place
}

/// Whether the word of an option is the name of another option, and not the
/// letter of the place or of the size of the control.
///
/// The v1 reads the options that have a name before the ones that have only
/// a letter (source `script_gui.cpp:4602-4613` of v1.1.37.02), and the
/// "hwnd" of a control is not the height of it.
///
/// Measured on the binary of v1.1.37.02: the button of the
/// options "w35 h35 vB1 hwndHB1" is 35x35, and the "h" of the "hwnd" does
/// not make the height of it zero.
fn the_option_has_the_name_of_another_option(word: &str, kind: &str) -> bool {
    let word = word.to_ascii_lowercase();
    if word.starts_with("hwnd") || word.starts_with("hscroll") || word.starts_with("hidden") {
        return true;
    }
    if word == "wrap" {
        return true;
    }
    // The "Hdr" is the header of a list view and the "Horz" is the
    // slider of the numbers, and the v1 reads them only for them.
    if word == "hdr" {
        return kind == "listview";
    }
    if word == "horz" {
        return kind == "updown";
    }
    false
}

/// The style of a button that writes its text in more than one line.
///
/// The `BS_MULTILINE` of the v1 (source `script_gui.cpp:2233-2238` of
/// v1.1.37.02).
const BS_MULTILINE: u32 = 0x2000;

/// The bits of the style of a control that the options of the `Gui, Add`
/// of the v1 ask for.
///
/// The v1: "Pure numbers are assumed to be style additions or removals"
/// (source `script_gui.cpp:5564-5572` of v1.1.37.02). The number is read
/// the way the `ATOU` of the v1 reads it: hexadecimal when it starts with
/// "0x", and decimal otherwise.
///
/// Measured on the binary of v1.1.37.02: the
/// `Gui, Test: Add, Button, x10 y10 w100 h30 +0x8000, Hello` gives a
/// button the style `0x5001A000` -- the `WS_CHILD`, the `WS_VISIBLE`, the
/// `WS_TABSTOP`, the `BS_FLAT` (0x8000) and the `BS_MULTILINE` (0x2000).
/// The `BS_MULTILINE` is there because the button has the width and the
/// height written.
fn the_style_of_the_add_options(options: &str, kind: &str, text: &str) -> TheStyleOfTheV1 {
    let mut style = TheStyleOfTheV1::default();
    for word in options.split_whitespace() {
        // The sign of the option says what happens to the bits: the `+`
        // adds them and the `-` takes them away. The option without a sign
        // adds them, the way the v1 does.
        let (adding, digits) = match word.as_bytes().first() {
            Some(b'+') => (true, &word[1..]),
            Some(b'-') => (false, &word[1..]),
            _ => (true, word),
        };
        // The "Icon" of the options names the icon of a file of icons
        // that a picture control shows.
        //
        // The v1 reads it at the source `script_gui.cpp:4632-4662` of
        // v1.1.37.02, and the number after it is the number of the icon.
        //
        // Measured on the binary of v1.1.37.02: the
        // `Gui, Add, Picture, Icon2, some.ico` is a control of the style
        // `0x50000003` -- the `SS_ICON` --.
        if digits.len() > 4 && digits[..4].eq_ignore_ascii_case("icon") {
            if let Ok(number) = digits[4..].parse::<u32>() {
                style.icon = Some(number);
            }
            continue;
        }
        // The "Checked" of the options puts the box in the state that they
        // ask for, and the word may have a number after it.
        //
        // The v1 reads it at the source `script_gui.cpp:4632-4662` of
        // v1.1.37.02: the "Checked" asks for the box to be checked, the
        // "CheckedGray" asks for the gray one and the "Checked1" or the
        // "Checked0" ask for the state that the number says.
        //
        // Measured on the binary of v1.1.37.02: the
        // `Gui, Add, Checkbox, vCHK Checked, Check` makes a check box that
        // is checked, and the `GuiControlGet, out, , CHK` of it answers "1".
        if digits.len() >= 7 && digits[..7].eq_ignore_ascii_case("checked") {
            let rest = &digits[7..];
            if rest.eq_ignore_ascii_case("gray") {
                style.checked = Some(if adding { 2 } else { 0 });
            } else if rest.is_empty() {
                style.checked = Some(if adding { 1 } else { 0 });
            } else if let Ok(number) = rest.parse::<i64>() {
                style.checked = Some(if number == -1 { 2 } else { number });
            }
            continue;
        }
        // `ChooseN`: initial 1-based selected tab/item.
        if digits.len() > 6 && digits[..6].eq_ignore_ascii_case("choose") {
            if let Ok(number) = digits[6..].parse::<i64>() {
                style.choose = Some(number);
            }
            continue;
        }
        // Named style options common in v1 `Gui, Add`.
        if digits.eq_ignore_ascii_case("readonly") && kind == "edit" {
            const ES_READONLY: u32 = 0x0800;
            if adding {
                style.add |= ES_READONLY;
            } else {
                style.remove |= ES_READONLY;
            }
            continue;
        }
        if digits.eq_ignore_ascii_case("hide") || digits.eq_ignore_ascii_case("hidden") {
            const WS_VISIBLE: u32 = 0x1000_0000;
            if adding {
                style.remove |= WS_VISIBLE;
            } else {
                style.add |= WS_VISIBLE;
            }
            continue;
        }
        if digits.eq_ignore_ascii_case("disabled") {
            const WS_DISABLED: u32 = 0x0800_0000;
            if adding {
                style.add |= WS_DISABLED;
            } else {
                style.remove |= WS_DISABLED;
            }
            continue;
        }
        if digits.eq_ignore_ascii_case("tabstop") {
            const WS_TABSTOP: u32 = 0x0001_0000;
            if adding {
                style.add |= WS_TABSTOP;
            } else {
                style.remove |= WS_TABSTOP;
            }
            continue;
        }
        if digits.eq_ignore_ascii_case("border") {
            const WS_BORDER: u32 = 0x0080_0000;
            if adding {
                style.add |= WS_BORDER;
            } else {
                style.remove |= WS_BORDER;
            }
            continue;
        }
        if digits.eq_ignore_ascii_case("default") && kind == "button" {
            const BS_DEFPUSHBUTTON: u32 = 0x0001;
            if adding {
                style.add |= BS_DEFPUSHBUTTON;
            } else {
                style.remove |= BS_DEFPUSHBUTTON;
            }
            continue;
        }
        if kind == "edit" {
            if digits.eq_ignore_ascii_case("multi") {
                const ES_MULTILINE: u32 = 0x0004;
                if adding {
                    style.add |= ES_MULTILINE;
                } else {
                    style.remove |= ES_MULTILINE;
                }
                continue;
            }
            if digits.eq_ignore_ascii_case("number") {
                const ES_NUMBER: u32 = 0x2000;
                if adding {
                    style.add |= ES_NUMBER;
                } else {
                    style.remove |= ES_NUMBER;
                }
                continue;
            }
            if adding && (digits.starts_with('r') || digits.starts_with('R')) {
                if let Ok(rows) = digits[1..].parse::<i64>() {
                    if rows > 1 {
                        const ES_MULTILINE: u32 = 0x0004;
                        const ES_WANTRETURN: u32 = 0x1000;
                        const WS_VSCROLL: u32 = 0x0020_0000;
                        style.add |= ES_MULTILINE | ES_WANTRETURN | WS_VSCROLL;
                    }
                    continue;
                }
            }
        }
        // `gLabel` (control event callback): exclude `Group` and `Grid`.
        if adding
            && digits.len() > 1
            && (digits.starts_with('g') || digits.starts_with('G'))
            && !digits.eq_ignore_ascii_case("group")
            && !digits.eq_ignore_ascii_case("grid")
        {
            style.glabel = digits[1..].to_string();
            // Static controls (Text / Picture) only send STN_CLICKED when
            // SS_NOTIFY (0x0100) is set.
            if matches!(kind, "text" | "picture") {
                const SS_NOTIFY: u32 = 0x0100;
                style.add |= SS_NOTIFY;
            }
            continue;
        }
        let Some(bits) = the_number_of_a_style(digits) else {
            continue;
        };
        if adding {
            style.add |= bits;
        } else {
            style.remove |= bits;
        }
    }
    // The `BS_MULTILINE` of a button: the v1 puts it there when the control
    // has the width or the height written, or when the text of it has a
    // line of its own.
    if matches!(kind, "button" | "checkbox" | "radio") {
        let place = the_place_of_the_add_options(options, kind);
        if place.width.is_some()
            || place.height.is_some()
            || text.contains('\n')
            || text.contains('\r')
        {
            style.add |= BS_MULTILINE;
        }
    }
    style
}

/// The number of a style that a word of the options holds.
///
/// The v1 reads it with the `ATOU` (source `script_gui.cpp:5567` of
/// v1.1.37.02): the number is hexadecimal when it starts with "0x", and
/// decimal otherwise. The word that is not a number is not a style.
fn the_number_of_a_style(text: &str) -> Option<u32> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if let Some(digits) = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X")) {
        if digits.is_empty() || !digits.chars().all(|digit| digit.is_ascii_hexdigit()) {
            return None;
        }
        return u32::from_str_radix(digits, 16).ok();
    }
    if !text.chars().all(|digit| digit.is_ascii_digit()) {
        return None;
    }
    text.parse::<u32>().ok()
}

/// The coordinate of one of the options of the `Gui, Add` of the v1.
///
/// The `rest` is what follows the letter of the option: the `+` of a
/// relative place, the `m` of the margin, the `p` of the control before it,
/// the `s` of the section, or a number.
fn is_signed_number_prefix(text: &str) -> bool {
    let bytes = text.as_bytes();
    match bytes.first() {
        Some(b) if b.is_ascii_digit() => true,
        Some(b'+' | b'-') => bytes.get(1).is_some_and(|b| b.is_ascii_digit()),
        _ => false,
    }
}

fn the_coordinate_of_the_options(rest: &str) -> Option<TheCoordinate> {
    if let Some(after) = rest.strip_prefix('+') {
        // The `x+m` is the edge of the control before it, the margin more.
        if after.eq_ignore_ascii_case("m") {
            return Some(TheCoordinate::AfterThePreviousWithTheMargin);
        }
        if !is_signed_number_prefix(after) {
            return None;
        }
        return Some(TheCoordinate::AfterThePrevious(the_number_of(after)));
    }
    let mut characters = rest.chars();
    match characters.next() {
        Some('m') | Some('M') => {
            let tail = characters.as_str();
            if tail.is_empty() || is_signed_number_prefix(tail) {
                Some(TheCoordinate::FromTheMargin(the_number_of(tail)))
            } else {
                None
            }
        }
        Some('p') | Some('P') => {
            let tail = characters.as_str();
            if tail.is_empty() || is_signed_number_prefix(tail) {
                Some(TheCoordinate::FromThePrevious(the_number_of(tail)))
            } else {
                None
            }
        }
        Some('s') | Some('S') => {
            let tail = characters.as_str();
            if tail.is_empty() || is_signed_number_prefix(tail) {
                Some(TheCoordinate::FromTheSection(the_number_of(tail)))
            } else {
                None
            }
        }
        _ => {
            // The place of the client area of the window.
            if is_signed_number_prefix(rest) {
                Some(TheCoordinate::Absolute(the_number_of(rest)))
            } else {
                None
            }
        }
    }
}

/// The size of one of the options of the `Gui, Add` of the v1.
///
/// The `wp` and the `hp` are the size of the control that was added before
/// it, and the number after them is added to it.
fn the_size_of_the_options(rest: &str) -> Option<TheSize> {
    let mut characters = rest.chars();
    match characters.next() {
        Some('p') | Some('P') => {
            let tail = characters.as_str();
            if tail.is_empty() || is_signed_number_prefix(tail) {
                Some(TheSize::FromThePrevious(the_number_of(tail)))
            } else {
                None
            }
        }
        _ => {
            if is_signed_number_prefix(rest) {
                Some(TheSize::Absolute(the_number_of(rest)))
            } else {
                None
            }
        }
    }
}

/// The number of an option of the v1.
///
/// The number is the one that the option starts with: the sign is part of
/// it, and what is after it is not.
fn the_number_of(text: &str) -> i64 {
    let mut characters = text.chars().peekable();
    let mut number = String::new();
    if let Some(first) = characters.peek() {
        if *first == '-' || *first == '+' {
            number.push(*first);
            characters.next();
        }
    }
    while let Some(one) = characters.peek() {
        if one.is_ascii_digit() {
            number.push(*one);
            characters.next();
        } else {
            break;
        }
    }
    number.parse::<i64>().unwrap_or(0)
}

/// The kind of a control of the v1, in the name the machine knows.
///
/// The v1 spells the kinds its own way: the "DropDownList" is the "ddl" of
/// the machine, the "Trackbar" is the "slider", and the "Tab3" is the
/// "tab". The others are the same name in the two.
fn the_kind_of_the_machine(kind: &str) -> String {
    let kind = kind.trim().to_ascii_lowercase();
    match kind.as_str() {
        "dropdownlist" => "ddl".to_string(),
        "trackbar" => "slider".to_string(),
        "tab2" | "tab3" => "tab".to_string(),
        "pic" => "picture".to_string(),
        _ => kind,
    }
}

/// `GuiControl, SubCommand, ControlID, Value`: a change in a control of the
/// window of the script.
///
/// The v2 has the class of the control, and the tree has the number of it
/// by its `ClassNN`.
///
/// Measured on the binary of v1.1.37.02: the sub-commands
/// that the v1 knows are the "Text", the "Move", the "MoveDraw", the
/// "Focus", the "Disable", the "Enable", the "Hide", the "Show", the
/// "Choose", the "ChooseString" and the "Font"; the "Redraw", the
/// "Delete" and the "Value" are refused with "Parameter #1 invalid.".
pub(crate) fn gui_control(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `GuiControl, SubCommand, ControlID, Value`.
    let sub_command = value_of(values, 0);
    let control = value_of(values, 1);
    let value = value_of(values, 2);
    // The name of the window comes before the sub-command: the
    // `GuiControl, MyGui:Show, MyButton`. The CHM of the v1.1.37.02 says:
    // "To operate upon a window other than the default, include its name
    // or number (or in [v1.1.03+] its HWND) followed by a colon in front
    // of the sub-command."
    let (name, sub) = the_name_and_the_sub_command(&sub_command);
    let window = the_window_of_the_gui(rt, &name, line)?;
    // The failure of a control that is not there is a note and not a
    // stop of the script: measured on the binary of v1.1.37.02 the `GuiControlGet` of a window that does not exist
    // writes the empty text into the variable of the caller and leaves
    // the `ErrorLevel` at one, and the script goes on.
    let answer = (|| -> Result<Value, RuntimeError> {
    match sub.to_ascii_lowercase().as_str() {
        "" | "text" => {
            // The v1: `GuiControl, Text, ControlID, NewText`. The text of
            // the control is written when there is one, and read when
            // there is not.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_value(&window, number as u64, Some(&value))
                .map(Value::text)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "move" | "movedraw" => {
            // The v1: `GuiControl, Move, ControlID, Options`, and the
            // options are the `x` and the `y` and the `w` and the `h`.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            let place = the_place_of_the_add_options(&value, "");
            rt.desktop()
                .v1_gui_place(&window, number as u64, place)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "focus" => {
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_focus(&window, number as u64)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "enable" => the_state_of_a_control(rt, &window, &control, "enabled", true, line),
        "disable" => the_state_of_a_control(rt, &window, &control, "enabled", false, line),
        "hide" => {
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_visible(&window, number as u64, false)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "show" => {
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_visible(&window, number as u64, true)
                .map(|_| Value::Text("".into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "choose" => {
            // The v1: `GuiControl, Choose, ControlID, ItemNumber`.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_choose(&window, number as u64, &value, false)
                .map(|index| Value::Text(index.to_string().into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "choosestring" => {
            // The v1: `GuiControl, ChooseString, ControlID, ItemText`.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_choose(&window, number as u64, &value, true)
                .map(|index| Value::Text(index.to_string().into()))
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "+redraw" | "-redraw" => {
            // The v1: the redraw of a control. The binary of the v1.1.37.02
            // accepts it, and the machine of the tree paints itself.
            Ok(Value::Text("".into()))
        }
        "font" => Err(RuntimeError {
            message: the_reason_of_a_gui_control_that_the_tree_does_not_carry(sub.trim()),
            line,
        }),
        "delete" => Err(RuntimeError {
            message: "the v1 does not have the delete of a control: the CHM of the v1.1.37.02 says \
                      of the sub-command: \"Delete : Not yet implemented.\""
                .to_string(),
            line,
        }),
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the GuiControl is not known."),
            line,
        }),
    }
    })();
    the_answer_of_an_act(rt, "GuiControl", answer)
}

/// The name of the window and the sub-command of a command of the
/// `GuiControl`.
///
/// The name of the window comes before the sub-command, with a colon
/// between the two.
fn the_name_and_the_sub_command(text: &str) -> (String, String) {
    match text.trim().split_once(':') {
        Some((name, rest)) => (name.trim().to_string(), rest.trim().to_string()),
        None => (String::new(), text.trim().to_string()),
    }
}

/// The reason of a sub-command of the `GuiControl` that the tree does not
/// carry.
///
/// The tree lays the controls out its own way, and it has no font for them.
fn the_reason_of_a_gui_control_that_the_tree_does_not_carry(sub_command: &str) -> String {
    match sub_command.to_ascii_lowercase().as_str() {
        "font" => "the tree does not carry the font of a control of the window of the script: the machine of the tree has no font for it.".to_string(),
        _ => "the tree does not move a control of the window of the script: the machine of the tree lays the controls out its own way.".to_string(),
    }
}

/// The number that names a control of the window of the script.
fn the_number_of_a_control(
    rt: &mut Runtime<'_>,
    window: &str,
    control: &str,
    line: u32,
) -> Result<i64, RuntimeError> {
    rt.desktop()
        .v1_gui_number_named(window, control)
        .map_err(|reason| crate::builtins::desktop_failure(reason, line))
}

/// The state of a control of the window of the script: the "enabled" of it.
fn the_state_of_a_control(
    rt: &mut Runtime<'_>,
    window: &str,
    control: &str,
    what: &str,
    on: bool,
    line: u32,
) -> Result<Value, RuntimeError> {
    let number = the_number_of_a_control(rt, window, control, line)?;
    rt.desktop()
        .v1_gui_flag(window, number as u64, what, Some(on))
        .map(|_| Value::Text("".into()))
        .map_err(|reason| crate::builtins::desktop_failure(reason, line))
}

/// `GuiControlGet, OutputVar, SubCommand, ControlID, Value`: what a control
/// of the window of the script has.
///
/// The v1 writes the answer into the variable of the caller, and the
/// sub-command says what the answer is: the text of the control when the
/// sub-command is not there, the place of it with the "Pos", the name of
/// it with the "Name" and the handle of it with the "Hwnd".
///
/// Measured on the binary of v1.1.37.02: the sub-commands
/// that the v1 knows are the "Pos", the "Focus", the "FocusV", the
/// "Enabled", the "Visible", the "Hwnd" and the "Name"; the "Type", the
/// "Value", the "Text", the "Style", the "ExStyle", the "Choice" and the
/// "Checked" are refused with "Parameter #2 invalid.".
pub(crate) fn gui_control_get(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `GuiControlGet, OutputVar, SubCommand, ControlID, Value`.
    let output = value_of(values, 0);
    let sub_command = value_of(values, 1);
    let mut control = value_of(values, 2);
    // The `Value` of the `GuiControlGet` is the "Text" of the caption of
    // the control, when the script asks for it.
    let value = value_of(values, 3);
    // The name of the window comes before the sub-command.
    let (name, sub) = the_name_and_the_sub_command(&sub_command);
    let window = the_window_of_the_gui(rt, &name, line)?;
    // The name of the variable of the caller names the control when the
    // control is left out: the CHM of the v1.1.37.02 says: "If blank or
    // omitted, it behaves as though the name of the output variable was
    // specified."
    if control.trim().is_empty() {
        control = output.clone();
    }
    // The v1: the window that is not there and the control that is not
    // there are not failures that stop the script, and the v1 says
    // nothing about them: the text of the caller stays empty and the
    // `ErrorLevel` is one.
    //
    // Measured on the binary of v1.1.37.02: the
    // `GuiControlGet, MyVar` of a control that does not exist writes the
    // empty text into `MyVar`, leaves the `ErrorLevel` at one, and the
    // script goes on without a word.
    //
    // The "Focus" and the "FocusV" are the two sub-commands that do not
    // name a control: they ask for the control that has the focus.
    let lower = sub.to_ascii_lowercase();
    let control_is_named = lower != "focus" && lower != "focusv";
    let window_is_there = rt.desktop().v1_gui_is_open(&window).unwrap_or(false);
    let control_is_there = !control_is_named
        || rt
            .desktop()
            .v1_gui_number_named(&window, &control)
            .is_ok();
    if !window_is_there || !control_is_there {
        crate::v1::commands::write_the_answer(
            rt,
            "GuiControlGet",
            values,
            0,
            Value::Text("".into()),
            line,
        )?;
        rt.set_the_error_level(Value::Int(1));
        return Ok(Value::Text("".into()));
    }
    // The failure of a control that is not there is a note and not a
    // stop of the script: the text of the caller stays empty and the
    // script goes on.
    let answer = (|| -> Result<Value, RuntimeError> {
    match sub.to_ascii_lowercase().as_str() {
        "" => {
            // The v1: the contents of the control are the answer when the
            // sub-command is not there, and the caption of it is the answer
            // when the `Value` is the "Text".
            //
            // The CHM of the v1.1.37.02 says: "To unconditionally retrieve
            // the text/caption of a CheckBox, Radio, DropDownList or
            // ComboBox rather than its contents, specify the word Text for
            // Value."
            //
            // Measured on the binary of v1.1.37.02: the
            // `GuiControlGet, out, , CHK` of a check box that the script
            // made checked answers "1", and the same command with the
            // "Text" in the `Value` answers "Check".
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            if value.trim().eq_ignore_ascii_case("text") {
                rt.desktop()
                    .v1_gui_caption(&window, number as u64)
                    .map(Value::text)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            } else {
                rt.desktop()
                    .v1_gui_value(&window, number as u64, None)
                    .map(Value::text)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))
            }
        }
        "pos" => {
            // The v1: the place of the control and the size of it are four
            // numbers, and they go into four variables: the name of the
            // caller with the letter `X`, `Y`, `W` and `H` at the end.
            //
            // The variables follow the scope of their names: they are the
            // globals of the script when a `global` declaration outside of
            // any function declared them, and the locals of the function
            // otherwise.
            //
            // The CHM of the v1.1.37.02 says of the sub-command: "Within a
            // function, to create a set of variables that is global
            // instead of local, declare OutputVar as a global variable
            // prior to using this command."
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            let (x, y, width, height) = rt
                .desktop()
                .v1_gui_control_pos(&window, number as u64)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            rt.assign_the_variable(&format!("{output}X"), Value::Int(x));
            rt.assign_the_variable(&format!("{output}Y"), Value::Int(y));
            rt.assign_the_variable(&format!("{output}W"), Value::Int(width));
            rt.assign_the_variable(&format!("{output}H"), Value::Int(height));
            Ok(Value::Text("".into()))
        }
        "focus" => rt
            .desktop()
            .v1_gui_focused(&window)
            .map(Value::text)
            .map_err(|reason| crate::builtins::desktop_failure(reason, line)),
        "focusv" => {
            // The v1: the name of the variable of the control that has the
            // focus.
            //
            // Measured on the binary of v1.1.37.02: the
            // answer is the name of the variable of the control, and the
            // text of the control when it has no variable of its own: the
            // CHM of the v1.1.37.02 says of the sub-command: "If that
            // control lacks an associated variable, the first 63
            // characters of the control's text/caption is retrieved
            // instead". The `GuiControl, PVwin: Focus, Press me` of a
            // button without a variable answers "Press me".
            let classnn = rt
                .desktop()
                .v1_gui_focused(&window)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            if classnn.is_empty() {
                return Ok(Value::Text("".into()));
            }
            let number = rt
                .desktop()
                .v1_gui_number_named(&window, &classnn)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            let name = rt
                .desktop()
                .v1_gui_control_name(&window, number as u64)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            if !name.is_empty() {
                return Ok(Value::Text(name.into()));
            }
            // The control has no variable of its own: the text of it is
            // the answer, and it is cut at the sixty-three characters.
            let text = rt
                .desktop()
                .v1_gui_caption(&window, number as u64)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            Ok(Value::Text(text.chars().take(63).collect::<String>().into()))
        }
        "enabled" => {
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_flag(&window, number as u64, "enabled", None)
                .map(Value::Int)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "visible" => {
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_flag(&window, number as u64, "visible", None)
                .map(Value::Int)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        "hwnd" => {
            // The v1: the handle of the control is a number with the "0x"
            // in front of it.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            let handle = rt
                .desktop()
                .v1_gui_handle(&window, number as u64)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            Ok(Value::Text((format!("0x{handle:x}")).into()))
        }
        "name" => {
            // The v1: the name of the variable of the control, and it is
            // empty when the control has none.
            let number = the_number_of_a_control(rt, &window, &control, line)?;
            rt.desktop()
                .v1_gui_control_name(&window, number as u64)
                .map(Value::text)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))
        }
        _ => Err(RuntimeError {
            message: format!(
                "the sub-command {sub_command} of the GuiControlGet is not known."
            ),
            line,
        }),
    }
    })();
    // The `ErrorLevel` of the `GuiControlGet` is one when the control is
    // not there: measured on the binary of v1.1.37.02, the
    // `GuiControlGet` of a window that does not exist writes the empty
    // text and leaves the `ErrorLevel` at one, and the script goes on.
    the_answer_that_goes_on(rt, "GuiControlGet", values, line, answer, true)
}

// -- the keyboard of the machine ----------------------------------------------------

/// `InputHook(Options, EndKeys, MatchList)`: the hook of the keyboard.
///
/// The v2 has the `InputHook` as a class, and the v1 has it as a function.
/// The options of the v1 are the letters of the v2: the "B" and the "C"
/// and the "E" and the "H" and the "I" and the "L" and the "M" and the "N"
/// and the "O" and the "P" and the "R" and the "S" and the "T" and the "V".
pub(crate) fn input_hook(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let options = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    let end_keys = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let match_list = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    // The options of the v1 become the options of the v2, and the letters
    // that the v2 does not have are refused with the reason.
    let mut arguments: Vec<Value> = Vec::new();
    if !options.trim().is_empty() {
        arguments.push(Value::Text((the_options_of_the_v2(&options, line)?).into()));
    }
    if !end_keys.trim().is_empty() {
        arguments.push(Value::Text(end_keys.into()));
    }
    if !match_list.trim().is_empty() {
        arguments.push(Value::Text(match_list.into()));
    }
    match crate::classes::construct("InputHook", &arguments, rt, line) {
        Some(answer) => answer.map_err(|failure| RuntimeError {
            message: crate::builtins::the_message_of(failure),
            line,
        }),
        None => Err(RuntimeError {
            message: "the class of the InputHook is not known.".to_string(),
            line,
        }),
    }
}

/// The options of the v1 as the options of the v2.
///
/// The letters of the v1 are the letters of the v2, and the ones that the
/// v2 does not have are refused with the reason.
fn the_options_of_the_v2(options: &str, line: u32) -> Result<String, RuntimeError> {
    // The number that follows a letter belongs to it: the `L0` is the
    // limit of the input, and the `0` of it is not an option of its own.
    // Read in the CHM of the v1.1.37.02 (`InputHook.htm`) and measured on
    // the binary of v1.1.37.02: the `InputHook("V L0")` of
    // a script of the v1 is the hook that waits with no limit.
    let mut carried = String::new();
    for letter in options.split_whitespace() {
        for one in letter.chars() {
            if one.is_ascii_digit() || one == '-' || one == '+' || one == '.' {
                carried.push(one);
                continue;
            }
            let upper = one.to_ascii_uppercase();
            match upper {
                'B' | 'C' | 'E' | 'H' | 'I' | 'L' | 'M' | 'N' | 'O' | 'P' | 'R' | 'S' | 'T' | 'V'
                | 'W' => carried.push(upper),
                _ => {
                    return Err(RuntimeError {
                        message: format!("the option {one} of the InputHook is not known."),
                        line,
                    })
                }
            }
        }
        carried.push(' ');
    }
    Ok(carried.trim_end().to_string())
}

// -- the menu of the machine --------------------------------------------------------

/// `Menu, MenuName, SubCommand, Value1, Value2, Value3, Value4`: the menu of
/// the machine.
///
/// The v1 has the menu as a command and the v2 has it as a class.
///
/// The sub-commands that the tree does not carry -- the "Standard", the
/// "NoStandard", the "Icon", the "NoIcon", the "Tip", the "Show", the
/// "Color", the "Click", the "MainWindow", the "NoMainWindow" and the
/// "UseErrorLevel" -- are refused with the reason: they are about the menu
/// bar of a window of the machine, and the menus of the tree are menus of
/// the script.
///
/// Measured on the binary of v1.1.37.02: the sub-commands
/// that the v1 knows are the "Add", the "Insert", the "Delete", the
/// "DeleteAll", the "Rename", the "Check", the "Uncheck", the
/// "ToggleCheck", the "Enable", the "Disable", the "ToggleEnable", the
/// "Default", the "NoDefault", the "Standard", the "NoStandard", the
/// "Icon", the "NoIcon", the "Tip", the "Show", the "Color", the "Click",
/// the "MainWindow", the "NoMainWindow" and the "UseErrorLevel".
pub(crate) fn menu(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Menu, MenuName, SubCommand, Value1, ...`.
    let name = value_of(values, 0);
    let sub_command = value_of(values, 1);
    let value = value_of(values, 2);
    // The v1: the menu of the tray of the script is the one that the name
    // "Tray" names, and the sub-commands of it are the ones of the icon of
    // the tray.
    if name.trim().eq_ignore_ascii_case("tray") {
        let answer = the_tray_of_the_script(rt, values, line);
        return the_answer_of_an_act(rt, "Menu", answer);
    }
    let answer = match sub_command.trim().to_ascii_lowercase().as_str() {
        "add" => {            // The v1: `Menu, MyMenu, Add, ItemName, Callback, Options`.
            let callback = value_of(values, 3);
            crate::v1::menus::add_item(rt, &name, &value, &callback, line)
        }
        "insert" => {
            // The v1: `Menu, MyMenu, Insert, ItemName, Callback, Options`.
            // The item goes above the last one of the menu.
            let callback = value_of(values, 3);
            crate::v1::menus::insert_item(rt, &name, &value, &callback, line)
        }
        "delete" => {
            // The v1: the `Menu, MyMenu, Delete` with no item is the
            // removal of the menu itself.
            if value.trim().is_empty() {
                crate::v1::menus::delete_the_menu(rt, &name, line)
            } else {
                crate::v1::menus::delete_item(rt, &name, &value, line)
            }
        }
        "deleteall" => crate::v1::menus::delete_all(rt, &name, line),
        "disable" => crate::v1::menus::set_enabled(rt, &name, &value, false, line),
        "enable" => crate::v1::menus::set_enabled(rt, &name, &value, true, line),
        "toggleenable" => crate::v1::menus::toggle_enabled(rt, &name, &value, line),
        "check" => crate::v1::menus::set_checked(rt, &name, &value, true, line),
        "uncheck" => crate::v1::menus::set_checked(rt, &name, &value, false, line),
        "togglecheck" => crate::v1::menus::toggle_checked(rt, &name, &value, line),
        "default" => crate::v1::menus::set_default(rt, &name, &value, line),
        "nodefault" => crate::v1::menus::no_default(rt, &name, line),
        "rename" => {
            // The v1: `Menu, MyMenu, Rename, NewName, OldName`. The new
            // name comes before the old one.
            let new_name = value_of(values, 2);
            let old_name = value_of(values, 3);
            crate::v1::menus::rename_item(rt, &name, &old_name, &new_name, line)
        }
        "icon" => {
            // The v1: `Menu, MyMenu, Icon, ItemName, IconFile, IconNumber`.
            // The item comes before the file, and the file before the
            // number.
            let item = value_of(values, 2);
            let icon = value_of(values, 3);
            let number = value_of(values, 4).trim().parse::<i64>().unwrap_or(1);
            crate::v1::menus::set_icon(rt, &name, &item, &icon, number, line)
        }
        "show" => {
            // The v1: `Menu, MyMenu, Show, X, Y`. The place of the menu
            // is the place of the mouse when the script does not name it.
            let x = value_of(values, 2);
            let y = value_of(values, 3);
            crate::v1::menus::show(rt, &name, &x, &y, line)
        }
        "useerrorlevel" => Ok(Value::Text("".into())),
        "standard" | "nostandard" | "noicon" | "tip" | "color" | "click" | "mainwindow"
        | "nomainwindow" => Err(RuntimeError {
            message: the_reason_of_a_menu_that_the_tree_does_not_carry(sub_command.trim()),
            line,
        }),
        _ => Err(RuntimeError {
            message: format!("the sub-command {sub_command} of the Menu is not known."),
            line,
        }),
    };
    // The v1: the bar of the menu of a window is the menu of the
    // script, and the items that the script adds to the menu after
    // the bar is attached are in the bar of the window.
    //
    // The source of the v1 (`script_gui.cpp:1913-1919` of v1.1.37.02)
    // writes the bar of the windows that use the menu again, when
    // the menu changes.
    //
    // Measured on the binary of v1.1.37.02: the main
    // window of a script of the v1 has five items in the bar of
    // the menu of it, and the tree made a bar with no items: the
    // items of the menu of the script are added after the bar is
    // attached.
    let items = crate::v1::menus::the_items_of_the_menu(rt, &name);
    let machine = crate::v1::menus::the_machine_menu_of(rt, &name);
    let _ = rt
        .desktop()
        .v1_gui_menu_bar_is_updated(&name, &items, machine.unwrap_or(0));
    // The bar of a window shows the submenus of the menu of the script,
    // and a change in a submenu is a change in the bar.
    for holder in crate::v1::menus::the_menus_that_have_the_submenu(rt, &name) {
        let items = crate::v1::menus::the_items_of_the_menu(rt, &holder);
        let machine = crate::v1::menus::the_machine_menu_of(rt, &holder);
        let _ = rt
            .desktop()
            .v1_gui_menu_bar_is_updated(&holder, &items, machine.unwrap_or(0));
    }
    the_answer_of_an_act(rt, "Menu", answer)
}

/// The sub-command of the menu of the tray of the script.
///
/// The v1: the `Menu, Tray, Icon, IconFile, IconNumber` puts the icon of
/// the file in the tray of the script, and the icon that cannot be loaded
/// is an error.
///
/// Measured on the binary of v1.1.37.02: the `Menu, Tray,
/// Icon` of a file that is not there is an error that the `Try` of the
/// script takes.
fn the_tray_of_the_script(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
) -> Result<Value, RuntimeError> {
    let sub_command = value_of(values, 1);
    let file = value_of(values, 2);
    match sub_command.trim().to_ascii_lowercase().as_str() {
        "icon" => {
            if file.trim().is_empty() {
                return Err(RuntimeError {
                    message: "the file of the icon of the tray may not be empty.".to_string(),
                    line,
                });
            }
            let number = value_of(values, 3).trim().parse::<i64>().unwrap_or(1);
            // The icon is loaded before it goes into the tray: an icon
            // that cannot be loaded is the failure of the command.
            //
            // Measured on the binary of v1.1.37.02: the
            // `Menu, Tray, Icon` of a file that is not there is the error
            // "Can't load icon." with the path of it.
            let canonical = rt
                .desktop()
                .canonical_path(file.trim())
                .unwrap_or_else(|_| file.trim().to_string());
            if rt.desktop().icon_of_a_file(&canonical, number).is_err() {
                return Err(RuntimeError {
                    message: "Can't load icon.".to_string(),
                    line,
                });
            }
            rt.set_the_icon_of_the_script(canonical.clone(), number);
            // The tray of the session takes the icon: a tray that refuses
            // is said in the report, and it is not a failure of the script.
            if let Some(reason) = rt.desktop().tray_set_icon(&canonical).err() {
                rt.note(format!(
                    "the tray of the session did not take the icon: {reason}"
                ));
            }
            Ok(Value::Text("".into()))
        }
        _ => Err(RuntimeError {
            message: the_reason_of_a_menu_that_the_tree_does_not_carry(sub_command.trim()),
            line,
        }),
    }
}

/// The reason of a sub-command of the `Menu` that the tree does not carry.
///
/// The menus of the tree are menus of the script: they have items, and the
/// items have a name, a mark and a state. The bar of a window of the
/// machine is another thing, and the tree does not carry it.
fn the_reason_of_a_menu_that_the_tree_does_not_carry(sub_command: &str) -> String {
    match sub_command.to_ascii_lowercase().as_str() {
        "standard" | "nostandard" => "the tree does not carry the standard items of a menu: the menus of the tree have the items that the script puts in them.".to_string(),
        _ => "the tree does not carry the menu bar of a window of the machine: the menus of the tree are menus of the script.".to_string(),
    }
}

/// `MenuGetHandle(MenuName)`: the handle of a menu of the machine.
///
/// The v2 has the `MenuFromHandle`, and it is the other way round: the v1
/// asks the handle of a menu by its name.
pub(crate) fn menu_get_handle(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let name = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    crate::v1::menus::the_handle_of(rt, &name, line)
}

/// `MenuGetName(Handle)`: the name of a menu of the machine.
pub(crate) fn menu_get_name(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let handle = values
        .first()
        .map(|value| value.to_text())
        .unwrap_or_default();
    crate::v1::menus::the_name_of(rt, &handle, line)
}

// -- the window of the progress -----------------------------------------------------

/// `Progress, ProgressParam1, SubText, MainText, WinTitle, FontName`: the
/// window of the progress.
///
/// The v2 has the progress as a control of a window of the script, and the
/// v1 has it as a window of its own. The tree has one window of the
/// script, and the progress takes its place: a script that has a window of
/// the GUI and a progress at the same time has the refusal of the desktop
/// as the answer.
pub(crate) fn progress(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `Progress, ProgressParam1, SubText, MainText, ...`.
    let percent = value_of(values, 0);
    let answer = if percent.trim().eq_ignore_ascii_case("off") {
        rt.desktop()
            .gui_destroy(0)
            .map(|_| Value::Text("".into()))
            .map_err(|reason| crate::builtins::desktop_failure(reason, line))
    } else {
        let number = percent.trim().parse::<i64>().unwrap_or(0);
        let handle = rt
            .desktop()
            .gui_open("Progress")
            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
        let (control, _) = rt
            .desktop()
            .gui_add(handle, "Progress", &number.to_string())
            .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
        rt.desktop()
            .gui_value(control, None)
            .map(Value::text)
            .map_err(|reason| crate::builtins::desktop_failure(reason, line))
    };
    the_answer_of_an_act(rt, "Progress", answer)
}

/// `SplashTextOn, Width, Height, Title, Text`: the window of a text.
///
/// The v1 has it as a window of its own, and the v2 has it as a window of
/// the script.
pub(crate) fn splash_text_on(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The v1: `SplashTextOn, Width, Height, Title, Text`.
    let width = values.get(1).map(|value| value.to_text()).unwrap_or_default();
    let height = values.get(2).map(|value| value.to_text()).unwrap_or_default();
    let title = values.get(3).map(|value| value.to_text()).unwrap_or_default();
    let text = values.get(4).map(|value| value.to_text()).unwrap_or_default();
    let handle = rt
        .desktop()
        .gui_open(&title)
        .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
    let (control, _) = rt
        .desktop()
        .gui_add(handle, "Text", &text)
        .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
    let width = width.trim().parse::<i64>().unwrap_or(200);
    let height = height.trim().parse::<i64>().unwrap_or(0);
    let answer = rt
        .desktop()
        .gui_size(control, width, height)
        .map(|_| Value::Text("".into()))
        .map_err(|reason| crate::builtins::desktop_failure(reason, line));
    // The window of the splash is remembered: the `SplashTextOff` takes
    // it away, and it is the only one that it takes away.
    if answer.is_ok() {
        rt.set_the_splash_of_the_script(handle);
    }
    answer
}

/// `SplashTextOff`: the window of a text goes away.
///
/// Measured on the binary of v1.1.37.02: the command takes no parameter,
/// and it takes the window that the `SplashTextOn` made off the screen.
///
/// Measured on the binary of v1.1.37.02: with no splash the command
/// does nothing at all -- it is not a failure, the script goes on, and
/// the `ErrorLevel` stays at zero.
pub(crate) fn splash_text_off(
    rt: &mut Runtime<'_>,
    _values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    if rt.take_the_splash_of_the_script().is_none() {
        // There is no splash: the v1 does nothing and goes on.
        return Ok(Value::Text("".into()));
    }
    rt.desktop()
        .gui_hide()
        .map(|_| Value::Text("".into()))
        .map_err(|reason| crate::builtins::desktop_failure(reason, line))
}

// -- the callback of the machine ----------------------------------------------------
/// `RegisterCallback(Function, Options, ParamCount, EventInfo)`: the address
/// of a function of the machine that calls a function of the script.
///
/// The address is a trampoline of the pool of the callbacks: the machine
/// calls it, and it runs the function of the script. The callback runs on
/// the thread of the script while a `DllCall` goes on; the number of the
/// parameters is the one the script gives, or the one the function
/// declares.
pub(crate) fn register_callback(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    #[cfg(feature = "std")]
    {
        let first = values.first().cloned().unwrap_or(Value::Text("".into()));
        // The v1: the callback of a method of a class is the method
        // itself. A script of the v1 asks for the callback of the
        // method of its taskbar interface, and the machine takes the hook
        // with it.
        //
        // Measured on the binary of the v1.1.37.02: the
        // `RegisterCallback(this.MyMethod)` of the v1 answers the address
        // of the callback of the method.
        if let Value::Object(object) = &first {
            let class = object.borrow().class.clone();
            if class == "func" || class == "boundfunc" {
                let params = match values.get(2).and_then(|value| value.to_number()) {
                    Some(number) if number >= 0.0 => number as usize,
                    _ => 4,
                };
                let address = rt
                    .desktop()
                    .create_callback(&first, params)
                    .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
                return Ok(Value::Int(address as i64));
            }
        }
        let name = value_of(values, 0);
        // The v1: the callback of a name that is not there is the empty
        // text, and it is not an error. A script of the v1 asks for
        // the callback of a method of it that the tree does not know, and
        // the machine takes the hook with the address that is nothing.
        //
        // Measured on the binary of the v1.1.37.02: the
        // `RegisterCallback("")` and the `RegisterCallback("NoSuchName")`
        // of the v1 answer the empty text, and the script goes on.
        // The v1: the name of a method of a class is the name of the
        // class and the name of the method, with a dot between them.
        //
        // Measured on the binary of the v1.1.37.02: the
        // `RegisterCallback("MyClass.OtherMethod")` of the v1 answers the
        // address of the callback of the method.
        if let Some(method) = rt.the_method_of_the_qualified_name(&name) {
            let params = match values.get(2).and_then(|value| value.to_number()) {
                Some(number) if number >= 0.0 => number as usize,
                _ => 4,
            };
            let address = rt
                .desktop()
                .create_callback(&method, params)
                .map_err(|reason| crate::builtins::desktop_failure(reason, line))?;
            return Ok(Value::Int(address as i64));
        }
        if name.is_empty() || rt.the_function_named(&name).is_none() {
            return Ok(Value::Text("".into()));
        }
        let params = match values.get(2).and_then(|value| value.to_number()) {
            Some(number) if number >= 0.0 => number as usize,
            _ => rt.the_number_of_parameters_of(&name).unwrap_or(0),
        };
        let event_info = values
            .get(3)
            .and_then(|value| value.to_number())
            .map(|number| number as i64);
        let address = crate::callback::register_with_event_info(&name, params, event_info);
        if address == 0 {
            return Err(RuntimeError {
                message: "the pool of the callbacks is full.".to_string(),
                line,
            });
        }
        Ok(Value::Int(address as i64))
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = (rt, values);
        Err(RuntimeError {
            message: "the callback of the machine needs the std of the tree.".to_string(),
            line,
        })
    }
}
/// The text of the parameter of this place, or the empty text.
fn value_of(values: &[Value], at: usize) -> String {
    values.get(at).map(|value| value.to_text()).unwrap_or_default()
}
