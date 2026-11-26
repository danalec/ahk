# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · method · `Array` · 语言的对象：Array、Map 与 Func

## 描述

键和值的表的类。`Map(key, value, ...)` 造出这样一个表，各条目的顺序就是放进去的顺序。

## 语法

```ahk
M := Map(Key1, Value1, ...)
```

## 参数

- **Pairs** — Alternating keys and values.

## 返回值

类为 `Map` 的对象

## 错误

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## 备注

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## 示例

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## 相关

[`Array`](Array.md), [`Object`](Object.md)
