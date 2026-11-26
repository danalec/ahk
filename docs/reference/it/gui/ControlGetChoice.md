# `ControlGetChoice`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · command · Finestre GUI e automazione delle finestre

## Descrizione

Reads the chosen item of a list.

## Sintassi

```ahk
S := ControlGetChoice(Control, WinTitle)
```

## Parametri

1..5.

## Valore di ritorno

The text.

## Esempio

```ahk
ControlGetChoice("ComboBox1")
```

## Correlati

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
