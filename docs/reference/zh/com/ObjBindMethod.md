# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Binds an object to a method, answering a BoundFunc.

## 语法

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## 参数

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## 返回值

A BoundFunc.

## 示例

```ahk
f := ObjBindMethod(o, "m", 1)
```

## 相关

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
