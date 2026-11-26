//! The Windows desktop of the runtime.
//!
//! The `ahk-box` crate is the desktop of the lab box -- a Linux machine
//! with uinput, Wayland and `grim`. This module is the other one: the
//! desktop of a Windows machine, with the keyboard of `user32`, the
//! windows of `user32`, the registry of `advapi32` and the processes of
//! `kernel32`.
//!
//! The runtime does no I/O of its own: a desktop answers the questions
//! of the `Desktop` trait of the `ahk` crate, and every question that
//! the machine cannot answer fails with the reason, and never answers
//! `false`.
//!
//! The module is behind the `windows` configuration: on a host that is
//! not Windows it is not here, and the crate of the desktops is the
//! desktop of the lab box alone.

pub mod audio;
pub mod clipboard;
pub mod com;
pub mod controls;
pub mod desktop;
pub mod dialogs;
pub mod events;
pub mod files;
pub mod gui;
pub mod hotkeys;
pub mod imagelist;
pub mod keyboard;
pub mod machine;
pub mod listview;
pub mod mixer;
pub mod menu;
pub mod messages;
pub mod network;
pub mod picker;
pub mod pictures;
pub mod pointer;
pub mod processes;
pub mod screen;
pub mod sound;
pub mod state;
pub mod styles;
pub mod topology;
pub mod sys;
pub mod tooltip;
pub mod toplevel;
pub mod treeview;
pub mod workers;
pub mod trash;

pub use desktop::WindowsDesktop;
