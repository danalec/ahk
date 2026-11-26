# `Mod`

| [English](../en/index.md) | [Español](../es/index.md) | [Português (Brasil)](../pt-BR/index.md) | [Français](../fr/index.md) | [Italiano](../it/index.md) | [Polski](../pl/index.md) | [Türkçe](../tr/index.md) | [Deutsch](../de/index.md) | [Русский](../ru/index.md) | [日本語](../ja/index.md) | [中文（简体）](index.md) | [繁體中文](../zh-Hant/index.md) | [한국어](../ko/index.md) |

中文（简体）

*AutoHotkey v1.1.37.02* · function · 语言核心：文本、数学与模式

## 描述

除法的余数。余数的符号与被除数相同，两个参数都是整数时结果也是整数。

## 语法

```ahk
R := Mod(Dividend, Divisor)
```

## 参数

- **Dividend** — The number.
- **Divisor** — Non-zero.

## 返回值

余数

## 错误

a divisor of zero fails with "Divide by zero."; a value that is not a number is refused with "Invalid parameter(s).".

## 备注

A zero divisor throws ZeroDivisionError.

## 示例

```ahk
Mod(7, 3)
```

## 相关

[`Abs`](Abs.md), [`Round`](Round.md), [`//`](__.md)
