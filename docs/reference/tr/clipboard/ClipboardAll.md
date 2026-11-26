# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · class · Pano

## Açıklama

Reads or writes the whole clipboard (all formats) as binary data.

## Sözdizimi

```ahk
Data := ClipboardAll()
```

## Parametreler

?.

## Dönüş değeri

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## Notlar

On Linux the clipboard carries text; the bytes are the text's bytes.

## Örnek

```ahk
saved := ClipboardAll()
```

## İlgili

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
