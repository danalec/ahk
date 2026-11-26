"""The capabilities that the desktop of a Windows machine still refuses.

    python scripts/what-windows-refuses.py

The `Desktop` trait of the `ahk` crate is the sum of the capabilities
of a machine, and the desktop of a Windows machine answers the ones it
carries. The rest are the refusals of the trait: "this desktop cannot
...".

The measure is by the source: the methods of the trait that the desktop
of the Windows machine does not write. The files of the machine are
written by a macro, and the macro is read as well.

The one method that the trait answers by itself is the timers: the
timers of the script are the timers of the tree, and the desktop of a
Windows machine does not take them over. The answer of the trait is the
answer of the machine, and it is carried.
"""

import pathlib
import re

TRAIT = pathlib.Path("crates/ahk/src/desktop/mod.rs")
DESKTOP = pathlib.Path("crates/ahk-box/src/windows/desktop.rs")
FILES = pathlib.Path("crates/ahk-box/src/windows/files.rs")

source = TRAIT.read_text(encoding="utf-8")
desktop = DESKTOP.read_text(encoding="utf-8")
files = FILES.read_text(encoding="utf-8")

traits = {}
for match in re.finditer(r"pub trait (\w+)(?::[^{]+)?\s*\{", source):
    name = match.group(1)
    brace = source.index("{", match.start())
    depth = 0
    cursor = brace
    while cursor < len(source):
        if source[cursor] == "{":
            depth += 1
        elif source[cursor] == "}":
            depth -= 1
            if depth == 0:
                break
        cursor += 1
    body = source[brace:cursor]
    traits[name] = re.findall(r"(?m)^    fn ([a-z_0-9]+)\(", body)

# the methods that the desktop of the Windows machine writes, and the
# ones that the macro of the files writes
written = set(re.findall(r"(?m)^    fn ([a-z_0-9]+)\(", desktop))
written |= set(re.findall(r"(?m)^        fn ([a-z_0-9]+)\(", files))

# the methods that the trait answers by itself, and the answer of the
# trait is the answer of the machine
carried_by_the_trait = {"takes_over_timers"}

missing = []
carried = 0
for name, methods in traits.items():
    if name == "Desktop":
        continue
    for method in methods:
        if method in written or method in carried_by_the_trait:
            carried += 1
        else:
            missing.append((name, method))

total = sum(len(methods) for methods in traits.values()) - len(traits["Desktop"])

print("the capabilities of a Windows machine:")
print("  %d of %d methods are carried" % (carried, total))
print()
if missing:
    print("the ones that the desktop still refuses (%d):" % len(missing))
    for name, method in missing:
        print("  %-10s %s" % (name, method))
else:
    print("the desktop refuses none of them.")
