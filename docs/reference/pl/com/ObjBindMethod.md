# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v1.1.37.02* · function · Automatyzacja COM (ComObject)

## Opis

Binds an object to a method, answering a BoundFunc.

## Składnia

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parametry

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Wartość zwracana

A BoundFunc.

## Przykład

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Powiązane

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
