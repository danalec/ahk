# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*KeySharp* · method · `Array` · 語言的物件：Array、Map 與 Func

## 描述

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## 語法

```ahk
M := Map(Key1, Value1, ...)
```

## 參數

- **Pairs** — Alternating keys and values.

## 傳回值

the object of the class `Map`

## 錯誤

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## 備註

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## 範例

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## 相關

[`Array`](Array.md), [`Object`](Object.md)
