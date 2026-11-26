# `ComObjFlags`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Gets/sets the flags of a COM wrapper.

## 構文

```ahk
F := ComObjFlags(Obj, Flags, Mask)
```

## パラメータ

- **Obj** — The wrapper.
- **Flags/Mask** — The bits.

## 戻り値

The flags.

## 例

```ahk
f := ComObjFlags(o)
```

## 関連

[`ComObject`](ComObject.md), [`ComObjValue`](ComObjValue.md)
