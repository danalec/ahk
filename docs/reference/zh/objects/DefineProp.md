# `DefineProp`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Object` · 语言的对象：Array、Map 与 Func

## 描述

Defines a property with a getter/setter or value.

## 语法

```ahk
Obj.DefineProp(Name, Desc)
```

## 参数

- **Name** — The property.
- **Desc** — An object with Value/Get/Set/Call.

## 返回值

Nothing.

## 示例

```ahk
o.DefineProp("x", {Get: (*) => 1})
```

## 相关

[`DeleteProp`](DeleteProp.md), [`HasProp`](HasProp.md), [`OwnProps`](OwnProps.md)
