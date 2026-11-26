# `ComObjValue`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Reads the underlying pointer/value of a COM wrapper.

## 语法

```ahk
P := ComObjValue(Obj)
```

## 参数

- **Obj** — The wrapper.

## 返回值

The pointer or value.

## 示例

```ahk
p := ComObjValue(o)
```

## 相关

[`ComObjFlags`](ComObjFlags.md), [`ObjPtr`](ObjPtr.md)
