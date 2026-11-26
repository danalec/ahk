# `Chr`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · function · The core of the language: text, math and patterns

## Description

The character with a code point.

## Syntax

```ahk
S := Chr(Number)
```

## Parameters

- **Number** — The code point.

## Return value

the text of that character

## Errors

a code outside 0 to 0x10FFFF is an invalid parameter; a surrogate code is a text of its own.

## Example

```ahk
Chr(65)
```

## Related

[`Ord`](Ord.md), [`Asc`](Asc.md)
