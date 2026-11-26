# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · class · Gli appunti

## Descrizione

Reads or writes the whole clipboard (all formats) as binary data.

## Sintassi

```ahk
Data := ClipboardAll()
```

## Parametri

?.

## Valore di ritorno

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## Osservazioni

On Linux the clipboard carries text; the bytes are the text's bytes.

## Esempio

```ahk
saved := ClipboardAll()
```

## Correlati

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
