# `InStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · 言語の中核: テキスト、数値、正規表現

## 説明

Where a text is in another text. The position counts from one and is zero when the needle is not there at all.

## 構文

```ahk
Pos := InStr(Haystack, Needle, CaseSense, Start, Occurrence)
```

## パラメータ

- **Haystack** — The text to search.
- **Needle** — The text to find.
- **CaseSense** — On/Off/Locale.
- **Start** — 1-based, negative from end.
- **Occurrence** — Which hit.

## 戻り値

the position of the match, or 0 when there is none

## エラー

a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.

## 備考

The sensitive search uses the vectorized substring scan (see docs/simd.md).

## 例

```ahk
InStr("hello", "ll")
```

## 関連

[`StrReplace`](StrReplace.md), [`SubStr`](SubStr.md), [`RegExMatch`](RegExMatch.md)
