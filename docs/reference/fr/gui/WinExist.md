# `WinExist`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · Fenêtres d'interface et automatisation des fenêtres

## Description

Whether a window with this title exists. The windows are the ones of the compositor, and a number names one of them.

## Syntaxe

```ahk
HWND := WinExist(WinTitle, WinText, ExcludeTitle, ExcludeText)
```

## Paramètres

- **WinTitle** — The criteria.

## Valeur de retour

the identifier of the window that is there, zero when there is none

## Erreurs

a window that is not there is an answer and not a failure; a question the compositor cannot answer fails with its reason.

## Remarques

Sets the last-found window for the Win* family.

## Exemple

```ahk
if WinExist("Untitled")
```

## Voir aussi

[`WinActive`](WinActive.md), [`WinActivate`](WinActivate.md)
