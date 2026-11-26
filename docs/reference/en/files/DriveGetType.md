# `DriveGetType`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · command · File and directory operations

## Description

The type of a drive (Fixed/Removable/CDROM/...).

## Syntax

```ahk
S := DriveGetType(Path)
```

## Parameters

1.

## Return value

The type.

## Example

```ahk
DriveGetType("C:\")
```

## Related

[`DriveGet`](DriveGet.md)
