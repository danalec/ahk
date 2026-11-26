# `A_SendMode`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · Raccourcis, chaînes de déclenchement et saisie

## Description

The method that a `Send` uses: `Input`, `Event`, `Play` or `InputThenPlay`.

## Syntaxe

```ahk
A_SendMode 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the name of the mode, as a text

## Erreurs

the write takes the name of one of the four modes, and any other name is refused with "Invalid value.".

## Exemple

```ahk
FileAppend A_SendMode, "*"
```
