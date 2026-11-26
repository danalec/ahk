# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v1.1.37.02* · function · Automazione COM (ComObject)

## Descrizione

Binds an object to a method, answering a BoundFunc.

## Sintassi

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parametri

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Valore di ritorno

A BoundFunc.

## Esempio

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Correlati

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
