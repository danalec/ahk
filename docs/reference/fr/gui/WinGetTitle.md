# `WinGetTitle`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

The title of the active window, or of the window that matches the text that was given.

## Syntaxe

```ahk
T := WinGetTitle(WinTitle, WinText)
```

## Paramètres

- **WinTitle** — The criteria; omitted = last found.

## Valeur de retour

the title of the window

## Erreurs

a window that was named and is not there answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Exemple

```ahk
title := WinGetTitle()
```

## Voir aussi

[`WinGetText`](WinGetText.md), [`WinGetPos`](WinGetPos.md)
