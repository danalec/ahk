# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · class · Les objets du langage : Array, Map et Func

## Description

The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.

## Syntaxe

```ahk
Enumerator 
```

## Paramètres

nothing that makes it

## Valeur de retour

the class of an enumerator, as a text

## Erreurs

the call of the class is refused with "Invalid base.".

## Exemple

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
