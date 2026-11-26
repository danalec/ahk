"""The real arity of the names of the v2, measured on the binary.

    python scripts/measure-arity-2.0.32.py [--names a,b,c] [--out FILE]

The table of the reference carries the `min` of each name, and the `min`
comes out of the table of the binary: it says how many parameters the
built-in wants, and it is zero for the commands of the v2, whose table
does not carry the count.

**The binary does not always agree with its own table.** Measured on the
v2.0.32: the `MouseClickDrag` is refused when the script loads with "Too
few parameters passed to function. / Specifically: MouseClickDrag", and
the table of the binary says that it takes none. The refusal of the load
is on stderr, and `/ErrorStdOut` writes it there: the arity of a name is
the smallest number of parameters whose call is not refused with those
words.

The names that touch the machine are left out: the call of them does what
it says, and a `Send` types into the window that has the focus.

The report is a table with the name, the kind, the arity that the binary
states and the one that it keeps. It is a report and not a change: the
table of the reference is changed by hand, with the measurement in it.
"""

import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BINARY = Path(r"C:\Program Files\AutoHotkey\v2\AutoHotkey64.exe")
REFERENCE = ROOT / "crates" / "ahk" / "reference" / "ahk-v2.0.32.surface.tsv"

# The names that do something to the machine when the line is accepted.
UNSAFE = set(
    """
    BlockInput Click ControlClick ControlFocus ControlHide ControlMove
    ControlSend ControlSendText ControlSetText DllCall FileSelect DirSelect
    Exit ExitApp Gui Hotkey Input InputBox KeyWait MenuSelect MsgBox
    MouseClick MouseClickDrag MouseDrag MouseMove PostMessage Reload Run
    RunWait Send SendEvent SendInput SendMessage SendPlay SendText Shutdown
    SoundBeep SoundPlay SoundSetMute SoundSetVolume SoundSetWaveVolume
    Suspend TrayIconClick Type WinActivate WinClose WinHide WinKill
    WinMinimize WinMinimizeAll WinRestore WinShow
    """.split()
)

# The words of the refusal of the arity, in the binary of the v2.0.32.
TOO_FEW = "Too few parameters passed to function."


def the_names():
    """The names of the table, with the kind of each one."""
    names = []
    with REFERENCE.open(encoding="utf-8") as table:
        for line in table:
            parts = line.rstrip("\n").split("\t")
            if len(parts) < 6 or parts[0] not in ("command", "function"):
                continue
            name = parts[2]
            if name in UNSAFE:
                continue
            try:
                fewest = int(parts[4])
                most = int(parts[5])
            except ValueError:
                continue
            names.append((name, parts[0] == "function", fewest, most))
    return names


def the_script(name, is_a_function, count):
    """The line that calls the name with `count` parameters."""
    arguments = ", ".join("1" for _ in range(count))
    if is_a_function:
        return "%s(%s)\n" % (name, arguments)
    return name + ("," + arguments if arguments else "") + "\n"


# The words of the refusal of a call that is written with commas and is the
# call of a function, in the binary of the v2.0.32.
NOT_A_COMMAND = "Function calls require a space"


def the_kind_of_the_binary(name, is_a_function, work):
    """Whether the binary has the name as a command or as a function.

    Measured on the binary of the v2.0.32: the `MouseClickDrag` is a
    function, and the table of the reference says that it is a command. A
    command is written with commas and a function with parentheses, and the
    binary refuses the commas of a function with "Function calls require a
    space or "(".
    """
    if is_a_function:
        return True
    refusal = the_refusal_of_the_load(the_script(name, False, 0), work)
    return NOT_A_COMMAND in refusal


def the_refusal_of_the_load(script, work):
    """The refusal of the load of the binary, or the empty text.

    The refusal of the run is a dialog of the machine, and it is not
    readable: the measure asks for the refusal of the *load*, and a name
    whose arity is fine is not refused with the words of the arity.
    """
    path = work / "probe.ahk"
    path.write_bytes(script.replace("\n", "\r\n").encode("utf-8"))
    try:
        process = subprocess.run(
            [str(BINARY), "/ErrorStdOut", str(path)],
            capture_output=True,
            text=True,
            timeout=8,
            cwd=str(work),
        )
    except subprocess.TimeoutExpired:
        # A dialog of the machine: the refusal of the run.
        return ""
    text = ((process.stdout or "") + (process.stderr or "")).strip()
    if " : ==> " in text:
        text = text.split(" : ==> ", 1)[1]
    return text


def the_arity(name, is_a_function, most, work):
    """The smallest number of parameters whose call is not refused.

    The words of the refusal of the arity are the ones of the load, and a
    refusal of another kind -- a parameter that wants a number and got a
    text -- says that the arity is fine.
    """
    for count in range(0, most + 1):
        script = the_script(name, is_a_function, count)
        refusal = the_refusal_of_the_load(script, work)
        if TOO_FEW not in refusal:
            return count
    return most


def main():
    wanted = []
    out = None
    for argument in sys.argv[1:]:
        if argument.startswith("--names="):
            wanted = [n for n in argument.split("=", 1)[1].split(",") if n]
        elif argument.startswith("--out="):
            out = ROOT / argument.split("=", 1)[1]
    names = the_names()
    if wanted:
        names = [n for n in names if n[0] in wanted]
    print("the names of the v2: %d" % len(names))
    rows = []
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        work = Path(folder)
        for number, (name, is_a_function, fewest, most) in enumerate(names):
            kind_is_a_function = the_kind_of_the_binary(name, is_a_function, work)
            arity = the_arity(name, kind_is_a_function, most, work)
            rows.append(
                (
                    name,
                    "function" if kind_is_a_function else "command",
                    "function" if is_a_function else "command",
                    fewest,
                    arity,
                )
            )
            if arity != fewest or kind_is_a_function != is_a_function:
                print(
                    "  %-22s the table says %s %d, the binary wants %s %d"
                    % (
                        name,
                        "function" if is_a_function else "command",
                        fewest,
                        "function" if kind_is_a_function else "command",
                        arity,
                    )
                )
    print(
        "the names that the binary and the table do not agree on: %d"
        % len([r for r in rows if r[3] != r[4] or r[2] != r[1]])
    )
    if out:
        with out.open("w", encoding="utf-8", newline="\n") as handle:
            handle.write("name\tkind of the table\tkind of the binary\tthe table\tthe binary\n")
            for name, kind, table_kind, fewest, arity in rows:
                handle.write(
                    "%s\t%s\t%s\t%d\t%d\n" % (name, kind, table_kind, fewest, arity)
                )
        print("written: %s" % out)


if __name__ == "__main__":
    main()
