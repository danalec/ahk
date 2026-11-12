#!/usr/bin/python3
"""Carry the capabilities that are not about X11 into the X11 backend.

The X11 backend of the box answers the windows, the keys and the screen
of an X11 session. Everything else -- the files, the dialogs, the
machine, the pictures, the network -- was refused there, while the
desktop of the box answers all of it.

WHY the delegation: those capabilities are not about X11 at all. The
files are the file system of the box, the dialogs are `zenity`, the
machine is the machine, and the pictures are read with the `png` crate.
The X11 backend therefore keeps a desktop of the box inside itself and
asks it, and the state that matters -- the files that are open and the
pictures that were loaded -- is kept there, because a desktop made for
each call would lose it between the calls.

The pointer is the exception that the X11 backend answers itself:
`XQueryPointer` is a symbol of Xlib, and the place of the pointer is
something only the X server knows.

The script is idempotent.

    python audit/port-x11-desktop.py
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent

X11 = ROOT / "crates" / "ahk-box" / "src" / "x11.rs"
FILES = ROOT / "crates" / "ahk-box" / "src" / "files.rs"
PICTURES = ROOT / "crates" / "ahk-box" / "src" / "pictures.rs"


def find_impl(lines, header):
    """The header of an impl and the line that closes it."""
    for index, line in enumerate(lines):
        if header in line:
            for end in range(index + 1, len(lines)):
                if lines[end].rstrip() == "}":
                    return index, end
            raise SystemExit(f"the impl {header} does not close")
    raise SystemExit(f"the tree has no {header}")


def signatures_of(lines, header):
    """The signatures of the methods of an impl, as (name, text, args)."""
    start, end = find_impl(lines, header)
    found = []
    index = start + 1
    while index < end:
        line = lines[index]
        stripped = line.strip()
        if stripped.startswith("fn "):
            block = [line]
            depth = line.count("(") - line.count(")")
            while depth > 0:
                index += 1
                block.append(lines[index])
                depth += lines[index].count("(") - lines[index].count(")")
            joined = " ".join(part.strip() for part in block)
            name = re.search(r"fn (\w+)\(", joined).group(1)
            # The parameters are the ones between the FIRST parenthesis and
            # its match: a return type has parentheses of its own, and the
            # last ")" of the line is not the end of the parameters.
            open_at = joined.index("(")
            depth = 0
            close_at = open_at
            for position in range(open_at, len(joined)):
                if joined[position] == "(":
                    depth += 1
                elif joined[position] == ")":
                    depth -= 1
                    if depth == 0:
                        close_at = position
                        break
            args = []
            for part in joined[open_at + 1 : close_at].split(","):
                part = part.strip()
                if not part or part.startswith("&mut self"):
                    continue
                arg = part.split(":")[0].strip()
                args.append(arg)
            found.append((name, block, args))
        index += 1
    return found


def groups(lines):
    """The lines in units: a comment and the method it belongs to.

    WHY the units: a method that is already there must not bring its
    comment again, and the comment must not be written without its
    method. The two travel together.
    """
    out = []
    current = []
    for line in lines:
        current.append(line)
        if line.strip() == "}":
            out.append(current)
            current = []
    if current:
        out.append(current)
    return out


def the_method_of(group):
    """The name of the method of a group, or nothing when it has none."""
    for line in group:
        if "fn " in line:
            return line.split("fn ")[1].split("(")[0]
    return None


def delegation(name, block, args, doc):
    """The method that asks the desktop of the box.

    The signature comes with its own opening brace -- it was read out of
    the tree -- so the body is written under it and the method is closed.
    """
    out = [doc]
    for line in block:
        out.append(line)
    out.append(f"        self.lab.{name}({', '.join(args)})")
    out.append("    }")
    return out


def main():
    x11 = read_lines(X11)

    # 0. The order of the parameters of `XGetWindowProperty`.
    #
    # WHY: Xlib's signature is
    #   (..., Atom req_type, Atom *actual_type_return, int *actual_format_return,
    #        unsigned long *nitems_return, ...)
    # and the wrapper of the tree had `actual_format` BEFORE `actual_type`.
    # The call wrote the type of the property into the format and the format
    # into the type -- and the X server wrote EIGHT bytes into the four of
    # the format: the stack of the caller was broken, and the check of the
    # format (`format == 32`) compared an ATOM, so the active window of an
    # X11 session was NEVER found.
    wrong = (
        "    xfn!(8, XGetWindowProperty(d: *mut c_void, w: c_ulong, property: c_ulong, "
        "long_offset: c_long, long_length: c_long, delete: c_int, req_type: c_ulong, "
        "actual_format: *mut c_int, actual_type: *mut c_ulong, nitems: *mut c_ulong, "
        "bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int);"
    )
    right = (
        "    xfn!(8, XGetWindowProperty(d: *mut c_void, w: c_ulong, property: c_ulong, "
        "long_offset: c_long, long_length: c_long, delete: c_int, req_type: c_ulong, "
        "actual_type: *mut c_ulong, actual_format: *mut c_int, nitems: *mut c_ulong, "
        "bytes_after: *mut c_ulong, prop: *mut *mut c_uchar) -> c_int);"
    )
    out = []
    for line in x11:
        if line.strip() == wrong.strip():
            out.append(right)
            continue
        out.append(line)
    x11 = out
    # And the call that answers with them follows the order of the wrapper.
    out = []
    index = 0
    while index < len(x11):
        line = x11[index]
        if (
            line.strip() == "&mut format,"
            and index + 1 < len(x11)
            and x11[index + 1].strip() == "&mut actual_type,"
        ):
            out.append("            &mut actual_type,")
            out.append("            &mut format,")
            index += 2
            continue
        out.append(line)
        index += 1
    x11 = out
    write_lines(X11, x11)

    # 1. The symbol of the pointer: XQueryPointer.
    if not any('b"XQueryPointer\\0"' in line for line in x11):
        x11 = [
            line.replace(
                '        b"XGetErrorText\\0",         // 23',
                '        b"XGetErrorText\\0",         // 23\n'
                '        b"XQueryPointer\\0",        // 24',
            )
            for line in x11
        ]
        x11 = [
            line.replace(
                "pub(super) const SYMBOL_COUNT: usize = 24;",
                "pub(super) const SYMBOL_COUNT: usize = 25;",
            )
            for line in x11
        ]
        x11 = [
            line.replace(
                "    xfn!(23, XGetErrorText(d: *mut c_void, code: c_int, buffer: *mut c_char, length: c_int) -> c_int);",
                "    xfn!(23, XGetErrorText(d: *mut c_void, code: c_int, buffer: *mut c_char, length: c_int) -> c_int);\n"
                "    /// `XQueryPointer`: the place of the pointer on the screen.\n"
                "    #[rustfmt::skip]\n"
                "    xfn!(24, XQueryPointer(d: *mut c_void, w: c_ulong, root_return: *mut c_ulong, child_return: *mut c_ulong, root_x: *mut c_int, root_y: *mut c_int, win_x: *mut c_int, win_y: *mut c_int, mask_return: *mut c_uint) -> c_int);",
            )
            for line in x11
        ]

    # 2. The desktop of the box inside the X11 one.
    if not any("lab: crate::desktop::LabDesktop," in line for line in x11):
        out = []
        for line in x11:
            out.append(line)
            if line.strip() == "failure: Option<String>,":
                out.append(
                    "    /// The desktop of the box, for the capabilities that are not about\n"
                    "    /// X11: the files, the dialogs, the machine, the pictures and the\n"
                    "    /// network.\n"
                    "    ///\n"
                    "    /// WHY it is kept and not made for each call: the files that are\n"
                    "    /// open and the pictures that were loaded are STATE, and a desktop\n"
                    "    /// made for each call would lose them between the calls.\n"
                    "    lab: crate::desktop::LabDesktop,"
                )
            if line.strip() == "failure: None,":
                out.append("            lab: crate::desktop::LabDesktop::new(),")
        x11 = out

    # 3. The place of the pointer: the X server knows it, XTEST only writes.
    if not any(
        "XQueryPointer answered nothing about the pointer" in line for line in x11
    ):
        start = None
        for index, line in enumerate(x11):
            if "fn mouse_position(&mut self)" in line:
                start = index
                break
        if start is None:
            raise SystemExit("the refusal of the pointer is not there")
        end = start
        while x11[end].strip() != "}":
            end += 1
        new = [
            "    fn mouse_position(&mut self) -> Result<(i64, i64), String> {",
            "        // The X server knows where the pointer is, and it says it with",
            "        // `XQueryPointer`: XTEST only WRITES places.",
            "        let display = self.connection()?;",
            "        let root = ffi::XDefaultRootWindow(display);",
            "        let mut root_return: c_ulong = 0;",
            "        let mut child_return: c_ulong = 0;",
            "        let mut root_x: c_int = 0;",
            "        let mut root_y: c_int = 0;",
            "        let mut win_x: c_int = 0;",
            "        let mut win_y: c_int = 0;",
            "        let mut mask: c_uint = 0;",
            "        let asked = ffi::XQueryPointer(",
            "            display,",
            "            root,",
            "            &mut root_return,",
            "            &mut child_return,",
            "            &mut root_x,",
            "            &mut root_y,",
            "            &mut win_x,",
            "            &mut win_y,",
            "            &mut mask,",
            "        );",
            "        if asked == 0 {",
            '            return Err("XQueryPointer answered nothing about the pointer".to_string());',
            "        }",
            "        Ok((root_x as i64, root_y as i64))",
            "    }",
        ]
        x11 = x11[:start] + new + x11[end + 1 :]

    write_lines(X11, x11)

    # 4. The files: the delegations, taken from the desktop of the box.
    files_impl = read_lines(FILES)
    x11 = read_lines(X11)
    methods = signatures_of(files_impl, "impl ahk::desktop::Files for LabDesktop {")
    existing = "\n".join(x11)
    block = []
    for name, signature, args in methods:
        if name in ("recycle", "empty_trash"):
            continue
        if f"fn {name}(" in existing:
            continue
        block = block + delegation(
            name,
            signature,
            args,
            "    /// The files of the machine: the file system of the box, which is\n"
            "    /// not about X11 at all.",
        )
        block.append("")
    if block:
        start, end = find_impl(x11, "impl Files for X11Desktop {")
        x11 = x11[:end] + block + x11[end:]
    write_lines(X11, x11)

    # 5. The pictures: the matching of a picture is the same either way,
    # and the capture of the screen is the business of the backend.
    pictures = read_lines(PICTURES)
    if not any("pub fn search_in(" in line for line in pictures):
        start = None
        for index, line in enumerate(pictures):
            if line.startswith("pub fn search("):
                start = index
                break
        if start is None:
            raise SystemExit("the search of the pictures is not there")
        end = start
        while pictures[end].rstrip() != "}":
            end += 1
        new = [
            "pub fn search(",
            "    desktop: &mut LabDesktop,",
            "    x1: i64,",
            "    y1: i64,",
            "    x2: i64,",
            "    y2: i64,",
            "    picture: &Picture,",
            "    options: &ImageOptions,",
            ") -> Option<(i64, i64)> {",
            "    let (left, top, width, height) = the_region(x1, y1, x2, y2);",
            "    let path = desktop",
            "        .capture(",
            "            left,",
            "            top,",
            "            width as i64,",
            "            height as i64,",
            "            &format!(\"ahk-image-{}\", millis()),",
            "        )",
            "        .ok()?;",
            "    let screen = read(&path).ok()?;",
            "    let _ = std::fs::remove_file(&path);",
            "    let inside = search_in(picture, &screen, options)?;",
            "    Some((inside.0 + left, inside.1 + top))",
            "}",
            "",
            "/// The region of the screen that a search covers.",
            "///",
            "/// WHY it is its own function: the two backends of the capture -- `grim`",
            "/// on the box and `XGetImage` on an X11 session -- need the same",
            "/// region, and the two must agree about what a region is.",
            "pub fn the_region(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64, usize, usize) {",
            "    let (left, top) = (x1.min(x2), y1.min(x2));",
            "    let (right, bottom) = (x2.max(x1), y2.max(y1));",
            "    let width = right - left + 1;",
            "    let height = bottom - top + 1;",
            "    (left, top, width.max(0) as usize, height.max(0) as usize)",
            "}",
            "",
            "/// Look for a picture in a picture of the screen.",
            "///",
            "/// WHY it is separate from `search`: the capture of the screen is the",
            "/// business of the backend -- `grim` on the box, `XGetImage` on an X11",
            "/// session -- and the MATCHING of the picture is the same either way.",
            "///",
            "/// The answer is the place of the picture INSIDE the picture of the",
            "/// screen: the caller adds the place of the screen itself.",
            "pub fn search_in(",
            "    picture: &Picture,",
            "    screen: &Picture,",
            "    options: &ImageOptions,",
            ") -> Option<(i64, i64)> {",
            "    let transparent = options.transparent.as_deref().and_then(parse_colour);",
            "    let variation = options.variation.clamp(0, 255) as i32;",
            "    if picture.width > screen.width || picture.height > screen.height {",
            "        return None;",
            "    }",
            "    for y in 0..=(screen.height - picture.height) {",
            "        for x in 0..=(screen.width - picture.width) {",
            "            if the_picture_is_there(screen, picture, x, y, variation, transparent) {",
            "                return Some((x as i64, y as i64));",
            "            }",
            "        }",
            "    }",
            "    None",
            "}",
            "",
            "/// The picture of a file, or the picture of one that was loaded.",
            "///",
            "/// WHY it is its own function: the pictures that were loaded live in",
            "/// the desktop that loaded them, and a backend that borrows that",
            "/// desktop asks for the picture here.",
            "pub fn the_picture_of(desktop: &LabDesktop, image: &Image) -> Result<Picture, ImageSearch> {",
            "    match image {",
            "        Image::File(name) => {",
            "            if !Path::new(name).exists() {",
            "                return Err(ImageSearch::NotLoadable);",
            "            }",
            "            read(name).map_err(|_| ImageSearch::NotLoadable)",
            "        }",
            "        Image::Loaded(handle) => {",
            "            let handle = *handle as usize;",
            "            match desktop.pictures.get(handle.wrapping_sub(1)) {",
            "                Some(picture) => Ok(Picture {",
            "                    pixels: picture.pixels.clone(),",
            "                    width: picture.width,",
            "                    height: picture.height,",
            "                }),",
            "                None => Err(ImageSearch::NotLoadable),",
            "            }",
            "        }",
            "    }",
            "}",
        ]
        pictures = pictures[:start] + new + pictures[end + 1 :]
    write_lines(PICTURES, pictures)

    # 6. The dialogs, the pictures, the network and the screen of the X11
    # backend: the desktop of the box answers them.
    x11 = read_lines(X11)
    if "impl Dialogs for X11Desktop {}" in "\n".join(x11):
        x11 = [
            line.replace(
                "impl Dialogs for X11Desktop {}",
                "\n".join(
                    [
                        "impl Dialogs for X11Desktop {",
                        "    /// A notification of the session: the box answers with",
                        "    /// `notify-send`, and the failure is the reason of the desktop.",
                        "    fn notify(&mut self, title: &str, text: &str) -> Result<(), String> {",
                        "        self.lab.notify(title, text)",
                        "    }",
                        "    /// The question of the user: `zenity` opens it on the display",
                        "    /// of the session.",
                        "    fn input_box(",
                        "        &mut self,",
                        "        prompt: &str,",
                        "        title: &str,",
                        "        options: &ahk::desktop::InputOptions,",
                        "        default: &str,",
                        "    ) -> Result<ahk::desktop::InputAnswer, String> {",
                        "        self.lab.input_box(prompt, title, options, default)",
                        "    }",
                        "    /// The choice of a file.",
                        "    fn file_select(",
                        "        &mut self,",
                        "        options: &ahk::desktop::FileOptions,",
                        "        root: &str,",
                        "        title: &str,",
                        "        filter: &str,",
                        "    ) -> Result<ahk::desktop::FileAnswer, String> {",
                        "        self.lab.file_select(options, root, title, filter)",
                        "    }",
                        "    /// The choice of a folder.",
                        "    fn dir_select(",
                        "        &mut self,",
                        "        starting: &str,",
                        "        options: i64,",
                        "        prompt: &str,",
                        "    ) -> Result<String, String> {",
                        "        self.lab.dir_select(starting, options, prompt)",
                        "    }",
                        "}",
                    ]
                ),
            )
            for line in x11
        ]
    if "impl Pictures for X11Desktop {}" in "\n".join(x11):
        x11 = [
            line.replace(
                "impl Pictures for X11Desktop {}",
                "\n".join(
                    [
                        "impl Pictures for X11Desktop {",
                        "    /// The picture of a file: the `png` crate reads it, and it is",
                        "    /// not about X11 at all.",
                        "    fn load_picture(",
                        "        &mut self,",
                        "        filename: &str,",
                        "        options: &ahk::desktop::PictureOptions,",
                        "    ) -> Result<Option<u32>, String> {",
                        "        self.lab.load_picture(filename, options)",
                        "    }",
                        "    /// The look for a picture on the screen: the capture is the",
                        "    /// X11 one (`XGetImage`), and the matching is the one of the box.",
                        "    fn image_search(",
                        "        &mut self,",
                        "        x1: i64,",
                        "        y1: i64,",
                        "        x2: i64,",
                        "        y2: i64,",
                        "        image: &ahk::desktop::Image,",
                        "        options: &ahk::desktop::ImageOptions,",
                        "    ) -> Result<ahk::desktop::ImageSearch, String> {",
                        "        let picture = match crate::pictures::the_picture_of(&self.lab, image) {",
                        "            Ok(picture) => picture,",
                        "            Err(answer) => return Ok(answer),",
                        "        };",
                        "        let picture = crate::pictures::resized(picture, options.width, options.height);",
                        "        let (left, top, width, height) = crate::pictures::the_region(x1, y1, x2, y2);",
                        "        if width == 0 || height == 0 {",
                        "            return Ok(ahk::desktop::ImageSearch::NotFound);",
                        "        }",
                        "        let pixels = self.image_rgb(left, top, width as i64, height as i64)?;",
                        "        let screen = crate::pictures::Picture {",
                        "            pixels,",
                        "            width,",
                        "            height,",
                        "        };",
                        "        Ok(match crate::pictures::search_in(&picture, &screen, options) {",
                        "            Some((x, y)) => ahk::desktop::ImageSearch::Found(x + left, y + top),",
                        "            None => ahk::desktop::ImageSearch::NotFound,",
                        "        })",
                        "    }",
                        "}",
                    ]
                ),
            )
            for line in x11
        ]
    if "impl Network for X11Desktop {}" in "\n".join(x11):
        x11 = [
            line.replace(
                "impl Network for X11Desktop {}",
                "\n".join(
                    [
                        "impl Network for X11Desktop {",
                        "    /// The download is the machine's, and the machine is the box.",
                        "    fn download(&mut self, url: &str, file: &str) -> Result<String, String> {",
                        "        self.lab.download(url, file)",
                        "    }",
                        "}",
                    ]
                ),
            )
            for line in x11
        ]
    write_lines(X11, x11)

    # 7. The machine and the screen of the X11 backend: the questions that
    # are not about X11 are the ones of the box.
    machine = [
        "    /// The name of the user of the machine.",
        "    fn user_name(&mut self) -> Result<String, String> {",
        "        self.lab.user_name()",
        "    }",
        "    /// The name of the machine.",
        "    fn computer_name(&mut self) -> Result<String, String> {",
        "        self.lab.computer_name()",
        "    }",
        "    /// The shell of the user.",
        "    fn shell(&mut self) -> Result<String, String> {",
        "        self.lab.shell()",
        "    }",
        "    /// The folders of the machine.",
        "    fn folders(&mut self) -> Result<ahk::desktop::Folders, String> {",
        "        self.lab.folders()",
        "    }",
        "    /// The version of the system of the machine.",
        "    fn os_version(&mut self) -> Result<String, String> {",
        "        self.lab.os_version()",
        "    }",
        "    /// The language of the machine.",
        "    fn language(&mut self) -> Result<String, String> {",
        "        self.lab.language()",
        "    }",
        "    /// Whether the machine is an administrator one.",
        "    fn is_admin(&mut self) -> Result<bool, String> {",
        "        self.lab.is_admin()",
        "    }",
        "    /// The times the machine has been idle.",
        "    fn idle_times(&mut self) -> Result<ahk::desktop::Idle, String> {",
        "        self.lab.idle_times()",
        "    }",
        "    /// The path of the program that runs the script.",
        "    fn executable(&mut self) -> Result<String, String> {",
        "        self.lab.executable()",
        "    }",
    ]
    x11 = read_lines(X11)
    start, end = find_impl(x11, "impl Machine for X11Desktop {")
    existing = "\n".join(x11[start:end])
    block = []
    for group in groups(machine):
        name = the_method_of(group)
        if name is None or f"fn {name}(" in existing:
            continue
        block = block + group
    if block:
        x11 = x11[:end] + block + x11[end:]
    write_lines(X11, x11)

    screen = [
        "    /// The scale of the output: the toolkit of the session says it, and",
        "    /// the helper of the window asks the toolkit.",
        "    fn screen_scale(&mut self) -> Result<i64, String> {",
        "        self.lab.screen_scale()",
        "    }",
        "    /// The text that a tip shows.",
        "    fn tooltip_show(&mut self, text: &str, x: i64, y: i64) -> Result<i64, String> {",
        "        self.lab.tooltip_show(text, x, y)",
        "    }",
        "    /// The place and the size of a window of the script.",
        "    fn window_move(&mut self, x: i64, y: i64, width: i64, height: i64) -> Result<(), String> {",
        "        self.lab.window_move(x, y, width, height)",
        "    }",
        "    /// The controls of the window of the script.",
        "    fn gui_controls(&mut self) -> Result<String, String> {",
        "        self.lab.gui_controls()",
        "    }",
        "    /// The text of every control of the window of the script.",
        "    fn gui_all_text(&mut self) -> Result<String, String> {",
        "        self.lab.gui_all_text()",
        "    }",
        "    /// The opacity of the window of the script.",
        "    fn gui_opacity(&mut self, setting: Option<i64>) -> Result<i64, String> {",
        "        self.lab.gui_opacity(setting)",
        "    }",
        "    /// The paste of a text into a control of the window of the script.",
        "    fn gui_paste(&mut self, number: u64, text: &str) -> Result<(), String> {",
        "        self.lab.gui_paste(number, text)",
        "    }",
    ]
    x11 = read_lines(X11)
    start, end = find_impl(x11, "impl Screen for X11Desktop {")
    existing = "\n".join(x11[start:end])
    block = []
    for group in groups(screen):
        name = the_method_of(group)
        if name is None or f"fn {name}(" in existing:
            continue
        block = block + group
    if block:
        x11 = x11[:end] + block + x11[end:]
    write_lines(X11, x11)
    # 8. The native calls, the hotkeys and the sound: the raven, the
    # keyboard of the machine and `pactl` are not about X11 either.
    natives = [
        "impl Native for X11Desktop {",
        "    /// The call of a library: the raven answers for the libraries of",
        "    /// Windows, and the machine answers for the ones of the machine.",
        "    fn call_native(",
        "        &mut self,",
        "        call: &mut ahk::dll::NativeCall,",
        "    ) -> Result<ahk::dll::NativeResult, ahk::dll::NativeFailure> {",
        "        self.lab.call_native(call)",
        "    }",
        "}",
    ]
    hotkeys = [
        "impl Hotkeys for X11Desktop {",
        "    /// The hotkeys of the script: the watcher of the keyboard of the",
        "    /// machine answers for them.",
        "    fn register_hotkey(&mut self, name: &str) -> Result<String, String> {",
        "        self.lab.register_hotkey(name)",
        "    }",
        "    fn unregister_hotkey(&mut self, name: &str) -> Result<String, String> {",
        "        self.lab.unregister_hotkey(name)",
        "    }",
        "    fn hotkeys(&mut self) -> Result<Vec<String>, String> {",
        "        self.lab.hotkeys()",
        "    }",
        "    fn fired_hotkey(&mut self) -> Result<Option<String>, String> {",
        "        self.lab.fired_hotkey()",
        "    }",
        "}",
    ]
    sound = [
        "impl Sound for X11Desktop {",
        "    /// The sound of the machine: `pactl` answers for the devices.",
        "    fn sound_device_named(&mut self, device: &str) -> Result<Option<String>, String> {",
        "        self.lab.sound_device_named(device)",
        "    }",
        "    fn sound_device_names(&mut self) -> Result<(String, String), String> {",
        "        self.lab.sound_device_names()",
        "    }",
        "    fn sound_volume(&mut self, device: &str) -> Result<f64, String> {",
        "        self.lab.sound_volume(device)",
        "    }",
        "    fn sound_set_volume(&mut self, device: &str, volume: f64) -> Result<(), String> {",
        "        self.lab.sound_set_volume(device, volume)",
        "    }",
        "    fn sound_muted(&mut self, device: &str) -> Result<bool, String> {",
        "        self.lab.sound_muted(device)",
        "    }",
        "    fn sound_set_muted(&mut self, device: &str, muted: bool) -> Result<(), String> {",
        "        self.lab.sound_set_muted(device, muted)",
        "    }",
        "}",
    ]
    x11 = read_lines(X11)
    joined = "\n".join(x11)
    for header, block, marker in (
        ("impl Native for X11Desktop {}", natives, "fn call_native("),
        ("impl Hotkeys for X11Desktop {}", hotkeys, "fn register_hotkey("),
        ("impl Sound for X11Desktop {}", sound, "fn sound_device_named("),
    ):
        # The impl is replaced when it is still the empty one; a method that
        # is already there means the work is done.
        if marker in joined:
            continue
        x11 = [line.replace(header, "\n".join(block)) for line in x11]
        joined = "\n".join(x11)
    write_lines(X11, x11)

    # 9. The machine of the X11 backend: the registry, the tray and the
    # two directories of the Windows are the ones of the box.
    machine_more = [
        "    /// The icon of the script in the tray of the session.",
        "    fn tray_set_icon(&mut self, name: &str) -> Result<(), String> {",
        "        self.lab.tray_set_icon(name)",
        "    }",
        "    /// The directory of the programs of the user.",
        "    fn program_files(&mut self) -> Result<String, String> {",
        "        self.lab.program_files()",
        "    }",
        "    /// The directory of the Windows of the machine.",
        "    fn windows_directory(&mut self) -> Result<String, String> {",
        "        self.lab.windows_directory()",
        "    }",
        "    /// The registry of the machine: the store of the box.",
        "    fn registry_create_key(&mut self, key: &str) -> Result<(), String> {",
        "        self.lab.registry_create_key(key)",
        "    }",
        "    fn registry_delete_key(&mut self, key: &str) -> Result<(), String> {",
        "        self.lab.registry_delete_key(key)",
        "    }",
        "    fn registry_delete(&mut self, key: &str, name: &str) -> Result<(), String> {",
        "        self.lab.registry_delete(key, name)",
        "    }",
        "    fn registry_write(",
        "        &mut self,",
        "        key: &str,",
        "        name: &str,",
        "        kind: &str,",
        "        value: &str,",
        "    ) -> Result<(), String> {",
        "        self.lab.registry_write(key, name, kind, value)",
        "    }",
        "    fn registry_value(",
        "        &mut self,",
        "        key: &str,",
        "        name: &str,",
        "    ) -> Result<Option<(String, String)>, String> {",
        "        self.lab.registry_value(key, name)",
        "    }",
    ]
    x11 = read_lines(X11)
    start, end = find_impl(x11, "impl Machine for X11Desktop {")
    existing = "\n".join(x11[start:end])
    block = []
    for group in groups(machine_more):
        name = the_method_of(group)
        if name is None or f"fn {name}(" in existing:
            continue
        block = block + group
    if block:
        x11 = x11[:end] + block + x11[end:]
    write_lines(X11, x11)

    # 10. The clipboard of an X11 session: the protocol of the selections.
    #
    # WHY the protocol and not `wl-copy`: the clipboard of an X11 session
    # is a SELECTION, and it is asked of the window that OWNS it. The
    # tree of the box uses `wl-copy` and `wl-paste`, and those speak
    # Wayland.
    if "fn clipboard_read(&mut self) -> Result<String, String> {" not in "\n".join(
        read_lines(X11)
    ):
        x11 = read_lines(X11)
        out = []
        for line in x11:
            if line.strip() == "pub(super) const SYMBOL_COUNT: usize = 25;":
                out.append("pub(super) const SYMBOL_COUNT: usize = 32;")
                continue
            out.append(line)
            if line.strip() == "b\"XQueryPointer\\0\",        // 24":
                out.append(
                    '        b"XConvertSelection\\0",     // 25'
                )
                out.append('        b"XSetSelectionOwner\\0",   // 26')
                out.append('        b"XGetSelectionOwner\\0",   // 27')
                out.append('        b"XCreateSimpleWindow\\0",  // 28')
                out.append('        b"XChangeProperty\\0",      // 29')
                out.append('        b"XSendEvent\\0",           // 30')
                out.append('        b"XMapWindow\\0",          // 31')
            if line.strip() == "lab: crate::desktop::LabDesktop,":
                out.append(
                    "    /// The window of the script that owns the clipboard of the"
                )
                out.append("    /// session.")
                out.append("    ///")
                out.append(
                    "    /// WHY a window of its own: the owner of a selection is a"
                )
                out.append("    /// WINDOW, and the root window of the screen is not one.")
                out.append("    clipboard_window: c_ulong,")
                out.append("    /// The text the script owns in the clipboard of the session.")
                out.append("    clipboard_text: Option<String>,")
            if line.strip() == "lab: crate::desktop::LabDesktop::new(),":
                out.append("            clipboard_window: 0,")
                out.append("            clipboard_text: None,")
        x11 = out

        # The calls of the protocol of the selections.
        wrappers = [
            "    /// `XConvertSelection`: the ask of a selection to its owner.",
            "    #[rustfmt::skip]",
            "    xfn!(25, XConvertSelection(d: *mut c_void, selection: c_ulong, target: c_ulong, property: c_ulong, requestor: c_ulong, time: c_ulong) -> c_int);",
            "    /// `XSetSelectionOwner`: the script becomes the owner of a selection.",
            "    #[rustfmt::skip]",
            "    xfn!(26, XSetSelectionOwner(d: *mut c_void, selection: c_ulong, owner: c_ulong, time: c_ulong) -> c_int);",
            "    /// `XGetSelectionOwner`: the window that owns a selection.",
            "    #[rustfmt::skip]",
            "    xfn!(27, XGetSelectionOwner(d: *mut c_void, selection: c_ulong) -> c_ulong);",
            "    /// `XCreateSimpleWindow`: the window of the script.",
            "    #[rustfmt::skip]",
            "    xfn!(28, XCreateSimpleWindow(d: *mut c_void, parent: c_ulong, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, border: c_ulong, background: c_ulong) -> c_ulong);",
            "    /// `XChangeProperty`: the answer of the owner of a selection.",
            "    #[rustfmt::skip]",
            "    xfn!(29, XChangeProperty(d: *mut c_void, w: c_ulong, property: c_ulong, kind: c_ulong, format: c_int, mode: c_int, data: *const c_uchar, items: c_int) -> c_int);",
            "    /// `XSendEvent`: the answer of the owner, sent to the one that asked.",
            "    #[rustfmt::skip]",
            "    xfn!(30, XSendEvent(d: *mut c_void, w: c_ulong, propagate: c_int, mask: c_long, event: *mut c_void) -> c_int);",
            "    /// `XMapWindow`: the window of the script, shown.",
            "    ///",
            "    /// WHY it is shown: the X server accepts the ownership of a selection",
            "    /// only from a VIEWABLE window, and an unmapped one is refused with",
            "    /// a `BadWindow` -- the clipboard would then have no owner at all.",
            "    #[rustfmt::skip]",
            "    xfn!(31, XMapWindow(d: *mut c_void, w: c_ulong) -> c_int);",
        ]
        out = []
        for line in x11:
            out.append(line)
            if line.strip().startswith("xfn!(24, XQueryPointer("):
                out.extend(wrappers)
        x11 = out
        write_lines(X11, x11)

    # 11. The mirror of the events of the selections, the methods of the
    # clipboard and the impl itself.
    if "struct XSelectionEvent {" not in "\n".join(read_lines(X11)):
        x11 = read_lines(X11)
        mirror = """/// The events of the selections of X11.
///
/// WHY one mirror for the two: `XSelectionRequestEvent` and
/// `XSelectionNotifyEvent` carry the SAME fields in the SAME order, and
/// the `type` of the event says which one it is.
///
/// WHY the word before the requestor: the measurement on the VM on
/// 2026-11-11 read the words of the event and found the window of the
/// requestor in TWO of them -- the one that is at the offset 32 and the
/// one that is at the offset 40. The requestor is the second one: the
/// selection of the event was read at the offset 48, the target at the
/// offset 56, the property at the offset 64 and the time at the offset
/// 72. The mirror carries the word that is before the requestor, and the
/// offsets are pinned so that a wrong one is a failure of the build and
/// not a wrong answer at the run.
#[repr(C)]
struct XSelectionEvent {
    kind: c_int,
    serial: c_ulong,
    send_event: c_int,
    display: *mut c_void,
    /// The word that the event carries before the requestor: it holds the
    /// window of the requestor as well.
    before_the_requester: c_ulong,
    requestor: c_ulong,
    selection: c_ulong,
    target: c_ulong,
    property: c_ulong,
    time: c_ulong,
}

const _: () = assert!(std::mem::offset_of!(XSelectionEvent, requestor) == 40);
const _: () = assert!(std::mem::offset_of!(XSelectionEvent, selection) == 48);
const _: () = assert!(std::mem::offset_of!(XSelectionEvent, target) == 56);
const _: () = assert!(std::mem::offset_of!(XSelectionEvent, property) == 64);
const _: () = assert!(std::mem::offset_of!(XSelectionEvent, time) == 72);
"""
        methods = """impl X11Desktop {
    /// The atom of a name of the X server.
    fn the_atom(display: *mut c_void, name: &str) -> c_ulong {
        let mut bytes = name.as_bytes().to_vec();
        bytes.push(0);
        ffi::XInternAtom(display, bytes.as_ptr().cast::<c_char>(), 0)
    }

    /// The window of the script that owns the clipboard of the session.
    ///
    /// WHY a window of its own: the owner of a selection is a WINDOW, and
    /// the root window of the screen is not one.
    fn the_clipboard_window(&mut self, display: *mut c_void) -> Result<c_ulong, String> {
        if self.clipboard_window != 0 {
            return Ok(self.clipboard_window);
        }
        let root = ffi::XDefaultRootWindow(display);
        let window = ffi::XCreateSimpleWindow(display, root, 0, 0, 1, 1, 0, 0, 0);
        if window == 0 {
            return Err("the X server did not give a window to the script".to_string());
        }
        // The window is shown: the X server accepts the ownership of a
        // selection only from a VIEWABLE window, and it refuses an
        // unmapped one with a `BadWindow` -- the clipboard would then have
        // no owner at all, and every read of it would answer nothing.
        ffi::XMapWindow(display, window);
        ffi::XFlush(display);
        self.clipboard_window = window;
        Ok(window)
    }

    /// Answer a request of the clipboard, with the text the script owns.
    ///
    /// WHY the answer is sent and not left to the server: the owner of a
    /// selection is the client itself, and the client is this tree.
    fn serve_the_selection(&mut self, display: *mut c_void, request: &XSelectionEvent) {
        let Some(text) = self.clipboard_text.clone() else {
            return;
        };
        let utf8 = Self::the_atom(display, "UTF8_STRING");
        let targets = Self::the_atom(display, "TARGETS");
        let mut property = if request.property == 0 {
            request.target
        } else {
            request.property
        };
        if request.target == utf8 {
            let bytes = text.as_bytes();
            ffi::XChangeProperty(
                display,
                request.requestor,
                property,
                utf8,
                8,
                0,
                bytes.as_ptr().cast::<c_uchar>(),
                bytes.len() as c_int,
            );
        } else if request.target == targets {
            let mut list = [targets, utf8];
            ffi::XChangeProperty(
                display,
                request.requestor,
                property,
                4,
                32,
                0,
                list.as_mut_ptr().cast::<c_uchar>(),
                list.len() as c_int,
            );
        } else {
            // A target the script does not carry: the property is None,
            // and that is how an owner says "no".
            property = 0;
        }
        let mut notify = XSelectionEvent {
            kind: SELECTION_NOTIFY,
            serial: 0,
            send_event: 1,
            display,
            before_the_requester: request.requestor,
            requestor: request.requestor,
            selection: request.selection,
            target: request.target,
            property,
            time: request.time,
        };
        ffi::XSendEvent(
            display,
            request.requestor,
            0,
            0,
            (&mut notify as *mut XSelectionEvent).cast::<c_void>(),
        );
        ffi::XFlush(display);
    }

    /// The text of a property of the window of the script.
    fn the_text_of_the_property(
        &self,
        display: *mut c_void,
        window: c_ulong,
        property: c_ulong,
    ) -> String {
        let mut actual_type: c_ulong = 0;
        let mut actual_format: c_int = 0;
        let mut items: c_ulong = 0;
        let mut after: c_ulong = 0;
        let mut data: *mut c_uchar = std::ptr::null_mut();
        let got = ffi::XGetWindowProperty(
            display,
            window,
            property,
            0,
            1_048_575,
            0,
            0,
            &mut actual_type,
            &mut actual_format,
            &mut items,
            &mut after,
            &mut data,
        );
        if got != 0 || data.is_null() {
            return String::new();
        }
        let length = match actual_format {
            8 => items as usize,
            16 => items as usize * 2,
            32 => items as usize * 4,
            _ => 0,
        };
        let bytes = unsafe { std::slice::from_raw_parts(data, length) };
        let text = String::from_utf8_lossy(bytes).into_owned();
        ffi::XFree(data.cast::<c_void>());
        text
    }

    /// The text of the clipboard of the session.
    fn read_the_clipboard(&mut self, display: *mut c_void) -> Result<String, String> {
        let window = self.the_clipboard_window(display)?;
        let clipboard = Self::the_atom(display, "CLIPBOARD");
        let utf8 = Self::the_atom(display, "UTF8_STRING");
        let property = Self::the_atom(display, "AHK_CLIPBOARD");
        // Nobody owns the clipboard: there is nothing to read, and that
        // is not a failure.
        if ffi::XGetSelectionOwner(display, clipboard) == 0 {
            return Ok(String::new());
        }
        ffi::XConvertSelection(display, clipboard, utf8, property, window, CURRENT_TIME);
        ffi::XFlush(display);
        // The owner answers with an EVENT, and the event arrives in the
        // queue of the connection: the wait is bounded, because a client
        // that never answers must not take the run with it.
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(1500);
        loop {
            while ffi::XPending(display) > 0 {
                let mut event = [0usize; EVENT_WORDS];
                ffi::XNextEvent(display, event.as_mut_ptr().cast::<c_void>());
                let kind = event[0] as c_int;
                if kind == SELECTION_REQUEST {
                    // Another client asks for the clipboard: the script
                    // owns it, and the script answers.
                    let request = unsafe { &*(event.as_ptr().cast::<XSelectionEvent>()) };
                    self.serve_the_selection(display, request);
                    continue;
                }
                if kind == SELECTION_NOTIFY {
                    let notify = unsafe { &*(event.as_ptr().cast::<XSelectionEvent>()) };
                    if notify.property == 0 {
                        // The owner refused the request.
                        return Ok(String::new());
                    }
                    return Ok(self.the_text_of_the_property(display, window, notify.property));
                }
            }
            if std::time::Instant::now() >= deadline {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        Err("the owner of the clipboard did not answer in time".to_string())
    }
}
"""
        clipboard = """impl Clipboard for X11Desktop {
    /// The text of the clipboard of the session.
    ///
    /// WHY the protocol of the selections: the clipboard of an X11
    /// session is a SELECTION, and it is asked of the window that owns
    /// it. The tree of the box uses `wl-paste`, and that speaks
    /// Wayland.
    fn clipboard_read(&mut self) -> Result<String, String> {
        let display = self.connection()?;
        self.read_the_clipboard(display)
    }

    /// Put a text in the clipboard of the session.
    ///
    /// WHY the script owns it: the owner of a selection is a WINDOW, and
    /// the script needs a window of its own to be one. The text is
    /// answered when another client asks for it.
    fn clipboard_write(&mut self, text: &str) -> Result<String, String> {
        let display = self.connection()?;
        let window = self.the_clipboard_window(display)?;
        let clipboard = Self::the_atom(display, "CLIPBOARD");
        self.clipboard_text = Some(text.to_string());
        let owned = ffi::XSetSelectionOwner(display, clipboard, window, CURRENT_TIME);
        ffi::XFlush(display);
        if owned == 0 {
            return Err("the X server did not accept the ownership of the clipboard".to_string());
        }
        Ok(format!("the clipboard of the session holds {text:?}"))
    }
}
"""
        out = []
        for line in x11:
            if line.strip() == "impl Clipboard for X11Desktop {}":
                out.append(methods.rstrip("\n"))
                out.append(clipboard.rstrip("\n"))
                continue
            out.append(line)
            if line.strip() == "/// Xlib's `XWindowAttributes` is 136 bytes on LP64; a larger buffer is safe.":
                out.append(mirror.rstrip("\n"))
        x11 = out
        write_lines(X11, x11)

    # 12. The events of the selections, and the answer of the owner when
    # the events of the keyboard are drained.
    if "const SELECTION_REQUEST: c_int = 30;" not in "\n".join(read_lines(X11)):
        x11 = read_lines(X11)
        out = []
        for line in x11:
            out.append(line)
            if line.strip() == "const KEY_PRESS: c_int = 2;":
                out.append("")
                out.append("/// The event of a request of a selection: another client asks.")
                out.append("const SELECTION_REQUEST: c_int = 30;")
                out.append("/// The event that carries the answer of the owner of a selection.")
                out.append("const SELECTION_NOTIFY: c_int = 31;")
        x11 = out
        write_lines(X11, x11)

    if "another client asks for the clipboard" not in "\n".join(read_lines(X11)):
        x11 = read_lines(X11)
        out = []
        for line in x11:
            out.append(line)
            if line.strip() == "if (event[0] as c_int) != KEY_PRESS {":
                out.append("            if (event[0] as c_int) == SELECTION_REQUEST {")
                out.append("                // Another client asks for the clipboard: the script")
                out.append("                // owns it, and the script answers.")
                out.append(
                    "                let request = unsafe { &*(event.as_ptr().cast::<XSelectionEvent>()) };"
                )
                out.append("                self.serve_the_selection(display, request);")
                out.append("                continue;")
                out.append("            }")
        x11 = out
        write_lines(X11, x11)

    print("the desktop of the box is behind the X11 backend")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
