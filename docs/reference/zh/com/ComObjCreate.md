# `ComObjCreate`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Creates a COM object (v1).

## 语法

```ahk
Obj := ComObjCreate(CLSID)
```

## 参数

1..2.

## 返回值

The object.

## 示例

```ahk
o := ComObjCreate("Shell.Application")
```

## 相关

[`ComObject`](ComObject.md)
