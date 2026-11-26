# `ComCall`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · function · COM 自動化 (ComObject)

## 描述

Calls a COM method by its vtable index.

## 語法

```ahk
R := ComCall(Index, Obj, Args..., "ReturnType")
```

## 參數

- **Index** — The vtable slot.
- **Obj** — The COM object.
- **Args** — Type/value pairs.

## 傳回值

The return value.

## 備註

On Linux the D-Bus model has no vtable index; refuses with the reason.

## 範例

```ahk
ComCall(0, obj, "Ptr")
```

## 相關

[`ComObject`](ComObject.md), [`ComObjQuery`](ComObjQuery.md)
