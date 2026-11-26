# `ObjFromPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · COM 自动化 (ComObject)

## 描述

Wraps an object pointer and adds a reference.

## 语法

```ahk
Obj := ObjFromPtrAddRef(Ptr)
```

## 参数

- **Ptr** — The pointer.

## 返回值

The object with a reference.

## 示例

```ahk
o := ObjFromPtrAddRef(p)
```

## 相关

[`ObjFromPtr`](ObjFromPtr.md), [`ObjRelease`](ObjRelease.md)
