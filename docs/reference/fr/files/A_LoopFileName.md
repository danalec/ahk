# `A_LoopFileName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · Les opérations sur les fichiers et les répertoires

## Description

The name of the file the file loop is on, without its folder.

## Syntaxe

```ahk
A_LoopFileName 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the name of the file, as a text; the empty text before the first file loop

## Erreurs

nothing to read; the write is refused.

## Exemple

```ahk
Loop Files "*.txt"
    FileAppend A_LoopFileName, "*"
```
