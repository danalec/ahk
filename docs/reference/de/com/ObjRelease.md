# `ObjRelease`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Deutsch

*AutoHotkey v2.0.31* · function · COM-Automatisierung (ComObject)

## Beschreibung

Releases a reference to an object pointer.

## Syntax

```ahk
ObjRelease Ptr
```

## Parameter

- **Ptr** — The pointer.

## Rückgabewert

The new reference count.

## Beispiel

```ahk
ObjRelease p
```

## Verwandt

[`ObjPtrAddRef`](ObjPtrAddRef.md), [`ObjFromPtr`](ObjFromPtr.md)
