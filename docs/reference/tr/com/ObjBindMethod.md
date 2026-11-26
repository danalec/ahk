# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v1.1.37.02* · function · COM otomasyonu (ComObject)

## Açıklama

Binds an object to a method, answering a BoundFunc.

## Sözdizimi

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parametreler

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Dönüş değeri

A BoundFunc.

## Örnek

```ahk
f := ObjBindMethod(o, "m", 1)
```

## İlgili

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
