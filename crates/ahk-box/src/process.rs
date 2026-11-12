//! The processes of the machine, read out of `/proc`.
//!
//! The process family of v2 asks six questions about a process: whether it
//! is there, what its executable is called, where its executable is, who its
//! parent is, whether it can be closed, and how its priority is set. The
//! box is a Linux machine, and `/proc` answers every one of them.
//!
//! # What the files answer
//!
//! - `/proc/<pid>/exe` is a symbolic link to the executable of the process.
//!   The file name of the link is the name of the process, and the whole of
//!   it is the path -- `ProcessGetName` and `ProcessGetPath`.
//! - `/proc/<pid>/comm` is the name the kernel gives the process, which is
//!   the file name of its executable cut to fifteen characters. A process of
//!   the kernel has no executable at all, and this is the only name it has.
//! - `/proc/<pid>/stat` is the state of the process: the fourth field of it
//!   is the pid of the parent. The second field is the name in parentheses,
//!   and the name may hold a space and a parenthesis, so the fields after it
//!   are counted from the last `)`; this is why the reading is not a split.
//! - `/proc` itself lists the processes that are there, one directory for
//!   each pid. A scan of it is what a name is matched against.
//!
//! # The dates
//!
//! The shape of each answer -- what a missing process answers, what a wait
//! answers, how the name is matched -- was measured on the binary of v2.0.19
//! on 2026-11-09, and the reading of `/proc` was pinned on the box of the
//! tree, on Linux, on the same day:
//!
//! - `ProcessExist()` is the pid of the script itself, `ProcessExist("999999")`
//!   is zero, and a name is matched against the file name of the executable
//!   without regard to the case. A process that is not there is zero, and it
//!   is not a failure.
//! - `ProcessGetName` and `ProcessGetPath` answer the name and the path, and a
//!   process that is not there is the failure "Target process not found." of
//!   the class `TargetError`. `ProcessGetParent` of a process that is not
//!   there answers zero.
//! - `ProcessClose` answers the pid it closed, and zero when the process was
//!   not there or could not be closed: the binary answers zero for a process
//!   it could not open, too.
//! - `ProcessWait` answers the pid when the process appears and zero at the
//!   timeout; `ProcessWaitClose` answers zero when the process is gone -- a
//!   process that never existed is gone -- and the pid that is still there at
//!   the timeout. Both wait a hundred milliseconds between two checks, which
//!   is the interval of the binary.
//! - `ProcessSetPriority` reads the first letter of the priority word, and a
//!   process that is not there is the failure "Target process not found.".
//!   The Linux answer to a priority is the nice value, and the mapping below
//!   is the one of the six classes of Windows to the range of `nice`.

use std::time::{Duration, Instant};

/// The interval a wait sleeps between two checks of the process.
///
/// The binary checks every hundred milliseconds, and the value is the one of
/// it: measured on the binary of v2.0.19 on 2026-11-09, a `ProcessWait` of a
/// process that is not there returns after the timeout it was given, and it
/// checks a few times over a second.
const SLEEP_INTERVAL: Duration = Duration::from_millis(100);

/// The signal a close sends: the polite one, and the one the close of v2
/// means -- it asks the process to end, and it does not shoot it.
///
/// `SIGTERM` is 15 on every architecture the tree is built for.
const SIGTERM: i32 = 15;

/// The whole of a process that a priority is set for: `PRIO_PROCESS`, the
/// process the pid names, and not a group of them.
#[cfg(unix)]
const PRIO_PROCESS: i32 = 0;

/// The reason the questions about another process have no answer here.
///
/// The reading of the processes of the machine is the reading of `/proc`, and
/// a machine that has no `/proc` -- Windows, and the Macs -- is told so. The
/// pid of the process of the script itself is known everywhere, and that one
/// question is answered on those machines too.
const NO_PROC: &str = "the processes of this machine are read out of /proc, and this machine has none";

/// Whether the machine reads its processes out of `/proc`.
fn the_proc_is_there() -> bool {
    std::fs::metadata("/proc").is_ok()
}

/// The pid of a process, and zero when there is no such process.
///
/// `None` asks about the process of the script itself, which is what the
/// omitted parameter of the binary means.
pub fn exist(which: Option<&str>) -> Result<u32, String> {
    let Some(which) = which else {
        return Ok(std::process::id());
    };
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    Ok(the_pid_of(which).unwrap_or(0))
}

/// The name of the executable of a process.
///
/// `None` asks about the process of the script itself, and the answer is
/// `None` when the process is not there -- the caller of this says the words
/// of the dialect for it.
pub fn name(which: Option<&str>) -> Result<Option<String>, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let Some(pid) = the_pid_asked_about(which) else {
        return Ok(None);
    };
    Ok(the_name_of(pid))
}

/// The full path of the executable of a process.
///
/// `None` asks about the process of the script itself, and the answer is
/// `None` when the process is not there.
pub fn path(which: Option<&str>) -> Result<Option<String>, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let Some(pid) = the_pid_asked_about(which) else {
        return Ok(None);
    };
    the_path_of(pid)
}

/// The pid of the parent of a process, and zero when the process is not
/// there.
pub fn parent(which: Option<&str>) -> Result<u32, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let Some(pid) = the_pid_asked_about(which) else {
        return Ok(0);
    };
    Ok(the_parent_of(pid).unwrap_or(0))
}

/// Close a process: the pid that was closed, and zero when the process was
/// not there or could not be closed.
///
/// The binary answers zero when it cannot open the process, which is what a
/// process of another user is to the script: the answer of a close that is
/// not allowed is zero, and not a failure.
pub fn close(which: &str) -> Result<u32, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let Some(pid) = the_pid_of(which) else {
        return Ok(0);
    };
    match the_signal(pid, SIGTERM) {
        Ok(()) => Ok(pid),
        Err(_) => Ok(0),
    }
}

/// Wait for a process to exist: the pid of it, and zero at the timeout.
///
/// A timeout of `None` waits forever, which is what the omitted timeout of
/// the binary does.
pub fn wait(which: &str, timeout: Option<f64>) -> Result<u32, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let started = Instant::now();
    loop {
        let pid = exist(Some(which))?;
        if pid != 0 {
            return Ok(pid);
        }
        if the_wait_is_over(started, timeout) {
            return Ok(0);
        }
        std::thread::sleep(SLEEP_INTERVAL);
    }
}

/// Wait for a process that is there to close.
///
/// The answer is zero when the process is gone -- a process that never
/// existed is gone -- and the pid of the process that is still there at the
/// timeout.
pub fn wait_close(which: &str, timeout: Option<f64>) -> Result<u32, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let started = Instant::now();
    loop {
        let pid = exist(Some(which))?;
        if pid == 0 {
            return Ok(0);
        }
        if the_wait_is_over(started, timeout) {
            return Ok(pid);
        }
        std::thread::sleep(SLEEP_INTERVAL);
    }
}

/// Set the priority of a process: the pid of it, and `None` when the process
/// is not there.
///
/// The priority is the word of v2, and only the first letter of it is read,
/// the way the binary reads it.
pub fn set_priority(priority: &str, which: Option<&str>) -> Result<Option<u32>, String> {
    if !the_proc_is_there() {
        return Err(NO_PROC.to_string());
    }
    let Some(pid) = the_pid_asked_about(which) else {
        return Ok(None);
    };
    let Some(nice) = the_nice_of(priority) else {
        return Err("the priority is not one of the six of the dialect".to_string());
    };
    the_nice_of_a_process(pid, nice)?;
    Ok(Some(pid))
}

// -- the reading of /proc ---------------------------------------------------------

/// The pid the call asks about: the pid of the script itself when the call
/// names nothing.
fn the_pid_asked_about(which: Option<&str>) -> Option<u32> {
    match which {
        None => Some(std::process::id()),
        Some(text) => {
            // The binary of v2 reads an empty text as the process of the
            // script itself in the second parameter of `ProcessSetPriority`:
            // measured on the binary of v2.0.19 on 2026-11-09.
            if text.is_empty() {
                return Some(std::process::id());
            }
            the_pid_of(text)
        }
    }
}

/// The pid a pid or a name answers, and nothing when there is none.
fn the_pid_of(which: &str) -> Option<u32> {
    if let Ok(pid) = which.parse::<u32>() {
        // A text that is a number is a pid, and a pid that is not there is
        // not a name to look for: the binary reads it the same way, and
        // `ProcessExist("999999")` is zero.
        return if the_process_is_there(pid) {
            Some(pid)
        } else {
            None
        };
    }
    the_pid_of_a_name(which)
}

/// Whether a pid is a process that is there.
///
/// The directory of a process exists while the process does: the kernel makes
/// it when the process is made and takes it away when the process is gone.
fn the_process_is_there(pid: u32) -> bool {
    std::fs::metadata(format!("/proc/{pid}")).is_ok()
}

/// The pids of the processes of the machine, in the order of their numbers.
fn the_processes() -> Vec<u32> {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return Vec::new();
    };
    let mut pids: Vec<u32> = entries
        .filter_map(|entry| entry.ok()?.file_name().to_str()?.parse::<u32>().ok())
        .collect();
    pids.sort_unstable();
    pids
}

/// The first process whose name is the text, without regard to the case.
fn the_pid_of_a_name(name: &str) -> Option<u32> {
    let wanted = name.to_lowercase();
    the_processes().into_iter().find(|pid| {
        the_name_of(*pid)
            .map(|found| found.to_lowercase() == wanted)
            .unwrap_or(false)
    })
}

/// The name of the executable of a process.
///
/// The name is the file name of `/proc/<pid>/exe`. A process of the kernel
/// has no executable, and the name the kernel gives it is `/proc/<pid>/comm`.
fn the_name_of(pid: u32) -> Option<String> {
    if let Some(path) = the_path_of(pid).ok().flatten() {
        let path = std::path::Path::new(&path);
        if let Some(name) = path.file_name() {
            return Some(name.to_string_lossy().to_string());
        }
    }
    let comm = std::fs::read_to_string(format!("/proc/{pid}/comm")).ok()?;
    let comm = comm.trim_end_matches('\n');
    if comm.is_empty() {
        None
    } else {
        Some(comm.to_string())
    }
}

/// The full path of the executable of a process.
///
/// The link says " (deleted)" after the path when the file the process runs
/// is gone, and the path of v2 is the path the process was started from: the
/// suffix is cut, so that the answer is the one of the file the process runs.
///
/// A process that is there but whose link cannot be read -- a process of
/// another user, on a machine where the link is hidden -- is the failure of
/// the machine, and not a process that is not there.
fn the_path_of(pid: u32) -> Result<Option<String>, String> {
    match std::fs::read_link(format!("/proc/{pid}/exe")) {
        Ok(path) => {
            let text = path.to_string_lossy().to_string();
            let text = text.strip_suffix(" (deleted)").unwrap_or(&text).to_string();
            Ok(Some(text))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            // The process that was there a moment ago is gone: the answer is
            // the one of a process that is not there.
            Ok(None)
        }
        Err(error) => Err(the_message_of(&error)),
    }
}

/// The pid of the parent of a process.
///
/// The parent is the fourth field of `/proc/<pid>/stat`, and the fields are
/// counted from the last `)` of it: the name of the process is in parentheses
/// and it may hold a space and a parenthesis of its own.
fn the_parent_of(pid: u32) -> Option<u32> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let close = stat.rfind(')')?;
    let mut fields = stat[close + 1..].split_whitespace();
    // The state of the process, and then its parent.
    fields.next()?;
    fields.next()?.parse::<u32>().ok()
}


/// The state of a process, as the kernel of the machine sees it.
pub struct TheState {
    /// Whether the process has exited.
    pub has_exited: bool,
    /// The code it exited with, when the kernel knows it.
    ///
    /// The kernel keeps the code of the exit for the parent of the
    /// process, and a process that is not ours has no code we can read:
    /// it stays unset.
    pub exit_code: Option<i32>,
}

/// The state of the process of the pid, and whether it is there at all.
///
/// `None` is the answer for a process that is not there: the directory
/// of it is not in `/proc`, and a process that is gone is over. The
/// state of a process that is there is the letter of `/proc/<pid>/stat`
/// -- the `R` of a process that runs, the `S` of one that sleeps, and
/// the `Z` of a zombie: a process that is over and whose parent did not
/// take it yet.
pub fn the_state_of(pid: u32) -> Option<TheState> {
    if !std::path::Path::new(&format!("/proc/{pid}")).exists() {
        return None;
    }
    let Some(state) = std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|stat| {
            // The name of the process is in parentheses and it may hold
            // a space and a parenthesis of its own, so the fields are
            // counted from the last `)`.
            let close = stat.rfind(')')?;
            stat[close + 1..].split_whitespace().next().map(|state| state.to_string())
        })
    else {
        // The process is there and the kernel does not say its state:
        // the process is over only when it is not there at all.
        return Some(TheState {
            has_exited: false,
            exit_code: None,
        });
    };
    Some(TheState {
        // The `Z` of a zombie is the only state that means the process
        // is over: it waits for its parent to take it.
        has_exited: state == "Z",
        exit_code: None,
    })
}

/// The state of a process on Windows, and the moment of its exit.
///
/// The kernel of the machine knows the state of every process it can
/// open: `OpenProcess` with the right of the query opens it,
/// `GetExitCodeProcess` answers the code of the exit -- and the 259 of
/// a process that is still active is the answer of a process that runs
/// -- and `GetProcessTimes` answers the moment of the exit, in the
/// hundred-nanoseconds since the epoch of Windows.
///
/// The epoch of Windows is 1601, and the epoch of the Unix is 1970: the
/// difference is the 11644473600 of the seconds between them.
#[cfg(windows)]
pub fn the_state_of_a_process_on_windows(pid: u32) -> Option<ahk::desktop::ProcessState> {
    use std::ffi::c_void;

    unsafe extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut c_void;
        fn GetExitCodeProcess(handle: *mut c_void, code: *mut u32) -> i32;
        fn GetProcessTimes(
            handle: *mut c_void,
            creation: *mut u64,
            exit: *mut u64,
            kernel: *mut u64,
            user: *mut u64,
        ) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
    }

    /// The right of the query of a process: enough to read its state.
    const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
    /// The code of a process that is still active.
    const STILL_ACTIVE: u32 = 259;
    /// The seconds between the epoch of Windows and the epoch of the Unix.
    const THE_EPOCH_OF_WINDOWS: u64 = 11_644_473_600;

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            // The kernel does not open it: the process is not there, or
            // the rights of the session do not reach it. The first is
            // the common one, and a process that is not there is over.
            return Some(ahk::desktop::ProcessState {
                has_exited: true,
                exit_code: None,
                exit_time: None,
            });
        }
        let mut code: u32 = 0;
        let asked = GetExitCodeProcess(handle, &mut code);
        let mut creation: u64 = 0;
        let mut exit: u64 = 0;
        let mut kernel: u64 = 0;
        let mut user: u64 = 0;
        let times = GetProcessTimes(handle, &mut creation, &mut exit, &mut kernel, &mut user);
        CloseHandle(handle);
        if asked == 0 {
            // The kernel does not answer the code: the state is unknown,
            // and the tree does not guess it.
            return None;
        }
        if code == STILL_ACTIVE {
            return Some(ahk::desktop::ProcessState {
                has_exited: false,
                exit_code: None,
                exit_time: None,
            });
        }
        // The moment of the exit is the hundred-nanoseconds of Windows,
        // and the text of it is the way KeySharp writes a moment.
        let exit_time = if times != 0 && exit != 0 {
            let millis = (exit / 10_000) as i64 - (THE_EPOCH_OF_WINDOWS * 1000) as i64;
            Some(ahk::date::the_local_text_of(millis))
        } else {
            None
        };
        Some(ahk::desktop::ProcessState {
            has_exited: true,
            exit_code: Some(code as i32),
            exit_time,
        })
    }
}
/// The nice value a priority word means.
///
/// The binary reads the first letter of the word -- measured on the binary of
/// v2.0.19 on 2026-11-09: `ProcessSetPriority("Bogus")` is a call of the
/// below-normal class, and `ProcessSetPriority(3)` is an invalid parameter,
/// because the first letter of "3" is not one of the six. The mapping of the
/// six classes to the range of `nice` is the one of Windows to Linux: the
/// idle class is the highest niceness, and the realtime class the lowest.
fn the_nice_of(priority: &str) -> Option<i32> {
    let letter = priority.chars().next()?.to_ascii_uppercase();
    Some(match letter {
        'L' => 19,
        'B' => 10,
        'N' => 0,
        'A' => -5,
        'H' => -10,
        'R' => -20,
        _ => return None,
    })
}

// -- the calls of the machine -----------------------------------------------------

/// Whether a wait that started at this moment is over.
///
/// The timeout of the binary is a number of seconds, and it is turned into
/// milliseconds by the binary, so a fraction of a millisecond is no wait at
/// all and a negative timeout is over at once.
fn the_wait_is_over(started: Instant, timeout: Option<f64>) -> bool {
    match timeout {
        // The omitted timeout waits forever, which is the one of the binary.
        None => false,
        Some(seconds) => {
            let millis = seconds * 1000.0;
            millis <= 0.0 || started.elapsed().as_secs_f64() * 1000.0 >= millis
        }
    }
}

/// The message of a failure of the machine, the way the tree says it: the
/// code of the system in front of what it says.
fn the_message_of(error: &std::io::Error) -> String {
    let text = error.to_string();
    // The text of the standard library carries the code at the end, and the
    // message of v2 carries it in front: "(2) The system cannot find the
    // file specified.".
    let text = text.split(" (os error ").next().unwrap_or(&text);
    match error.raw_os_error() {
        Some(code) => format!("({code}) {text}"),
        None => text.to_string(),
    }
}

/// Send a signal to a process.
#[cfg(unix)]
fn the_signal(pid: u32, signal: i32) -> Result<(), std::io::Error> {
    // The call answers zero when the signal was sent, and minus one with the
    // reason in the error of the system when it was not.
    let answer = unsafe { kill(pid as i32, signal) };
    if answer == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Send a signal to a process.
///
/// A machine that has no such call cannot close a process, and the caller
/// says the reason of the platform: the box of the tree is Linux, and the
/// name is reached there.
#[cfg(not(unix))]
fn the_signal(_pid: u32, _signal: i32) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "this machine has no kill",
    ))
}

/// Set the nice value of a process.
#[cfg(unix)]
fn the_nice_of_a_process(pid: u32, nice: i32) -> Result<(), String> {
    let answer = unsafe { setpriority(PRIO_PROCESS, pid, nice) };
    if answer == 0 {
        Ok(())
    } else {
        Err(the_message_of(&std::io::Error::last_os_error()))
    }
}

/// Set the nice value of a process.
///
/// A machine that has no such call cannot set the priority of a process, and
/// the answer is the failure of the platform.
#[cfg(not(unix))]
fn the_nice_of_a_process(_pid: u32, _nice: i32) -> Result<(), String> {
    Err("this machine has no setpriority".to_string())
}

#[cfg(unix)]
unsafe extern "C" {
    /// The declaration of `kill` the C library gives.
    #[link_name = "kill"]
    fn kill(pid: i32, signal: i32) -> i32;

    /// The declaration of `setpriority` the C library gives.
    ///
    /// The first argument is the kind of the thing the second one names:
    /// `PRIO_PROCESS` for a process. The third one is the nice value.
    #[link_name = "setpriority"]
    fn setpriority(which: i32, who: u32, priority: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pid that is not there.
    ///
    /// The largest pid Linux has is 4194304, and the number here is above it:
    /// the kernel never makes it, so the answer of the four questions about
    /// it is the answer of a process that is not there.
    #[cfg(target_os = "linux")]
    const NOBODY: u32 = 4_194_305;

    /// The process of the script is a process that is there.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_process_of_the_script_is_there() {
        let pid = std::process::id();
        assert_eq!(exist(None), Ok(pid));
        assert_eq!(exist(Some(&pid.to_string())), Ok(pid));
        assert!(parent(None).unwrap() > 0, "the test has a parent");
    }

    /// A pid that is not there answers zero, and it is not a failure.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_process_that_is_not_there_answers_zero() {
        assert_eq!(exist(Some(&NOBODY.to_string())), Ok(0));
        assert_eq!(exist(Some("a-name-no-process-has")), Ok(0));
        assert_eq!(parent(Some(&NOBODY.to_string())), Ok(0));
        assert_eq!(close(&NOBODY.to_string()), Ok(0));
        assert_eq!(name(Some(&NOBODY.to_string())), Ok(None));
        assert_eq!(path(Some(&NOBODY.to_string())), Ok(None));
        assert_eq!(set_priority("Normal", Some(&NOBODY.to_string())), Ok(None));
    }

    /// The name and the path of the process of the tests.
    #[cfg(target_os = "linux")]
    #[test]
    fn the_name_and_the_path_of_the_process_of_the_tests() {
        let exe = std::env::current_exe().expect("the test binary has a path");
        let name = name(None).unwrap().expect("the test process has a name");
        let expected = exe
            .file_name()
            .map(|file_name| file_name.to_string_lossy().to_string());
        assert_eq!(Some(name.clone()), expected);
        let path = path(None).unwrap().expect("the test process has a path");
        assert!(path.ends_with(&name), "the path {path} ends with the name {name}");
    }

    /// A wait of a process that is not there answers zero, and the wait of a
    /// process that never existed is closed at once.
    #[cfg(target_os = "linux")]
    #[test]
    fn a_wait_for_a_process_that_is_not_there_answers_zero() {
        let started = Instant::now();
        assert_eq!(wait(&NOBODY.to_string(), Some(0.2)), Ok(0));
        assert!(started.elapsed() >= Duration::from_millis(200), "the wait waited");
        assert_eq!(wait_close(&NOBODY.to_string(), Some(0.2)), Ok(0));
    }

    /// The six words of the priority, and the letters of them.
    #[test]
    fn the_priority_words_are_the_six_of_the_dialect() {
        assert_eq!(the_nice_of("Low"), Some(19));
        assert_eq!(the_nice_of("BelowNormal"), Some(10));
        assert_eq!(the_nice_of("Normal"), Some(0));
        assert_eq!(the_nice_of("AboveNormal"), Some(-5));
        assert_eq!(the_nice_of("High"), Some(-10));
        assert_eq!(the_nice_of("Realtime"), Some(-20));
        // The binary reads only the first letter of the word.
        assert_eq!(the_nice_of("Bogus"), Some(10));
        assert_eq!(the_nice_of("normal"), Some(0));
        // A number is not one of the words of the dialect.
        assert_eq!(the_nice_of("3"), None);
        assert_eq!(the_nice_of(""), None);
    }

    /// The message of the machine carries the code in front of the text.
    ///
    /// The text is the one the standard library gives the number, and the
    /// shape is the one of the binary: the code of the system in front of
    /// what it says, "(2) The system cannot find the file specified.".
    #[test]
    fn the_message_of_the_machine_is_the_code_and_the_text() {
        let error = std::io::Error::from_raw_os_error(2);
        let message = the_message_of(&error);
        assert!(message.starts_with("(2) "), "{message}");
        assert!(!message.contains("os error"), "{message}");
    }

    /// A wait that is over at once is over, and one with no timeout is not.
    #[test]
    fn the_end_of_a_wait_is_the_end_of_its_timeout() {
        let started = Instant::now();
        assert!(!the_wait_is_over(started, None));
        assert!(the_wait_is_over(started, Some(0.0)));
        assert!(the_wait_is_over(started, Some(-1.0)));
        assert!(!the_wait_is_over(started, Some(10.0)));
    }
}