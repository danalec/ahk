# `ObjBindMethod`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v1.1.37.02* · function · Automatización COM (ComObject)

## Descripción

Binds an object to a method, answering a BoundFunc.

## Sintaxis

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parámetros

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Valor de retorno

A BoundFunc.

## Ejemplo

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Relacionados

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
