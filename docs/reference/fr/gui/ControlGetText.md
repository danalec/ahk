# `ControlGetText`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Reads the text of a control.

## Syntaxe

```ahk
T := ControlGetText(Control, WinTitle)
```

## Paramètres

- **Control** — ClassNN.
- **WinTitle** — The window.

## Valeur de retour

The text.

## Exemple

```ahk
t := ControlGetText("Edit1", "ahk_class Notepad")
```

## Voir aussi

[`ControlSetText`](ControlSetText.md), [`WinGetText`](WinGetText.md)
