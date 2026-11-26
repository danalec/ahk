# `ObjPtrAddRef`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · Automatización COM (ComObject)

## Descripción

The pointer of an object, adding a reference.

## Sintaxis

```ahk
P := ObjPtrAddRef(Obj)
```

## Parámetros

- **Obj** — The object.

## Valor de retorno

The pointer with a reference.

## Ejemplo

```ahk
p := ObjPtrAddRef(o)
```

## Relacionados

[`ObjPtr`](ObjPtr.md), [`ObjRelease`](ObjRelease.md)
