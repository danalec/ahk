# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Türkçe

*AutoHotkey v2.0.31* · function · COM otomasyonu (ComObject)

## Açıklama

Wraps a COM interface pointer as an object.

## Sözdizimi

```ahk
Obj := ComObjFromPtr(Ptr)
```

## Parametreler

- **Ptr** — The interface pointer.

## Dönüş değeri

The object.

## Örnek

```ahk
o := ComObjFromPtr(p)
```

## İlgili

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
