# `SendMessage`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Sends a message to a window and waits.

## Syntaxe

```ahk
R := SendMessage(Msg, wParam, lParam, Control, WinTitle, Timeout)
```

## Paramètres

1..9.

## Valeur de retour

The result.

## Exemple

```ahk
SendMessage(0x100, 0, 0)
```

## Voir aussi

[`Menu`](Menu.md), [`Gui`](Gui.md), [`OnMessage`](OnMessage.md)
