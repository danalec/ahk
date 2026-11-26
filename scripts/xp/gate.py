#!/usr/bin/env python3
"""The gate of the XP floor: the import table of a PE, and the question.

The floor of a binary is its import table, and nothing else
(`docs/windows-legacy.md`).  A binary whose table asks for something
Windows XP SP3 does not have does not start on XP: the loader resolves
the imports before the first instruction of the program, and refuses.

    python scripts/xp/gate.py <the .exe>

The gate fails, and says why, when:

  - the machine of the binary is neither the thirty-two bits one nor
    the sixty-four bits one;
  - the subsystem version is newer than the one of XP (5.1);
  - a DLL of the api sets (`api-ms-win-*`, `ext-ms-*`) is in the table;
  - a DLL outside the allowlist is in the table;
  - a symbol is in the table that the machine of the lab said it does
    not have (the MISSING lines of the floor);
  - a symbol is in the table that nobody asked the machine about --
    the floor is an oracle of three answers, and a floor that was not
    measured is a floor that is guessed.

The authority is the measurement: `floor-xp-sp3.txt` and
`floor-xp-x64.txt`, the answers of the two XP machines of the lab to
the probe of `scripts/xp/floor` (352 symbols asked of the SP3 of
thirty-two bits, 277 of the x64 SP2, none missing in either). The
allowlist of the DLLs is the measurement of the last AutoHotkey v1
(1.1.37.02, the last one that runs on XP): the fourteen classic DLLs
of its table, plus `msvcrt.dll`, the CRT that Windows has since NT,
plus the three the tree asks and XP SP3 has (iphlpapi, userenv,
ws2_32).
"""

import os
import struct
import sys

# The DLLs of the floor: the fourteen of the AutoHotkey v1 and the CRT,
# plus the three that the tree asks and that XP SP3 has (measured on
# the machine of the lab):
#
#   iphlpapi.dll   the addresses of the adapters (XP has it)
#   userenv.dll    the profiles of the users (XP has it)
#   ws2_32.dll     the sockets (XP has it)
ALLOWED_DLLS = {
    "kernel32.dll",
    "user32.dll",
    "gdi32.dll",
    "advapi32.dll",
    "shell32.dll",
    "shlwapi.dll",
    "comdlg32.dll",
    "comctl32.dll",
    "ole32.dll",
    "oleaut32.dll",
    "winmm.dll",
    "psapi.dll",
    "version.dll",
    "wsock32.dll",
    "wininet.dll",
    "imm32.dll",
    "ntdll.dll",
    "msvcrt.dll",
    "iphlpapi.dll",
    "userenv.dll",
    "ws2_32.dll",
}

# The symbols that are newer than XP SP3, with the Windows that brought
# them.
#
# The table is MEASURED, not quoted: the probe of `scripts/xp/floor`
# asked the XP machine of the lab for every symbol of the tree, and
# these are the twenty-nine of the 372 that it does not have.  The
# evidence is `scripts/xp/floor-xp-sp3.txt`.
#
# The symbols that the documents put after XP and that the machine DOES
# have are not here, because the machine is the oracle: XP has
# `GetAdaptersAddresses` (iphlpapi.dll), `SetWinEventHook`,
# `InitializeCriticalSectionEx`, `CancelIo`, `SwitchToThread`,
# `WaitForSingleObjectEx` and `GetFileTime`.
NEWER_THAN_XP = {
    "WaitOnAddress": "Windows 8",
    "WakeByAddressSingle": "Windows 8",
    "WakeByAddressAll": "Windows 8",
    "GetSystemTimePreciseAsFileTime": "Windows 8",
    "IsThreadAFiber": "Windows 8",
    "GetDpiForMonitor": "Windows 8.1",
    "GetDpiForSystem": "Windows 10 1607",
    "SetProcessDpiAwarenessContext": "Windows 10 1703",
    "CompareStringOrdinal": "Windows Vista",
    "CreateSymbolicLinkW": "Windows Vista",
    "CreateWaitableTimerExW": "Windows Vista",
    "DeleteProcThreadAttributeList": "Windows Vista",
    "FlsAlloc": "Windows Vista",
    "FlsFree": "Windows Vista",
    "FlsSetValue": "Windows Vista",
    "GetFileInformationByHandleEx": "Windows Vista",
    "GetFinalPathNameByHandleW": "Windows Vista",
    "GetUserDefaultLocaleName": "Windows Vista",
    "InitOnceBeginInitialize": "Windows Vista",
    "InitOnceComplete": "Windows Vista",
    "InitializeProcThreadAttributeList": "Windows Vista",
    "QueryFullProcessImageNameW": "Windows Vista",
    "SetFileInformationByHandle": "Windows Vista",
    "SetThreadStackGuarantee": "Windows Vista",
    "UpdateProcThreadAttribute": "Windows Vista",
    "SHCreateItemFromParsingName": "Windows Vista",
    "SHGetKnownFolderPath": "Windows Vista",
    "GetHostNameW": "the export is not on XP SP3",
    "ProcessPrng": "Windows 7 (the DLL itself is not on XP SP3)",
    # The four below are not in the table of the tree, so the probe did
    # not ask the machine about them: they are quoted, not measured.
    "GetTickCount64": "Windows Vista",
    "DwmGetWindowAttribute": "Windows Vista",
    "AddClipboardFormatListener": "Windows Vista",
    "GetDpiForWindow": "Windows 10 1607",
}

# The floor measured on the machines of the laboratory: the symbols
# that XP has, and the symbols that it does not.  Two machines answer:
# the XP SP3 of thirty-two bits, and the XP x64 SP2 of sixty-four --
# the eight symbols that only exist as such on a sixty-four-bit
# Windows can only be asked of the second.
#
# WHY: the table is the ORACLE. A symbol that the probe did not ask
# about is a symbol that nobody knows: the gate fails on it, and the
# floor is measured again.
AQUI = os.path.dirname(os.path.abspath(__file__))
FLOORS = [
    os.path.join(AQUI, "floor-xp-sp3.txt"),
    os.path.join(AQUI, "floor-xp-x64.txt"),
]


def the_floor():
    """The symbols that XP has, and the ones that it does not."""
    present, missing = set(), set()
    for floor in FLOORS:
        with open(floor, encoding="utf-8") as mao:
            for line in mao:
                line = line.strip()
                if line.startswith("MISSING "):
                    missing.add(line[8:])
                elif line.startswith("PRESENT "):
                    present.add(line[8:])
    return present, missing


MACHINE_I386 = 0x014C
MACHINE_AMD64 = 0x8664


class PeError(Exception):
    pass


class Pe:
    """The little of the PE that the gate needs: the header and the imports."""

    def __init__(self, data):
        self.data = data
        if data[:2] != b"MZ":
            raise PeError("not a DOS executable")
        (self.pe_offset,) = struct.unpack_from("<I", data, 0x3C)
        if data[self.pe_offset:self.pe_offset + 4] != b"PE\0\0":
            raise PeError("not a PE")
        coff = self.pe_offset + 4
        (self.machine, self.sections_count) = struct.unpack_from("<HH", data, coff)
        (self.optional_size,) = struct.unpack_from("<H", data, coff + 16)
        opt = coff + 20
        (self.magic,) = struct.unpack_from("<H", data, opt)
        if self.magic == 0x10B:  # PE32
            self.pe32plus = False
        elif self.magic == 0x20B:  # PE32+
            self.pe32plus = True
        else:
            raise PeError("unknown optional header magic 0x%04X" % self.magic)
        # The subsystem version is at the same place in the two.
        (self.subsystem_major, self.subsystem_minor) = struct.unpack_from(
            "<HH", data, opt + 40
        )
        (self.subsystem,) = struct.unpack_from("<H", data, opt + 68)
        # The data directories: the import table is the second one.
        directories = opt + (112 if self.pe32plus else 96)
        (self.import_rva, self.import_size) = struct.unpack_from(
            "<II", data, directories + 8
        )
        self.sections = []
        table = opt + self.optional_size
        for i in range(self.sections_count):
            at = table + i * 40
            name = data[at:at + 8].rstrip(b"\0").decode("latin1")
            vsize, vaddr, raw_size, raw_ptr = struct.unpack_from("<IIII", data, at + 8)
            self.sections.append((name, vaddr, vsize, raw_ptr, raw_size))

    def offset_of(self, rva):
        """The place of an RVA in the file."""
        for _name, vaddr, vsize, raw_ptr, raw_size in self.sections:
            if vaddr <= rva < vaddr + max(vsize, raw_size):
                return raw_ptr + (rva - vaddr)
        raise PeError("RVA 0x%X is in no section" % rva)

    def string_at(self, rva):
        at = self.offset_of(rva)
        end = self.data.index(b"\0", at)
        return self.data[at:end].decode("latin1")

    def imports(self):
        """The table of the imports: a list of (dll, [symbols])."""
        if self.import_rva == 0:
            return []
        at = self.offset_of(self.import_rva)
        out = []
        while True:
            fields = struct.unpack_from("<IIIII", self.data, at)
            at += 20
            if fields == (0, 0, 0, 0, 0):
                break
            original, _stamp, _chain, name_rva, first = fields
            dll = self.string_at(name_rva).lower()
            thunk = original or first
            symbols = []
            toff = self.offset_of(thunk)
            step = 8 if self.pe32plus else 4
            while True:
                (entry,) = struct.unpack_from("<Q" if self.pe32plus else "<I",
                                             self.data, toff)
                if entry == 0:
                    break
                toff += step
                if entry & (1 << (63 if self.pe32plus else 31)):
                    symbols.append("#%d" % (entry & 0xFFFF))
                else:
                    symbols.append(self.string_at(entry + 2))
            out.append((dll, symbols))
        return out


def check(path):
    with open(path, "rb") as handle:
        pe = Pe(handle.read())

    problems = []
    if pe.machine not in (MACHINE_I386, MACHINE_AMD64):
        problems.append(
            "the machine of the binary is 0x%04X, and the floor wants the "
            "thirty-two bits one (0x%04X) or the sixty-four bits one "
            "(0x%04X)" % (pe.machine, MACHINE_I386, MACHINE_AMD64)
        )
    if (pe.subsystem_major, pe.subsystem_minor) > (5, 1):
        problems.append(
            "the subsystem version is %d.%d, and XP SP3 is 5.1"
            % (pe.subsystem_major, pe.subsystem_minor)
        )

    table = pe.imports()
    present, missing = the_floor()
    for dll, symbols in table:
        if dll.startswith("api-ms-win-") or dll.startswith("ext-ms-"):
            problems.append(
                "%s is a DLL of the api sets, and XP has no api set" % dll
            )
        elif dll not in ALLOWED_DLLS:
            problems.append("%s is not on the allowlist of the floor" % dll)
        for symbol in symbols:
            chave = "%s!%s" % (dll, symbol)
            if chave in missing:
                problems.append(
                    "%s!%s is %s, and XP SP3 does not have it"
                    % (dll, symbol, NEWER_THAN_XP.get(symbol, "a function of Vista"))
                )
            elif chave not in present:
                problems.append(
                    "%s was not asked about the machine: the floor is measured "
                    "again, and the probe asks it" % chave
                )

    print("file:      %s" % path)
    print("machine:   0x%04X%s" % (
        pe.machine,
        {MACHINE_I386: " (i386)", MACHINE_AMD64: " (x86_64)"}.get(pe.machine, "")))
    print("subsystem: %d.%d (%s)" % (
        pe.subsystem_major, pe.subsystem_minor,
        {2: "GUI", 3: "console"}.get(pe.subsystem, str(pe.subsystem))))
    print("imports:")
    for dll, symbols in table:
        print("  %-14s %d symbol(s)" % (dll, len(symbols)))
        for symbol in symbols:
            mark = "  <-- newer than XP" if symbol in NEWER_THAN_XP else ""
            print("      %s%s" % (symbol, mark))
    print()
    if problems:
        print("THE GATE FAILS:")
        for problem in problems:
            print("  - %s" % problem)
        return 1
    print("THE GATE PASSES: the table asks for nothing that XP SP3 lacks.")
    return 0


def main():
    if len(sys.argv) != 2:
        raise SystemExit(__doc__)
    try:
        return check(sys.argv[1])
    except PeError as error:
        print("cannot read the PE: %s" % error)
        return 2


if __name__ == "__main__":
    sys.exit(main())
