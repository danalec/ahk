# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](../zh/index.md) | [繁體中文](index.md) | [한국어](../ko/index.md) |

繁體中文

*AutoHotkey v2.0.31* · method · `Object` · 語言核心：文字、數學與模式

## 描述

Whether an object has a property.

## 語法

```ahk
B := HasProp(Obj, Name)
```

## 參數

- **Obj** — The object.
- **Name** — The property.

## 傳回值

1 when the object has such a property, 0 when it has not

## 錯誤

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## 範例

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## 相關

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
