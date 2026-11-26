# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v1.1.37.02* · function · L'automatisation COM (ComObject)

## Description

Binds an object to a method, answering a BoundFunc.

## Syntaxe

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## Paramètres

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## Valeur de retour

A BoundFunc.

## Exemple

```ahk
f := ObjBindMethod(o, "m", 1)
```

## Voir aussi

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
