# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · class · Obiekty języka: Array, Map i Func

## Opis

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## Składnia

```ahk
Arr := Array(Values...)
```

## Parametry

- **Values** — The initial elements.

## Wartość zwracana

the object of the class `Array`

## Błędy

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## Uwagi

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## Przykład

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## Powiązane

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
