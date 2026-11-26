# `Func`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言的对象：Array、Map 与 Func

## 描述

函数的类：脚本里的函数、方法、lambda 或内置函数。这个类造不出来：`Func(...)` 回答“Invalid base.”；函数值的类就是 `Func`。

## 语法

```ahk
Func …
```

## 参数

nothing that makes it

## 返回值

函数值的类，作为文本

## 错误

the call of the class is refused with "Invalid base.".

## 示例

```ahk
FileAppend Abs.__Class, "*"
```
