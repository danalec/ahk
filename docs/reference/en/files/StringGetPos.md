# `StringGetPos`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

The position of a substring (v1, 0-based).

## Syntax

```ahk
StringGetPos Out, Haystack, Needle
```

## Parameters

3..255.

## Return value

Nothing; 0-based pos or -1.

## Example

```ahk
StringGetPos p, s, "x"
```

## Related

[`InStr`](InStr.md)
