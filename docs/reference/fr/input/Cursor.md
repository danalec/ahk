# `Cursor`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · variable · Raccourcis, chaînes de déclenchement et saisie

## Description

The shape of the cursor of the pointer, the name the v2 gives it. The shape belongs to the machine, and the machine is asked for it.

## Syntaxe

```ahk
Cursor 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the name of the shape, as a text ("Arrow", "IBeam", "Wait", ...)

## Erreurs

the read fails with the reason of the desktop when the desktop cannot tell the shape of the cursor; the write is refused.

## Exemple

```ahk
FileAppend A_Cursor, "*"
```
