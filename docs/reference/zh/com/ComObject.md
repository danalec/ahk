# `ComObject`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · class · COM 自动化 (ComObject)

## 描述

Creates or gets a COM object (Windows).

## 语法

```ahk
Obj := ComObject(CLSID, IID)
```

## 参数

- **CLSID** — The class id or ProgID.
- **IID** — The interface (default IDispatch).

## 返回值

A COM object.

## 备注

On the Linux box the object model of the session is D-Bus (DBusObject); the COM dispatch by index and the event sink refuse with the reason.

## 示例

```ahk
shell := ComObject("Shell.Application")
```

## 相关

[`ComValue`](ComValue.md), [`ObjGetBase`](ObjGetBase.md)
