# `WinClose`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Closes a window. It closes the window of the script itself: the listing of the compositor publishes no close request for a window of another program.

## Syntaxe

```ahk
WinClose WinTitle, WinText, SecondsToWait
```

## Paramètres

- **WinTitle** — The criteria.
- **SecondsToWait** — Then force.

## Valeur de retour

nothing

## Erreurs

a window that is not there answers 'Target window not found.'; a window of another program cannot be closed this way.

## Exemple

```ahk
WinClose "Untitled"
```

## Voir aussi

[`WinKill`](WinKill.md), [`WinActivate`](WinActivate.md)
