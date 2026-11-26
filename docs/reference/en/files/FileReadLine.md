# `FileReadLine`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Reads a line by number (v1).

## Syntax

```ahk
FileReadLine Out, Filename, LineNum
```

## Parameters

3..255.

## Return value

Nothing.

## Example

```ahk
FileReadLine s, "a.txt", 2
```

## Related

[`FileRead`](FileRead.md), [`LoopRead`](LoopRead.md)
