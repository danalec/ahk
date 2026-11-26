//! The desktop of a worker on Windows, and the proof that the worker runs
//! on a thread of its own.

/// The factory of the desktop of a worker on Windows.
pub fn a_windows_desktop() -> Box<dyn ahk::desktop::Desktop> {
    Box::new(super::desktop::WindowsDesktop::new())
}

#[cfg(test)]
mod tests {
    use ahk::desktop::ThreadValue;

    /// The worker runs on a thread that is not the one of the test.
    #[test]
    fn a_worker_runs_on_another_thread() {
        let source = r#"WorkerFunc() {
 return DllCall("Kernel32\GetCurrentThreadId", "uint")
}
"#;
        let id = crate::workers::create(
            super::a_windows_desktop,
            source,
            "v2",
            "WorkerFunc",
            vec![],
            vec![],
            None,
        )
        .unwrap();
        let start = std::time::Instant::now();
        let result = loop {
            let state = crate::workers::state(id).unwrap();
            if state.terminated {
                break state.result;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        };
        let this_thread = unsafe { windows_sys::Win32::System::Threading::GetCurrentThreadId() };
        match result {
            Some(ThreadValue::Int(worker_thread)) => {
                assert_ne!(worker_thread as u32, this_thread);
            }
            other => panic!("the thread id of the worker: {other:?}"),
        }
    }
}
