# `A_ScriptName`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · L'analyseur et l'exécution du script

## Description

The name of the script. The script of the runner comes on the standard input and has no file, so the name is an asterisk. A script may write the name, and the value written is the one it keeps.

## Syntaxe

```ahk
A_ScriptName 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the text "*", or the text that was written

## Erreurs

nothing to read; the write takes any value.

## Exemple

```ahk
FileAppend A_ScriptName, "*"
```
