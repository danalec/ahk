"""Measure the names of the v1 that the CHM documents at an anchor.

The probe of the tree reads the pages of the CHM, and a page may carry
several names: `ListView.htm` carries `LV_Add`, `LV_GetText` and the
rest of the family, and `GetKey.htm` carries `GetKeyName`, `GetKeySC`
and `GetKeyVK`. The index of the CHM (`Index.hhk`) says which name is
at which anchor, and the anchor is the section of the page that carries
the syntax of the name.

This measures those names on the binary, with the questions of the
probe, and writes the rows of the surface that the answers are:

    python measure-anchored.py <the v1 directory> <the table to read> [<the table to write>]

The names that the binary does not know are left out: a statement of
the language (`Break`, `Catch`, `Goto`) is not a name of the surface,
and a topic of the documentation (`ListView`, `TreeView`) is not
either.

A command of the v1 may take the line that follows as its body -- the
family of the legacy `if` does -- and the question of the command needs
a body, or the v1 complains about the body and not about the name.
"""
import importlib.util
import pathlib
import re
import sys
from concurrent.futures import ThreadPoolExecutor

HERE = pathlib.Path(__file__).parent
spec = importlib.util.spec_from_file_location("probe", HERE / "ahk-v1.1.37.02.probe.py")
probe_module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(probe_module)

Probe = probe_module.Probe
NOT_ACTION = probe_module.NOT_ACTION
NO_FUNCTION = probe_module.NO_FUNCTION
ILLEGAL_NAME = probe_module.ILLEGAL_NAME
TOO_FEW = probe_module.TOO_FEW
TOO_MANY = probe_module.TOO_MANY

# The most parameters a scan looks for. A name that takes more than this
# takes any number of them, and the table says 255.
LIMIT = 8

# The statements of the language of the v1. They are the grammar of the
# script, and the table of the surface does not carry them.
STATEMENTS = {
    "Break", "Continue", "Catch", "Finally", "For", "Gosub", "Goto", "Loop",
    "Return", "Switch", "Throw", "Try", "Until", "While", "IfBetween",
    "IfExpression", "IfIn", "IfIs",
}


def sections_of(chm):
    """The sections of the pages of the documentation, by their anchor.

    A page of the CHM may carry several names, and each one of them has
    its own section: the section of `LV_Add` is between the anchor of
    `LV_Add` and the anchor of the next name. The syntax and the
    parameters of a name are the ones of its section, and not the ones
    of the whole page.
    """
    sections = {}
    for page in (chm / "docs" / "lib").glob("*.htm"):
        text = page.read_text(encoding="utf-8", errors="replace")
        # The anchors of the page, in the order of the text.
        marks = [(match.start(), match.group(1)) for match in
                 re.finditer(r'<h[2-4][^>]*id="([^"]+)"', text)]
        marks.append((len(text), None))
        for index in range(len(marks) - 1):
            start, anchor = marks[index]
            end = marks[index + 1][0]
            body = text[start:end]
            sections[(page.stem, anchor)] = (
                probe_module.syntaxes_of(body),
                probe_module.parameters_of(body),
            )
        sections[(page.stem, "")] = (
            probe_module.syntaxes_of(text),
            probe_module.parameters_of(text),
        )
    return sections


def the_names_of_the_index(chm):
    """The names of the index of the CHM, with the anchor that documents each."""
    index = (chm / "Index.hhk").read_text(encoding="utf-8", errors="replace")
    entries = re.findall(
        r'<param name="Name" value="([^"]*)"><param name="Local" value="([^"]*)">',
        index,
    )
    names = {}
    for name, local in entries:
        if not local.startswith("docs/lib/"):
            continue
        target = local[len("docs/lib/"):]
        page, _, anchor = target.partition("#")
        if not page.endswith(".htm"):
            continue
        label = name.strip()
        if label.endswith("()"):
            label = label[:-2]
        if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", label):
            continue
        names.setdefault(label, (page[:-4], anchor))
    return names


class Measurer:
    """The questions of the probe, for the names that take a body."""

    def __init__(self, binary, work):
        self.probe = Probe(binary, work)
        self.binary = binary
        self.work = work
        self.counter = 0

    def run(self, body, timeout=15):
        # Every question has a script of its own: the questions run at
        # the same time, and the script of one is the script of the
        # other otherwise.
        self.counter += 1
        script = self.work / ("q%d.ahk" % self.counter)
        script.write_bytes(body.replace("\n", "\r\n").encode("utf-8"))
        import subprocess

        try:
            proc = subprocess.run(
                [str(self.binary), "/ErrorStdOut", str(script)],
                capture_output=True,
                text=True,
                timeout=timeout,
            )
        except subprocess.TimeoutExpired:
            return "TIMEOUT"
        message = proc.stderr or ""
        if "==>" in message:
            message = message.split("==>")[-1]
        return " ".join(message.split())

    def kind(self, name):
        """The kind of the name: a command, a function, or both.

        The command of the v1 may take the line that follows as its body,
        and the question asks with a body: the line of the body is there,
        and the name is measured and not the body.
        """
        command = False
        for line in ("%s, 0" % name, "%s" % name):
            message = self.run("if (0)\n    %s\n    x := 1\n" % line)
            if NOT_ACTION in message or message == "TIMEOUT":
                continue
            command = True
            break
        message = self.run("if (0)\n    x := %s(0)\n    y := 1\n" % name)
        function = NO_FUNCTION not in message
        if command and function:
            return "command and function"
        if command:
            return "command"
        if function:
            return "function"
        return "-"

    def arity(self, name, form):
        """The fewest and the most parameters of the name."""
        accepted = []
        for count in range(LIMIT + 1):
            if form == "command":
                args = ", ".join(['"a"'] * count)
                line = ("%s, %s" % (name, args)) if args else name
                body = "if (0)\n    %s\n    x := 1\n" % line
            else:
                args = ", ".join(["0"] * count)
                body = "if (0)\n    x := %s(%s)\n    y := 1\n" % (name, args)
            message = self.run(body)
            if not TOO_FEW.search(message) and not TOO_MANY.search(message):
                accepted.append(count)
        if not accepted:
            return None, None
        low, high = accepted[0], accepted[-1]
        return low, (255 if high == LIMIT else high)

    def shape(self, name, count, form):
        """The positions of the parameters that want the name of a variable."""
        if form == "function":
            # The v1 does not check the parameters of a function at load
            # time, so the shape of a function comes from the
            # documentation alone.
            return []
        positions = []
        values = ['"p%d"' % (index + 1) for index in range(count)]
        for _ in range(count + 1):
            line = "%s, %s" % (name, ", ".join(values))
            message = self.run("if (0)\n    %s\n    x := 1\n" % line)
            found = ILLEGAL_NAME.search(message)
            if not found:
                break
            digits = found.group(1)[1:]
            if not digits.isdigit():
                break
            index = int(digits) - 1
            if index in positions:
                break
            positions.append(index)
            values[index] = "v%d" % (index + 1)
        return positions


def the_rows_of_the_table(path):
    """The rows of a table of the surface, by name."""
    rows = {}
    for line in pathlib.Path(path).read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#") or line.startswith("kind"):
            continue
        fields = line.split("\t")
        if len(fields) < 6:
            continue
        rows.setdefault(fields[2], []).append(fields)
    return rows


def main():
    directory = pathlib.Path(sys.argv[1]).resolve()
    binary = directory / "bin" / "AutoHotkeyU64.exe"
    table = pathlib.Path(sys.argv[2])
    chm = probe_module.decompile(directory)

    known = the_rows_of_the_table(table)
    index = the_names_of_the_index(chm)
    candidates = {
        name: page for name, page in index.items()
        if name not in known and name not in STATEMENTS
    }

    work = directory / "probe"
    work.mkdir(exist_ok=True)
    measurer = Measurer(binary, work)
    sections = sections_of(chm)

    def measure(name):
        kind = measurer.kind(name)
        if kind == "-":
            return name, None
        forms = ["command", "function"] if kind == "command and function" else [kind]
        low, high = measurer.arity(name, forms[0])
        rows = []
        for form in forms:
            page, anchor = candidates[name]
            syntaxes, parameters = sections.get((page, anchor), ([], []))
            line = probe_module.syntax_for(syntaxes, name, form)
            names_of_parameters = probe_module.parameters_of_a_syntax(line, form)
            said = dict(parameters)
            positions = measurer.shape(name, low or 0, form)
            shapes = []
            for index, parameter in enumerate(names_of_parameters):
                shapes.append(
                    probe_module.shape_of(
                        said.get(parameter, ""),
                        parameter,
                        index in positions,
                        form == "command",
                    )
                )
            rows.append((
                form,
                "" if low is None else low,
                "" if high is None else high,
                ",".join(shapes),
            ))
        return name, rows

    with ThreadPoolExecutor(max_workers=6) as pool:
        measured = list(pool.map(measure, sorted(candidates)))

    lines = []
    for name, rows in measured:
        if rows is None:
            continue
        for form, low, high, shapes in rows:
            lines.append("%s\t\t%s\t%s\t%s\t%s" % (form, name, low, high, shapes))

    if len(sys.argv) > 3:
        pathlib.Path(sys.argv[3]).write_bytes(("\n".join(lines) + "\n").encode("utf-8"))
    else:
        sys.stdout.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
