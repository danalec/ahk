# `WinMenuSelectItem`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Selects a menu item of a window.

## Syntaxe

```ahk
WinMenuSelectItem WinTitle, Menu, Items...
```

## Paramètres

3..255.

## Valeur de retour

Nothing.

## Exemple

```ahk
WinMenuSelectItem "A", , "File", "Open"
```

## Voir aussi

[`WinActivate`](WinActivate.md), [`WinExist`](WinExist.md)
