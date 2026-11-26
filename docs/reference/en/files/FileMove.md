# `FileMove`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Moves a file, or every file that a pattern names. The source and the destination are the ones of `FileCopy`.

## Syntax

```ahk
FileMove Source, Dest, Overwrite
```

## Parameters

- **Source** — Path/wildcard.
- **Dest** — Destination.
- **Overwrite** — 1 to overwrite.

## Return value

nothing

## Errors

Error `Failed` when the source is not there, or when the destination is there and the overwrite is not asked for.

## Example

```ahk
FileMove "a.txt", "b.txt"
```

## Related

[`FileCopy`](FileCopy.md), [`DirMove`](DirMove.md)
