# `ComObjActive`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Gets a running COM object by CLSID (the ROT).

## 構文

```ahk
Obj := ComObjActive(CLSID)
```

## パラメータ

- **CLSID** — The class id.

## 戻り値

The running object.

## 例

```ahk
o := ComObjActive("{...}")
```

## 関連

[`ComObjGet`](ComObjGet.md), [`ComObject`](ComObject.md)
