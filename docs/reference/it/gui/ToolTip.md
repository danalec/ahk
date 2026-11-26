# `ToolTip`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · command · Finestre GUI e automazione delle finestre

## Descrizione

Shows a small window of text at a place of the screen. Called with no text it takes the window away.

## Sintassi

```ahk
ToolTip Text, X, Y, WhichToolTip
```

## Parametri

- **Text** — The text; omitted hides.
- **X/Y** — Position.
- **WhichToolTip** — 1..20.

## Valore di ritorno

the number that identifies the window, or the empty text when the window is taken away

## Errori

the desktop reports the failure of the call.

## Esempio

```ahk
ToolTip "hello", 10, 10
```

## Correlati

[`TrayTip`](TrayTip.md), [`MsgBox`](MsgBox.md)
