# `Ord`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

The code point of the first character of a text.

## Syntax

```ahk
N := Ord(String)
```

## Parameters

- **String** — The text.

## Return value

the code point, or zero when the text has no character

## Errors

none; a lone surrogate answers with the code unit of it.

## Example

```ahk
Ord("A")
```

## Related

[`Chr`](Chr.md), [`Asc`](Asc.md)
