# `Throw`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*KeySharp* · function · 语言核心：文本、数学与模式

## 描述

抛出（抛出）一个值的语句。这个值通常是错误对象，也可以是任何值。

## 语法

```ahk
Throw 
```

## 参数

Value

## 返回值

无返回值；该语句改变脚本的流程

## 错误

none; the value is caught by the catch that names its class.

## 示例

```ahk
throw ValueError("boom")
```
