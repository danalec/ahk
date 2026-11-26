# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Connects a COM object's events to script functions.

## 语法

```ahk
ComObjConnect Obj, Prefix
```

## 参数

- **Obj** — The object.
- **Prefix** — The function prefix.

## 返回值

Nothing; the event sink refuses on Linux.

## 示例

```ahk
ComObjConnect ie, "IE_"
```

## 相关

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
