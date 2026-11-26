# `A_IsAdmin`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · variable · L'analyseur et l'exécution du script

## Description

Whether the script runs with the rights of an administrator: the effective user id, and the root is the one that is zero.

## Syntaxe

```ahk
A_IsAdmin 
```

## Paramètres

nothing; it is a variable, not a call

## Valeur de retour

1 when the script is elevated, 0 when it is not

## Erreurs

the read fails when the machine cannot say; the write is refused.

## Exemple

```ahk
FileAppend A_IsAdmin, "*"
```
