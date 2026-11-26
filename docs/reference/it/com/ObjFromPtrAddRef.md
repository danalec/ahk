# `ObjFromPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Italiano

*AutoHotkey v2.0.31* · function · Automazione COM (ComObject)

## Descrizione

Wraps an object pointer and adds a reference.

## Sintassi

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## Parametri

- **Ptr** — The pointer.

## Valore di ritorno

The object with a reference.

## Esempio

```ahk
o := ObjFromPtrAddRef(p)
```

## Correlati

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
