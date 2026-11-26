# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · method · `Object` · 語言的物件：Array、Map 與 Func

## 描述

Iterates the own properties of an object.

## 語法

```ahk
for name, value in Obj.OwnProps()
```

## 參數

?.

## 傳回值

The enumerator.

## 範例

```ahk
for n, v in o.OwnProps()
```

## 相關

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
