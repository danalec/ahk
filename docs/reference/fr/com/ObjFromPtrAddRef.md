# `ObjFromPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

Français

*AutoHotkey v2.0.31* · function · L'automatisation COM (ComObject)

## Description

Wraps an object pointer and adds a reference.

## Syntaxe

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## Paramètres

- **Ptr** — The pointer.

## Valeur de retour

The object with a reference.

## Exemple

```ahk
o := ObjFromPtrAddRef(p)
```

## Voir aussi

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
