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
    block = [line for line in machine if f"fn {line.split('(')[0][3:]}(" not in existing]
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
    for line in screen:
        if "fn " not in line:
            block = block + [line]
            continue
        name = line.split("fn ")[1].split("(")[0]
        if f"fn {name}(" not in existing:
            block = block + [line]
    if block:
        x11 = x11[:end] + block + x11[end:]
    write_lines(X11, x11)

    print("the desktop of the box is behind the X11 backend")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
