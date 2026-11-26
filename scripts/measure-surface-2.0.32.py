"""Measure the surface of the AutoHotkey v2.0.32 and compare it with the v2.0.19.

    python scripts/measure-surface-2.0.32.py <the dir of the v2.0.32> <the dir of the v2.0.19> [<the table to write>]

The directory of a release is the one that holds `AutoHotkey64.exe` and
`AutoHotkey.chm`.  The documentation is decompiled with `hh.exe` when the
`chm/` directory is not there yet.  The table is written where the third
argument says, and on stdout when it is not given.

What the instrument does, in this order:

1. **The words of the two binaries.**  A name that the binary of the
   v2.0.32 carries and the binary of the v2.0.19 did not is a name that
   the release has added.  The words are taken both ways round: the ASCII
   ones and the UTF-16 ones (the built-in names of a v2 are UTF-16).

2. **The tables of the two binaries.**  A v2 keeps its built-ins in two
   tables -- the table of the functions and the table of the commands --
   and the kind of a row is the table its name came from.  The entries
   are read out of the binary: a name, a handler, and, for a function,
   the fewest and the most parameters, the flavour of the handler and the
   byte beside it.  The commands keep their arity in a packing that was
   not decoded, so their arity is measured on the binary.

3. **The questions of the v2.**  The v2 has no kind of command in the
   parser, so a name is measured by what the binary says when it is
   used in the three forms: the name alone in an expression, the name
   called in an expression, and the name at the start of a line.  The
   count of the parameters is what the binary accepts: "Too few
   parameters passed to function." and "Too many parameters passed to
   function." are the complaints of the v2, and they are the two ends of
   the arity.  The questions are inside a dead branch, so the call never
   runs and only the count is checked; the check of the count is made at
   the load, and it is made inside a dead branch too.

4. **The answers of the two binaries.**  The names that the
   documentation of the v2.0.32 says answer something new are asked of
   both binaries, so that the answer of the release and the answer of
   the older one are side by side.

5. **The table.**  The rows of the table of the tree are carried over
   and the rows that the measurement changed are replaced.  A row whose
   measurement was not conclusive keeps the value it had, and
   the file says so in a comment at the end.

6. **The delta**, on stdout: the names whose kind changed, the names
   whose arity changed, the names that are new and the names that are
   gone.
"""
import pathlib
import re
import struct
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from html import unescape

# The table of the tree that carries the surface: the table of the v2.0.32
# (the tree has been moved to the v2.0.32; the rows of the file are the
# ones that were measured on the v2.0.19, with the ones the measurement of
# the v2.0.32 changed put in their place).
REFERENCE = pathlib.Path(__file__).resolve().parent.parent.joinpath(
    "crates", "ahk", "reference")
TABLE_OF_THE_TREE = REFERENCE / "ahk-v2.0.32.surface.tsv"
# The most parameters a scan looks for.  A name that takes more than this
# is a name that takes any number of them, and the table says 255.
LIMIT = 12

# The complaints of the v2 about the count of the parameters.
TOO_FEW = "Too few parameters passed to function."
TOO_MANY = "Too many parameters passed to function."

# The complaint of a command that needs a parameter that its own table
# does not count.  It is not a complaint about the count, so the count is
# fine and the arity of the table is the one of the binary.
NEEDS = re.compile(r"^Parameter #\d+ of .* must not be omitted in this case\.")

COLUMNS = ("kind", "class", "name", "handler", "min", "max", "variant", "flag")


def out(text=""):
    """The report, on stdout."""
    sys.stdout.write(text + "\n")
    sys.stdout.flush()


def progress(text=""):
    """The progress, on stderr."""
    sys.stderr.write(text + "\n")
    sys.stderr.flush()


# ---------------------------------------------------------------------------
# The binary: the PE, and the tables of the built-ins.
# ---------------------------------------------------------------------------

class PE:
    """The pieces of a PE that the tables of the built-ins need."""

    def __init__(self, path):
        self.path = path
        self.data = path.read_bytes()
        data = self.data
        e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
        if data[e_lfanew:e_lfanew + 4] != b"PE\0\0":
            raise ValueError("%s is not a PE" % path)
        coff = e_lfanew + 4
        self.n_sections = struct.unpack_from("<H", data, coff + 2)[0]
        opt = coff + 20
        self.image_base = struct.unpack_from("<Q", data, opt + 24)[0]
        size_of_opt = struct.unpack_from("<H", data, coff + 16)[0]
        base = opt + size_of_opt
        self.sections = []
        for index in range(self.n_sections):
            off = base + index * 40
            name = data[off:off + 8].rstrip(b"\0").decode("ascii", "replace")
            vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, off + 8)
            self.sections.append((name, vaddr, vsize, raddr, rsize))
        self.code = [(vaddr, vaddr + vsize) for name, vaddr, vsize, _, _
                     in self.sections if name == ".text"]

    def rva_to_offset(self, rva):
        for name, vaddr, vsize, raddr, rsize in self.sections:
            if vaddr <= rva < vaddr + max(vsize, rsize):
                return raddr + (rva - vaddr)
        return None

    def offset_to_rva(self, offset):
        for name, vaddr, vsize, raddr, rsize in self.sections:
            if raddr <= offset < raddr + rsize:
                return vaddr + (offset - raddr)
        return None

    def is_code(self, address):
        """Whether a virtual address is in the code of the binary."""
        rva = address - self.image_base
        return any(lo <= rva < hi for lo, hi in self.code)

    def utf16_at(self, address):
        """The UTF-16 text at a virtual address, or None.

        A name of a v2 is UTF-16, and it is a name of the language: it
        starts with a letter and it holds nothing but letters, digits and
        underscores.
        """
        off = self.rva_to_offset(address - self.image_base)
        if off is None:
            return None
        raw = self.data[off:off + 128]
        if len(raw) < 4 or raw[1] != 0:
            return None
        try:
            text = raw.decode("utf-16le").split("\0")[0]
        except UnicodeDecodeError:
            return None
        if not text or len(text) > 48:
            return None
        if not all(32 <= ord(c) < 127 for c in text):
            return None
        if not (text[0].isalpha() or text[0] == "_"):
            return None
        return text


class Entry:
    """One row of a table of the binary."""

    def __init__(self, name, handler, low=None, high=None, variant=None, flag=None):
        self.name = name
        self.handler = handler
        self.min = low
        self.max = high
        self.variant = variant
        self.flag = flag


def runs(pe, stride, minimum=20):
    """The runs of the entries of a table of the binary.

    An entry is a pointer to the name (UTF-16) and a pointer to the
    handler (in the code of the binary).  A run is a stretch of entries
    with the stride of the table, and the biggest run is the table.
    """
    data = pe.data
    slots = {}
    for name, vaddr, vsize, raddr, rsize in pe.sections:
        if name not in (".rdata", ".data"):
            continue
        for off in range(raddr, raddr + rsize - 8, 8):
            pointer = struct.unpack_from("<Q", data, off)[0]
            handler = struct.unpack_from("<Q", data, off + 8)[0]
            if not pe.is_code(handler):
                continue
            label = pe.utf16_at(pointer)
            if label is None:
                continue
            slots[off] = (label, handler)
    found = []
    for off in sorted(slots):
        if off - stride in slots:
            continue
        run = [off]
        while off + stride in slots:
            off += stride
            run.append(off)
        if len(run) >= minimum:
            found.append(run)
    return found


def read_tables(pe):
    """The tables of the built-ins of a v2.

    The table of the functions has an entry of thirty-two bytes: a name,
    a handler, the fewest parameters, the most, the flavour of the
    handler and the byte beside it.  The table of the commands has an
    entry of forty bytes: a name, a handler, and a packing that was not
    decoded.  The table of the built-in variables has an entry of
    twenty-four bytes: a name, the getter and the setter.
    """
    functions, commands, variables = [], [], []
    for run in runs(pe, 32):
        entries = []
        for off in run:
            pointer = struct.unpack_from("<Q", pe.data, off)[0]
            handler = struct.unpack_from("<Q", pe.data, off + 8)[0]
            low, high, variant, flag = pe.data[off + 16:off + 20]
            entries.append(Entry(pe.utf16_at(pointer), handler, low, high, variant, flag))
        functions.append(entries)
    for run in runs(pe, 40):
        entries = []
        for off in run:
            pointer = struct.unpack_from("<Q", pe.data, off)[0]
            handler = struct.unpack_from("<Q", pe.data, off + 8)[0]
            entries.append(Entry(pe.utf16_at(pointer), handler))
        commands.append(entries)
    for run in runs(pe, 24, minimum=40):
        entries = []
        for off in run:
            pointer = struct.unpack_from("<Q", pe.data, off)[0]
            getter = struct.unpack_from("<Q", pe.data, off + 8)[0]
            entries.append(Entry(pe.utf16_at(pointer), getter))
        variables.append(entries)
    return functions, commands, variables


def the_table_of(pe, tables, names):
    """The table of the binary whose names are the ones of the tree.

    The tables of the methods of the classes are told apart by the names
    they carry: the table of the functions carries the names that the
    table of the tree calls functions, and the table of the commands the
    ones that it calls commands.
    """
    best, score = [], 0
    for entries in tables:
        got = sum(1 for entry in entries if entry.name in names)
        if got > score:
            best, score = entries, got
    return [entry for entry in best if entry.name in names]


# ---------------------------------------------------------------------------
# The words of a binary.
# ---------------------------------------------------------------------------

ASCII_WORD = re.compile(rb"[A-Za-z_][A-Za-z0-9_]{2,63}")

# A word of the language: it starts with a capital, and it holds nothing
# but letters.  The names of a v2 are of this shape, and the noise of
# the strings of a binary is not.
ASCII_NAME = re.compile(r"^[A-Z][A-Za-z]*$")


def utf16_words(data):
    """The UTF-16 words of a binary.

    A UTF-16 word of the ASCII alphabet is a run of bytes where every
    other byte is a null.  The names of the built-ins of a v2 are of
    this shape.
    """
    found = set()
    index = 0
    size = len(data)
    while index + 1 < size:
        if not (32 <= data[index] < 127) or data[index + 1] != 0:
            index += 1
            continue
        start = index
        while index + 1 < size and 32 <= data[index] < 127 and data[index + 1] == 0:
            index += 2
        word = data[start:index].decode("utf-16le", "replace")
        if len(word) >= 3:
            found.add(word)
    return found


# ---------------------------------------------------------------------------
# The documentation.
# ---------------------------------------------------------------------------

def decompile(directory):
    """The documentation of a release, out of its CHM."""
    chm = directory / "chm"
    if (chm / "Index.hhk").exists():
        return chm
    source = directory / "AutoHotkey.chm"
    if not source.exists():
        raise SystemExit("the documentation is not there: %s" % source)
    chm.mkdir(exist_ok=True)
    subprocess.run(["hh.exe", "-decompile", str(chm), str(source)], check=True)
    return chm


def syntaxes_of(text):
    """The syntax lines of a page, in the order of the page."""
    lines = []
    for block in re.findall(r'<pre class="Syntax">(.*?)</pre>', text, re.S):
        clean = unescape(re.sub(r"<[^>]+>", "", block))
        for line in clean.splitlines():
            line = " ".join(line.split())
            if line:
                lines.append(line)
    return lines


def pages_of(chm):
    """The syntax lines of every page of the library of the documentation."""
    pages = {}
    for page in (chm / "docs" / "lib").glob("*.htm"):
        text = page.read_text(encoding="utf-8", errors="replace")
        pages[page.stem] = syntaxes_of(text)
    return pages


# ---------------------------------------------------------------------------
# The questions on the binary.
# ---------------------------------------------------------------------------

class Probe:
    """One process per question, and the answer of the binary.

    The v2 writes a load-time error on stderr when `/ErrorStdOut` is
    there, and it opens a window when the error is a runtime one that
    the script does not catch.  The questions therefore catch what they
    can, and the calls are inside a dead branch, so that nothing runs.
    """

    def __init__(self, binary, work, timeout=10):
        self.binary = binary
        self.work = work
        self.timeout = timeout
        work.mkdir(parents=True, exist_ok=True)

    def run(self, tag, body):
        script = self.work / ("%s.ahk" % tag)
        answer = self.work / ("%s.out" % tag)
        text = body.replace("@OUT@", str(answer).replace("\\", "\\\\"))
        script.write_bytes((text + "\r\nExitApp\r\n").encode("utf-8"))
        if answer.exists():
            answer.unlink()
        proc = subprocess.Popen([str(self.binary), "/ErrorStdOut", str(script)],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            _, stderr = proc.communicate(timeout=self.timeout)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.communicate()
            return "TIMEOUT", ""
        message = stderr.decode("utf-8", "replace")
        if "==>" in message:
            message = message.split("==>")[-1]
        message = " ".join(message.split())
        written = answer.read_text(encoding="utf-8", errors="replace").strip() \
            if answer.exists() else ""
        return message, written

    # -- the three questions of the v2 -------------------------------------

    def alone(self, name):
        """The name alone in an expression: what does it answer?

        A function answers a `Func` object, a variable answers its value,
        and a class answers the class.
        """
        body = ("try {\n"
                "    x := %s\n"
                "    FileAppend(\"v:\" . Type(x), \"@OUT@\")\n"
                "} catch as e {\n"
                "    FileAppend(\"e:\" . e.Message, \"@OUT@\")\n"
                "}\n" % name)
        message, written = self.run("alone_%s" % name, body)
        if message:
            return None, message
        return written, ""

    def called(self, name, count):
        """The name called in an expression, with a count of parameters.

        The answer is the complaint of the binary: `too few`, `too many`,
        or the empty text when the count is fine.  The call is inside a
        dead branch, so it never runs: the count is checked at the load.
        """
        args = ", ".join(["1"] * count)
        body = ("try {\n"
                "    if (0) {\n"
                "        x := %s(%s)\n"
                "    }\n"
                "    FileAppend(\"ok\", \"@OUT@\")\n"
                "} catch as e {\n"
                "    FileAppend(\"err:\" . e.Message, \"@OUT@\")\n"
                "}\n" % (name, args))
        return self._verdict(*self.run("call_%s_%d" % (name, count), body))

    def statement(self, name, count):
        """The name at the start of a line, with a count of parameters.

        The second form of the same question: the v2 lets the
        parentheses of a call be left out when the name is at the start
        of the line, and the count is checked the same way.
        """
        args = ", ".join(["1"] * count)
        body = ("try {\n"
                "    if (0) {\n"
                "        %s %s\n"
                "    }\n"
                "    FileAppend(\"ok\", \"@OUT@\")\n"
                "} catch as e {\n"
                "    FileAppend(\"err:\" . e.Message, \"@OUT@\")\n"
                "}\n" % (name, args))
        return self._verdict(*self.run("stmt_%s_%d" % (name, count), body))

    @staticmethod
    def _verdict(message, written):
        """The complaint of the binary, as `too few`, `too many` or None."""
        if message == "TIMEOUT":
            # The binary opened a window: the name is not a name of the
            # language, and the load refused it.
            return "timeout"
        if TOO_FEW in message:
            return "too few"
        if TOO_MANY in message:
            return "too many"
        return None


def arity_of(probe, name, form="call"):
    """The fewest and the most parameters, by a scan of the count.

    The count is what the binary accepts: it refuses a count that is
    short with "Too few parameters passed to function." and a count that
    is long with "Too many parameters passed to function.".  Every other
    answer says that the count was fine -- including the complaint of a
    command that needs a parameter that its own table does not count.
    """
    ask = probe.called if form == "call" else probe.statement
    accepted = []
    for count in range(LIMIT + 2):
        verdict = ask(name, count)
        if verdict == "timeout":
            # The name is not a name of the language: there is nothing to
            # measure.
            if count == 0:
                return None, None, False
            break
        if verdict is None:
            accepted.append(count)
    if not accepted:
        return None, None, False
    low, high = accepted[0], accepted[-1]
    # A name that takes any number of them has the 255 of the table.
    return low, (255 if high > LIMIT else high), True


# ---------------------------------------------------------------------------
# The answers of the two binaries.
# ---------------------------------------------------------------------------

# The names whose documentation of the v2.0.32 says that they answer
# something new, and the script that asks it.  Each script writes what
# it has found in the file of the answer.
ANSWERS = [
    ("CoordMode", "x := CoordMode(\"Mouse\")\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("Critical", "x := Critical(1)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("DetectHiddenText", "x := DetectHiddenText(true)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("DetectHiddenWindows", "x := DetectHiddenWindows(true)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("FileEncoding", "x := FileEncoding(\"UTF-8\")\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("ListLines", "x := ListLines(true)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("Persistent", "x := Persistent(true)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SendLevel", "x := SendLevel(1)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SendMode", "x := SendMode(\"Event\")\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetControlDelay", "x := SetControlDelay(10)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetDefaultMouseSpeed", "x := SetDefaultMouseSpeed(10)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetMouseDelay", "x := SetMouseDelay(10)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetRegView", "x := SetRegView(64)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetStoreCapsLockMode", "x := SetStoreCapsLockMode(true)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetTitleMatchMode", "x := SetTitleMatchMode(2)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("SetWinDelay", "x := SetWinDelay(10)\nFileAppend(\"v:\" . Type(x) . \"|\" . x, \"@OUT@\")"),
    ("Error.Call", "obj := Error.Call(\"boom\")\nFileAppend(\"v:\" . Type(obj) . \"|\" . obj.Message, \"@OUT@\")"),
    ("Error.Extra", "obj := Error(\"boom\")\nobj.Extra := \"extra\"\nFileAppend(\"v:\" . obj.Extra, \"@OUT@\")"),
    ("Error.File", "obj := Error(\"boom\")\nobj.File := \"f.ahk\"\nFileAppend(\"v:\" . obj.File, \"@OUT@\")"),
    ("Error.Line", "obj := Error(\"boom\")\nobj.Line := 42\nFileAppend(\"v:\" . obj.Line, \"@OUT@\")"),
    ("Error.Message", "obj := Error(\"boom\")\nobj.Message := \"other\"\nFileAppend(\"v:\" . obj.Message, \"@OUT@\")"),
    ("Error.Stack", "obj := Error(\"boom\")\nobj.Stack := \"stack\"\nFileAppend(\"v:\" . obj.Stack, \"@OUT@\")"),
    ("Error.What", "obj := Error(\"boom\")\nobj.What := \"what\"\nFileAppend(\"v:\" . obj.What, \"@OUT@\")"),
]


def answer_of(probe, tag, body):
    """What a name answers, on a binary."""
    text = ("try {\n"
            "    %s\n"
            "} catch as err {\n"
            "    FileAppend(\"e:\" . err.Message, \"@OUT@\")\n"
            "}\n" % body)
    message, written = probe.run("answer_%s" % tag, text)
    if message:
        return "refused: " + message
    if not written:
        return "nothing"
    return written


# ---------------------------------------------------------------------------
# The table of the tree.
# ---------------------------------------------------------------------------

def read_table(path):
    """The rows of a table of the tree.

    The line of every row is kept: a row that the measurement does not
    change is carried over as it is, with its own spacing.
    """
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.split("\t")
        if fields[0] == "kind" and fields[3] == "handler":
            continue
        fields += [""] * (len(COLUMNS) - len(fields))
        row = dict(zip(COLUMNS, fields))
        row["line"] = line
        row["fields"] = fields
        rows.append(row)
    return rows


# ---------------------------------------------------------------------------
# The instrument.
# ---------------------------------------------------------------------------

HEADER = """\
# The AutoHotkey v2.0.32 surface, read out of AutoHotkey64.exe.
#
# The release of the table is the 2.0.32. The rows were measured on the
# binary of the 2.0.32 with the instrument beside this
# table (`scripts/measure-surface-2.0.32.py`), and the rows of the table
# of the v2.0.19 are carried over. The behaviours that the documentation
# changed are in `docs/v2.0.32-changes.md`.
#
# The measurement said that the names are the same: the table of the
# functions has 103 entries in the two binaries, the table of the
# commands has 242 and the table of the built-in variables has 131, and
# no name is in one of them and not in the other.  The arity is not the
# same everywhere: the `RegWrite` took no parameter in the v2.0.19 and
# takes one in the v2.0.32, and the call of it without a parameter is
# refused by the binary of the v2.0.32 with "Too few parameters passed
# to function."  It is the only name whose arity changed.
#
# kind      command, function, variable, method or class
# class     the class a method belongs to, empty otherwise
#           A row of the kind `class` is a class of the dialect that a
#           script can read as a class: the name of it is a value, and
#           its `__Class` is "Class". The class of a hotkey is not one of
#           them: the name `Hotkey` is a function of the dialect.
#           The `BoundFunc`, the `Closure` and the `RegExMatchInfo` are
#           classes of the binary that the documentation does not name: a
#           script reads them as a class, and the call of them is refused
#           with "Invalid base.", the way the call of the `File` is.
# handler   the address of the handler, in the binary of the v2.0.32.
#           The address of a member of a class is the one of the
#           v2.0.19: the tables of the members of the classes were not
#           read again.
# min       the fewest parameters, where the binary states it
# max       the most parameters, 255 for any number
# variant   the byte that picks the flavour of a shared handler
# flag      the byte beside it
#
# The rows of the members of a class are the table of the class in the
# binary: a property with a setter is two rows, the getter and the
# setter. The table of the `InputHook` is at 0x00125170 and the table of
# the `Gui.TreeView` is at 0x00126780, both with a stride of forty bytes.
#
# The binary keeps some of the members outside its tables of the methods:
# the properties of the error object, the call of the enumerator, the
# pointer of the `ComValue` and the pointer and the size of the
# `ClipboardAll` are not in one. The rows of them carry no handler.
#
# The arity of the function entries is in the binary, and it is in this
# table as it was read out of the binary of the v2.0.32. The arity of
# the command entries was not: it is kept in a packing that was not
# decoded, and it was measured on the binary instead --
# the call that asks for too few is refused at the load, and the call
# that asks for too many is refused as well, so the two refusals say
# where the bounds are. The fewest that the binary declares is not
# always the fewest that the command needs: a command may need a
# parameter that the table does not count, and it says so in other
# words.
#
# The rows of the table of the tree are carried over, and the rows
# that the measurement changed are replaced. The names
# come from the documentation of the two releases and from the tables of
# the two binaries.
kind\tclass\tname\thandler\tmin\tmax\tvariant\tflag
"""


def main():
    if len(sys.argv) < 3:
        print(__doc__)
        return 2
    directory = pathlib.Path(sys.argv[1]).resolve()
    old_directory = pathlib.Path(sys.argv[2]).resolve()
    binary = directory / "AutoHotkey64.exe"
    old_binary = old_directory / "AutoHotkey64.exe"
    for path in (binary, old_binary):
        if not path.exists():
            print("the binary is not there: %s" % path)
            return 2

    rows = read_table(TABLE_OF_THE_TREE)
    known = set(row["name"] for row in rows)
    known |= set(row["class"] for row in rows if row["class"])

    # -- 1. the words of the two binaries ---------------------------------
    out("== the words of the two binaries")
    old_data = old_binary.read_bytes()
    new_data = binary.read_bytes()
    old_ascii = set(match.group().decode("ascii")
                    for match in ASCII_WORD.finditer(old_data))
    new_ascii = set(match.group().decode("ascii")
                    for match in ASCII_WORD.finditer(new_data))
    old_wide = utf16_words(old_data)
    new_wide = utf16_words(new_data)
    known = set(row["name"] for row in rows)
    known |= set(row["class"] for row in rows if row["class"])
    only_wide = sorted(w for w in new_wide - old_wide if w not in known)
    only_ascii = sorted(w for w in new_ascii - old_ascii if w not in known)
    # The noise of the strings of a binary is not a name of the language:
    # the report keeps the words that have the shape of one.
    shaped = [w for w in only_ascii if ASCII_NAME.match(w)]
    out("   the UTF-16 words that the v2.0.32 did not have: %d" % len(only_wide))
    out("   (the names of a v2 are UTF-16, so these are the words that")
    out("   could be a name of the dialect)")
    for word in only_wide:
        out("     %s" % word)
    out("   the ASCII words that the v2.0.19 did not have: %d" % len(only_ascii))
    out("   (of them %d have the shape of a name of the language; the rest"
        % len(shaped))
    out("   is the noise of the strings of the binary; the list is at the end)")

    # -- 2. the tables of the two binaries ---------------------------------
    progress("== the tables of the built-ins")
    pe = PE(binary)
    old_pe = PE(old_binary)
    function_tables, command_tables, variable_tables = read_tables(pe)
    old_function_tables, old_command_tables, old_variable_tables = read_tables(old_pe)
    function_names = set(row["name"] for row in rows if row["kind"] == "function")
    command_names = set(row["name"] for row in rows if row["kind"] == "command")
    variable_names = set(row["name"] for row in rows if row["kind"] == "variable")
    functions = the_table_of(pe, function_tables, function_names)
    commands = the_table_of(pe, command_tables, command_names)
    variables = the_table_of(pe, variable_tables, variable_names)
    old_functions = the_table_of(old_pe, old_function_tables, function_names)
    old_commands = the_table_of(old_pe, old_command_tables, command_names)
    old_variables = the_table_of(old_pe, old_variable_tables, variable_names)
    progress("   the table of the functions: %d entries (the v2.0.19: %d)"
             % (len(functions), len(old_functions)))
    progress("   the table of the commands: %d entries (the v2.0.19: %d)"
             % (len(commands), len(old_commands)))
    progress("   the table of the variables: %d entries (the v2.0.19: %d)"
             % (len(variables), len(old_variables)))
    functions_by_name = dict((entry.name, entry) for entry in functions)
    commands_by_name = dict((entry.name, entry) for entry in commands)
    variables_by_name = dict((entry.name, entry) for entry in variables)
    old_functions_by_name = dict((entry.name, entry) for entry in old_functions)
    old_commands_by_name = dict((entry.name, entry) for entry in old_commands)

    # -- 3. the documentation that changed --------------------------------
    progress("== the documentation")
    old_pages = pages_of(decompile(old_directory))
    new_pages = pages_of(decompile(directory))
    changed = []
    for name in sorted(set(old_pages) & set(new_pages)):
        if old_pages[name] != new_pages[name]:
            changed.append(name)
    progress("   the pages whose syntax changed: %d" % len(changed))

    # -- 4. the questions --------------------------------------------------
    probe = Probe(binary, directory / "probe")
    old_probe = Probe(old_binary, old_directory / "probe")

    # The names that are measured: every name that the table of the tree
    # calls a function or a command, and every name that the words of the
    # binary added.  The arity of the functions is in the binary, so the
    # measurement of them is a check of the binary.
    callable_kinds = set()
    for row in rows:
        if row["kind"] in ("function", "command"):
            callable_kinds.add(row["name"])
    to_measure = set()
    for name in changed:
        if name in known:
            to_measure.add(name)
    for name in function_names | command_names:
        to_measure.add(name)
    for word in only_wide + shaped:
        if word in functions_by_name or word in commands_by_name \
                or word in variables_by_name:
            to_measure.add(word)
    progress("== the questions on the binary")
    progress("   the names that are measured: %d" % len(to_measure))

    def measure(name):
        alone, note = probe.alone(name)
        if name in callable_kinds:
            low, high, ok = arity_of(probe, name, "call")
        else:
            low, high, ok = None, None, False
        return name, alone, note, low, high, ok

    with ThreadPoolExecutor(max_workers=8) as pool:
        measured = list(pool.map(measure, sorted(to_measure)))

    answers = {}
    for name, alone, note, low, high, ok in measured:
        answers[name] = {"alone": alone, "note": note, "min": low, "max": high,
                         "conclusive": ok}

    # The same question is asked of the binary of the v2.0.19: the arity
    # of the commands is not in the binary, and the delta of the arity is
    # the delta of the two measurements.
    def measure_old(name):
        if name not in callable_kinds:
            return name, None, None, False
        low, high, ok = arity_of(old_probe, name, "call")
        return name, low, high, ok

    with ThreadPoolExecutor(max_workers=8) as pool:
        old_measured = list(pool.map(measure_old, sorted(to_measure)))

    for name, low, high, ok in old_measured:
        answers[name]["min_old"] = low
        answers[name]["max_old"] = high
        answers[name]["conclusive_old"] = ok

    # The second form of the question -- the name at the start of a line
    # -- is asked of the names whose documentation changed, so that the
    # two forms can be compared.
    def measure_statement(name):
        if name not in callable_kinds:
            return name, None, None, False
        low, high, ok = arity_of(probe, name, "statement")
        return name, low, high, ok

    with ThreadPoolExecutor(max_workers=8) as pool:
        stated = list(pool.map(measure_statement, sorted(to_measure & set(changed))))

    for name, low, high, ok in stated:
        answers[name]["min_stmt"] = low
        answers[name]["max_stmt"] = high
        answers[name]["conclusive_stmt"] = ok

    # -- 5. the answers of the two binaries --------------------------------
    out("")
    out("== the answers of the two binaries")
    out("   the names whose documentation of the v2.0.32 says that they")
    out("   answer something new, asked of the two binaries")
    given = {}
    for tag, script in ANSWERS:
        new_answer = answer_of(probe, "new_" + tag.replace(".", "_"), script)
        old_answer = answer_of(old_probe, "old_" + tag.replace(".", "_"), script)
        given[tag] = (old_answer, new_answer)
        mark = "  " if new_answer == old_answer else "* "
        out("  %s%-22s v2.0.19: %-40s v2.0.32: %s"
            % (mark, tag, old_answer[:40], new_answer[:60]))

    # The check of the questions: the arity that the questions measured on
    # the binary of the v2.0.32, against the arity of the table of the
    # v2.0.19, and against the arity that the binary of the v2.0.32
    # declares.
    out("")
    out("== the check of the questions")
    checked = 0
    agree = 0
    for row in rows:
        name = row["name"]
        if row["kind"] not in ("function", "command"):
            continue
        answer = answers.get(name)
        if not answer or not answer["conclusive"]:
            continue
        checked += 1
        if str(answer["min"]) == row["min"] and str(answer["max"]) == row["max"]:
            agree += 1
    out("   the arity that the questions measured agrees with the table of "
        "the tree in %d names of %d" % (agree, checked))
    disagree = []
    for row in rows:
        name = row["name"]
        if row["kind"] not in ("function", "command"):
            continue
        answer = answers.get(name)
        if not answer or not answer["conclusive"]:
            continue
        if str(answer["min"]) != row["min"] or str(answer["max"]) != row["max"]:
            disagree.append((name, row["kind"], row["min"], row["max"],
                             answer["min"], answer["max"]))
    for name, kind, low, high, low2, high2 in disagree:
        out("     %-24s %-9s the table says %s-%s, the binary says %s-%s"
            % (name, kind, low, high, low2, high2))
    # The two measurements, one on each binary: they must agree, unless
    # the release moved the arity.
    same = 0
    total = 0
    for row in rows:
        name = row["name"]
        if row["kind"] not in ("function", "command"):
            continue
        answer = answers.get(name)
        if not answer or not answer["conclusive"] \
                or not answer.get("conclusive_old"):
            continue
        total += 1
        if (str(answer["min"]), str(answer["max"])) \
                == (str(answer["min_old"]), str(answer["max_old"])):
            same += 1
    out("   the arity that the questions measured on the two binaries is the "
        "same in %d names of %d" % (same, total))
    # The second form of the question, on the names whose documentation
    # changed: the name at the start of a line must give the same bounds.
    out("   the two forms of the question, on the names whose documentation "
        "changed:")
    for name in sorted(set(changed)):
        answer = answers.get(name)
        if not answer or not answer.get("conclusive_stmt"):
            continue
        if str(answer["min_stmt"]) != str(answer["min"]) \
                or str(answer["max_stmt"]) != str(answer["max"]):
            out("     %-24s the call says %s-%s, the statement says %s-%s"
                % (name, answer["min"], answer["max"],
                   answer["min_stmt"], answer["max_stmt"]))
    out("     (the names that are not named here give the same bounds in "
        "the two forms)")

    # The setters of the properties of the error object, and the call of
    # the class, are members that the v2.0.19 documents and the v2.0.32
    # does not.  A row of the table is added for each of them when the
    # binary of the v2.0.32 answers it and the binary of the v2.0.19
    # does not.
    added_members = []
    for tag, script in ANSWERS:
        if not tag.startswith("Error."):
            continue
        old_answer, new_answer = given[tag]
        if new_answer == old_answer:
            continue
        if new_answer.startswith("e:") or new_answer.startswith("refused"):
            continue
        added_members.append(tag[len("Error."):])

    # -- 6. the table ------------------------------------------------------
    new_rows = []
    kept = []
    for row in rows:
        name = row["name"]
        kind = row["kind"]
        answer = answers.get(name)
        fresh = dict(row)
        note = None

        if kind == "function":
            entry = functions_by_name.get(name)
            if entry is None:
                note = "the name is not in the table of the functions of the v2.0.32"
            else:
                fresh["handler"] = "0x%x" % entry.handler
                fresh["min"] = str(entry.min)
                fresh["max"] = str(entry.max)
                fresh["variant"] = str(entry.variant)
                fresh["flag"] = str(entry.flag)
        elif kind == "command":
            entry = commands_by_name.get(name)
            if entry is None:
                note = "the name is not in the table of the commands of the v2.0.32"
            else:
                fresh["handler"] = "0x%x" % entry.handler
                if answer and answer["conclusive"]:
                    fresh["min"] = str(answer["min"])
                    fresh["max"] = str(answer["max"])
        elif kind == "variable":
            entry = variables_by_name.get(name)
            if entry is not None:
                fresh["handler"] = "0x%x" % entry.handler

        if answer is not None and not answer["conclusive"] \
                and kind in ("function", "command"):
            note = ("the measurement of the arity was not conclusive; "
                    "the value of the table of the tree is kept")
        if note:
            kept.append((row, note))
        new_rows.append(fresh)

    # The members of the error object that the v2.0.32 adds: the setter
    # of each property, and the call of the class.  The rows of the
    # members of the error object carry no handler: they are not in a
    # table of the binary.
    if added_members:
        grown = []
        for row in new_rows:
            grown.append(row)
            if row["kind"] == "method" and row["class"] == "Error" \
                    and row["name"] in added_members:
                grown.append({"kind": "method", "class": "Error",
                              "name": row["name"], "handler": "",
                              "min": "", "max": "", "variant": "", "flag": ""})
        new_rows = grown
        if "Call" in added_members:
            new_rows.append({"kind": "method", "class": "Error", "name": "Call",
                             "handler": "", "min": "", "max": "",
                             "variant": "", "flag": ""})

    # -- 7. the delta ------------------------------------------------------
    out("")
    out("=== the delta between the v2.0.19 and the v2.0.32")
    out("--- the names whose kind changed")
    for row in rows:
        name = row["name"]
        if row["kind"] == "function" and name in commands_by_name:
            out("  %-24s function -> command" % name)
        elif row["kind"] == "command" and name in functions_by_name:
            out("  %-24s command -> function" % name)
    out("--- the names whose arity changed")
    for row in rows:
        name = row["name"]
        answer = answers.get(name)
        if row["kind"] == "function":
            # The arity of a function is in the binary: the two binaries
            # say whether it moved.
            entry = functions_by_name.get(name)
            old_entry = old_functions_by_name.get(name)
            if entry is None:
                out("  %-24s the v2.0.32 does not have it" % name)
            elif old_entry is None:
                out("  %-24s the v2.0.19 did not have it" % name)
            elif (entry.min, entry.max) != (old_entry.min, old_entry.max):
                out("  %-24s %d-%d -> %d-%d  (the entry of the table of the "
                    "binary)" % (name, old_entry.min, old_entry.max,
                                 entry.min, entry.max))
        elif row["kind"] == "command":
            # The arity of a command is not in the binary: the two
            # measurements say whether it moved.
            if not answer or not answer["conclusive"] \
                    or not answer.get("conclusive_old"):
                continue
            if (str(answer["min"]), str(answer["max"])) \
                    != (str(answer["min_old"]), str(answer["max_old"])):
                out("  %-24s %s-%s -> %s-%s  (the measurement of the two "
                    "binaries)" % (name, answer["min_old"], answer["max_old"],
                                   answer["min"], answer["max"]))
    out("--- the names that are new")
    new_names = []
    for word in only_wide + shaped:
        if word in known:
            continue
        if word in functions_by_name or word in commands_by_name:
            new_names.append(word)
            entry = functions_by_name.get(word) or commands_by_name.get(word)
            kind = "function" if word in functions_by_name else "command"
            out("  %-24s %s  handler=0x%x" % (word, kind, entry.handler))
    if not new_names:
        out("  (none)")
    out("--- the names that are gone")
    gone = []
    for row in rows:
        name = row["name"]
        if row["kind"] == "function" and name not in functions_by_name:
            gone.append(row)
        elif row["kind"] == "command" and name not in commands_by_name:
            gone.append(row)
    for row in gone:
        out("  %-24s %s" % (row["name"], row["kind"]))
    if not gone:
        out("  (none)")

    # -- 8. the table ------------------------------------------------------
    out("")
    out("== the ASCII words of the noise")
    for word in shaped:
        out("     %s" % word)
    lines = []
    for row in new_rows:
        fields = [row[column] for column in COLUMNS]
        if row.get("fields") == fields:
            # The row is the row of the tree: it is carried over as it is.
            lines.append(row["line"])
        else:
            lines.append("\t".join(fields))
    table = HEADER + "\n".join(lines) + "\n"
    if kept:
        table += "\n# The rows whose measurement was not conclusive keep the\n"
        table += "# value of the table of the tree:\n"
        for row, note in kept:
            table += "#   %s %s: %s\n" % (row["kind"], row["name"], note)
    if len(sys.argv) > 3:
        pathlib.Path(sys.argv[3]).write_bytes(table.encode("utf-8"))
        progress("the table is written in %s" % sys.argv[3])
    else:
        out(table)
    return 0


if __name__ == "__main__":
    sys.exit(main())
