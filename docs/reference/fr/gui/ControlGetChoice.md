# `ControlGetChoice`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Reads the chosen item of a list.

## Syntaxe

```ahk
S := ControlGetChoice(Control, WinTitle)
```

## Paramètres

1..5.

## Valeur de retour

The text.

## Exemple

```ahk
ControlGetChoice("ComboBox1")
```

## Voir aussi

[`ControlSend`](ControlSend.md), [`Gui`](Gui.md)
