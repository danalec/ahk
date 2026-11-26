# `ComObjArray`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · COM 自动化 (ComObject)

## 描述

Creates a COM safe array.

## 语法

```ahk
A := ComObjArray(Type, Counts...)
```

## 参数

2..255.

## 返回值

A COM array.

## 示例

```ahk
a := ComObjArray(0xB, 3)
```

## 相关

[`ComObject`](ComObject.md), [`ComValue`](ComValue.md)
