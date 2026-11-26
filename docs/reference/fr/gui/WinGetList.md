# `WinGetList`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

The identifiers of every window that matches the title that was given.

## Syntaxe

```ahk
WinGetList …
```

## Paramètres

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valeur de retour

an array of the identifiers, empty when nothing matches

## Erreurs

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Exemple

```ahk
list := WinGetList("Untitled")
```
