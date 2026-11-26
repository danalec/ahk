# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · class · Объекты языка: Array, Map и Func

## Описание

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## Синтаксис

```ahk
Arr := Array(Values...)
```

## Параметры

- **Values** — The initial elements.

## Возвращаемое значение

the object of the class `Array`

## Ошибки

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## Замечания

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## Пример

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## См. также

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
