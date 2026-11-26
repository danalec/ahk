# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 描述

Gets a running COM object by CLSID (the ROT).

## 語法

```ahk
Obj := ComObjActive(CLSID)
```

## 參數

- **CLSID** — The class id.

## 傳回值

The running object.

## 範例

```ahk
o := ComObjActive("{...}")
```

## 相關

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
