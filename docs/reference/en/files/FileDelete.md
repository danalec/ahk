# `FileDelete`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v1.1.37.02* · command · File and directory operations

## Description

Takes a file away, or every file that a pattern names. A pattern that matches nothing is not a failure.

## Syntax

```ahk
FileDelete Pattern
```

## Parameters

- **Pattern** — A path or wildcard.

## Return value

nothing

## Errors

OSError when the file cannot be taken away.

## Example

```ahk
FileDelete "out.txt"
```

## Related

[`DirDelete`](DirDelete.md), [`FileRecycle`](FileRecycle.md)
