# `CoordMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · L'analyseur et l'exécution du script

## Description

The way the coordinates are read. The five targets of v2 are the mouse, the pixels, the caret, the menu and the tooltip; the tree has no windows, so the modes of the caret, the menu and the tooltip are kept all the same and nothing uses them.

## Syntaxe

```ahk
CoordMode …
```

## Paramètres

Target [, Relative]

## Valeur de retour

the mode that was there before

## Erreurs

a target or a relative that v2 does not know is refused as an invalid parameter.

## Exemple

```ahk
CoordMode "Mouse", "Screen"
```
