# `DirExist`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · File and directory operations

## Description

Answers the attributes of the first folder that the name or the pattern matches.

## Syntax

```ahk
A := DirExist(Path)
```

## Parameters

- **Path** — A path or wildcard.

## Return value

`D` when the folder is there, and the empty text when it is not; a file that matches the pattern is not a folder.

## Errors

nothing fails.

## Example

```ahk
x := DirExist("out")
```

## Related

[`FileExist`](FileExist.md)
