# `InStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · 語言核心：文字、數學與模式

## 描述

Where a text is in another text. The position counts from one and is zero when the needle is not there at all.

## 語法

```ahk
Pos := InStr(Haystack, Needle, CaseSense, Start, Occurrence)
```

## 參數

- **Haystack** — The text to search.
- **Needle** — The text to find.
- **CaseSense** — On/Off/Locale.
- **Start** — 1-based, negative from end.
- **Occurrence** — Which hit.

## 傳回值

the position of the match, or 0 when there is none

## 錯誤

a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.

## 備註

The sensitive search uses the vectorized substring scan (see docs/simd.md).

## 範例

```ahk
InStr("hello", "ll")
```

## 相關

[`StrReplace`](StrReplace.md), [`SubStr`](SubStr.md), [`RegExMatch`](RegExMatch.md)
