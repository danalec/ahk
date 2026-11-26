# `Map`

| [English](index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

English

*KeySharp* · method · `Array` · The objects of the language: Array, Map and Func

## Description

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## Syntax

```ahk
M := Map(Key1, Value1, ...)
```

## Parameters

- **Pairs** — Alternating keys and values.

## Return value

the object of the class `Map`

## Errors

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## Remarks

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## Example

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## Related

[`Array`](Array.md), [`Object`](Object.md)
