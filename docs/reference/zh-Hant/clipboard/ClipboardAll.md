# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · class · 剪貼簿

## 描述

Reads or writes the whole clipboard (all formats) as binary data.

## 語法

```ahk
Data := ClipboardAll()
```

## 參數

?.

## 傳回值

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## 備註

On Linux the clipboard carries text; the bytes are the text's bytes.

## 範例

```ahk
saved := ClipboardAll()
```

## 相關

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
