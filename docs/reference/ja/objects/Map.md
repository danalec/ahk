# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*KeySharp* · method · `Array` · 言語のオブジェクト: Array、Map、Func

## 説明

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## 構文

```ahk
M := Map(Key1, Value1, ...)
```

## パラメータ

- **Pairs** — Alternating keys and values.

## 戻り値

the object of the class `Map`

## エラー

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## 備考

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## 例

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## 関連

[`Array`](Array.md), [`Object`](Object.md)
