# `ClipboardAll`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v2.0.31* · class · A área de transferência

## Descrição

Reads or writes the whole clipboard (all formats) as binary data.

## Sintaxe

```ahk
Data := ClipboardAll()
```

## Parâmetros

?.

## Valor de retorno

A Buffer of the clipboard's serialized data; assigning it restores the clipboard.

## Observações

On Linux the clipboard carries text; the bytes are the text's bytes.

## Exemplo

```ahk
saved := ClipboardAll()
```

## Relacionados

[`OnClipboardChange`](OnClipboardChange.md), [`A_Clipboard`](A_Clipboard.md)
