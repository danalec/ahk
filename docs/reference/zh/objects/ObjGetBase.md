# `ObjGetBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · function · 语言的对象：Array、Map 与 Func

## 描述

对象的基类：它那个类的原型。

## 语法

```ahk
ObjGetBase …
```

## 参数

the object

## 返回值

基类的对象

## 错误

a value that is not an object is refused with "Expected an object but got ...".

## 示例

```ahk
FileAppend ObjGetBase({a: 1}).__Class, "*"
```
