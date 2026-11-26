#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Write the per-name reference in the thirteen languages of the surface.

    python scripts/gen-reference-docs.py          # write docs/reference/<lang>/

One page per name of every dialect, CHM-shaped: Description, Syntax,
Parameters, Return, Errors, Remarks, Example, Related. The scaffolding
(section headers, family titles, navigation) is translated like the surface
(surface_i18n); the per-name prose comes from the behaviour chunks in the
language that has it (en/zh/pt-BR) and from the authored knowledge base
(scripts/reference_kb.py, English) as the fallback -- the same way the
surface falls back to English for the behaviour of the other languages.

This is NOT a copy of the AutoHotkey CHM (GPLv2); it is authored for this
Apache-2.0 tree and adds the tree's own behaviour and cross-platform notes.
"""

import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))

import surface_areas  # noqa: E402
import surface_behavior  # noqa: E402
import surface_i18n  # noqa: E402
import reference_kb  # noqa: E402

OUT = ROOT / "docs" / "reference"

DIALECTS = [
    ("v2", "AutoHotkey v2.0.31", "ahk-v2.0.32.surface.tsv"),
    ("v1", "AutoHotkey v1.1.37.02", "ahk-v1.1.37.02.surface.tsv"),
    ("ks", "KeySharp", "ahk-ks.surface.tsv"),
]

# The section headers of a reference page, in the thirteen languages.
REF = {
    "en": ["Description", "Syntax", "Parameters", "Return value", "Errors", "Remarks", "Example", "Related", "The ficha above is the measured surface; the full prose for this name is being authored.", "The reference of every name", "One page per name of the three dialects: description, syntax, parameters, return, errors, remarks, example, related. Original to this tree, with its own behaviour and cross-platform notes."],
    "es": ["Descripción", "Sintaxis", "Parámetros", "Valor de retorno", "Errores", "Observaciones", "Ejemplo", "Relacionados", "La ficha anterior es la superficie medida; la prosa completa de este nombre está en redacción.", "La referencia de cada nombre", "Una página por nombre de los tres dialectos: descripción, sintaxis, parámetros, retorno, errores, observaciones, ejemplo, relacionados. Original de este árbol, con su propio comportamiento y notas multiplataforma."],
    "pt-BR": ["Descrição", "Sintaxe", "Parâmetros", "Valor de retorno", "Erros", "Observações", "Exemplo", "Relacionados", "A ficha acima é a superfície medida; a prosa completa deste nome está sendo redigida.", "A referência de cada nome", "Uma página por nome dos três dialetos: descrição, sintaxe, parâmetros, retorno, erros, observações, exemplo, relacionados. Original desta árvore, com o seu próprio comportamento e notas multiplataforma."],
    "fr": ["Description", "Syntaxe", "Paramètres", "Valeur de retour", "Erreurs", "Remarques", "Exemple", "Voir aussi", "La fiche ci-dessus est la surface mesurée ; la prose complète de ce nom est en cours de rédaction.", "La référence de chaque nom", "Une page par nom des trois dialectes : description, syntaxe, paramètres, retour, erreurs, remarques, exemple, voir aussi. Original de cet arbre, avec son propre comportement et des notes multiplateformes."],
    "it": ["Descrizione", "Sintassi", "Parametri", "Valore di ritorno", "Errori", "Osservazioni", "Esempio", "Correlati", "La scheda sopra è la superficie misurata; la prosa completa di questo nome è in stesura.", "Il riferimento di ogni nome", "Una pagina per nome dei tre dialetti: descrizione, sintassi, parametri, ritorno, errori, osservazioni, esempio, correlati. Originale di questo albero, con il proprio comportamento e note multipiattaforma."],
    "pl": ["Opis", "Składnia", "Parametry", "Wartość zwracana", "Błędy", "Uwagi", "Przykład", "Powiązane", "Powyższa ficha to zmierzona powierzchnia; pełna proza tej nazwy jest w opracowaniu.", "Referencja każdej nazwy", "Jedna strona na nazwę trzech dialektów: opis, składnia, parametry, zwrot, błędy, uwagi, przykład, powiązane. Oryginalne dla tego drzewa, z własnym zachowaniem i uwagami wieloplatformowymi."],
    "tr": ["Açıklama", "Sözdizimi", "Parametreler", "Dönüş değeri", "Hatalar", "Notlar", "Örnek", "İlgili", "Yukarıdaki ficha ölçülen yüzeydir; bu adın tam düzyazısı hazırlanmaktadır.", "Her adın referansı", "Üç lehçenin her adı için bir sayfa: açıklama, sözdizimi, parametreler, dönüş, hatalar, notlar, örnek, ilgili. Bu ağaca özgü, kendi davranışı ve platformlar arası notlarıyla."],
    "de": ["Beschreibung", "Syntax", "Parameter", "Rückgabewert", "Fehler", "Bemerkungen", "Beispiel", "Verwandt", "Die obige Ficha ist die gemessene Oberfläche; die vollständige Prosa dieses Namens wird verfasst.", "Die Referenz jedes Namens", "Eine Seite je Name der drei Dialekte: Beschreibung, Syntax, Parameter, Rückgabe, Fehler, Bemerkungen, Beispiel, Verwandt. Original dieses Baums, mit eigenem Verhalten und plattformübergreifenden Hinweisen."],
    "ru": ["Описание", "Синтаксис", "Параметры", "Возвращаемое значение", "Ошибки", "Замечания", "Пример", "См. также", "Приведённая ficha — измеренная поверхность; полная проза этого имени готовится.", "Справочник по каждому имени", "По странице на имя трёх диалектов: описание, синтаксис, параметры, возврат, ошибки, замечания, пример, см. также. Оригинально для этого дерева, с собственным поведением и кроссплатформенными заметками."],
    "ja": ["説明", "構文", "パラメータ", "戻り値", "エラー", "備考", "例", "関連", "上記の ficha は計測された表面です。この名前の完全な散文は執筆中です。", "各名前のリファレンス", "3つの方言の各名前に1ページ：説明、構文、パラメータ、戻り値、エラー、備考、例、関連。このツリー独自のもので、自身の動作とクロスプラットフォームの注記を含みます。"],
    "zh": ["描述", "语法", "参数", "返回值", "错误", "备注", "示例", "相关", "上面的 ficha 是测得的表面；此名的完整散文正在撰写。", "每个名字的参考", "三个方言的每个名字一页：描述、语法、参数、返回值、错误、备注、示例、相关。本树原创，含自身行为与跨平台说明。"],
    "zh-Hant": ["描述", "語法", "參數", "傳回值", "錯誤", "備註", "範例", "相關", "上面的 ficha 是量測的表面；此名的完整散文正在撰寫。", "每個名字的參考", "三個方言的每個名字一頁：描述、語法、參數、傳回值、錯誤、備註、範例、相關。本樹原創，含自身行為與跨平台說明。"],
    "ko": ["설명", "구문", "매개변수", "반환 값", "오류", "설명", "예제", "관련", "위 ficha는 측정된 표면입니다. 이 이름의 전체 산문은 작성 중입니다.", "모든 이름의 참조", "세 방언의 각 이름마다 한 페이지: 설명, 구문, 매개변수, 반환, 오류, 설명, 예제, 관련. 이 트리의 독자적인 것으로, 자체 동작과 크로스 플랫폼 노트를 포함합니다."],
}


def read_surface(path):
    header = None
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line.strip() or line.startswith("#"):
            continue
        f = line.split("\t")
        if header is None:
            header = f
            continue
        rows.append(dict(zip(header, f)))
    return rows


def safe(name):
    return name.replace(".", "_").replace(" ", "_").replace("/", "_")


def nav(lang):
    cells = " | ".join(
        f"[{surface_i18n.LANG_NAMES[l]}]({'../' + l + '/' if l != lang else ''}index.md)"
        for l in surface_i18n.LANGS
    )
    return "| " + cells + " |\n\n" + surface_i18n.LANG_NAMES[lang] + "\n"


def page(row, dialect, label, lang):
    name = row["name"]
    kind = row.get("kind", "")
    klass = row.get("class", "")
    kb = reference_kb.KB.get(name)
    beh = surface_behavior.behavior_of(name, lang)
    family = surface_areas.area_of(name, dialect, klass) or "misc"
    title = surface_i18n.family_title(lang, family)
    H = REF[lang]

    desc = (beh or {}).get("summary") or (kb or {}).get("desc") or \
        f"A {kind} of {label}" + (f", of the class `{klass}`." if klass else ".")
    syntax = (kb or {}).get("syntax") or f"{name} " + ("…" if row.get("max") else "")
    if kb and kb.get("params"):
        params = "\n".join(f"- **{n}** — {t}" for n, t in kb["params"])
    elif beh and beh.get("params"):
        params = beh["params"]
    else:
        lo, hi = row.get("min", ""), row.get("max", "")
        params = (f"{lo}..{hi}." if lo and hi and lo != hi else f"{lo or hi or '?'}.")
    returns = (beh or {}).get("returns") or (kb or {}).get("returns") or "—"
    errors = (beh or {}).get("errors") or (kb or {}).get("errors")
    remarks = (kb or {}).get("remarks")
    example = (beh or {}).get("example") or (kb or {}).get("example")
    related = (kb or {}).get("related") or []

    L = [f"# `{name}`\n", nav(lang),
         f"*{label}* · {kind}" + (f" · `{klass}`" if klass else "") + f" · {title}\n",
         f"## {H[0]}\n\n{desc}\n",
         f"## {H[1]}\n\n```ahk\n{syntax}\n```\n",
         f"## {H[2]}\n\n{params}\n",
         f"## {H[3]}\n\n{returns}\n"]
    if errors:
        L.append(f"## {H[4]}\n\n{errors}\n")
    if remarks:
        L.append(f"## {H[5]}\n\n{remarks}\n")
    if example:
        L.append(f"## {H[6]}\n\n```ahk\n{example}\n```\n")
    if related:
        L.append(f"## {H[7]}\n\n" + ", ".join(f"[`{r}`]({safe(r)}.md)" for r in related) + "\n")
    if not kb and not beh:
        L.append("---\n\n*" + H[8] + "*\n")
    return family, "\n".join(L)


def main():
    # The earlier monolingual layout (root family dirs) is replaced by the
    # per-language trees; remove it so no stale page is left.
    for old in OUT.iterdir() if OUT.exists() else []:
        if old.is_dir() and old.name not in surface_i18n.LANGS:
            shutil.rmtree(old)
    OUT.mkdir(parents=True, exist_ok=True)

    surfaces = {d: read_surface(ROOT / "crates" / "ahk" / "reference" / t) for d, _, t in DIALECTS}

    for lang in surface_i18n.LANGS:
        index = {}
        for dialect, label, _ in DIALECTS:
            for row in surfaces[dialect]:
                family, text = page(row, dialect, label, lang)
                d = OUT / lang / family
                d.mkdir(parents=True, exist_ok=True)
                (d / (safe(row["name"]) + ".md")).write_text(text, encoding="utf-8")
                index.setdefault(family, {}).setdefault(row["name"], []).append(dialect)
        H = REF[lang]
        L = [f"# {H[9]}\n", nav(lang), H[10] + "\n"]
        for family in sorted(index):
            L.append(f"\n## {surface_i18n.family_title(lang, family)}\n")
            for name in sorted(index[family]):
                ds = ", ".join(index[family][name])
                L.append(f"- [`{name}`]({family}/{safe(name)}.md) — {ds}")
        (OUT / lang / "index.md").write_text("\n".join(L) + "\n", encoding="utf-8")

    # The root index is the language selector (English tree is en/).
    sel = ["# The reference of every name\n",
           "| " + " | ".join(f"[{surface_i18n.LANG_NAMES[l]}]({l}/index.md)" for l in surface_i18n.LANGS) + " |\n",
           "\nOne page per name of the three dialects, in the thirteen languages of the surface.\n"]
    (OUT / "index.md").write_text("\n".join(sel), encoding="utf-8")
    print("wrote", OUT, "langs:", len(surface_i18n.LANGS),
          "names:", {d: len(r) for d, r in surfaces.items()})


if __name__ == "__main__":
    main()
