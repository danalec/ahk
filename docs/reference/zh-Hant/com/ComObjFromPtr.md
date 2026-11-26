# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · COM 自動化 (ComObject)

## 描述

Wraps a COM interface pointer as an object.

## 語法

```ahk
Obj := ComObjFromPtr(Ptr)
```

## 參數

- **Ptr** — The interface pointer.

## 傳回值

The object.

## 範例

```ahk
o := ComObjFromPtr(p)
```

## 相關

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
