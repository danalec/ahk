# `OwnProps`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Object` · 语言的对象：Array、Map 与 Func

## 描述

Iterates the own properties of an object.

## 语法

```ahk
for name, value in Obj.OwnProps()
```

## 参数

?.

## 返回值

The enumerator.

## 示例

```ahk
for n, v in o.OwnProps()
```

## 相关

[`HasOwnProp`](HasOwnProp.md), [`DefineProp`](DefineProp.md)
