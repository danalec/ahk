# `MouseGetPos`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · command · Raccourcis, chaînes de déclenchement et saisie

## Description

Reads the place of the pointer this runtime drives, out of the kernel. It is the only pointer on the box whose place can be read at all: the physical mouse is a relative device.

## Syntaxe

```ahk
MouseGetPos OutputX, OutputY, OutputWin, OutputControl, Flag
```

## Paramètres

- **Output*** — Variables that receive the answers.

## Valeur de retour

nothing; the place is written into the variables of the caller

## Erreurs

when the kernel will not answer, the desktop answers the place the runtime last put the pointer at, and says so in a note.

## Exemple

```ahk
MouseGetPos &x, &y
```

## Voir aussi

[`MouseMove`](MouseMove.md), [`PixelGetColor`](PixelGetColor.md)
