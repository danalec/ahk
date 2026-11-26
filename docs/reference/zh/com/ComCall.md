# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · COM 自动化 (ComObject)

## 描述

Calls a COM method by its vtable index.

## 语法

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## 参数

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## 返回值

The return value.

## 备注

On Linux the D-Bus model has no vtable index; refuses with the reason.

## 示例

```ahk
ComCall(0, obj, "Ptr")
```

## 相关

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
