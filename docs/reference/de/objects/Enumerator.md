# `Enumerator`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · class · Die Objekte der Sprache: Array, Map und Func

## Beschreibung

The class of the walker of a loop: an enumerator hands out the pieces of a sequence, one turn at a time. The class is not made; the `__Enum` of a value makes one, and the call of it is the turn of the enumeration.

## Syntax

```ahk
Enumerator 
```

## Parameter

nothing that makes it

## Rückgabewert

the class of an enumerator, as a text

## Fehler

the call of the class is refused with "Invalid base.".

## Beispiel

```ahk
for item in [1, 2]
    FileAppend Type(item), "*"
```
