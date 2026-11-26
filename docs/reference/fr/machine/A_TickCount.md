# `A_TickCount`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · La machine, les processus et l'environnement

## Description

The number of milliseconds the runtime has counted since an arbitrary origin. The count never goes backwards.

## Syntaxe

```ahk
A_TickCount 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the number of milliseconds, as a number

## Erreurs

the write is refused; the name is a built-in variable that the dialect does not let a script write.

## Exemple

```ahk
FileAppend A_TickCount, "*"
```
