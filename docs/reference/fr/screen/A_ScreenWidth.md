# `A_ScreenWidth`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · Capture d'écran et fonctions de pixel

## Description

The width of the screen of the desktop, in pixels.

## Syntaxe

```ahk
A_ScreenWidth 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the width, as a number

## Erreurs

the read fails with the reason of the desktop when the screen cannot be measured; the write is refused.

## Exemple

```ahk
FileAppend A_ScreenWidth, "*"
```
