# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · class · Die Objekte der Sprache: Array, Map und Func

## Beschreibung

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## Syntax

```ahk
Arr := Array(Values...)
```

## Parameter

- **Values** — The initial elements.

## Rückgabewert

the object of the class `Array`

## Fehler

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## Bemerkungen

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## Beispiel

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## Verwandt

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
