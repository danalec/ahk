"""Measure the surface of the AutoHotkey v1.1.37.02.

    python ahk-v1.1.37.02.probe.py <the directory of the v1.1.37.02> [<the table to write>]

The directory is the one of the portable v1: `bin/AutoHotkeyU64.exe` and
`AutoHotkey.chm` inside it. The documentation is decompiled with `hh.exe`
when the `chm/` directory is not there yet, and the table is written where
the second argument says (on stdout when it is not given).

The names come from the documentation: the pages of the CHM, and the page of
the built-in variables. The kind of each one is measured on the binary, with
three questions that the v1 asks in three different ways, and the three of
them are inside a dead branch: the resolution of a name in the v1 is static,
so the answer is a LOAD error and nothing runs. `/ErrorStdOut` prints it on
stdout.

    if (0)
        Name, 0          # a command?  the script loads, or the error is a
                         # complaint about a parameter
    if (0)
        x := Name()      # a function?  "Call to nonexistent function" says no
    x := IsSet(Name)    # a variable?  the value 1 says yes

The arity comes out of the same binary: a scan of the count of the
parameters, and the complaint that says that the count is short or long.

The rules that came out of the measurement, and the two blind spots the
questions have, are written down in `docs/ahk-v1-dialect.md` (the section
1.3).
"""
import pathlib
import re
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from html import unescape

NOT_ACTION = "does not contain a recognized action"
NO_FUNCTION = "Call to nonexistent function"
PARAM_COMPLAINT = re.compile(
    r"Parameter #\d+( and \d+)? (invalid|required|must not be blank)"
    r"|Parameters? \d+( through \d+)? must not be blank"
    r"|requires at least \d+ parameter"
    r"|Too few parameters")
TOO_FEW = re.compile(r"requires at least \d+ parameter|Too few parameters passed to function"
                     r"|Parameter #\d+ required")
TOO_MANY = re.compile(r"requires at most \d+ parameter|Too many parameters passed to function")

# The most parameters a scan looks for. A name that takes more than this is
# a name that takes any number of them.
LIMIT = 8

# The statements of the language of the v1.  The probe sees them as commands
# in one case only; the others it separates by their own message.
STATEMENTS = {
    "Throw": "Throw, value: the statement of the grammar, not a command",
}


def decompile(directory):
    """The documentation of the v1, out of its CHM."""
    chm = directory / "chm"
    if (chm / "Index.hhk").exists():
        return chm
    source = directory / "AutoHotkey.chm"
    if not source.exists():
        source = directory / "bin" / "AutoHotkey.chm"
    chm.mkdir(exist_ok=True)
    subprocess.run(["hh.exe", "-decompile", str(chm), str(source)], check=True)
    return chm


def names_of_the_docs(chm):
    index = (chm / "Index.hhk").read_text(encoding="utf-8", errors="replace")
    entries = re.findall(
        r'<param name="Name" value="([^"]*)"><param name="Local" value="([^"]*)">',
        index,
    )
    pages = set()
    for _, local in entries:
        if local.startswith("docs/lib/") and local.endswith(".htm"):
            stem = local[len("docs/lib/"):-4]
            if stem != "index":
                pages.add(stem)
    variables_page = (chm / "docs" / "Variables.htm").read_text(encoding="utf-8", errors="replace")
    variables = sorted(set(re.findall(r"\bA_[A-Za-z][A-Za-z0-9_]*\b", variables_page)))
    commands = ["#" + stem[1:] if stem.startswith("_") else stem for stem in sorted(pages)]
    return commands, variables


# The words of the documentation that say what a name is for. The name
# that a command writes into is an Output; the name that it reads from is
# a Variable; the name of something else -- an environment variable, a
# window -- is a Value, because it is not a variable of the script.
OUTPUT_WORD = re.compile(r"output variable|in which to store", re.I)
READ_WORD = re.compile(r"the name of the (input )?variable|name of the input variable", re.I)
ILLEGAL_NAME = re.compile(r'illegal character:\s*""?([A-Za-z_]\w*)""?')


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


def parameters_of(text):
    """The parameters of a page: the name of each one and what it says."""
    found = []
    for match in re.finditer(r"<dt>(.*?)</dt>\s*<dd>(.*?)</dd>", text, re.S):
        name = unescape(re.sub(r"<[^>]+>", "", match.group(1))).strip()
        said = " ".join(unescape(re.sub(r"<[^>]+>", " ", match.group(2))).split())
        found.append((name, said))
    return found


def parameters_of_a_syntax(line, kind):
    """The names of the parameters of a syntax line, in order."""
    if kind == "function":
        inside = line[line.find("(") + 1:line.rfind(")")]
        return [part.strip() for part in inside.split(",") if part.strip()]
    parts = [part.strip() for part in line.split(",")]
    if len(parts) > 1:
        return parts[1:]
    # The new form of a command: `Send Keys` -- the name and the first
    # parameter share the line, and there is no comma to split on.
    words = line.split(None, 1)
    return [words[1].strip()] if len(words) > 1 else []


def syntax_for(syntaxes, name, kind):
    """The syntax line of the form that the kind asks for.

    A page may carry several: `ControlGet` has sixteen, and
    `GetKeyState` has both a command and a function.
    """
    if kind == "command":
        # The name of a command is followed by a comma in the syntax of
        # the v1 (`StringLen, OutputVar, InputVar`), and the new form
        # carries the first parameter on the same line (`Send Keys`).
        chosen = [line for line in syntaxes
                  if line.split() and line.split()[0].rstrip(",") == name]
    else:
        chosen = [line for line in syntaxes if re.search(r"\b%s\s*\(" % re.escape(name), line)]
    if not chosen:
        chosen = syntaxes
    # The variant with the most parameters: the others are shorter forms
    # of the same one.
    return max(chosen, key=lambda line: len(line.split(",")))


def pages_of(chm):
    """The syntax and the parameters of every page of the documentation."""
    pages = {}
    for page in (chm / "docs" / "lib").glob("*.htm"):
        text = page.read_text(encoding="utf-8", errors="replace")
        pages[page.stem] = (syntaxes_of(text), parameters_of(text))
    return pages


class Probe:
    """One process per question, and the error that answers it."""

    def __init__(self, binary, work):
        self.binary = binary
        self.work = work

    def run(self, tag, body, timeout=15):
        out = self.work / f"{tag}.txt"
        script = self.work / f"{tag}.ahk"
        text = body.replace("%OUT%", str(out).replace("\\", "\\\\"))
        if "ExitApp" not in text:
            text += "\nExitApp\n"
        script.write_bytes(text.replace("\n", "\r\n").encode("utf-8"))
        if out.exists():
            out.unlink()
        proc = subprocess.Popen([str(self.binary), "/ErrorStdOut", str(script)],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            _, stderr = proc.communicate(timeout=timeout)
            code = proc.returncode
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.communicate()
            return "TIMEOUT", "", ""
        written = out.read_text(encoding="utf-8", errors="replace") if out.exists() else ""
        return stderr, written.strip(), code

    @staticmethod
    def message(stderr):
        if "==>" in stderr:
            stderr = stderr.split("==>")[-1]
        return " ".join(stderr.split())

    def command(self, name):
        stderr, written, code = self.run(f"cmd_{name}",
                                         f"if (0)\n    {name}, 0\nFileAppend, ok, %OUT%\n")
        msg = self.message(stderr)
        if written == "ok":
            return "command", msg
        if NOT_ACTION in msg:
            return "not a command", msg
        if PARAM_COMPLAINT.search(msg):
            return "command", msg
        if msg.startswith("Target label does not exist") and "Specifically:" not in msg:
            return "command", msg
        if code == 0 and not msg:
            # The name took the next line as its body: a statement, not a command.
            return "statement", "the statement took the next line as its body"
        return "not a command", msg

    def function(self, name):
        stderr, _, _ = self.run(f"fn_{name}",
                                 f"if (0)\n    x := {name}()\nFileAppend, ok, %OUT%\n")
        msg = self.message(stderr)
        return (NO_FUNCTION not in msg), msg

    def variable(self, name):
        _, written, _ = self.run(f"var_{name}",
                                 f"x := IsSet({name})\nFileAppend, %x%, %OUT%\n")
        if written == "1":
            return True, ""
        if written in ("", "TIMEOUT"):
            # IsSet refuses it: it is a built-in that cannot be written.
            _, value, _ = self.run(f"read_{name}", f"x := {name}\nFileAppend, %x%, %OUT%\n")
            if value != "":
                return True, "IsSet refused it; the direct read answers " + value
            return False, "IsSet refused it and the direct read is empty"
        return False, ""

    def directive(self, name):
        stderr, written, _ = self.run(f"dir_{name[1:]}", f"{name}\nFileAppend, ok, %OUT%\n")
        msg = self.message(stderr)
        if written == "ok" or "Parameter #1 required" in msg:
            return True, msg
        return False, msg

    def shape(self, name, count):
        """The positions of the parameters that want the name of a variable.

        The v1 checks a command at load time, and the complaint names the
        parameter that is wrong: a quoted string is not the name of a
        variable. Each position gets a text of its own, so the complaint
        says which one it is, and the probe goes on with the next.

        The answer is the list of the positions, and whether the question
        was conclusive: a command that wants more parameters than the
        page says complains about the count first, and then nothing is
        known.
        """
        values = ['"p%d"' % (index + 1) for index in range(count)]
        positions = []
        for _ in range(count + 1):
            body = "if (0)\n    %s, %s" % (name, ", ".join(values))
            message = self.message(self.run(f"sh_{name}_{len(positions)}", body)[0])
            found = ILLEGAL_NAME.search(message)
            if not found:
                # The count of the parameters is the other complaint, and it
                # comes first: the question was not asked.
                conclusive = "requires at least" not in message and "Too few" not in message
                return positions, conclusive
            index = int(found.group(1)[1:]) - 1
            positions.append(index)
            values[index] = "v%d" % (index + 1)
        return positions, True

    def arity(self, name, form):
        """The fewest and the most parameters, by a scan of the count.

        The count is what the binary complains about, and the complaint is a
        different one for each case: "requires at least N parameters" and
        "Too few parameters passed to function" say that the count is short,
        "requires at most N parameters" and "Too many parameters passed to
        function" say that it is long. Every other answer -- a value that is
        not accepted, a name that is not a variable, a label that does not
        exist -- says that the count was fine, and the scan moves on.
        """
        accepted = []
        for count in range(LIMIT + 1):
            args = ", ".join(["0"] * count)
            if form == "command":
                body = f"if (0)\n    {name}, {args}" if count else f"if (0)\n    {name}"
            else:
                body = f"if (0)\n    x := {name}({args})"
            message = self.message(self.run(f"ar_{form}_{name}_{count}", body)[0])
            if not TOO_FEW.search(message) and not TOO_MANY.search(message):
                accepted.append(count)
        if not accepted:
            return None, None
        low, high = accepted[0], accepted[-1]
        # A name that takes any number of them has the 255 of the table of v2.
        return low, (255 if high == LIMIT else high)


def shape_of(documented, syntax_name, measured, conclusive):
    """The shape of one parameter.

    The binary says whether the parameter wants the *name* of a variable,
    and the documentation says whether that name is written into (an
    Output) or read from (a Variable). A function is not checked at load
    time, so for those the documentation alone decides.
    """
    written = bool(OUTPUT_WORD.search(documented)) or syntax_name.lower().startswith("out")
    if conclusive:
        if not measured:
            return "Value"
        return "Output" if written else "Variable"
    if written:
        return "Output"
    if READ_WORD.search(documented):
        return "Variable"
    return "Value"


def shapes_of(probe, pages, name, kind, low):
    """The shapes of the parameters of a name, in order."""
    if kind == "directive":
        # A directive is not a row of the built-in table, and every
        # parameter of a directive is a value: there is no name of a
        # variable to read from and no variable to write into.
        return []
    page = pages.get(name[1:] if name.startswith("#") else name)
    if not page:
        return []
    syntaxes, parameters = page
    line = syntax_for(syntaxes, name, kind)
    names = parameters_of_a_syntax(line, kind)
    said = dict(parameters)
    count = max(len(names), low or 0)
    if kind == "command":
        measured, conclusive = probe.shape(name, count)
    else:
        measured, conclusive = [], False
    return [shape_of(said.get(parameter, ""), parameter, index in measured, conclusive)
            for index, parameter in enumerate(names)]


def kind_of(probe, name):
    if name.startswith("#"):
        ok, note = probe.directive(name)
        return ("directive", note) if ok else ("-", note)

    if name in STATEMENTS:
        return "-", STATEMENTS[name]

    cmd, cmd_note = probe.command(name)
    fn, fn_note = probe.function(name)
    var, var_note = probe.variable(name)

    if cmd == "command" and fn:
        return "command and function", cmd_note
    if cmd == "command":
        return "command", cmd_note
    if cmd == "statement":
        return "-", cmd_note
    if fn:
        return "function", fn_note
    if var:
        return "variable", var_note
    return "-", cmd_note or fn_note or var_note


HEADER = """\
# The AutoHotkey v1.1.37.02 surface, measured on the binary.
#
# kind      command, function, variable or directive
# class     the class a method belongs to; the v1 has no runtime classes,
#           and the column is empty for every row
# min       the fewest parameters, measured on 2026-10-29
# max       the most parameters, 255 for any number: the v1 ignores the
#           parameters that a command does not need, and a function that
#           takes any number of them says so
# shape     the shape of each parameter, in order: Value (an expression),
#           Variable (the name of a variable of the script) or Output (the
#           name of a variable of the script that the command writes into).
#           Measured on 2026-10-29: the binary says whether a parameter
#           wants a name -- a command is checked at load time, and the
#           complaint is on stderr -- and the documentation says whether
#           that name is written into or read from. A function is checked
#           at runtime, so its shape comes from the documentation alone.
#           A directive has no shape: it is not a row of the built-in
#           table, and every parameter of a directive is a value.
#
# The names come from the documentation of the v1 -- the pages of the CHM
# and the page of the built-in variables -- and the kind and the arity of
# each one were measured on the binary on 2026-10-29 with the probe beside
# this table. The rules of the questions, and the two blind spots they
# have, are written down in docs/ahk-v1-dialect.md, the section 1.3.
#
# A name that is a command and a function (GetKeyState) has two rows, the
# way the table of the v2 carries a name twice when it is two things:
# SendMode is a command and a variable of the v2.
kind\tclass\tname\tmin\tmax\tshape
"""


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    directory = pathlib.Path(sys.argv[1]).resolve()
    binary = directory / "bin" / "AutoHotkeyU64.exe"
    if not binary.exists():
        print("the binary of the v1 is not there: %s" % binary)
        return 2

    chm = decompile(directory)
    commands, variables = names_of_the_docs(chm)
    names = commands + [v for v in variables if v not in commands]

    pages = pages_of(chm)

    work = directory / "probe"
    work.mkdir(exist_ok=True)
    probe = Probe(binary, work)

    with ThreadPoolExecutor(max_workers=8) as pool:
        kinds = list(pool.map(lambda n: kind_of(probe, n), names))

    # The arity of the names that are commands or functions.
    calls = [(n, k[0]) for n, k in zip(names, kinds)
             if k[0] in ("command", "function", "command and function")]

    def bounds(entry):
        name, kind = entry
        low, high = probe.arity(name, "command" if kind == "command" else "function")
        if kind == "command and function":
            low_f, high_f = probe.arity(name, "function")
            if low_f is not None:
                low = low if low is not None else low_f
                high = high_f if high is None else max(high, high_f)
        return name, low, high

    with ThreadPoolExecutor(max_workers=8) as pool:
        arities = {name: (low, high) for name, low, high in pool.map(bounds, calls)}

    # The rows of the table: a name that is two things has two of them.
    rows = []
    for name, (kind, _) in zip(names, kinds):
        if kind == "-":
            continue
        if kind == "command and function":
            rows.append((name, "command"))
            rows.append((name, "function"))
            continue
        rows.append((name, kind))

    with ThreadPoolExecutor(max_workers=8) as pool:
        shapes = list(pool.map(
            lambda row: shapes_of(probe, pages, row[0], row[1], arities.get(row[0], (None, None))[0]),
            rows,
        ))

    lines = []
    for (name, kind), shape in zip(rows, shapes):
        low, high = arities.get(name, (None, None))
        low = "" if low is None else low
        high = "" if high is None else high
        lines.append("%s\t\t%s\t%s\t%s\t%s" % (kind, name, low, high, ",".join(shape)))

    table = HEADER + "\n".join(lines) + "\n"
    if len(sys.argv) > 2:
        pathlib.Path(sys.argv[2]).write_bytes(table.encode("utf-8"))
    else:
        sys.stdout.write(table)

    counts = {}
    for kind, _ in kinds:
        counts[kind] = counts.get(kind, 0) + 1
    print("# the kinds: %s" % counts, file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
