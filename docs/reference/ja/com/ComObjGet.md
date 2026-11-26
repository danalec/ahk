# `ComObjGet`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Gets a COM object from a moniker (a file/URL/ProgID).

## 構文

```ahk
Obj := ComObjGet(Name)
```

## パラメータ

- **Name** — The moniker.

## 戻り値

The object.

## 例

```ahk
o := ComObjGet("winmgmts:")
```

## 関連

[`ComObject`](ComObject.md), [`ComObjActive`](ComObjActive.md)
