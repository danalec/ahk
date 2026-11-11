#!/usr/bin/python3
"""Merge the client of the compositor of the branch into the one of the tree.

The two clients of the compositor diverged: the tree of the owner learned
to name the windows of the session with a number (`windows_with_ids`), and
the branch learned where the windows are and how big they are (the
`geometry` event of the protocol of COSMIC) and how many pixels of an
output are in one point of it (the `scale` event).

Neither client is copied: the merge is by hand, and the script writes the
two texts that the two trees disagree about. The script refuses to write
when an anchor is not where it says it is, and it can be run twice.

    python audit/port-wayland-merge.py
"""

import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from port_text import read_lines, write_lines  # noqa: E402

ROOT = pathlib.Path(__file__).resolve().parent.parent

WAYLAND = ROOT / "crates" / "ahk-box" / "src" / "wayland.rs"
LINUX = ROOT / "crates" / "ahk-box" / "src" / "wayland" / "linux.rs"
TRAIT = ROOT / "crates" / "ahk" / "src" / "desktop.rs"
BOX = ROOT / "crates" / "ahk-box" / "src" / "desktop.rs"
TESTS = ROOT / "crates" / "ahk" / "tests" / "builtins.rs"

# The window of the compositor says where it is and in what state.
THE_WINDOW = [
    (
        [
            "    /// Whether the window is the active one.",
            "    pub active: bool,",
        ],
        [
            "    /// Whether the window is the active one.",
            "    pub active: bool,",
            "    /// Whether the window is maximized.",
            "    pub maximized: bool,",
            "    /// Whether the window is minimized.",
            "    pub minimized: bool,",
            "    /// Where the window is and how big it is, when the compositor says it.",
            "    ///",
            "    /// `None` is the compositor that did not say it, and it is not the same",
            "    /// as a window at the origin with no size: the geometry of a window that",
            "    /// the compositor has not described is not known, and a script must be",
            "    /// told that, not given a zero.",
            "    pub geometry: Option<Geometry>,",
        ],
    ),
]

# The geometry of a window, in the coordinates of the screen.
THE_GEOMETRY = """
/// The geometry of a window, in the coordinates of the screen.
///
/// The x and the y are the upper-left corner and not the center, and the
/// width and the height are of the window the user sees.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Geometry {
    /// The left of the window.
    pub x: i32,
    /// The top of the window.
    pub y: i32,
    /// How wide the window is.
    pub width: i32,
    /// How tall the window is.
    pub height: i32,
}
"""

# The two states of a window that the compositor says.
THE_STATES = [
    (
        [
            "const ACTIVATED: u32 = 2;",
        ],
        [
            "const ACTIVATED: u32 = 2;",
            "/// The state of a window that is maximized.",
            "const MAXIMIZED: u32 = 1;",
            "/// The state of a window that is minimized.",
            "const MINIMIZED: u32 = 4;",
        ],
    ),
]

# The state of the window: the tree of the owner knew only the focus.
THE_STATE_OF_THE_WINDOW = (
    [
        "        if let CosmicEvent::State { state: states } = event {",
        "            // The states are numbers, four bytes each, and the array",
        "            // carries its length in bytes.",
        "            toplevel.active = states.chunks_exact(4).any(|state| {",
        "                u32::from_ne_bytes([state[0], state[1], state[2], state[3]]) == ACTIVATED",
        "            });",
        "            // The compositor has said the state of this window.",
        "            toplevel.state_seen = true;",
        "        }",
    ],
    [
        "        if let CosmicEvent::State { state: states } = &event {",
        "            // The states are numbers, four bytes each, and the array",
        "            // carries its length in bytes.",
        "            let mut active = false;",
        "            let mut maximized = false;",
        "            let mut minimized = false;",
        "            for chunk in states.chunks_exact(4) {",
        "                match u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) {",
        "                    ACTIVATED => active = true,",
        "                    MAXIMIZED => maximized = true,",
        "                    MINIMIZED => minimized = true,",
        "                    _ => {}",
        "                }",
        "            }",
        "            toplevel.active = active;",
        "            toplevel.maximized = maximized;",
        "            toplevel.minimized = minimized;",
        "            // The compositor has said the state of this window.",
        "            toplevel.state_seen = true;",
        "        }",
        "",
        "        // The geometry of the window is the event of the version 2 of the",
        "        // protocol, and the client asks for that version or a later one.",
        "        if let CosmicEvent::Geometry {",
        "            x,",
        "            y,",
        "            width,",
        "            height,",
        "            ..",
        "        } = event",
        "        {",
        "            toplevel.geometry = Some(Geometry {",
        "                x,",
        "                y,",
        "                width,",
        "                height,",
        "            });",
        "        }",
    ],
)

# The window of the client: what it carries to the rest of the crate.
THE_WINDOW_OF_THE_CLIENT = (
    [
        "            active: self.active,",
        "        }",
        "    }",
        "}",
    ],
    [
        "            active: self.active,",
        "            maximized: self.maximized,",
        "            minimized: self.minimized,",
        "            geometry: self.geometry,",
        "        }",
        "    }",
        "}",
    ],
)

# The scale of the output: how many pixels of it are in one point.
THE_SCALE = (
    [
        "            // The scale and the transform of the output are not the",
        "            // rectangle of the monitor in the coordinates of the session.",
        "            _ => {}",
    ],
    [
        "            // The scale of the output is not the rectangle of the monitor",
        "            // in the coordinates of the session: it is how many pixels of",
        "            // the output are in one point of it.",
        "            OutputEvent::Scale { factor } => output.scale = factor as i64,",
        "            _ => {}",
    ],
)

# The monitor of the trait carries the scale.
THE_MONITOR = (
    [
        "    /// The bottom edge: one past the last row.",
        "    pub bottom: i64,",
    ],
    [
        "    /// The bottom edge: one past the last row.",
        "    pub bottom: i64,",
        "    /// The scale of the monitor: 1 is the ordinary one.",
        "    ///",
        "    /// It is how many pixels of the output are in one point of it, and",
        "    /// the compositor of the box does not always say it.",
        "    pub scale: i64,",
    ],
)


def the_block(lines, old):
    """The place of a text, as the lines of the tree."""
    for index in range(len(lines) - len(old) + 1):
        if [line.rstrip("\r") for line in lines[index:index + len(old)]] == old:
            return index
    return None


def replace(lines, old, new, what):
    at = the_block(lines, old)
    if at is None:
        raise SystemExit("the text of %s is not where it should be" % what)
    lines[at:at + len(old)] = new


def add(lines, old, new, what):
    at = the_block(lines, old)
    if at is None:
        raise SystemExit("the text of %s is not where it should be" % what)
    lines[at:at] = new


def the_function(lines, name):
    """The block of a function of the tree, with its comment."""
    start = None
    for index, line in enumerate(lines):
        if re.match(r"\s*(pub(\(\w+\))?\s+)?fn\s+%s\b" % re.escape(name), line):
            start = index
            break
    if start is None:
        return None
    opened = start
    while opened < len(lines) and "{" not in lines[opened]:
        opened += 1
    depth = 0
    begun = False
    end = None
    for index in range(opened, len(lines)):
        depth += lines[index].count("{") - lines[index].count("}")
        if "{" in lines[index]:
            begun = True
        if begun and depth <= 0:
            end = index + 1
            break
    if end is None:
        return None
    first = start
    while first > 0 and (
        lines[first - 1].strip().startswith("///")
        or lines[first - 1].strip().startswith("#[")
    ):
        first -= 1
    return (first, end)


def main():
    wayland = read_lines(WAYLAND)
    linux = read_lines(LINUX)
    trait = read_lines(TRAIT)
    box = read_lines(BOX)

    # 1. The window of the compositor says where it is.
    for old, new in THE_WINDOW:
        if the_block(wayland, new) is None:
            replace(wayland, old, new, "the window of the compositor")
    if the_block(wayland, ["pub struct Geometry {"]) is None:
        # The geometry goes at the end of the file: the module has its
        # comment at the head, and the order of the types is the order of
        # the questions.
        wayland.extend([""] + THE_GEOMETRY.split("\n"))

    # 2. The states of the window.
    for old, new in THE_STATES:
        if the_block(linux, new) is None:
            replace(linux, old, new, "the states of a window")

    # 3. The state and the geometry of the window, in the client.
    if the_block(linux, THE_STATE_OF_THE_WINDOW[1]) is None:
        replace(
            linux,
            THE_STATE_OF_THE_WINDOW[0],
            THE_STATE_OF_THE_WINDOW[1],
            "the state of a window",
        )

    # 4. What the client carries to the rest of the crate.
    if the_block(linux, THE_WINDOW_OF_THE_CLIENT[1]) is None:
        replace(
            linux,
            THE_WINDOW_OF_THE_CLIENT[0],
            THE_WINDOW_OF_THE_CLIENT[1],
            "the window of the client",
        )

    # 4b. The window of the client says where it is and in what state.
    if the_block(linux, ["    /// Whether the window is maximized."]) is None:
        replace(
            linux,
            [
                "    /// Whether the window is the active one.",
                "    active: bool,",
                "    /// Whether the compositor has said the state of the window.",
            ],
            [
                "    /// Whether the window is the active one.",
                "    active: bool,",
                "    /// Whether the window is maximized.",
                "    maximized: bool,",
                "    /// Whether the window is minimized.",
                "    minimized: bool,",
                "    /// Where the window is and how big it is, when the compositor said it.",
                "    ///",
                "    /// The `geometry` event of `zcosmic_toplevel_info_v1` carries it, and",
                "    /// the event arrives since the version 2 of that protocol -- which is",
                "    /// the version this client asks for.",
                "    geometry: Option<Geometry>,",
                "    /// Whether the compositor has said the state of the window.",
            ],
            "the window of the client",
        )

    # 4c. The geometry is the type of the module.
    if the_block(linux, ["use super::{Geometry, Window};"]) is None:
        replace(
            linux,
            ["use super::Window;"],
            ["use super::{Geometry, Window};"],
            "the geometry of the module",
        )

    # 5. The scale of the output.
    if the_block(linux, THE_SCALE[1]) is None:
        replace(linux, THE_SCALE[0], THE_SCALE[1], "the scale of the output")
    if the_block(linux, ["    /// The scale of the output: how many pixels of it are in one point."]) is None:
        replace(
            linux,
            ["    /// Whether the mode the compositor has said is the current one."],
            [
                "    /// The scale of the output: how many pixels of it are in one point.",
                "    scale: i64,",
                "    /// Whether the mode the compositor has said is the current one.",
            ],
            "the scale of the output",
        )
    if the_block(linux, ["            scale: self.scale,"]) is None:
        replace(
            linux,
            ["            bottom: self.y + self.height,"],
            ["            bottom: self.y + self.height,", "            scale: self.scale,"],
            "the monitor of the client",
        )

    # 6. The monitor of the trait carries the scale.
    if the_block(trait, THE_MONITOR[1]) is None:
        replace(trait, THE_MONITOR[0], THE_MONITOR[1], "the monitor of the trait")

    # 7. The monitors of the compositor are the ones of the client of the
    # tree of the owner: the client of the branch read its own outputs.
    found = the_function(box, "wayland_monitors")
    if found is not None and "self.wayland()?.monitors()" not in "\n".join(box[found[0]:found[1]]):
        box[found[0]:found[1]] = [
            "    /// The monitors of the compositor, by Wayland.",
            "    ///",
            "    /// The client of the compositor knows the outputs and their modes,",
            "    /// and the scale of an output is in the mode of it.",
            "    #[cfg(target_os = \"linux\")]",
            "    fn wayland_monitors(&mut self) -> Result<Vec<ahk::desktop::Monitor>, String> {",
            "        self.wayland()?.monitors()",
            "    }",
        ]

    # 8. The fake of the suite answers two monitors, and the monitor of the
    # trait carries the scale: the ordinary one.
    tests = read_lines(TESTS)
    if the_block(tests, ["            scale: 1,"]) is None:
        for index, line in enumerate(tests):
            if line.rstrip("\r").strip() == "bottom: 2160,":
                tests.insert(index + 1, "                scale: 1,")
        write_lines(TESTS, tests)

    write_lines(WAYLAND, wayland)
    write_lines(LINUX, linux)
    write_lines(TRAIT, trait)
    write_lines(BOX, box)
    print("the client of the compositor is merged")
    return 0


if __name__ == "__main__":
    sys.exit(main())


