# `ComObjFromPtr`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · Automatización COM (ComObject)

## Descripción

Wraps a COM interface pointer as an object.

## Sintaxis

```ahk
Obj := ComObjFromPtr(Ptr)
```

## Parámetros

- **Ptr** — The interface pointer.

## Valor de retorno

The object.

## Ejemplo

```ahk
o := ComObjFromPtr(p)
```

## Relacionados

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
