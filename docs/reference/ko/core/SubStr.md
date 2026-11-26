# `SubStr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v1.1.37.02* · function · 언어의 핵심: 텍스트, 수학, 패턴

## 설명

The part of a text between two positions. The positions count from one, and a negative position counts from the end: -1 is the last character.

## 구문

```ahk
Part := SubStr(String, Start, Length)
```

## 매개변수

- **String** — The text.
- **Start** — 1-based; negative counts from the end.
- **Length** — Omitted = to the end.

## 반환 값

the text of that part; a negative length leaves that many characters out at the end

## 오류

none; the text is cut in the code units of UTF-16, so the second unit of a surrogate pair is a text of its own.

## 설명

Counts characters, not bytes.

## 예제

```ahk
SubStr("hello", 2, 3)
```

## 관련

[`StrLen`](StrLen.md), [`InStr`](InStr.md), [`StrSplit`](StrSplit.md)
