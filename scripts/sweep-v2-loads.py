"""The runs of the v2: the binary of v2.0.32 against the tree.

    python scripts/sweep-v2-loads.py [runner] [--limit=N]

The tree speaks the v2 with the `--dialect=v2` of the runner, and this
script runs the same line through the binary and through the tree and
says the ones whose complaint is not the same.

The names come from the table of the reference
(`crates/ahk/reference/ahk-v2.0.32.surface.tsv`), which was measured on
the binary: a name that the tree does not carry is then a difference and
not a silence.

**The words of the complaint of the v2 are not readable.** The v2 draws
its dialogs with the DirectX UI of it, and the text of the complaint is
not a text of a window that the sweep can read: the reader of the sweep
of the v1 answers "(dialog)" for it, and the sweep compares the
**existence** of the refusal -- the official refused and the tree refused
too, or one of them refused and the other did not.

The binary of the v2 takes the script in a file, and it relaunches
itself: the dialog of the refusal is of the process that it starts, and
the reader of the sweep finds it by the name of the script.

The commands that touch the machine are left out: the line with
parameters does what it says, and a `Send` types into the window that has
the focus.

Every line runs in a folder of its own, so that the state of the machine
is the same for the binary and for the tree.
"""

import importlib.util
import json
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# The binary of the v2.0.32 is not in the tree, and not in the machine
# either: `scripts/reference_binaries.py` says where it is, and the copy
# that is installed in the machine is the v2.1-alpha, whose answers are not
# the ones of the v2.0.32.
sys.path.insert(0, str(Path(__file__).resolve().parent))
import reference_binaries  # noqa: E402

BINARY = reference_binaries.the_v2()
# The build that answers the errors as text needs the switch that says it,
# and the vanilla release does not know the switch.
FLAGS = reference_binaries.the_v2_flags(BINARY)
RUNNER = (
    Path(sys.argv[1])
    if len(sys.argv) > 1 and not sys.argv[1].startswith("--")
    else ROOT / "target" / "release" / "ahk.exe"
)
REFERENCE = ROOT / "crates" / "ahk" / "reference" / "ahk-v2.0.32.surface.tsv"

# The reader of the sweep of the v1 knows how to read the dialog of the
# machine, and the v2 draws the refusal in one.
_spec = importlib.util.spec_from_file_location(
    "sweep_v1", ROOT / "scripts" / "sweep-v1-loads.py"
)
_sweep_v1 = importlib.util.module_from_spec(_spec)
sys.argv = ["sweep-v1-loads.py"]
_spec.loader.exec_module(_sweep_v1)
the_dialog_of_the_process = _sweep_v1.the_dialog_of_the_process

# The names that do something to the machine when the line is accepted.
#
# The sound is one of them: the `SoundSetMute` and the `SoundSetVolume`
# of the v2 change the audio of the machine, and the sound of the owner
# of the machine is not the thing that the sweep is measuring.
UNSAFE = set(
    """
    BlockInput Click ControlClick ControlFocus ControlHide ControlMove
    ControlSend ControlSendText ControlSetText DllCall FileSelect DirSelect
    Gui Hotkey Input InputBox KeyWait MenuSelect MsgBox MouseClick MouseDrag
    MouseClickDrag MouseMove MouseDrag PostMessage Reload Run RunWait Send
    SendEvent SendInput
    SendMessage SendPlay SendText Shutdown SoundBeep SoundPlay SoundSetMute
    SoundSetVolume SoundSetWaveVolume Suspend TrayIconClick Type
    WinActivate WinClose WinHide WinKill WinMinimize WinMinimizeAll
    WinRestore WinShow
    """.split()
)


def the_limit():
    """How many names the run takes, when the script says it."""
    for argument in sys.argv:
        if argument.startswith("--limit="):
            return int(argument.split("=", 1)[1])
    return 0


def the_names():
    """The names of the v2, with the kind of each one."""
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
                int(parts[4])
                int(parts[5])
            except ValueError:
                continue
            names.append((name, parts[0] == "function"))
    return names


def the_script(name, is_a_function, count):
    """The line that calls the name with `count` parameters.

    The table of the v2 does not carry the shape of the parameters, and
    every one of them is a text here: a line that the v2 refuses because
    a parameter wants a variable is refused by the tree the same way, and
    the sweep compares the existence of the refusal.
    """
    arguments = ", ".join("a" for _ in range(count))
    if is_a_function:
        return "%s(%s)\n" % (name, arguments)
    return name + ("," + arguments if arguments else "") + "\n"


def the_binary(script, work):
    """The answer of the binary: the kind of it, and the complaint.

    The `/ErrorStdOut` of the v2 writes the refusal of the load on
    stderr, and the words of it are there. The refusal of the run is a
    dialog, and the words of it are not readable: the answer of the run
    is "(dialog)".
    """
    path = work / "probe.ahk"
    path.write_bytes(script.replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(BINARY), "/ErrorStdOut"] + FLAGS + [str(path)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        # The script runs in the folder of the case: a command that writes a
        # file writes it there, and not in the tree of the repository.
        cwd=str(work),
    )
    deadline = time.time() + 10
    while True:
        dialog = the_dialog_of_the_process(process.pid, path.name)
        if dialog:
            process.kill()
            process.communicate()
            return "run", "(dialog)"
        if process.poll() is not None:
            break
        if time.time() > deadline:
            process.kill()
            process.communicate()
            return "waits", ""
        time.sleep(0.05)
    out, err = process.communicate()
    text = (
        (out or b"").decode("utf-8", "replace")
        + (err or b"").decode("utf-8", "replace")
    ).strip()
    if " : ==> " in text:
        text = text.split(" : ==> ", 1)[1]
    if not text:
        return "ok", ""
    return ("load" if process.returncode == 2 else "run"), text.replace("\r\n", "\n").strip()


def the_tree(script, work):
    """The answer of the tree: the kind of it, and the complaint."""
    try:
        process = subprocess.run(
            [str(RUNNER), "--dialect=v2"],
            input=script,
            capture_output=True,
            text=True,
            timeout=15,
            cwd=str(work),
        )
    except subprocess.TimeoutExpired:
        return "waits", ""
    try:
        report = json.loads(process.stdout)
    except ValueError:
        return "waits", ""
    error = (report.get("error") or "").strip()
    if not error:
        return "ok", ""
    return ("load" if process.returncode == 2 else "run"), error.replace("\r\n", "\n")


def main():
    names = the_names()
    limit = the_limit()
    if limit:
        names = names[:limit]
    print("the names of the v2: %d" % len(names))
    different = []
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        root = Path(folder)
        for number, (name, is_a_function) in enumerate(names):
            for count in range(0, 4):
                script = the_script(name, is_a_function, count)
                work = root / ("case%d_%d" % (number, count))
                work.mkdir(parents=True)
                answer, words = the_binary(script, work)
                if answer == "waits":
                    break
                our_answer, our_words = the_tree(script, work)
                if answer != our_answer:
                    different.append((name, count, answer, words, our_answer, our_words))
                    continue
                # The words of the refusal of the run of the v2 are not
                # readable: the sweep compares the existence of it.
                if words and words != "(dialog)" and our_words and words != our_words:
                    different.append((name, count, answer, words, our_answer, our_words))
    print("the different: %d" % len(different))
    for name, count, answer, words, our_answer, our_words in different:
        print("  %-22s with %d" % (name, count))
        print("    the binary (%s): %s" % (answer, words.replace("\n", " | ")))
        print("    the tree   (%s): %s" % (our_answer, our_words.replace("\n", " | ")))


if __name__ == "__main__":
    main()
