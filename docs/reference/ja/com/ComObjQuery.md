# `ComObjQuery`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Queries another interface of a COM object.

## 構文

```ahk
Obj := ComObjQuery(Obj, IID)
```

## パラメータ

- **Obj** — The object.
- **IID** — The interface.

## 戻り値

The interface object.

## 例

```ahk
q := ComObjQuery(o, "{...}")
```

## 関連

[`ComObject`](ComObject.md), [`ComCall`](ComCall.md)
