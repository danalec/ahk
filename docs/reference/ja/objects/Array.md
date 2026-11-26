# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · class · 言語のオブジェクト: Array、Map、Func

## 説明

The class of a row of values. `Array(...)` makes one with the values it is given, and the literal `[1, 2, 3]` makes one too.

## 構文

```ahk
Arr := Array(Values...)
```

## パラメータ

- **Values** — The initial elements.

## 戻り値

the object of the class `Array`

## エラー

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## 備考

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## 例

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## 関連

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
