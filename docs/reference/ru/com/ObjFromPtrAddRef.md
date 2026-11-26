# `ObjFromPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Русский

*AutoHotkey v2.0.31* · function · COM-автоматизация (ComObject)

## Описание

Wraps an object pointer and adds a reference.

## Синтаксис

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## Параметры

- **Ptr** — The pointer.

## Возвращаемое значение

The object with a reference.

## Пример

```ahk
o := ObjFromPtrAddRef(p)
```

## См. также

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
