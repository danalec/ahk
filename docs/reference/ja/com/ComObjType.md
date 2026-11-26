# `ComObjType`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Reads the type/name of a COM object.

## 構文

```ahk
S := ComObjType(Obj, What)
```

## パラメータ

- **Obj** — The object.
- **What** — Name/Class/IID/...

## 戻り値

The requested string.

## 例

```ahk
n := ComObjType(o, "Name")
```

## 関連

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
