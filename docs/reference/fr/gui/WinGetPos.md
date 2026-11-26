# `WinGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

Where the window is and how big it is, as the compositor reports it.

## Syntaxe

```ahk
WinGetPos X, Y, W, H, WinTitle
```

## Paramètres

- **Output*** — Variables.
- **WinTitle** — The criteria.

## Valeur de retour

nothing; the place and the size are written into the variables of the caller

## Erreurs

the text of the window and the exclusions are not carried yet; a window that is not there answers 'Target window not found.'

## Exemple

```ahk
WinGetPos "Untitled", , &x, &y, &w, &h
```

## Voir aussi

[`WinMove`](WinMove.md), [`WinGetTitle`](WinGetTitle.md)
