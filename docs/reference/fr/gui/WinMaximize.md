# `WinMaximize`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Maximizes the window the script made, and only that one; the helper of the box is the one that knows how to do it.

## Syntaxe

```ahk
WinMaximize …
```

## Paramètres

WinTitle

## Valeur de retour

the empty text

## Erreurs

"Target window not found." when the script has no window of its own, or when the title does not match the one it has.

## Exemple

```ahk
WinMaximize "MyWindow"
```
