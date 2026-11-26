// SPDX-License-Identifier: Apache-2.0
//! The includes of a script: the files it asks for, and the refusals of them.
//!
//! The rules are the ones of the official binaries, and each test says where
//! its rule comes from:
//!
//! * Measured on the binary of the v2.0.32: an `#Include` that is inside a
//!   comment is not an include, and the script that has one loads.
//! * Measured on the binary of the v2.0.32: a library that is not there is
//!   refused with "Script library not found." and the line of the directive.
//! * Measured on the binary of the v1.1.37.02: the words of the v1 are
//!   "Function library not found.".
//! * Measured on the binary of the v2.0.32: a file that cannot be opened is
//!   refused with `#Include file "<the path as it was written>" cannot be
//!   opened.`, and the line of the directive.

use std::path::{Path, PathBuf};

use ahk::dialect::{self, Dialect};
use ahk_cli::includes::resolve_includes;

/// The refusal of the includes of a source, and the line it is in.
fn the_refusal_of(
    source: &str,
    dialect: &'static dyn Dialect,
) -> Option<(String, u32)> {
    match resolve_includes(source, Path::new("."), None, &mut Vec::new(), dialect) {
        Ok(_) => None,
        Err(refusal) => Some((refusal.message, refusal.line)),
    }
}

/// A directory of the test, made under the temporary directory.
fn a_test_directory(name: &str) -> PathBuf {
    let directory = std::env::temp_dir().join(format!("ahk-includes-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("the test directory is made");
    directory
}

/// An `#Include` that is inside a block comment is not an include.
///
/// Measured on the binary of the v2.0.32: the script that has an
/// `#Include <ActiveScript>` inside a `/* ... */` block loads, and the
/// directive of the block is of no account.
#[test]
fn an_include_inside_a_block_comment_is_not_an_include() {
    let source = "/*\n#Include <adash>\n*/\nFileAppend(\"the comment is not an include\", \"*\")\n";
    assert!(
        the_refusal_of(source, dialect::v2()).is_none(),
        "the include of the comment is not resolved"
    );
}

/// An `#Include` that is inside a comment of a line is not an include.
///
/// Measured on the binary of the v2.0.32: the `;` of a comment begins it,
/// and the directive that follows it is of no account.
#[test]
fn an_include_inside_a_line_comment_is_not_an_include() {
    let source = "; #Include <adash>\nFileAppend(\"the comment is not an include\", \"*\")\n";
    assert!(
        the_refusal_of(source, dialect::v2()).is_none(),
        "the include of the comment is not resolved"
    );
}

/// An `#Include` with no parameter is refused with the words of the official.
///
/// Measured on the binary of the v2.0.32: the directive with no parameter is
/// refused with "Parameter #1 required" and the line of the directive after
/// it (`ERR_PARAM1_REQUIRED`).
#[test]
fn an_include_with_no_parameter_is_refused_with_the_words_of_the_official() {
    let Some((message, line)) = the_refusal_of("#Include\n", dialect::v2()) else {
        panic!("the directive with no parameter is refused");
    };
    assert_eq!(message, "Parameter #1 required\n     Specifically: #Include");
    assert_eq!(line, 1);
}

/// An `#Include` whose parameter is empty is refused with the words of the
/// official.
///
/// Measured on the binary of the v2.0.32: the directive whose parameter is
/// empty is refused with "Parameter #1 invalid." (`ERR_PARAM1_INVALID`).
#[test]
fn an_include_with_an_empty_parameter_is_refused_with_the_words_of_the_official() {
    let Some((message, line)) = the_refusal_of("#Include \"\"\n", dialect::v2()) else {
        panic!("the directive with an empty parameter is refused");
    };
    assert_eq!(message, "Parameter #1 invalid.");
    assert_eq!(line, 1);
}

/// A library that is not there is refused with the words of the official.
///
/// Measured on the binary of the v2.0.32: the script that asks for a
/// library that is not there is refused with "Script library not found."
/// and the line of the directive after it.
#[test]
fn a_library_that_is_not_there_is_refused_with_the_words_of_the_official() {
    let source = "FileAppend(\"x\", \"*\")\n\n#Include <adash>\n";
    let Some((message, line)) = the_refusal_of(source, dialect::v2()) else {
        panic!("the library that is not there is refused");
    };
    assert_eq!(
        message,
        "Script library not found.\n     Specifically: #Include <adash>"
    );
    assert_eq!(line, 3, "the line of the directive is the line of the refusal");
}

/// The words of the refusal of a library are the ones of the dialect.
///
/// Measured on the binary of the v1.1.37.02: the v1 refuses a library that
/// is not there with "Function library not found." and not with the words
/// of the v2.
#[test]
fn the_v1_refuses_a_library_with_the_words_of_the_v1() {
    let source = "#Include <adash>\n";
    let Some((message, line)) = the_refusal_of(source, dialect::v1()) else {
        panic!("the library that is not there is refused");
    };
    assert_eq!(
        message,
        "Function library not found.\n     Specifically: #Include <adash>"
    );
    assert_eq!(line, 1);
}

/// The `Specifically` of the refusal names the directive and its argument.
///
/// Measured on the binary of the v2.0.32: the comment that follows the
/// directive is not a part of the line that the refusal names.
#[test]
fn the_refusal_names_the_directive_and_not_the_comment_that_follows_it() {
    let source = "#Include <adash> ; the library of the script\n";
    let Some((message, _line)) = the_refusal_of(source, dialect::v2()) else {
        panic!("the library that is not there is refused");
    };
    assert_eq!(
        message,
        "Script library not found.\n     Specifically: #Include <adash>"
    );
}

/// A file that is not there is refused with the words of the official.
///
/// Measured on the binary of the v2.0.32: the file that cannot be opened is
/// refused with the path as the script wrote it, and not with the path of
/// the disk.
#[test]
fn a_file_that_is_not_there_is_refused_with_the_words_of_the_official() {
    let source = "FileAppend(\"x\", \"*\")\n#Include \"nope.ahk\"\n";
    let Some((message, line)) = the_refusal_of(source, dialect::v2()) else {
        panic!("the file that is not there is refused");
    };
    assert_eq!(message, "#Include file \"nope.ahk\" cannot be opened.");
    assert_eq!(line, 2, "the line of the directive is the line of the refusal");
}

/// An `#Include` that names a directory is not a refusal.
///
/// Measured on the binary of the v2.0.32: the directory becomes the working
/// directory of the rest of the load, and the line itself is of no account.
#[test]
fn an_include_that_names_a_directory_is_not_a_refusal() {
    let directory = a_test_directory("include-that-names-a-directory");
    let source = format!("#Include \"{}\"\n", directory.display());
    let resolved = resolve_includes(
        &source,
        &directory,
        None,
        &mut Vec::new(),
        dialect::v2(),
    )
    .expect("the directory is not a refusal");
    assert!(
        !resolved.contains("cannot be opened"),
        "the directory is not a file that cannot be opened: {resolved}"
    );
    let _ = std::fs::remove_dir_all(&directory);
}

/// The file of a library that is there is put in the script.
#[test]
fn a_library_that_is_there_is_put_in_the_script() {
    let directory = a_test_directory("library-that-is-there");
    let library = directory.join("Lib");
    std::fs::create_dir_all(&library).expect("the Lib folder is made");
    std::fs::write(library.join("adash.ahk"), "FileAppend(\"the library\", \"*\")\n")
        .expect("the library is written");
    let source = "#Include <adash>\n";
    let resolved = resolve_includes(
        source,
        &directory,
        None,
        &mut Vec::new(),
        dialect::v2(),
    )
    .expect("the library is found");
    assert!(
        resolved.contains("the library"),
        "the library is in the script: {resolved}"
    );
    let _ = std::fs::remove_dir_all(&directory);
}

/// The file of an include that is there is put in the script.
#[test]
fn a_file_that_is_there_is_put_in_the_script() {
    let directory = a_test_directory("file-that-is-there");
    let included = directory.join("included.ahk");
    std::fs::write(&included, "FileAppend(\"the included\", \"*\")\n")
        .expect("the included file is written");
    let source = format!("#Include \"{}\"\n", included.display());
    let resolved = resolve_includes(
        &source,
        &directory,
        None,
        &mut Vec::new(),
        dialect::v2(),
    )
    .expect("the file is found");
    assert!(
        resolved.contains("the included"),
        "the file is in the script: {resolved}"
    );
    let _ = std::fs::remove_dir_all(&directory);
}
