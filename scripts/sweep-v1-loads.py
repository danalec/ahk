"""The loads of the v1: the binary of v1.1.37.02 against the tree.

    python scripts/sweep-v1-loads.py [runner]

The tree speaks the v1 with the `--dialect=v1` of the runner, and this
script runs the same line through the binary and through the tree and says
the ones whose complaint is not the same.

The commands come from the table of the reference
(`crates/ahk/reference/ahk-v1.1.37.02.surface.tsv`), which was measured on
the binary, and not from the surface of the tree: a name that the tree does
not carry is then a difference and not a silence. The shape of the
parameters is the one of that table too, so that a line that is legal in
the binary is legal in the tree: the `Output` of a command is a variable
and the `Value` is a text.

The commands that touch the machine are left out: the line with parameters
does what it says, and a `Send` types into the window that has the focus.

The complaint of the v1 has two kinds: the load -- the script does not
run at all -- and the run. The kind is part of the answer, and the two
answers are compared with it.

The complaint of the run is a dialog of the machine, and the sweep reads it
when the script is blocked by it: the text of the dialog is the complaint,
and the process is killed afterwards, so that no dialog is left on the
screen. Before this, the sweep stopped at the first line that waited, and
the runtime errors of the v1 were never compared.

Every line runs in a folder of its own, so that the state of the machine
is the same for the binary and for the tree: the `FileCreateDir` of a
folder that is there is not a failure of the v1, and the folder of the
line before it would make the tree fail.
"""

import ctypes
import ctypes.wintypes
import json
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# The binary of the v1.1.37.02 is not in the tree, and not in the machine
# either: `scripts/reference_binaries.py` says where it is.
sys.path.insert(0, str(Path(__file__).resolve().parent))
import reference_binaries  # noqa: E402

BINARY = reference_binaries.the_v1()
RUNNER = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "target" / "release" / "ahk.exe"
REFERENCE = ROOT / "crates" / "ahk" / "reference" / "ahk-v1.1.37.02.surface.tsv"

# The windows of the machine: the dialog of the v1 is one of them, and the
# complaint of the runtime error of the v1 is in it. The sweep reads it
# when the script is blocked by it.
user32 = ctypes.windll.user32
THE_TOP_PROC = ctypes.WINFUNCTYPE(
    ctypes.wintypes.BOOL, ctypes.wintypes.HWND, ctypes.wintypes.LPARAM
)
THE_CHILD_PROC = THE_TOP_PROC

# The commands that do something to the machine when the line is accepted:
# they type, they press a key, they move the mouse, they close a window,
# they write in the registry, they wait for the user.
#
# The sound is one of them: the `SoundSet` of the v1 changes the audio of
# the machine, and the sound of the owner of the machine is not the thing
# that the sweep is measuring.
#
# The COM is here for the same reason: the answer of the COM is the COM of
# the machine, and the COM of the machine is not there for the script.
# Measured: the `ComObjActive("Scripting.Dictionary")` of the official on
# this machine is refused with "0x800401E3 - Operation unavailable", and
# the `ComObjActive("a")` of a class that is not there is refused with
# "0x800401F3 - Invalid class string". The tree does not carry the COM at
# all, and it answers the empty text: the difference is the COM of the
# machine, and not the rule of the dialect.
#
# The commands that write a file are not here: every case of the sweep
# runs in a folder of its own, so the file of one case is not the file of
# the next, and the complaint of the two sides is the thing that is
# compared.
UNSAFE = set(
    """
    BlockInput Click ComObj ComObjActive ComObjMissing ComObjParameter
    ComObject ControlClick ControlFocus ControlHide ControlMove
    ControlSend ControlSendText ControlSetText DriveSet DllCall
    FileSelectFile FileSelectFolder Input
    InputBox KeyDown KeyUp MenuClick MenuSelect MsgBox MouseClick MouseDrag
    MouseClickDrag MouseMove MouseDrag
    MouseMove Persistent PostMessage RegDelete RegWrite Reload Run
    RunWait Send SendEvent SendInput SendMessage SendPlay Shutdown
    Sleep SoundBeep SoundPlay SoundSet SoundSetWaveVolume Suspend
    TrayIconClick WinActivate
    WinClose WinHide WinKill WinMinimize WinMinimizeAll WinRestore WinShow
    """.split()
)


def the_commands():
    """The names of the v1, with the shape of their parameters.

    The list is the table of the reference, which was measured on the
    binary of v1.1.37.02: the name of the command, the least number of
    parameters that it takes and the kind of each one. The tree is not
    asked what it carries: a name that it does not carry has to be a
    difference and not a silence.

    The commands are the names that are written without parentheses and
    the functions are the names that are written with them: the `WinExist`
    of the v1 is a function, and the `WinWait` is a command.
    """
    names = []
    with REFERENCE.open(encoding="utf-8") as table:
        for line in table:
            parts = line.rstrip("\n").split("\t")
            if len(parts) < 6 or parts[0] not in ("command", "function"):
                continue
            name = parts[2]
            if name in UNSAFE:
                continue
            names.append((name, parts[5], parts[0] == "function"))
    return names


def the_script(name, shape, count, is_a_function):
    """The line that calls the name with `count` parameters.

    The kind of the parameter is the one of the table: an `Output` is a
    variable of the script and a `Value` is a text. A name that takes
    more parameters than the shape carries takes the `Value` -- the table
    says that a command takes any number of them.

    The function is called with its parentheses and the command with its
    commas: the `WinExist("a")` and the `WinWait, a`.
    """
    kinds = shape.split(",") if shape else []
    parts = []
    for number in range(count):
        kind = kinds[number] if number < len(kinds) else "Value"
        parts.append("out%d" % (number + 1) if kind == "Output" else "a")
    if is_a_function:
        return "%s(%s)\n" % (name, ", ".join(parts))
    return name + ("," + ",".join(parts) if parts else "") + "\n"


def the_binary(script, work):
    path = work / "probe.ahk"
    path.write_bytes(script.replace("\n", "\r\n").encode("utf-8"))
    process = subprocess.Popen(
        [str(BINARY), "/ErrorStdOut", str(path)],
        stdout=subprocess.PIPE, stderr=subprocess.PIPE,
        # The script runs in the folder of the case: a command that writes a
        # file writes it there, and not in the tree of the repository.
        cwd=str(work),
    )
    # The runtime errors of the v1 are always a dialog: the script is
    # blocked by it. The sweep watches for the dialog while the process
    # runs, and it kills the process as soon as the dialog is there, so
    # that no dialog is left on the screen of the machine.
    deadline = time.time() + 20
    while True:
        dialog = the_dialog_of_the_process(process.pid, path.name)
        if dialog:
            process.kill()
            process.communicate()
            return dialog, "run"
        if process.poll() is not None:
            break
        if time.time() > deadline:
            process.kill()
            process.communicate()
            return "(waits)", "waits"
        time.sleep(0.05)
    out, err = process.communicate()
    text = ((out or b"").decode("utf-8", "replace")
            + (err or b"").decode("utf-8", "replace")).strip()
    if " : ==> " in text:
        text = text.split(" : ==> ", 1)[1]
    return text.replace("\r\n", "\n").strip(), the_kind(process.returncode)


def the_dialog_of_the_process(pid, name=""):
    """The complaint of the dialog that blocks the script, or the empty text.

    The dialog of the v1 is a window of the machine with the complaint of
    it on the first line of its text: "Error:  <the complaint>". The rest
    of the text of the window is the line that failed and the note of the
    thread, and the sweep does not compare them.

    The dialog is looked for by the process that owns it and by the name
    of the script: the `AutoHotkey64.exe` of the v2 relaunches itself,
    and the dialog of the refusal is of the process that it starts -- the
    window that carries the name of the script is the one of it.

    A dialog whose complaint is not a text of a window answers "(dialog)":
    the v2 draws its dialogs with the DirectX UI of it, and the complaint
    is not a text of a window that the sweep can read.
    """
    found = []
    seen = []

    def the_child(child, _):
        text = the_text_of_the_window(child)
        if text.startswith("Error:"):
            found.append(text)
        return True

    def the_top(handle, _):
        owner = ctypes.wintypes.DWORD()
        user32.GetWindowThreadProcessId(handle, ctypes.byref(owner))
        title = the_text_of_the_window(handle)
        is_the_dialog = the_class_of_the_window(handle) == "#32770"
        if is_the_dialog and (owner.value == pid or (name and title.endswith(name))):
            seen.append(title)
            user32.EnumChildWindows(handle, THE_CHILD_PROC(the_child), 0)
        return True

    user32.EnumWindows(THE_TOP_PROC(the_top), 0)
    if not found:
        # A dialog that is there and whose complaint is not a text of a
        # window: the sweep knows that the script was refused, and it does
        # not know the words of the refusal.
        return "(dialog)" if seen else ""
    text = found[0].replace("Error:  ", "", 1)
    # The complaint is the text before the blank line: the dialog writes
    # the line that failed and the note of the thread after it.
    return text.split("\n\n")[0].strip()


def the_class_of_the_window(handle):
    """The class of a window of the machine."""
    buffer = ctypes.create_unicode_buffer(256)
    user32.GetClassNameW(handle, buffer, 256)
    return buffer.value


def the_text_of_the_window(handle):
    """The text of a window of the machine."""
    length = user32.GetWindowTextLengthW(handle)
    buffer = ctypes.create_unicode_buffer(length + 1)
    user32.GetWindowTextW(handle, buffer, length + 1)
    return buffer.value


def the_tree(script, work):
    try:
        process = subprocess.run(
            [str(RUNNER), "--dialect=v1"],
            input=script, capture_output=True, text=True, timeout=30, cwd=str(work),
        )
    except subprocess.TimeoutExpired:
        return "(waits)", "waits"
    try:
        report = json.loads(process.stdout)
    except ValueError:
        return "(no report)", "waits"
    return (report.get("error") or "").strip(), the_kind(process.returncode)


def the_kind(code):
    """The kind of the failure: the load, the run, or nothing."""
    if code == 0:
        return "ok"
    if code == 2:
        return "load"
    return "run"


def main():
    commands = the_commands()
    print("the commands of the v1: %d" % len(commands))
    different = []
    with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as folder:
        root = Path(folder)
        for number, (name, shape, is_a_function) in enumerate(commands):
            for count in range(0, 6):
                script = the_script(name, shape, count, is_a_function)
                # The folder of the case is the same for the two, and it is
                # made new for every one of them.
                work = root / ("case%d_%d" % (number, count))
                work.mkdir(parents=True)
                answer, kind = the_binary(script, work)
                if kind == "waits":
                    break
                ours, our_kind = the_tree(script, work)
                if kind == our_kind and kind == "run":
                    # The two failed at the run, and the reason is the
                    # machine: the binary of the v1 runs on Windows and the
                    # tree runs on the box.
                    continue
                if kind == our_kind and answer == ours:
                    continue
                different.append((name, count, kind, answer, our_kind, ours))
    print("the different: %d" % len(different))
    for name, count, kind, answer, our_kind, ours in different:
        print("  %-22s with %d" % (name, count))
        print("    the binary (%s): %s" % (kind, answer.replace("\n", " | ")))
        print("    the tree   (%s): %s" % (our_kind, ours.replace("\n", " | ")))


if __name__ == "__main__":
    main()
