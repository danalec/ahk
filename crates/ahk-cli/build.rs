//! The manifest of the runner.
//!
//! The machine has two sets of the common controls -- the old ones and
//! the ones of Windows Vista -- and a program takes the old ones unless
//! its manifest says that it wants the new ones. The tree view of the old
//! ones speaks only the old language: its messages are the ones of the
//! narrow characters, and the texts of a script are wide.
//!
//! The manifest is the way of the machine to take the new ones, and the
//! tree of the script needs them: the tree view of a script is a tree
//! view of the machine.
//!
//! The icon of the runner is the one of the tree (`resources/icon.ico`).
//! The link of Microsoft does not take an icon: it takes a resource, and
//! the resource of the icon is what the `rc.exe` of the SDK compiles
//! from a script of resources. The machine that does not have the
//! `rc.exe` stays without an icon, and the binary is the same without
//! it.

use std::path::{Path, PathBuf};
use std::process::Command;

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
    // The flags go to the binaries of the runner and to nothing else.
    // The manifest lives at the root of the tree, beside the crate; a
    // build of the published crate (crates.io) has no tree around it, so
    // the manifest is not there and the link goes without it -- the
    // binary is the same, only without the machine's manifest.
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../ahk.manifest");
    if manifest.is_file() {
        println!("cargo::rustc-link-arg-bins=/MANIFEST:EMBED");
        println!(
            "cargo::rustc-link-arg-bins=/MANIFESTINPUT:{}",
            manifest.display()
        );
        println!("cargo::rerun-if-changed=../../ahk.manifest");
    }
    // O icone: o recurso que o `rc.exe` compila vai para o link como
    // uma entrada do link, e o binario de fora leva o icone da arvore.
    if let Some(resource) = the_resource_of_the_icon() {
        println!("cargo::rustc-link-arg-bins={}", resource.display());
    }
}

/// The resource of the icon of the tree, when the machine has the compiler
/// of resources.
///
/// The icon is `resources/icon.ico`, and the resource is a script of one
/// line that the `rc.exe` compila. The path of the icon in the script is
/// whole, and with the bars in front, because the `rc.exe` reads the
/// path as it is.
fn the_resource_of_the_icon() -> Option<PathBuf> {
    let icon = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../resources/icon.ico");
    if !icon.is_file() {
        return None;
    }
    println!("cargo::rerun-if-changed=../../resources/icon.ico");
    let compiler = the_compiler_of_resources()?;
    let out = PathBuf::from(std::env::var("OUT_DIR").ok()?);
    let script = out.join("the_icon.rc");
    let path = icon.to_string_lossy().replace('\\', "/");
    std::fs::write(&script, format!("IDI_ICON1 ICON \"{path}\"\n")).ok()?;
    let resource = out.join("the_icon.res");
    let status = Command::new(&compiler)
        .arg("/fo")
        .arg(&resource)
        .arg(&script)
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    Some(resource)
}

/// The compiler of resources of the machine: the `rc.exe` of the Windows
/// SDK.
///
/// The path of the prompt of the Visual Studio is the first one that is
/// tried, and the SDK is the second: the versions of the SDK are
/// folders, and the newest is the last one. The `rc.exe` is of the
/// target: the resource of a binary of thirty-two bits is another one.
fn the_compiler_of_resources() -> Option<PathBuf> {
    if let Ok(folder) = std::env::var("WindowsSdkVerBinPath") {
        let compiler = Path::new(&folder).join("rc.exe");
        if compiler.is_file() {
            return Some(compiler);
        }
    }
    let arch = match std::env::var("CARGO_CFG_TARGET_ARCH").ok()?.as_str() {
        "x86_64" => "x64",
        "x86" => "x86",
        "aarch64" => "arm64",
        _ => return None,
    };
    for folder in ["ProgramFiles(x86)", "ProgramFiles"] {
        let Ok(root) = std::env::var(folder) else {
            continue;
        };
        let bin = PathBuf::from(root).join("Windows Kits").join("10").join("bin");
        let Ok(versions) = std::fs::read_dir(bin) else {
            continue;
        };
        let mut versions: Vec<PathBuf> = versions
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| path.is_dir())
            .collect();
        versions.sort();
        for version in versions.iter().rev() {
            let compiler = version.join(arch).join("rc.exe");
            if compiler.is_file() {
                return Some(compiler);
            }
        }
    }
    None
}
