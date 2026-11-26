# `WinSetStyle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Finestre GUI e automazione delle finestre

## Descrizione

Writes the style of a window. The style is absolute: the text may carry a `+`, a `-` or a `^` before the number, and the runtime computes the new style out of the one that is there -- `+0x10` adds the bits, `-0x10` takes them away, `^0x10` turns them around, and a text with no prefix replaces the style. The number is read the way the `ATOU` of the v2 reads it: hexadecimal when the text says "0x", decimal otherwise, and zero when the text holds no number.

## Sintassi

```ahk
WinSetStyle …
```

## Parametri

Style, WinTitle, [WinText], [ExcludeTitle], [ExcludeText]

## Valore di ritorno

the empty text

## Errori

an empty text refuses with 'Parameter #1 of WinSetStyle is invalid.', and 'empty string' is in the `Extra`; a text that holds no number is not a refusal -- it is read as zero, and the failure that comes is the one of the window.

## Esempio

```ahk
WinSetStyle("+0x10", "ZZZ_NO_WINDOW_ZZZ")
```
