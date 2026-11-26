# `StrLen`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

Counts the characters of a text in the code units of UTF-16, so a character outside the basic plane counts as two.

## Syntax

```ahk
Count := StrLen(String)
```

## Parameters

- **String** — The text.

## Return value

the number of code units

## Errors

none; a value that is not text is read as its text.

## Remarks

Astral characters count as one each on this tree.

## Example

```ahk
StrLen("hello")
```

## Related

[`SubStr`](SubStr.md), [`InStr`](InStr.md)
