# `Array`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · class · 语言的对象：Array、Map 与 Func

## 描述

一排值的类。`Array(...)` 用给定的值造出数组，字面量 `[1, 2, 3]` 也一样。

## 语法

```ahk
Arr := Array(Values...)
```

## 参数

- **Values** — The initial elements.

## 返回值

类为 `Array` 的对象

## 错误

nothing to make the array; the places count from one, and an index that is not there is refused with "Invalid index.".

## 备注

Methods: Push/Pop/InsertAt/RemoveAt/Get/Set/Length/Capacity/Clone/__Enum.

## 示例

```ahk
a := Array(1, 2, 3)
FileAppend a.Length, "*"
```

## 相关

[`Map`](Map.md), [`Object`](Object.md), [`StrSplit`](StrSplit.md)
