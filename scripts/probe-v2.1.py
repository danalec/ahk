"""Probe the runner for the mechanisms of the v2.1, case by case.

The instrument behind `docs/ahk-v2.1-dialect.md`. Each case is a script of
its own, because a load error of one case must not hide the answers of the
others. The runner reads the script on stdin and answers a JSON report;
this prints one line per case and dialect: the output of the run, or the
first line of the refusal.

    python scripts/probe-v2.1.py [--ahk PATH] [--dialects v2,v2.1,ks]

The default binary is `target/debug/ahk.exe` beside this script's tree.
The debug build is slow -- a case that loops may need the runner's own
`--timeout` budget to finish inside the subprocess budget -- so each run
carries `--timeout=6` and the subprocess waits fifteen seconds. A case
that answers TIMEOUT is a case to re-run with a bigger budget, not a
refusal of the language.
"""
import argparse
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_AHK = os.path.join(HERE, "..", "target", "debug", "ahk.exe")

# (label, script, dialects) -- one construct per script.
CASES = [
    ("??= token", "z := unset\nz ??= 5\nMsgBox z", ("v2", "v2.1")),
    ("??= keeps a value", "x := 3\nx ??= 5\nMsgBox x", ("v2.1",)),
    ("??= on array item", "a := [unset]\na[1] ??= 9\nMsgBox a[1]", ("v2", "v2.1")),
    ("??= on present prop", "o := {a: 3}\no.a ??= 5\nMsgBox o.a", ("v2.1",)),
    ("??= on missing prop", "o := {}\no.a ??= 5\nMsgBox o.a", ("v2.1",)),
    ("?? expression left", "F() {\n    return unset\n}\nMsgBox (F() ?? \"none\")", ("v2", "v2.1")),
    ("?? chain left", "x := unset\nMsgBox (x?.y ?? \"d\")", ("v2.1",)),
    ("?? prop left (implicit maybe)", "o := {}\nMsgBox (o.missing ?? \"d\")", ("v2", "v2.1")),
    ("optional chain ?.", "x := unset\nMsgBox (x?.a is unset)", ("v2", "v2.1")),
    ("optional chain ?[", "x := unset\nMsgBox (x?[1] is unset)", ("v2", "v2.1")),
    ("optional chain ?(", "F(a?) {\n    return a\n}\nMsgBox (F()?.z is unset)", ("v2", "v2.1")),
    ("postfix (x?)", "x := unset\nMsgBox ((x?) is unset)", ("v2", "v2.1")),
    ("%name?% dynamic", "n := unset\no := {X: 1}\nMsgBox (o.%n?% ?? \"none\")", ("v2", "v2.1")),
    ("chain does not cross ()", "h := {Prop: unset}\ntry {\n    v := (h?.Prop)?.Inner\n    MsgBox \"no throw\"\n} catch as e {\n    MsgBox Type(e)\n}", ("v2.1",)),
    ("short-circuit operator", "n := 0\nBump() {\n    global n\n    return ++n\n}\nx := unset\nr := x?.v + Bump()\nMsgBox IsSet(r) \" \" n", ("v2", "v2.1")),
    ("chain write skipped", "x := unset\nx?.a := 5\nMsgBox IsSet(x)", ("v2", "v2.1")),
    ("chain write present", "o := {a: {b: 1}}\no?.a.b := 7\nMsgBox o.a.b", ("v2", "v2.1")),
    ("final ? omits param", "F(a?) {\n    return IsSet(a)\n}\nx := unset\nMsgBox F((x?.y)?)", ("v2.1",)),
    ("x is unset", "x := unset\nMsgBox (x is unset)", ("v2", "v2.1")),
    ("x is unset (has value)", "x := 5\nMsgBox (x is unset)", ("v2.1",)),
    ("x = unset", "x := unset\nMsgBox (x = unset)", ("v2", "v2.1")),
    ("while + Until", "x := 1\nwhile true\n{\n    x++\n}\nUntil (A_Index == 5)\nMsgBox x", ("v2", "v2.1")),
    ("Loop + Until", "x := 1\ny := 20\nLoop\n{\n    x *= 2\n}\nUntil x > y\nMsgBox x", ("v2", "v2.1")),
    ("for + Until", "x := 0\nfor v in [1,2,3]\n{\n    x++\n}\nUntil v = 2\nMsgBox x", ("v2", "v2.1")),
    ("!~= operator", "MsgBox (\"abc\" !~= \"\\d\")", ("v2", "v2.1")),
    ("mandatory after optional", "F(a?, b) {\n    return b\n}\nMsgBox F(, \"B\")", ("v2", "v2.1")),
    ("function def expr (named)", "x := (f() {\n    return 5\n})()\nMsgBox x", ("v2.1",)),
    ("function def expr (anon)", "x := (() {\n    return 7\n})()\nMsgBox x", ("v2.1",)),
    ("return unset", "F() {\n    return unset\n}\nMsgBox (F() is unset)", ("v2", "v2.1")),
    ("no-value return", "F() {\n}\nMsgBox (F() is unset)", ("v2.1",)),
    ("arrow tail unset", "F() {\n}\nG() => F()\nMsgBox (G() is unset)", ("v2.1",)),
    ("object literal unset prop", "o := {prop: unset}\nMsgBox ObjOwnPropCount(o)", ("v2", "v2.1")),
    ("array literal unset elem", "a := [unset]\nMsgBox a.Length", ("v2", "v2.1")),
    ("prop := unset then read", "o := {a: 1}\no.a := unset\ntry {\n    v := o.a\n    MsgBox \"read ok\"\n} catch as e {\n    MsgBox \"throw: \" Type(e)\n}\nMsgBox \"count \" ObjOwnPropCount(o)", ("v2", "v2.1")),
    ("map item := unset", "m := Map(\"k\", 1)\nm[\"k\"] := unset\nMsgBox \"has \" m.Has(\"k\") \" count \" m.Count", ("v2", "v2.1")),
    ("setter value unset", "class C {\n    stored := 0\n    p {\n        set {\n            this.stored := IsSet(value) ? 1 : 0\n        }\n    }\n}\nc := C()\nc.p := unset\nMsgBox c.stored", ("v2.1",)),
    ("IsSet(member expr)", "o := {}\nMsgBox IsSet(o.missing)", ("v2", "v2.1")),
    ("IsSet(chain)", "x := unset\nMsgBox IsSet(x?.a)", ("v2", "v2.1")),
    ("Type(x?)", "x := unset\nMsgBox Type(x?)", ("v2", "v2.1")),
    ("Type() omitted", "MsgBox Type()", ("v2.1",)),
    ("assign unset to var", "x := 5\nx := unset\nMsgBox IsSet(x)", ("v2", "v2.1")),
    ("Map.Get absent", "m := Map()\ntry {\n    v := m.Get(\"k\")\n    MsgBox \"no throw\"\n} catch as e {\n    MsgBox Type(e)\n}", ("v2.1",)),
    ("Array.Get absent", "a := [1]\ntry {\n    v := a.Get(5)\n    MsgBox \"no throw\"\n} catch as e {\n    MsgBox Type(e)\n}", ("v2.1",)),
    ("#Module", "#Module M\nMsgBox \"hi\"", ("v2.1",)),
    ("#Import", "#Import M\nMsgBox \"hi\"", ("v2.1",)),
    ("struct definition", "struct POINT {\n    x : Int32, y : Int32\n}\np := POINT()\nMsgBox p.Size", ("v2.1",)),
    ("struct as a name (v2)", "struct := 5\nMsgBox struct", ("v2",)),
    ("Props function", "o := {a: 1}\nfor k, v in Props(o)\n    MsgBox k", ("v2.1",)),
    ("o.Props method", "o := {a: 1}\nfor k, v in o.Props()\n    MsgBox k", ("v2.1",)),
    ("PropRef class", "o := {a: 1}\nr := PropRef(o, \"a\")\nMsgBox %r%", ("v2.1", "ks")),
    ("&prop (virtual ref)", "o := {a: 1}\nr := &o.a\n%r% := 9\nMsgBox o.a", ("v2.1",)),
    ("DefineProp function", "o := {}\nDefineProp(o, \"a\", {value: 5})\nMsgBox o.a", ("v2.1",)),
    ("Class(BaseClass)", "C := Class(Map)\nMsgBox Type(C())", ("v2.1",)),
    ("ATan2", "MsgBox ATan2(1, 1)", ("v2", "v2.1", "ks")),
    ("WinGetEnabled", "MsgBox WinGetEnabled(\"A\")", ("v2.1", "ks")),
    ("A_HotIf", "MsgBox A_HotIf", ("v2", "v2.1", "ks")),
    ("A_KeybdHookInstalled", "MsgBox A_KeybdHookInstalled", ("v2.1", "ks")),
    ("A_AhkVersion", "MsgBox A_AhkVersion", ("v2", "v2.1", "ks", "ks20")),
    ("#Requires v2.1", "#Requires AutoHotkey v2.1\nMsgBox \"met\"", ("v2", "v2.1")),
    ("#Requires v2.0 in v2.1", "#Requires AutoHotkey v2.0\nMsgBox \"met\"", ("v2.1",)),
    ("F(a?) is v2.0", "F(a?) {\n    return IsSet(a)\n}\nMsgBox F() \" \" F(1)", ("v2", "v2.1")),
    ("var? pass-through is v2.0", "F(a?) {\n    return IsSet(a)\n}\nx := unset\nMsgBox F(x?)", ("v2",)),
    ("?? var left is v2.0", "x := unset\nMsgBox x ?? 5", ("v2",)),
]


def run(ahk, script, dialect):
    try:
        p = subprocess.run(
            [ahk, f"--dialect={dialect}", "--timeout=6"],
            input=script.encode("utf-8"),
            capture_output=True,
            timeout=15,
        )
    except subprocess.TimeoutExpired:
        return {"ok": False, "error": "TIMEOUT (subprocess budget; re-run with a bigger one)"}
    try:
        return json.loads(p.stdout.decode("utf-8"))
    except Exception:
        return {"ok": False, "error": f"(no report) rc={p.returncode}"}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--ahk", default=DEFAULT_AHK, help="the runner to probe")
    ap.add_argument("--dialects", default=None, help="comma list; overrides the dialects of each case")
    args = ap.parse_args()
    override = tuple(args.dialects.split(",")) if args.dialects else None

    for label, script, dialects in CASES:
        for d in override or dialects:
            r = run(args.ahk, script, d)
            if r.get("ok"):
                ans = " | ".join(r.get("output", [])) or "(empty)"
                print(f"{label:38} {d:5} OK   {ans}")
            else:
                err = (r.get("error") or "").split("\n")[0][:110]
                print(f"{label:38} {d:5} FAIL {err}")


if __name__ == "__main__":
    sys.exit(main())
