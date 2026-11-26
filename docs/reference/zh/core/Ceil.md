# `Ceil`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

不小于该数的整数。

## 语法

```ahk
N := Ceil(Number)
```

## 参数

- **Number** — The value.

## 返回值

该整数；原来的数不是整数时返回浮点数

## 错误

a value that is not a number is refused.

## 示例

```ahk
Ceil(1.2)
```

## 相关

[`Floor`](Floor.md), [`Round`](Round.md)
