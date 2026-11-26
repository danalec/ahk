# `Map`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*KeySharp* · method · `Array` · Gli oggetti del linguaggio: Array, Map e Func

## Descrizione

The class of a table of keys and values. `Map(key, value, ...)` makes one, and the order of the entries is the order they were put in.

## Sintassi

```ahk
M := Map(Key1, Value1, ...)
```

## Parametri

- **Pairs** — Alternating keys and values.

## Valore di ritorno

the object of the class `Map`

## Errori

an odd number of arguments is refused with "Invalid number of parameters."; a key that is not there is refused with "Item has no value.".

## Osservazioni

Backed by a BTreeMap on this tree (ordered, comparison-based). CaseSense/Default properties; Get/Set/Has/Delete/Clear/Count/__Enum.

## Esempio

```ahk
m := Map("x", 1)
FileAppend m.Count, "*"
```

## Correlati

[`Array`](Array.md), [`Object`](Object.md)
