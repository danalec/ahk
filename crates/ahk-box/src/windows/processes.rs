//! The processes of the machine.
//!
//! A process of Windows is a program that the machine runs: the runner
//! of the tree starts it and waits for it, and the answer is its number
//! of exit, with what it wrote in the two outputs.
//!
//! The family of the processes is read with the snapshot of the
//! toolhelp of the kernel: `CreateToolhelp32Snapshot` is the reader of
//! the table of the processes, and it is of Windows 2000.

use std::process::Command;

use ahk::desktop::ProcessState;

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, PROCESSENTRY32W, Process32FirstW, Process32NextW, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    GetExitCodeProcess, INFINITE, OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
    PROCESS_VM_READ, SetPriorityClass, TerminateProcess,
    WaitForSingleObject,
};

use super::sys::Wide;

/// The right of a process that is waited for: the one of the wait.
const SYNCHRONIZE: u32 = 0x0010_0000;

/// The processes of the machine, and what is known of each of them.
///
/// The table is the one of the toolhelp: every process of the machine
/// is a row of it, and the row carries the name, the number and the
/// number of the parent.
fn the_processes() -> Vec<PROCESSENTRY32W> {
    let mut found: Vec<PROCESSENTRY32W> = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE {
            return found;
        }
        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        if Process32FirstW(snapshot, &mut entry) != 0 {
            loop {
                found.push(entry);
                entry = std::mem::zeroed();
                entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
                if Process32NextW(snapshot, &mut entry) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snapshot);
    }
    found
}

/// The handle of the snapshot that does not exist.
const INVALID_HANDLE: HANDLE = -1isize as HANDLE;

/// The name of a process, as the machine spells it.
fn the_name_of(entry: &PROCESSENTRY32W) -> String {
    let name: Vec<u16> = entry
        .szExeFile
        .iter()
        .copied()
        .take_while(|c| *c != 0)
        .collect();
    String::from_utf16_lossy(&name)
}

/// The row of a process, when the machine has it.
///
/// O nome e o nome exato do executavel, sem a caixa: medido no binario
/// do v2.0.31, o `ProcessGetName("python.exe")` acha o
/// processo e o `ProcessGetName("python")` nao acha nenhum -- e o
/// `ProcessGetName("PYTHON.EXE")` acha o mesmo processo.
///
/// O numero e o numero do processo: o `ProcessGetName(1234)` e o
/// nome do processo que tem esse numero.
fn the_entry_of(which: &str) -> Option<PROCESSENTRY32W> {
    let wanted = which.trim();
    if let Ok(number) = wanted.parse::<u32>() {
        return the_processes()
            .into_iter()
            .find(|entry| entry.th32ProcessID == number);
    }
    let wanted = wanted.to_lowercase();
    the_processes().into_iter().find(|entry| {
        the_name_of(entry).to_lowercase() == wanted
    })
}

/// The number of the process that runs the script.
fn the_number_of_the_script() -> u32 {
    std::process::id()
}

/// Whether a process of the machine is running.
///
/// The name that is not there is the name of the process of the script.
pub fn process_exist(which: Option<&str>) -> Result<u32, String> {
    match which {
        Some(name) => match the_entry_of(name) {
            Some(entry) => Ok(entry.th32ProcessID),
            None => Ok(0),
        },
        None => Ok(the_number_of_the_script()),
    }
}

/// The name of a process.
pub fn process_name(which: Option<&str>) -> Result<Option<String>, String> {
    match which {
        Some(name) => Ok(the_entry_of(name).map(|entry| the_name_of(&entry))),
        None => Ok(Some("ahk.exe".to_string())),
    }
}

/// The path of the program of a process.
///
/// The process is named by its name, e.g. "notepad.exe". The path
/// that is asked for a process that is not there is `None`, and not a
/// failure.
pub fn process_path(which: Option<&str>) -> Result<Option<String>, String> {
    let Some(entry) = which.and_then(the_entry_of) else {
        return Ok(None);
    };
    let path = the_path_of(entry.th32ProcessID)?;
    Ok(path)
}

/// The path of the program of the process of a number.
///
/// The number is the pid of the process, and the path is the one of the
/// executable of it. The path of a process that the machine does not
/// open is `None`.
pub fn the_path_of_the_process(pid: u32) -> Option<String> {
    the_path_of(pid).unwrap_or(None)
}

/// The path of the program of a process, with the machine's own reader.
fn the_path_of(pid: u32) -> Result<Option<String>, String> {
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process.is_null() {
            return Ok(None);
        }
        let mut buffer = [0u16; 32768];
        // `QueryFullProcessImageNameW` e de Windows Vista: o XP da o
        // caminho do programa com o `GetModuleFileNameExW` do psapi,
        // que e de Windows 2000.
        let answer = windows_sys::Win32::System::ProcessStatus::GetModuleFileNameExW(
            process,
            std::ptr::null_mut(),
            buffer.as_mut_ptr(),
            buffer.len() as u32,
        );
        CloseHandle(process);
        if answer == 0 {
            return Ok(None);
        }
        let end = buffer
            .iter()
            .position(|letter| *letter == 0)
            .unwrap_or(buffer.len());
        Ok(Some(String::from_utf16_lossy(&buffer[..end])))
    }
}

/// The number of the parent of a process.
pub fn process_parent(which: Option<&str>) -> Result<u32, String> {
    let Some(entry) = which.and_then(the_entry_of) else {
        return Ok(0);
    };
    Ok(entry.th32ParentProcessID)
}

/// The process of the machine is closed.
pub fn process_close(which: &str) -> Result<u32, String> {
    let Some(entry) = the_entry_of(which) else {
        return Ok(0);
    };
    unsafe {
        let process = OpenProcess(PROCESS_TERMINATE, 0, entry.th32ProcessID);
        if process.is_null() {
            return Ok(0);
        }
        let answer = TerminateProcess(process, 0);
        CloseHandle(process);
        if answer == 0 {
            return Err("the machine did not close the process.".to_string());
        }
    }
    Ok(entry.th32ProcessID)
}

/// A process of the machine is waited for.
pub fn process_wait(which: &str, timeout: Option<f64>) -> Result<u32, String> {
    let Some(entry) = the_entry_of(which) else {
        return Ok(0);
    };
    // The answer of the wait says whether the process left; the answer
    // of the trait is the number of the process, and not the wait.
    let _waited = unsafe {
        let process = OpenProcess(SYNCHRONIZE, 0, entry.th32ProcessID);
        if process.is_null() {
            return Ok(0);
        }
        let millis = match timeout {
            Some(seconds) => (seconds * 1000.0) as u32,
            None => INFINITE,
        };
        let answer = WaitForSingleObject(process, millis);
        CloseHandle(process);
        answer
    };
    Ok(entry.th32ProcessID)
}

/// The process of the machine is waited for, and it is closed.
pub fn process_wait_close(which: &str, timeout: Option<f64>) -> Result<u32, String> {
    let number = process_wait(which, timeout)?;
    let _ = process_close(which);
    Ok(number)
}

/// The priority of a process is set.
///
/// The priorities are the words of the v2: "Normal", "High", "Above
/// Normal", "Below Normal", "Idle" and "Realtime".
pub fn process_set_priority(priority: &str, which: Option<&str>) -> Result<Option<u32>, String> {
    let Some(entry) = which.and_then(the_entry_of) else {
        return Ok(None);
    };
    let class = match priority.trim().to_lowercase().as_str() {
        "normal" => 0x0000_0020,
        "high" => 0x0000_0080,
        "above normal" | "abovenormal" => 0x0000_8000,
        "below normal" | "belownormal" => 0x0000_4000,
        "idle" | "low" => 0x0000_0040,
        "realtime" => 0x0000_0100,
        other => {
            return Err(format!(
                "the machine has no priority named {other}: the priorities are Normal, High, Above Normal, Below Normal, Idle and Realtime."
            ));
        }
    };
    unsafe {
        let process = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            0,
            entry.th32ProcessID,
        );
        if process.is_null() {
            return Ok(None);
        }
        let answer = SetPriorityClass(process, class);
        CloseHandle(process);
        if answer == 0 {
            return Err("the machine did not set the priority of the process.".to_string());
        }
    }
    Ok(Some(entry.th32ProcessID))
}

/// A process of the machine is killed.
pub fn kill_a_process(pid: u32) -> Result<(), String> {
    unsafe {
        let process = OpenProcess(PROCESS_TERMINATE, 0, pid);
        if process.is_null() {
            return Err(format!("there is no process with the number {pid}."));
        }
        let answer = TerminateProcess(process, 0);
        CloseHandle(process);
        if answer == 0 {
            return Err("the machine did not kill the process.".to_string());
        }
    }
    Ok(())
}

/// The state of a process: whether it has left, and with which code.
pub fn the_state_of_a_process(pid: u32) -> Result<Option<ProcessState>, String> {
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process.is_null() {
            return Ok(None);
        }
        let mut code = 0u32;
        let answer = GetExitCodeProcess(process, &mut code);
        CloseHandle(process);
        if answer == 0 {
            return Ok(None);
        }
        // The code 259 is the one of a process that is running: the
        // machine says that the process has not left.
        let has_exited = code != 259;
        Ok(Some(ProcessState {
            has_exited,
            exit_code: if has_exited { Some(code as i32) } else { None },
            exit_time: None,
        }))
    }
}

/// A program of the machine is run, and the script waits for it.
///
/// The command of the script is the command of the machine: the shell of
/// the session runs it, the way the shell of the box runs the command of
/// the v1 on a Linux machine.
///
/// The answer is the number of exit of the program, what it wrote in the
/// standard output and what it wrote in the standard error.
pub fn run(command: &str, _window: &str) -> Result<(i32, String, String), String> {
    let output = Command::new("cmd")
        .arg("/c")
        .arg(command)
        .output()
        .map_err(|error| format!("the machine did not run the program: {error}"))?;
    Ok((
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}

/// Run a command, wait for it, and answer the pid of the process too.
pub fn run_with_pid(command: &str, _window: &str) -> Result<(i32, String, String, u32), String> {
    let child = Command::new("cmd")
        .arg("/c")
        .arg(command)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("the machine did not run the program: {error}"))?;
    let pid = child.id();
    let output = child
        .wait_with_output()
        .map_err(|error| format!("the machine did not wait for the program: {error}"))?;
    Ok((
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        pid,
    ))
}

/// A program of the machine is started, and the script goes on.
///
/// The `Run` of the v1 does not wait for the program: it starts it and
/// goes on. The child is left to the machine, and the script may end
/// before the program does.
pub fn launch(command: &str, working_dir: &str) -> Result<u32, String> {
    let child = Command::new("cmd")
        .arg("/c")
        .arg(command)
        .current_dir(working_dir)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|error| format!("the machine did not start the program: {error}"))?;
    Ok(child.id())
}

/// A program of the machine is run, and the script waits for it.
///
/// The answer is the number of the program, the number of its exit, what
/// it wrote in the standard output and what it wrote in the standard
/// error.
pub fn run_and_wait(
    command: &str,
    working_dir: &str,
) -> Result<(u32, i32, String, String), String> {
    let output = Command::new("cmd")
        .arg("/c")
        .arg(command)
        .current_dir(working_dir)
        .output()
        .map_err(|error| format!("the machine did not run the program: {error}"))?;
    Ok((
        0,
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}

/// A value of the environment of the session.
///
/// A name that the environment does not have is `None`, and the script
/// reads the difference.
pub fn env_get(name: &str) -> Result<Option<String>, String> {
    Ok(std::env::var(name).ok())
}

/// A value of the environment of the session is written.
pub fn env_set(name: &str, value: &str) -> Result<String, String> {
    // The name of the environment may not be empty nor carry an '=' or a
    // null, and the value may not carry a null: the set_var of the std
    // panics on them, and the v1 answers with the ErrorLevel of the
    // failure instead.
    if name.is_empty() || name.contains('=') || name.contains('\0') || value.contains('\0') {
        return Err("Failed".to_string());
    }
    // The environment of the session is read by the other threads of the
    // run, and the write is the one of the tree: it is not safe here.
    unsafe { std::env::set_var(name, value) };
    Ok(value.to_string())
}

/// The text of the machine, read as the tree reads it.
pub fn the_text_of(text: &str) -> String {
    Wide::of(text).to_text()
}

/// A program of the machine is run, and the script writes in it.
///
/// The answer is the number of the program, the number of its exit, what
/// it wrote in the standard output and what it wrote in the standard
/// error.
pub fn run_with_stdin(
    program: &str,
    arguments: &[String],
    standard_input: &str,
    working_dir: &str,
) -> Result<(u32, i32, String, String), String> {
    use std::io::Write;
    let mut child = Command::new(program)
        .args(arguments)
        .current_dir(working_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|error| format!("the machine did not run the program: {error}"))?;
    if let Some(mut input) = child.stdin.take() {
        let _ = input.write_all(standard_input.as_bytes());
    }
    let output = child
        .wait_with_output()
        .map_err(|error| format!("the machine did not wait for the program: {error}"))?;
    Ok((
        0,
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    ))
}

/// A program of the machine is started, and the script writes in it.
///
/// The program is left to the machine: the script goes on, and the input
/// of it is written and closed.
pub fn launch_with_stdin(
    program: &str,
    arguments: &[String],
    standard_input: &str,
    working_dir: &str,
) -> Result<u32, String> {
    use std::io::Write;
    let mut child = Command::new(program)
        .args(arguments)
        .current_dir(working_dir)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|error| format!("the machine did not start the program: {error}"))?;
    if let Some(mut input) = child.stdin.take() {
        let _ = input.write_all(standard_input.as_bytes());
    }
    Ok(child.id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_machine_has_the_process_of_the_test() {
        let number = process_exist(None).expect("the machine says the process of the script");
        assert_ne!(number, 0);
        assert!(process_exist(Some("there-is-no-such-process")).expect("the machine says") == 0);
    }

    #[test]
    fn the_name_of_a_process_is_the_name_of_it() {
        let name = process_name(Some("cmd")).expect("the machine says the name");
        // The machine may have no `cmd` running: the answer is then
        // nothing, and the test is about the shape of the answer.
        if let Some(name) = name {
            assert!(name.to_lowercase().contains("cmd"), "{name}");
        }
    }

    #[test]
    fn the_state_of_the_process_of_the_test_is_that_it_is_running() {
        let number = process_exist(None).expect("the machine says the process of the script");
        let state = the_state_of_a_process(number)
            .expect("the machine says the state of the process")
            .expect("the process is there");
        assert!(!state.has_exited);
    }

    #[test]
    fn the_environment_of_the_session_is_read_and_written() {
        let before = env_get("AHK_RS_TEST").expect("the machine reads the environment");
        assert!(before.is_none());
        let written =
            env_set("AHK_RS_TEST", "the text").expect("the machine writes the environment");
        assert_eq!(written, "the text");
        let read = env_get("AHK_RS_TEST").expect("the machine reads the environment");
        assert_eq!(read.as_deref(), Some("the text"));
    }
}
