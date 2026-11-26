# `ObjBindMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](index.md) | [中文（简体）](../zh/index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

日本語

*AutoHotkey v1.1.37.02* · function · COM 自動化 (ComObject)

## 説明

Binds an object to a method, answering a BoundFunc.

## 構文

```ahk
F := ObjBindMethod(Obj, Method, BoundArgs...)
```

## パラメータ

- **Obj** — The object.
- **Method** — The method name.
- **BoundArgs** — Leading args.

## 戻り値

A BoundFunc.

## 例

```ahk
f := ObjBindMethod(o, "m", 1)
```

## 関連

[`Func`](Func.md), [`BoundFunc`](BoundFunc.md), [`ObjGetMethod`](ObjGetMethod.md)
