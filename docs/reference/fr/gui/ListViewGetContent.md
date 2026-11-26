# `ListViewGetContent`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Reads the content of a ListView.

## Syntaxe

```ahk
S := ListViewGetContent(Options, Control, WinTitle)
```

## Paramètres

0..6.

## Valeur de retour

The text.

## Exemple

```ahk
ListViewGetContent("", "SysListView321")
```

## Voir aussi

[`Gui`](Gui.md), [`ControlGetText`](ControlGetText.md)
