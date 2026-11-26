# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 描述

Gets/sets the flags of a COM wrapper.

## 語法

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## 參數

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## 傳回值

The flags.

## 範例

```ahk
f := ComObjFlags(o)
```

## 相關

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
