# ahk-box

The machine as a script target: the implementation of the `Desktop` trait of
the [`ahk`](https://crates.io/crates/ahk) language for a Linux box, and the
answers it gives on the other platforms.

What it carries:

- **A keyboard of our own.** `/dev/uinput` opened directly, with the 112 key
  codes the physical keyboard declares, so that a key the compositor dropped is
  a bug in this code and not a mystery in the device beside it.
- **An absolute pointer**, on a device of its own, because libinput classifies
  a pointer and a keyboard differently and one device cannot be both.
- **The windows of the compositor**, spoken through
  `ext_foreign_toplevel_list_v1` and the COSMIC protocol that extends it.
- **The screen**: a capture through `grim`, and the PNG reader and writer that
  make a pixel a colour -- the format is the crate's own, so the tree needs
  nothing from outside itself.
- **The rest of the machine**: the files, the clipboard, the dialogs, the sound,
  the pictures, the network, and the call into a library of the system.

On a platform that is not Linux the crate still builds, and the primitives that
are Linux-only answer with the reason instead of pretending.

## Documentation

[`docs/desktop-backend.md`](https://github.com/danalec/ahk/blob/master/docs/desktop-backend.md)
is the guide: the fifteen capability traits, what a host has to answer, and the
platform matrix.

## Licence

Apache-2.0.