# `IsNumber`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · function · The core of the language: text, math and patterns

## Description

Whether a value is a number: an integer, a float, or a text that holds one. A hexadecimal text is a number too.

## Syntax

```ahk
B := IsNumber(Value)
```

## Parameters

- **Value** — Any value.

## Return value

1 when it is a number, 0 when it is not

## Errors

a second parameter is refused with "the code page of the text is not carried yet.".

## Example

```ahk
IsNumber("42")
```

## Related

[`IsInteger`](IsInteger.md), [`IsFloat`](IsFloat.md)
