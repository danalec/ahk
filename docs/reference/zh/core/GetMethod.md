# `GetMethod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Object` · 语言核心：文本、数学与模式

## 描述

对象的某个方法，作为一个可以调用的值。

## 语法

```ahk
GetMethod 
```

## 参数

the object, the name of the method, and the number of parameters it would take

## 返回值

该方法，作为类为 `Func` 的值

## 错误

a method that is not there is refused with the type of the value and the name of the member.

## 示例

```ahk
o := {a: 1}
FileAppend GetMethod(o, "HasOwnProp").__Class, "*"
```
