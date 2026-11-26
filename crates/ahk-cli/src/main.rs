//! The runner: a script on stdin, a report on stdout.
//!
//! This is the binary that runs on the lab box. The MCP hands it a script
//! over stdin -- in memory, never a file -- and reads the report back.
//!
//! ```sh
//! echo 'Msg "hello"' | ahk
//! ```
//!
//! A script may come from a file instead: the path of the file is the one
//! argument that is not a flag, and the file is read instead of the
//! standard input.
//!
//! ```sh
//! ahk script.ahk
//! ```
//!
//! The runtime then knows the file of the script, and the `Edit` and the
//! `Reload` of v2.0.31 have a file to work with. The includes of the
//! script are resolved against the directory of the file, and the path of
//! the file is a full path, so that `A_ScriptFullPath` is one.
//!
//! The exit code is 0 when the script finished, 1 when it failed, and 2
//! when it did not parse; the report is on stdout either way.
//!
//! The `--surface` flag is the other way round: it prints the table of
//! the names of v2 with the state of each one, and it does not read a
//! script.

#![cfg_attr(windows, windows_subsystem = "windows")]

use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Duration;

use ahk_cli::{includes, json, report};


use ahk::dialect::{self, Dialect};
use ahk::parse_with;
use ahk::runtime::Runtime;
use ahk::surface;
#[cfg(not(windows))]
use ahk_box::LabDesktop;

/// Where the runtime says it is running.
///
/// A script that types forever has to be stoppable from outside, and the
/// only thing that can stop it is the process. The pid file is how the
/// `ahk_stop` tool finds it -- and how a human finds it, with `cat`.
///
/// `XDG_CACHE_HOME` is the first choice, then the cache directory of the
/// home directory. A host with neither -- a Windows one that nobody
/// configured -- gets the temporary directory, and never the working
/// directory: the file has to be where the stop looks for it, and the
/// working directory changes from one call to the next.
fn pid_file() -> PathBuf {
    let base = match std::env::var_os("XDG_CACHE_HOME") {
        Some(base) => PathBuf::from(base),
        None => match std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            Some(home) => PathBuf::from(home).join(".cache"),
            None => std::env::temp_dir(),
        },
    };
    base.join("ahk").join("run.pid")
}

fn write_pid_file() {
    let path = pid_file();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, format!("{}\n", std::process::id()));
}

fn clear_pid_file() {
    let _ = std::fs::remove_file(pid_file());
}

/// How much stack the run of a script gets.
///
/// The frames of the tree are heavier than the ones of v2, and a script
/// that recurses needs a stack that the recursion rail fits in.
const STACK_BYTES: usize = 128 * 1024 * 1024;
/// How much of that stack the run may use.
const STACK_BUDGET: usize = 64 * 1024 * 1024;

fn main() -> std::process::ExitCode {
    // A GUI-subsystem binary has no console of its own: when a terminal
    // launched it, attach to that console so the report is printed; when
    // the shell launched it (double-click / drag-drop), stay console-less.
    #[cfg(windows)]
    let has_console = the_parent_console();
    #[cfg(not(windows))]
    let has_console = true;
    // The help is answered before anything else: a host that asks for it
    // wants the usage of the runner, and not a run.
    if std::env::args()
        .skip(1)
        .any(|argument| argument == "--help" || argument == "-h")
    {
        print!("{}", the_help());
        return std::process::ExitCode::from(0);
    }
    // The dialect that the caller asked for, if it asked for one. A script
    // may name the dialect it needs, and the choice is made after the script
    // is read; the surface is answered without a script, and the v2 is the
    // dialect of the tree.
    let asked = the_dialect();
    let dialect = asked.unwrap_or_else(dialect::v2);
    // The flag is answered before the script is read: the command is
    // about the tree, and it must not wait for a script that is not
    // coming.
    if std::env::args()
        .skip(1)
        .any(|argument| argument == "--surface")
    {
        // The behaviour is the call of every name that has a probe: the
        // state says what a name is, and the probe says whether it works.
        let behaviour = std::env::args()
            .skip(1)
            .any(|argument| argument == "--behavior" || argument == "--behaviour");
        // The JSON is for the CI, and the text is for a human.
        if std::env::args()
            .skip(1)
            .any(|argument| argument == "--json")
        {
            print_surface_json(dialect, behaviour);
        } else {
            print_surface(dialect, behaviour);
        }
        return std::process::ExitCode::from(0);
    }

    // The MCP server: a client debugs AHK development by reading the errors
    // of a script (check / run / surface) over JSON-RPC on stdio.
    if std::env::args().skip(1).any(|argument| argument == "--mcp") {
        ahk_cli::mcp::serve();
        return std::process::ExitCode::from(0);
    }

    // The file association: `.ahk` opens with this runner, the way the
    // original AutoHotkey64.exe is the handler of the scripts. The open
    // verb runs the script resident (--wait) so hotkeys and GUI stay alive,
    // like the original. HKCU overrides the machine association (scoop).
    if std::env::args()
        .skip(1)
        .any(|argument| argument == "--associate" || argument == "--unassociate")
    {
        let register = std::env::args().skip(1).any(|a| a == "--associate");
        return the_association(register);
    }
    let (script_raw, script_args) = the_script_path_and_args();
    let script = script_raw.map(|path| the_full_path(&path));
    let is_file = script.is_some();

    let source = match script.as_deref() {
        Some(path) => match includes::read_script_file(path) {
            Ok(source) => source,
            Err(error) => {
                // A file that cannot be read is a failure of the load,
                // and it is reported as one.
                let error = format!("the script \"{path:?}\" cannot be read: {error}");
                let report = report::the_failure_of_the_load(&error, 0, 0);
                println!("{report}");
                return std::process::ExitCode::from(2);
            }
        },
        None => {
            let mut source = String::new();
            if let Err(error) = std::io::stdin().read_to_string(&mut source) {
                eprintln!("cannot read the script: {error}");
                return std::process::ExitCode::from(2);
            }
            source
        }
    };
    // A script run from a file lives until it ends or is stopped, the way
    // the original AutoHotkey runs it: no budget of time unless one is asked
    // for. A script on stdin keeps the bounded default of the runner.
    let timeout = std::env::args()
        .skip(1)
        .find_map(|argument| {
            argument
                .strip_prefix("--timeout=")
                .and_then(|value| value.parse::<f64>().ok())
        })
        .unwrap_or(if is_file { 0.0 } else { 60.0 });
    // A espera e' o pedido do runner: o run fica vivo depois que as
    // declaracoes do script acabam, como um script que pede para viver.
    let wait = std::env::args()
        .skip(1)
        .any(|argument| argument == "--wait");

    // The includes are resolved here and not in the language: the
    // language crate does no I/O, and the runner is the one that has a
    // disk to read from. The base is the directory of the file of the
    // script, and the working directory when the script has no file.
    let base = script
        .as_deref()
        .and_then(Path::parent)
        .unwrap_or(Path::new("."));
    // The dialect that the resolution of the includes speaks: the one the
    // caller asked for, or the one the script names in its `#Requires`, or
    // the v2 of the tree. The dialect of the parse is the one of the
    // source with the includes in it, and it is asked for below.
    let dialect_of_the_includes = asked
        .or_else(|| dialect::the_dialect_of_the_source(&source))
        .unwrap_or_else(dialect::v2);
    let source = match includes::resolve_includes(
        &source,
        base,
        script.as_deref(),
        &mut Vec::new(),
        dialect_of_the_includes,
    ) {
        Ok(source) => source,
        Err(refusal) => {
            // The refusal of an include is the refusal of the load, and it
            // carries the line of the directive that asked for it.
            let report = report::the_failure_of_the_load(&refusal.message, refusal.line, 0);
            println!("{report}");
            if is_file && !has_console {
                the_gui_error(&refusal.message);
            }
            return std::process::ExitCode::from(2);
        }
    };

    // The dialect of the script: the one the caller asked for, or the one the
    // script names in its `#Requires`, or the v2 of the tree. The `#Requires`
    // is still checked when the script runs, so a script that asks for a
    // version that the dialect does not meet is refused the way it was.
    let dialect = asked
        .or_else(|| dialect::the_dialect_of_the_source(&source))
        .unwrap_or_else(dialect::v2);

    // The v1 includes the library of a function the script calls but does
    // not define: the `Lib\FuncName.ahk` of the script, of the documents
    // or of the interpreter. The `BarChart` of a script of the v1 is
    // the `Lib\BarChart.ahk` that no `#Include` names.
    //
    // The tokens of the source come with it: the source is read once, and
    // the parse of the whole script reads the tokens of it instead of
    // reading the source again.
    let (source, lexed) = include_the_function_libraries(source, base, dialect);

    let parsed = match lexed {
        Some((tokens, merged_lines)) => ahk::Parser::with_dialect(&source, tokens, dialect)
            .with_merged_lines(merged_lines)
            .parse_script(),
        None => parse_with(&source, dialect),
    };
    let statements = match parsed {
        Ok(statements) => statements,
        Err(error) => {
            let report = report::the_failure_of_the_load(&error.message, error.line, error.column);
            println!("{report}");
            if is_file && !has_console { the_gui_error(&error.message); }
            return std::process::ExitCode::from(2);
        }
    };

    // From here on, the run can be stopped from outside: the pid file says
    // which process to kill.
    write_pid_file();
    let _guard = PidGuard;

    // The path of the file of the script goes down with the run: the
    // runtime knows the file, and the `Edit` and the `Reload` of
    // v2.0.31 have a file to work with.
    let script_path = script.map(|path| path.to_string_lossy().into_owned());
    // A copy of the path for the runner itself: the run of the script takes
    // the path with it, and the runner needs it again when the script asks
    // to run again.
    let the_file = script_path.clone();

    // The script runs on a thread with a stack of its own: the tree's
    // frames are heavier than the ones of v2, and the recursion rail has
    // to fit in the stack of the run. The report is written down there,
    // because a value of the runtime is not `Send` and cannot travel.
    let handle = std::thread::Builder::new()
        .stack_size(STACK_BYTES)
        .spawn(move || {
            // `AHK_DESKTOP=x11` selects the X11 backend (XTEST + XGetImage)
            // instead of the uinput one: a FreeBSD box has no /dev/uinput,
            // and a Wayland box has no X11 focus for uinput to reach. Both
            // are the same AHK language, with a different desktop.
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            let x11 =
                ahk_box::desktop_kind().eq_ignore_ascii_case("x11") && ahk_box::x11_available();
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            let mut x11_desktop = ahk_box::x11::X11Desktop::new();
            // On a Windows machine the desktop is the one of the Windows:
            // the keyboard of `user32`, the windows of `user32` and the
            // registry of `advapi32`. The lab box is a Linux desktop, and
            // it is not there.
            #[cfg(windows)]
            let mut windows_desktop = ahk_box::windows::WindowsDesktop::new();
            #[cfg(not(windows))]
            let mut lab_desktop = LabDesktop::new();
            #[cfg(not(windows))]
            lab_desktop.ensure_session_env();
            #[cfg(any(target_os = "linux", target_os = "freebsd"))]
            let desktop: &mut dyn ahk::desktop::Desktop = if x11 {
                &mut x11_desktop
            } else {
                &mut lab_desktop
            };
            #[cfg(windows)]
            let desktop: &mut dyn ahk::desktop::Desktop = &mut windows_desktop;
            #[cfg(not(any(target_os = "linux", target_os = "freebsd", windows)))]
            let desktop: &mut dyn ahk::desktop::Desktop = &mut lab_desktop;
            let mut runtime = Runtime::new(desktop, Duration::from_secs_f64(timeout))
                .with_stack_budget(STACK_BUDGET)
                .with_source(&source)
                .with_dialect(dialect);
            if wait {
                // O run e' residente: ele fica vivo depois que as
                // declaracoes do script acabam.
                runtime = runtime.with_resident(true);
            }
            if timeout <= 0.0 {
                // The `--timeout=0` says that the script is a daemon: the
                // run has no budget of time, and its output is the file
                // of the pid and the stop that reads it.
                runtime = runtime.with_no_budget(true);
            }
            // The runtime knows the file of the script when the host gave
            // one, and the `Edit` and the `Reload` of v2.0.31 have a file
            // to work with.
            if let Some(path) = script_path.as_deref() {
                runtime = runtime.with_script_path(path);
            }
            if !script_args.is_empty() {
                runtime = runtime.with_args(script_args);
            }
            let report = runtime.run_and_stay(&statements);
            #[cfg_attr(windows, allow(unused_mut))]
            let mut notes = report.notes.clone();
            #[cfg(not(windows))]
            notes.extend(lab_desktop.notes().iter().cloned());
            let payload = report::the_report(&report, &notes);
            (payload, report.ok, report.reload, report.exit_code)
        })
        .expect("the thread of the run starts");
    let (payload, ok, reload, exit_code) = match handle.join() {
        Ok(triple) => triple,
        Err(_) => {
            eprintln!("the run of the script died");
            return std::process::ExitCode::from(1);
        }
    };
    // The script asked to run again: the `Reload` of v2.0.31 starts the
    // script again in a process of its own, and this instance ends. The
    // runner is the one that owns the process, so the runner is the one
    // that starts the new instance -- with the same file and the same
    // arguments of the run.
    if reload {
        if let Some(path) = the_file.as_deref() {
            if let Err(reason) = the_script_runs_again(path) {
                eprintln!("the script did not run again: {reason}");
                return std::process::ExitCode::from(2);
            }
        }
    }
    println!("{payload}");
    if !ok && is_file && !has_console {
        let msg = the_error_of_payload(&payload).unwrap_or_else(|| "the script failed".to_string());
        the_gui_error(&msg);
    }
    // The exit code goes out through the return, and not through
    // `process::exit`: that one leaves without running the destructors, and
    // the guard that removes the pid file is a destructor.
    //
    // The code that the script asked for is the code of the process:
    // measured on the binary of v1.1.37.02, the `ExitApp, 1` leaves the
    // code one and the process ends with it, and the same is the `ExitApp(1)`
    // of the v2.0.32.
    let code = exit_code.unwrap_or(if ok { 0 } else { 1 });
    std::process::ExitCode::from(code as u8)
}

/// The script runs again, in a process of its own.
///
/// The program is the runner itself -- the script of the tree is run by it
/// -- and the file of the script is the one argument: the run goes on with
/// the same file and the same arguments.
fn the_script_runs_again(script: &str) -> Result<(), String> {
    let program = std::env::current_exe()
        .map_err(|error| format!("the machine did not say what this program is: {error}"))?;
    let mut command = std::process::Command::new(program);
    command.arg(script);
    // The flags of the run are the runner's, and the runner reads them from
    // the command line: the ones that are not the file of the script are
    // carried to the new instance.
    for argument in std::env::args().skip(1) {
        if argument.starts_with('-') {
            command.arg(argument);
        }
    }
    command
        .stdin(std::process::Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("the machine did not start the script again: {error}"))
}

/// The file of the script, when the host gave one.
///
/// The runner reads the script from the standard input, and a host that
/// runs a file of its own names the file here: the path is the one
/// argument that is not a flag. A run with no such argument keeps the
/// behaviour of the runner that reads the standard input, and the runtime
/// has no file.
/// Attach to the console of the parent (the terminal that launched us),
/// and point the standard handles at it when they are not already set
/// (a redirection keeps its own handles). A GUI-subsystem binary otherwise
/// has no console, so a double-click or a drag-drop shows no terminal.
#[cfg(windows)]
mod ffi {
    use std::ffi::c_void;
    #[link(name = "kernel32")]
    unsafe extern "system" {
        pub fn AttachConsole(id: u32) -> i32;
        pub fn GetStdHandle(h: u32) -> *mut c_void;
        pub fn SetStdHandle(h: u32, v: *mut c_void) -> i32;
        pub fn CreateFileW(
            name: *const u16,
            access: u32,
            share: u32,
            sec: *const c_void,
            disp: u32,
            flags: u32,
            tmpl: *mut c_void,
        ) -> *mut c_void;
    }
    #[link(name = "shell32")]
    unsafe extern "system" {
        pub fn SHChangeNotify(event: i32, flags: u32, a: *const c_void, b: *const c_void);
    }
    #[link(name = "user32")]
    unsafe extern "system" {
        pub fn MessageBoxW(hwnd: *mut c_void, text: *const u16, caption: *const u16, kind: u32) -> i32;
    }
}

/// Attach to the console of the parent (the terminal that launched us),
/// and point the standard handles at it when they are not already set
/// (a redirection keeps its own handles). A GUI-subsystem binary otherwise
/// has no console, so a double-click or a drag-drop shows no terminal.
#[cfg(windows)]
fn the_parent_console() -> bool {
    const ATTACH_PARENT: u32 = u32::MAX;
    const STD_IN: u32 = -10i32 as u32;
    const STD_OUT: u32 = -12i32 as u32;
    const STD_ERR: u32 = -11i32 as u32;
    unsafe {
        if ffi::AttachConsole(ATTACH_PARENT) == 0 {
            return false; // no parent console: double-click / drag-drop, stay silent
        }
        let conout = wide("CONOUT$");
        let conin = wide("CONIN$");
        let out = ffi::CreateFileW(conout.as_ptr(), 0x4000_0000, 3, std::ptr::null(), 3, 0, std::ptr::null_mut());
        let inp = ffi::CreateFileW(conin.as_ptr(), 0x8000_0000, 3, std::ptr::null(), 3, 0, std::ptr::null_mut());
        let invalid = -1isize as *mut std::ffi::c_void;
        let h = ffi::GetStdHandle(STD_OUT);
        if (h.is_null() || h == invalid) && !out.is_null() { ffi::SetStdHandle(STD_OUT, out); }
        let h = ffi::GetStdHandle(STD_ERR);
        if (h.is_null() || h == invalid) && !out.is_null() { ffi::SetStdHandle(STD_ERR, out); }
        let h = ffi::GetStdHandle(STD_IN);
        if (h.is_null() || h == invalid) && !inp.is_null() { ffi::SetStdHandle(STD_IN, inp); }
    }
    true
}

/// In file mode with no console (double-click / drag-drop), a failure is
/// shown as a message box, the way the original AutoHotkey reports a script
/// that does not load or run.
#[cfg(windows)]
fn the_gui_error(message: &str) {
    unsafe {
        ffi::MessageBoxW(std::ptr::null_mut(), wide(message).as_ptr(), wide("ahk").as_ptr(), 0x10);
    }
}

/// A desktop that is not Windows has no message box of the machine to show
/// the failure with: the report on the standard output is the whole of it.
#[cfg(not(windows))]
fn the_gui_error(_message: &str) {}

#[cfg(windows)]
fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// The `error` field of a report payload, for the message box of a failed
/// run in file mode. A naive scan of the JSON string is enough for a dialog.
fn the_error_of_payload(payload: &str) -> Option<String> {
    let at = payload.find("\"error\":\"")? + "\"error\":\"".len();
    let rest = &payload[at..];
    let mut out = String::new();
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('"') => out.push('"'),
                Some('\\') => out.push('\\'),
                Some(other) => out.push(other),
                None => break,
            },
            other => out.push(other),
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn the_association(register: bool) -> std::process::ExitCode {
    #[cfg(windows)]
    {
        let exe = std::env::current_exe()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "ahk.exe".to_string());
        let reg = |args: &[&str]| -> bool {
            std::process::Command::new("reg")
                .args(args)
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        if register {
            let open = format!("\"{}\" \"%1\"", exe);
            let mut ok = reg(&["add", "HKCU\\Software\\Classes\\.ahk", "/ve", "/d", "ahk.Script", "/f"])
                && reg(&["add", "HKCU\\Software\\Classes\\ahk.Script", "/ve", "/d", "AutoHotkey Script (ahk rust)", "/f"])
                && reg(&["add", "HKCU\\Software\\Classes\\ahk.Script\\DefaultIcon", "/ve", "/d", &format!("{},0", exe), "/f"])
                && reg(&["add", "HKCU\\Software\\Classes\\ahk.Script\\shell\\open\\command", "/ve", "/d", &open, "/f"]);
            // The shell may still pick the progid the original installer left
            // (ahk_auto_file / AutoHotkeyScript); point their open command at
            // this runner too, so whichever it chooses runs the tree.
            for progid in ["ahk_auto_file", "AutoHotkeyScript"] {
                ok &= reg(&["add", &format!("HKCU\\Software\\Classes\\{}\\shell\\open\\command", progid), "/ve", "/d", &open, "/f"]);
            }
            if ok {
                // Windows 10/11 keeps a per-user override (UserChoice /
                // UserChoiceLatest) that wins over the classes default; drop
                // it so the .ahk default (ahk.Script) is the handler.
                let ux = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileExts\\.ahk";
                reg(&["delete", &format!("{}\\UserChoice", ux), "/f"]);
                reg(&["delete", &format!("{}\\UserChoiceLatest", ux), "/f"]);
                // Tell the shell the association changed, so explorer picks
                // the new handler without a logoff.
                unsafe { ffi::SHChangeNotify(0x0800_0000, 0x1000, std::ptr::null(), std::ptr::null()); }
                println!("associated .ahk with {} (open runs resident)", exe);
                return std::process::ExitCode::from(0);
            }
            eprintln!("the association could not be written");
            return std::process::ExitCode::from(1);
        }
        let ok = reg(&["delete", "HKCU\\Software\\Classes\\.ahk", "/f"])
            & reg(&["delete", "HKCU\\Software\\Classes\\ahk.Script", "/f"]);
        println!(".ahk association removed (the machine's handler returns)");
        std::process::ExitCode::from(if ok { 0 } else { 1 })
    }
    #[cfg(not(windows))]
    {
        let _ = register;
        eprintln!("the .ahk file association is a thing of Windows");
        std::process::ExitCode::from(1)
    }
}

fn the_script_path_and_args() -> (Option<PathBuf>, Vec<String>) {
    let mut args = std::env::args().skip(1);
    let mut script = None;
    let mut script_args = Vec::new();
    for argument in args.by_ref() {
        if !argument.starts_with('-') {
            script = Some(PathBuf::from(argument));
            break;
        }
    }
    if script.is_some() {
        script_args.extend(args);
    }
    (script, script_args)
}

/// The path of the script as a full path.
///
/// An absolute path is kept as it is, and a relative one is joined with
/// the working directory: `A_ScriptFullPath` is a full path, and the
/// `Edit` and the `Reload` of v2.0.31 need one. The working directory
/// that cannot be read leaves the path as it is, and the run fails on the
/// file that is not there.
///
/// The path has the separator of the Windows, and not the one of the
/// shell that launched the runner: the host of the runner may write the
/// path with the slash, and the names of the script are the ones that
/// the v1 writes.
///
/// Measured on the binary of v1.1.37.02: the script of
/// the path "C:/Users/some/some.ahk" has the `A_ScriptFullPath` of
/// "C:\\Users\\some\\some.ahk", the `A_ScriptDir` of
/// "C:\\Users\\<user>" and the `A_ScriptName` of "some.ahk".
fn the_full_path(path: &Path) -> PathBuf {
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(directory) => directory.join(path),
            Err(_) => path.to_path_buf(),
        }
    };
    #[cfg(windows)]
    let path = PathBuf::from(path.to_string_lossy().replace('/', "\\"));
    path
}

/// The usage of the runner.
///
/// The runner has no man page, and the host that calls it is a program:
/// the usage is a few lines, and it says that the script may come from a
/// file.
fn the_help() -> String {
    let mut text = String::new();
    text.push_str("usage: ahk [flags] [script]\n");
    text.push('\n');
    text.push_str(
        "The script comes from the standard input, and the report goes to the\n\
         standard output. A path as the one argument that is not a flag names\n\
         the file of the script: the file is read instead of the standard\n\
         input, the includes are resolved against its directory, and the\n\
         runtime knows the file.\n",
    );
    text.push('\n');
    text.push_str("flags:\n");
    text.push_str("  --dialect=v1     speak the v1 dialect instead of the v2\n");
    text.push_str("                   (a script that names its dialect in #Requires\n");
    text.push_str("                   is spoken in it; the flag overrules the script)\n");
    text.push_str("  --timeout=N      stop the script after N seconds (60 by default,\n");
    text.push_str("                   and 0 means that the run has no budget of time)\n");
    text.push_str("  --wait           keep the run alive after the statements of the script end\n");
    text.push_str("  --associate      register .ahk to open with this runner (Windows)\n");
    text.push_str("  --unassociate    remove the .ahk association (Windows)\n");
    text.push_str("  --mcp            serve a Model Context Protocol server on stdio (check/run/surface/v1/diff)\n");
    text.push_str("  --surface        print the names of the dialect and the state of each\n");
    text.push_str("  --surface --json the same table, as JSON\n");
    text.push_str("  --behavior       with --surface, say whether each name works\n");
    text.push_str("  --help           print this usage\n");
    text
}

/// The dialect that the run speaks.
///
/// The v2 is the dialect of the tree, and it is the one of every run that
/// does not ask for another one. The `--dialect=v1` asks for the legacy
/// dialect of AutoHotkey, which is measured on the binary of v1.1.37.02.
fn the_dialect() -> Option<&'static dyn Dialect> {
    let asked = std::env::args().skip(1).find_map(|argument| {
        argument
            .strip_prefix("--dialect=")
            .map(|value| value.to_lowercase())
    });
    match asked.as_deref() {
        Some("v1") => Some(dialect::v1()),
        // The mechanisms of v2.1 beside the v2.0.19: the names and the tables
        // are the ones of the v2, and the mechanisms that the v2.1 adds are the
        // ones that the gate turns on.
        Some("v2.1") | Some("v2_1") => Some(dialect::v2_1()),
        // The dialect of KeySharp: the v2.1 mechanisms and the extensions of
        // KeySharp beside them. The refusal names the dialects the tree
        // speaks, and this one is the third of them.
        Some("ks") => Some(dialect::ks()),
        // The KeySharp without the mechanisms of the v2.1: the two
        // switches of the dialect are orthogonal, and this is the fourth
        // combination of them.
        Some("ks20") | Some("ks2.0") => Some(dialect::ks_20()),
        Some("v2") => Some(dialect::v2()),
        // No dialect was asked for: the script itself may name one, and the
        // caller decides after reading it.
        None => None,
        Some(other) => {
            eprintln!(
                "the dialect {other} is not one this tree speaks: the tree speaks v1, v2, v2.1, ks and ks20."
            );
            std::process::exit(2);
        }
    }
}

/// Print the surface of v2 and where every name of it stands here.
///
/// The report is a line for every name of the surface and a summary at
/// the end: the number is the point of the command, and it is what makes
/// "what is missing" a measurement and not an opinion.
fn print_surface(dialect: &'static dyn Dialect, behaviour: bool) {
    let entries = surface::entries_of(dialect);
    let mut out = String::new();
    for entry in &entries {
        out.push_str(&entry.line());
        if behaviour {
            if let Some(answer) = ahk::behavior::behaviour_of(dialect, &entry.full_name()) {
                out.push_str("\tbehaviour=");
                out.push_str(&answer);
            }
        }
        out.push('\n');
    }
    out.push_str(&surface::summary_of(&entries, dialect).text());
    out.push('\n');
    print!("{out}");
}

/// Print the surface of the dialect as JSON.
///
/// The text of the surface is for a human, and the JSON is for the CI: the
/// numbers of the summary and the state of every name, with the reason of
/// the ones that are refused or that are pending.
///
/// The plan of the v1 dialect asks for this shape: "the JSON, for the CI,
/// is a piece of F5".
fn print_surface_json(dialect: &'static dyn Dialect, behaviour: bool) {
    let entries = surface::entries_of(dialect);
    let summary = surface::summary_of(&entries, dialect);
    let mut report = json::Object::new(64 * 1024);
    report.text("dialect", summary.dialect);
    report.number("total", summary.total as i64);
    report.number("values", summary.values as i64);
    report.number("refusals", summary.refusals as i64);
    report.number("pending", summary.pending as i64);
    report.field("entries", |out| {
        out.push('[');
        for (index, entry) in entries.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            let mut row = json::Object::new(96);
            row.text("name", entry.name);
            row.text("kind", entry.kind.as_str());
            row.text(
                "state",
                match entry.state {
                    surface::State::Value => "value",
                    surface::State::Refusal(_) => "refusal",
                    surface::State::Pending(_) => "pending",
                },
            );
            if !entry.class.is_empty() {
                row.text("class", entry.class);
            }
            if let surface::State::Refusal(reason) | surface::State::Pending(reason) = entry.state {
                row.text("reason", reason);
            }
            if behaviour {
                if let Some(answer) = ahk::behavior::behaviour_of(dialect, &entry.full_name()) {
                    row.text("behavior", &answer);
                }
            }
            out.push_str(&row.finish());
        }
        out.push(']');
    });
    println!("{}", report.finish());
}

/// The libraries of the functions the script calls but does not define.
///
/// The v1 searches the `Lib` folder for the `FuncName.ahk` of a call that
/// has no function, and includes it. The runner does the same: it reads
/// the names the parse leaves undefined, includes the library of each,
/// and reads again until no new library is found.
///
/// The source comes back with the tokens of it: the source is lexed once
/// for the names, and the parse of the whole script reads the tokens
/// instead of reading the source again. The tokens are `None` when the
/// source does not lex -- the parse of the whole script is the one that
/// says so -- and when the loop ends without a library to add.
fn include_the_function_libraries(
    mut source: String,
    base: &Path,
    dialect: &'static dyn Dialect,
) -> (String, Option<(Vec<ahk::lexer::Token>, ahk::lexer::MergedLines)>) {
    let script_dir = includes::absolute_dir(base);
    let mut seen: Vec<PathBuf> = Vec::new();
    for _ in 0..64 {
        // The names the script calls but does not define. The source is
        // lexed once here, and the tokens of it are the ones of the parse
        // of the whole script.
        let (missing, lexed) = match ahk::lexer::tokenize_with_lines(&source, dialect) {
            Ok((lexed, merged_lines)) => {
                let (missing, lexed) =
                    ahk::parser::the_undefined_functions_of_the_tokens(&source, lexed, dialect);
                (missing, Some((lexed, merged_lines)))
            }
            // The source does not lex: there is no name to look for, and
            // the parse of the whole script is the one that says so.
            Err(_) => (Vec::new(), None),
        };
        let mut added = false;
        for name in missing {
            let Some(path) = includes::find_library(&name, &script_dir) else {
                continue;
            };
            let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
            if seen.contains(&canon) {
                continue;
            }
            seen.push(canon);
            let Ok(text) = includes::read_script_file(&path) else {
                continue;
            };
            let directory = path.parent().unwrap_or(Path::new(".")).to_path_buf();
            let resolved = includes::resolve_includes(
                &text,
                &directory,
                Some(&path),
                &mut Vec::new(),
                dialect,
            )
            .unwrap_or(text);
            source.push('\n');
            source.push_str(&resolved);
            added = true;
        }
        if !added {
            // The source is the one the tokens are of.
            return (source, lexed);
        }
    }
    // The loop ended with the source changed: the tokens are of the source
    // before the last library, and the parse of the whole script reads the
    // source again.
    (source, None)
}

/// Removes the pid file when the run ends, however it ends.
struct PidGuard;

impl Drop for PidGuard {
    fn drop(&mut self) {
        clear_pid_file();
    }
}
