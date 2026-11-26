# `HasBase`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v2.0.31* · method · `Object` · 语言核心：文本、数学与模式

## 描述

某个对象的基类是不是另一个对象。

## 语法

```ahk
HasBase 
```

## 参数

the object, and the base

## 返回值

基类是它时为 1，否则为 0

## 错误

a base that is not an object is refused before the question of the object is asked.

## 示例

```ahk
FileAppend HasBase({a: 1}, {}), "*"
```
