# `Object`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言的对象：Array、Map 与 Func

## 描述

类链的根：运行时的每个对象都是它。`Object()` 造出一个空对象，字面量 `{a: 1}` 造出一个带属性的对象。

## 语法

```ahk
Obj := Object(Props...)
```

## 参数

- **Props** — Alternating names and values.

## 返回值

类为 `Object` 的对象

## 错误

nothing; a member that is not there is refused with the type of the value and the name of the member.

## 备注

Base is Object; OwnProps/HasProp/DefineProp/DeleteProp; __Get/__Set/__Call meta-functions.

## 示例

```ahk
o := Object()
FileAppend o.__Class, "*"
```

## 相关

[`Map`](Map.md), [`Array`](Array.md), [`Gui`](Gui.md)
