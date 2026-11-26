# `ObjPtrAddRef`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v2.0.31* · function · COM 自動化 (ComObject)

## 説明

The pointer of an object, adding a reference.

## 構文

```ahk
P := ObjPtrAddRef(Obj)
```

## パラメータ

- **Obj** — The object.

## 戻り値

The pointer with a reference.

## 例

```ahk
p := ObjPtrAddRef(o)
```

## 関連

[`ObjPtr`](ObjPtr.md), [`ObjRelease`](ObjRelease.md)
