# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Queries another interface of a COM object.

## 语法

```ahk
Obj := ComObjQuery(Obj, IID)
```

## 参数

- **Obj** — The object.
- **IID** — The interface.

## 返回值

The interface object.

## 示例

```ahk
q := ComObjQuery(o, "{...}")
```

## 相关

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
