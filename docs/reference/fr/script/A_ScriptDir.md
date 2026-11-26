# `A_ScriptDir`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · L'analyseur et l'exécution du script

## Description

The directory where the run started. The script of the runner arrives on the standard input and has no file of its own, so the directory of the script is the directory of the run.

## Syntaxe

```ahk
A_ScriptDir 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the path of the directory, as a text

## Erreurs

the read fails when the runtime does not keep the directory where the run started; the write is refused.

## Exemple

```ahk
FileAppend A_ScriptDir, "*"
```
