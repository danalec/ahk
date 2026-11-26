# `StrReplace`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The text with one part of it swapped for another. The case sense says whether the letters are taken as they are, and it is off when the caller does not say.

## 구문

```ahk
New := StrReplace(Haystack, Search, Replace, CaseSense, OutputVarCount, Limit)
```

## 매개변수

- **Haystack** — The text.
- **Search** — What to find.
- **Replace** — What to put (default empty).
- **CaseSense** — On/Off.
- **OutputVarCount** — Receives the number of replacements.
- **Limit** — Max replacements (-1 all).

## 반환 값

the text with the replacements

## 오류

the count must be a variable reference; a needle that is empty changes nothing and the text comes back as it is; the logical case sense is an invalid parameter.

## 설명

An empty Search is refused (Parameter #2 invalid).

## 예제

```ahk
StrReplace("a-b-c", "-", "+")
```

## 관련

[`InStr`](InStr.md), [`RegExReplace`](RegExReplace.md), [`StrSplit`](StrSplit.md)
