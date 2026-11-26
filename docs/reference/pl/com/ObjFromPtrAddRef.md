# `ObjFromPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Automatyzacja COM (ComObject)

## Opis

Wraps an object pointer and adds a reference.

## Składnia

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## Parametry

- **Ptr** — The pointer.

## Wartość zwracana

The object with a reference.

## Przykład

```ahk
o := ObjFromPtrAddRef(p)
```

## Powiązane

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
