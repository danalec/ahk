# `ObjRelease`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · COM 自动化 (ComObject)

## 描述

Releases a reference to an object pointer.

## 语法

```ahk
ObjRelease Ptr
```

## 参数

- **Ptr** — The pointer.

## 返回值

The new reference count.

## 示例

```ahk
ObjRelease p
```

## 相关

[`ObjPtrAddRef`](ObjPtrAddRef.md), [`ObjFromPtr`](ObjFromPtr.md)
