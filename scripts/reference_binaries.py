"""Where the reference binaries are.

The tree measures itself against the official interpreters: the v2.0.32 and
the v1.1.37.02. The binaries are not in the tree, and they are not in the
machine either -- the temporary folder of the machine is cleaned, and the
copy that is installed in `C:\\Program Files\\AutoHotkey\\v2` is the
**v2.1-alpha**, whose answers are not the ones of the v2.0.32.

The variables `AHK_V2` and `AHK_V1` say where the binaries are, and the
places that the machine keeps them are the fallback:

| The binary | Where it is | Where it comes from |
| --- | --- | --- |
| the v2.0.32 | `../ahk-v2-headless/bin/AutoHotkey64.exe` | the vanilla release with the patch that answers the errors as text |
| the v2.0.32 | `%TEMP%\\ahk-v2-measure-2.0.32\\AutoHotkey64.exe` | the vanilla release, unpacked |
| the v1.1.37.02 | `%TEMP%\\ahk_init\\v1\\bin\\AutoHotkeyU64.exe` | the zip of the release, unpacked |
| the v1.1.37.02 | `C:\\Program Files\\AutoHotkey\\v1.1.37.02\\AutoHotkeyU64.exe` | the copy that is installed |

The v2.0.32 of the upstream is published with the SHA256 of its files, and
the digests are the ones of the page of the release: the v1.1.37.02 is
`6F3663F7CDD25063C8C8728F5D9B07813CED8780522FD1F124BA539E2854215F`
(`AutoHotkey_1.1.37.02.zip`).
"""

import os
from pathlib import Path

TEMP = Path(os.environ["TEMP"])

# The v2.0.32: the build that answers the errors as text, and the copy that
# was unpacked. The copy that is installed in the machine is not one of them
# -- it is the v2.1-alpha, and a measurement against it is a measurement
# against another dialect.
V2_HEADLESS = Path(
    r"C:\Users\danalec\Documents\src\ahk-v2-headless\bin\AutoHotkey64.exe"
)
V2_MEASURED = TEMP / "ahk-v2-measure-2.0.32" / "AutoHotkey64.exe"

# The v1.1.37.02.
V1_UNPACKED = TEMP / "ahk_init" / "v1" / "bin" / "AutoHotkeyU64.exe"
V1_INSTALLED = Path(r"C:\Program Files\AutoHotkey\v1.1.37.02\AutoHotkeyU64.exe")


def the_v2():
    """The binary of the v2.0.32 that the measurements use."""
    said = os.environ.get("AHK_V2")
    if said:
        return Path(said)
    for candidate in (V2_HEADLESS, V2_MEASURED):
        if candidate.exists():
            return candidate
    return V2_HEADLESS


def the_v2_flags(binary=None):
    """The switches of the binary of the v2.

    The build that answers the errors as text needs the switch that says
    it, and the vanilla release does not know the switch.
    """
    said = os.environ.get("AHK_V2_FLAGS")
    if said is not None:
        return said.split()
    where = str(binary if binary is not None else the_v2()).lower()
    return ["--headless"] if "headless" in where else []


def the_v1():
    """The binary of the v1.1.37.02 that the measurements use."""
    said = os.environ.get("AHK_V1")
    if said:
        return Path(said)
    for candidate in (V1_UNPACKED, V1_INSTALLED):
        if candidate.exists():
            return candidate
    return V1_UNPACKED
