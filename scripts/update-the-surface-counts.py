"""Atualiza os numeros da superficie da arvore depois do COM.

A arvore agora responde as duas classes do COM -- `ComObject` e
`ComValue` -- e a superficie grows: o v2 tem 820 nomes que respondem e
13 que recusam, e o v1 tem 311 e 97.

O script troca os numeros nos lugares onde a superficie e' escrita, e
diz o que trocou.
"""

import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent

# (o padrao, a troca)
THE_TREES = [
    # A onda dos ultimos quinze: o v2 passa a responder com o nome inteiro,
    # e o `Reload` e o `Thread` deixam de recusar.
    ("| 833 | 831 | 2 | 0 |", "| 833 | 833 | 0 | 0 |"),
    ('"values": 831, "refusals": 2', '"values": 833, "refusals": 0'),
    ("**833** entries: 831 answer with a value, 2 are refused", "**833** entries: 833 answer with a value, 0 are refused"),
    ("**833** entries: 833 answer with a value, none are refused", "**833** entries: 833 answer with a value, 0 are refused"),
    ("**831** answer with a value**, 2 are refused", "**833** answer with a value**, 0 are refused"),
    ("831 answer with a value, 2 are refused", "833 answer with a value, 0 are refused"),
    ("833 answer with a value, none are refused", "833 answer with a value, 0 are refused"),
    ("833 entries: 831 answer with a value, 2 are refused", "833 entries: 833 answer with a value, 0 are refused"),
    ("833 entries: 833 answer with a value, none are refused", "833 entries: 833 answer with a value, 0 are refused"),
    ("**831** values, **2** refusals", "**833** values, **0** refusals"),
    ("**833** values, **0** refusals", "**833** values, **0** refusals"),
    ("**831** | **2**", "**833** | **0**"),
    ("with **831**", "with **833**"),
    ("| Answer with a value | 831 | 311 |", "| Answer with a value | 833 | 311 |"),
    ("the measurement of today is 831/2", "the measurement of today is 833/0"),
    ("831/2 today", "833/0 today"),
    ("today 831 of 833 entries", "today 833 of 833 entries"),
    ("Of the 833 entries, 831 answer with a value and 2 refuse", "Of the 833 entries, 833 answer with a value and none refuses"),
    # A onda 1 dos ultimos quinze: o `A_Cursor` e o `Edit` passam a responder,
    # e o v2 fica com 831 que respondem e 2 que recusam.
    ("| 833 | 820 | 13 | 0 |", "| 833 | 831 | 2 | 0 |"),
    ('"values": 820, "refusals": 13', '"values": 831, "refusals": 2'),
    ("**833** entries: 820 answer with a value, 13 are refused", "**833** entries: 831 answer with a value, 2 are refused"),
    ("**820** answer with a value**, 13 are refused", "**831** answer with a value**, 2 are refused"),
    ("820 answer with a value, 13 are refused", "831 answer with a value, 2 are refused"),
    ("833 entries: 820 answer with a value, 13 are refused", "833 entries: 831 answer with a value, 2 are refused"),
    ("**820** values, **13** refusals", "**831** values, **2** refusals"),
    ("**820** | **13**", "**831** | **2**"),
    ("with **820**", "with **831**"),
    ("| Answer with a value | 820 | 311 |", "| Answer with a value | 831 | 311 |"),
    ("the measurement of today is 820/13", "the measurement of today is 831/2"),
    ("820/13 today", "831/2 today"),
    ("today 820 of 833 entries", "today 831 of 833 entries"),
    ("Of the 833 entries, 820 answer with a value and 13 refuse", "Of the 833 entries, 831 answer with a value and 2 refuse"),
    # A mudanca anterior: o COM.
    ("| 833 | 818 | 15 | 0 |", "| 833 | 820 | 13 | 0 |"),
    ("| 408 | 309 | 99 | 0 |", "| 408 | 311 | 97 | 0 |"),
    ('"values": 818, "refusals": 15', '"values": 820, "refusals": 13'),
    ('"values": 309, "refusals": 99', '"values": 311, "refusals": 97'),
    ("**833** entries: 818 answer with a value, 15 are refused", "**833** entries: 820 answer with a value, 13 are refused"),
    ("**818** answer with a value**, 15 are refused", "**820** answer with a value**, 13 are refused"),
    ("818 answer with a value, 15 are refused", "820 answer with a value, 13 are refused"),
    ("833 entries: 818 answer with a value, 15 are refused", "833 entries: 820 answer with a value, 13 are refused"),
    ("**818** values, **15** refusals", "**820** values, **13** refusals"),
    ("**818** | **15**", "**820** | **13**"),
    ("with **818**", "with **820**"),
    ("| Answer with a value | 818 | 309 |", "| Answer with a value | 820 | 311 |"),
    ("the measurement of today is 818/15", "the measurement of today is 820/13"),
    ("818/15 today", "820/13 today"),
    ("today 818 of 833 entries", "today 820 of 833 entries"),
    ("Of the 833 entries, 818 answer with a value and 15 refuse", "Of the 833 entries, 820 answer with a value and 13 refuse"),
    ("**309** answer with a value", "**311** answer with a value"),
]

# Os arquivos onde a superficie e' escrita.
WHERE = [
    "crates/ahk-cli/tests/runner.rs",
    "docs/ahk-gaps-report.md",
    "docs/ahk-v1-dialect.md",
    "docs/ahk-v2-reference.md",
    "docs/ahk-x11-comparison.md",
    "docs/dialects.md",
    "docs/runner-and-report.md",
    "docs/v2-capability-report.md",
]
WHERE += [str(path.relative_to(ROOT)) for path in sorted((ROOT / "docs/surface").glob("*/v1.md"))]
WHERE += [str(path.relative_to(ROOT)) for path in sorted((ROOT / "docs/surface").glob("*/v2.md"))]
WHERE += ["docs/surface/v1.md", "docs/surface/v2.md"]


def main() -> int:
    changed = []
    for name in WHERE:
        path = ROOT / name
        if not path.exists():
            print(f"o arquivo {name} não está lá", file=sys.stderr)
            continue
        text = path.read_text(encoding="utf-8")
        original = text
        for old, new in THE_TREES:
            text = text.replace(old, new)
        if text != original:
            path.write_text(text, encoding="utf-8")
            changed.append(name)
    print(f"{len(changed)} arquivos mudaram")
    for name in changed:
        print(f"  {name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
