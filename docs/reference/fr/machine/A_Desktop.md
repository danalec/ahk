# `A_Desktop`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · La machine, les processus et l'environnement

## Description

The folder of the desktop of the user: the folder the machine knows, and `$HOME/Desktop` when it knows none.

## Syntaxe

```ahk
A_Desktop 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

the path of the folder, as a text

## Erreurs

the read fails with "the machine has no such folder." when the machine has no such folder; the write is refused.

## Exemple

```ahk
FileAppend A_Desktop, "*"
```
