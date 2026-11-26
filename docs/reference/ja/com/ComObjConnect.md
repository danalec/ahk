# `ComObjConnect`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Connects a COM object's events to script functions.

## 構文

```ahk
ComObjConnect Obj, Prefix
```

## パラメータ

- **Obj** — The object.
- **Prefix** — The function prefix.

## 戻り値

Nothing; the event sink refuses on Linux.

## 例

```ahk
ComObjConnect ie, "IE_"
```

## 関連

[`ComObject`](ComObject.md), [`OnMessage`](OnMessage.md)
