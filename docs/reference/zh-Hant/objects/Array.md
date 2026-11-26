# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · class · 語言的物件：Array、Map 與 Func

## 描述

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## 語法

```ahk
Arr := Array(Values...)
```

## 參數

- **Values** — The initial elements.

## 傳回值

the object of the class `Array`

## 錯誤

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## 備註

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## 範例

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## 相關

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
