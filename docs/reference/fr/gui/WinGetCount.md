# `WinGetCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

How many windows match the title that was given.

## Syntaxe

```ahk
WinGetCount …
```

## Paramètres

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valeur de retour

the number of the windows, a number

## Erreurs

the text of the window and the exclusions are not carried yet; a question the compositor cannot answer fails with its reason.

## Exemple

```ahk
count := WinGetCount("Untitled")
```
