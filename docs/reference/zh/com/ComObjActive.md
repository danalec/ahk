# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Gets a running COM object by CLSID (the ROT).

## 语法

```ahk
Obj := ComObjActive(CLSID)
```

## 参数

- **CLSID** — The class id.

## 返回值

The running object.

## 示例

```ahk
o := ComObjActive("{...}")
```

## 相关

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
