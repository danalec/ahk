# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · class · クリップボード

## 説明

Reads or writes the whole clipboard (all formats) as binary data.

## 構文

```ahk
Data := ClipboardAll()
```

## パラメータ

?.

## 戻り値

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## 備考

On Linux the clipboard carries text; the bytes are the text's bytes.

## 例

```ahk
saved := ClipboardAll()
```

## 関連

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
