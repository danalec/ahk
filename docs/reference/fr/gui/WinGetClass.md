# `WinGetClass`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Fenêtres d'interface et automatisation des fenêtres

## Description

The class of the window that matches the title that was given.

## Syntaxe

```ahk
WinGetClass …
```

## Paramètres

[WindowTitle] [, WinText] [, ExcludeTitle] [, ExcludeText]

## Valeur de retour

the class of the window

## Erreurs

a title that matches no window answers 'Target window not found.'; the text of the window and the exclusions are not carried yet.

## Exemple

```ahk
class := WinGetClass("Untitled")
```
