# `ObjPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · COM otomasyonu (ComObject)

## Açıklama

The pointer of an object, adding a reference.

## Sözdizimi

```ahk
P := ObjPtrAddRef(Obj)
```

## Parametreler

- **Obj** — The object.

## Dönüş değeri

The pointer with a reference.

## Örnek

```ahk
p := ObjPtrAddRef(o)
```

## İlgili

[`ObjPtr`](ObjPtr.md), [`ObjRelease`](ObjRelease.md)
