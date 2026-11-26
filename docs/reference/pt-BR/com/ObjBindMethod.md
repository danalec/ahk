# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Português (Brasil)

*AutoHotkey v1.1.37.02* · function · Automação COM (ComObject)

## Descrição

Binds an object to a method, answering a BoundFunc.

## Sintaxe

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Parâmetros

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Valor de retorno

A BoundFunc.

## Exemplo

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Relacionados

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
