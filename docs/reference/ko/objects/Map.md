# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](index.md) |

한국어

*KeySharp* · method · `Array` · 언어의 객체: Array, Map, Func

## 설명

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## 구문

```ahk
M := Map(Key1, Value1, ...)
```

## 매개변수

- **Pairs** — Alternating keys and values.

## 반환 값

the object of the class `Map`

## 오류

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## 설명

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## 예제

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## 관련

[`Array`](Array.md), [`Object`](Object.md)
