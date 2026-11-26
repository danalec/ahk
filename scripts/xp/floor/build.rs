//! The link of the probe of the floor.
//!
//! The classes of COM live in the `ole32` of the machine, and the
//! target of the probe does not link it by itself: the probe asks the
//! machine about the classes, and the ask needs the library.

fn main() {
    println!("cargo::rustc-link-lib=ole32");
}
