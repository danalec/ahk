# `ComObjFromPtr`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · COM 自動化 (ComObject)

## 説明

Wraps a COM interface pointer as an object.

## 構文

```ahk
Obj := ComObjFromPtr(Ptr)
```

## パラメータ

- **Ptr** — The interface pointer.

## 戻り値

The object.

## 例

```ahk
o := ComObjFromPtr(p)
```

## 関連

[`ObjPtr`](ObjPtr.md), [`ComObjQuery`](ComObjQuery.md)
