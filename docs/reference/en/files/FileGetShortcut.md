# `FileGetShortcut`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Reads a shortcut's fields.

## Syntax

```ahk
FileGetShortcut Path, OutTarget, OutDir, OutArgs, OutDesc, OutIcon, OutIconNum, OutRunState
```

## Parameters

1..255.

## Return value

Nothing; the vars are filled.

## Example

```ahk
FileGetShortcut "a.lnk", &t
```

## Related

[`FileCreateShortcut`](FileCreateShortcut.md)
