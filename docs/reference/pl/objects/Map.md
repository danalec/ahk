# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*KeySharp* · method · `Array` · Obiekty języka: Array, Map i Func

## Opis

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## Składnia

```ahk
M := Map(Key1, Value1, ...)
```

## Parametry

- **Pairs** — Alternating keys and values.

## Wartość zwracana

the object of the class `Map`

## Błędy

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## Uwagi

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## Przykład

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## Powiązane

[`Array`](Array.md), [`Object`](Object.md)
