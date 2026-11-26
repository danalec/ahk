# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Polski

*AutoHotkey v2.0.31* · function · Automatyzacja COM (ComObject)

## Opis

Wraps a COM interface pointer as an object.

## Składnia

```ahk
Obj := ComObjFromPtr(Ptr)
```

## Parametry

- **Ptr** — The interface pointer.

## Wartość zwracana

The object.

## Przykład

```ahk
o := ComObjFromPtr(p)
```

## Powiązane

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
