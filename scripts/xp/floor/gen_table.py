"""Write the table of the floor probe.

The table is the UNION of the imports of every binary of the tree that
goes to the machine of the lab, built for thirty-two bits:

    python scripts/xp/floor/gen_table.py \\
        target/i686-win7-windows-gnu/release/ahk.exe \\
        /tmp/helloxp/target/i686-win7-windows-gnu/release/hello.exe \\
        > scripts/xp/floor/src/table.rs

The probe carries that table and asks the XP machine, one symbol at a
time, which of them it has.

WHY the union and not one binary: the gate (`scripts/xp/gate.py`) is an
oracle of three answers -- measured missing, measured present, never
asked about -- and it fails on the third.  A table taken from `ahk.exe`
alone leaves the gate unable to vouch for the minimal `hello` that is
the reproducer of the lab, and `hello` imports more than `ahk.exe`
does: the `--gc-sections` of the tree cuts from `ahk.exe` the sections
it never references, and the sockets of `ws2_32` and the `userenv` are
among them.  The floor has to cover what is actually run.

The API sets are the DLLs of the table of the modern binary that XP
does not have as such: the symbol is asked of the DLL that holds it on
the old machines (the kernel32 of the waits, the user32 of the
windows), so that the answer of the machine is about the *symbol* and
not about the name of the DLL.
"""

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, ".."))

import gate  # noqa: E402

# The API set and the DLL of the old machines that holds the symbol.
CANONICAL = {
    "api-ms-win-core-synch-l1-2-0.dll": "kernel32.dll",
    "api-ms-win-core-synch-l1-1-0.dll": "kernel32.dll",
    "api-ms-win-core-winrt-l1-1-0.dll": "combase.dll",
    "api-ms-win-shcore-scaling-l1-1-1.dll": "shcore.dll",
    "api-ms-win-shcore-scaling-l1-1-2.dll": "shcore.dll",
}


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    # The union of the tables.  The count per binary goes to stderr, so
    # that stdout stays the table.
    rows = set()
    contagem = []
    for path in sys.argv[1:]:
        with open(path, "rb") as handle:
            pe = gate.Pe(handle.read())
        do_binario = set()
        for dll, symbols in pe.imports():
            dll = CANONICAL.get(dll.lower(), dll)
            for symbol in symbols:
                do_binario.add((dll, symbol))
        rows |= do_binario
        contagem.append((path, len(do_binario)))
    rows = sorted(rows)
    out = sys.stdout
    out.write("// The table of the floor probe: the symbols of the tree, and the\n")
    out.write("// DLL that holds each of them.  It is written by\n")
    out.write("// `scripts/xp/floor/gen_table.py`, from the UNION of the tables of\n")
    out.write("// the imports of every binary of the tree that goes to the machine\n")
    out.write("// of the lab, built for thirty-two bits.\n")
    out.write("pub const SYMBOLS: &[(&str, &str)] = &[\n")
    for dll, symbol in rows:
        out.write('    ("%s", "%s"),\n' % (dll, symbol))
    out.write("];\n")
    sys.stderr.write("the union: %d symbols\n" % len(rows))
    for path, quantos in contagem:
        sys.stderr.write("  %5d  %s\n" % (quantos, path))
    return 0


if __name__ == "__main__":
    sys.exit(main())
