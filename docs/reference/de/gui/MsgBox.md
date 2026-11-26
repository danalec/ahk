# `MsgBox`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v1.1.37.02* · command · GUI-Fenster und Fensterautomatisierung

## Beschreibung

Shows a message. The tree has no window of its own, so the text goes into the output of the report instead of onto the screen, and the call answers the same text.

## Syntax

```ahk
Result := MsgBox(Text, Title, Options)
```

## Parameter

- **Text** — The message. If omitted, an empty box.
- **Title** — The title of the window. Defaults to the script's name.
- **Options** — A number or a text of icons/buttons: icon (16/32/48/64), buttons (0..5), default button, modality, timeout, and the owner.

## Rückgabewert

the text of the message

## Fehler

more than one parameter is refused with "the options, the title and the variable that takes the answer are not carried yet."

## Bemerkungen

On the Linux box the tree has no window of its own in a headless run: the text goes to the report output and the call answers the text. On Windows it is the MessageBox of user32. A timeout answers Timeout.

## Beispiel

```ahk
MsgBox "hello"
```

## Verwandt

[`InputBox`](InputBox.md), [`ToolTip`](ToolTip.md), [`TrayTip`](TrayTip.md)
