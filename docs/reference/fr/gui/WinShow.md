# `WinShow`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Puts the window of the script back on the screen, after a `WinHide`.

## Syntaxe

```ahk
WinShow …
```

## Paramètres

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valeur de retour

nothing

## Erreurs

a window that is not there answers 'Target window not found.'; a window of another program cannot be shown this way.

## Exemple

```ahk
WinShow "Untitled"
```
