# `InStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

Where a text is in another text. The position counts from one and is zero when the needle is not there at all.

## 구문

```ahk
Pos := InStr(Haystack, Needle, CaseSense, Start, Occurrence)
```

## 매개변수

- **Haystack** — The text to search.
- **Needle** — The text to find.
- **CaseSense** — On/Off/Locale.
- **Start** — 1-based, negative from end.
- **Occurrence** — Which hit.

## 반환 값

the position of the match, or 0 when there is none

## 오류

a needle that is empty is an invalid parameter; the logical case sense is an invalid parameter; a start or an occurrence of zero is an invalid parameter.

## 설명

The sensitive search uses the vectorized substring scan (see docs/simd.md).

## 예제

```ahk
InStr("hello", "ll")
```

## 관련

[`StrReplace`](StrReplace.md), [`SubStr`](SubStr.md), [`RegExMatch`](RegExMatch.md)
