# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*AutoHotkey v2.0.31* · class · 언어의 객체: Array, Map, Func

## 설명

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## 구문

```ahk
Arr := Array(Values...)
```

## 매개변수

- **Values** — The initial elements.

## 반환 값

the object of the class `Array`

## 오류

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## 설명

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## 예제

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## 관련

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
