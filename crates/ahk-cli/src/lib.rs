//! The parts of the runner that a test reads.
//!
//! The runner is a binary, and a binary cannot be imported. The two modules
//! that a test wants to read back -- the JSON it writes and the report it
//! builds out of a run -- are therefore here, and the binary uses them: the
//! report is written by one implementation and read by the same one, and a
//! test that disagreed with the runner would be a test of the test.

pub mod json;
pub mod report;