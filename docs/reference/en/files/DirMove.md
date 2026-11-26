# `DirMove`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · File and directory operations

## Description

Moves a directory, with everything in it. A destination that is there is merged with the one that comes: the files of the source win.

## Syntax

```ahk
DirMove …
```

## Parameters

Source, Dest, [Overwrite]

## Return value

nothing

## Errors

Error `Failed` when the source is not a directory; a name that holds a wildcard is refused, and the reference fails with `Failed` on a wildcard too.

## Example

```ahk
DirMove "src", "dst"
```
