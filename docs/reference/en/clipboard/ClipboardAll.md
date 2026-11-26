# `ClipboardAll`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*AutoHotkey v2.0.31* · class · The clipboard

## Description

Reads or writes the whole clipboard (all formats) as binary data.

## Syntax

```ahk
Data := ClipboardAll()
```

## Parameters

?.

## Return value

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## Remarks

On Linux the clipboard carries text; the bytes are the text's bytes.

## Example

```ahk
saved := ClipboardAll()
```

## Related

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
