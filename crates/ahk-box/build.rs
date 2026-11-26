//! The manifest of the tests of the desktop.
//!
//! The machine has two sets of the common controls -- the old ones and
//! the ones of Windows Vista -- and a program takes the old ones unless
//! its manifest says that it wants the new ones. The tree view of the old
//! ones speaks only the old language: its messages are the ones of the
//! narrow characters, and the texts of a script are wide.
//!
//! The tests of the desktop make a tree view of the machine, and the tree
//! of a script is a tree view of the machine: the manifest goes to them
//! as well.

fn main() {
    // The flags are the ones of the linker of Microsoft, and the linker
    // of the other hosts does not know them. The target of the GNU
    // (mingw) is a Windows one, and its linker is the one of GNU: it
    // refuses the flags, and the link fails with "cannot find
    // /MANIFEST:EMBED". The manifest of the tree is a thing of the
    // MSVC target only.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows")
        || std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc")
    {
        return;
    }
    // The flags go to the tests of the crate and to nothing else: the
    // path of the manifest is the one of the folder that the build of the
    // crate runs in.
    println!("cargo::rustc-link-arg-tests=/MANIFEST:EMBED");
    println!("cargo::rustc-link-arg-tests=/MANIFESTINPUT:ahk.manifest");
    println!("cargo::rerun-if-changed=../../ahk.manifest");
}
