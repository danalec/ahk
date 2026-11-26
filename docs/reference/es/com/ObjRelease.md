# `ObjRelease`

| [English](../en/index.md) | [Español](index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Español

*AutoHotkey v2.0.31* · function · Automatización COM (ComObject)

## Descripción

Releases a reference to an object pointer.

## Sintaxis

```ahk
ObjRelease Ptr
```

## Parámetros

- **Ptr** — The pointer.

## Valor de retorno

The new reference count.

## Ejemplo

```ahk
ObjRelease p
```

## Relacionados

[`ObjPtrAddRef`](ObjPtrAddRef.md), [`ObjFromPtr`](ObjFromPtr.md)
