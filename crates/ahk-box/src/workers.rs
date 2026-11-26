//! The worker threads of the script, on every desktop.
//!
//! A worker runs a function of the script on a real thread of the
//! machine. The values of the script hold references that do not cross a
//! thread, so the worker takes the source of the script and parses it
//! again on its own side, with a desktop and a runtime of its own, and
//! runs the function there. The arguments and the answer cross as numbers
//! and text, and the worker starts with a picture of the globals the
//! script had when the worker was created.
//!
//! The desktop of the worker is made by a factory the desktop gives: the
//! factory runs on the thread of the worker, so the desktop is made there
//! and does not cross a thread. The state of every worker lives in a table
//! the threads share: the worker writes it when it finishes, and the
//! script reads it to ask whether the worker is alive and what it answered.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

use ahk::desktop::{Desktop, ThreadValue, WorkerState};
use ahk::runtime::{Runtime, Value};

/// The factory of the desktop of a worker: it runs on the thread of the
/// worker and makes a desktop of its own.
pub type DesktopFactory = fn() -> Box<dyn Desktop>;

/// The workers, by the number that names them.
static WORKERS: LazyLock<Mutex<HashMap<u64, Arc<Mutex<WorkerState>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// The number of the next worker.
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// The budget of a worker: it runs until the function ends, and the
/// budget is a guard of a run that never ends.
const THE_BUDGET: Duration = Duration::from_secs(3600);

/// The number of finished workers whose state is kept: the state of the
/// older finished ones is let go, so the table does not grow without end.
const THE_KEPT_FINISHED: usize = 64;

/// Create a worker that runs a function, and answer the number of it.
pub fn create(
    factory: DesktopFactory,
    source: &str,
    dialect: &str,
    function: &str,
    args: Vec<ThreadValue>,
    globals: Vec<(String, ThreadValue)>,
    shared: Option<ahk::desktop::SharedGlobals>,
) -> Result<u64, String> {
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let state = Arc::new(Mutex::new(WorkerState {
        alive: true,
        terminated: false,
        result: None,
        exit_code: 0,
        task: function.to_string(),
        error: None,
    }));
    {
        let mut workers = WORKERS.lock().map_err(|_| "the table of the workers is busy.")?;
        // Let go of the oldest finished workers beyond the number kept:
        // the alive ones stay, and the recent finished ones keep their
        // answer for the script to read.
        let mut finished: Vec<u64> = workers
            .iter()
            .filter(|(_, state)| state.lock().map(|s| s.terminated).unwrap_or(false))
            .map(|(id, _)| *id)
            .collect();
        if finished.len() > THE_KEPT_FINISHED {
            finished.sort_unstable();
            let excess = finished.len() - THE_KEPT_FINISHED;
            for old_id in finished.into_iter().take(excess) {
                workers.remove(&old_id);
            }
        }
        workers.insert(id, state.clone());
    }
    let source = source.to_string();
    let dialect = dialect.to_string();
    let function = function.to_string();
    std::thread::Builder::new()
        .name(format!("ahk-worker-{id}"))
        .spawn(move || {
            run_the_worker(factory, &source, &dialect, &function, args, globals, shared, &state);
        })
        .map_err(|error| format!("the machine did not start the worker: {error}"))?;
    Ok(id)
}

/// The state of a worker.
pub fn state(id: u64) -> Result<WorkerState, String> {
    let workers = WORKERS.lock().map_err(|_| "the table of the workers is busy.")?;
    match workers.get(&id) {
        Some(state) => {
            let state = state.lock().map_err(|_| "the worker is busy.")?;
            Ok(state.clone())
        }
        None => Err(format!("there is no worker with the number {id}.")),
    }
}

/// The workers that are alive: the number and the task of each.
pub fn list() -> Result<Vec<(u64, String)>, String> {
    let workers = WORKERS.lock().map_err(|_| "the table of the workers is busy.")?;
    let mut alive = Vec::new();
    for (id, state) in workers.iter() {
        if let Ok(state) = state.lock() {
            if state.alive {
                alive.push((*id, state.task.clone()));
            }
        }
    }
    Ok(alive)
}

/// Run the function of the worker on its own side.
#[allow(clippy::too_many_arguments)]
fn run_the_worker(
    factory: DesktopFactory,
    source: &str,
    dialect_name: &str,
    function: &str,
    args: Vec<ThreadValue>,
    globals: Vec<(String, ThreadValue)>,
    shared: Option<ahk::desktop::SharedGlobals>,
    state: &Arc<Mutex<WorkerState>>,
) {
    let finish = |state: &Arc<Mutex<WorkerState>>,
                  result: Option<ThreadValue>,
                  code: i64,
                  error: Option<String>| {
        if let Ok(mut state) = state.lock() {
            state.alive = false;
            state.terminated = true;
            state.result = result;
            state.exit_code = code;
            state.error = error;
        }
    };
    let dialect = match dialect_name {
        "v1" => ahk::dialect::v1(),
        "ks" => ahk::dialect::ks(),
        "ks20" | "ks2.0" => ahk::dialect::ks_20(),
        "v2.1" | "v2_1" => ahk::dialect::v2_1(),
        _ => ahk::dialect::v2(),
    };
    let statements = match ahk::parse_with(source, dialect) {
        Ok(statements) => statements,
        Err(error) => {
            finish(state, None, 1, Some(error.message));
            return;
        }
    };
    let values: Vec<Value> = args.iter().map(|arg| arg.to_value()).collect();
    let the_globals: Vec<(String, Value)> = globals
        .iter()
        .map(|(name, value)| (name.clone(), value.to_value()))
        .collect();
    let mut desktop = factory();
    let mut runtime = Runtime::new(&mut *desktop, THE_BUDGET)
        .with_dialect(dialect)
        .with_source(source)
        .with_globals(the_globals);
    if let Some(shared) = shared {
        runtime = runtime.with_shared_globals(shared);
    }
    let (report, answer) = runtime.run_the_function(&statements, function, &values);
    let result = if report.ok { Some(ThreadValue::of_value(&answer)) } else { None };
    let error = if report.ok { None } else { report.error.clone() };
    finish(state, result, report.exit_code.unwrap_or(0) as i64, error);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The value of the script crosses a thread: the object is held behind
    /// the reentrant lock, so the whole value is Send. This is the point of
    /// the migration of the cell.
    #[test]
    fn the_value_of_the_script_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<ahk::runtime::Value>();
        assert_send::<ahk::runtime::Object>();
    }

    /// A worker runs a function on another thread and answers the result.
    /// The lab desktop is enough: the function is a pure computation.
    #[test]
    fn a_worker_runs_a_function_on_another_thread() {
        fn lab() -> Box<dyn Desktop> {
            Box::new(crate::desktop::LabDesktop::default())
        }
        let source = "WorkerFunc(x) {\n return x * 3\n}\n";
        let id = create(lab, source, "v2", "WorkerFunc", vec![ThreadValue::Int(7)], vec![], None)
            .unwrap();
        let start = std::time::Instant::now();
        loop {
            let state = state(id).unwrap();
            if state.terminated {
                assert!(!state.alive);
                assert_eq!(state.result, Some(ThreadValue::Int(21)));
                break;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    /// A worker reads a global the main thread writes while it runs: the
    /// sharing is live, not a picture taken at the creation.
    #[test]
    fn a_worker_shares_the_globals_live() {
        fn lab() -> Box<dyn Desktop> {
            Box::new(crate::desktop::LabDesktop::default())
        }
        let shared: ahk::desktop::SharedGlobals =
            std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new()));
        shared.lock().unwrap().insert("x".to_string(), ahk::runtime::Value::Int(1));
        let source = "WorkerFunc() {
 global x
 Sleep(80)
 return x
}
";
        let id = create(lab, source, "v2", "WorkerFunc", vec![], vec![], Some(shared.clone()))
            .unwrap();
        // The main writes while the worker sleeps: the worker sees the new value.
        shared.lock().unwrap().insert("x".to_string(), ahk::runtime::Value::Int(999));
        let start = std::time::Instant::now();
        loop {
            let state = state(id).unwrap();
            if state.terminated {
                assert_eq!(state.result, Some(ThreadValue::Int(999)));
                break;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    /// A worker writes a property of a shared object, and the main thread
    /// sees the write: the object is the same cell behind the reentrant
    /// lock, not a copy. This is the payoff of the migration of the cell.
    #[test]
    fn a_worker_shares_an_object_live() {
        fn lab() -> Box<dyn Desktop> {
            Box::new(crate::desktop::LabDesktop::default())
        }
        let mut object = ahk::runtime::Object {
            class: "object".to_string(),
            properties: std::collections::BTreeMap::new(),
        };
        object.put("value", ahk::runtime::Value::Int(1));
        let obj = ahk::runtime::Value::Object(ahk::runtime::SharedCell::new(object));
        let shared: ahk::desktop::SharedGlobals =
            std::sync::Arc::new(std::sync::Mutex::new(std::collections::BTreeMap::new()));
        shared.lock().unwrap().insert("obj".to_string(), obj.clone());
        let source = "WorkerFunc() {
 global obj
 obj.value := 42
}
";
        let id = create(lab, source, "v2", "WorkerFunc", vec![], vec![], Some(shared.clone()))
            .unwrap();
        let start = std::time::Instant::now();
        loop {
            let state = state(id).unwrap();
            if state.terminated {
                break;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let store = shared.lock().unwrap();
        match store.get("obj").unwrap() {
            ahk::runtime::Value::Object(cell) => {
                assert_eq!(cell.borrow().value_of("value"), Some(ahk::runtime::Value::Int(42)));
            }
            _ => panic!("the shared obj is not an object"),
        }
    }

    /// A worker takes an array and answers an array.
    #[test]
    fn a_worker_takes_and_answers_an_array() {
        fn lab() -> Box<dyn Desktop> {
            Box::new(crate::desktop::LabDesktop::default())
        }
        let source = "WorkerFunc(a) {
 s := 0
 for v in a
  s += v
 return [s, a.Length]
}
";
        let args = vec![ThreadValue::Array(vec![
            ThreadValue::Int(10),
            ThreadValue::Int(20),
            ThreadValue::Int(30),
        ])];
        let id = create(lab, source, "v2", "WorkerFunc", args, vec![], None).unwrap();
        let start = std::time::Instant::now();
        loop {
            let state = state(id).unwrap();
            if state.terminated {
                assert_eq!(
                    state.result,
                    Some(ThreadValue::Array(vec![
                        ThreadValue::Int(60),
                        ThreadValue::Int(3)
                    ]))
                );
                break;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    /// A worker starts with the globals the script gave it.
    #[test]
    fn a_worker_starts_with_the_globals() {
        fn lab() -> Box<dyn Desktop> {
            Box::new(crate::desktop::LabDesktop::default())
        }
        let source = "WorkerFunc() {\n global x\n return x + 1\n}\n";
        let globals = vec![("x".to_string(), ThreadValue::Int(40))];
        let id = create(lab, source, "v2", "WorkerFunc", vec![], globals, None).unwrap();
        let start = std::time::Instant::now();
        loop {
            let state = state(id).unwrap();
            if state.terminated {
                assert_eq!(state.result, Some(ThreadValue::Int(41)));
                break;
            }
            if start.elapsed() > std::time::Duration::from_secs(10) {
                panic!("the worker did not finish");
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }
}
