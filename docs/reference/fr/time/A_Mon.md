# `A_Mon`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · Les dates et les heures

## Description

The month of the local time of the machine. This tree answers the same two digits as `A_MM`, and not the short name of the month.

## Syntaxe

```ahk
A_Mon 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the month, as a text of two digits

## Erreurs

the read fails when the clock of the runtime does not know the date of the machine; the write is refused.

## Exemple

```ahk
FileAppend A_Mon, "*"
```
