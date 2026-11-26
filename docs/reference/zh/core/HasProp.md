# `HasProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Object` · 语言核心：文本、数学与模式

## 描述

某个对象是否有某个属性。

## 语法

```ahk
B := HasProp(Obj, Name)
```

## 参数

- **Obj** — The object.
- **Name** — The property.

## 返回值

对象有该属性时为 1，没有时为 0

## 错误

nothing; a value that is not an object has the property `__Class` and no other, so the answer for it is one only for that name.

## 示例

```ahk
o := {a: 1}
FileAppend HasProp(o, "a"), "*"
```

## 相关

[`HasMethod`](HasMethod.md), [`DefineProp`](DefineProp.md), [`OwnProps`](OwnProps.md)
